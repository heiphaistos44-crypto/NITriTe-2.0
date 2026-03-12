<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Package, Download, Play, FolderOpen, RefreshCw,
  Info, HardDrive, Zap, FileCode, Image, Globe,
  Wrench, Monitor, Film, FileText, Trash2, CheckCircle,
} from "lucide-vue-next";

const notify = useNotificationStore();

interface PortableApp {
  id: string;
  name: string;
  description: string;
  category: string;
  size: string;
  url: string;
  exe_name: string;
}

const apps = ref<PortableApp[]>([]);
const installedMap = ref<Record<string, boolean>>({});
const search = ref("");
const activeCategory = ref("Tous");
const loading = ref(false);

const categories = computed(() => {
  const cats = new Set(apps.value.map((a) => a.category));
  return ["Tous", ...Array.from(cats).sort()];
});

const filteredApps = computed(() => {
  let result = apps.value;
  if (activeCategory.value !== "Tous") {
    result = result.filter((a) => a.category === activeCategory.value);
  }
  const q = search.value.toLowerCase();
  if (q) {
    result = result.filter(
      (a) =>
        a.name.toLowerCase().includes(q) ||
        a.description.toLowerCase().includes(q) ||
        a.category.toLowerCase().includes(q)
    );
  }
  return result;
});

const categoryIcon = (cat: string) => {
  const map: Record<string, any> = {
    Systeme: HardDrive,
    Developpement: FileCode,
    Reseau: Globe,
    Utilitaires: Wrench,
    Multimedia: Film,
    Bureautique: FileText,
    Nettoyage: Trash2,
  };
  return map[cat] || Package;
};

const devApps: PortableApp[] = [
  { id: "crystaldiskinfo", name: "CrystalDiskInfo", description: "Surveillance sante disques S.M.A.R.T.", category: "Systeme", size: "6 MB", url: "", exe_name: "" },
  { id: "cpuz", name: "CPU-Z", description: "Informations detaillees CPU, carte mere, RAM", category: "Systeme", size: "3 MB", url: "", exe_name: "" },
  { id: "gpuz", name: "GPU-Z", description: "Informations detaillees carte graphique", category: "Systeme", size: "9 MB", url: "", exe_name: "" },
  { id: "hwmonitor", name: "HWMonitor", description: "Moniteur de temperatures et voltages", category: "Systeme", size: "3 MB", url: "", exe_name: "" },
  { id: "notepadpp", name: "Notepad++", description: "Editeur de texte avance", category: "Developpement", size: "12 MB", url: "", exe_name: "" },
  { id: "7zip", name: "7-Zip", description: "Compression/decompression d'archives", category: "Utilitaires", size: "5 MB", url: "", exe_name: "" },
  { id: "putty", name: "PuTTY", description: "Client SSH/Telnet portable", category: "Reseau", size: "4 MB", url: "", exe_name: "" },
  { id: "winscp", name: "WinSCP", description: "Client SFTP/FTP/SCP", category: "Reseau", size: "18 MB", url: "", exe_name: "" },
  { id: "windirstat", name: "WinDirStat", description: "Analyseur d'espace disque", category: "Systeme", size: "2 MB", url: "", exe_name: "" },
  { id: "treesize", name: "TreeSize Free", description: "Analyseur d'espace disque en arborescence", category: "Systeme", size: "8 MB", url: "", exe_name: "" },
  { id: "autoruns", name: "Autoruns", description: "Gestion avancee du demarrage (Sysinternals)", category: "Systeme", size: "5 MB", url: "", exe_name: "" },
  { id: "procexp", name: "Process Explorer", description: "Gestionnaire de processus avance", category: "Systeme", size: "4 MB", url: "", exe_name: "" },
  { id: "tcpview", name: "TCPView", description: "Visualiseur connexions reseau", category: "Reseau", size: "2 MB", url: "", exe_name: "" },
  { id: "vlc", name: "VLC Portable", description: "Lecteur multimedia universel", category: "Multimedia", size: "45 MB", url: "", exe_name: "" },
  { id: "irfanview", name: "IrfanView", description: "Visionneuse d'images legere", category: "Multimedia", size: "4 MB", url: "", exe_name: "" },
  { id: "sumatrapdf", name: "SumatraPDF", description: "Lecteur PDF leger", category: "Bureautique", size: "7 MB", url: "", exe_name: "" },
  { id: "bleachbit", name: "BleachBit", description: "Nettoyeur systeme", category: "Nettoyage", size: "24 MB", url: "", exe_name: "" },
  { id: "recuva", name: "Recuva", description: "Recuperation fichiers supprimes", category: "Utilitaires", size: "8 MB", url: "", exe_name: "" },
];

async function loadApps() {
  loading.value = true;
  try {
    apps.value = await invoke<PortableApp[]>("get_portable_apps");

    // Verifier les installations
    const map: Record<string, boolean> = {};
    for (const app of apps.value) {
      map[app.id] = await invoke<boolean>("check_portable_installed", { appId: app.id });
    }
    installedMap.value = map;
  } catch {
    apps.value = devApps;
  }
  loading.value = false;
}

async function downloadApp(app: PortableApp) {
  try {
    // Ouvrir l'URL de telechargement dans le navigateur
    await invoke("open_url", { url: app.url });
    notify.info(`Ouverture du telechargement pour ${app.name}. Placez les fichiers dans le dossier logiciel/${app.id}/`);
  } catch {
    notify.error("Erreur lors de l'ouverture du lien");
  }
}

async function launchApp(app: PortableApp) {
  try {
    await invoke("launch_portable", { appId: app.id });
    notify.success(`${app.name} lance`);
  } catch (e: any) {
    notify.error(e?.toString() || `Impossible de lancer ${app.name}`);
  }
}

async function openPortablesFolder() {
  try {
    await invoke("open_portables_dir");
  } catch {
    notify.error("Impossible d'ouvrir le dossier");
  }
}

onMounted(loadApps);
</script>

<template>
  <div class="portables-page">
    <div class="page-header">
      <div>
        <h1><Package :size="22" /> Applications Portables</h1>
        <p class="page-subtitle">{{ apps.length }} applications sans installation, utilisables depuis une cle USB</p>
      </div>
      <div class="header-actions">
        <NButton variant="secondary" size="sm" @click="openPortablesFolder">
          <FolderOpen :size="14" />
          Ouvrir le dossier
        </NButton>
        <NButton variant="primary" size="sm" @click="loadApps" :loading="loading">
          <RefreshCw :size="14" />
          Actualiser
        </NButton>
      </div>
    </div>

    <!-- Info card -->
    <NCard>
      <div class="info-banner">
        <Info :size="20" style="color: var(--accent-primary); flex-shrink: 0;" />
        <div>
          <p class="info-title">Comment utiliser les applications portables ?</p>
          <p class="info-text">
            Telechargez l'application, extrayez-la dans le dossier <code>logiciel/&lt;nom&gt;/</code>
            a cote de NiTriTe, puis lancez-la directement depuis cette page. Aucune installation Windows requise.
          </p>
        </div>
      </div>
    </NCard>

    <!-- Filters -->
    <div class="filters-row">
      <NSearchBar v-model="search" placeholder="Rechercher une application portable..." />
      <div class="cat-filters">
        <button
          v-for="cat in categories"
          :key="cat"
          class="cat-btn"
          :class="{ active: activeCategory === cat }"
          @click="activeCategory = cat"
        >
          <component :is="cat === 'Tous' ? Package : categoryIcon(cat)" :size="12" />
          {{ cat }}
        </button>
      </div>
    </div>

    <!-- Apps grid -->
    <div class="apps-grid">
      <NCard v-for="app in filteredApps" :key="app.id" hoverable>
        <div class="app-card">
          <div class="app-icon-wrap">
            <component :is="categoryIcon(app.category)" :size="24" style="color: var(--accent-primary)" />
          </div>
          <div class="app-details">
            <div class="app-top">
              <span class="app-name">{{ app.name }}</span>
              <NBadge v-if="installedMap[app.id]" variant="success">
                <CheckCircle :size="10" /> Installe
              </NBadge>
              <NBadge v-else variant="neutral">{{ app.category }}</NBadge>
            </div>
            <p class="app-desc">{{ app.description }}</p>
            <div class="app-bottom">
              <span class="app-size">{{ app.size }}</span>
              <div class="app-actions">
                <NButton
                  v-if="installedMap[app.id]"
                  variant="primary"
                  size="sm"
                  @click="launchApp(app)"
                >
                  <Play :size="14" />
                  Lancer
                </NButton>
                <NButton variant="secondary" size="sm" @click="downloadApp(app)">
                  <Download :size="14" />
                  {{ installedMap[app.id] ? 'MAJ' : 'Telecharger' }}
                </NButton>
              </div>
            </div>
          </div>
        </div>
      </NCard>
    </div>

    <div v-if="filteredApps.length === 0" class="empty-state">
      <Package :size="40" class="empty-icon" />
      <p>Aucune application portable ne correspond a votre recherche</p>
    </div>
  </div>
</template>

<style scoped>
.portables-page {
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
.page-header h1 {
  font-size: 22px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 10px;
}
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }
.header-actions { display: flex; gap: 8px; }

.info-banner {
  display: flex;
  gap: 12px;
  align-items: flex-start;
}
.info-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}
.info-text {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.6;
}
.info-text code {
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 4px;
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  color: var(--accent-primary);
}

.filters-row {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}
.cat-filters {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}
.cat-btn {
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
.cat-btn:hover { background: var(--bg-tertiary); color: var(--text-primary); }
.cat-btn.active { background: var(--accent-muted); color: var(--accent-primary); border-color: var(--accent-primary); }

.apps-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 12px;
}

.app-card {
  display: flex;
  gap: 14px;
}
.app-icon-wrap {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  background: var(--accent-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.app-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.app-top {
  display: flex;
  align-items: center;
  gap: 8px;
}
.app-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}
.app-desc {
  font-size: 12px;
  color: var(--text-muted);
}
.app-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 4px;
}
.app-size {
  font-size: 12px;
  color: var(--text-muted);
  font-family: "JetBrains Mono", monospace;
}
.app-actions {
  display: flex;
  gap: 6px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 40px;
  text-align: center;
}
.empty-icon { color: var(--text-muted); opacity: 0.4; }
.empty-state p { color: var(--text-muted); font-size: 14px; }
</style>
