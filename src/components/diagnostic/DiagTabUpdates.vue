<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCw, Download, CheckCircle, AlertTriangle, Package } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";

const props = defineProps<{ updatesHistory: any[] }>();

// === État local ===
const scanningWU    = ref(false);
const pendingWU     = ref<any[]>([]);
const wuScanned     = ref(false);
const triggeringWU  = ref(false);
const wuMessage     = ref("");

const wingetList    = ref<any[]>([]);
const wingetLoading = ref(false);
const wingetUpgrading = ref(false);
const wingetMsg     = ref("");
const wingetOk      = ref(false);

const chocoList     = ref<any[]>([]);
const chocoLoading  = ref(false);
const chocoInstalled = ref(false);
const chocoUpgrading = ref(false);
const chocoMsg      = ref("");

const scoopList     = ref<any[]>([]);
const scoopLoading  = ref(false);
const scoopInstalled = ref(false);
const scoopUpgrading = ref(false);
const scoopMsg      = ref("");

// === Windows Update ===
async function scanWU() {
  scanningWU.value = true; wuScanned.value = false; wuMessage.value = "";
  try { pendingWU.value = await invoke("scan_pending_windows_updates"); }
  catch { pendingWU.value = []; }
  wuScanned.value = true; scanningWU.value = false;
}
async function triggerWU() {
  triggeringWU.value = true;
  try { wuMessage.value = await invoke("trigger_windows_update"); }
  catch { wuMessage.value = "Erreur lors du déclenchement"; }
  triggeringWU.value = false;
}

// === Winget ===
async function loadWinget() {
  wingetLoading.value = true; wingetMsg.value = "";
  try {
    wingetOk.value = await invoke("check_winget");
    if (wingetOk.value) wingetList.value = await invoke("list_upgradable");
  } catch { wingetOk.value = false; }
  wingetLoading.value = false;
}
async function upgradeWinget() {
  wingetUpgrading.value = true; wingetMsg.value = "Mise à jour en cours...";
  try {
    await invoke("upgrade_all");
    wingetMsg.value = "Mise à jour winget terminée ✓";
    await loadWinget();
  } catch (e: any) { wingetMsg.value = "Erreur : " + e; }
  wingetUpgrading.value = false;
}

// === Chocolatey ===
async function loadChoco() {
  chocoLoading.value = true; chocoMsg.value = "";
  try {
    chocoInstalled.value = await invoke("check_chocolatey");
    if (chocoInstalled.value) chocoList.value = await invoke("list_chocolatey_upgrades");
  } catch { chocoInstalled.value = false; }
  chocoLoading.value = false;
}
async function upgradeChoco() {
  chocoUpgrading.value = true; chocoMsg.value = "Mise à jour en cours...";
  try {
    const r: any = await invoke("upgrade_chocolatey_all");
    chocoMsg.value = r?.success ? `${r.upgraded_count} paquet(s) mis à jour ✓` : `Erreur : ${r?.error || "inconnue"}`;
    await loadChoco();
  } catch (e: any) { chocoMsg.value = "Erreur : " + e; }
  chocoUpgrading.value = false;
}

// === Scoop ===
async function loadScoop() {
  scoopLoading.value = true; scoopMsg.value = "";
  try {
    scoopInstalled.value = await invoke("check_scoop");
    if (scoopInstalled.value) scoopList.value = await invoke("list_scoop_upgrades");
  } catch { scoopInstalled.value = false; }
  scoopLoading.value = false;
}
async function upgradeScoop() {
  scoopUpgrading.value = true; scoopMsg.value = "Mise à jour en cours...";
  try {
    await invoke("upgrade_scoop_all");
    scoopMsg.value = "Mise à jour Scoop terminée ✓";
    await loadScoop();
  } catch (e: any) { scoopMsg.value = "Erreur : " + e; }
  scoopUpgrading.value = false;
}

function severityVariant(s: string) {
  const l = s.toLowerCase();
  if (l.includes("critical")) return "danger";
  if (l.includes("important")) return "warning";
  if (l.includes("moderate") || l.includes("low")) return "info";
  return "neutral";
}

onMounted(() => { loadWinget(); loadChoco(); loadScoop(); });
</script>

<template>
  <!-- ===== Windows Update ===== -->
  <p class="diag-section-label">Windows Update — Mises à jour disponibles</p>
  <div class="card-block">
    <div style="display:flex;gap:8px;flex-wrap:wrap;align-items:center;margin-bottom:10px">
      <NButton variant="primary" size="sm" :disabled="scanningWU" @click="scanWU">
        <NSpinner v-if="scanningWU" :size="12" />
        <RefreshCw v-else :size="13" />
        {{ scanningWU ? 'Scan en cours...' : 'Scanner les mises à jour' }}
      </NButton>
      <NButton v-if="wuScanned && pendingWU.length > 0" variant="success" size="sm"
        :disabled="triggeringWU" @click="triggerWU">
        <Download :size="13" />
        {{ triggeringWU ? 'Déclenchement...' : 'Installer toutes (' + pendingWU.length + ')' }}
      </NButton>
      <span v-if="wuMessage" class="muted" style="font-size:12px">{{ wuMessage }}</span>
    </div>

    <div v-if="!wuScanned && !scanningWU" class="diag-empty" style="padding:8px 0">
      Cliquez sur "Scanner" pour détecter les mises à jour Windows disponibles (utilise le cache local)
    </div>
    <div v-else-if="wuScanned && !pendingWU.length" style="display:flex;align-items:center;gap:8px;color:var(--success);font-size:13px">
      <CheckCircle :size="14" /> Windows est à jour — aucune mise à jour en attente
    </div>
    <div v-else-if="pendingWU.length" class="table-wrap">
      <table class="data-table">
        <thead><tr><th>Titre</th><th>KB</th><th>Sévérité</th><th>Taille</th><th>Téléchargé</th></tr></thead>
        <tbody>
          <tr v-for="(u, i) in pendingWU" :key="i">
            <td style="max-width:300px;white-space:normal;word-break:break-word;font-size:11px">{{ u.title }}</td>
            <td><code style="font-size:10px">{{ u.kb_ids || "—" }}</code></td>
            <td><NBadge :variant="severityVariant(u.severity)" style="font-size:10px">{{ u.severity }}</NBadge></td>
            <td class="muted" style="font-size:11px">{{ u.size_mb > 0 ? u.size_mb.toFixed(0)+' MB' : '—' }}</td>
            <td>
              <CheckCircle v-if="u.is_downloaded" :size="13" class="ic-ok" />
              <span v-else class="muted" style="font-size:11px">Non</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>

  <!-- ===== Winget ===== -->
  <p class="diag-section-label">WinGet — Gestionnaire de paquets Windows</p>
  <div class="card-block">
    <div v-if="wingetLoading" style="display:flex;align-items:center;gap:8px;font-size:13px"><NSpinner :size="14" /> Chargement...</div>
    <div v-else-if="!wingetOk" class="diag-empty">WinGet non détecté sur ce système</div>
    <template v-else>
      <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap;margin-bottom:10px">
        <NButton variant="ghost" size="sm" :disabled="wingetLoading" @click="loadWinget">
          <RefreshCw :size="12" /> Actualiser
        </NButton>
        <NButton v-if="wingetList.length > 0" variant="primary" size="sm"
          :disabled="wingetUpgrading" @click="upgradeWinget">
          <Download v-if="!wingetUpgrading" :size="13" />
          <NSpinner v-else :size="12" />
          {{ wingetUpgrading ? 'Mise à jour...' : 'Tout mettre à jour (' + wingetList.length + ')' }}
        </NButton>
        <span v-if="wingetMsg" class="muted" style="font-size:12px">{{ wingetMsg }}</span>
      </div>
      <div v-if="!wingetList.length" style="display:flex;align-items:center;gap:8px;color:var(--success);font-size:13px">
        <CheckCircle :size="14" /> Tous les paquets winget sont à jour
      </div>
      <div v-else class="table-wrap">
        <table class="data-table">
          <thead><tr><th>Nom</th><th>ID</th><th>Installé</th><th>Disponible</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in wingetList.slice(0, 50)" :key="i">
              <td>{{ p.name || p.Name || "—" }}</td>
              <td><code style="font-size:10px">{{ p.id || p.Id || "—" }}</code></td>
              <td class="muted" style="font-size:11px">{{ p.version || p.Version || "—" }}</td>
              <td><NBadge variant="info" style="font-size:10px">{{ p.available || p.Available || "—" }}</NBadge></td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>

  <!-- ===== Chocolatey ===== -->
  <p class="diag-section-label">Chocolatey</p>
  <div class="card-block">
    <div v-if="chocoLoading" style="display:flex;align-items:center;gap:8px;font-size:13px"><NSpinner :size="14" /> Chargement...</div>
    <div v-else-if="!chocoInstalled" class="diag-empty">Chocolatey non installé sur ce système</div>
    <template v-else>
      <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap;margin-bottom:10px">
        <NButton variant="ghost" size="sm" @click="loadChoco"><RefreshCw :size="12" /> Actualiser</NButton>
        <NButton v-if="chocoList.length > 0" variant="primary" size="sm"
          :disabled="chocoUpgrading" @click="upgradeChoco">
          <Download v-if="!chocoUpgrading" :size="13" />
          <NSpinner v-else :size="12" />
          {{ chocoUpgrading ? 'Mise à jour...' : 'Tout mettre à jour (' + chocoList.length + ')' }}
        </NButton>
        <span v-if="chocoMsg" class="muted" style="font-size:12px">{{ chocoMsg }}</span>
      </div>
      <div v-if="!chocoList.length" style="display:flex;align-items:center;gap:8px;color:var(--success);font-size:13px">
        <CheckCircle :size="14" /> Tous les paquets Chocolatey sont à jour
      </div>
      <div v-else class="table-wrap">
        <table class="data-table">
          <thead><tr><th>Paquet</th><th>Installé</th><th>Disponible</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in chocoList" :key="i">
              <td><Package :size="12" style="margin-right:4px;opacity:.6" />{{ typeof p === 'string' ? p.split('|')[0] : (p.name || p) }}</td>
              <td class="muted" style="font-size:11px">{{ typeof p === 'string' ? p.split('|')[1] : (p.current_version || "—") }}</td>
              <td><NBadge variant="info" style="font-size:10px">{{ typeof p === 'string' ? p.split('|')[2] : (p.available_version || "—") }}</NBadge></td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>

  <!-- ===== Scoop ===== -->
  <p class="diag-section-label">Scoop</p>
  <div class="card-block">
    <div v-if="scoopLoading" style="display:flex;align-items:center;gap:8px;font-size:13px"><NSpinner :size="14" /> Chargement...</div>
    <div v-else-if="!scoopInstalled" class="diag-empty">Scoop non installé sur ce système</div>
    <template v-else>
      <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap;margin-bottom:10px">
        <NButton variant="ghost" size="sm" @click="loadScoop"><RefreshCw :size="12" /> Actualiser</NButton>
        <NButton v-if="scoopList.length > 0" variant="primary" size="sm"
          :disabled="scoopUpgrading" @click="upgradeScoop">
          <Download v-if="!scoopUpgrading" :size="13" />
          <NSpinner v-else :size="12" />
          {{ scoopUpgrading ? 'Mise à jour...' : 'Tout mettre à jour (' + scoopList.length + ')' }}
        </NButton>
        <span v-if="scoopMsg" class="muted" style="font-size:12px">{{ scoopMsg }}</span>
      </div>
      <div v-if="!scoopList.length" style="display:flex;align-items:center;gap:8px;color:var(--success);font-size:13px">
        <CheckCircle :size="14" /> Tous les paquets Scoop sont à jour
      </div>
      <div v-else class="table-wrap">
        <table class="data-table">
          <thead><tr><th>Paquet</th><th>Installé</th><th>Disponible</th></tr></thead>
          <tbody>
            <tr v-for="(p, i) in scoopList" :key="i">
              <td>{{ p.name || p.Name || p }}</td>
              <td class="muted" style="font-size:11px">{{ p.installed || p.Installed || "—" }}</td>
              <td><NBadge variant="info" style="font-size:10px">{{ p.available || p.Available || "—" }}</NBadge></td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>

  <!-- ===== Historique KB ===== -->
  <p class="diag-section-label">Historique des mises à jour installées — {{ updatesHistory.length }}</p>
  <div v-if="!updatesHistory.length" class="diag-empty">Chargement de l'historique...</div>
  <div v-else class="table-wrap">
    <table class="data-table">
      <thead><tr><th>KB ID</th><th>Description</th><th>Installé le</th><th>Par</th></tr></thead>
      <tbody>
        <tr v-for="(u, i) in updatesHistory.slice(0, 200)" :key="i">
          <td><code>{{ u.hotfix_id }}</code></td>
          <td>{{ u.description }}</td>
          <td class="muted">{{ u.installed_on }}</td>
          <td class="muted">{{ u.installed_by || "—" }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
