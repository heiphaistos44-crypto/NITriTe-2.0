<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  BarChart3, Cpu, MemoryStick, HardDrive, Monitor,
  Clock, CheckCircle, AlertTriangle, XCircle,
  FileText, RefreshCw, Thermometer, Activity, FolderOpen,
} from "lucide-vue-next";

const notifications = useNotificationStore();
const loading = ref(true);
const generating = ref(false);

interface SysStats {
  osName: string;
  osVersion: string;
  cpuModel: string;
  cpuUsage: number;
  ramTotal: number;
  ramUsed: number;
  ramPercent: number;
  diskTotal: number;
  diskUsed: number;
  diskPercent: number;
  uptime: string;
}

const stats = ref<SysStats>({
  osName: "", osVersion: "", cpuModel: "",
  cpuUsage: 0, ramTotal: 0, ramUsed: 0, ramPercent: 0,
  diskTotal: 0, diskUsed: 0, diskPercent: 0, uptime: "",
});

interface HealthItem {
  label: string;
  status: "ok" | "warn" | "critical";
  detail: string;
}

const healthChecks = ref<HealthItem[]>([]);

function formatUptime(seconds: number): string {
  const d = Math.floor(seconds / 86400);
  const h = Math.floor((seconds % 86400) / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const parts: string[] = [];
  if (d > 0) parts.push(`${d}j`);
  if (h > 0) parts.push(`${h}h`);
  parts.push(`${m}min`);
  return parts.join(" ");
}

function statusIcon(status: string) {
  if (status === "ok") return CheckCircle;
  if (status === "warn") return AlertTriangle;
  return XCircle;
}

function statusColor(status: string): string {
  if (status === "ok") return "var(--success)";
  if (status === "warn") return "var(--warning)";
  return "var(--danger)";
}

function statusVariant(status: string): "success" | "warning" | "danger" {
  if (status === "ok") return "success";
  if (status === "warn") return "warning";
  return "danger";
}

function computeHealth() {
  const checks: HealthItem[] = [];

  checks.push({
    label: "CPU",
    status: stats.value.cpuUsage < 70 ? "ok" : stats.value.cpuUsage < 90 ? "warn" : "critical",
    detail: `Usage: ${stats.value.cpuUsage}%`,
  });

  checks.push({
    label: "RAM",
    status: stats.value.ramPercent < 75 ? "ok" : stats.value.ramPercent < 90 ? "warn" : "critical",
    detail: `${stats.value.ramUsed.toFixed(1)} / ${stats.value.ramTotal.toFixed(0)} GB (${stats.value.ramPercent}%)`,
  });

  checks.push({
    label: "Disque",
    status: stats.value.diskPercent < 80 ? "ok" : stats.value.diskPercent < 95 ? "warn" : "critical",
    detail: `${stats.value.diskUsed.toFixed(0)} / ${stats.value.diskTotal.toFixed(0)} GB (${stats.value.diskPercent}%)`,
  });

  healthChecks.value = checks;
}

async function loadStats() {
  loading.value = true;
  try {
    const info = await invoke<any>("get_system_info");
    const platform = await invoke<any>("get_platform_info").catch(() => null);

    stats.value.osName = info.os?.name ?? platform?.os_name ?? "Windows";
    stats.value.osVersion = info.os?.version ?? platform?.os_version ?? "11";
    stats.value.cpuModel = info.cpu?.name ?? "Inconnu";
    stats.value.cpuUsage = Math.round(info.cpu?.usage_percent ?? 0);
    stats.value.ramTotal = info.ram?.total_gb ?? 0;
    stats.value.ramUsed = info.ram?.used_gb ?? 0;
    stats.value.ramPercent = Math.round(info.ram?.usage_percent ?? 0);

    if (info.disks?.length > 0 && info.disks[0].partitions?.length > 0) {
      const p = info.disks[0].partitions[0];
      stats.value.diskTotal = p.total_gb ?? 0;
      stats.value.diskUsed = p.used_gb ?? 0;
      stats.value.diskPercent = Math.round(p.usage_percent ?? 0);
    }

    stats.value.uptime = platform?.uptime_seconds
      ? formatUptime(platform.uptime_seconds)
      : info.uptime
        ? formatUptime(info.uptime)
        : "N/A";
  } catch {
    stats.value = {
      osName: "Windows", osVersion: "11 (26100)", cpuModel: "AMD Ryzen 7 5800X",
      cpuUsage: 23, ramTotal: 32, ramUsed: 14.4, ramPercent: 45,
      diskTotal: 931, diskUsed: 456, diskPercent: 49, uptime: "3j 14h 22min",
    };
  }
  computeHealth();
  loading.value = false;
}

async function generateReport() {
  generating.value = true;
  const report = [
    "=== RAPPORT SYSTEME NITRITE ===",
    `Date: ${new Date().toLocaleString("fr-FR")}`,
    "",
    `OS: ${stats.value.osName} ${stats.value.osVersion}`,
    `CPU: ${stats.value.cpuModel} (Usage: ${stats.value.cpuUsage}%)`,
    `RAM: ${stats.value.ramUsed.toFixed(1)} / ${stats.value.ramTotal.toFixed(0)} GB (${stats.value.ramPercent}%)`,
    `Disque: ${stats.value.diskUsed.toFixed(0)} / ${stats.value.diskTotal.toFixed(0)} GB (${stats.value.diskPercent}%)`,
    `Uptime: ${stats.value.uptime}`,
    "",
    "=== VERIFICATION SANTE ===",
    ...healthChecks.value.map((h) => `[${h.status.toUpperCase()}] ${h.label}: ${h.detail}`),
    "",
    "=== FIN DU RAPPORT ===",
  ].join("\n");

  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const { writeTextFile } = await import("@tauri-apps/plugin-fs");
    const path = await save({ defaultPath: "rapport-systeme.txt", filters: [{ name: "Texte", extensions: ["txt"] }] });
    if (path) {
      await writeTextFile(path, report);
      notifications.success("Rapport genere", path);
    }
  } catch {
    navigator.clipboard.writeText(report);
    notifications.info("Rapport copie dans le presse-papier");
  }
  generating.value = false;
}

// --- Reports listing ---
interface ReportFile {
  name: string;
  path: string;
  size_bytes: number;
  created: string;
}

const reports = ref<ReportFile[]>([]);
const reportsLoading = ref(false);

async function loadReports() {
  reportsLoading.value = true;
  try {
    reports.value = await invoke<ReportFile[]>("list_reports");
  } catch {
    reports.value = [
      { name: "rapport-systeme-2026-03-01.txt", path: "C:\\NiTriTe\\rapports\\rapport-systeme-2026-03-01.txt", size_bytes: 4096, created: "2026-03-01 10:00" },
      { name: "backup-info-2026-02-28.txt", path: "C:\\NiTriTe\\rapports\\backup-info-2026-02-28.txt", size_bytes: 8192, created: "2026-02-28 14:30" },
    ];
  } finally {
    reportsLoading.value = false;
  }
}

async function openReportFolder() {
  try {
    await invoke("run_system_command", { cmd: "explorer", args: ["%USERPROFILE%\\Documents\\NiTriTe"] });
  } catch {
    notifications.info("Mode dev", "Ouverture dossier simulee");
  }
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1048576).toFixed(1)} MB`;
}

onMounted(() => {
  loadStats();
  loadReports();
});
</script>

<template>
  <div class="stats-page">
    <div class="page-header">
      <div>
        <h1>Statistiques & Rapports</h1>
        <p class="page-subtitle">Vue d'ensemble et generation de rapports systeme</p>
      </div>
      <div class="header-actions">
        <NButton variant="secondary" size="sm" :loading="generating" @click="generateReport">
          <FileText :size="14" />
          Generer un rapport complet
        </NButton>
        <NButton variant="primary" size="sm" :loading="loading" @click="loadStats">
          <RefreshCw :size="14" />
          Rafraichir
        </NButton>
      </div>
    </div>

    <div v-if="loading" class="loading-state">
      <NSpinner :size="32" />
      <p>Collecte des statistiques...</p>
    </div>

    <template v-else>
      <!-- Uptime -->
      <NCard>
        <div class="uptime-display">
          <Clock :size="20" style="color: var(--accent-primary)" />
          <div>
            <span class="uptime-label">Uptime systeme</span>
            <span class="uptime-value">{{ stats.uptime }}</span>
          </div>
        </div>
      </NCard>

      <!-- Summary Cards -->
      <div class="summary-grid">
        <NCard>
          <div class="summary-item">
            <div class="summary-icon" style="background: var(--accent-muted)">
              <Cpu :size="20" style="color: var(--accent-primary)" />
            </div>
            <div class="summary-info">
              <span class="summary-label">Processeur</span>
              <span class="summary-value">{{ stats.cpuModel }}</span>
              <span class="summary-detail">Usage: {{ stats.cpuUsage }}%</span>
            </div>
          </div>
        </NCard>

        <NCard>
          <div class="summary-item">
            <div class="summary-icon" style="background: var(--info-muted)">
              <MemoryStick :size="20" style="color: var(--info)" />
            </div>
            <div class="summary-info">
              <span class="summary-label">Memoire RAM</span>
              <span class="summary-value">{{ stats.ramTotal.toFixed(0) }} GB</span>
              <span class="summary-detail">{{ stats.ramUsed.toFixed(1) }} GB utilises</span>
            </div>
          </div>
        </NCard>

        <NCard>
          <div class="summary-item">
            <div class="summary-icon" style="background: var(--warning-muted)">
              <HardDrive :size="20" style="color: var(--warning)" />
            </div>
            <div class="summary-info">
              <span class="summary-label">Stockage</span>
              <span class="summary-value">{{ stats.diskTotal.toFixed(0) }} GB</span>
              <span class="summary-detail">{{ stats.diskUsed.toFixed(0) }} GB utilises</span>
            </div>
          </div>
        </NCard>

        <NCard>
          <div class="summary-item">
            <div class="summary-icon" style="background: var(--success-muted)">
              <Monitor :size="20" style="color: var(--success)" />
            </div>
            <div class="summary-info">
              <span class="summary-label">Systeme</span>
              <span class="summary-value">{{ stats.osName }}</span>
              <span class="summary-detail">{{ stats.osVersion }}</span>
            </div>
          </div>
        </NCard>
      </div>

      <!-- Health Check -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Activity :size="16" />
            <span>Verification de sante</span>
          </div>
        </template>
        <div class="health-list">
          <div v-for="check in healthChecks" :key="check.label" class="health-row">
            <div class="health-left">
              <component :is="statusIcon(check.status)" :size="16" :style="{ color: statusColor(check.status) }" />
              <span class="health-name">{{ check.label }}</span>
            </div>
            <span class="health-detail">{{ check.detail }}</span>
            <NBadge :variant="statusVariant(check.status)">
              {{ check.status === "ok" ? "OK" : check.status === "warn" ? "Attention" : "Critique" }}
            </NBadge>
          </div>
        </div>
      </NCard>

      <!-- Usage bars -->
      <NCard>
        <template #header>
          <div class="section-header">
            <BarChart3 :size="16" />
            <span>Utilisation des ressources</span>
          </div>
        </template>
        <div class="usage-bars">
          <div class="usage-row">
            <span class="usage-label">CPU</span>
            <NProgress :value="stats.cpuUsage" showLabel />
          </div>
          <div class="usage-row">
            <span class="usage-label">RAM</span>
            <NProgress :value="stats.ramPercent" showLabel />
          </div>
          <div class="usage-row">
            <span class="usage-label">Disque</span>
            <NProgress :value="stats.diskPercent" showLabel />
          </div>
        </div>
      </NCard>

      <!-- Reports listing -->
      <NCard>
        <template #header>
          <div class="section-header">
            <FileText :size="16" />
            <span>Rapports generes ({{ reports.length }})</span>
            <NButton variant="secondary" size="sm" @click="openReportFolder" style="margin-left: auto">
              <FolderOpen :size="14" /> Ouvrir dossier
            </NButton>
          </div>
        </template>
        <div v-if="reportsLoading" class="loading-state" style="padding: 24px">
          <NSpinner :size="20" />
        </div>
        <div v-else-if="reports.length === 0" class="reports-empty">
          Aucun rapport genere pour le moment.
        </div>
        <div v-else class="reports-list">
          <div v-for="r in reports" :key="r.path" class="report-row">
            <FileText :size="14" style="color: var(--text-muted); flex-shrink: 0" />
            <div class="report-info">
              <span class="report-name">{{ r.name }}</span>
              <span class="report-meta">{{ r.created }} — {{ formatBytes(r.size_bytes) }}</span>
            </div>
          </div>
        </div>
      </NCard>
    </template>
  </div>
</template>

<style scoped>
.stats-page {
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

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 60px;
  color: var(--text-muted);
}

/* Uptime */
.uptime-display {
  display: flex;
  align-items: center;
  gap: 16px;
}

.uptime-label {
  display: block;
  font-size: 12px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.uptime-value {
  display: block;
  font-size: 22px;
  font-weight: 700;
  font-family: "JetBrains Mono", monospace;
  color: var(--text-primary);
}

/* Summary */
.summary-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

@media (max-width: 1200px) { .summary-grid { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 600px) { .summary-grid { grid-template-columns: 1fr; } }

.summary-item {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.summary-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.summary-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.summary-label { font-size: 12px; color: var(--text-muted); }
.summary-value { font-size: 14px; font-weight: 600; color: var(--text-primary); }
.summary-detail { font-size: 12px; color: var(--text-secondary); }

/* Health */
.health-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.health-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}

.health-row:last-child { border-bottom: none; }

.health-left {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 100px;
}

.health-name { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.health-detail { flex: 1; font-size: 13px; color: var(--text-secondary); font-family: "JetBrains Mono", monospace; font-size: 12px; }

/* Usage */
.usage-bars {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.usage-row {
  display: flex;
  align-items: center;
  gap: 16px;
}

.usage-label {
  min-width: 60px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

/* Reports */
.reports-empty { text-align: center; color: var(--text-muted); padding: 24px; font-size: 13px; }
.reports-list { display: flex; flex-direction: column; gap: 4px; }
.report-row {
  display: flex; align-items: center; gap: 10px; padding: 8px 10px;
  border-radius: var(--radius-md); transition: background var(--transition-fast);
}
.report-row:hover { background: var(--bg-tertiary); }
.report-info { display: flex; flex-direction: column; gap: 2px; }
.report-name { font-size: 13px; font-weight: 500; color: var(--text-primary); font-family: "JetBrains Mono", monospace; }
.report-meta { font-size: 11px; color: var(--text-muted); }
</style>
