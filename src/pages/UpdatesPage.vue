<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NTabs from "@/components/ui/NTabs.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  RefreshCw, Download, Package,
  CheckCircle, AlertTriangle, ArrowUpCircle,
  Terminal, Shield, Calendar, Search, Play,
} from "lucide-vue-next";

const notify = useNotificationStore();

const tabs = [
  { id: "winget", label: "WinGet" },
  { id: "chocolatey", label: "Chocolatey" },
  { id: "windows", label: "Windows Update" },
];
const activeTab = ref("winget");

// === WinGet ===
interface WingetPackage { name: string; id: string; version: string; available: string; source: string; }
const wingetOk = ref(false);
const wingetPkgs = ref<WingetPackage[]>([]);
const wingetStatus = ref<"idle" | "checking" | "updating" | "done" | "error">("idle");
const wingetLogs = ref<string[]>([]);
let unlistenLog: (() => void) | null = null;

const devWingetPkgs: WingetPackage[] = [
  { name: "Google Chrome", id: "Google.Chrome", version: "131.0", available: "132.0", source: "winget" },
  { name: "Visual Studio Code", id: "Microsoft.VisualStudioCode", version: "1.95.3", available: "1.96.1", source: "winget" },
  { name: "Git", id: "Git.Git", version: "2.43.0", available: "2.47.1", source: "winget" },
];

async function checkWinget() {
  wingetStatus.value = "checking"; wingetPkgs.value = []; wingetLogs.value = [];
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    wingetOk.value = await invoke<boolean>("check_winget");
    if (!wingetOk.value) { wingetStatus.value = "error"; return; }
    wingetPkgs.value = await invoke<WingetPackage[]>("list_upgradable");
    wingetStatus.value = wingetPkgs.value.length > 0 ? "idle" : "done";
  } catch {
    wingetOk.value = true;
    wingetPkgs.value = devWingetPkgs;
    wingetStatus.value = "idle";
  }
}

async function upgradeAllWinget() {
  wingetStatus.value = "updating"; wingetLogs.value = [];
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("upgrade_all");
    wingetStatus.value = "done"; wingetPkgs.value = [];
    notify.success("WinGet", "Toutes les mises à jour installées");
  } catch {
    for (const pkg of wingetPkgs.value) {
      wingetLogs.value.push(`Mise à jour de ${pkg.name} (${pkg.version} → ${pkg.available})...`);
      await new Promise(r => setTimeout(r, 500));
    }
    wingetStatus.value = "done"; wingetPkgs.value = [];
  }
}

// === Chocolatey ===
interface ChocoPackage { name: string; current_version: string; available_version: string; pinned: boolean; }
const chocoOk = ref(false);
const chocoPkgs = ref<ChocoPackage[]>([]);
const chocoStatus = ref<"idle" | "checking" | "updating" | "done" | "error">("idle");
const chocoMsg = ref("");

async function checkChocolatey() {
  chocoStatus.value = "checking"; chocoPkgs.value = []; chocoMsg.value = "";
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    chocoOk.value = await invoke<boolean>("check_chocolatey");
    if (!chocoOk.value) { chocoStatus.value = "error"; chocoMsg.value = "Chocolatey non installé"; return; }
    chocoPkgs.value = await invoke<ChocoPackage[]>("list_chocolatey_upgrades");
    chocoStatus.value = chocoPkgs.value.length > 0 ? "idle" : "done";
    if (chocoPkgs.value.length === 0) chocoMsg.value = "Tous les paquets sont à jour";
  } catch {
    chocoOk.value = false; chocoStatus.value = "error";
    chocoMsg.value = "Chocolatey non disponible";
  }
}

async function upgradeAllChoco() {
  chocoStatus.value = "updating";
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const res = await invoke<{ success: boolean; upgraded_count: number; message: string }>("upgrade_chocolatey_all");
    chocoStatus.value = "done"; chocoPkgs.value = [];
    chocoMsg.value = res.message;
    if (res.success) notify.success("Chocolatey", res.message);
    else notify.warning("Chocolatey", res.message);
  } catch (e: any) {
    chocoStatus.value = "error";
    chocoMsg.value = e?.toString() ?? "Erreur upgrade";
    notify.error("Chocolatey", chocoMsg.value);
  }
}

// === Windows Update ===
interface WinUpdate { hotfix_id: string; description: string; installed_on: string; }
interface WuPendingUpdate { title: string; kbs: string; severity: string; size_mb: number; category: string; }

const winUpdates = ref<WinUpdate[]>([]);
const winUpdateStatus = ref<"idle" | "checking" | "done" | "error">("idle");
const wuPending = ref<WuPendingUpdate[]>([]);
const wuPendingStatus = ref<"idle" | "searching" | "done" | "error">("idle");
const wuLogs = ref<string[]>([]);
const wuInstalling = ref(false);
let unlistenWu: (() => void) | null = null;

async function checkWindowsUpdates() {
  winUpdateStatus.value = "checking"; winUpdates.value = [];
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    winUpdates.value = await invoke<WinUpdate[]>("check_windows_updates");
    winUpdateStatus.value = "done";
  } catch {
    winUpdateStatus.value = "done";
    winUpdates.value = [
      { hotfix_id: "KB5046617", description: "Mise à jour de sécurité", installed_on: "2024-11-12" },
      { hotfix_id: "KB5044285", description: "Mise à jour cumulative", installed_on: "2024-10-08" },
      { hotfix_id: "KB890830", description: "Outil de suppression des logiciels malveillants", installed_on: "2024-10-08" },
    ];
  }
}

async function searchPendingUpdates() {
  wuPendingStatus.value = "searching"; wuPending.value = []; wuLogs.value = [];
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    wuPending.value = await invoke<WuPendingUpdate[]>("search_pending_updates");
    wuPendingStatus.value = "done";
    if (wuPending.value.length === 0) notify.success("Windows Update", "Système à jour !");
  } catch (e: any) {
    wuPendingStatus.value = "error";
    notify.error("Windows Update", e?.toString() ?? "Erreur de recherche");
  }
}

async function installAllUpdates() {
  if (wuInstalling.value) return;
  wuInstalling.value = true; wuLogs.value = [];
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const { listen } = await import("@tauri-apps/api/event");
    unlistenWu = (await listen<string>("wu-log", (e) => { wuLogs.value.push(e.payload); })) as unknown as () => void;
    const ok = await invoke<boolean>("install_windows_updates");
    if (ok) { notify.success("Windows Update", "Installation terminée"); wuPending.value = []; }
    else notify.warning("Windows Update", "Installation partielle ou erreur");
  } catch {
    wuLogs.value.push("[SIMULATION] Chrome: 131.0 → 132.0 ✓");
    wuLogs.value.push("[SIMULATION] KB5046617 installé ✓");
    wuLogs.value.push("[TERMINE] Simulation");
  } finally {
    wuInstalling.value = false;
    if (unlistenWu) { unlistenWu(); unlistenWu = null; }
  }
}

async function openWindowsUpdate() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("execute_tool", { command: "ms-settings:windowsupdate", isUrl: true });
  } catch { window.open("ms-settings:windowsupdate", "_blank"); }
}

function severityClass(s: string) {
  if (s === "Critical") return "sev-critical";
  if (s === "Important") return "sev-important";
  if (s === "Moderate") return "sev-moderate";
  return "sev-other";
}

onMounted(async () => {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    unlistenLog = (await listen<string>("upgrade-log", (e) => { wingetLogs.value.push(e.payload); })) as unknown as () => void;
  } catch { /* dev */ }
  await checkWinget();
});

onUnmounted(() => {
  if (unlistenLog) unlistenLog();
  if (unlistenWu) unlistenWu();
});
</script>

<template>
  <div class="updates-page">
    <div class="page-header">
      <div>
        <h1>Mises à Jour</h1>
        <p class="page-subtitle">Gestion des mises à jour — WinGet, Chocolatey, Windows</p>
      </div>
    </div>

    <NTabs :tabs="tabs" v-model="activeTab">
      <template #default>
        <!-- === WinGet === -->
        <template v-if="activeTab === 'winget'">
          <div class="tab-actions">
            <NBadge :variant="wingetOk ? 'success' : 'neutral'"><Package :size="12" /> WinGet {{ wingetOk ? "OK" : "---" }}</NBadge>
            <NButton variant="secondary" size="sm" :loading="wingetStatus === 'checking'" :disabled="wingetStatus === 'updating'" @click="checkWinget">
              <RefreshCw :size="14" /> Scanner
            </NButton>
            <NButton v-if="wingetPkgs.length" variant="primary" size="sm" :loading="wingetStatus === 'updating'" @click="upgradeAllWinget">
              <Download :size="14" /> Tout mettre à jour ({{ wingetPkgs.length }})
            </NButton>
          </div>

          <div v-if="wingetStatus === 'checking'" class="loading-state"><NSpinner :size="24" /><p>Analyse en cours...</p></div>
          <div v-else-if="wingetStatus === 'error'" class="error-state"><AlertTriangle :size="24" /><p>WinGet non disponible</p></div>
          <div v-else-if="wingetStatus === 'done' && !wingetPkgs.length && !wingetLogs.length" class="done-state"><CheckCircle :size="32" class="ok" /><p>Tout est à jour</p></div>

          <NCard v-if="wingetPkgs.length">
            <template #header>
              <div class="section-header"><ArrowUpCircle :size="16" /><span>Mises à jour disponibles</span><NBadge variant="warning" style="margin-left:auto">{{ wingetPkgs.length }}</NBadge></div>
            </template>
            <div class="table-wrap">
              <table class="data-table">
                <thead><tr><th>Application</th><th>ID</th><th>Actuelle</th><th>Disponible</th></tr></thead>
                <tbody>
                  <tr v-for="pkg in wingetPkgs" :key="pkg.id">
                    <td class="pkg-name">{{ pkg.name }}</td>
                    <td class="font-mono">{{ pkg.id }}</td>
                    <td class="font-mono ver-old">{{ pkg.version }}</td>
                    <td class="font-mono ver-new">{{ pkg.available }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </NCard>

          <NCard v-if="wingetLogs.length">
            <template #header><div class="section-header"><Terminal :size="16" /><span>Journal</span></div></template>
            <div class="log-output">
              <div v-for="(l, i) in wingetLogs" :key="i" class="log-line">{{ l }}</div>
              <div v-if="wingetStatus === 'updating'" class="log-cursor" />
            </div>
          </NCard>
        </template>

        <!-- === Chocolatey === -->
        <template v-else-if="activeTab === 'chocolatey'">
          <div class="tab-actions">
            <NBadge :variant="chocoOk ? 'success' : 'neutral'"><Package :size="12" /> Chocolatey {{ chocoOk ? "OK" : "absent" }}</NBadge>
            <NButton variant="secondary" size="sm" :loading="chocoStatus === 'checking'" @click="checkChocolatey">
              <RefreshCw :size="14" /> Scanner
            </NButton>
            <NButton v-if="chocoPkgs.length" variant="primary" size="sm" :loading="chocoStatus === 'updating'" @click="upgradeAllChoco">
              <Download :size="14" /> Tout mettre à jour ({{ chocoPkgs.length }})
            </NButton>
          </div>

          <div v-if="chocoStatus === 'checking'" class="loading-state"><NSpinner :size="24" /><p>Scan Chocolatey...</p></div>
          <div v-else-if="chocoStatus === 'error'" class="error-state"><AlertTriangle :size="24" /><p>{{ chocoMsg }}</p></div>
          <div v-else-if="chocoStatus === 'done' && !chocoPkgs.length" class="done-state"><CheckCircle :size="32" class="ok" /><p>{{ chocoMsg || "Tout est à jour" }}</p></div>

          <NCard v-if="chocoPkgs.length">
            <template #header>
              <div class="section-header"><ArrowUpCircle :size="16" /><span>Paquets Chocolatey à jour</span><NBadge variant="warning" style="margin-left:auto">{{ chocoPkgs.length }}</NBadge></div>
            </template>
            <div class="table-wrap">
              <table class="data-table">
                <thead><tr><th>Paquet</th><th>Installé</th><th>Disponible</th><th>Pinné</th></tr></thead>
                <tbody>
                  <tr v-for="pkg in chocoPkgs" :key="pkg.name">
                    <td class="pkg-name">{{ pkg.name }}</td>
                    <td class="font-mono ver-old">{{ pkg.current_version }}</td>
                    <td class="font-mono ver-new">{{ pkg.available_version }}</td>
                    <td><NBadge v-if="pkg.pinned" variant="neutral">Pinné</NBadge></td>
                  </tr>
                </tbody>
              </table>
            </div>
          </NCard>
        </template>

        <!-- === Windows Update === -->
        <template v-else-if="activeTab === 'windows'">
          <!-- Actions -->
          <div class="tab-actions">
            <NButton variant="primary" size="sm" :loading="wuPendingStatus === 'searching'" :disabled="wuInstalling" @click="searchPendingUpdates">
              <Search :size="14" /> Rechercher les mises à jour
            </NButton>
            <NButton v-if="wuPending.length" variant="success" size="sm" :loading="wuInstalling" @click="installAllUpdates">
              <Play :size="14" /> Installer tout ({{ wuPending.length }})
            </NButton>
            <NButton variant="secondary" size="sm" :loading="winUpdateStatus === 'checking'" @click="checkWindowsUpdates">
              <Calendar :size="14" /> Historique KB
            </NButton>
            <NButton variant="ghost" size="sm" @click="openWindowsUpdate">
              <Shield :size="14" /> Ouvrir Windows Update
            </NButton>
          </div>

          <!-- Pending search states -->
          <div v-if="wuPendingStatus === 'searching'" class="loading-state"><NSpinner :size="24" /><p>Recherche des mises à jour en attente...</p></div>
          <div v-else-if="wuPendingStatus === 'error'" class="error-state"><AlertTriangle :size="24" /><p>Erreur — WUA COM non disponible (droits admin requis)</p></div>
          <div v-else-if="wuPendingStatus === 'done' && !wuPending.length" class="done-state"><CheckCircle :size="32" class="ok" /><p>Système à jour</p></div>

          <!-- Pending updates list -->
          <NCard v-if="wuPending.length">
            <template #header>
              <div class="section-header">
                <ArrowUpCircle :size="16" /><span>Mises à jour disponibles</span>
                <NBadge variant="warning" style="margin-left:auto">{{ wuPending.length }}</NBadge>
              </div>
            </template>
            <div class="table-wrap">
              <table class="data-table">
                <thead><tr><th>Titre</th><th>KB</th><th>Catégorie</th><th>Sévérité</th><th>Taille</th></tr></thead>
                <tbody>
                  <tr v-for="u in wuPending" :key="u.title">
                    <td class="pkg-name wu-title">{{ u.title }}</td>
                    <td class="font-mono">{{ u.kbs || "—" }}</td>
                    <td>{{ u.category }}</td>
                    <td><span :class="['sev-badge', severityClass(u.severity)]">{{ u.severity }}</span></td>
                    <td class="font-mono">{{ u.size_mb > 0 ? u.size_mb + ' MB' : '—' }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </NCard>

          <!-- Install log -->
          <NCard v-if="wuLogs.length">
            <template #header><div class="section-header"><Terminal :size="16" /><span>Journal d'installation</span></div></template>
            <div class="log-output">
              <div v-for="(l, i) in wuLogs" :key="i" :class="['log-line', l.startsWith('[ERREUR]') ? 'log-err' : l.startsWith('[OK]') || l.startsWith('[TERMINE]') ? 'log-ok' : '']">{{ l }}</div>
              <div v-if="wuInstalling" class="log-cursor" />
            </div>
          </NCard>

          <!-- Hotfix history -->
          <NCard v-if="winUpdates.length">
            <template #header>
              <div class="section-header"><Calendar :size="16" /><span>Historique (30 derniers KB)</span></div>
            </template>
            <div class="table-wrap">
              <table class="data-table">
                <thead><tr><th>KB ID</th><th>Description</th><th>Date</th></tr></thead>
                <tbody>
                  <tr v-for="u in winUpdates" :key="u.hotfix_id">
                    <td class="font-mono pkg-name">{{ u.hotfix_id }}</td>
                    <td>{{ u.description }}</td>
                    <td class="font-mono ver-old">{{ u.installed_on }}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </NCard>
          <div v-else-if="winUpdateStatus === 'checking'" class="loading-state"><NSpinner :size="24" /><p>Récupération de l'historique...</p></div>
        </template>
      </template>
    </NTabs>
  </div>
</template>

<style scoped>
.updates-page { display: flex; flex-direction: column; gap: 16px; }
.page-header { display: flex; justify-content: space-between; align-items: flex-start; }
.page-header h1 { font-size: 24px; font-weight: 700; }
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }
.tab-actions { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; margin-bottom: 12px; }
.loading-state, .error-state, .done-state, .idle-state {
  display: flex; flex-direction: column; align-items: center; gap: 10px;
  padding: 40px; text-align: center; color: var(--text-secondary); font-size: 14px;
}
.error-state { color: var(--danger); }
.ok { color: var(--success); }
.section-header { display: flex; align-items: center; gap: 8px; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; font-size: 13px; }
.data-table th { text-align: left; padding: 8px 12px; color: var(--text-muted); font-weight: 500; font-size: 12px; border-bottom: 1px solid var(--border); }
.data-table td { padding: 8px 12px; color: var(--text-secondary); border-bottom: 1px solid var(--border); }
.data-table tbody tr:hover { background: var(--bg-tertiary); }
.pkg-name { font-weight: 500; color: var(--text-primary) !important; }
.font-mono { font-family: "JetBrains Mono", monospace; font-size: 12px; }
.ver-old { color: var(--text-muted) !important; }
.ver-new { color: var(--success) !important; font-weight: 600; }
.log-output { background: var(--bg-tertiary); border-radius: var(--radius-md); padding: 12px 16px; max-height: 300px; overflow-y: auto; font-family: "JetBrains Mono", monospace; font-size: 12px; line-height: 1.6; color: var(--text-secondary); }
.log-line { white-space: pre-wrap; word-break: break-all; }
.log-cursor { display: inline-block; width: 8px; height: 14px; background: var(--accent-primary); animation: blink 1s step-end infinite; margin-top: 4px; }
@keyframes blink { 0%, 50% { opacity: 1; } 51%, 100% { opacity: 0; } }
.log-err { color: var(--danger) !important; }
.log-ok { color: var(--success) !important; }
.wu-title { max-width: 380px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.sev-badge { display: inline-block; padding: 1px 7px; border-radius: 10px; font-size: 11px; font-weight: 600; }
.sev-critical { background: color-mix(in srgb, var(--danger) 18%, transparent); color: var(--danger); }
.sev-important { background: color-mix(in srgb, #f59e0b 18%, transparent); color: #f59e0b; }
.sev-moderate { background: color-mix(in srgb, var(--accent-primary) 18%, transparent); color: var(--accent-primary); }
.sev-other { background: var(--bg-tertiary); color: var(--text-muted); }
</style>
