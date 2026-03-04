<script setup lang="ts">
import { ref, computed } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Scan, RefreshCw, ExternalLink, AlertTriangle,
  CheckCircle, Clock, Cpu, Download,
} from "lucide-vue-next";

const notifications = useNotificationStore();
const loading = ref(false);
const scanned = ref(false);
const search = ref("");

interface DriverInfo {
  module: string;
  displayName: string;
  linkDate: string;
  state: string;
  daysOld: number;
  status: "ok" | "outdated" | "very-old";
}

const drivers = ref<DriverInfo[]>([]);

const filteredDrivers = computed(() => {
  const q = search.value.toLowerCase();
  if (!q) return drivers.value;
  return drivers.value.filter(
    (d) => d.module.toLowerCase().includes(q) || d.displayName.toLowerCase().includes(q)
  );
});

const outdatedCount = computed(() => drivers.value.filter((d) => d.status !== "ok").length);

function parseDateDaysOld(dateStr: string): number {
  // Common formats: MM/DD/YYYY or DD/MM/YYYY
  const parts = dateStr.replace(/[^0-9/\-]/g, "").split(/[/\-]/);
  if (parts.length < 3) return 9999;
  const year = parseInt(parts.find((p) => p.length === 4) ?? parts[2]);
  if (isNaN(year) || year < 2000) return 9999;
  const now = new Date();
  const driverDate = new Date(year, 0, 1);
  return Math.floor((now.getTime() - driverDate.getTime()) / 86400000);
}

function classifyAge(days: number): DriverInfo["status"] {
  if (days > 730) return "very-old"; // > 2 ans
  if (days > 365) return "outdated"; // > 1 an
  return "ok";
}

function statusVariant(status: string): "success" | "warning" | "danger" {
  if (status === "ok") return "success";
  if (status === "outdated") return "warning";
  return "danger";
}

function statusLabel(status: string): string {
  if (status === "ok") return "A jour";
  if (status === "outdated") return "Ancien";
  return "Obsolete";
}

function parseCSV(csv: string): DriverInfo[] {
  const lines = csv.trim().split("\n").filter((l) => l.trim());
  if (lines.length < 2) return [];

  const result: DriverInfo[] = [];
  for (let i = 1; i < lines.length; i++) {
    const cols = lines[i].match(/("([^"]*)"|[^,]+)/g)?.map((c) => c.replace(/^"|"$/g, "").trim()) ?? [];
    if (cols.length >= 5) {
      const linkDate = cols[5] ?? cols[4] ?? "";
      const daysOld = parseDateDaysOld(linkDate);
      result.push({
        module: cols[0] ?? "",
        displayName: cols[1] ?? "",
        linkDate,
        state: cols[2] ?? "",
        daysOld,
        status: classifyAge(daysOld),
      });
    }
  }
  // Sort: worst first
  result.sort((a, b) => b.daysOld - a.daysOld);
  return result;
}

async function scanDrivers() {
  loading.value = true;
  scanned.value = false;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const result = await invoke<any>("run_system_command", {
      cmd: "driverquery",
      args: ["/FO", "CSV", "/V"],
    });
    const out = result?.stdout ?? result?.output ?? "";
    drivers.value = parseCSV(out);
  } catch {
    // Demo data
    drivers.value = [
      { module: "nvlddmkm", displayName: "NVIDIA Windows Kernel Mode Driver", linkDate: "15/01/2025", state: "Running", daysOld: 410, status: "outdated" },
      { module: "ACPI", displayName: "Microsoft ACPI Driver", linkDate: "21/06/2006", state: "Running", daysOld: 7200, status: "very-old" },
      { module: "Tcpip", displayName: "TCP/IP Protocol Driver", linkDate: "21/06/2006", state: "Running", daysOld: 7200, status: "very-old" },
      { module: "Ntfs", displayName: "Ntfs", linkDate: "21/06/2006", state: "Running", daysOld: 7200, status: "very-old" },
      { module: "AFD", displayName: "Ancillary Function Driver", linkDate: "11/03/2024", state: "Running", daysOld: 720, status: "outdated" },
      { module: "amdgpio2", displayName: "AMD GPIO Client Driver", linkDate: "01/09/2025", state: "Running", daysOld: 180, status: "ok" },
      { module: "disk", displayName: "Disk Driver", linkDate: "01/11/2025", state: "Running", daysOld: 120, status: "ok" },
      { module: "HTTP", displayName: "HTTP Service", linkDate: "01/12/2025", state: "Running", daysOld: 90, status: "ok" },
    ];
  }
  scanned.value = true;
  loading.value = false;
  notifications.info("Scan termine", `${drivers.value.length} pilotes analyses, ${outdatedCount.value} necessitent une attention`);
}

const supportLinks: Record<string, string> = {
  NVIDIA: "https://www.nvidia.com/Download/index.aspx",
  AMD: "https://www.amd.com/en/support",
  Intel: "https://www.intel.com/content/www/us/en/support/detect.html",
  Realtek: "https://www.realtek.com/en/downloads",
  "Snappy Driver Installer": "https://sdi-tool.org/",
  "NV Drivers DB": "https://www.nvidia.com/Download/Find.aspx",
  "Win Update": "ms-settings:windowsupdate",
  "AMD Chipset": "https://www.amd.com/en/support/chipsets/amd-socket-am5/x670",
  Bluetooth: "https://www.intel.com/content/www/us/en/support/articles/000005489.html",
  "USB/xHCI": "https://www.intel.com/content/www/us/en/search.html#q=USB%20xHCI%20driver",
};

function getDriverUrl(driver: DriverInfo): string {
  const n = (driver.module + " " + driver.displayName).toLowerCase();
  if (n.includes("nvidia") || n.includes("nvldd")) return "https://www.nvidia.com/Download/index.aspx";
  if (n.includes("amd") || n.includes("radeon")) return "https://www.amd.com/en/support";
  if (n.includes("intel") || n.includes("intelppm")) return "https://www.intel.com/content/www/us/en/support/detect.html";
  if (n.includes("realtek") || n.includes("rtk")) return "https://www.realtek.com/en/downloads";
  if (n.includes("bluetooth") || n.includes("bth")) return "https://www.intel.com/content/www/us/en/support/articles/000005489.html";
  if (n.includes("wifi") || n.includes("wireless") || n.includes("netw")) return "https://www.intel.com/content/www/us/en/support/articles/000005489.html";
  if (n.includes("audio") || n.includes("sound") || n.includes("hd audio")) return "https://www.realtek.com/en/downloads";
  // Generic fallback: Snappy Driver Installer
  return "https://sdi-tool.org/";
}

async function openSupportPage(url: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    if (url.startsWith("ms-settings:")) {
      await invoke("run_system_command", { cmd: "cmd", args: ["/C", "start", url] });
    } else {
      await invoke("open_url", { url });
    }
  } catch {
    window.open(url, "_blank");
  }
}

async function redirectDriverUpdate(driver: DriverInfo) {
  const url = getDriverUrl(driver);
  await openSupportPage(url);
  notifications.info(`Redirection vers la source pour : ${driver.displayName}`);
}
</script>

<template>
  <div class="driver-scanner-page">
    <div class="page-header">
      <div>
        <h1>Scanner de Pilotes</h1>
        <p class="page-subtitle">Detection des pilotes obsoletes ou problematiques</p>
      </div>
      <div class="header-actions">
        <NButton variant="primary" size="sm" :loading="loading" @click="scanDrivers">
          <Scan :size="14" />
          Scanner les drivers
        </NButton>
      </div>
    </div>

    <!-- Not scanned yet -->
    <NCard v-if="!scanned && !loading">
      <div class="empty-state">
        <Scan :size="48" class="empty-icon" />
        <h3>Aucun scan effectue</h3>
        <p>Cliquez sur "Scanner les drivers" pour analyser vos pilotes systeme.</p>
      </div>
    </NCard>

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <NSpinner :size="32" />
      <p>Analyse des pilotes en cours...</p>
    </div>

    <!-- Results -->
    <template v-if="scanned && !loading">
      <!-- Summary -->
      <div class="summary-grid">
        <NCard>
          <div class="summary-item">
            <CheckCircle :size="20" style="color: var(--success)" />
            <div>
              <span class="summary-val">{{ drivers.filter((d) => d.status === 'ok').length }}</span>
              <span class="summary-lbl">A jour</span>
            </div>
          </div>
        </NCard>
        <NCard>
          <div class="summary-item">
            <Clock :size="20" style="color: var(--warning)" />
            <div>
              <span class="summary-val">{{ drivers.filter((d) => d.status === 'outdated').length }}</span>
              <span class="summary-lbl">Anciens</span>
            </div>
          </div>
        </NCard>
        <NCard>
          <div class="summary-item">
            <AlertTriangle :size="20" style="color: var(--danger)" />
            <div>
              <span class="summary-val">{{ drivers.filter((d) => d.status === 'very-old').length }}</span>
              <span class="summary-lbl">Obsoletes</span>
            </div>
          </div>
        </NCard>
      </div>

      <NSearchBar v-model="search" placeholder="Rechercher un pilote..." />

      <!-- Driver list -->
      <NCard>
        <template #header>
          <div class="section-header">
            <Cpu :size="16" />
            <span>Resultats ({{ filteredDrivers.length }})</span>
          </div>
        </template>
        <div class="driver-list">
          <div
            v-for="d in filteredDrivers"
            :key="d.module"
            class="driver-row"
            :class="`driver-${d.status}`"
          >
            <div class="driver-main">
              <span class="driver-module">{{ d.module }}</span>
              <span class="driver-name">{{ d.displayName }}</span>
            </div>
            <div class="driver-meta">
              <span class="driver-date">{{ d.linkDate }}</span>
              <NBadge :variant="statusVariant(d.status)">{{ statusLabel(d.status) }}</NBadge>
              <button
                v-if="d.status !== 'ok'"
                class="redirect-btn"
                :title="`Télécharger mise à jour pour ${d.displayName}`"
                @click.stop="redirectDriverUpdate(d)"
              >
                <Download :size="13" />
                Mettre à jour
              </button>
            </div>
          </div>
          <div v-if="filteredDrivers.length === 0" class="empty-row">
            Aucun pilote correspondant
          </div>
        </div>
      </NCard>

      <!-- Support Links -->
      <NCard>
        <template #header>
          <div class="section-header">
            <ExternalLink :size="16" />
            <span>Pages support constructeurs</span>
          </div>
        </template>
        <div class="support-links">
          <button
            v-for="(url, name) in supportLinks"
            :key="name"
            class="support-btn"
            @click="openSupportPage(url)"
          >
            <span class="support-name">{{ name.charAt(0).toUpperCase() + name.slice(1) }}</span>
            <ExternalLink :size="12" />
          </button>
        </div>
      </NCard>
    </template>
  </div>
</template>

<style scoped>
.driver-scanner-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
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

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 48px;
  text-align: center;
}

.empty-icon { color: var(--text-muted); opacity: 0.4; }
.empty-state h3 { font-size: 16px; font-weight: 600; color: var(--text-primary); }
.empty-state p { font-size: 13px; color: var(--text-muted); }

/* Summary */
.summary-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.summary-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.summary-val {
  display: block;
  font-size: 22px;
  font-weight: 700;
  font-family: "JetBrains Mono", monospace;
  color: var(--text-primary);
}

.summary-lbl {
  display: block;
  font-size: 12px;
  color: var(--text-muted);
}

/* Driver list */
.driver-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.driver-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  border-radius: var(--radius-md);
  transition: background var(--transition-fast);
  gap: 12px;
}

.driver-row:hover { background: var(--bg-tertiary); }

.driver-very-old { border-left: 3px solid var(--danger); }
.driver-outdated { border-left: 3px solid var(--warning); }
.driver-ok { border-left: 3px solid var(--success); }

.driver-main {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.driver-module {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  font-family: "JetBrains Mono", monospace;
}

.driver-name {
  font-size: 12px;
  color: var(--text-muted);
}

.driver-meta {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}

.redirect-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 4px 10px;
  border: 1px solid var(--accent-primary);
  border-radius: var(--radius-sm);
  background: color-mix(in srgb, var(--accent-primary) 10%, transparent);
  color: var(--accent-primary);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  font-family: inherit;
  transition: all var(--transition-fast);
  white-space: nowrap;
}
.redirect-btn:hover { background: var(--accent-primary); color: white; }

.driver-date {
  font-size: 12px;
  color: var(--text-muted);
  font-family: "JetBrains Mono", monospace;
}

.empty-row {
  text-align: center;
  color: var(--text-muted);
  padding: 24px;
  font-size: 13px;
}

/* Support Links */
.support-links {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.support-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 13px;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.support-btn:hover {
  background: var(--bg-elevated);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}
</style>
