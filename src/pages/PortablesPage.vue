<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import { useNotificationStore } from "@/stores/notifications";
import { PORTABLE_APPS, PORTABLE_CATEGORIES } from "@/data/portableApps";
import type { PortableApp } from "@/data/portableApps";
import {
  Package, Download, Play, FolderOpen, Star,
  HardDrive, FileCode, Globe, Wrench, Film,
  FileText, Trash2, Shield, Image, RefreshCw, Database,
} from "lucide-vue-next";

const notify = useNotificationStore();

const installedMap  = ref<Record<string, boolean>>({});
const search        = ref("");
const activeCategory = ref("Tous");
const showPopular   = ref(false);
const loading       = ref(false);

// ── Catalogue local enrichi ────────────────────────────────────
const apps = ref<PortableApp[]>(PORTABLE_APPS);

// ── Icônes par catégorie ──────────────────────────────────────
const CAT_ICONS: Record<string, any> = {
  Système:       HardDrive,
  Réseau:        Globe,
  Développement: FileCode,
  Utilitaires:   Wrench,
  Multimédia:    Film,
  Bureautique:   FileText,
  Nettoyage:     Trash2,
  Sécurité:      Shield,
  Graphisme:     Image,
  Récupération:  Database,
};
const catIcon = (cat: string) => CAT_ICONS[cat] ?? Package;

// ── Compteurs par catégorie ───────────────────────────────────
const categoryCounts = computed(() => {
  const counts: Record<string, number> = { Tous: apps.value.length };
  for (const a of apps.value) {
    counts[a.category] = (counts[a.category] ?? 0) + 1;
  }
  return counts;
});

// ── Filtres ───────────────────────────────────────────────────
const filteredApps = computed(() => {
  let result = apps.value;
  if (activeCategory.value !== "Tous") {
    result = result.filter((a) => a.category === activeCategory.value);
  }
  if (showPopular.value) {
    result = result.filter((a) => a.popular);
  }
  const q = search.value.toLowerCase().trim();
  if (q) {
    result = result.filter(
      (a) =>
        a.name.toLowerCase().includes(q) ||
        a.description.toLowerCase().includes(q) ||
        a.category.toLowerCase().includes(q)
    );
  }
  return result;
});

const popularCount = computed(() => apps.value.filter((a) => a.popular).length);

// ── Actions ───────────────────────────────────────────────────
async function openDownload(app: PortableApp) {
  if (!app.url) {
    notify.warning(`Pas d'URL pour ${app.name}`);
    return;
  }
  try {
    await invoke("open_url", { url: app.url });
    notify.info(`Ouverture de la page de téléchargement pour ${app.name}`);
  } catch {
    notify.error("Impossible d'ouvrir le lien de téléchargement");
  }
}

async function launchApp(app: PortableApp) {
  try {
    await invoke("launch_portable", { appId: app.id });
    notify.success(`${app.name} lancé`);
  } catch (e: any) {
    notify.error(e?.toString() || `Impossible de lancer ${app.name}`);
  }
}

async function openPortablesFolder() {
  try {
    await invoke("open_portables_dir");
  } catch {
    notify.error("Impossible d'ouvrir le dossier");
  }
}

async function refreshInstalled() {
  loading.value = true;
  const map: Record<string, boolean> = {};
  for (const app of apps.value) {
    try {
      map[app.id] = await invoke<boolean>("check_portable_installed", { appId: app.id });
    } catch {
      map[app.id] = false;
    }
  }
  installedMap.value = map;
  loading.value = false;
}
</script>

<template>
  <div class="portables-page">

    <!-- En-tête ─────────────────────────────────────── -->
    <div class="port-header">
      <div>
        <h1><Package :size="20" /> Applications Portables</h1>
        <p class="port-subtitle">
          {{ apps.length }} applications • {{ popularCount }} recommandées — aucune installation requise
        </p>
      </div>
      <div class="port-header-actions">
        <NButton variant="secondary" size="sm" @click="openPortablesFolder">
          <FolderOpen :size="13" /> Dossier logiciel/
        </NButton>
        <NButton variant="ghost" size="sm" @click="refreshInstalled" :loading="loading">
          <RefreshCw :size="13" /> Vérifier installées
        </NButton>
      </div>
    </div>

    <!-- Filtres ─────────────────────────────────────── -->
    <div class="port-filters">
      <NSearchBar v-model="search" placeholder="Rechercher parmi {{ apps.length }} applications…" />
      <div class="port-cats">
        <button
          v-for="cat in PORTABLE_CATEGORIES" :key="cat"
          class="port-cat-btn"
          :class="{ active: activeCategory === cat && !showPopular }"
          @click="activeCategory = cat; showPopular = false"
        >
          <component :is="cat === 'Tous' ? Package : catIcon(cat)" :size="12" />
          {{ cat }}
          <span class="port-cat-count">{{ categoryCounts[cat] ?? 0 }}</span>
        </button>
        <button
          class="port-cat-btn port-cat-popular"
          :class="{ active: showPopular }"
          @click="showPopular = !showPopular; if (showPopular) activeCategory = 'Tous'"
        >
          <Star :size="12" />
          Recommandées
          <span class="port-cat-count">{{ popularCount }}</span>
        </button>
      </div>
    </div>

    <!-- Résultats count ─────────────────────────────── -->
    <div class="port-results-bar">
      <span>{{ filteredApps.length }} application{{ filteredApps.length > 1 ? 's' : '' }}</span>
      <span v-if="search" class="port-clear-search" @click="search = ''">✕ Effacer</span>
    </div>

    <!-- Grille d'apps ───────────────────────────────── -->
    <div class="port-grid">
      <div
        v-for="app in filteredApps"
        :key="app.id"
        class="port-card"
        :class="{ 'port-card--installed': installedMap[app.id] }"
      >
        <div class="port-card-icon">
          <component :is="catIcon(app.category)" :size="22" />
          <Star v-if="app.popular" class="port-popular-star" :size="10" />
        </div>

        <div class="port-card-body">
          <div class="port-card-top">
            <span class="port-card-name">{{ app.name }}</span>
            <div class="port-card-badges">
              <NBadge v-if="installedMap[app.id]" variant="success" size="sm">✓ Installée</NBadge>
              <NBadge variant="neutral" size="sm">{{ app.category }}</NBadge>
            </div>
          </div>
          <p class="port-card-desc">{{ app.description }}</p>
          <div class="port-card-bottom">
            <span class="port-card-size">{{ app.size }}</span>
            <div class="port-card-actions">
              <NButton
                v-if="installedMap[app.id]"
                variant="primary"
                size="sm"
                @click="launchApp(app)"
              >
                <Play :size="12" /> Lancer
              </NButton>
              <NButton
                variant="secondary"
                size="sm"
                @click="openDownload(app)"
              >
                <Download :size="12" />
                {{ installedMap[app.id] ? 'MAJ' : 'Télécharger' }}
              </NButton>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- État vide ───────────────────────────────────── -->
    <div v-if="filteredApps.length === 0" class="port-empty">
      <Package :size="36" style="color:var(--text-muted);opacity:0.3" />
      <p>Aucune application ne correspond à <strong>{{ search || activeCategory }}</strong></p>
      <NButton variant="ghost" size="sm" @click="search=''; activeCategory='Tous'; showPopular=false">
        Réinitialiser les filtres
      </NButton>
    </div>

  </div>
</template>

<style scoped>
.portables-page {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

/* ── Header ────────────────────────────────── */
.port-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 10px;
}
.port-header h1 {
  font-size: 20px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
}
.port-subtitle { color: var(--text-muted); font-size: 12px; margin-top: 3px; }
.port-header-actions { display: flex; gap: 6px; }

/* ── Filtres ────────────────────────────────── */
.port-filters {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.port-cats {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}
.port-cat-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 5px 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--text-muted);
  font-family: inherit;
  font-size: 11.5px;
  cursor: pointer;
  transition: all var(--transition-fast);
}
.port-cat-btn:hover { background: var(--bg-tertiary); color: var(--text-secondary); }
.port-cat-btn.active {
  background: var(--accent-muted);
  color: var(--accent-primary);
  border-color: rgba(249,115,22,0.4);
}
.port-cat-count {
  background: var(--bg-elevated);
  color: var(--text-muted);
  font-size: 10px;
  padding: 1px 5px;
  border-radius: 99px;
  font-weight: 600;
  min-width: 18px;
  text-align: center;
}
.port-cat-btn.active .port-cat-count {
  background: rgba(249,115,22,0.2);
  color: var(--accent-primary);
}
.port-cat-popular.active {
  background: rgba(234, 179, 8, 0.1);
  color: #eab308;
  border-color: rgba(234, 179, 8, 0.4);
}
.port-cat-popular.active .port-cat-count {
  background: rgba(234, 179, 8, 0.2);
  color: #eab308;
}

/* ── Barre résultats ────────────────────────── */
.port-results-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 11.5px;
  color: var(--text-muted);
}
.port-clear-search {
  cursor: pointer;
  color: var(--accent-primary);
  font-weight: 500;
}
.port-clear-search:hover { text-decoration: underline; }

/* ── Grille ─────────────────────────────────── */
.port-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 8px;
}

/* ── Card ───────────────────────────────────── */
.port-card {
  display: flex;
  gap: 12px;
  padding: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  transition: border-color 0.15s, background 0.15s, transform 0.12s;
  cursor: default;
}
.port-card:hover {
  border-color: var(--border-hover);
  background: var(--bg-tertiary);
  transform: translateY(-1px);
}
.port-card--installed { border-color: rgba(74, 222, 128, 0.25); }

.port-card-icon {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  background: var(--accent-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  color: var(--accent-primary);
  position: relative;
}
.port-popular-star {
  position: absolute;
  top: -4px;
  right: -4px;
  background: #eab308;
  color: #000;
  border-radius: 50%;
  padding: 2px;
  width: 14px;
  height: 14px;
}

.port-card-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}
.port-card-top {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}
.port-card-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
}
.port-card-badges { display: flex; gap: 4px; flex-wrap: wrap; margin-left: auto; }
.port-card-desc {
  font-size: 11.5px;
  color: var(--text-muted);
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
.port-card-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 2px;
}
.port-card-size {
  font-size: 11px;
  color: var(--text-muted);
  font-family: "JetBrains Mono", monospace;
}
.port-card-actions { display: flex; gap: 4px; }

/* ── État vide ──────────────────────────────── */
.port-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 48px 24px;
  text-align: center;
  color: var(--text-muted);
}
.port-empty p { font-size: 14px; }
</style>
