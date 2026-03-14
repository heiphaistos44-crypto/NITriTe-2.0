<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NTabs from "@/components/ui/NTabs.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NDropdown from "@/components/ui/NDropdown.vue";
import NInput from "@/components/ui/NInput.vue";
import NModal from "@/components/ui/NModal.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Terminal, Play, Shield, FileCode,
  Code, Trash2, ScrollText, FolderOpen, Save, RefreshCw, BookOpen, Clock,
} from "lucide-vue-next";

const notify = useNotificationStore();

// --- Builtin scripts ---
interface BuiltinScript {
  name: string;
  description: string;
  category: string;
  script_type: "cmd" | "powershell";
  content: string;
  requires_admin: boolean;
}

const scripts = ref<BuiltinScript[]>([]);
const scriptsLoading = ref(true);

const categories = computed(() => {
  const cats = [...new Set(scripts.value.map((s) => s.category))];
  return cats.map((c) => ({ id: c, label: c }));
});

const activeCategory = ref("");

const filteredScripts = computed(() => {
  if (!activeCategory.value) return scripts.value;
  return scripts.value.filter((s) => s.category === activeCategory.value);
});

async function loadScripts() {
  scriptsLoading.value = true;
  try {
    scripts.value = await invoke<BuiltinScript[]>("get_builtin_scripts");
  } catch {
    scripts.value = [
      { name: "Flush DNS", description: "Vide le cache DNS pour resoudre les problemes de connexion", category: "Reseau", script_type: "cmd", content: "ipconfig /flushdns", requires_admin: true },
      { name: "Reset Winsock", description: "Reinitialise le catalogue Winsock", category: "Reseau", script_type: "cmd", content: "netsh winsock reset", requires_admin: true },
      { name: "Renouveler IP", description: "Release et renouvellement de l'adresse IP", category: "Reseau", script_type: "cmd", content: "ipconfig /release && ipconfig /renew", requires_admin: true },
      { name: "SFC /scannow", description: "Verifie et repare les fichiers systeme corrompus", category: "Reparation", script_type: "cmd", content: "sfc /scannow", requires_admin: true },
      { name: "DISM Health", description: "Repare l'image Windows avec DISM", category: "Reparation", script_type: "cmd", content: "DISM /Online /Cleanup-Image /RestoreHealth", requires_admin: true },
      { name: "Check Disk", description: "Verifie l'integrite du systeme de fichiers", category: "Reparation", script_type: "cmd", content: "chkdsk C: /f", requires_admin: true },
      { name: "Vider les logs", description: "Supprime tous les journaux d'evenements Windows", category: "Nettoyage", script_type: "powershell", content: "Get-EventLog -LogName * | ForEach { Clear-EventLog $_.Log }", requires_admin: true },
      { name: "Vider cache icones", description: "Reinitialise le cache des icones Windows", category: "Nettoyage", script_type: "cmd", content: "ie4uinit.exe -show", requires_admin: false },
      { name: "Gros fichiers", description: "Liste les fichiers de plus de 500 MB sur le disque C:", category: "Analyse", script_type: "powershell", content: "Get-ChildItem C:\\ -Recurse -ErrorAction SilentlyContinue | Where-Object { $_.Length -gt 500MB } | Sort-Object Length -Descending | Select-Object FullName, @{N='SizeMB';E={[math]::Round($_.Length/1MB,1)}} | Format-Table -AutoSize", requires_admin: false },
      { name: "Rapport batterie", description: "Genere un rapport detaille de la batterie", category: "Analyse", script_type: "cmd", content: "powercfg /batteryreport /output %USERPROFILE%\\battery-report.html && start %USERPROFILE%\\battery-report.html", requires_admin: true },
    ];
  } finally {
    scriptsLoading.value = false;
    if (categories.value.length && !activeCategory.value) {
      activeCategory.value = categories.value[0].id;
    }
  }
}

// --- Script execution ---
const runningScriptId = ref<string | null>(null);
const outputLines = ref<string[]>([]);
const outputRef = ref<HTMLElement | null>(null);
let unlisten: (() => void) | null = null;
const elapsed = ref(0);
let elapsedTimer: ReturnType<typeof setInterval> | null = null;

function startElapsed() {
  elapsed.value = 0;
  elapsedTimer = setInterval(() => { elapsed.value++; }, 1000);
}
function stopElapsed() {
  if (elapsedTimer) { clearInterval(elapsedTimer); elapsedTimer = null; }
}

async function setupListener() {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    const u = await listen<string>("script-output", (event) => {
      outputLines.value.push(event.payload);
      scrollOutput();
    });
    unlisten = u;
  } catch {
    // Mode dev - pas d'events
  }
}

function scrollOutput() {
  setTimeout(() => {
    if (outputRef.value) {
      outputRef.value.scrollTop = outputRef.value.scrollHeight;
    }
  }, 50);
}

// Extrait les chemins Windows depuis une ligne de sortie
function extractPaths(line: string): string[] {
  const pattern = /([A-Z]:\\[^\s"<>|*?]+)/gi;
  const matches = line.match(pattern) ?? [];
  return matches.filter(p => p.length > 3);
}

const detectedPaths = ref<string[]>([]);

async function openPath(p: string) {
  try {
    await invoke("open_path", { path: p });
  } catch { window.open(`file:///${p}`, "_blank"); }
}

async function executeBuiltinScript(script: BuiltinScript) {
  runningScriptId.value = script.name;
  outputLines.value = [`> Execution de "${script.name}"...`, ""];
  detectedPaths.value = [];
  startElapsed();
  try {
    const result = await invoke<{ success: boolean; output: string; exit_code: number }>("execute_script", {
      content: script.content,
      scriptType: script.script_type,
    });
    outputLines.value.push(result.output);
    outputLines.value.push("", `--- Termine en ${elapsed.value}s (code: ${result.exit_code}) ---`);
    const paths = extractPaths(result.output);
    detectedPaths.value = [...new Set(paths)].slice(0, 8);
    notify.success("Script termine", `${script.name} — ${elapsed.value}s`);
  } catch {
    outputLines.value.push("Execution simulee en mode dev...");
    outputLines.value.push("Operation terminee.");
    outputLines.value.push("", "--- Termine (code: 0) ---");
    notify.info("Mode dev", `Simulation : ${script.name}`);
  } finally {
    stopElapsed();
    runningScriptId.value = null;
  }
}

// --- Custom script + bibliothèque localStorage ---
const customScript = ref("");
const customType = ref("cmd");
const customRunning = ref(false);
const customName = ref("");

interface SavedScript { id: string; name: string; type: string; content: string; created_at: string }
const savedScripts = ref<SavedScript[]>([]);
const showLibrary = ref(false);

function loadLibrary() {
  try { savedScripts.value = JSON.parse(localStorage.getItem("nitrite_custom_scripts") ?? "[]"); }
  catch { savedScripts.value = []; }
}
function persistLibrary() { localStorage.setItem("nitrite_custom_scripts", JSON.stringify(savedScripts.value)); }

function saveToLibrary() {
  if (!customScript.value.trim()) { notify.warning("Script vide", "Entrez un script avant de sauvegarder."); return; }
  const id = Date.now().toString();
  const name = customName.value.trim() || `Script ${savedScripts.value.length + 1}`;
  savedScripts.value.unshift({ id, name, type: customType.value, content: customScript.value, created_at: new Date().toLocaleString("fr-FR") });
  persistLibrary();
  notify.success("Sauvegardé", `"${name}" ajouté à la bibliothèque`);
  customName.value = "";
}

function loadFromLibrary(s: SavedScript) {
  customScript.value = s.content;
  customType.value = s.type;
  customName.value = s.name;
  showLibrary.value = false;
  notify.info("Script chargé", s.name);
}

function deleteFromLibrary(id: string) {
  savedScripts.value = savedScripts.value.filter(s => s.id !== id);
  persistLibrary();
}

const typeOptions = [
  { value: "cmd", label: "CMD" },
  { value: "powershell", label: "PowerShell" },
];

async function executeCustomScript() {
  if (!customScript.value.trim()) {
    notify.warning("Script vide", "Entrez un script a executer.");
    return;
  }
  customRunning.value = true;
  outputLines.value = [`> Script personnalise (${customType.value})...`, ""];
  startElapsed();
  try {
    const result = await invoke<{ success: boolean; output: string; exit_code: number }>("execute_script", {
      content: customScript.value,
      scriptType: customType.value,
    });
    outputLines.value.push(result.output);
    outputLines.value.push("", `--- Termine en ${elapsed.value}s (code: ${result.exit_code}) ---`);
    notify.success("Script termine", `Script personnalise — ${elapsed.value}s`);
  } catch {
    outputLines.value.push("Execution simulee en mode dev...");
    outputLines.value.push(customScript.value);
    outputLines.value.push("", "--- Termine (code: 0) ---");
    notify.info("Mode dev", "Simulation : script personnalise");
  } finally {
    stopElapsed();
    customRunning.value = false;
  }
}

function clearOutput() {
  outputLines.value = [];
}

// --- Script file browser ---
interface ScriptFile {
  name: string;
  path: string;
  size_bytes: number;
  script_type: string;
}

const scriptFiles = ref<ScriptFile[]>([]);
const scriptFilesLoading = ref(false);
const scriptBrowseDir = ref("C:\\Scripts");
const selectedFile = ref<ScriptFile | null>(null);
const fileContent = ref("");
const fileEdited = ref(false);

async function loadScriptFiles() {
  scriptFilesLoading.value = true;
  try {
    scriptFiles.value = await invoke<ScriptFile[]>("list_script_files", { dir: scriptBrowseDir.value });
  } catch {
    scriptFiles.value = [
      { name: "cleanup.ps1", path: "C:\\Scripts\\cleanup.ps1", size_bytes: 2048, script_type: "powershell" },
      { name: "backup.bat", path: "C:\\Scripts\\backup.bat", size_bytes: 512, script_type: "cmd" },
    ];
  } finally {
    scriptFilesLoading.value = false;
  }
}

async function openScriptFile(file: ScriptFile) {
  selectedFile.value = file;
  fileEdited.value = false;
  try {
    fileContent.value = await invoke<string>("read_script_file", { path: file.path });
  } catch {
    fileContent.value = `# Contenu simule de ${file.name}\necho "Hello World"`;
  }
}

async function saveScriptFile() {
  if (!selectedFile.value) return;
  try {
    await invoke("save_script_file", { path: selectedFile.value.path, content: fileContent.value });
    notify.success("Fichier sauvegarde");
    fileEdited.value = false;
  } catch {
    notify.info("Mode dev", "Sauvegarde simulee");
    fileEdited.value = false;
  }
}

async function runScriptFile() {
  if (!selectedFile.value) return;
  const st = selectedFile.value.script_type === "powershell" ? "powershell" : "cmd";
  outputLines.value = [`> Execution de "${selectedFile.value.name}"...`, ""];
  try {
    const result = await invoke<{ success: boolean; output: string; exit_code: number }>("execute_script", {
      content: fileContent.value,
      scriptType: st,
    });
    outputLines.value.push(result.output);
    outputLines.value.push("", `--- Termine (code: ${result.exit_code}) ---`);
    notify.success("Script termine");
  } catch {
    outputLines.value.push("Execution simulee en mode dev...");
    outputLines.value.push("", "--- Termine (code: 0) ---");
    notify.info("Mode dev", "Simulation");
  }
}

onMounted(() => {
  loadScripts();
  setupListener();
  loadLibrary();
});

onUnmounted(() => {
  if (unlisten) unlisten();
  stopElapsed();
});
</script>

<template>
  <div class="scripts">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1>Scripts</h1>
        <p class="page-subtitle">Executez des scripts de maintenance et de diagnostic</p>
      </div>
    </div>

    <div class="scripts-layout">
      <!-- Left: scripts list -->
      <div class="scripts-panel">
        <!-- Builtin scripts -->
        <NCard>
          <template #header>
            <div class="section-header">
              <FileCode :size="16" />
              <span>Scripts integres</span>
            </div>
          </template>

          <div v-if="scriptsLoading" class="loading-state">
            <NSpinner :size="24" />
            <p>Chargement des scripts...</p>
          </div>

          <template v-else>
            <NTabs v-if="categories.length" :tabs="categories" v-model="activeCategory">
              <template #default="{ activeTab }">
                <div class="scripts-grid">
                  <div
                    v-for="script in filteredScripts"
                    :key="script.name"
                    class="script-card"
                  >
                    <div class="script-header">
                      <span class="script-name">{{ script.name }}</span>
                      <div class="script-badges">
                        <NBadge :variant="script.script_type === 'powershell' ? 'accent' : 'info'">
                          {{ script.script_type === 'powershell' ? 'PowerShell' : 'CMD' }}
                        </NBadge>
                        <NBadge v-if="script.requires_admin" variant="warning">
                          <Shield :size="10" /> Admin
                        </NBadge>
                      </div>
                    </div>
                    <p class="script-desc">{{ script.description }}</p>
                    <NButton
                      variant="secondary"
                      size="sm"
                      :loading="runningScriptId === script.name"
                      :disabled="runningScriptId !== null"
                      @click="executeBuiltinScript(script)"
                    >
                      <Play :size="14" />
                      Executer
                    </NButton>
                  </div>
                </div>
              </template>
            </NTabs>
          </template>
        </NCard>

        <!-- Custom script -->
        <NCard>
          <template #header>
            <div class="section-header">
              <Code :size="16" />
              <span>Script personnalisé</span>
              <div style="margin-left:auto;display:flex;gap:6px">
                <NButton variant="ghost" size="sm" @click="showLibrary = true">
                  <BookOpen :size="13" /> Bibliothèque ({{ savedScripts.length }})
                </NButton>
              </div>
            </div>
          </template>

          <div class="custom-script">
            <div class="custom-top">
              <NInput v-model="customName" placeholder="Nom du script (optionnel)..." style="flex:1" />
              <NDropdown :options="typeOptions" v-model="customType" placeholder="Type" />
              <NButton variant="secondary" size="sm" @click="saveToLibrary" title="Sauvegarder dans la bibliothèque">
                <Save :size="13" />
              </NButton>
              <NButton variant="primary" size="sm" :loading="customRunning" :disabled="customRunning || !customScript.trim()" @click="executeCustomScript">
                <Play :size="14" /> Exécuter
              </NButton>
            </div>
            <textarea
              v-model="customScript"
              class="custom-textarea"
              placeholder="Entrez votre script ici..."
              rows="8"
              spellcheck="false"
            ></textarea>
          </div>
        </NCard>

        <!-- Modal bibliothèque -->
        <NModal :open="showLibrary" @close="showLibrary = false" title="Bibliothèque de scripts">
          <div v-if="savedScripts.length === 0" style="text-align:center;padding:30px;color:var(--text-muted);font-size:13px">
            <BookOpen :size="32" style="opacity:.3;margin-bottom:10px" />
            <p>Aucun script sauvegardé.<br>Créez un script et cliquez <strong>Sauvegarder</strong>.</p>
          </div>
          <div v-else style="display:flex;flex-direction:column;gap:8px;max-height:400px;overflow-y:auto">
            <div v-for="s in savedScripts" :key="s.id" class="lib-item">
              <div class="lib-info">
                <span class="lib-name">{{ s.name }}</span>
                <div style="display:flex;align-items:center;gap:6px;margin-top:2px">
                  <NBadge :variant="s.type === 'powershell' ? 'accent' : 'info'" style="font-size:10px">{{ s.type }}</NBadge>
                  <span style="font-size:10px;color:var(--text-muted)"><Clock :size="10" style="display:inline" /> {{ s.created_at }}</span>
                </div>
                <pre class="lib-preview">{{ s.content.slice(0, 80) }}{{ s.content.length > 80 ? '...' : '' }}</pre>
              </div>
              <div style="display:flex;flex-direction:column;gap:4px">
                <NButton variant="primary" size="sm" @click="loadFromLibrary(s)"><Play :size="12" /> Charger</NButton>
                <NButton variant="danger" size="sm" @click="deleteFromLibrary(s.id)"><Trash2 :size="12" /></NButton>
              </div>
            </div>
          </div>
          <template #footer>
            <NButton variant="ghost" @click="showLibrary = false">Fermer</NButton>
          </template>
        </NModal>

        <!-- Script File Browser -->
        <NCard>
          <template #header>
            <div class="section-header">
              <FolderOpen :size="16" />
              <span>Scripts locaux</span>
              <NButton variant="secondary" size="sm" :loading="scriptFilesLoading" @click="loadScriptFiles" style="margin-left: auto">
                <RefreshCw :size="14" />
              </NButton>
            </div>
          </template>
          <div class="file-browser">
            <div class="browse-bar">
              <input v-model="scriptBrowseDir" class="browse-input" placeholder="C:\Scripts" @keyup.enter="loadScriptFiles" />
              <NButton variant="secondary" size="sm" @click="loadScriptFiles">
                <FolderOpen :size="14" /> Parcourir
              </NButton>
            </div>
            <div v-if="scriptFiles.length" class="file-list">
              <button
                v-for="f in scriptFiles"
                :key="f.path"
                class="file-item"
                :class="{ active: selectedFile?.path === f.path }"
                @click="openScriptFile(f)"
              >
                <FileCode :size="14" />
                <span class="file-name">{{ f.name }}</span>
                <NBadge variant="info">{{ f.script_type }}</NBadge>
              </button>
            </div>
            <p v-else class="file-empty">Aucun script trouve. Cliquez "Parcourir".</p>
            <div v-if="selectedFile" class="file-editor">
              <div class="editor-header">
                <span class="editor-filename">{{ selectedFile.name }}</span>
                <div class="editor-actions">
                  <NButton variant="secondary" size="sm" @click="runScriptFile"><Play :size="14" /> Executer</NButton>
                  <NButton variant="primary" size="sm" :disabled="!fileEdited" @click="saveScriptFile"><Save :size="14" /> Sauvegarder</NButton>
                </div>
              </div>
              <textarea
                v-model="fileContent"
                class="custom-textarea"
                rows="10"
                spellcheck="false"
                @input="fileEdited = true"
              ></textarea>
            </div>
          </div>
        </NCard>
      </div>

      <!-- Right: output console -->
      <NCard class="console-card">
        <template #header>
          <div class="section-header">
            <Terminal :size="16" />
            <span>Console de sortie</span>
            <span v-if="runningScriptId || customRunning" class="elapsed-badge">{{ elapsed }}s</span>
            <NButton variant="secondary" size="sm" @click="clearOutput" style="margin-left: auto">
              <Trash2 :size="14" />
              Vider
            </NButton>
          </div>
        </template>

        <div ref="outputRef" class="console-output">
          <div v-if="outputLines.length === 0" class="console-empty">
            <ScrollText :size="24" style="opacity: 0.3" />
            <p>Executez un script pour voir la sortie ici</p>
          </div>
          <pre v-else class="console-text"><template v-for="(line, i) in outputLines" :key="i">{{ line }}
</template></pre>
        </div>

        <!-- Chemins détectés dans l'output -->
        <div v-if="detectedPaths.length" class="detected-paths">
          <p class="paths-title"><FolderOpen :size="13" /> Chemins détectés — cliquer pour ouvrir :</p>
          <button
            v-for="p in detectedPaths"
            :key="p"
            class="path-chip"
            @click="openPath(p)"
          >
            {{ p }}
          </button>
        </div>
      </NCard>
    </div>
  </div>
</template>

<style scoped>
.scripts {
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

.scripts-layout {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  align-items: start;
}

@media (max-width: 1100px) {
  .scripts-layout { grid-template-columns: 1fr; }
}

.scripts-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 40px;
  color: var(--text-muted);
}

.scripts-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.script-card {
  padding: 14px;
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.script-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.script-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.script-badges {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.script-desc {
  font-size: 12px;
  color: var(--text-muted);
  line-height: 1.4;
}

/* Custom script */
.custom-script {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.custom-top {
  display: flex;
  align-items: center;
  gap: 8px;
}

.custom-textarea {
  width: 100%;
  padding: 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  resize: vertical;
  outline: none;
  transition: border-color var(--transition-fast);
  min-height: 120px;
}

.custom-textarea:focus {
  border-color: var(--accent-primary);
}

.custom-textarea::placeholder {
  color: var(--text-muted);
}

/* Console */
.console-card {
  position: sticky;
  top: 16px;
}

.console-output {
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  min-height: 400px;
  max-height: 600px;
  overflow-y: auto;
  padding: 12px;
}

.console-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: 300px;
  color: var(--text-muted);
  font-size: 13px;
}

.console-text {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  line-height: 1.6;
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
}

.detected-paths {
  margin-top: 12px;
  padding: 10px 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border);
}

.paths-title {
  font-size: 12px;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 8px;
}

.path-chip {
  display: inline-block;
  margin: 3px 4px 3px 0;
  padding: 3px 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--accent-primary);
  border-radius: 20px;
  color: var(--accent-primary);
  font-family: "JetBrains Mono", monospace;
  font-size: 11px;
  cursor: pointer;
  transition: background var(--transition-fast);
}

.path-chip:hover {
  background: color-mix(in srgb, var(--accent-primary) 15%, transparent);
}

/* File browser */
.file-browser { display: flex; flex-direction: column; gap: 12px; }
.browse-bar { display: flex; gap: 8px; }
.browse-input {
  flex: 1; padding: 6px 10px; background: var(--bg-primary); border: 1px solid var(--border);
  border-radius: var(--radius-md); color: var(--text-primary); font-family: "JetBrains Mono", monospace; font-size: 12px;
}
.browse-input:focus { outline: none; border-color: var(--accent-primary); }
.file-list { display: flex; flex-direction: column; gap: 2px; max-height: 200px; overflow-y: auto; }
.file-item {
  display: flex; align-items: center; gap: 8px; padding: 6px 10px; border: none; border-radius: var(--radius-md);
  background: transparent; cursor: pointer; font-family: inherit; text-align: left; width: 100%; transition: background var(--transition-fast);
}
.file-item:hover { background: var(--bg-tertiary); }
.file-item.active { background: var(--accent-muted); }
.file-name { flex: 1; font-size: 13px; color: var(--text-primary); }
.file-empty { font-size: 12px; color: var(--text-muted); text-align: center; padding: 16px; }
.file-editor { display: flex; flex-direction: column; gap: 8px; }
.editor-header { display: flex; align-items: center; justify-content: space-between; }
.editor-filename { font-size: 13px; font-weight: 500; color: var(--text-primary); font-family: "JetBrains Mono", monospace; }
.editor-actions { display: flex; gap: 6px; }

.elapsed-badge {
  font-family: "JetBrains Mono", monospace;
  font-size: 11px;
  color: var(--accent-primary);
  background: var(--accent-muted);
  padding: 2px 8px;
  border-radius: 99px;
  border: 1px solid color-mix(in srgb, var(--accent-primary) 30%, transparent);
  animation: heartbeat 1s ease-in-out infinite;
}

/* Bibliothèque de scripts */
.lib-item {
  display: flex; gap: 12px; align-items: flex-start;
  padding: 12px; border: 1px solid var(--border); border-radius: 8px;
  background: var(--bg-secondary); transition: border-color .15s;
}
.lib-item:hover { border-color: var(--accent-primary); }
.lib-info { flex: 1; min-width: 0; }
.lib-name { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.lib-preview {
  margin-top: 6px; font-family: "JetBrains Mono", monospace; font-size: 11px;
  color: var(--text-muted); background: var(--bg-tertiary); padding: 6px 8px;
  border-radius: 4px; white-space: pre-wrap; word-break: break-all; max-height: 48px; overflow: hidden;
}
</style>
