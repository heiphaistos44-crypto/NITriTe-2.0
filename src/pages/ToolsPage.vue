<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import NTabs from "@/components/ui/NTabs.vue";
import NBadge from "@/components/ui/NBadge.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import {
  Wrench, Play, ExternalLink,
  ShieldCheck, Stethoscope, Trash2,
  Wifi, Settings, Download, Headphones,
} from "lucide-vue-next";

interface ToolInfo {
  id: string;
  name: string;
  description: string;
  command: string;
  is_url: boolean;
  category: string;
  section?: string;
  icon?: string;
}

const loading = ref(true);
const tools = ref<ToolInfo[]>([]);
const searchQuery = ref("");
const activeCategory = ref("all");
const launchingId = ref<string | null>(null);

const categoryTabs = [
  { id: "all", label: "Tout" },
  { id: "reparation", label: "Réparation" },
  { id: "diagnostics", label: "Diagnostics" },
  { id: "nettoyage", label: "Nettoyage" },
  { id: "reseau", label: "Réseau" },
  { id: "parametres", label: "Paramètres" },
  { id: "telechargements", label: "Téléchargements" },
  { id: "fabricants", label: "Fabricants" },
  { id: "benchmark", label: "Benchmark" },
  { id: "fournisseurs", label: "Fournisseurs" },
  { id: "activation", label: "Activation" },
  { id: "winget", label: "WinGet" },
  { id: "documentation", label: "Docs" },
  { id: "drivers", label: "Drivers" },
];

const categoryIcons: Record<string, any> = {
  reparation: ShieldCheck,
  diagnostics: Stethoscope,
  nettoyage: Trash2,
  reseau: Wifi,
  parametres: Settings,
  telechargements: Download,
  fabricants: Headphones,
};

const filteredTools = computed(() => {
  let result = tools.value;
  if (activeCategory.value !== "all") {
    result = result.filter((t) => t.category === activeCategory.value);
  }
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase();
    result = result.filter(
      (t) =>
        t.name.toLowerCase().includes(q) ||
        t.description.toLowerCase().includes(q)
    );
  }
  return result;
});

const devTools: ToolInfo[] = [
  // Reparation
  { id: "1", name: "SFC /scannow", description: "Verification des fichiers systeme Windows", command: "sfc /scannow", is_url: false, category: "reparation" },
  { id: "2", name: "DISM Repair", description: "Reparation de l'image systeme Windows", command: "DISM /Online /Cleanup-Image /RestoreHealth", is_url: false, category: "reparation" },
  { id: "3", name: "chkdsk", description: "Verification et reparation du disque dur", command: "chkdsk C: /f /r", is_url: false, category: "reparation" },

  // Diagnostics
  { id: "4", name: "Moniteur de ressources", description: "Surveillance detaillee des ressources systeme", command: "resmon", is_url: false, category: "diagnostics" },
  { id: "5", name: "Informations systeme", description: "Details complets du systeme", command: "msinfo32", is_url: false, category: "diagnostics" },
  { id: "6", name: "Gestionnaire de peripheriques", description: "Gestion des pilotes et peripheriques", command: "devmgmt.msc", is_url: false, category: "diagnostics" },
  { id: "7", name: "Observateur d'evenements", description: "Journaux d'evenements Windows", command: "eventvwr.msc", is_url: false, category: "diagnostics" },

  // Nettoyage
  { id: "8", name: "Nettoyage de disque", description: "Supprimer les fichiers temporaires", command: "cleanmgr", is_url: false, category: "nettoyage" },
  { id: "9", name: "Prefetch cleanup", description: "Nettoyer le dossier Prefetch", command: "del /q /s %SystemRoot%\\Prefetch\\*", is_url: false, category: "nettoyage" },
  { id: "10", name: "Temp cleanup", description: "Nettoyer les fichiers temporaires", command: "del /q /s %TEMP%\\*", is_url: false, category: "nettoyage" },

  // Reseau
  { id: "11", name: "Reset Winsock", description: "Reinitialiser le catalogue Winsock", command: "netsh winsock reset", is_url: false, category: "reseau" },
  { id: "12", name: "Flush DNS", description: "Vider le cache DNS", command: "ipconfig /flushdns", is_url: false, category: "reseau" },
  { id: "13", name: "Reset IP", description: "Reinitialiser la configuration TCP/IP", command: "netsh int ip reset", is_url: false, category: "reseau" },

  // Parametres
  { id: "14", name: "Windows Update", description: "Parametres de mise a jour Windows", command: "ms-settings:windowsupdate", is_url: true, category: "parametres" },
  { id: "15", name: "Applications par defaut", description: "Configurer les applications par defaut", command: "ms-settings:defaultapps", is_url: true, category: "parametres" },
  { id: "16", name: "Systeme", description: "Parametres systeme generaux", command: "ms-settings:about", is_url: true, category: "parametres" },
  { id: "17", name: "Reseau & Internet", description: "Parametres reseau", command: "ms-settings:network", is_url: true, category: "parametres" },

  // Telechargements
  { id: "18", name: "CrystalDiskInfo (web)", description: "Site officiel CrystalDiskInfo — version récente", command: "https://crystalmark.info/en/software/crystaldiskinfo/", is_url: true, category: "telechargements" },
  { id: "19", name: "HWMonitor (web)", description: "Site officiel HWMonitor — version récente", command: "https://www.cpuid.com/softwares/hwmonitor.html", is_url: true, category: "telechargements" },
  { id: "20", name: "CPU-Z (web)", description: "Site officiel CPU-Z — version récente", command: "https://www.cpuid.com/softwares/cpu-z.html", is_url: true, category: "telechargements" },
  { id: "25", name: "MajorGeeks", description: "Base de logiciels systeme et utilitaires", command: "https://www.majorgeeks.com/", is_url: true, category: "telechargements" },
  { id: "26", name: "Malekal", description: "Guides securite, drivers et Windows", command: "https://www.malekal.com/", is_url: true, category: "telechargements" },
  { id: "27", name: "YggTorrent", description: "Tracker torrent francophone", command: "https://www.yggtorrent.org/auth/login", is_url: true, category: "telechargements" },
  { id: "28", name: "La Cale", description: "Portail de telechargement", command: "https://la-cale.space/login", is_url: true, category: "telechargements" },
  { id: "29", name: "Gemini Tracker", description: "Tracker torrent prive", command: "https://gemini-tracker.org/login", is_url: true, category: "telechargements" },
  { id: "30", name: "C411 Tracker", description: "Portail torrent communautaire", command: "https://staging-68d548c5bd4.c411.org/login", is_url: true, category: "telechargements" },

  // Portables locaux — Diagnostic
  { id: "p1", name: "Autoruns64", description: "Gestion avancée des entrées de démarrage Windows (Sysinternals)", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\Autoruns\\Autoruns64.exe", is_url: false, category: "diagnostics" },
  { id: "p3", name: "GetDataBack Pro", description: "Récupération de données avancée depuis des disques endommagés", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\GetDataBack Pro 5.55.Portable\\GetDataBackProPortable.exe", is_url: false, category: "diagnostics" },
  { id: "p6", name: "CPU-Z Portable", description: "Informations détaillées sur le CPU, RAM et carte mère", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\CPU-ZPortable\\CPU-ZPortable.exe", is_url: false, category: "diagnostics" },
  { id: "p7", name: "CrystalDiskInfo", description: "Analyse S.M.A.R.T. et santé des disques durs/SSD", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\CrystalDisk\\DiskInfo64.exe", is_url: false, category: "diagnostics" },
  { id: "p8", name: "CrystalDiskMark Portable", description: "Benchmark de vitesse des disques (lecture/écriture)", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\CrystalDiskMarkPortable\\CrystalDiskMarkPortable.exe", is_url: false, category: "benchmark" },
  { id: "p9", name: "HWiNFO Portable", description: "Informations matérielles complètes et monitoring temps réel", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\HWiNFOPortable\\HWiNFOPortable.exe", is_url: false, category: "diagnostics" },
  { id: "p10", name: "HWMonitor Portable", description: "Surveillance des températures, voltages et ventilateurs", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\HWMonitorPortable\\HWMonitorPortable.exe", is_url: false, category: "diagnostics" },
  { id: "p11", name: "Hard Disk Sentinel Pro", description: "Surveillance avancée de la santé des disques (HDD/SSD/NVMe)", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\Hard Disk Sentinel Pro Portable Version 4.60 Build 7377 FR\\HDSentinel.exe", is_url: false, category: "diagnostics" },
  { id: "p12", name: "Process Explorer Portable", description: "Gestionnaire de processus avancé (Sysinternals)", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\ProcessExplorerPortable\\ProcessExplorerPortable.exe", is_url: false, category: "diagnostics" },
  { id: "p13", name: "UserDiag", description: "Diagnostic utilisateur Windows complet", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\UserDiag\\UserDiag.exe", is_url: false, category: "diagnostics" },
  // Portables locaux — Nettoyage
  { id: "p2", name: "Bulk Crap Uninstaller", description: "Désinstalleur massif d'applications avec nettoyage des résidus", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\BCUninstaller_5.9.0_net6.0-windows10.0.18362.0\\BCUninstaller.exe", is_url: false, category: "nettoyage" },
  { id: "p14", name: "AdwCleaner", description: "Suppression des adwares, PUP et toolbars indésirables", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\AdwCleaner\\adwcleaner.exe", is_url: false, category: "nettoyage" },
  { id: "p15", name: "WiseCare 365", description: "Optimisation et nettoyage système complet", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\WiseCare365\\WiseCare365.exe", is_url: false, category: "nettoyage" },
  { id: "p16", name: "Wise Disk Cleaner Portable", description: "Nettoyage des fichiers inutiles et defragmentation", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\WiseDiskCleanerPortable\\WiseDiskCleanerPortable.exe", is_url: false, category: "nettoyage" },
  { id: "p17", name: "Spybot Portable", description: "Détection et suppression des logiciels espions", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\SpybotPortable\\SpybotPortable.exe", is_url: false, category: "nettoyage" },
  // Portables locaux — Drivers
  { id: "p4", name: "Display Driver Uninstaller (DDU)", description: "Désinstallation propre et complète des pilotes graphiques", command: "C:\\Users\\Momo\\Desktop\\Nitrite 2.0\\logiciel\\DDU v18.1.4.2\\Display Driver Uninstaller.exe", is_url: false, category: "drivers" },
  // WinDirStat — à télécharger
  { id: "p5", name: "WinDirStat", description: "Analyse visuelle de l'espace disque (à télécharger)", command: "https://windirstat.net/download.html", is_url: true, category: "diagnostics" },

  // Fabricants
  { id: "21", name: "Dell Support", description: "Support et pilotes Dell", command: "https://www.dell.com/support", is_url: true, category: "fabricants" },
  { id: "22", name: "HP Support", description: "Support et pilotes HP", command: "https://support.hp.com", is_url: true, category: "fabricants" },
  { id: "23", name: "Lenovo Support", description: "Support et pilotes Lenovo", command: "https://support.lenovo.com", is_url: true, category: "fabricants" },
  { id: "24", name: "ASUS Support", description: "Support et pilotes ASUS", command: "https://www.asus.com/support/", is_url: true, category: "fabricants" },
];

function normalizeCategory(section: string): string {
  const s = section.toLowerCase()
    .normalize("NFD").replace(/[\u0300-\u036f]/g, ""); // remove accents
  if (s.includes("reparation")) return "reparation";
  if (s.includes("maintenance") || s.includes("nettoyage")) return "nettoyage";
  if (s.includes("diagnostic") || s.includes("info")) return "diagnostics";
  if (s.includes("reseau") || s.includes("internet") || s.includes("network")) return "reseau";
  if (s.includes("parametre") || s.includes("ms-settings") || s.includes("windows setting")) return "parametres";
  if (s.includes("telechargement") || s.includes("download")) return "telechargements";
  if (s.includes("activation")) return "activation";
  if (s.includes("fabricant") || s.includes("support")) return "fabricants";
  if (s.includes("benchmark") || s.includes("test") || s.includes("performa")) return "benchmark";
  if (s.includes("fournisseur") || s.includes("achat")) return "fournisseurs";
  if (s.includes("winget") || s.includes("package")) return "winget";
  if (s.includes("documentation") || s.includes("doc")) return "documentation";
  if (s.includes("driver") || s.includes("pilote")) return "drivers";
  return "diagnostics";
}

async function loadTools() {
  loading.value = true;
  try {
    const raw = await invoke<any[]>("get_tools");
    tools.value = raw.map((t, i) => ({
      id: String(i),
      name: t.name,
      description: t.description || "",
      command: t.command,
      is_url: t.is_url,
      category: normalizeCategory(t.section || t.category || ""),
    }));
  } catch {
    tools.value = devTools;
  }
  loading.value = false;
}

async function launchTool(tool: ToolInfo) {
  launchingId.value = tool.id;
  try {
    await invoke("execute_tool", { command: tool.command, isUrl: tool.is_url });
  } catch {
    // Dev fallback : ouvrir l'URL dans le navigateur ou log
    if (tool.is_url && tool.command.startsWith("http")) {
      window.open(tool.command, "_blank");
    }
  }
  setTimeout(() => {
    launchingId.value = null;
  }, 500);
}

function getCategoryIcon(category: string) {
  return categoryIcons[category] || Wrench;
}

onMounted(loadTools);
</script>

<template>
  <div class="tools-page">
    <!-- Banner -->
    <DiagBanner :icon="Wrench" title="Boîte à Outils" desc="Outils Windows intégrés et utilitaires système" color="teal" />

    <!-- Header -->
    <div class="page-header">
      <div>
        <h1>Outils Systeme</h1>
        <p class="page-subtitle">Lancer des outils de maintenance et diagnostic Windows</p>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <NSpinner :size="32" />
      <p>Chargement des outils...</p>
    </div>

    <template v-else>
      <!-- Search -->
      <div class="toolbar">
        <NSearchBar v-model="searchQuery" placeholder="Rechercher un outil..." />
      </div>

      <NTabs :tabs="categoryTabs" v-model="activeCategory">
        <template #default="{ activeTab }">
          <div v-if="filteredTools.length" class="tools-grid">
            <div v-for="tool in filteredTools" :key="tool.id" class="tool-card">
              <div class="tool-icon-wrap">
                <component :is="getCategoryIcon(tool.category)" :size="20" />
              </div>
              <div class="tool-info">
                <div class="tool-name">
                  {{ tool.name }}
                  <ExternalLink v-if="tool.is_url" :size="12" class="tool-url-icon" />
                </div>
                <div class="tool-desc">{{ tool.description }}</div>
              </div>
              <NButton
                variant="secondary"
                size="sm"
                :loading="launchingId === tool.id"
                @click="launchTool(tool)"
              >
                <Play :size="14" />
                Lancer
              </NButton>
            </div>
          </div>

          <div v-else class="empty-state">
            Aucun outil trouve pour cette recherche.
          </div>
        </template>
      </NTabs>
    </template>
  </div>
</template>

<style scoped>
.tools-page {
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
  color: var(--text-secondary);
  font-size: 13px;
  margin-top: 2px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 60px;
  color: var(--text-secondary);
}

.toolbar {
  max-width: 400px;
}

/* Tools Grid */
.tools-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 10px;
}

.tool-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: 14px;
  transition: all var(--transition-normal);
}

.tool-card:hover {
  border-color: var(--accent-primary);
  box-shadow: 0 4px 20px rgba(249,115,22,.15);
  transform: translateY(-2px);
}

.tool-icon-wrap {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-md);
  background: linear-gradient(135deg, var(--accent-muted), transparent);
  box-shadow: 0 3px 10px rgba(249,115,22,.2);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  color: var(--accent-primary);
}

.tool-info {
  flex: 1;
  min-width: 0;
}

.tool-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 6px;
}

.tool-url-icon {
  color: var(--text-muted);
}

.tool-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  text-align: center;
  color: var(--text-secondary);
  font-size: 13px;
  padding: 40px;
}
</style>
