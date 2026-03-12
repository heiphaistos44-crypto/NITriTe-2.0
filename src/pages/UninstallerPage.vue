<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Trash2, Search, RefreshCw, CheckSquare, Square,
  CheckCircle, XCircle, AlertTriangle, Package,
  ChevronUp, ChevronDown, Eye, X, FolderOpen,
  RotateCcw, Archive,
} from "lucide-vue-next";

const notify = useNotificationStore();

interface InstalledApp {
  name: string; version: string; publisher: string;
  uninstall_string: string; source: string;
  install_size_kb: number; install_date: string;
}

interface UninstallJob {
  app: InstalledApp;
  status: "pending" | "running" | "done" | "error";
  message: string;
  residuals: string[];      // résidus trouvés (pas encore supprimés)
  residualsHandled: boolean; // true quand l'utilisateur a agi sur les résidus
}

interface ResidualCleanResult {
  success: boolean; deleted_count: number; failed_count: number; message: string;
}

const apps = ref<InstalledApp[]>([]);
const loading = ref(false);
const search = ref("");
const selected = ref<Set<string>>(new Set());
const filterPublisher = ref("");

// B — Tri
type SortKey = "name" | "publisher" | "size" | "date" | "version";
const sortKey = ref<SortKey>("name");
const sortDir = ref<"asc" | "desc">("asc");
function setSort(key: SortKey) {
  if (sortKey.value === key) sortDir.value = sortDir.value === "asc" ? "desc" : "asc";
  else { sortKey.value = key; sortDir.value = "asc"; }
}

// C — Preview résidus (avant uninstall)
const previewApp = ref<InstalledApp | null>(null);
const previewItems = ref<string[]>([]);
const loadingPreview = ref(false);
async function showPreview(app: InstalledApp) {
  previewApp.value = app; previewItems.value = []; loadingPreview.value = true;
  try {
    previewItems.value = await invoke<string[]>("preview_residuals", { appName: app.name, publisher: app.publisher });
  } catch (e: any) { notify.error("Erreur preview", String(e)); }
  loadingPreview.value = false;
}

const jobs = ref<UninstallJob[]>([]);
const uninstalling = ref(false);
const progress = ref(0);
const currentStep = ref("");
const showResults = ref(false);

// Résidus en cours de traitement
const handlingResiduals = ref<Record<string, boolean>>({});
const extractTarget = ref("C:\\NiTriTe\\Résidus");

const filtered = computed(() => {
  const q = search.value.toLowerCase();
  const pub = filterPublisher.value.toLowerCase();
  return apps.value.filter(a =>
    a.name.toLowerCase().includes(q) &&
    (pub === "" || a.publisher.toLowerCase().includes(pub))
  );
});

const sorted = computed(() => {
  const list = [...filtered.value];
  const dir = sortDir.value === "asc" ? 1 : -1;
  list.sort((a, b) => {
    switch (sortKey.value) {
      case "name":      return dir * a.name.localeCompare(b.name);
      case "publisher": return dir * a.publisher.localeCompare(b.publisher);
      case "version":   return dir * a.version.localeCompare(b.version);
      case "size":      return dir * (a.install_size_kb - b.install_size_kb);
      case "date":      return dir * a.install_date.localeCompare(b.install_date);
    }
  });
  return list;
});

const selectedCount = computed(() => selected.value.size);
const allChecked = computed(() =>
  filtered.value.length > 0 && filtered.value.every(a => selected.value.has(a.name))
);
const pendingResiduals = computed(() => jobs.value.filter(j => j.status === "done" && !j.residualsHandled && j.residuals.length > 0));

function toggleSelect(name: string) {
  if (selected.value.has(name)) selected.value.delete(name);
  else selected.value.add(name);
  selected.value = new Set(selected.value);
}
function toggleAll() {
  if (allChecked.value) filtered.value.forEach(a => selected.value.delete(a.name));
  else filtered.value.forEach(a => selected.value.add(a.name));
  selected.value = new Set(selected.value);
}

async function loadApps() {
  loading.value = true; apps.value = []; selected.value = new Set();
  try {
    apps.value = await invoke<InstalledApp[]>("list_installed_apps_for_uninstall");
  } catch (e: any) { notify.error("Chargement échoué", String(e)); }
  loading.value = false;
}

async function startUninstall() {
  const toUninstall = apps.value.filter(a => selected.value.has(a.name));
  if (!toUninstall.length) return;

  jobs.value = toUninstall.map(a => ({ app: a, status: "pending", message: "", residuals: [], residualsHandled: false }));
  uninstalling.value = true;
  progress.value = 0;
  showResults.value = false;
  previewApp.value = null;


  for (let i = 0; i < jobs.value.length; i++) {
    const job = jobs.value[i];
    job.status = "running";
    currentStep.value = `(${i + 1}/${jobs.value.length}) Désinstallation de ${job.app.name}...`;
    try {
      const result = await invoke<{ success: boolean; message: string; residuals_found: string[] }>("uninstall_app_clean", {
        appName: job.app.name,
        uninstallString: job.app.uninstall_string,
        publisher: job.app.publisher,
      });
      job.status = result.success ? "done" : "error";
      job.message = result.message;
      job.residuals = result.residuals_found ?? [];
      // On retire de la liste seulement si désinstallation réussie
      if (result.success) {
        apps.value = apps.value.filter(a => a.name !== job.app.name);
        selected.value.delete(job.app.name);
      }
    } catch (e: any) {
      job.status = "error";
      job.message = String(e);
    }
    progress.value = Math.round(((i + 1) / jobs.value.length) * 100);
  }

  const doneCount  = jobs.value.filter(j => j.status === "done").length;
  const errCount   = jobs.value.filter(j => j.status === "error").length;
  const residCount = jobs.value.filter(j => j.residuals.length > 0).length;

  if (errCount === 0) notify.success("Désinstallation terminée", `${doneCount} app(s) supprimée(s)`);
  else notify.warning("Partiellement terminé", `${doneCount} succès, ${errCount} échec(s)`);

  uninstalling.value = false;
  currentStep.value = "";
  showResults.value = doneCount > 0 || errCount > 0;

  if (residCount > 0) notify.warning(`${residCount} app(s) avec résidus`, "Consultez le panneau Résultats pour nettoyer.");
}

async function deleteJobResiduals(job: UninstallJob) {
  handlingResiduals.value[job.app.name] = true;
  try {
    const r = await invoke<ResidualCleanResult>("delete_residuals", { paths: job.residuals });
    if (r.success) notify.success("Résidus supprimés", r.message);
    else notify.error("Suppression partielle", r.message);
    job.residualsHandled = true;
  } catch (e: any) { notify.error("Erreur", String(e)); }
  handlingResiduals.value[job.app.name] = false;
}

async function extractJobResiduals(job: UninstallJob) {
  handlingResiduals.value[job.app.name] = true;
  try {
    const r = await invoke<ResidualCleanResult>("extract_residuals", {
      paths: job.residuals,
      target: extractTarget.value,
    });
    if (r.success) notify.success("Résidus extraits + supprimés", r.message);
    else notify.error("Extraction partielle", r.message);
    job.residualsHandled = true;
  } catch (e: any) { notify.error("Erreur", String(e)); }
  handlingResiduals.value[job.app.name] = false;
}

async function pickExtractTarget() {
  const { open } = await import("@tauri-apps/plugin-dialog");
  const dir = await open({ directory: true, multiple: false, title: "Dossier d'extraction" });
  if (dir) extractTarget.value = dir as string;
}

function formatSize(kb: number) {
  if (!kb) return "—";
  if (kb < 1024) return `${kb} Ko`;
  return `${(kb / 1024).toFixed(1)} Mo`;
}

onMounted(loadApps);
</script>

<template>
  <div class="uninstaller-page">
    <DiagBanner :icon="Trash2" title="Désinstallateur Propre"
      desc="Désinstallation silencieuse automatique — registre nettoyé, fichiers purgés" color="red" />

    <!-- Toolbar -->
    <div class="toolbar">
      <div class="toolbar-left">
        <div class="search-wrap">
          <Search :size="14" class="search-icon" />
          <input v-model="search" placeholder="Rechercher une application..." class="search-input" />
        </div>
        <input v-model="filterPublisher" placeholder="Filtrer par éditeur..." class="filter-input" />
      </div>
      <div class="toolbar-right">
        <span class="apps-count">{{ apps.length }} apps</span>
        <NButton variant="ghost" size="sm" :loading="loading" @click="loadApps">
          <RefreshCw :size="13" /> Actualiser
        </NButton>
        <NButton variant="danger" size="sm" :disabled="selectedCount === 0 || uninstalling" @click="startUninstall">
          <Trash2 :size="13" /> Désinstaller ({{ selectedCount }})
        </NButton>
      </div>
    </div>

    <!-- Progress -->
    <div v-if="uninstalling" class="progress-bar">
      <div class="progress-header"><NSpinner :size="14" /><span>{{ currentStep }}</span></div>
      <NProgress :value="progress" showLabel size="sm" />
    </div>

    <!-- ══ RÉSULTATS + RÉSIDUS ══ -->
    <div v-if="showResults && !uninstalling" class="results-panel">
      <div class="results-header">
        <span class="results-title">Résultats de désinstallation</span>
        <button class="close-btn" @click="showResults = false; jobs = []"><X :size="14" /></button>
      </div>

      <!-- Extraction target global -->
      <div v-if="pendingResiduals.length > 0" class="extract-global-row">
        <label class="small-label">Dossier d'extraction :</label>
        <input v-model="extractTarget" class="extract-input" />
        <NButton variant="ghost" size="sm" @click="pickExtractTarget"><FolderOpen :size="12" /></NButton>
      </div>

      <div v-for="job in jobs" :key="job.app.name" class="job-block" :class="job.status">
        <!-- En-tête du job -->
        <div class="job-row">
          <CheckCircle v-if="job.status === 'done'"  :size="14" class="ic-success" />
          <XCircle     v-else-if="job.status === 'error'" :size="14" class="ic-error" />
          <NSpinner    v-else :size="14" />
          <span class="job-name">{{ job.app.name }}</span>
          <span class="job-msg">{{ job.message }}</span>
        </div>

        <!-- Résidus trouvés -->
        <div v-if="job.residuals.length > 0 && !job.residualsHandled" class="residuals-block">
          <div class="residuals-header">
            <AlertTriangle :size="12" style="color:#d97706;flex-shrink:0" />
            <span class="res-label">{{ job.residuals.length }} résidu(s) détecté(s) :</span>
            <div class="res-actions">
              <NButton
                variant="danger" size="sm"
                :loading="!!handlingResiduals[job.app.name]"
                @click="deleteJobResiduals(job)"
              >
                <Trash2 :size="11" /> Supprimer définitivement
              </NButton>
              <NButton
                variant="ghost" size="sm"
                :loading="!!handlingResiduals[job.app.name]"
                @click="extractJobResiduals(job)"
              >
                <Archive :size="11" /> Extraire puis supprimer
              </NButton>
              <NButton variant="ghost" size="sm" @click="job.residualsHandled = true">
                Ignorer
              </NButton>
            </div>
          </div>
          <div class="residuals-list">
            <div v-for="r in job.residuals" :key="r" class="residual-item">{{ r }}</div>
          </div>
        </div>
        <div v-else-if="job.residuals.length > 0 && job.residualsHandled" class="res-done">
          <CheckCircle :size="12" class="ic-success" /> Résidus traités
        </div>
        <div v-else-if="job.status === 'done'" class="res-done">
          <CheckCircle :size="12" class="ic-success" /> Aucun résidu détecté
        </div>
      </div>
    </div>

    <!-- C — Preview résidus (avant uninstall) -->
    <div v-if="previewApp" class="preview-panel">
      <div class="preview-header">
        <Eye :size="14" style="color:#d97706" />
        <span class="preview-title">Résidus potentiels — <strong>{{ previewApp.name }}</strong></span>
        <button class="close-btn" @click="previewApp = null"><X :size="14" /></button>
      </div>
      <div v-if="loadingPreview" class="preview-loading"><NSpinner :size="14" /><span>Analyse...</span></div>
      <div v-else-if="previewItems.length === 0" class="preview-empty">Aucun résidu détecté.</div>
      <div v-else class="preview-list">
        <div v-for="item in previewItems" :key="item" class="residual-item">{{ item }}</div>
      </div>
      <p class="preview-hint">Ces éléments seront proposés à la suppression après désinstallation.</p>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="loading-state"><NSpinner :size="24" /><p>Lecture du registre Windows...</p></div>

    <!-- Table -->
    <div v-else-if="apps.length > 0" class="apps-table-wrap">
      <table class="apps-table">
        <thead>
          <tr>
            <th class="col-check">
              <button class="check-all" @click="toggleAll">
                <CheckSquare v-if="allChecked" :size="15" style="color:var(--accent-primary)" />
                <Square v-else :size="15" />
              </button>
            </th>
            <th class="col-name sortable"      @click="setSort('name')">
              Application
              <ChevronUp v-if="sortKey==='name'&&sortDir==='asc'" :size="11" class="sort-ic"/>
              <ChevronDown v-if="sortKey==='name'&&sortDir==='desc'" :size="11" class="sort-ic"/>
            </th>
            <th class="col-version sortable"   @click="setSort('version')">
              Version
              <ChevronUp v-if="sortKey==='version'&&sortDir==='asc'" :size="11" class="sort-ic"/>
              <ChevronDown v-if="sortKey==='version'&&sortDir==='desc'" :size="11" class="sort-ic"/>
            </th>
            <th class="col-publisher sortable" @click="setSort('publisher')">
              Éditeur
              <ChevronUp v-if="sortKey==='publisher'&&sortDir==='asc'" :size="11" class="sort-ic"/>
              <ChevronDown v-if="sortKey==='publisher'&&sortDir==='desc'" :size="11" class="sort-ic"/>
            </th>
            <th class="col-size sortable"      @click="setSort('size')">
              Taille
              <ChevronUp v-if="sortKey==='size'&&sortDir==='asc'" :size="11" class="sort-ic"/>
              <ChevronDown v-if="sortKey==='size'&&sortDir==='desc'" :size="11" class="sort-ic"/>
            </th>
            <th class="col-date sortable"      @click="setSort('date')">
              Installé le
              <ChevronUp v-if="sortKey==='date'&&sortDir==='asc'" :size="11" class="sort-ic"/>
              <ChevronDown v-if="sortKey==='date'&&sortDir==='desc'" :size="11" class="sort-ic"/>
            </th>
            <th class="col-preview"></th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="app in sorted" :key="app.name"
            class="app-row" :class="{ selected: selected.has(app.name) }"
            @click="toggleSelect(app.name)"
          >
            <td class="col-check">
              <CheckSquare v-if="selected.has(app.name)" :size="14" style="color:var(--accent-primary)" />
              <Square v-else :size="14" style="color:var(--text-muted)" />
            </td>
            <td class="col-name"><span class="app-icon"><Package :size="13" /></span>{{ app.name }}</td>
            <td class="col-version">{{ app.version || '—' }}</td>
            <td class="col-publisher">{{ app.publisher || '—' }}</td>
            <td class="col-size">{{ formatSize(app.install_size_kb) }}</td>
            <td class="col-date">{{ app.install_date || '—' }}</td>
            <td class="col-preview" @click.stop>
              <button class="preview-btn" @click="showPreview(app)" title="Voir les résidus potentiels">
                <Eye :size="13" />
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-if="sorted.length === 0" class="no-results">Aucun résultat pour « {{ search }} »</p>
    </div>

    <!-- Empty -->
    <div v-else class="empty-state">
      <AlertTriangle :size="32" style="color:var(--text-muted)" />
      <p>Aucune application détectée</p>
      <NButton variant="ghost" @click="loadApps"><RefreshCw :size="13" /> Réessayer</NButton>
    </div>
  </div>
</template>

<style scoped>
.uninstaller-page { display: flex; flex-direction: column; gap: 12px; height: 100%; }

.toolbar { display: flex; align-items: center; justify-content: space-between; gap: 10px; flex-wrap: wrap; }
.toolbar-left { display: flex; align-items: center; gap: 8px; flex: 1; }
.toolbar-right { display: flex; align-items: center; gap: 8px; }
.search-wrap { position: relative; flex: 1; max-width: 320px; }
.search-icon { position: absolute; left: 10px; top: 50%; transform: translateY(-50%); color: var(--text-muted); pointer-events: none; }
.search-input, .filter-input { padding: 7px 12px 7px 32px; border: 1px solid var(--border); border-radius: var(--radius-md); background: var(--bg-tertiary); color: var(--text-primary); font-family: inherit; font-size: 12px; width: 100%; outline: none; transition: border-color 0.15s; }
.filter-input { padding-left: 12px; max-width: 200px; }
.search-input:focus, .filter-input:focus { border-color: var(--accent-primary); }
.apps-count { font-size: 11px; color: var(--text-muted); font-family: "JetBrains Mono", monospace; }

.progress-bar { display: flex; flex-direction: column; gap: 6px; padding: 10px 14px; background: var(--bg-secondary); border-radius: var(--radius-md); border: 1px solid var(--border); }
.progress-header { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--text-secondary); }

/* ══ Résultats ══ */
.results-panel { display: flex; flex-direction: column; gap: 8px; padding: 12px 14px; background: var(--bg-secondary); border-radius: var(--radius-lg); border: 1px solid var(--border); }
.results-header { display: flex; align-items: center; gap: 8px; }
.results-title { flex: 1; font-size: 13px; font-weight: 700; color: var(--text-primary); }
.close-btn { background: none; border: none; cursor: pointer; padding: 2px; color: var(--text-muted); border-radius: var(--radius-sm); }
.close-btn:hover { color: var(--text-primary); }

.extract-global-row { display: flex; align-items: center; gap: 8px; padding: 6px 0; border-top: 1px solid var(--border); }
.small-label { font-size: 11px; color: var(--text-muted); flex-shrink: 0; }
.extract-input { flex: 1; padding: 5px 9px; font-size: 11px; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); font-family: monospace; outline: none; }

.job-block { display: flex; flex-direction: column; gap: 6px; padding: 8px 10px; border-radius: var(--radius-md); background: var(--bg-tertiary); border-left: 3px solid var(--border); }
.job-block.done  { border-left-color: var(--success); }
.job-block.error { border-left-color: var(--danger); }
.job-row { display: flex; align-items: center; gap: 8px; font-size: 12px; }
.job-name { font-weight: 600; flex-shrink: 0; color: var(--text-primary); }
.job-msg { color: var(--text-muted); font-size: 11px; }
.ic-success { color: var(--success); flex-shrink: 0; }
.ic-error   { color: var(--danger);  flex-shrink: 0; }

.residuals-block { display: flex; flex-direction: column; gap: 6px; }
.residuals-header { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.res-label { font-size: 11px; color: #d97706; font-weight: 600; flex-shrink: 0; }
.res-actions { display: flex; gap: 6px; flex-wrap: wrap; }
.residuals-list { max-height: 100px; overflow-y: auto; background: var(--bg-secondary); border-radius: var(--radius-sm); padding: 6px 10px; display: flex; flex-direction: column; gap: 1px; }
.residual-item { font-size: 10px; font-family: monospace; color: var(--text-secondary); }
.res-done { display: flex; align-items: center; gap: 6px; font-size: 11px; color: var(--success); }

/* C — Preview avant uninstall */
.preview-panel { display: flex; flex-direction: column; gap: 8px; padding: 12px 14px; background: var(--bg-secondary); border: 1px solid #d97706; border-radius: var(--radius-lg); }
.preview-header { display: flex; align-items: center; gap: 8px; }
.preview-title { flex: 1; font-size: 13px; color: var(--text-primary); }
.preview-loading { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--text-muted); }
.preview-empty { font-size: 12px; color: var(--text-muted); }
.preview-list { max-height: 120px; overflow-y: auto; background: var(--bg-tertiary); border-radius: var(--radius-sm); padding: 8px 10px; display: flex; flex-direction: column; gap: 1px; }
.preview-hint { font-size: 11px; color: var(--text-muted); font-style: italic; }

.loading-state { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 40px; color: var(--text-muted); font-size: 13px; }

.apps-table-wrap { flex: 1; overflow-y: auto; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--bg-secondary); }
.apps-table { width: 100%; border-collapse: collapse; font-size: 12px; }
.apps-table th { position: sticky; top: 0; background: var(--bg-tertiary); padding: 8px 12px; text-align: left; font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; border-bottom: 1px solid var(--border); z-index: 1; }
.sortable { cursor: pointer; user-select: none; white-space: nowrap; }
.sortable:hover { color: var(--text-primary); }
.sort-ic { vertical-align: middle; margin-left: 3px; }

.app-row { border-bottom: 1px solid var(--border); cursor: pointer; transition: background 0.1s; }
.app-row:hover { background: var(--bg-tertiary); }
.app-row.selected { background: var(--accent-muted); }
.app-row td { padding: 7px 12px; color: var(--text-primary); vertical-align: middle; }
.col-check { width: 36px; text-align: center; }
.col-name { min-width: 200px; }
.col-version { width: 100px; color: var(--text-muted) !important; font-family: "JetBrains Mono", monospace; }
.col-publisher { color: var(--text-secondary) !important; }
.col-size { width: 80px; color: var(--text-muted) !important; font-family: "JetBrains Mono", monospace; text-align: right; }
.col-date { width: 100px; color: var(--text-muted) !important; font-family: "JetBrains Mono", monospace; }
.col-preview { width: 36px; text-align: center; }
.app-icon { display: inline-flex; color: var(--text-muted); margin-right: 6px; vertical-align: middle; }

.preview-btn { background: none; border: 1px solid var(--border); cursor: pointer; padding: 3px 6px; border-radius: var(--radius-sm); color: var(--text-muted); transition: all 0.1s; display: flex; align-items: center; }
.preview-btn:hover { border-color: #d97706; color: #d97706; }

.check-all { background: none; border: none; cursor: pointer; display: flex; align-items: center; justify-content: center; padding: 2px; }
.no-results { text-align: center; padding: 20px; color: var(--text-muted); font-size: 13px; }
.empty-state { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 40px; color: var(--text-muted); font-size: 13px; }
</style>
