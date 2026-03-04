<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import NBadge from "@/components/ui/NBadge.vue";
import {
  ScrollText, RefreshCw, Trash2, ArrowDownToLine,
  Filter, Info, AlertTriangle, XCircle,
} from "lucide-vue-next";

interface LogEntry {
  timestamp: string;
  level: "INFO" | "WARN" | "ERROR";
  message: string;
}

const logs = ref<LogEntry[]>([]);
const search = ref("");
const activeFilter = ref<"ALL" | "INFO" | "WARN" | "ERROR">("ALL");
const autoScroll = ref(true);
const outputEl = ref<HTMLDivElement | null>(null);
const activeTab = ref<"app" | "windows">("app");

// Windows Event Logs
interface WinEventEntry {
  id: number;
  level: string;
  source: string;
  timestamp: string;
  message: string;
}

const winEvents = ref<WinEventEntry[]>([]);
const winLogName = ref("System");
const winEventsLoading = ref(false);

const filteredWinEvents = computed(() => {
  let result = winEvents.value;
  if (activeFilter.value !== "ALL") {
    const map: Record<string, string> = { "INFO": "Information", "WARN": "Warning", "ERROR": "Error" };
    const target = map[activeFilter.value];
    if (target) result = result.filter(e => e.level === target);
  }
  const q = search.value.toLowerCase();
  if (q) {
    result = result.filter(e => e.message.toLowerCase().includes(q) || e.source.toLowerCase().includes(q));
  }
  return result;
});

async function loadWinEvents() {
  winEventsLoading.value = true;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    winEvents.value = await invoke<WinEventEntry[]>("get_event_logs", { logName: winLogName.value, count: 100 });
  } catch {
    winEvents.value = [
      { id: 1001, level: "Error", source: "BugCheck", timestamp: "2026-03-01 09:15:22", message: "Le systeme a redemarrage sans arret propre." },
      { id: 7036, level: "Information", source: "Service Control Manager", timestamp: "2026-03-01 10:00:01", message: "Le service Windows Update est entre dans l'etat arrete." },
      { id: 41, level: "Warning", source: "Kernel-Power", timestamp: "2026-03-01 08:30:00", message: "Le systeme a redemarrage sans arret propre prealable." },
    ];
  } finally {
    winEventsLoading.value = false;
  }
}

function winLevelVariant(level: string): "info" | "warning" | "danger" {
  if (level === "Error" || level === "Critical") return "danger";
  if (level === "Warning") return "warning";
  return "info";
}

const filteredLogs = computed(() => {
  let result = logs.value;
  if (activeFilter.value !== "ALL") {
    result = result.filter((l) => l.level === activeFilter.value);
  }
  const q = search.value.toLowerCase();
  if (q) {
    result = result.filter((l) => l.message.toLowerCase().includes(q) || l.timestamp.includes(q));
  }
  return result;
});

function scrollToBottom() {
  if (!autoScroll.value) return;
  nextTick(() => {
    if (outputEl.value) outputEl.value.scrollTop = outputEl.value.scrollHeight;
  });
}

watch(() => filteredLogs.value.length, scrollToBottom);

function levelVariant(level: string): "info" | "warning" | "danger" {
  if (level === "WARN") return "warning";
  if (level === "ERROR") return "danger";
  return "info";
}

function clearLogs() {
  logs.value = [];
}

const devLogs: LogEntry[] = [
  { timestamp: "2026-03-01T10:00:00", level: "INFO", message: "Application NiTriTe demarree" },
  { timestamp: "2026-03-01T10:00:01", level: "INFO", message: "Chargement des modules systeme..." },
  { timestamp: "2026-03-01T10:00:02", level: "INFO", message: "Module diagnostique initialise" },
  { timestamp: "2026-03-01T10:00:03", level: "INFO", message: "Module reseau initialise" },
  { timestamp: "2026-03-01T10:00:05", level: "WARN", message: "Mise a jour disponible: v1.1 -> v1.2" },
  { timestamp: "2026-03-01T10:00:10", level: "INFO", message: "Scan systeme termine: 0 probleme detecte" },
  { timestamp: "2026-03-01T10:01:00", level: "INFO", message: "Requete get_system_info executee en 1.2s" },
  { timestamp: "2026-03-01T10:01:15", level: "WARN", message: "Usage CPU eleve detecte: 87%" },
  { timestamp: "2026-03-01T10:02:00", level: "ERROR", message: "Echec connexion au service Windows Update" },
  { timestamp: "2026-03-01T10:02:30", level: "INFO", message: "Nouvelle tentative de connexion..." },
  { timestamp: "2026-03-01T10:02:35", level: "INFO", message: "Connexion Windows Update retablie" },
  { timestamp: "2026-03-01T10:03:00", level: "INFO", message: "Verification des pilotes terminee" },
  { timestamp: "2026-03-01T10:03:45", level: "WARN", message: "Espace disque faible sur D:\\ (< 10 GB)" },
  { timestamp: "2026-03-01T10:04:00", level: "ERROR", message: "Impossible de lire le fichier de config: permission refusee" },
  { timestamp: "2026-03-01T10:05:00", level: "INFO", message: "Nettoyage fichiers temporaires: 450 MB liberes" },
  { timestamp: "2026-03-01T10:06:00", level: "INFO", message: "Sauvegarde automatique effectuee" },
  { timestamp: "2026-03-01T10:07:30", level: "WARN", message: "Pilote nvlddmkm.sys date de plus de 6 mois" },
  { timestamp: "2026-03-01T10:08:00", level: "INFO", message: "Scan antivirus rapide termine: aucune menace" },
];

async function loadLogs() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const result = await invoke<LogEntry[]>("get_app_logs");
    logs.value = result;
  } catch {
    logs.value = [...devLogs];
  }
  scrollToBottom();
}

onMounted(loadLogs);
</script>

<template>
  <div class="logs-page">
    <div class="page-header">
      <div>
        <h1>Logs</h1>
        <p class="page-subtitle">Journal d'activite de l'application</p>
      </div>
      <div class="header-actions">
        <NButton
          :variant="autoScroll ? 'primary' : 'secondary'"
          size="sm"
          @click="autoScroll = !autoScroll"
        >
          <ArrowDownToLine :size="14" />
          Auto-scroll
        </NButton>
        <NButton variant="secondary" size="sm" @click="clearLogs">
          <Trash2 :size="14" />
          Effacer
        </NButton>
        <NButton variant="primary" size="sm" @click="loadLogs">
          <RefreshCw :size="14" />
          Rafraichir
        </NButton>
      </div>
    </div>

    <!-- Tab selector -->
    <div class="tab-row">
      <button class="tab-btn" :class="{ active: activeTab === 'app' }" @click="activeTab = 'app'">Logs Application</button>
      <button class="tab-btn" :class="{ active: activeTab === 'windows' }" @click="activeTab = 'windows'; if (winEvents.length === 0) loadWinEvents()">Logs Windows</button>
    </div>

    <!-- Filters -->
    <div class="filters-row">
      <NSearchBar v-model="search" placeholder="Rechercher dans les logs..." />
      <div v-if="activeTab === 'windows'" class="win-log-select">
        <select v-model="winLogName" @change="loadWinEvents" class="log-select">
          <option value="System">System</option>
          <option value="Application">Application</option>
          <option value="Security">Security</option>
          <option value="Setup">Setup</option>
        </select>
      </div>
      <div class="level-filters">
        <button
          v-for="level in (['ALL', 'INFO', 'WARN', 'ERROR'] as const)"
          :key="level"
          class="filter-btn"
          :class="{ active: activeFilter === level }"
          @click="activeFilter = level"
        >
          <Filter v-if="level === 'ALL'" :size="12" />
          <Info v-else-if="level === 'INFO'" :size="12" />
          <AlertTriangle v-else-if="level === 'WARN'" :size="12" />
          <XCircle v-else :size="12" />
          {{ level === "ALL" ? "Tous" : level }}
        </button>
      </div>
    </div>

    <!-- App Logs -->
    <NCard v-if="activeTab === 'app'" padding="none">
      <template #header>
        <div class="section-header">
          <ScrollText :size="16" />
          <span>Entrees ({{ filteredLogs.length }})</span>
        </div>
      </template>
      <div class="logs-output" ref="outputEl">
        <div
          v-for="(log, i) in filteredLogs"
          :key="i"
          class="log-line"
          :class="`log-${log.level.toLowerCase()}`"
        >
          <span class="log-time">{{ log.timestamp.split("T")[1]?.substring(0, 8) ?? log.timestamp }}</span>
          <NBadge :variant="levelVariant(log.level)" class="log-badge">{{ log.level }}</NBadge>
          <span class="log-msg">{{ log.message }}</span>
        </div>
        <div v-if="filteredLogs.length === 0" class="logs-empty">
          Aucun log correspondant
        </div>
      </div>
    </NCard>

    <!-- Windows Event Logs -->
    <NCard v-if="activeTab === 'windows'" padding="none">
      <template #header>
        <div class="section-header">
          <ScrollText :size="16" />
          <span>Windows Events - {{ winLogName }} ({{ filteredWinEvents.length }})</span>
          <NButton variant="secondary" size="sm" :loading="winEventsLoading" @click="loadWinEvents" style="margin-left: auto">
            <RefreshCw :size="14" />
          </NButton>
        </div>
      </template>
      <div v-if="winEventsLoading" class="logs-empty">Chargement des evenements Windows...</div>
      <div v-else class="logs-output">
        <div
          v-for="(ev, i) in filteredWinEvents"
          :key="i"
          class="log-line"
          :class="`log-${ev.level === 'Error' || ev.level === 'Critical' ? 'error' : ev.level === 'Warning' ? 'warn' : 'info'}`"
        >
          <span class="log-time">{{ ev.timestamp }}</span>
          <NBadge :variant="winLevelVariant(ev.level)" class="log-badge">{{ ev.level }}</NBadge>
          <span class="log-source">[{{ ev.source }}]</span>
          <span class="log-msg">{{ ev.message }}</span>
        </div>
        <div v-if="filteredWinEvents.length === 0 && !winEventsLoading" class="logs-empty">
          Aucun evenement correspondant
        </div>
      </div>
    </NCard>
  </div>
</template>

<style scoped>
.logs-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 12px;
}

.page-header h1 { font-size: 24px; font-weight: 700; }
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }
.header-actions { display: flex; gap: 8px; }

.section-header { display: flex; align-items: center; gap: 8px; }

.filters-row {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.level-filters {
  display: flex;
  gap: 4px;
}

.filter-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--text-muted);
  font-family: inherit;
  font-size: 12px;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.filter-btn:hover { background: var(--bg-tertiary); color: var(--text-primary); }
.filter-btn.active { background: var(--accent-muted); color: var(--accent-primary); border-color: var(--accent-primary); }

.logs-output {
  max-height: 500px;
  overflow-y: auto;
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  padding: 8px;
}

.log-line {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background var(--transition-fast);
}

.log-line:hover { background: var(--bg-tertiary); }

.log-time {
  color: var(--text-muted);
  flex-shrink: 0;
  font-size: 11px;
  min-width: 64px;
}

.log-badge { flex-shrink: 0; }

.log-msg {
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-break: break-word;
}

.log-error .log-msg { color: var(--danger); }
.log-warn .log-msg { color: var(--warning); }

.logs-empty {
  text-align: center;
  color: var(--text-muted);
  padding: 40px;
  font-family: inherit;
}

.tab-row {
  display: flex;
  gap: 4px;
  border-bottom: 1px solid var(--border);
  padding-bottom: 0;
}

.tab-btn {
  padding: 8px 16px;
  border: none;
  border-bottom: 2px solid transparent;
  background: none;
  color: var(--text-muted);
  font-family: inherit;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.tab-btn:hover { color: var(--text-primary); }
.tab-btn.active { color: var(--accent-primary); border-bottom-color: var(--accent-primary); }

.log-select {
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 12px;
  padding: 6px 8px;
}

.log-source {
  color: var(--text-muted);
  font-size: 11px;
  flex-shrink: 0;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
