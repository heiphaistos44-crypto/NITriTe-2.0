<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from "vue-router";
import StatsCard from "@/components/shared/StatsCard.vue";
import SparklineChart from "@/components/ui/SparklineChart.vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NBadge from "@/components/ui/NBadge.vue";
import {
  Cpu, MemoryStick, HardDrive, Wifi,
  Stethoscope, Trash2, RefreshCw, Save, Shield,
  Clock, CheckCircle, AlertTriangle, Activity,
  ArrowUp, ArrowDown, X, Zap,
  Monitor, Thermometer, Flame, Pause, Play,
} from "lucide-vue-next";

const router = useRouter();

// --- Interfaces monitoring détaillé ---
interface CoreUsage { id: number; usage: number; }
interface GpuInfo { name: string; usage_percent: number; vram_used_mb: number; vram_total_mb: number; temperature_c: number; }
interface DiskTemp { name: string; temp_c: number; }

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
const isSimulation = ref(false);

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

// --- Monitoring détaillé ---
const cpuCores = ref<CoreUsage[]>([]);
const gpuData = ref<GpuInfo[]>([]);
const diskTemps = ref<DiskTemp[]>([]);
const diskReadKbs = ref(0);
const diskWriteKbs = ref(0);
const networkDownKbs = ref(0);
const networkUpKbs = ref(0);
const paused = ref(false);
const cpuTemp = ref(0);

// Historique pour sparklines (20 derniers points)
const cpuHistory = ref<number[]>([]);
const ramHistory = ref<number[]>([]);
const netHistory = ref<number[]>([]);
const diskHistory = ref<number[]>([]);
const MAX_HISTORY = 20;

function pushHistory(arr: typeof cpuHistory, val: number) {
  arr.value.push(val);
  if (arr.value.length > MAX_HISTORY) arr.value.shift();
}

let unlisten: (() => void) | null = null;
let devInterval: ReturnType<typeof setInterval> | null = null;

const recentActivity = ref([
  { time: "Maintenant", message: "Application démarrée", type: "info" as const },
]);

const quickActions = [
  { label: "Diagnostic", icon: Stethoscope, route: "/diagnostic", color: "accent" as const, grad: "linear-gradient(135deg,#f97316,#fb923c)" },
  { label: "Nettoyage", icon: Trash2, route: "/optimizations", color: "success" as const, grad: "linear-gradient(135deg,#22c55e,#16a34a)" },
  { label: "Mises à jour", icon: RefreshCw, route: "/updates", color: "info" as const, grad: "linear-gradient(135deg,#3b82f6,#2563eb)" },
  { label: "Sauvegarde", icon: Save, route: "/backup", color: "warning" as const, grad: "linear-gradient(135deg,#eab308,#ca8a04)" },
  { label: "Scan Antivirus", icon: Shield, route: "/scanvirus", color: "danger" as const, grad: "linear-gradient(135deg,#ef4444,#dc2626)" },
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

  // Monitoring détaillé
  cpuCores.value = (raw.cpu_per_core ?? raw.cpu_cores ?? []).map((u: number | CoreUsage, i: number) =>
    typeof u === "number" ? { id: i, usage: u } : u
  );
  // Ne pas écraser avec [] si l'event ne contient pas de données GPU → évite le flicker
  if (Array.isArray(raw.gpu_data) && raw.gpu_data.length > 0) {
    gpuData.value = raw.gpu_data;
  }
  if (Array.isArray(raw.disk_temps) && raw.disk_temps.length > 0) {
    diskTemps.value = raw.disk_temps;
  } else if (!Array.isArray(raw.disk_temps)) {
    diskTemps.value = [];
  }
  diskReadKbs.value = raw.disk_read_kbs ?? 0;
  diskWriteKbs.value = raw.disk_write_kbs ?? 0;
  cpuTemp.value = raw.cpu_temp_c ?? 0;
  networkDownKbs.value = raw.network_down_kbs ?? raw.net_download_kbs ?? 0;
  networkUpKbs.value = raw.network_up_kbs ?? raw.net_upload_kbs ?? 0;

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

  pushHistory(cpuHistory, cpuUsage.value);
  pushHistory(ramHistory, ramUsage.value);
  pushHistory(netHistory, Math.min(100, (networkDownKbs.value + networkUpKbs.value) / 100));
  pushHistory(diskHistory, diskUsage.value);
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

function coreBarColor(usage: number): string {
  if (usage > 90) return "var(--danger)";
  if (usage > 70) return "var(--warning)";
  return "var(--accent-primary)";
}

function tempColor(t: number): string {
  if (t > 85) return "var(--danger)";
  if (t > 70) return "var(--warning)";
  return "var(--success)";
}

const networkStatus = computed(() => {
  const total = networkDown.value + networkUp.value;
  if (total === 0) return "Inactif";
  return `↓ ${formatSpeed(networkDown.value)} ↑ ${formatSpeed(networkUp.value)}`;
});

onMounted(async () => {
  // Charger les logs récents
  try {
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
    // Mode dev : simulation (backend inaccessible)
    isSimulation.value = true;
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
      networkDownKbs.value = networkDown.value;
      pushHistory(cpuHistory, cpuUsage.value);
      pushHistory(ramHistory, ramUsage.value);
      pushHistory(netHistory, Math.min(100, networkDown.value / 20));
      pushHistory(diskHistory, diskUsage.value);
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
    await invoke("stop_monitoring");
  } catch { /* ignore */ }
});
</script>

<template>
  <div class="dashboard">
    <!-- Welcome -->
    <div class="welcome">
      <div>
        <h1 class="welcome-title">Tableau de bord</h1>
        <p class="welcome-date">{{ new Date().toLocaleDateString("fr-FR", { weekday: "long", year: "numeric", month: "long", day: "numeric" }) }}</p>
      </div>
      <NBadge :variant="isLive ? 'success' : isSimulation ? 'warning' : 'neutral'" class="live-badge">
        <span v-if="isLive" class="live-dot"></span>
        {{ isLive ? "Monitoring actif" : isSimulation ? "⚠ Simulation" : "Mode statique" }}
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

    <!-- Sparklines Tendances -->
    <div v-if="cpuHistory.length >= 3" class="sparklines-row">
      <div class="sparkline-card">
        <span class="sparkline-label">CPU</span>
        <SparklineChart :data="cpuHistory" color="var(--accent-primary)" :height="36" :fill="true" label="cpu" />
        <span class="sparkline-val">{{ cpuUsage }}%</span>
      </div>
      <div class="sparkline-card">
        <span class="sparkline-label">RAM</span>
        <SparklineChart :data="ramHistory" color="var(--info, #60a5fa)" :height="36" :fill="true" label="ram" />
        <span class="sparkline-val">{{ ramUsage }}%</span>
      </div>
      <div class="sparkline-card">
        <span class="sparkline-label">Disque</span>
        <SparklineChart :data="diskHistory" color="var(--warning)" :height="36" :fill="true" label="disk" />
        <span class="sparkline-val">{{ diskUsage }}%</span>
      </div>
      <div class="sparkline-card">
        <span class="sparkline-label">Réseau</span>
        <SparklineChart :data="netHistory" color="var(--success)" :height="36" :fill="true" label="net" />
        <span class="sparkline-val">{{ formatSpeed(networkDown) }}</span>
      </div>
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
            <div class="action-icon" :style="{ background: action.grad }">
              <component :is="action.icon" :size="16" style="color:white" />
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

    <!-- Monitoring Détaillé : Header -->
    <div class="monitoring-header" v-if="isLive">
      <h2 class="section-title-mon"><Activity :size="16" /> Monitoring Temps Réel</h2>
      <div style="display:flex;align-items:center;gap:8px">
        <NBadge :variant="paused ? 'warning' : 'success'">{{ paused ? 'Pause' : 'Live' }}</NBadge>
        <NButton variant="ghost" size="sm" @click="paused = !paused">
          <Pause v-if="!paused" :size="13" /><Play v-else :size="13" />
        </NButton>
      </div>
    </div>

    <!-- GPU Cards -->
    <div class="gpu-cards" v-if="gpuData.length && isLive && !paused">
      <NCard v-for="(gpu, i) in gpuData" :key="i">
        <template #header>
          <div class="card-header-row">
            <Monitor :size="14" />
            <span>{{ gpu.name }}</span>
          </div>
        </template>
        <div class="gpu-stat-row">
          <span class="gpu-stat-label">Utilisation</span>
          <div class="mon-bar-track" style="flex:1">
            <div class="mon-bar-fill" :style="{ width: `${gpu.usage_percent}%`, background: coreBarColor(gpu.usage_percent) }" />
          </div>
          <span class="mon-val">{{ Math.round(gpu.usage_percent) }}%</span>
        </div>
        <div class="gpu-stat-row" v-if="gpu.vram_total_mb > 0">
          <span class="gpu-stat-label">VRAM</span>
          <div class="mon-bar-track" style="flex:1">
            <div class="mon-bar-fill" :style="{ width: `${(gpu.vram_used_mb/gpu.vram_total_mb)*100}%`, background: 'var(--info)' }" />
          </div>
          <span class="mon-val">{{ gpu.vram_used_mb }} / {{ gpu.vram_total_mb }} MB</span>
        </div>
        <div class="gpu-stat-row" v-if="gpu.temperature_c > 0">
          <span class="gpu-stat-label">Temp.</span>
          <span class="mon-val" :style="{ color: tempColor(gpu.temperature_c) }">{{ gpu.temperature_c }}°C</span>
        </div>
      </NCard>
    </div>

    <!-- Températures -->
    <NCard v-if="isLive && !paused && (cpuTemp > 0 || gpuData.some(g => g.temperature_c > 0) || diskTemps.length > 0)">
      <template #header>
        <div class="card-header-row">
          <Thermometer :size="16" />
          <span>Températures Composants</span>
          <Flame v-if="cpuTemp > 80 || gpuData.some(g => g.temperature_c > 85) || diskTemps.some(d => d.temp_c > 55)"
            :size="14" style="color:var(--danger);margin-left:4px" />
        </div>
      </template>
      <div class="temps-grid">
        <div v-if="cpuTemp > 0" class="temp-item">
          <Cpu :size="14" />
          <span class="temp-label">CPU</span>
          <div class="temp-bar-track" style="flex:1">
            <div class="temp-bar-fill" :style="{ width: `${Math.min((cpuTemp/100)*100,100)}%`, background: tempColor(cpuTemp) }" />
          </div>
          <span class="temp-value" :style="{ color: tempColor(cpuTemp) }">{{ cpuTemp }}°C</span>
        </div>
        <div v-for="(gpu, i) in gpuData.filter(g => g.temperature_c > 0)" :key="`gpu-t-${i}`" class="temp-item">
          <Monitor :size="14" />
          <span class="temp-label">{{ gpu.name.split(' ').slice(0,2).join(' ') }}</span>
          <div class="temp-bar-track" style="flex:1">
            <div class="temp-bar-fill" :style="{ width: `${Math.min((gpu.temperature_c/100)*100,100)}%`, background: tempColor(gpu.temperature_c) }" />
          </div>
          <span class="temp-value" :style="{ color: tempColor(gpu.temperature_c) }">{{ gpu.temperature_c }}°C</span>
        </div>
        <div v-for="(disk, i) in diskTemps" :key="`disk-t-${i}`" class="temp-item">
          <HardDrive :size="14" />
          <span class="temp-label">{{ disk.name.length > 16 ? disk.name.slice(0,16)+'…' : disk.name }}</span>
          <div class="temp-bar-track" style="flex:1">
            <div class="temp-bar-fill" :style="{ width: `${Math.min((disk.temp_c/80)*100,100)}%`, background: tempColor(disk.temp_c-10) }" />
          </div>
          <span class="temp-value" :style="{ color: tempColor(disk.temp_c-10) }">{{ disk.temp_c }}°C</span>
        </div>
      </div>
    </NCard>

    <!-- CPU Cores + Réseau/Disque -->
    <div class="monitor-grid" v-if="isLive && !paused && cpuCores.length > 0">
      <NCard>
        <template #header>
          <div class="card-header-row"><Cpu :size="16" /><span>Usage par Cœur</span></div>
        </template>
        <div class="cores-grid">
          <div v-for="core in cpuCores" :key="core.id" class="core-item">
            <span class="core-label">C{{ core.id }}</span>
            <div class="mon-bar-track">
              <div class="mon-bar-fill" :style="{ width: `${core.usage}%`, background: coreBarColor(core.usage) }" />
            </div>
            <span class="mon-val">{{ core.usage }}%</span>
          </div>
        </div>
      </NCard>
      <NCard>
        <template #header>
          <div class="card-header-row"><Wifi :size="16" /><span>Débits Réseau &amp; Disque</span></div>
        </template>
        <div class="net-speeds">
          <div class="net-item">
            <div class="net-icon net-download"><ArrowDown :size="15" /></div>
            <div><div class="net-lbl">Download</div><div class="net-val">{{ formatSpeed(networkDownKbs) }}</div></div>
          </div>
          <div class="net-item">
            <div class="net-icon net-upload"><ArrowUp :size="15" /></div>
            <div><div class="net-lbl">Upload</div><div class="net-val">{{ formatSpeed(networkUpKbs) }}</div></div>
          </div>
          <div class="net-sep" />
          <div class="net-item">
            <div class="net-icon net-read"><HardDrive :size="15" /></div>
            <div><div class="net-lbl">Lecture Disque</div><div class="net-val">{{ formatSpeed(diskReadKbs) }}</div></div>
          </div>
          <div class="net-item">
            <div class="net-icon net-write"><HardDrive :size="15" /></div>
            <div><div class="net-lbl">Écriture Disque</div><div class="net-val">{{ formatSpeed(diskWriteKbs) }}</div></div>
          </div>
        </div>
      </NCard>
    </div>

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

.welcome-title {
  font-size: 26px;
  font-weight: 800;
  background: linear-gradient(135deg, var(--text-primary) 40%, var(--accent-primary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  line-height: 1.2;
}
.welcome-date { color: var(--text-secondary); font-size: 13px; margin-top: 3px; }

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

/* Sparklines tendances */
.sparklines-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}
@media (max-width: 1200px) { .sparklines-row { grid-template-columns: repeat(2, 1fr); } }
.sparkline-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
}
.sparkline-label {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: .06em;
  color: var(--text-muted);
  width: 36px;
  flex-shrink: 0;
}
.sparkline-val {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  font-weight: 700;
  color: var(--text-primary);
  width: 52px;
  text-align: right;
  flex-shrink: 0;
}

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
.quick-actions { display: flex; flex-direction: column; gap: 6px; }
.action-btn {
  display: flex; align-items: center; gap: 12px; padding: 10px 12px;
  border: 1px solid transparent; border-radius: var(--radius-md); background: transparent;
  cursor: pointer; font-family: inherit; font-size: 13px; color: var(--text-primary);
  transition: all var(--transition-fast); text-align: left; width: 100%; font-weight: 500;
}
.action-btn:hover {
  background: var(--bg-tertiary);
  border-color: var(--border);
  transform: translateX(3px);
}
.action-icon {
  width: 34px; height: 34px; border-radius: var(--radius-md);
  display: flex; align-items: center; justify-content: center; flex-shrink: 0;
  box-shadow: 0 2px 8px rgba(0,0,0,.3);
}

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
.proc-bar { flex: 1; height: 5px; background: var(--bg-elevated); border: 1px solid var(--border); border-radius: 99px; overflow: hidden; }
.proc-bar-fill { height: 100%; border-radius: 99px; transition: width 0.5s ease; }
.proc-bar-cpu { background: linear-gradient(90deg, var(--accent-primary), var(--accent-hover)); }

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

/* Monitoring intégré */
.monitoring-header { display: flex; justify-content: space-between; align-items: center; }
.section-title-mon { display: flex; align-items: center; gap: 8px; font-size: 15px; font-weight: 700; color: var(--text-primary); }

.gpu-cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }
.gpu-stat-row { display: flex; align-items: center; gap: 10px; margin-top: 8px; }
.gpu-stat-label { font-size: 12px; color: var(--text-secondary); min-width: 75px; }

.monitor-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
@media (max-width: 900px) { .monitor-grid { grid-template-columns: 1fr; } }

.temps-grid { display: flex; flex-direction: column; gap: 10px; }
.temp-item { display: flex; align-items: center; gap: 8px; }
.temp-label { font-size: 12px; color: var(--text-secondary); min-width: 80px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.temp-bar-track { height: 6px; border-radius: 99px; background: var(--bg-elevated); border: 1px solid var(--border); overflow: hidden; }
.temp-bar-fill { height: 100%; border-radius: 99px; transition: width 400ms ease, background 400ms ease; }
.temp-value { font-size: 12px; font-weight: 600; font-family: "JetBrains Mono", monospace; min-width: 44px; text-align: right; }

.cores-grid { display: flex; flex-direction: column; gap: 6px; }
.core-item { display: flex; align-items: center; gap: 8px; }
.core-label { font-size: 11px; color: var(--text-secondary); min-width: 22px; font-family: "JetBrains Mono", monospace; }
.mon-bar-track { flex: 1; height: 6px; border-radius: 99px; background: var(--bg-elevated); border: 1px solid var(--border); overflow: hidden; }
.mon-bar-fill { height: 100%; border-radius: 99px; background: linear-gradient(90deg, var(--accent-primary), var(--accent-hover)); transition: width 300ms ease; }
.mon-val { font-size: 11px; font-weight: 500; color: var(--text-secondary); font-family: "JetBrains Mono", monospace; min-width: 36px; text-align: right; }

.net-speeds { display: flex; flex-direction: column; gap: 10px; }
.net-item { display: flex; align-items: center; gap: 10px; }
.net-icon { width: 32px; height: 32px; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; flex-shrink: 0; }
.net-download { background: linear-gradient(135deg, rgba(34,197,94,.25), rgba(34,197,94,.08)); color: var(--success); }
.net-upload { background: linear-gradient(135deg, rgba(59,130,246,.25), rgba(59,130,246,.08)); color: var(--info); }
.net-read { background: var(--info-muted); color: var(--info); }
.net-write { background: var(--warning-muted); color: var(--warning); }
.net-lbl { font-size: 11px; color: var(--text-secondary); }
.net-val { font-size: 15px; font-weight: 600; color: var(--text-primary); font-family: "JetBrains Mono", monospace; }
.net-sep { height: 1px; background: var(--border); margin: 2px 0; }
</style>
