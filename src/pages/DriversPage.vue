<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NTabs from "@/components/ui/NTabs.vue";
import { useNotificationStore } from "@/stores/notifications";
import { Cpu, RefreshCw, Download, FileSpreadsheet, FileText, FolderOpen, ExternalLink, CheckCircle, XCircle } from "lucide-vue-next";

const notifications = useNotificationStore();
const loading = ref(true);
const search = ref("");
const activeDriverCategory = ref("all");

const driverCategoryTabs = [
  { id: "all", label: "Tous" },
  { id: "gpu", label: "GPU" },
  { id: "audio", label: "Audio" },
  { id: "usb", label: "USB" },
  { id: "wifi", label: "WiFi" },
  { id: "reseau", label: "Reseau" },
  { id: "bluetooth", label: "Bluetooth" },
  { id: "stockage", label: "Stockage" },
  { id: "systeme", label: "Systeme" },
  { id: "autre", label: "Autre" },
];

function getDriverCategory(module: string, displayName: string): string {
  const m = (module + " " + displayName).toLowerCase();
  if (/nvidia|geforce|quadro|nvlddmkm|dxgkrnl|nvdmapx|amd.*video|radeon|ati.*display/.test(m)) return "gpu";
  if (/realtek.*audio|audio|sound|ac97|hdaudio|portcls|drmk|ks.*filter|splitter|wave/.test(m)) return "audio";
  if (/usb|usbhub|xhci|ehci|ohci|uhci|usbstor|usbvideo|usbprint|wusb/.test(m)) return "usb";
  if (/wifi|wlan|wireless|802\.11/.test(m)) return "wifi";
  if (/tcpip|ndis|ethernet|netbt|afd|lan|e1000|rtl8|rl8|broadcom.*net|intel.*net|marvell|bfe|nsi/.test(m)) return "reseau";
  if (/bluetooth|bth|rfcomm|bthport/.test(m)) return "bluetooth";
  if (/disk|nvme|sata|ahci|storahci|stornvme|ide|atapi|ntfs|fat|exfat|refs|cdrom|storport|raid/.test(m)) return "stockage";
  if (/acpi|pci|smbus|gpio|i2c|isa|wdf|clfs|hal\b|kernel|nt\b|wmi|power/.test(m)) return "systeme";
  return "autre";
}

interface DriverEntry {
  module: string;
  displayName: string;
  driverType: string;
  linkDate: string;
  state: string;
}

const drivers = ref<DriverEntry[]>([]);

const filteredDrivers = computed(() => {
  let result = drivers.value;
  if (activeDriverCategory.value !== "all") {
    result = result.filter(d => getDriverCategory(d.module, d.displayName) === activeDriverCategory.value);
  }
  const q = search.value.toLowerCase();
  if (q) {
    result = result.filter(d =>
      d.module.toLowerCase().includes(q) ||
      d.displayName.toLowerCase().includes(q) ||
      d.driverType.toLowerCase().includes(q) ||
      d.state.toLowerCase().includes(q)
    );
  }
  return result;
});

function parseCSV(csv: string): DriverEntry[] {
  const lines = csv.trim().split("\n").filter((l) => l.trim());
  if (lines.length < 2) return [];

  const result: DriverEntry[] = [];
  for (let i = 1; i < lines.length; i++) {
    const cols = lines[i].match(/("([^"]*)"|[^,]+)/g)?.map((c) => c.replace(/^"|"$/g, "").trim()) ?? [];
    if (cols.length >= 5) {
      result.push({
        module: cols[0] ?? "",
        displayName: cols[1] ?? "",
        driverType: cols[3] ?? "",
        linkDate: cols[5] ?? cols[4] ?? "",
        state: cols[2] ?? "",
      });
    }
  }
  return result;
}

async function loadDrivers() {
  loading.value = true;
  try {
    const result = await invoke<any>("run_system_command", {
      cmd: "driverquery",
      args: ["/FO", "CSV", "/V"],
    });
    const out = result?.stdout ?? result?.output ?? "";
    drivers.value = parseCSV(out);
  } catch {
    // Donnees de demo
    drivers.value = [
      { module: "1394ohci", displayName: "1394 OHCI Compliant Host Controller", driverType: "Kernel", linkDate: "21/06/2006", state: "Stopped" },
      { module: "3ware", displayName: "3ware", driverType: "Kernel", linkDate: "17/05/2015", state: "Stopped" },
      { module: "ACPI", displayName: "Microsoft ACPI Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "AcpiDev", displayName: "ACPI Devices driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Stopped" },
      { module: "acpiex", displayName: "Microsoft ACPIEx Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "AFD", displayName: "Ancillary Function Driver", driverType: "Kernel", linkDate: "11/03/2024", state: "Running" },
      { module: "ahcache", displayName: "Application Compatibility Cache", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "amdgpio2", displayName: "AMD GPIO Client Driver", driverType: "Kernel", linkDate: "01/09/2023", state: "Running" },
      { module: "amdi2c", displayName: "AMD I2C Controller Driver", driverType: "Kernel", linkDate: "01/09/2023", state: "Stopped" },
      { module: "AmdK8", displayName: "AMD K8 Processor Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Stopped" },
      { module: "Beep", displayName: "Beep", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "BthA2dp", displayName: "Microsoft Bluetooth A2dp driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Stopped" },
      { module: "CLFS", displayName: "Common Log (CLFS)", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "disk", displayName: "Disk Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "HTTP", displayName: "HTTP Service", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "intelppm", displayName: "Intel Processor Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Stopped" },
      { module: "Ndu", displayName: "Network Data Usage Monitoring Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
      { module: "Ntfs", displayName: "Ntfs", driverType: "File System", linkDate: "21/06/2006", state: "Running" },
      { module: "nvlddmkm", displayName: "NVIDIA Windows Kernel Mode Driver", driverType: "Kernel", linkDate: "15/01/2025", state: "Running" },
      { module: "Tcpip", displayName: "TCP/IP Protocol Driver", driverType: "Kernel", linkDate: "21/06/2006", state: "Running" },
    ];
  }
  loading.value = false;
}

async function getExportFolder(): Promise<string> {
  try {
    const { homeDir, join } = await import("@tauri-apps/api/path");
    return await join(await homeDir(), "Documents", "NiTriTe", "exports");
  } catch { return ""; }
}

async function ensureExportDir(folder: string) {
  if (!folder) return;
  try { const { mkdir } = await import("@tauri-apps/plugin-fs"); await mkdir(folder, { recursive: true }); } catch { /* ignore */ }
}

async function openExportFolder() {
  const folder = await getExportFolder();
  if (!folder) return;
  await ensureExportDir(folder);
  try {
    await invoke("open_path", { path: folder });
  } catch { notifications.error("Impossible d'ouvrir le dossier"); }
}

async function writeToExports(filename: string, content: string) {
  const folder = await getExportFolder();
  try {
    await ensureExportDir(folder);
    const { writeTextFile } = await import("@tauri-apps/plugin-fs");
    const { join } = await import("@tauri-apps/api/path");
    const fullPath = folder ? await join(folder, filename) : filename;
    await writeTextFile(fullPath, content);
    notifications.success(`Export ${filename} sauvegardé`);
  } catch {
    navigator.clipboard.writeText(content);
    notifications.info("Copié dans le presse-papier", "Export fichier indisponible en mode dev");
  }
}

async function exportCSV() {
  const header = "Module,Nom,Type,Date,Etat";
  const rows = filteredDrivers.value.map(
    (d) => `"${d.module}","${d.displayName}","${d.driverType}","${d.linkDate}","${d.state}"`
  );
  await writeToExports("drivers.csv", [header, ...rows].join("\n"));
}

async function exportTxt() {
  const col = (s: string, w: number) => s.padEnd(w).slice(0, w);
  const header = `${col("Module", 25)} ${col("Nom", 45)} ${col("Type", 15)} ${col("Date", 12)} Etat`;
  const sep = "-".repeat(115);
  const rows = filteredDrivers.value.map(d =>
    `${col(d.module, 25)} ${col(d.displayName, 45)} ${col(d.driverType, 15)} ${col(d.linkDate, 12)} ${d.state}`
  );
  await writeToExports("drivers.txt", [header, sep, ...rows].join("\n"));
}

function stateVariant(state: string): "success" | "warning" | "neutral" {
  if (state.toLowerCase().includes("running")) return "success";
  if (state.toLowerCase().includes("stopped")) return "neutral";
  return "warning";
}

function parseDriverDate(dateStr: string): Date | null {
  const m = dateStr.match(/(\d{1,2})\/(\d{1,2})\/(\d{4})/);
  if (!m) return null;
  return new Date(parseInt(m[3]), parseInt(m[2]) - 1, parseInt(m[1]));
}

function driverAgeInfo(dateStr: string): { label: string; variant: "success" | "warning" | "danger" | "neutral" } {
  const date = parseDriverDate(dateStr);
  if (!date) return { label: "?", variant: "neutral" };
  const ageYears = (Date.now() - date.getTime()) / (1000 * 60 * 60 * 24 * 365.25);
  if (ageYears < 1) return { label: "A jour", variant: "success" };
  if (ageYears < 2) return { label: "Ancien", variant: "warning" };
  return { label: "Obsolete", variant: "danger" };
}

// Recommended drivers
interface RecommendedDriver {
  driver: {
    id: string;
    name: string;
    description: string;
    category: string;
    url: string;
  };
  installed: boolean;
}

const recommended = ref<RecommendedDriver[]>([]);
const recommendedLoading = ref(true);

async function loadRecommended() {
  recommendedLoading.value = true;
  try {
    recommended.value = await invoke<RecommendedDriver[]>("get_recommended_drivers");
  } catch {
    recommended.value = [
      { driver: { id: "vcredist-2015-2022", name: "Visual C++ 2015-2022", description: "Runtime requis par la plupart des applications", category: "Runtime", url: "https://aka.ms/vs/17/release/vc_redist.x64.exe" }, installed: true },
      { driver: { id: "dotnet-8", name: ".NET 8 Runtime", description: "Runtime .NET 8 LTS", category: "Runtime", url: "#" }, installed: false },
      { driver: { id: "nvidia-drivers", name: "NVIDIA GeForce Drivers", description: "Pilotes GPU NVIDIA", category: "GPU", url: "#" }, installed: true },
    ];
  } finally {
    recommendedLoading.value = false;
  }
}

async function openDriverUrl(url: string) {
  try {
    await invoke("open_url", { url });
  } catch {
    window.open(url, "_blank");
  }
}

const recommendedCategories = computed(() => {
  const cats = new Map<string, RecommendedDriver[]>();
  for (const r of recommended.value) {
    const cat = r.driver.category;
    if (!cats.has(cat)) cats.set(cat, []);
    cats.get(cat)!.push(r);
  }
  return cats;
});

onMounted(() => {
  loadDrivers();
  loadRecommended();
});
</script>

<template>
  <div class="drivers-page">
    <div class="page-header">
      <div>
        <h1>Drivers</h1>
        <p class="page-subtitle">Liste des pilotes systeme installes</p>
      </div>
      <div class="header-actions">
        <NButton variant="ghost" size="sm" @click="openExportFolder"><FolderOpen :size="14" /> Exports</NButton>
        <NButton variant="secondary" size="sm" @click="exportTxt"><FileText :size="14" /> TXT</NButton>
        <NButton variant="secondary" size="sm" @click="exportCSV">
          <FileSpreadsheet :size="14" />
          CSV
        </NButton>
        <NButton variant="primary" size="sm" :loading="loading" @click="loadDrivers">
          <RefreshCw :size="14" />
          Rafraichir
        </NButton>
      </div>
    </div>

    <!-- Recommended Drivers -->
    <NCard>
      <template #header>
        <div class="section-header">
          <Download :size="16" />
          <span>Telechargements recommandes</span>
          <NButton variant="secondary" size="sm" :loading="recommendedLoading" @click="loadRecommended" style="margin-left: auto">
            <RefreshCw :size="14" />
          </NButton>
        </div>
      </template>
      <div v-if="recommendedLoading" class="loading-state"><NSpinner :size="24" /><p>Detection...</p></div>
      <div v-else class="rec-drivers">
        <div v-for="[cat, items] in recommendedCategories" :key="cat" class="rec-category">
          <h4 class="rec-cat-title">{{ cat }}</h4>
          <div class="rec-list">
            <div v-for="r in items" :key="r.driver.id" class="rec-item">
              <div class="rec-status">
                <CheckCircle v-if="r.installed" :size="16" style="color: var(--success)" />
                <XCircle v-else :size="16" style="color: var(--text-muted)" />
              </div>
              <div class="rec-info">
                <span class="rec-name">{{ r.driver.name }}</span>
                <span class="rec-desc">{{ r.driver.description }}</span>
              </div>
              <NBadge v-if="r.installed" variant="success">Installe</NBadge>
              <NButton v-else variant="primary" size="sm" @click="openDriverUrl(r.driver.url)">
                <ExternalLink :size="12" /> Telecharger
              </NButton>
            </div>
          </div>
        </div>
      </div>
    </NCard>

    <NSearchBar v-model="search" placeholder="Rechercher un pilote..." />

    <div v-if="loading" class="loading-state">
      <NSpinner :size="32" />
      <p>Chargement des pilotes...</p>
    </div>

    <template v-else>
      <NTabs :tabs="driverCategoryTabs" v-model="activeDriverCategory">
        <template #default>
          <NCard>
            <template #header>
              <div class="section-header">
                <Cpu :size="16" />
                <span>Pilotes ({{ filteredDrivers.length }})</span>
              </div>
            </template>
            <div class="table-wrapper">
              <table class="drivers-table">
                <thead>
                  <tr>
                    <th>Module</th>
                    <th>Nom</th>
                    <th>Type</th>
                    <th>Date</th>
                    <th>Age</th>
                    <th>Etat</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="d in filteredDrivers" :key="d.module" :class="`age-${driverAgeInfo(d.linkDate).variant}`">
                    <td class="font-mono">{{ d.module }}</td>
                    <td>{{ d.displayName }}</td>
                    <td>{{ d.driverType }}</td>
                    <td class="font-mono">{{ d.linkDate }}</td>
                    <td><NBadge :variant="driverAgeInfo(d.linkDate).variant">{{ driverAgeInfo(d.linkDate).label }}</NBadge></td>
                    <td><NBadge :variant="stateVariant(d.state)">{{ d.state }}</NBadge></td>
                  </tr>
                  <tr v-if="filteredDrivers.length === 0">
                    <td colspan="6" class="empty-row">Aucun pilote dans cette categorie</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </NCard>
        </template>
      </NTabs>
    </template>
  </div>
</template>

<style scoped>
.drivers-page {
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

.table-wrapper {
  overflow-x: auto;
}

.drivers-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.drivers-table th {
  text-align: left;
  padding: 10px 12px;
  color: var(--text-muted);
  font-weight: 500;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 1px solid var(--border);
  white-space: nowrap;
}

.drivers-table td {
  padding: 8px 12px;
  color: var(--text-secondary);
  border-bottom: 1px solid var(--border);
}

.drivers-table tbody tr:hover {
  background: var(--bg-tertiary);
}

.drivers-table tbody tr.age-warning {
  border-left: 3px solid var(--warning);
}

.drivers-table tbody tr.age-danger {
  border-left: 3px solid var(--danger);
}

.drivers-table tbody tr.age-success {
  border-left: 3px solid transparent;
}

.font-mono {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
}

.empty-row {
  text-align: center;
  color: var(--text-muted);
  padding: 24px !important;
}

/* Recommended Drivers */
.rec-drivers { display: flex; flex-direction: column; gap: 16px; }
.rec-cat-title {
  font-size: 12px;
  font-weight: 700;
  color: var(--accent-primary);
  text-transform: uppercase;
  letter-spacing: 0.8px;
  margin-bottom: 6px;
  padding: 4px 8px;
  background: color-mix(in srgb, var(--accent-primary) 10%, transparent);
  border-left: 3px solid var(--accent-primary);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}
.rec-list { display: flex; flex-direction: column; gap: 2px; }

.rec-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  transition: background var(--transition-fast);
}
.rec-item:hover { background: var(--bg-tertiary); }

.rec-status { flex-shrink: 0; }
.rec-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.rec-name { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.rec-desc { font-size: 11px; color: var(--text-muted); }
</style>
