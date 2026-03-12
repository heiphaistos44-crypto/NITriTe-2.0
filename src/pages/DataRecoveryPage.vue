<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import { useNotificationStore } from "@/stores/notifications";
import RecoveryTabDisk from "@/components/recovery/RecoveryTabDisk.vue";
import RecoveryShadowCompare from "@/components/recovery/RecoveryShadowCompare.vue";
import {
  Database, RefreshCw, RotateCcw, Clock, Trash2,
  FileText, CheckCircle, XCircle, Folder, Search,
  Save, CheckSquare, Square, FolderOpen, HardDrive, GitCompare, Filter,
  Shield, Lightbulb,
} from "lucide-vue-next";

const notify = useNotificationStore();

interface ShadowCopy {
  id: string; volume: string; creation_time: string;
  provider: string; device_path: string;
}
interface RecoveredFile {
  name: string; path: string; size_bytes: number; deleted_date: string; source: string; is_dir: boolean;
}
interface RestoreResult { success: boolean; message: string; restored_path: string; }
interface BatchRestoreResult { success: boolean; restored_count: number; failed_count: number; message: string; }

type Tab = "shadow" | "recycle" | "mft" | "backup" | "disk";
const activeTab = ref<Tab>("shadow");

// ── Shadow Copies ─────────────────────────────────────────────
const shadows = ref<ShadowCopy[]>([]);
const loadingShadows = ref(false);
const selectedShadow = ref<ShadowCopy | null>(null);
const shadowFiles = ref<RecoveredFile[]>([]);
const browsingPath = ref(""); // chemin UNC complet courant, ou "" pour la racine
const pathStack = ref<string[]>([]); // chemins UNC complets pour le fil d'Ariane
// Affichage lisible du chemin (sans le préfixe \\?\GLOBALROOT\Device\ShadowCopyX)
const browsingDisplay = computed(() => {
  if (!browsingPath.value) return "Racine";
  const parts = browsingPath.value.split("\\").filter(Boolean);
  return parts.slice(4).join("\\") || "Racine";
});
const loadingBrowse = ref(false);
const restoreTarget = ref("C:\\NiTriTe\\Restaurés");
const selectedFiles = ref<Set<string>>(new Set());
const searchQuery = ref("");
const isSearching = ref(false);
const searchMode = ref(false);
const batchRestoring = ref(false);
const extFilter = ref(""); // E — filtre par extension (ex: ".jpg,.png")
const showCompare = ref(false); // F — panneau comparaison

// E — fichiers filtrés par extension
const filteredShadowFiles = computed(() => {
  if (!extFilter.value.trim()) return shadowFiles.value;
  const exts = extFilter.value.toLowerCase().split(",").map(e => e.trim()).filter(Boolean);
  return shadowFiles.value.filter(f => {
    if (f.is_dir) return true;
    const dot = f.name.lastIndexOf(".");
    const ext = dot >= 0 ? f.name.slice(dot).toLowerCase() : "";
    return exts.some(e => e === ext || e === ext.slice(1));
  });
});

async function loadShadows() {
  loadingShadows.value = true; shadows.value = [];
  try {
    shadows.value = await invoke<ShadowCopy[]>("list_shadow_copies");
  } catch (e: any) {
    notify.error("Erreur Shadow Copy", String(e));
  }
  loadingShadows.value = false;
}

async function browseShadow(shadow: ShadowCopy, path: string = "") {
  selectedShadow.value = shadow;
  selectedFiles.value = new Set();
  searchMode.value = false;
  searchQuery.value = "";
  // Normalise vers chemin UNC complet (shadow_path() côté Rust gère les deux formes)
  const devicePath = shadow.device_path.replace(/\\+$/, "");
  let fullPath: string;
  if (!path) {
    fullPath = "";
  } else if (path.startsWith("\\\\?\\") || path.startsWith("\\\\")) {
    fullPath = path.replace(/\\+$/, "");
  } else {
    fullPath = devicePath + "\\" + path.replace(/^\\+/, "");
  }
  // Reconstruire le fil d'Ariane avec chemins UNC complets
  if (!fullPath) {
    pathStack.value = [];
  } else {
    const rel = fullPath.startsWith(devicePath)
      ? fullPath.slice(devicePath.length).replace(/^\\+/, "")
      : fullPath.split("\\").filter(Boolean).slice(4).join("\\");
    let acc = devicePath;
    pathStack.value = rel.split("\\").filter(Boolean).map(part => {
      acc = acc + "\\" + part;
      return acc;
    });
  }
  browsingPath.value = fullPath;
  loadingBrowse.value = true;
  try {
    shadowFiles.value = await invoke<RecoveredFile[]>("browse_shadow_copy", {
      devicePath: shadow.device_path,
      relativePath: fullPath,
    });
  } catch (e: any) {
    notify.error("Erreur navigation", String(e));
  }
  loadingBrowse.value = false;
}

async function navigateInto(file: RecoveredFile) {
  if (!selectedShadow.value || !file.is_dir) return;
  // file.path de PowerShell (FullName) est déjà le chemin UNC complet
  await browseShadow(selectedShadow.value, file.path);
}

async function navigateTo(path: string) {
  if (!selectedShadow.value) return;
  await browseShadow(selectedShadow.value, path);
}

async function searchInShadow() {
  if (!selectedShadow.value || !searchQuery.value.trim()) return;
  searchMode.value = true;
  isSearching.value = true;
  selectedFiles.value = new Set();
  try {
    shadowFiles.value = await invoke<RecoveredFile[]>("search_shadow_copy", {
      devicePath: selectedShadow.value.device_path,
      query: searchQuery.value.trim(),
      basePath: browsingPath.value,
    });
  } catch (e: any) {
    notify.error("Erreur recherche", String(e));
  }
  isSearching.value = false;
}

function toggleFile(path: string) {
  const s = new Set(selectedFiles.value);
  if (s.has(path)) s.delete(path); else s.add(path);
  selectedFiles.value = s;
}
function toggleAllFiles() {
  const files = filteredShadowFiles.value.filter(f => !f.is_dir);
  if (selectedFiles.value.size === files.length) {
    selectedFiles.value = new Set();
  } else {
    selectedFiles.value = new Set(files.map(f => f.path));
  }
}

async function batchRestore() {
  if (selectedFiles.value.size === 0) return;
  batchRestoring.value = true;
  try {
    const result = await invoke<BatchRestoreResult>("restore_files_batch", {
      files: Array.from(selectedFiles.value),
      targetFolder: restoreTarget.value,
    });
    if (result.success) notify.success(`${result.restored_count} fichier(s) restauré(s)`, result.message);
    else notify.error(`Échec (${result.failed_count} erreur(s))`, result.message);
    selectedFiles.value = new Set();
  } catch (e: any) {
    notify.error("Erreur restauration", String(e));
  }
  batchRestoring.value = false;
}

async function restoreFromShadow(file: RecoveredFile) {
  try {
    const result = await invoke<RestoreResult>("restore_from_shadow", {
      sourcePath: file.path,
      targetFolder: restoreTarget.value,
    });
    if (result.success) notify.success("Restauré", `→ ${result.restored_path}`);
    else notify.error("Échec restauration", result.message);
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
}

// ── Corbeille ─────────────────────────────────────────────────
const recycleFiles = ref<RecoveredFile[]>([]);
const loadingRecycle = ref(false);

async function loadRecycleBin() {
  loadingRecycle.value = true; recycleFiles.value = [];
  try {
    recycleFiles.value = await invoke<RecoveredFile[]>("scan_recycle_bin");
  } catch (e: any) {
    notify.error("Erreur Corbeille", String(e));
  }
  loadingRecycle.value = false;
}

async function restoreRecycle(file: RecoveredFile) {
  try {
    const result = await invoke<RestoreResult>("restore_recycle_bin_item", {
      originalPath: file.path,
    });
    if (result.success) {
      notify.success("Restauré", file.name);
      recycleFiles.value = recycleFiles.value.filter(f => f.path !== file.path);
    } else {
      notify.error("Échec", result.message);
    }
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
}

// ── Journal MFT (suppressions récentes) ──────────────────────
const mftFiles = ref<RecoveredFile[]>([]);
const loadingMft = ref(false);
const mftDrive = ref("C");

async function scanMft() {
  loadingMft.value = true; mftFiles.value = [];
  try {
    mftFiles.value = await invoke<RecoveredFile[]>("scan_deleted_files", { drive: mftDrive.value });
  } catch (e: any) {
    notify.error("Erreur scan MFT", String(e));
  }
  loadingMft.value = false;
}

// G — Scan tous lecteurs NTFS
async function scanAllMft() {
  loadingMft.value = true; mftFiles.value = [];
  try {
    mftFiles.value = await invoke<RecoveredFile[]>("scan_all_deleted_files");
  } catch (e: any) {
    notify.error("Erreur scan global", String(e));
  }
  loadingMft.value = false;
}

// ── Sauvegarde Profil ──────────────────────────────────────
interface UserFolder { name: string; path: string; size_mb: number; shadow_relative: string; }
interface BackupResult { success: boolean; message: string; duration_secs: number; folders_count: number; }

const userFolders = ref<UserFolder[]>([]);
const loadingFolders = ref(false);
const selectedFolders = ref<Set<string>>(new Set());
const backupTarget = ref("");
const backingUp = ref(false);
const backupProgress = ref(0);
const backupProgressMsg = ref("");
const backupResult = ref<BackupResult | null>(null);

async function loadUserFolders(force = false) {
  if (!force && userFolders.value.length > 0) return;
  loadingFolders.value = true;
  try {
    userFolders.value = await invoke<UserFolder[]>("get_user_profile_folders");
    // Sélection par défaut : Documents + Bureau (uniquement au premier chargement)
    if (force || selectedFolders.value.size === 0) {
      selectedFolders.value = new Set(
        userFolders.value.filter(f => ["Documents", "Bureau"].includes(f.name)).map(f => f.path)
      );
    }
  } catch (e: any) {
    notify.error("Erreur profil", String(e));
  }
  loadingFolders.value = false;
}

function toggleFolder(path: string) {
  if (selectedFolders.value.has(path)) selectedFolders.value.delete(path);
  else selectedFolders.value.add(path);
  selectedFolders.value = new Set(selectedFolders.value);
}

async function pickBackupTarget() {
  const { open } = await import("@tauri-apps/plugin-dialog");
  const dir = await open({ directory: true, multiple: false, title: "Choisir le dossier de destination" });
  if (dir) backupTarget.value = dir as string;
}

async function startProfileBackup() {
  if (!backupTarget.value || selectedFolders.value.size === 0) return;
  backingUp.value = true;
  backupResult.value = null;
  backupProgress.value = 5;
  backupProgressMsg.value = "Démarrage de la sauvegarde...";

  const { listen } = await import("@tauri-apps/api/event");
  const unlisten = await listen<{ folder: string; percent: number; message: string }>(
    "backup-profile-progress", ev => {
      backupProgress.value = ev.payload.percent;
      backupProgressMsg.value = ev.payload.message;
    }
  );
  try {
    backupResult.value = await invoke<BackupResult>("backup_user_folders", {
      folders: Array.from(selectedFolders.value),
      target: backupTarget.value,
    });
    if (backupResult.value.success) notify.success("Sauvegarde terminée", backupResult.value.message);
    else notify.error("Échec", backupResult.value.message);
  } catch (e: any) {
    notify.error("Erreur sauvegarde", String(e));
  }
  unlisten();
  backingUp.value = false;
}

function formatSize_mb(mb: number) {
  if (mb < 1024) return `${mb} Mo`;
  return `${(mb / 1024).toFixed(1)} Go`;
}

function formatDate(raw: string) {
  if (!raw) return "—";
  try { return new Date(raw).toLocaleString("fr-FR"); } catch { return raw; }
}
function formatSize(bytes: number) {
  if (!bytes) return "—";
  if (bytes < 1024) return `${bytes} o`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} Ko`;
  return `${(bytes / 1024 / 1024).toFixed(1)} Mo`;
}

onMounted(loadShadows);
</script>

<template>
  <div class="recovery-page">

    <!-- Header Premium -->
    <div class="recovery-hero">
      <div class="hero-icon-wrap">
        <div class="hero-icon"><Database :size="26" /></div>
      </div>
      <div class="hero-text">
        <h1 class="hero-title">Récupération de Données</h1>
        <p class="hero-desc">Restaurez vos fichiers depuis les Shadow Copies, la Corbeille, le journal NTFS ou un disque</p>
      </div>
    </div>

    <!-- Carte "Comment ça marche" -->
    <div class="how-card">
      <div class="how-title"><Lightbulb :size="14" /> Comment ça marche — 5 méthodes de récupération</div>
      <div class="how-grid">
        <div class="how-item"><span class="how-num">1</span><div><strong>Shadow Copy</strong> — Points de restauration VSS de Windows (le plus fiable)</div></div>
        <div class="how-item"><span class="how-num">2</span><div><strong>Corbeille</strong> — Fichiers récemment supprimés mais non vidés</div></div>
        <div class="how-item"><span class="how-num">3</span><div><strong>Journal NTFS</strong> — USN Journal : suppressions récentes sur volumes NTFS</div></div>
        <div class="how-item"><span class="how-num">4</span><div><strong>Sauvegarde Profil</strong> — Copie via Robocopy de vos dossiers utilisateur</div></div>
        <div class="how-item"><span class="how-num">5</span><div><strong>Récupération Disque</strong> — Scan bas niveau via outils tiers (Recuva/TestDisk)</div></div>
      </div>
    </div>

    <!-- Onglets améliorés -->
    <div class="tabs">
      <button class="tab-btn" :class="{ active: activeTab === 'shadow' }" @click="activeTab = 'shadow'; loadShadows()">
        <Clock :size="14" /> Points de Restauration
        <span v-if="shadows.length > 0" class="tab-badge">{{ shadows.length }}</span>
      </button>
      <button class="tab-btn" :class="{ active: activeTab === 'recycle' }" @click="activeTab = 'recycle'; loadRecycleBin()">
        <Trash2 :size="14" /> Corbeille
        <span v-if="recycleFiles.length > 0" class="tab-badge tab-badge-warn">{{ recycleFiles.length }}</span>
      </button>
      <button class="tab-btn" :class="{ active: activeTab === 'mft' }" @click="activeTab = 'mft'">
        <Search :size="14" /> Journal NTFS
        <span v-if="mftFiles.length > 0" class="tab-badge">{{ mftFiles.length }}</span>
      </button>
      <button class="tab-btn" :class="{ active: activeTab === 'backup' }" @click="activeTab = 'backup'; loadUserFolders()">
        <Save :size="14" /> Sauvegarde Profil
      </button>
      <button class="tab-btn" :class="{ active: activeTab === 'disk' }" @click="activeTab = 'disk'">
        <HardDrive :size="14" /> Récupération Disque
      </button>
    </div>

    <!-- ══ SHADOW COPIES ══ -->
    <div v-if="activeTab === 'shadow'" class="tab-content">
      <div class="toolbar">
        <NButton variant="ghost" size="sm" :loading="loadingShadows" @click="loadShadows">
          <RefreshCw :size="13" /> Actualiser
        </NButton>
        <span class="count">{{ shadows.length }} point(s) de restauration trouvé(s)</span>
      </div>

      <div v-if="loadingShadows" class="loading-state"><NSpinner :size="20" /><span>Recherche des Shadow Copies...</span></div>
      <div v-else-if="shadows.length === 0" class="empty">
        <Clock :size="28" />
        <p>Aucun point de restauration VSS trouvé</p>
        <p class="hint">Activez la protection du système dans Propriétés Système → Protection du système</p>
      </div>
      <div v-else>
        <div class="shadows-list">
          <div
            v-for="s in shadows" :key="s.id"
            class="shadow-card"
            :class="{ selected: selectedShadow?.id === s.id }"
          >
            <div class="shadow-header">
              <div class="shadow-icon"><Clock :size="16" /></div>
              <div class="shadow-info">
                <span class="shadow-date">{{ formatDate(s.creation_time) }}</span>
                <div class="shadow-meta-row">
                  <span class="shadow-vol">Volume : {{ s.volume }}</span>
                  <span v-if="s.provider" class="shadow-provider">· {{ s.provider }}</span>
                </div>
              </div>
              <NButton variant="primary" size="sm" @click="browseShadow(s)">
                <Folder :size="12" /> Parcourir
              </NButton>
            </div>
          </div>
        </div>

        <!-- Navigateur de fichiers de la shadow copy -->
        <div v-if="selectedShadow" class="file-browser">
          <!-- En-tête : destination + recherche -->
          <div class="browser-header">
            <div class="restore-target-row">
              <label class="target-label">Restaurer vers :</label>
              <input v-model="restoreTarget" class="target-input" />
            </div>
            <div class="search-row">
              <input
                v-model="searchQuery"
                class="search-input"
                placeholder="Rechercher un fichier..."
                @keydown.enter="searchInShadow"
              />
              <NButton variant="primary" size="sm" :loading="isSearching" @click="searchInShadow">
                <Search :size="12" /> Chercher
              </NButton>
              <NButton v-if="searchMode" variant="ghost" size="sm" @click="browseShadow(selectedShadow!, browsingPath)">
                ✕ Effacer
              </NButton>
              <!-- E — Filtre extension -->
              <div class="ext-filter-wrap">
                <Filter :size="12" class="ext-icon" />
                <input v-model="extFilter" class="ext-input" placeholder=".jpg,.docx…" title="Filtrer par extension" />
              </div>
              <!-- F — Comparaison -->
              <NButton variant="ghost" size="sm" @click="showCompare = !showCompare">
                <GitCompare :size="12" /> {{ showCompare ? 'Fermer' : 'Comparer' }}
              </NButton>
            </div>
          </div>

          <!-- Fil d'Ariane (breadcrumb) -->
          <div class="breadcrumb" v-if="!searchMode">
            <button class="crumb" :class="{ current: pathStack.length === 0 }" @click="pathStack.length > 0 && navigateTo('')">🖥 Racine</button>
            <template v-for="(seg, i) in pathStack" :key="seg">
              <span class="crumb-sep">/</span>
              <button
                class="crumb"
                :class="{ current: i === pathStack.length - 1 }"
                @click="i < pathStack.length - 1 && navigateTo(seg)"
              >{{ seg.split('\\').filter((s: string) => s).pop() }}</button>
            </template>
          </div>
          <div v-else class="search-info">
            🔍 Résultats pour "<strong>{{ searchQuery }}</strong>" dans {{ browsingDisplay }}
            — {{ shadowFiles.length }} fichier(s) trouvé(s)
          </div>

          <!-- F — Comparaison -->
          <RecoveryShadowCompare
            v-if="showCompare && selectedShadow && !searchMode"
            :device-path="selectedShadow.device_path"
            :sub-path="browsingPath"
          />

          <!-- Actions batch -->
          <div class="batch-bar" v-if="filteredShadowFiles.length > 0">
            <label class="check-all" @click="toggleAllFiles">
              <CheckSquare v-if="selectedFiles.size > 0 && selectedFiles.size === filteredShadowFiles.filter(f => !f.is_dir).length" :size="14" />
              <Square v-else :size="14" />
              <span>{{ selectedFiles.size > 0 ? `${selectedFiles.size} sélectionné(s)` : 'Tout sélectionner' }}</span>
            </label>
            <NButton
              v-if="selectedFiles.size > 0"
              variant="primary"
              size="sm"
              :loading="batchRestoring"
              @click="batchRestore"
            >
              <RotateCcw :size="12" /> Restaurer la sélection ({{ selectedFiles.size }})
            </NButton>
            <span v-if="extFilter" class="count">{{ filteredShadowFiles.length }} / {{ shadowFiles.length }} fichier(s)</span>
          </div>

          <div v-if="loadingBrowse || isSearching" class="loading-state"><NSpinner :size="16" /><span>{{ isSearching ? 'Recherche...' : 'Chargement...' }}</span></div>
          <div v-else-if="filteredShadowFiles.length === 0" class="empty-small">
            {{ extFilter ? `Aucun fichier avec ces extensions` : searchMode ? 'Aucun fichier trouvé pour cette recherche' : 'Dossier vide ou inaccessible' }}
          </div>
          <div v-else class="files-table">
            <div class="file-row header-row">
              <span></span><span>Nom</span><span>Taille</span><span>Modifié</span><span></span>
            </div>
            <div
              v-for="f in filteredShadowFiles" :key="f.path"
              class="file-row"
              :class="{ 'is-dir': f.is_dir, 'is-selected': selectedFiles.has(f.path) }"
            >
              <span class="file-check">
                <template v-if="!f.is_dir">
                  <CheckSquare v-if="selectedFiles.has(f.path)" :size="13" class="chk on" @click.stop="toggleFile(f.path)" />
                  <Square v-else :size="13" class="chk" @click.stop="toggleFile(f.path)" />
                </template>
              </span>
              <span
                class="file-name"
                :style="f.is_dir ? 'cursor:pointer;color:var(--accent-primary)' : ''"
                @click="f.is_dir ? navigateInto(f) : undefined"
              >
                <Folder v-if="f.is_dir" :size="12" style="color:var(--accent-primary)" />
                <FileText v-else :size="12" />
                {{ f.name }}
              </span>
              <span class="file-size">{{ f.is_dir ? '—' : formatSize(f.size_bytes) }}</span>
              <span class="file-date">{{ formatDate(f.deleted_date) }}</span>
              <NButton v-if="!f.is_dir" variant="ghost" size="sm" @click="restoreFromShadow(f)">
                <RotateCcw :size="11" /> Restaurer
              </NButton>
              <NButton v-else variant="ghost" size="sm" @click="navigateInto(f)">
                → Ouvrir
              </NButton>
            </div>
          </div>

          <div class="browse-shortcuts">
            <NButton variant="ghost" size="sm" @click="browseShadow(selectedShadow!, '')">📁 Racine</NButton>
            <NButton variant="ghost" size="sm" @click="browseShadow(selectedShadow!, 'Users')">📁 Utilisateurs</NButton>
            <template v-for="f in userFolders.filter(x => ['Documents','Bureau','Images','Telechargements'].includes(x.name))" :key="f.path">
              <NButton variant="ghost" size="sm" @click="browseShadow(selectedShadow!, f.shadow_relative)">
                📂 {{ f.name }}
              </NButton>
            </template>
          </div>
        </div>
      </div>
    </div>

    <!-- ══ CORBEILLE ══ -->
    <div v-if="activeTab === 'recycle'" class="tab-content">
      <div class="toolbar">
        <NButton variant="ghost" size="sm" :loading="loadingRecycle" @click="loadRecycleBin">
          <RefreshCw :size="13" /> Actualiser
        </NButton>
        <span class="count">{{ recycleFiles.length }} fichier(s) dans la corbeille</span>
      </div>

      <div v-if="loadingRecycle" class="loading-state"><NSpinner :size="20" /><span>Lecture de la Corbeille...</span></div>
      <div v-else-if="recycleFiles.length === 0" class="empty">
        <Trash2 :size="28" />
        <p>La Corbeille est vide</p>
        <p class="hint">Aucun fichier supprimé récemment à restaurer</p>
      </div>
      <div v-else>
        <div class="recycle-info-banner">
          <Shield :size="13" />
          <span>{{ recycleFiles.length }} fichier(s) supprimé(s) récupérables. Cliquez "Restaurer" pour les remettre à leur emplacement d'origine.</span>
        </div>
        <div class="files-table files-table-simple">
          <div class="file-row-simple header-row">
            <span>Nom</span><span>Taille</span><span>Supprimé le</span><span></span>
          </div>
          <div v-for="f in recycleFiles" :key="f.path" class="file-row-simple">
            <span class="file-name"><FileText :size="12" /> {{ f.name }}</span>
            <span class="file-size">{{ formatSize(f.size_bytes) }}</span>
            <span class="file-date">{{ formatDate(f.deleted_date) }}</span>
            <NButton variant="ghost" size="sm" @click="restoreRecycle(f)">
              <RotateCcw :size="11" /> Restaurer
            </NButton>
          </div>
        </div>
      </div>
    </div>

    <!-- ══ JOURNAL MFT ══ -->
    <div v-if="activeTab === 'mft'" class="tab-content">
      <div class="info-banner">
        <Search :size="14" />
        <div>
          <strong>Journal NTFS (USN)</strong> — Le système de fichiers NTFS enregistre toutes les opérations récentes dans un journal interne.
          Cet outil scanne ce journal pour retrouver les noms des fichiers récemment supprimés.
          <br><em style="font-size:11px;opacity:.8">⚠ Le journal ne contient que les métadonnées (nom, date). Pour récupérer le contenu, utilisez Recuva ou TestDisk.</em>
        </div>
      </div>

      <div class="toolbar">
        <label class="drive-label">Lecteur :</label>
        <select v-model="mftDrive" class="drive-select-sm">
          <option v-for="d in ['C','D','E','F','G']" :key="d" :value="d">{{ d }}:</option>
        </select>
        <NButton variant="primary" size="sm" :loading="loadingMft" @click="scanMft">
          <Search :size="13" /> Scanner
        </NButton>
        <NButton variant="ghost" size="sm" :loading="loadingMft" @click="scanAllMft">
          <Database :size="13" /> Tous les lecteurs NTFS
        </NButton>
        <span class="count">{{ mftFiles.length }} entrée(s)</span>
      </div>

      <div v-if="loadingMft" class="loading-state"><NSpinner :size="20" /><span>Lecture du journal NTFS...</span></div>
      <div v-else-if="mftFiles.length === 0 && !loadingMft" class="empty">
        <Database :size="28" /><p>Lancez le scan pour voir les suppressions récentes</p>
      </div>
      <div v-else class="files-table files-table-mft">
        <div class="file-row-mft header-row">
          <span>Nom du fichier</span><span>Date suppression</span><span>Source</span>
        </div>
        <div v-for="f in mftFiles" :key="f.path" class="file-row-mft">
          <span class="file-name"><FileText :size="12" /> {{ f.name }}</span>
          <span class="file-date">{{ formatDate(f.deleted_date) }}</span>
          <span class="file-source badge-muted">Journal MFT</span>
        </div>
      </div>
      <p class="hint-note">💡 Pour récupérer ces fichiers, utilisez Recuva ou TestDisk (disponibles dans l'installeur).</p>
    </div>

    <!-- ══ SAUVEGARDE PROFIL ══ -->
    <div v-if="activeTab === 'backup'" class="tab-content">
      <div class="info-banner">
        <Save :size="14" />
        <span>Sauvegardes vos dossiers utilisateur (Documents, Bureau, Images…) vers un disque externe ou un autre emplacement. Utilise Robocopy en mode backup pour contourner les restrictions d'accès.</span>
      </div>

      <!-- Sélection dossiers -->
      <div class="toolbar">
        <NButton variant="ghost" size="sm" :loading="loadingFolders" @click="loadUserFolders(true)">
          <RefreshCw :size="13" /> Actualiser les dossiers
        </NButton>
        <span class="count">{{ userFolders.length }} dossier(s) détecté(s)</span>
      </div>

      <div v-if="loadingFolders" class="loading-state"><NSpinner :size="20" /><span>Lecture du profil...</span></div>
      <div v-else class="folders-section">
        <p class="section-label">Dossiers à sauvegarder</p>
        <div class="folders-grid">
          <div
            v-for="f in userFolders"
            :key="f.path"
            class="folder-card"
            :class="{ selected: selectedFolders.has(f.path) }"
            @click="toggleFolder(f.path)"
          >
            <CheckSquare v-if="selectedFolders.has(f.path)" :size="15" class="ic-check" />
            <Square v-else :size="15" class="ic-uncheck" />
            <Folder :size="20" class="folder-icon" />
            <div class="folder-info">
              <span class="folder-name">{{ f.name }}</span>
              <span class="folder-size">{{ formatSize_mb(f.size_mb) }}</span>
            </div>
          </div>
        </div>

        <!-- Destination -->
        <p class="section-label">Destination</p>
        <div class="target-row">
          <input v-model="backupTarget" class="target-input-lg" placeholder="Chemin de destination (ex: D:\Sauvegardes\Profil)" />
          <NButton variant="ghost" size="sm" @click="pickBackupTarget">
            <FolderOpen :size="13" /> Parcourir
          </NButton>
        </div>

        <!-- Progression -->
        <div v-if="backingUp || backupProgress > 0" class="progress-section">
          <div class="progress-header">
            <NSpinner v-if="backingUp" :size="14" />
            <span>{{ backupProgressMsg }}</span>
          </div>
          <NProgress :value="backupProgress" showLabel size="lg" />
        </div>

        <!-- Résultat -->
        <div v-if="backupResult" class="result-card" :class="backupResult.success ? 'success' : 'error'">
          <CheckCircle v-if="backupResult.success" :size="18" />
          <XCircle v-else :size="18" />
          <div>
            <p class="result-title">{{ backupResult.success ? 'Sauvegarde terminée' : 'Échec de la sauvegarde' }}</p>
            <p class="result-msg">{{ backupResult.message }}</p>
            <p class="result-meta">{{ backupResult.folders_count }} dossier(s) · {{ Math.floor(backupResult.duration_secs / 60) }}min {{ backupResult.duration_secs % 60 }}s</p>
          </div>
        </div>

        <!-- Bouton lancer -->
        <NButton
          variant="primary"
          :disabled="!backupTarget || selectedFolders.size === 0 || backingUp"
          :loading="backingUp"
          @click="startProfileBackup"
        >
          <Save :size="14" /> Lancer la sauvegarde ({{ selectedFolders.size }} dossier(s))
        </NButton>
      </div>
    </div>

    <!-- ══ RÉCUPÉRATION DISQUE ══ -->
    <div v-if="activeTab === 'disk'" class="tab-content">
      <RecoveryTabDisk />
    </div>
  </div>
</template>

<style scoped>
.recovery-page { display: flex; flex-direction: column; gap: 14px; }

/* Hero */
.recovery-hero {
  display: flex; align-items: center; gap: 16px; padding: 18px 22px;
  background: linear-gradient(135deg, var(--bg-secondary), color-mix(in srgb, var(--success) 5%, var(--bg-secondary)));
  border: 1px solid var(--border); border-radius: var(--radius-xl); position: relative; overflow: hidden;
}
.recovery-hero::before {
  content:''; position:absolute; top:-30px; right:-30px; width:120px; height:120px; border-radius:50%;
  background: radial-gradient(circle, color-mix(in srgb, var(--success) 10%, transparent), transparent 70%);
  pointer-events: none;
}
.hero-icon-wrap { flex-shrink: 0; }
.hero-icon {
  width: 48px; height: 48px; border-radius: var(--radius-lg);
  background: linear-gradient(135deg, var(--success), color-mix(in srgb, var(--success) 70%, var(--info)));
  display: flex; align-items: center; justify-content: center; color: white;
  box-shadow: 0 4px 16px color-mix(in srgb, var(--success) 35%, transparent);
  animation: float-rec 3s ease-in-out infinite;
}
@keyframes float-rec { 0%,100%{transform:translateY(0)} 50%{transform:translateY(-3px)} }
.hero-text { flex: 1; }
.hero-title { font-size: 20px; font-weight: 800; color: var(--text-primary); }
.hero-desc { font-size: 12px; color: var(--text-secondary); margin-top: 4px; }

/* How card */
.how-card {
  padding: 14px 16px; background: var(--bg-secondary); border: 1px solid var(--border);
  border-radius: var(--radius-lg); border-left: 3px solid var(--info);
}
.how-title { font-size: 12px; font-weight: 700; color: var(--info); text-transform: uppercase; letter-spacing:.05em; display:flex; align-items:center; gap:6px; margin-bottom:10px; }
.how-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(240px,1fr)); gap: 8px; }
.how-item { display:flex; align-items:flex-start; gap:10px; font-size:12px; color:var(--text-secondary); }
.how-num { min-width:22px; height:22px; border-radius:50%; background:var(--accent-muted); color:var(--accent-primary); font-weight:700; font-size:11px; display:flex; align-items:center; justify-content:center; flex-shrink:0; }

/* Tabs */
.tabs { display: flex; gap: 6px; flex-wrap: wrap; }
.tab-btn {
  display: flex; align-items: center; gap: 7px; padding: 9px 14px;
  border-radius: var(--radius-md); border: 1.5px solid var(--border);
  background: var(--bg-tertiary); cursor: pointer; font-family: inherit;
  font-size: 13px; color: var(--text-secondary); transition: all 0.15s; position: relative;
}
.tab-btn:hover { border-color: var(--text-muted); color: var(--text-primary); }
.tab-btn.active { border-color: var(--accent-primary); color: var(--accent-primary); background: var(--bg-secondary); }
.tab-badge { font-size: 10px; font-weight: 700; padding: 1px 6px; border-radius: 99px; background: var(--accent-muted); color: var(--accent-primary); min-width: 18px; text-align: center; }
.tab-badge-warn { background: var(--warning-muted); color: var(--warning); }

/* Recycle banner */
.recycle-info-banner {
  display:flex; align-items:center; gap:8px; padding:10px 14px;
  background: var(--success-muted); color: var(--success); border: 1px solid color-mix(in srgb,var(--success) 30%,transparent);
  border-radius: var(--radius-md); font-size:12px; margin-bottom:8px;
}

/* Shadow icon */
.shadow-icon { width:32px; height:32px; border-radius:var(--radius-md); background:var(--accent-muted); display:flex; align-items:center; justify-content:center; color:var(--accent-primary); flex-shrink:0; }
.shadow-meta-row { display:flex; align-items:center; gap:6px; }
.shadow-provider { font-size:10px; color:var(--text-muted); font-family:monospace; }

.tab-content { display: flex; flex-direction: column; gap: 12px; }
.toolbar { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.count { font-size: 11px; color: var(--text-muted); font-family: monospace; }

.info-banner {
  display: flex; gap: 8px; align-items: flex-start; padding: 10px 14px;
  border-radius: var(--radius-md); font-size: 12px; line-height: 1.5;
  background: var(--info-muted); color: var(--info); border: 1px solid var(--info);
}

.loading-state { display: flex; align-items: center; gap: 10px; padding: 20px; font-size: 13px; color: var(--text-muted); }
.empty { display: flex; flex-direction: column; align-items: center; gap: 10px; padding: 40px; color: var(--text-muted); font-size: 13px; }
.empty-small { padding: 16px; text-align: center; font-size: 12px; color: var(--text-muted); }
.hint { font-size: 11px; text-align: center; }

/* Shadows */
.shadows-list { display: flex; flex-direction: column; gap: 6px; }
.shadow-card {
  padding: 10px 14px; border: 1px solid var(--border); border-radius: var(--radius-md);
  background: var(--bg-secondary); cursor: pointer; transition: border-color 0.15s;
}
.shadow-card.selected { border-color: var(--accent-primary); }
.shadow-header { display: flex; align-items: center; gap: 10px; }
.shadow-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.shadow-date { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.shadow-vol { font-size: 11px; color: var(--text-muted); font-family: monospace; }

/* File browser */
.file-browser { display: flex; flex-direction: column; gap: 8px; background: var(--bg-secondary); border: 1px solid var(--accent-primary); border-radius: var(--radius-lg); padding: 12px; margin-top: 4px; }
.browser-header { display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px; }
.browser-title { font-weight: 700; font-size: 13px; color: var(--accent-primary); }
.restore-target-row { display: flex; align-items: center; gap: 8px; }
.target-label { font-size: 11px; color: var(--text-muted); }
.target-input { padding: 4px 8px; font-size: 11px; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); font-family: monospace; width: 280px; }
.browse-shortcuts { display: flex; gap: 6px; flex-wrap: wrap; padding-top: 4px; border-top: 1px solid var(--border); }

/* Extension filter */
.ext-filter-wrap { display: flex; align-items: center; gap: 4px; }
.ext-icon { color: var(--text-muted); flex-shrink: 0; }
.ext-input { width: 90px; padding: 5px 7px; font-size: 11px; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); font-family: monospace; outline: none; transition: border-color 0.15s; }
.ext-input:focus { border-color: var(--accent-primary); }
/* Search bar */
.search-row { display: flex; align-items: center; gap: 6px; flex: 1; }
.search-input { flex: 1; padding: 6px 10px; font-size: 12px; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); font-family: inherit; outline: none; transition: border-color 0.15s; }
.search-input:focus { border-color: var(--accent-primary); }

/* Breadcrumb */
.breadcrumb { display: flex; align-items: center; flex-wrap: wrap; gap: 2px; font-size: 12px; }
.crumb { background: none; border: none; padding: 2px 6px; cursor: pointer; color: var(--accent-primary); font-family: inherit; font-size: 12px; border-radius: var(--radius-sm); transition: background 0.12s; }
.crumb:hover { background: var(--bg-tertiary); }
.crumb.current { color: var(--text-primary); font-weight: 600; cursor: default; }
.crumb-sep { color: var(--text-muted); }
.search-info { font-size: 12px; color: var(--text-secondary); padding: 4px 0; }
.batch-bar { display: flex; align-items: center; gap: 12px; padding: 6px 4px; border-bottom: 1px solid var(--border); }
.check-all { display: flex; align-items: center; gap: 6px; font-size: 12px; cursor: pointer; color: var(--text-secondary); user-select: none; }

/* Files table */
.files-table { display: flex; flex-direction: column; border: 1px solid var(--border); border-radius: var(--radius-md); overflow: hidden; }
.file-row { display: grid; grid-template-columns: 24px 1fr 70px 130px 90px; align-items: center; gap: 8px; padding: 5px 10px; border-bottom: 1px solid var(--border); font-size: 12px; }
.file-row:last-child { border-bottom: none; }
.header-row { background: var(--bg-tertiary); font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: .05em; }
.file-row.is-dir { background: color-mix(in srgb, var(--accent-primary) 4%, transparent); }
.file-row.is-selected { background: color-mix(in srgb, var(--accent-primary) 8%, transparent); }
.file-check { display: flex; align-items: center; justify-content: center; }
.chk { cursor: pointer; color: var(--text-muted); transition: color 0.1s; }
.chk:hover, .chk.on { color: var(--accent-primary); }
.file-name { display: flex; align-items: center; gap: 6px; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.file-size, .file-date { color: var(--text-muted); font-family: monospace; }
.file-source { font-size: 10px; }
.badge-muted { background: var(--bg-elevated); color: var(--text-muted); padding: 2px 6px; border-radius: 4px; }
/* Corbeille / MFT simples (pas de checkbox) */
.file-row-simple { display: grid; grid-template-columns: 1fr 80px 150px 90px; align-items: center; gap: 8px; padding: 5px 10px; border-bottom: 1px solid var(--border); font-size: 12px; }
.file-row-simple:last-child { border-bottom: none; }
.file-row-mft { display: grid; grid-template-columns: 1fr 160px 100px; align-items: center; gap: 8px; padding: 5px 10px; border-bottom: 1px solid var(--border); font-size: 12px; }
.file-row-mft:last-child { border-bottom: none; }

/* MFT */
.drive-label { font-size: 12px; color: var(--text-secondary); }
.drive-select-sm { padding: 6px 8px; border: 1px solid var(--border); border-radius: var(--radius-sm); background: var(--bg-tertiary); color: var(--text-primary); font-family: inherit; font-size: 12px; }
.hint-note { font-size: 12px; color: var(--text-muted); padding: 8px 0; }

/* Sauvegarde Profil */
.folders-section { display: flex; flex-direction: column; gap: 14px; }
.section-label { font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: .06em; color: var(--text-muted); }
.folders-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(170px, 1fr)); gap: 10px; }
.folder-card { display: flex; align-items: center; gap: 10px; padding: 12px 14px; border: 1.5px solid var(--border); border-radius: var(--radius-lg); background: var(--bg-secondary); cursor: pointer; transition: all 0.15s; user-select: none; }
.folder-card:hover { border-color: var(--text-muted); background: var(--bg-tertiary); }
.folder-card.selected { border-color: var(--accent-primary); background: var(--accent-muted); }
.ic-check { color: var(--accent-primary); flex-shrink: 0; }
.ic-uncheck { color: var(--text-muted); flex-shrink: 0; }
.folder-icon { color: var(--accent-primary); flex-shrink: 0; }
.folder-card:not(.selected) .folder-icon { color: var(--text-muted); }
.folder-info { display: flex; flex-direction: column; gap: 2px; overflow: hidden; }
.folder-name { font-size: 13px; font-weight: 600; color: var(--text-primary); white-space: nowrap; }
.folder-size { font-size: 11px; color: var(--text-muted); font-family: monospace; }
.target-row { display: flex; gap: 8px; align-items: center; }
.target-input-lg { flex: 1; padding: 8px 12px; border: 1px solid var(--border); border-radius: var(--radius-md); background: var(--bg-tertiary); color: var(--text-primary); font-family: monospace; font-size: 12px; outline: none; transition: border-color 0.15s; }
.target-input-lg:focus { border-color: var(--accent-primary); }
.progress-section { display: flex; flex-direction: column; gap: 8px; padding: 12px 14px; background: var(--bg-secondary); border-radius: var(--radius-md); border: 1px solid var(--border); }
.progress-header { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--text-secondary); }
.result-card { display: flex; align-items: flex-start; gap: 12px; padding: 12px 14px; border-radius: var(--radius-lg); border: 1px solid; }
.result-card.success { background: var(--success-muted); border-color: var(--success); color: var(--success); }
.result-card.error { background: var(--danger-muted); border-color: var(--danger); color: var(--danger); }
.result-title { font-weight: 700; font-size: 13px; }
.result-msg { font-size: 12px; color: var(--text-secondary); margin-top: 3px; }
.result-meta { font-size: 11px; color: var(--text-muted); margin-top: 2px; font-family: monospace; }
</style>
