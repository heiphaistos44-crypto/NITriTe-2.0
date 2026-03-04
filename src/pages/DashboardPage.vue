<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useRouter } from "vue-router";
import StatsCard from "@/components/shared/StatsCard.vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NBadge from "@/components/ui/NBadge.vue";
import {
  Cpu, MemoryStick, HardDrive, Wifi,
  Stethoscope, Trash2, RefreshCw, Save, Shield,
  Clock, CheckCircle, AlertTriangle, Activity,
  ArrowUp, ArrowDown, X, Zap,
} from "lucide-vue-next";

const router = useRouter();

// --- Métriques temps réel ---
const cpuUsage = ref(0);
const ramUsage = ref(0);
const ramUsedGb = ref(0);
const ramTotalGb = ref(0);
const diskUsage = ref(0);
const networkDown = ref(0);
const networkUp = ref(0);
const healthScore = ref(0);
const loading = ref(true);
const isLive = ref(false);

// --- Noms composants ---
const cpuName = ref("");
const gpuName = ref("");
const diskModel = ref("");

interface ProcessInfo {
  pid: number;
  name: string;
  cpu_percent: number;
  memory_mb: number;
}

interface AlertItem {
  id: number;
  level: string;
  message: string;
  time: string;
}

const topProcesses = ref<ProcessInfo[]>([]);
const alerts = ref<AlertItem[]>([]);
let alertIdCounter = 0;

let unlisten: (() => void) | null = null;
let devInterval: ReturnType<typeof setInterval> | null = null;

const recentActivity = ref([
  { time: "Maintenant", message: "Application démarrée", type: "info" as const },
]);

const quickActions = [
  { label: "Diagnostic", icon: Stethoscope, route: "/diagnostic", color: "accent" as const },
  { label: "Nettoyage", icon: Trash2, route: "/optimizations", color: "success" as const },
  { label: "Mises à jour", icon: RefreshCw, route: "/updates", color: "info" as const },
  { label: "Sauvegarde", icon: Save, route: "/backup", color: "warning" as const },
  { label: "Scan Antivirus", icon: Shield, route: "/scanvirus", color: "danger" as const },
  { label: "Monitoring", icon: Activity, route: "/monitoring", color: "accent" as const },
];

function computeHealthScore() {
  let score = 100;
  if (cpuUsage.value > 90) score -= 20;
  else if (cpuUsage.value > 80) score -= 10;
  else if (cpuUsage.value > 60) score -= 5;
  if (ramUsage.value > 90) score -= 25;
  else if (ramUsage.value > 85) score -= 15;
  else if (ramUsage.value > 70) score -= 5;
  if (diskUsage.value > 95) score -= 25;
  else if (diskUsage.value > 90) score -= 15;
  else if (diskUsage.value > 80) score -= 5;
  healthScore.value = Math.max(0, score);
}

function healthColor(): string {
  if (healthScore.value >= 80) return "var(--success)";
  if (healthScore.value >= 50) return "var(--warning)";
  return "var(--danger)";
}

function healthLabel(): string {
  if (healthScore.value >= 80) return "Excellent";
  if (healthScore.value >= 60) return "Bon";
  if (healthScore.value >= 40) return "Moyen";
  return "Critique";
}

function applyMonitorData(raw: any) {
  cpuUsage.value = Math.round(raw.cpu_percent ?? raw.cpu_usage ?? 0);
  const ramUsed = raw.ram_used_gb ?? 0;
  const ramTotal = raw.ram_total_gb ?? 0;
  ramUsedGb.value = Math.round(ramUsed * 10) / 10;
  ramTotalGb.value = Math.round(ramTotal * 10) / 10;
  ramUsage.value = Math.round(raw.ram_percent ?? (ramTotal > 0 ? (ramUsed / ramTotal) * 100 : 0));
  diskUsage.value = Math.round(raw.disk_percent ?? 0);
  networkDown.value = Math.round(raw.network_down_kbs ?? raw.net_download_kbs ?? 0);
  networkUp.value = Math.round(raw.network_up_kbs ?? raw.net_upload_kbs ?? 0);

  // Top processus
  topProcesses.value = (raw.top_processes ?? []).slice(0, 8).map((p: any) => ({
    pid: p.pid,
    name: p.name,
    cpu_percent: Math.round(p.cpu_percent * 10) / 10,
    memory_mb: Math.round(p.memory_mb ?? 0),
  }));

  // Alertes du backend
  if (raw.alerts && Array.isArray(raw.alerts)) {
    for (const a of raw.alerts) {
      const existing = alerts.value.find(x => x.message === a.message);
      if (!existing) {
        alerts.value.unshift({
          id: ++alertIdCounter,
          level: a.level,
          message: a.message,
          time: new Date().toLocaleTimeString("fr-FR", { hour: "2-digit", minute: "2-digit" }),
        });
      }
    }
    if (alerts.value.length > 10) alerts.value = alerts.value.slice(0, 10);
  }

  computeHealthScore();
}

function dismissAlert(id: number) {
  alerts.value = alerts.value.filter(a => a.id !== id);
}

function clearAlerts() {
  alerts.value = [];
}

function formatSpeed(kbs: number): string {
  if (kbs >= 1024) return `${(kbs / 1024).toFixed(1)} MB/s`;
  return `${Math.round(kbs)} KB/s`;
}

const networkStatus = computed(() => {
  const total = networkDown.value + networkUp.value;
  if (total === 0) return "Inactif";
  return `↓ ${formatSpeed(networkDown.value)} ↑ ${formatSpeed(networkUp.value)}`;
});

onMounted(async () => {
  // Charger les logs récents
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const logs = await invoke<any[]>("get_app_logs");
    if (logs && logs.length > 0) {
      recentActivity.value = logs.slice(0, 5).map(l => ({
        time: l.timestamp?.substring(11, 16) || "—",
        message: l.message || "—",
        type: (l.level === "ERROR" ? "error" : l.level === "WARN" ? "warning" : "info") as any,
      }));
    }
  } catch { /* pas de logs */ }

  // Démarrer le monitoring live
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const { listen } = await import("@tauri-apps/api/event");
    await invoke("start_monitoring");
    isLive.value = true;
    unlisten = (await listen<any>("system-monitor", (event) => {
      applyMonitorData(event.payload);
    })) as unknown as () => void;

    // Premier chargement via get_system_info
    const info = await invoke<any>("get_system_info");
    cpuUsage.value = Math.round(info.cpu?.usage_percent ?? 0);
    ramUsedGb.value = Math.round((info.ram?.used_gb ?? 0) * 10) / 10;
    ramTotalGb.value = Math.round((info.ram?.total_gb ?? 0) * 10) / 10;
    ramUsage.value = Math.round(info.ram?.usage_percent ?? 0);
    if (info.disks?.length > 0 && info.disks[0].partitions?.length > 0) {
      diskUsage.value = Math.round(info.disks[0].partitions[0].usage_percent);
    }
    cpuName.value = info.cpu?.name ?? "";
    diskModel.value = info.disks?.[0]?.model ?? info.disks?.[0]?.name ?? "";
    if (info.gpus?.length > 0) gpuName.value = info.gpus[0].name ?? "";
    computeHealthScore();
  } catch {
    // Mode dev : simulation
    cpuUsage.value = 23;
    ramUsage.value = 45;
    ramUsedGb.value = 14.4;
    ramTotalGb.value = 32;
    diskUsage.value = 49;
    networkDown.value = 120;
    topProcesses.value = [
      { pid: 1234, name: "chrome.exe", cpu_percent: 12.5, memory_mb: 480 },
      { pid: 5678, name: "code.exe", cpu_percent: 8.3, memory_mb: 350 },
      { pid: 2345, name: "explorer.exe", cpu_percent: 2.1, memory_mb: 120 },
    ];
    devInterval = setInterval(() => {
      cpuUsage.value = Math.round(10 + Math.random() * 40);
      ramUsage.value = Math.round(40 + Math.random() * 20);
      networkDown.value = Math.round(Math.random() * 2000);
      computeHealthScore();
    }, 2000);
    computeHealthScore();
  }
  loading.value = false;
});

onUnmounted(async () => {
  if (unlisten) unlisten();
  if (devInterval) clearInterval(devInterval);
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("stop_monitoring");
  } catch { /* ignore */ }
});
</script>

<template>
  <div class="dashboard">
    <!-- Welcome -->
    <div class="welcome">
      <div>
        <h1>Tableau de bord</h1>
        <p class="welcome-date">{{ new Date().toLocaleDateString("fr-FR", { weekday: "long", year: "numeric", month: "long", day: "numeric" }) }}</p>
      </div>
      <NBadge :variant="isLive ? 'success' : 'neutral'" class="live-badge">
        <span v-if="isLive" class="live-dot"></span>
        {{ isLive ? "Monitoring actif" : "Mode statique" }}
      </NBadge>
    </div>

    <!-- Alertes actives -->
    <div v-if="alerts.length > 0" class="alerts-bar">
      <div v-for="alert in alerts" :key="alert.id" class="alert-item" :class="`alert-${alert.level}`">
        <AlertTriangle :size="14" />
        <span>{{ alert.message }}</span>
        <span class="alert-time">{{ alert.time }}</span>
        <button class="alert-dismiss" @click="dismissAlert(alert.id)">
          <X :size="12" />
        </button>
      </div>
      <button v-if="alerts.length > 1" class="clear-alerts" @click="clearAlerts">
        Effacer tout
      </button>
    </div>

    <!-- Stats Cards -->
    <div class="stats-grid stagger-children">
      <StatsCard title="CPU" :subtitle="cpuName || 'Processeur'" :value="`${cpuUsage}%`" :icon="Cpu" :progress="cpuUsage" color="accent" />
      <StatsCard title="RAM" :subtitle="`${ramUsedGb} / ${ramTotalGb} GB`" :value="`${ramUsage}%`" :icon="MemoryStick" :progress="ramUsage" color="info" />
      <StatsCard title="Disque" :subtitle="diskModel || 'Partition C:'" :value="`${diskUsage}%`" :icon="HardDrive" :progress="diskUsage" color="warning" />
      <StatsCard title="Réseau" :subtitle="networkStatus" :value="formatSpeed(networkDown)" :icon="Wifi" color="success" />
    </div>

    <!-- Health + Quick Actions -->
    <div class="dashboard-grid">
      <!-- Health Score -->
      <NCard>
        <template #header>
          <div class="card-header-row">
            <Shield :size="16" />
            <span>Santé Système</span>
          </div>
        </template>
        <div class="health-section">
          <div class="health-score" :style="{ color: healthColor() }">
            {{ healthScore }}<span class="health-max">/100</span>
          </div>
          <div class="health-label" :style="{ color: healthColor() }">{{ healthLabel() }}</div>
          <NProgress :value="healthScore" size="lg" />
          <div class="health-details">
            <div class="health-item">
              <CheckCircle v-if="cpuUsage < 80" :size="14" style="color: var(--success)" />
              <AlertTriangle v-else :size="14" style="color: var(--warning)" />
              <span>CPU: {{ cpuUsage }}% {{ cpuUsage < 80 ? "✓" : "— Élevé" }}</span>
            </div>
            <div class="health-item">
              <CheckCircle v-if="ramUsage < 85" :size="14" style="color: var(--success)" />
              <AlertTriangle v-else :size="14" style="color: var(--danger)" />
              <span>RAM: {{ ramUsage }}% {{ ramUsage < 85 ? "✓" : "— Critique" }}</span>
            </div>
            <div class="health-item">
              <CheckCircle v-if="diskUsage < 90" :size="14" style="color: var(--success)" />
              <AlertTriangle v-else :size="14" style="color: var(--danger)" />
              <span>Disque: {{ diskUsage }}% {{ diskUsage < 90 ? "✓" : "— Critique" }}</span>
            </div>
            <div class="health-item">
              <ArrowDown :size="14" style="color: var(--success)" />
              <span>↓ {{ formatSpeed(networkDown) }} ↑ {{ formatSpeed(networkUp) }}</span>
            </div>
            <div v-if="gpuName" class="health-item">
              <CheckCircle :size="14" style="color: var(--success)" />
              <span class="muted" style="font-size:11px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;max-width:200px">{{ gpuName }}</span>
            </div>
          </div>
        </div>
      </NCard>

      <!-- Quick Actions -->
      <NCard>
        <template #header>
          <div class="card-header-row">
            <Zap :size="16" />
            <span>Actions Rapides</span>
          </div>
        </template>
        <div class="quick-actions">
          <button
            v-for="action in quickActions"
            :key="action.label"
            class="action-btn"
            @click="router.push(action.route)"
          >
            <div class="action-icon" :style="{ background: `var(--${action.color}-muted, var(--accent-muted))` }">
              <component :is="action.icon" :size="18" :style="{ color: `var(--${action.color}, var(--accent-primary))` }" />
            </div>
            <span>{{ action.label }}</span>
          </button>
        </div>
      </NCard>
    </div>

    <!-- Top Processus -->
    <NCard v-if="topProcesses.length > 0">
      <template #header>
        <div class="card-header-row">
          <Activity :size="16" />
          <span>Top Processus</span>
          <NBadge variant="neutral">{{ topProcesses.length }}</NBadge>
        </div>
      </template>
      <div class="processes-table">
        <div class="process-header">
          <span>Processus</span>
          <span>PID</span>
          <span>CPU</span>
          <span>RAM</span>
        </div>
        <div v-for="proc in topProcesses" :key="proc.pid" class="process-row">
          <span class="proc-name">{{ proc.name }}</span>
          <span class="proc-pid font-mono">{{ proc.pid }}</span>
          <div class="proc-cpu">
            <span class="proc-val">{{ proc.cpu_percent.toFixed(1) }}%</span>
            <div class="proc-bar">
              <div class="proc-bar-fill proc-bar-cpu" :style="{ width: `${Math.min(100, proc.cpu_percent * 2)}%` }"></div>
            </div>
          </div>
          <div class="proc-ram">
            <span class="proc-val">{{ proc.memory_mb >= 1024 ? `${(proc.memory_mb/1024).toFixed(1)} GB` : `${proc.memory_mb} MB` }}</span>
          </div>
        </div>
      </div>
    </NCard>

    <!-- Recent Activity -->
    <NCard>
      <template #header>
        <div class="card-header-row">
          <Clock :size="16" />
          <span>Activité Récente</span>
        </div>
      </template>
      <div class="activity-list">
        <div v-for="(item, i) in recentActivity" :key="i" class="activity-item">
          <div class="activity-dot" :class="`dot-${item.type}`"></div>
          <span class="activity-time">{{ item.time }}</span>
          <span class="activity-msg">{{ item.message }}</span>
        </div>
        <div v-if="recentActivity.length === 1" class="activity-empty">
          Aucune activité récente. Lancez un diagnostic ou une optimisation.
        </div>
      </div>
    </NCard>
  </div>
</template>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.welcome {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.welcome h1 { font-size: 24px; font-weight: 700; }
.welcome-date { color: var(--text-muted); font-size: 13px; margin-top: 2px; }

.live-badge { display: flex; align-items: center; gap: 6px; }
.live-dot {
  width: 7px; height: 7px;
  border-radius: 50%;
  background: var(--success);
  animation: pulse 2s ease-in-out infinite;
}
@keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }

/* Alerts */
.alerts-bar {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
}

.alert-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border-radius: var(--radius-md);
  font-size: 12px;
  font-weight: 500;
}

.alert-warning { background: rgba(var(--warning-rgb, 245, 158, 11), 0.15); color: var(--warning); border: 1px solid var(--warning); }
.alert-critical { background: rgba(var(--danger-rgb, 239, 68, 68), 0.15); color: var(--danger); border: 1px solid var(--danger); }
.alert-info { background: var(--accent-muted); color: var(--accent-primary); border: 1px solid var(--accent-primary); }

.alert-time { color: inherit; opacity: 0.6; font-size: 11px; }
.alert-dismiss { background: none; border: none; cursor: pointer; color: inherit; opacity: 0.6; padding: 0; display: flex; }
.alert-dismiss:hover { opacity: 1; }
.clear-alerts {
  font-size: 11px; color: var(--text-muted); background: none; border: none;
  cursor: pointer; padding: 4px 8px; border-radius: var(--radius-sm);
}
.clear-alerts:hover { color: var(--text-secondary); background: var(--bg-tertiary); }

/* Stats */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}
@media (max-width: 1200px) { .stats-grid { grid-template-columns: repeat(2, 1fr); } }

.dashboard-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}
@media (max-width: 1000px) { .dashboard-grid { grid-template-columns: 1fr; } }

.card-header-row { display: flex; align-items: center; gap: 8px; }

/* Health */
.health-section { text-align: center; display: flex; flex-direction: column; align-items: center; gap: 8px; }
.health-score { font-size: 48px; font-weight: 800; font-family: "JetBrains Mono", monospace; line-height: 1; }
.health-max { font-size: 20px; opacity: 0.4; }
.health-label { font-size: 14px; font-weight: 600; }
.health-details { display: flex; flex-direction: column; gap: 6px; margin-top: 8px; width: 100%; }
.health-item { display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--text-secondary); }

/* Quick Actions */
.quick-actions { display: flex; flex-direction: column; gap: 4px; }
.action-btn {
  display: flex; align-items: center; gap: 12px; padding: 10px 12px;
  border: none; border-radius: var(--radius-md); background: transparent;
  cursor: pointer; font-family: inherit; font-size: 13px; color: var(--text-primary);
  transition: all var(--transition-fast); text-align: left; width: 100%;
}
.action-btn:hover { background: var(--bg-tertiary); }
.action-icon { width: 32px; height: 32px; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; flex-shrink: 0; }

/* Processes */
.processes-table { display: flex; flex-direction: column; gap: 4px; }
.process-header {
  display: grid; grid-template-columns: 1fr 70px 140px 100px;
  gap: 8px; padding: 6px 8px;
  font-size: 11px; font-weight: 600; color: var(--text-muted);
  text-transform: uppercase; letter-spacing: 0.05em;
  border-bottom: 1px solid var(--border);
}
.process-row {
  display: grid; grid-template-columns: 1fr 70px 140px 100px;
  gap: 8px; padding: 6px 8px; border-radius: var(--radius-sm);
  font-size: 12px; align-items: center;
  transition: background var(--transition-fast);
}
.process-row:hover { background: var(--bg-tertiary); }
.proc-name { font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.proc-pid { color: var(--text-muted); font-size: 11px; }
.proc-cpu, .proc-ram { display: flex; align-items: center; gap: 6px; }
.proc-val { font-family: "JetBrains Mono", monospace; font-size: 11px; min-width: 42px; }
.proc-bar { flex: 1; height: 4px; background: var(--border); border-radius: 2px; overflow: hidden; }
.proc-bar-fill { height: 100%; border-radius: 2px; transition: width 0.5s ease; }
.proc-bar-cpu { background: var(--accent-primary); }

/* Activity */
.activity-list { display: flex; flex-direction: column; gap: 8px; }
.activity-item { display: flex; align-items: center; gap: 10px; font-size: 13px; }
.activity-dot { width: 6px; height: 6px; border-radius: 50%; flex-shrink: 0; }
.dot-info { background: var(--info); }
.dot-success { background: var(--success); }
.dot-warning { background: var(--warning); }
.dot-error { background: var(--danger); }
.activity-time { color: var(--text-muted); min-width: 50px; font-family: "JetBrains Mono", monospace; font-size: 11px; }
.activity-msg { color: var(--text-secondary); }
.activity-empty { color: var(--text-muted); font-size: 13px; text-align: center; padding: 12px; }

.font-mono { font-family: "JetBrains Mono", monospace; }
</style>
