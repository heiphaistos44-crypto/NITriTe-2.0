<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useNotificationStore } from "@/stores/notifications";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NInput from "@/components/ui/NInput.vue";
import {
  Package, RefreshCw, Download, CheckCircle, XCircle,
  Search, AlertTriangle, ExternalLink, Layers,
} from "lucide-vue-next";

const notify = useNotificationStore();

interface Dependency {
  id:          string;
  name:        string;
  category:    string;
  description: string;
  installed:   boolean;
  version:     string;
  winget_id:   string;
  install_url: string;
}

const deps          = ref<Dependency[]>([]);
const loading       = ref(false);
const installing    = ref<string | null>(null);
const filterText    = ref("");
const selectedCat   = ref("Tous");
const lastChecked   = ref<string | null>(null);

const categories = computed(() => {
  const cats = new Set(deps.value.map(d => d.category));
  return ["Tous", ...Array.from(cats).sort()];
});

const filtered = computed(() => {
  let list = deps.value;
  if (selectedCat.value !== "Tous") list = list.filter(d => d.category === selectedCat.value);
  if (filterText.value.trim()) {
    const q = filterText.value.toLowerCase();
    list = list.filter(d => d.name.toLowerCase().includes(q) || d.description.toLowerCase().includes(q));
  }
  return list;
});

const stats = computed(() => ({
  total:     deps.value.length,
  installed: deps.value.filter(d => d.installed).length,
  missing:   deps.value.filter(d => !d.installed).length,
}));

async function checkAll() {
  loading.value = true;
  try {
    deps.value = await invoke<Dependency[]>("check_all_dependencies");
    lastChecked.value = new Date().toLocaleTimeString("fr-FR");
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  loading.value = false;
}

async function installDep(dep: Dependency) {
  if (!dep.winget_id && !dep.install_url) {
    notify.warning(dep.name, "Pas d'ID winget — installez manuellement.");
    return;
  }
  if (!dep.winget_id) {
    // Ouvrir URL manuelle
    await invoke("open_url", { url: dep.install_url });
    return;
  }
  installing.value = dep.id;
  notify.info(dep.name, "Installation via WinGet en cours...");
  try {
    const msg = await invoke<string>("install_dependency", { wingetId: dep.winget_id });
    notify.success(dep.name, msg);
    // Re-vérifier cette dépendance
    await checkAll();
  } catch (e: any) {
    notify.error(dep.name, String(e));
  }
  installing.value = null;
}

async function openUrl(url: string) {
  if (!url) return;
  await invoke("open_url", { url });
}

onMounted(checkAll);
</script>

<template>
  <div class="dep-page">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1><Package :size="22" style="margin-right:10px;vertical-align:middle;color:var(--accent-primary)" />Gestionnaire de Dépendances</h1>
        <p class="page-subtitle">Vérifiez et installez les outils essentiels pour votre environnement</p>
      </div>
      <NButton variant="primary" :disabled="loading" @click="checkAll">
        <NSpinner v-if="loading" :size="14" />
        <RefreshCw v-else :size="14" />
        {{ loading ? "Vérification..." : "Tout vérifier" }}
      </NButton>
    </div>

    <!-- Stats globales -->
    <div v-if="deps.length" class="stats-bar">
      <div class="stat-item ok">
        <CheckCircle :size="16" />
        <span>{{ stats.installed }} installé{{ stats.installed > 1 ? 's' : '' }}</span>
      </div>
      <div class="stat-item warn">
        <XCircle :size="16" />
        <span>{{ stats.missing }} manquant{{ stats.missing > 1 ? 's' : '' }}</span>
      </div>
      <div class="stat-item muted">
        <Layers :size="16" />
        <span>{{ stats.total }} au total</span>
      </div>
      <div class="progress-bar-wrap">
        <div class="progress-bar-fill" :style="{ width: (stats.installed / stats.total * 100) + '%' }" />
      </div>
      <span v-if="lastChecked" class="muted" style="font-size:11px;margin-left:auto">Vérifié à {{ lastChecked }}</span>
    </div>

    <!-- Loading -->
    <div v-if="loading && !deps.length" class="loading-state">
      <NSpinner :size="28" />
      <p>Vérification de l'environnement...</p>
    </div>

    <template v-else-if="deps.length">
      <!-- Filtres -->
      <div class="filters">
        <NInput v-model="filterText" placeholder="Rechercher..." style="max-width:240px">
          <template #prefix><Search :size="14" /></template>
        </NInput>
        <div class="cat-pills">
          <button v-for="cat in categories" :key="cat"
            :class="['cat-pill', { active: selectedCat === cat }]"
            @click="selectedCat = cat">
            {{ cat }}
            <span v-if="cat !== 'Tous'" class="cat-count">
              {{ deps.filter(d => d.category === cat && !d.installed).length > 0
                ? deps.filter(d => d.category === cat && !d.installed).length + ' ✗'
                : '✓' }}
            </span>
          </button>
        </div>
      </div>

      <!-- Tableau dépendances -->
      <div class="dep-table-wrap">
        <table class="dep-table">
          <thead>
            <tr>
              <th>État</th>
              <th>Outil</th>
              <th>Catégorie</th>
              <th>Version</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="dep in filtered" :key="dep.id" :class="{ 'row-missing': !dep.installed }">
              <td class="cell-status">
                <CheckCircle v-if="dep.installed" :size="16" class="ic-ok" />
                <XCircle v-else :size="16" class="ic-err" />
              </td>
              <td>
                <div class="dep-name">{{ dep.name }}</div>
                <div class="dep-desc">{{ dep.description }}</div>
              </td>
              <td>
                <NBadge variant="neutral" style="font-size:10px">{{ dep.category }}</NBadge>
              </td>
              <td class="cell-version">
                <span v-if="dep.installed && dep.version" class="version-text">{{ dep.version }}</span>
                <span v-else-if="dep.installed" class="muted">—</span>
                <NBadge v-else variant="danger" style="font-size:10px">Non installé</NBadge>
              </td>
              <td class="cell-action">
                <div v-if="!dep.installed" style="display:flex;gap:6px;align-items:center">
                  <NButton v-if="dep.winget_id" variant="primary" size="sm"
                    :disabled="installing === dep.id"
                    @click="installDep(dep)">
                    <NSpinner v-if="installing === dep.id" :size="12" />
                    <Download v-else :size="12" />
                    {{ installing === dep.id ? 'Installation...' : 'Installer' }}
                  </NButton>
                  <NButton v-if="dep.install_url" variant="ghost" size="sm"
                    @click="openUrl(dep.install_url)">
                    <ExternalLink :size="12" />
                  </NButton>
                </div>
                <div v-else style="display:flex;gap:6px;align-items:center">
                  <span class="ok-text">Installé</span>
                  <NButton v-if="dep.install_url" variant="ghost" size="sm"
                    @click="openUrl(dep.install_url)">
                    <ExternalLink :size="12" />
                  </NButton>
                </div>
              </td>
            </tr>
            <tr v-if="!filtered.length">
              <td colspan="5" style="text-align:center;color:var(--text-muted);padding:24px;font-size:13px">
                Aucun résultat pour "{{ filterText }}"
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Note WinGet -->
      <div class="note-box">
        <AlertTriangle :size="14" style="color:var(--warning);flex-shrink:0;margin-top:1px" />
        <span>Les installations via <strong>WinGet</strong> s'exécutent silencieusement en arrière-plan.
          Relancez une vérification après quelques instants pour confirmer.
          Pour les outils sans ID WinGet (Scoop, Chocolatey), cliquez sur <ExternalLink :size="11" style="vertical-align:middle" /> pour accéder à la page officielle.</span>
      </div>
    </template>
  </div>
</template>

<style scoped>
.dep-page { display:flex;flex-direction:column;gap:18px; }
.page-header { display:flex;justify-content:space-between;align-items:flex-start;flex-wrap:wrap;gap:12px; }
.page-header h1 { font-size:22px;font-weight:700; }
.page-subtitle { color:var(--text-muted);font-size:13px;margin-top:3px; }

.stats-bar { display:flex;align-items:center;gap:16px;padding:12px 16px;
  background:var(--bg-secondary);border:1px solid var(--border);border-radius:8px;flex-wrap:wrap; }
.stat-item { display:flex;align-items:center;gap:6px;font-size:13px;font-weight:500; }
.stat-item.ok { color:var(--success); }
.stat-item.warn { color:var(--danger); }
.stat-item.muted { color:var(--text-muted); }
.progress-bar-wrap { flex:1;min-width:120px;height:6px;background:var(--bg-tertiary);border-radius:3px;overflow:hidden; }
.progress-bar-fill { height:100%;background:var(--success);border-radius:3px;transition:width .4s ease; }

.loading-state { display:flex;flex-direction:column;align-items:center;gap:12px;padding:50px;color:var(--text-secondary);font-size:13px; }

.filters { display:flex;align-items:center;gap:12px;flex-wrap:wrap; }
.cat-pills { display:flex;gap:6px;flex-wrap:wrap; }
.cat-pill { padding:4px 12px;border-radius:16px;border:1px solid var(--border);background:transparent;
  font-size:12px;color:var(--text-secondary);cursor:pointer;transition:all .15s;display:flex;align-items:center;gap:5px; }
.cat-pill:hover { border-color:var(--accent-primary);color:var(--accent-primary); }
.cat-pill.active { background:var(--accent-primary);border-color:var(--accent-primary);color:#fff; }
.cat-count { font-size:10px;opacity:.75; }

.dep-table-wrap { overflow-x:auto; }
.dep-table { width:100%;border-collapse:collapse;font-size:13px; }
.dep-table th { text-align:left;padding:8px 12px;color:var(--text-muted);font-weight:500;font-size:11px;
  text-transform:uppercase;letter-spacing:.5px;border-bottom:1px solid var(--border);background:var(--bg-secondary); }
.dep-table td { padding:10px 12px;border-bottom:1px solid var(--border);vertical-align:middle; }
.dep-table tbody tr:hover { background:var(--bg-secondary); }
.dep-table .row-missing { background:color-mix(in srgb,var(--danger) 4%,transparent); }
.cell-status { width:40px;text-align:center; }
.dep-name { font-weight:500;color:var(--text-primary); }
.dep-desc { font-size:11px;color:var(--text-muted);margin-top:2px; }
.cell-version { font-family:"JetBrains Mono",monospace;font-size:11px;color:var(--text-muted); }
.version-text { color:var(--success);font-weight:500; }
.cell-action { width:160px; }
.ok-text { font-size:12px;color:var(--success);font-weight:500; }
.ic-ok { color:var(--success); }
.ic-err { color:var(--danger); }

.note-box { display:flex;gap:10px;align-items:flex-start;font-size:12px;color:var(--text-secondary);
  padding:10px 14px;background:color-mix(in srgb,var(--warning) 8%,transparent);
  border:1px solid color-mix(in srgb,var(--warning) 30%,transparent);border-radius:6px;line-height:1.6; }
</style>
