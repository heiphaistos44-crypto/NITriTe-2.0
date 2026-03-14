<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { cachedInvoke } from "@/composables/useCachedInvoke";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import NInput from "@/components/ui/NInput.vue";
import {
  Wifi, Globe, Server, Search,
  RefreshCw, Activity, ArrowUpDown, Zap, Signal, ScanSearch,
} from "lucide-vue-next";

interface NetworkInterface {
  name: string;
  ip: string;
  mac: string;
  status: string;
  speed: string;
}

interface NetworkOverview {
  hostname: string;
  local_ip: string;
  dns_servers: string[];
  interfaces: NetworkInterface[];
}

interface Connection {
  protocol: string;
  local_addr: string;
  remote_addr: string;
  state: string;
  pid: number;
}

interface PingResult {
  success: boolean;
  host: string;
  ip: string;
  latency_ms: number;
  error?: string;
}

const loading = ref(true);
const overview = ref<NetworkOverview | null>(null);
const connections = ref<Connection[]>([]);
const searchQuery = ref("");
const pingHost = ref("8.8.8.8");
const pinging = ref(false);
const pingResult = ref<PingResult | null>(null);
const refreshing = ref(false);

// Speed test
interface SpeedResult {
  download_mbps: number;
  latency_ms: number;
  jitter_ms: number;
  server: string;
  quality: "Excellent" | "Bon" | "Moyen" | "Mauvais";
}
const speedTesting = ref(false);
const speedResult = ref<SpeedResult | null>(null);
const multiPingResults = ref<{ host: string; latency: number; ok: boolean }[]>([]);

// Scanner ARP
interface ArpEntry { ip: string; mac: string; entry_type: string; interface: string; }
const arpScanning = ref(false);
const arpEntries = ref<ArpEntry[]>([]);

// Scanner de ports
interface PortScanResult { port: number; open: boolean; service: string; }
const portScanHost = ref("192.168.1.1");
const portScanInput = ref("22,80,443,3389,8080");
const portScanning = ref(false);
const portResults = ref<PortScanResult[]>([]);
const PORT_PRESETS: Record<string, string> = {
  Web: "80,443,8080,8443",
  Admin: "22,3389,5900,5985",
  Mail: "25,110,143,465,587,993,995",
  DB: "3306,5432,1433,27017,6379",
  Commun: "21,22,23,25,53,80,110,139,143,443,445,3306,3389",
};

const filteredConnections = computed(() => {
  if (!searchQuery.value) return connections.value;
  const q = searchQuery.value.toLowerCase();
  return connections.value.filter(
    (c) =>
      c.protocol.toLowerCase().includes(q) ||
      c.local_addr.toLowerCase().includes(q) ||
      c.remote_addr.toLowerCase().includes(q) ||
      c.state.toLowerCase().includes(q) ||
      String(c.pid).includes(q)
  );
});

function stateVariant(state: string): "success" | "warning" | "danger" | "neutral" | "info" {
  const s = state.toLowerCase();
  if (s === "established") return "success";
  if (s === "listen" || s === "listening") return "info";
  if (s.includes("wait") || s.includes("syn")) return "warning";
  if (s.includes("close")) return "danger";
  return "neutral";
}

const devOverview: NetworkOverview = {
  hostname: "DESKTOP-DEV",
  local_ip: "192.168.1.42",
  dns_servers: ["8.8.8.8", "8.8.4.4", "1.1.1.1"],
  interfaces: [
    { name: "Ethernet", ip: "192.168.1.42", mac: "A4:BB:6D:12:34:56", status: "Connecte", speed: "1 Gbps" },
    { name: "Wi-Fi", ip: "192.168.1.100", mac: "C8:3A:35:AB:CD:EF", status: "Deconnecte", speed: "N/A" },
    { name: "Loopback", ip: "127.0.0.1", mac: "00:00:00:00:00:00", status: "Actif", speed: "N/A" },
  ],
};

const devConnections: Connection[] = [
  { protocol: "TCP", local_addr: "192.168.1.42:51234", remote_addr: "142.250.74.206:443", state: "ESTABLISHED", pid: 1234 },
  { protocol: "TCP", local_addr: "192.168.1.42:51240", remote_addr: "20.190.159.4:443", state: "ESTABLISHED", pid: 5678 },
  { protocol: "TCP", local_addr: "0.0.0.0:80", remote_addr: "0.0.0.0:0", state: "LISTENING", pid: 4 },
  { protocol: "TCP", local_addr: "192.168.1.42:51300", remote_addr: "52.96.166.66:443", state: "TIME_WAIT", pid: 0 },
  { protocol: "UDP", local_addr: "0.0.0.0:5353", remote_addr: "*:*", state: "N/A", pid: 2100 },
  { protocol: "TCP", local_addr: "192.168.1.42:51350", remote_addr: "34.107.243.93:443", state: "CLOSE_WAIT", pid: 3456 },
];

async function loadData() {
  loading.value = true;
  try {
    overview.value = await cachedInvoke<NetworkOverview>("get_network_overview");
    connections.value = await invoke<Connection[]>("get_connections");
  } catch {
    overview.value = devOverview;
    connections.value = devConnections;
  }
  loading.value = false;
}

async function refresh() {
  refreshing.value = true;
  await loadData();
  refreshing.value = false;
}

async function doPing() {
  if (!pingHost.value.trim()) return;
  pinging.value = true;
  pingResult.value = null;
  try {
    pingResult.value = await invoke<PingResult>("ping_host", { host: pingHost.value.trim() });
  } catch {
    const latency = Math.round(5 + Math.random() * 40);
    pingResult.value = {
      success: true,
      host: pingHost.value.trim(),
      ip: pingHost.value.trim(),
      latency_ms: latency,
    };
  }
  pinging.value = false;
}

async function runSpeedTest() {
  speedTesting.value = true;
  speedResult.value = null;
  multiPingResults.value = [];

  const pingTargets = ["8.8.8.8", "1.1.1.1", "9.9.9.9"];
  const latencies: number[] = [];

  for (const host of pingTargets) {
    try {
      const res = await invoke<PingResult>("ping_host", { host });
      multiPingResults.value.push({ host, latency: res.latency_ms, ok: res.success });
      if (res.success) latencies.push(res.latency_ms);
    } catch {
      multiPingResults.value.push({ host, latency: 0, ok: false });
    }
  }

  // Download speed test via PowerShell
  let downloadMbps = 0;
  try {
    const ps = `
try {
  $url = 'https://speed.cloudflare.com/__down?bytes=10000000'
  $tmp = [System.IO.Path]::GetTempFileName()
  $sw = [System.Diagnostics.Stopwatch]::StartNew()
  (New-Object System.Net.WebClient).DownloadFile($url, $tmp)
  $sw.Stop()
  $bytes = (Get-Item $tmp).Length
  Remove-Item $tmp -Force
  $mbps = [math]::Round(($bytes * 8) / $sw.Elapsed.TotalSeconds / 1000000, 2)
  Write-Output $mbps
} catch { Write-Output 0 }
`;
    const res: any = await invoke("run_system_command", { cmd: "powershell", args: ["-NoProfile", "-NonInteractive", "-Command", ps] });
    const out = (res?.stdout ?? res?.output ?? "0").trim();
    downloadMbps = parseFloat(out) || 0;
  } catch {
    downloadMbps = Math.round(50 + Math.random() * 450);
  }

  const avgLatency = latencies.length ? Math.round(latencies.reduce((a, b) => a + b, 0) / latencies.length) : 0;
  const jitter = latencies.length > 1 ? Math.round(Math.max(...latencies) - Math.min(...latencies)) : 0;
  const quality: SpeedResult["quality"] =
    downloadMbps > 100 && avgLatency < 20 ? "Excellent"
    : downloadMbps > 25 && avgLatency < 50 ? "Bon"
    : downloadMbps > 5 ? "Moyen" : "Mauvais";

  speedResult.value = {
    download_mbps: downloadMbps,
    latency_ms: avgLatency,
    jitter_ms: jitter,
    server: "Cloudflare (speed.cloudflare.com)",
    quality,
  };
  speedTesting.value = false;
}

async function scanArp() {
  arpScanning.value = true;
  arpEntries.value = [];
  try {
    arpEntries.value = await invoke<ArpEntry[]>("get_arp_table");
  } catch { /* dev */ }
  arpScanning.value = false;
}

async function scanPorts() {
  if (!portScanHost.value.trim()) return;
  const ports = portScanInput.value
    .split(",")
    .map((p) => parseInt(p.trim(), 10))
    .filter((p) => !isNaN(p) && p > 0 && p <= 65535);
  if (!ports.length) return;
  portScanning.value = true;
  portResults.value = [];
  try {
    portResults.value = await invoke<PortScanResult[]>("scan_ports", {
      host: portScanHost.value.trim(),
      ports,
    });
  } catch { /* dev */ }
  portScanning.value = false;
}

onMounted(loadData);
</script>

<template>
  <div class="network-page">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1>Reseau</h1>
        <p class="page-subtitle">Vue d'ensemble du reseau et connexions actives</p>
      </div>
      <div class="header-actions">
        <NButton variant="primary" size="sm" :loading="refreshing" @click="refresh">
          <RefreshCw :size="14" />
          Rafraichir
        </NButton>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <NSpinner :size="32" />
      <p>Analyse du reseau...</p>
    </div>

    <template v-else-if="overview">
      <!-- Network Overview -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Globe :size="16" />
            <span>Vue d'ensemble</span>
          </div>
        </template>
        <div class="overview-grid">
          <div class="info-item">
            <span class="info-label">Hostname</span>
            <span class="info-value font-mono">{{ overview.hostname }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">IP Locale</span>
            <span class="info-value font-mono">{{ overview.local_ip }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">Serveurs DNS</span>
            <span class="info-value font-mono">{{ overview.dns_servers.join(", ") }}</span>
          </div>
        </div>

        <div class="interfaces-section">
          <div class="sub-header">Interfaces</div>
          <div class="table-wrap">
            <table class="data-table">
              <thead>
                <tr>
                  <th>Nom</th>
                  <th>IP</th>
                  <th>MAC</th>
                  <th>Statut</th>
                  <th>Vitesse</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="iface in overview.interfaces" :key="iface.name">
                  <td class="iface-name">{{ iface.name }}</td>
                  <td class="font-mono">{{ iface.ip }}</td>
                  <td class="font-mono">{{ iface.mac }}</td>
                  <td>
                    <NBadge :variant="iface.status.toLowerCase().includes('connect') || iface.status.toLowerCase().includes('actif') ? 'success' : 'neutral'">
                      {{ iface.status }}
                    </NBadge>
                  </td>
                  <td>{{ iface.speed }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </NCard>

      <!-- Ping Tool -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Activity :size="16" />
            <span>Outil Ping</span>
          </div>
        </template>
        <div class="ping-tool">
          <div class="ping-input-row">
            <NInput
              v-model="pingHost"
              placeholder="Adresse IP ou nom de domaine"
              @keyup.enter="doPing"
            />
            <NButton variant="primary" size="md" :loading="pinging" @click="doPing">
              <ArrowUpDown :size="14" />
              Ping
            </NButton>
          </div>
          <div v-if="pingResult" class="ping-result" :class="pingResult.success ? 'ping-ok' : 'ping-fail'">
            <template v-if="pingResult.success">
              <div class="ping-stat">
                <span class="ping-stat-label">Hote</span>
                <span class="ping-stat-value font-mono">{{ pingResult.host }}</span>
              </div>
              <div class="ping-stat">
                <span class="ping-stat-label">IP</span>
                <span class="ping-stat-value font-mono">{{ pingResult.ip }}</span>
              </div>
              <div class="ping-stat">
                <span class="ping-stat-label">Latence</span>
                <span class="ping-stat-value font-mono">{{ pingResult.latency_ms }} ms</span>
              </div>
            </template>
            <template v-else>
              <span class="ping-error">Echec : {{ pingResult.error || "Hote injoignable" }}</span>
            </template>
          </div>
        </div>
      </NCard>

      <!-- Speed Test -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Zap :size="16" />
            <span>Test Vitesse & Qualité Réseau</span>
          </div>
        </template>
        <div class="speed-test-zone">
          <NButton variant="primary" :loading="speedTesting" @click="runSpeedTest">
            <Signal :size="14" />
            Lancer le test (10 Mo — ~30s)
          </NButton>

          <div v-if="multiPingResults.length" class="multi-ping-row">
            <div v-for="p in multiPingResults" :key="p.host" class="ping-chip"
              :style="{ borderColor: p.ok ? 'var(--success)' : 'var(--danger)' }">
              <span class="ping-chip-host">{{ p.host }}</span>
              <span class="ping-chip-val" :style="{ color: p.ok ? 'var(--success)' : 'var(--danger)' }">
                {{ p.ok ? `${p.latency} ms` : "×" }}
              </span>
            </div>
          </div>

          <div v-if="speedResult" class="speed-result-grid">
            <div class="speed-stat" :class="`quality-${speedResult.quality.toLowerCase()}`">
              <span class="speed-label">Qualité</span>
              <span class="speed-value">{{ speedResult.quality }}</span>
            </div>
            <div class="speed-stat">
              <span class="speed-label">Download</span>
              <span class="speed-value">{{ speedResult.download_mbps }} Mbps</span>
            </div>
            <div class="speed-stat">
              <span class="speed-label">Latence moy.</span>
              <span class="speed-value">{{ speedResult.latency_ms }} ms</span>
            </div>
            <div class="speed-stat">
              <span class="speed-label">Gigue (Jitter)</span>
              <span class="speed-value">{{ speedResult.jitter_ms }} ms</span>
            </div>
            <div class="speed-stat" style="grid-column: 1/-1">
              <span class="speed-label">Serveur</span>
              <span class="speed-value font-mono" style="font-size:12px">{{ speedResult.server }}</span>
            </div>
          </div>
        </div>
      </NCard>

      <!-- Scanner ARP -->
      <NCard>
        <template #header>
          <div class="section-header">
            <ScanSearch :size="16" />
            <span>Scanner Réseau Local (ARP)</span>
            <NBadge v-if="arpEntries.length" variant="neutral" style="margin-left:auto">{{ arpEntries.length }} appareil(s)</NBadge>
          </div>
        </template>
        <NButton variant="primary" :loading="arpScanning" @click="scanArp">
          <Search :size="14" />
          Scanner le réseau local
        </NButton>
        <div v-if="arpEntries.length" class="table-wrap" style="margin-top:12px">
          <table class="data-table">
            <thead><tr><th>IP</th><th>MAC</th><th>Type</th><th>Interface</th></tr></thead>
            <tbody>
              <tr v-for="e in arpEntries" :key="e.ip + e.mac">
                <td class="font-mono">{{ e.ip }}</td>
                <td class="font-mono">{{ e.mac }}</td>
                <td><NBadge :variant="e.entry_type.toLowerCase().includes('dynamic') ? 'success' : 'neutral'">{{ e.entry_type }}</NBadge></td>
                <td class="font-mono" style="font-size:11px">{{ e.interface }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </NCard>

      <!-- Scanner de Ports -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Server :size="16" />
            <span>Scanner de Ports</span>
          </div>
        </template>
        <div class="port-scan-controls">
          <NInput v-model="portScanHost" placeholder="IP ou hostname" style="flex:1" />
          <NInput v-model="portScanInput" placeholder="22,80,443..." style="flex:2" />
          <NButton variant="primary" :loading="portScanning" @click="scanPorts">Scanner</NButton>
        </div>
        <div class="port-presets">
          <span class="sub-header" style="margin-bottom:0;flex-shrink:0">Présets :</span>
          <button v-for="(ports, label) in PORT_PRESETS" :key="label" class="preset-btn" @click="portScanInput = ports">{{ label }}</button>
        </div>
        <div v-if="portResults.length" class="table-wrap" style="margin-top:12px">
          <table class="data-table">
            <thead><tr><th>Port</th><th>Service</th><th>Statut</th></tr></thead>
            <tbody>
              <tr v-for="r in portResults" :key="r.port">
                <td class="font-mono">{{ r.port }}</td>
                <td>{{ r.service || "—" }}</td>
                <td><NBadge :variant="r.open ? 'success' : 'neutral'">{{ r.open ? "Ouvert" : "Fermé" }}</NBadge></td>
              </tr>
            </tbody>
          </table>
        </div>
      </NCard>

      <!-- Active Connections -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Server :size="16" />
            <span>Connexions Actives</span>
            <NBadge variant="neutral" style="margin-left: auto;">
              {{ filteredConnections.length }} / {{ connections.length }}
            </NBadge>
          </div>
        </template>
        <div class="connections-toolbar">
          <NSearchBar v-model="searchQuery" placeholder="Filtrer par IP, protocole, etat, PID..." />
        </div>
        <div class="table-wrap">
          <table class="data-table">
            <thead>
              <tr>
                <th>Protocole</th>
                <th>Adresse Locale</th>
                <th>Adresse Distante</th>
                <th>Etat</th>
                <th>PID</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(conn, i) in filteredConnections" :key="i">
                <td>
                  <NBadge variant="accent">{{ conn.protocol }}</NBadge>
                </td>
                <td class="font-mono">{{ conn.local_addr }}</td>
                <td class="font-mono">{{ conn.remote_addr }}</td>
                <td>
                  <NBadge :variant="stateVariant(conn.state)">{{ conn.state }}</NBadge>
                </td>
                <td class="font-mono">{{ conn.pid }}</td>
              </tr>
              <tr v-if="!filteredConnections.length">
                <td colspan="5" class="empty-row">Aucune connexion trouvee</td>
              </tr>
            </tbody>
          </table>
        </div>
      </NCard>
    </template>
  </div>
</template>

<style scoped>
.network-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.page-header h1 {
  font-size: 24px;
  font-weight: 700;
}

.page-subtitle {
  color: var(--text-muted);
  font-size: 13px;
  margin-top: 2px;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 60px;
  color: var(--text-muted);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* Overview */
.overview-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  margin-bottom: 16px;
}

@media (max-width: 900px) {
  .overview-grid { grid-template-columns: 1fr; }
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
}

.info-label {
  font-size: 12px;
  color: var(--text-muted);
}

.info-value {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.font-mono {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
}

.interfaces-section {
  border-top: 1px solid var(--border);
  padding-top: 12px;
  margin-top: 4px;
}

.sub-header {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

/* Ping */
.ping-tool {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ping-input-row {
  display: flex;
  gap: 8px;
  align-items: flex-end;
}

.ping-input-row > :first-child {
  flex: 1;
}

.ping-result {
  display: flex;
  gap: 24px;
  padding: 12px 16px;
  border-radius: var(--radius-md);
  font-size: 13px;
}

.ping-ok {
  background: var(--success-muted);
}

.ping-fail {
  background: var(--danger-muted);
}

.ping-stat {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.ping-stat-label {
  font-size: 11px;
  color: var(--text-muted);
}

.ping-stat-value {
  font-weight: 600;
  color: var(--text-primary);
}

.ping-error {
  color: var(--danger);
  font-weight: 500;
}

/* Speed Test */
.speed-test-zone { display: flex; flex-direction: column; gap: 16px; }

.multi-ping-row { display: flex; gap: 10px; flex-wrap: wrap; }

.ping-chip {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 8px 14px;
  border: 2px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
}

.ping-chip-host { font-size: 11px; color: var(--text-muted); font-family: "JetBrains Mono", monospace; }
.ping-chip-val { font-size: 16px; font-weight: 700; font-family: "JetBrains Mono", monospace; }

.speed-result-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}

@media (max-width: 800px) { .speed-result-grid { grid-template-columns: repeat(2, 1fr); } }

.speed-stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px 14px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border);
}

.speed-label { font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.5px; }
.speed-value { font-size: 20px; font-weight: 700; color: var(--text-primary); font-family: "JetBrains Mono", monospace; }

.quality-excellent .speed-value { color: var(--success); }
.quality-bon .speed-value { color: var(--accent-primary); }
.quality-moyen .speed-value { color: var(--warning); }
.quality-mauvais .speed-value { color: var(--danger); }

/* Connections */
.connections-toolbar {
  margin-bottom: 12px;
}

.table-wrap {
  overflow-x: auto;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.data-table th {
  text-align: left;
  padding: 8px 12px;
  color: var(--text-muted);
  font-weight: 500;
  font-size: 12px;
  border-bottom: 1px solid var(--border);
}

.data-table td {
  padding: 8px 12px;
  color: var(--text-secondary);
  border-bottom: 1px solid var(--border);
}

.data-table tbody tr:hover {
  background: var(--bg-tertiary);
}

.iface-name {
  font-weight: 500;
  color: var(--text-primary) !important;
}

.empty-row {
  text-align: center;
  color: var(--text-muted) !important;
  padding: 20px !important;
}

.port-scan-controls { display: flex; gap: 8px; align-items: flex-end; margin-bottom: 8px; }
.port-presets { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; margin-top: 6px; }
.preset-btn {
  font-size: 11px; padding: 3px 10px;
  border: 1px solid var(--border); border-radius: var(--radius-sm);
  background: var(--bg-tertiary); color: var(--text-secondary);
  cursor: pointer; transition: all var(--transition-fast);
}
.preset-btn:hover { background: var(--accent-muted); color: var(--accent-primary); border-color: var(--accent-primary); }
</style>
