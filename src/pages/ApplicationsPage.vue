<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import NTabs from "@/components/ui/NTabs.vue";
import {
  Package, Download, CheckCircle, AlertTriangle,
  Globe, FileText, Code, Film, MessageSquare,
  Shield, Settings, Wifi, Gamepad2, RefreshCw,
} from "lucide-vue-next";

interface AppInfo {
  id: string;
  name: string;
  description: string;
  winget_id: string;
  category: string;
  icon?: string;
}

const loading = ref(true);
const apps = ref<AppInfo[]>([]);
const wingetOk = ref(false);
const searchQuery = ref("");
const activeCategory = ref("all");
const installingIds = ref<Set<string>>(new Set());
const installedIds = ref<Set<string>>(new Set());
const installLogs = ref<Record<string, string[]>>({});

let unlistenLog: (() => void) | null = null;

const categoryTabs = [
  { id: "all", label: "Tout" },
  { id: "navigateurs", label: "Navigateurs" },
  { id: "bureautique", label: "Bureautique" },
  { id: "dev", label: "Dev" },
  { id: "multimedia", label: "Multimedia" },
  { id: "communication", label: "Communication" },
  { id: "securite", label: "Securite" },
  { id: "systeme", label: "Systeme" },
  { id: "reseau", label: "Reseau" },
  { id: "gaming", label: "Gaming" },
];

const categoryIcons: Record<string, any> = {
  navigateurs: Globe,
  bureautique: FileText,
  dev: Code,
  multimedia: Film,
  communication: MessageSquare,
  securite: Shield,
  systeme: Settings,
  reseau: Wifi,
  gaming: Gamepad2,
};

function normalizeStr(s: string) {
  return s.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "");
}

const filteredApps = computed(() => {
  let result = apps.value;
  if (activeCategory.value !== "all") {
    result = result.filter((a) => normalizeStr(a.category) === normalizeStr(activeCategory.value));
  }
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase();
    result = result.filter(
      (a) =>
        a.name.toLowerCase().includes(q) ||
        a.description.toLowerCase().includes(q) ||
        a.winget_id.toLowerCase().includes(q)
    );
  }
  return result;
});

// Mapping catégories programs.json → ApplicationsPage
const CATEGORY_MAP: Record<string, string> = {
  "navigateurs": "navigateurs", "bureautique": "bureautique", "developpement": "dev",
  "multimedia": "multimedia", "streaming video": "multimedia", "streaming audio": "multimedia",
  "communication": "communication", "reseaux sociaux": "communication",
  "securite": "securite", "antivirus": "securite", "desinstallateurs antivirus": "securite",
  "utilitaires systeme": "systeme", "utilitaires": "systeme", "compression": "systeme",
  "outils essentiels": "systeme", "suites professionnelles": "bureautique",
  "productivite": "bureautique", "pdf et documents": "bureautique",
  "ia & assistants": "dev", "imprimantes & scan": "systeme", "services apple": "systeme",
  "internet": "reseau", "stockage cloud": "reseau",
  "jeux": "gaming",
};

const devApps: AppInfo[] = [
  // Navigateurs
  { id: "1", name: "Google Chrome", description: "Navigateur web rapide et securise", winget_id: "Google.Chrome", category: "navigateurs" },
  { id: "2", name: "Mozilla Firefox", description: "Navigateur web open source", winget_id: "Mozilla.Firefox", category: "navigateurs" },
  { id: "3", name: "Brave", description: "Navigateur axe sur la vie privee", winget_id: "Brave.Brave", category: "navigateurs" },
  { id: "4", name: "Microsoft Edge", description: "Navigateur Microsoft Chromium", winget_id: "Microsoft.Edge", category: "navigateurs" },
  { id: "5", name: "Opera GX", description: "Navigateur gaming", winget_id: "Opera.OperaGX", category: "navigateurs" },
  // Bureautique
  { id: "10", name: "LibreOffice", description: "Suite bureautique complete et gratuite", winget_id: "TheDocumentFoundation.LibreOffice", category: "bureautique" },
  { id: "11", name: "Notepad++", description: "Editeur de texte avance", winget_id: "Notepad++.Notepad++", category: "bureautique" },
  { id: "12", name: "Obsidian", description: "Prise de notes en Markdown", winget_id: "Obsidian.Obsidian", category: "bureautique" },
  { id: "13", name: "Notion", description: "Workspace collaboratif tout-en-un", winget_id: "Notion.Notion", category: "bureautique" },
  // Dev
  { id: "20", name: "Visual Studio Code", description: "Editeur de code source", winget_id: "Microsoft.VisualStudioCode", category: "dev" },
  { id: "21", name: "Git", description: "Systeme de controle de version", winget_id: "Git.Git", category: "dev" },
  { id: "22", name: "Node.js LTS", description: "Runtime JavaScript", winget_id: "OpenJS.NodeJS.LTS", category: "dev" },
  { id: "23", name: "Python 3.12", description: "Langage de programmation", winget_id: "Python.Python.3.12", category: "dev" },
  { id: "24", name: "Docker Desktop", description: "Conteneurisation d'applications", winget_id: "Docker.DockerDesktop", category: "dev" },
  { id: "25", name: "Postman", description: "Test d'API REST", winget_id: "Postman.Postman", category: "dev" },
  // Multimedia
  { id: "30", name: "VLC", description: "Lecteur multimedia universel", winget_id: "VideoLAN.VLC", category: "multimedia" },
  { id: "31", name: "OBS Studio", description: "Streaming et enregistrement video", winget_id: "OBSProject.OBSStudio", category: "multimedia" },
  { id: "32", name: "GIMP", description: "Editeur d'images open source", winget_id: "GIMP.GIMP", category: "multimedia" },
  { id: "33", name: "Audacity", description: "Editeur audio open source", winget_id: "Audacity.Audacity", category: "multimedia" },
  { id: "34", name: "Spotify", description: "Streaming musical", winget_id: "Spotify.Spotify", category: "multimedia" },
  { id: "35", name: "HandBrake", description: "Transcodeur video open source", winget_id: "HandBrake.HandBrake", category: "multimedia" },
  // Communication
  { id: "40", name: "Discord", description: "Communication vocale et textuelle", winget_id: "Discord.Discord", category: "communication" },
  { id: "41", name: "Zoom", description: "Visioconference", winget_id: "Zoom.Zoom", category: "communication" },
  { id: "42", name: "Slack", description: "Messagerie professionnelle", winget_id: "SlackTechnologies.Slack", category: "communication" },
  { id: "43", name: "Microsoft Teams", description: "Collaboration Microsoft", winget_id: "Microsoft.Teams", category: "communication" },
  { id: "44", name: "Telegram", description: "Messagerie securisee", winget_id: "Telegram.TelegramDesktop", category: "communication" },
  // Securite
  { id: "50", name: "Malwarebytes", description: "Protection anti-malware", winget_id: "Malwarebytes.Malwarebytes", category: "securite" },
  { id: "51", name: "Bitwarden", description: "Gestionnaire de mots de passe open source", winget_id: "Bitwarden.Bitwarden", category: "securite" },
  { id: "52", name: "KeePass", description: "Gestionnaire de mots de passe local", winget_id: "DominikReichl.KeePass", category: "securite" },
  { id: "53", name: "ProtonVPN", description: "VPN securise et prive", winget_id: "ProtonTechnologies.ProtonVPN", category: "securite" },
  { id: "54", name: "VeraCrypt", description: "Chiffrement de volumes", winget_id: "IDRIX.VeraCrypt", category: "securite" },
  { id: "55", name: "GlassWire", description: "Moniteur reseau et firewall", winget_id: "GlassWire.GlassWire", category: "securite" },
  // Systeme
  { id: "60", name: "7-Zip", description: "Archiveur de fichiers multi-format", winget_id: "7zip.7zip", category: "systeme" },
  { id: "61", name: "CPU-Z", description: "Informations detaillees sur le materiel", winget_id: "CPUID.CPU-Z", category: "systeme" },
  { id: "62", name: "HWiNFO64", description: "Monitoring materiel complet", winget_id: "REALiX.HWiNFO", category: "systeme" },
  { id: "63", name: "CrystalDiskInfo", description: "Sante des disques (SMART)", winget_id: "CrystalDewWorld.CrystalDiskInfo", category: "systeme" },
  { id: "64", name: "Autoruns", description: "Gestion des programmes au demarrage (Sysinternals)", winget_id: "Microsoft.Sysinternals.Autoruns", category: "systeme" },
  { id: "65", name: "Process Explorer", description: "Gestionnaire de processus avance (Sysinternals)", winget_id: "Microsoft.Sysinternals.ProcessExplorer", category: "systeme" },
  { id: "66", name: "WinDirStat", description: "Visualisation espace disque", winget_id: "WinDirStat.WinDirStat", category: "systeme" },
  { id: "67", name: "Everything", description: "Recherche de fichiers instantanee", winget_id: "voidtools.Everything", category: "systeme" },
  { id: "68", name: "TreeSize Free", description: "Analyse taille des dossiers", winget_id: "JAMSoftware.TreeSize.Free", category: "systeme" },
  { id: "69", name: "PowerToys", description: "Outils systeme Microsoft avances", winget_id: "Microsoft.PowerToys", category: "systeme" },
  { id: "70", name: "Speccy", description: "Informations systeme completes", winget_id: "Piriform.Speccy", category: "systeme" },
  { id: "71", name: "GPU-Z", description: "Informations carte graphique", winget_id: "TechPowerUp.GPU-Z", category: "systeme" },
  { id: "72", name: "Rufus", description: "Creation de cles USB bootables", winget_id: "Rufus.Rufus", category: "systeme" },
  { id: "73", name: "Ventoy", description: "Multi-boot USB open source", winget_id: "Ventoy.Ventoy", category: "systeme" },
  { id: "74", name: "ShutUp10++", description: "Confidentialite Windows 10/11", winget_id: "OO-Software.ShutUp10", category: "systeme" },
  // Reseau
  { id: "80", name: "WinSCP", description: "Client SFTP/FTP graphique", winget_id: "WinSCP.WinSCP", category: "reseau" },
  { id: "81", name: "PuTTY", description: "Client SSH et Telnet", winget_id: "PuTTY.PuTTY", category: "reseau" },
  { id: "82", name: "Advanced IP Scanner", description: "Scanner reseau local rapide", winget_id: "Famatech.AdvancedIPScanner", category: "reseau" },
  { id: "83", name: "Wireshark", description: "Analyseur de trafic reseau", winget_id: "WiresharkFoundation.Wireshark", category: "reseau" },
  { id: "84", name: "FileZilla", description: "Client FTP/SFTP/FTPS", winget_id: "TimKosse.FileZilla.Client", category: "reseau" },
  { id: "85", name: "qBittorrent", description: "Client BitTorrent open source", winget_id: "qBittorrent.qBittorrent", category: "reseau" },
  { id: "86", name: "Free Download Manager", description: "Gestionnaire de telechargements", winget_id: "SoftdeluxeGroup.FreeDownloadManager", category: "reseau" },
  { id: "87", name: "JDownloader 2", description: "Telechargeur automatique multi-hotes", winget_id: "AppWork.JDownloader", category: "reseau" },
  { id: "88", name: "NetLimiter 4", description: "Controle de bande passante", winget_id: "Locktime.NetLimiter.4", category: "reseau" },
  { id: "89", name: "mRemoteNG", description: "Gestionnaire de connexions distantes", winget_id: "mRemoteNG.mRemoteNG", category: "reseau" },
  { id: "90", name: "Cyberduck", description: "Client FTP/SFTP/WebDAV/Cloud", winget_id: "iterate.Cyberduck", category: "reseau" },
  { id: "91", name: "Nmap", description: "Scanner de ports et reseau", winget_id: "Nmap.Nmap", category: "reseau" },
  { id: "92", name: "Angry IP Scanner", description: "Scanner IP leger et rapide", winget_id: "AngryIPScanner.AngryIPScanner", category: "reseau" },
  { id: "93", name: "Speedtest CLI", description: "Test de vitesse internet (Ookla)", winget_id: "Ookla.Speedtest", category: "reseau" },
  // Gaming
  { id: "100", name: "Steam", description: "Plateforme de jeux PC principale", winget_id: "Valve.Steam", category: "gaming" },
  { id: "101", name: "Epic Games Launcher", description: "Launcher Epic Games et jeux gratuits", winget_id: "EpicGames.EpicGamesLauncher", category: "gaming" },
  { id: "102", name: "GOG Galaxy", description: "Client de jeux DRM-free GOG.com", winget_id: "GOG.Galaxy", category: "gaming" },
  { id: "103", name: "Battle.net", description: "Launcher jeux Blizzard", winget_id: "Blizzard.BattleNet", category: "gaming" },
  { id: "104", name: "Ubisoft Connect", description: "Launcher et social Ubisoft", winget_id: "Ubisoft.Connect", category: "gaming" },
  { id: "105", name: "EA App", description: "Launcher EA (remplace Origin)", winget_id: "ElectronicArts.EADesktop", category: "gaming" },
  { id: "106", name: "Xbox App", description: "Xbox et Game Pass sur PC", winget_id: "Microsoft.GamingApp", category: "gaming" },
  { id: "107", name: "Heroic Games Launcher", description: "Launcher Epic + GOG open source", winget_id: "HeroicGamesLauncher.HeroicGamesLauncher", category: "gaming" },
  { id: "108", name: "Playnite", description: "Bibliotheque de jeux unifiee", winget_id: "Playnite.Playnite", category: "gaming" },
  { id: "109", name: "MSI Afterburner", description: "Overclock GPU et monitoring", winget_id: "Guru3D.Afterburner", category: "gaming" },
  { id: "110", name: "GeForce Experience", description: "Drivers et optimisations NVIDIA", winget_id: "Nvidia.GeForceExperience", category: "gaming" },
  { id: "111", name: "Parsec", description: "Remote gaming et bureau distant", winget_id: "Parsec.Parsec", category: "gaming" },
  { id: "112", name: "Moonlight", description: "Game streaming (NVIDIA GameStream)", winget_id: "MoonlightGameStreamingProject.Moonlight", category: "gaming" },
  { id: "113", name: "RetroArch", description: "Emulateur multi-systemes", winget_id: "Libretro.RetroArch", category: "gaming" },
  { id: "114", name: "Dolphin Emulator", description: "Emulateur GameCube/Wii", winget_id: "DolphinEmu.Dolphin", category: "gaming" },
  { id: "115", name: "Cheat Engine", description: "Modification de valeurs en memoire", winget_id: "CheatEngine.CheatEngine", category: "gaming" },
  { id: "116", name: "Razer Cortex", description: "Optimiseur gaming Razer", winget_id: "RazerInc.RazerCortex", category: "gaming" },
];

function mapCategory(raw: string): string {
  return CATEGORY_MAP[normalizeStr(raw)] ?? normalizeStr(raw);
}

async function loadApps() {
  loading.value = true;
  try {
    const wg = await invoke<boolean>("check_winget");
    wingetOk.value = wg;
    const raw = await invoke<AppInfo[]>("get_apps");
    apps.value = raw.map(a => ({ ...a, category: mapCategory(a.category) }));
  } catch {
    wingetOk.value = true;
    apps.value = devApps;
  }
  loading.value = false;
}

async function installApp(app: AppInfo) {
  if (installingIds.value.has(app.id)) return;
  installingIds.value.add(app.id);
  installLogs.value[app.id] = [];
  try {
    await invoke("install_app", { wingetId: app.winget_id });
    installedIds.value.add(app.id);
  } catch {
    // Simulation dev
    await new Promise((r) => setTimeout(r, 2000));
    installLogs.value[app.id].push(`Installation de ${app.name}...`);
    await new Promise((r) => setTimeout(r, 1000));
    installedIds.value.add(app.id);
  }
  installingIds.value.delete(app.id);
}

function getCategoryIcon(category: string) {
  return categoryIcons[category] || Package;
}

onMounted(async () => {
  await loadApps();
  try {
    const { listen } = await import("@tauri-apps/api/event");
    unlistenLog = (await listen<{ app_id: string; line: string }>("install-log", (event) => {
      const { app_id, line } = event.payload;
      if (!installLogs.value[app_id]) installLogs.value[app_id] = [];
      installLogs.value[app_id].push(line);
    })) as unknown as () => void;
  } catch {
    // Mode dev
  }
});

onUnmounted(() => {
  if (unlistenLog) unlistenLog();
});
</script>

<template>
  <div class="apps-page">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1>Applications</h1>
        <p class="page-subtitle">Installer des applications via WinGet</p>
      </div>
      <div class="header-actions">
        <NBadge :variant="wingetOk ? 'success' : 'danger'">
          <Package :size="12" style="margin-right: 4px;" />
          WinGet {{ wingetOk ? "disponible" : "non detecte" }}
        </NBadge>
        <NButton variant="secondary" size="sm" :loading="loading" @click="loadApps">
          <RefreshCw :size="14" />
          Rafraichir
        </NButton>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="loading-state">
      <NSpinner :size="32" />
      <p>Chargement des applications...</p>
    </div>

    <template v-else>
      <!-- Search + Tabs -->
      <div class="toolbar">
        <NSearchBar v-model="searchQuery" placeholder="Rechercher une application..." />
      </div>

      <NTabs :tabs="categoryTabs" v-model="activeCategory">
        <template #default="{ activeTab }">
          <!-- Apps Grid -->
          <div v-if="filteredApps.length" class="apps-grid">
            <div v-for="app in filteredApps" :key="app.id" class="app-card">
              <div class="app-icon-wrap">
                <component :is="getCategoryIcon(app.category)" :size="24" />
              </div>
              <div class="app-info">
                <div class="app-name">{{ app.name }}</div>
                <div class="app-desc">{{ app.description }}</div>
                <div class="app-winget font-mono">{{ app.winget_id }}</div>
              </div>
              <div class="app-action">
                <NButton
                  v-if="installedIds.has(app.id)"
                  variant="success"
                  size="sm"
                  disabled
                >
                  <CheckCircle :size="14" />
                  Installe
                </NButton>
                <NButton
                  v-else
                  variant="primary"
                  size="sm"
                  :loading="installingIds.has(app.id)"
                  @click="installApp(app)"
                >
                  <Download :size="14" />
                  Installer
                </NButton>
              </div>
            </div>
          </div>

          <div v-else class="empty-state">
            Aucune application trouvee pour cette recherche.
          </div>
        </template>
      </NTabs>
    </template>
  </div>
</template>

<style scoped>
.apps-page {
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
  align-items: center;
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

.toolbar {
  max-width: 400px;
}

/* Apps Grid */
.apps-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 12px;
}

.app-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  transition: all var(--transition-normal);
}

.app-card:hover {
  border-color: var(--border-hover);
  box-shadow: var(--shadow-md);
}

.app-icon-wrap {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  color: var(--accent-primary);
}

.app-info {
  flex: 1;
  min-width: 0;
}

.app-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.app-desc {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.app-winget {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 2px;
}

.font-mono {
  font-family: "JetBrains Mono", monospace;
}

.app-action {
  flex-shrink: 0;
}

.empty-state {
  text-align: center;
  color: var(--text-muted);
  font-size: 13px;
  padding: 40px;
}
</style>
