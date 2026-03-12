<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NButton from "@/components/ui/NButton.vue";
import NInput from "@/components/ui/NInput.vue";
import NToggle from "@/components/ui/NToggle.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import { useAppStore, type ThemeName } from "@/stores/app";
import { useNotificationStore } from "@/stores/notifications";
import { useAiStore } from "@/stores/ai";
import {
  Settings, Palette, Activity, Bot, Download, Info,
  Save, RotateCcw, Wifi, CheckCircle, XCircle, FolderOpen,
} from "lucide-vue-next";

const appStore = useAppStore();
const notify   = useNotificationStore();
const aiStore  = useAiStore();

// ── État onglets ─────────────────────────────────────────────
type Tab = "interface" | "performance" | "ai" | "export" | "about";
const activeTab = ref<Tab>("interface");

// ── Paramètres locaux ─────────────────────────────────────────
// ollamaUrl / ollamaModel / temperature → proxy vers le store AI (v-model compatible)
const ollamaUrl   = computed({ get: () => aiStore.ollamaUrl,   set: v => { aiStore.ollamaUrl   = v; markChanged(); } });
const ollamaModel = computed({ get: () => aiStore.ollamaModel, set: v => { aiStore.ollamaModel = v; markChanged(); } });
const ollamaTemp  = computed({ get: () => aiStore.temperature, set: v => { aiStore.temperature = v; markChanged(); } });
const monitorInterval = ref(2000);
const processCount = ref(10);
const sidebarDefault = ref(false);
const autoSave = ref(false);
const exportFormat = ref<"json" | "txt" | "html" | "md">("json");
const ollamaStatus = ref<"idle" | "ok" | "error">("idle");
const ollamaTesting = ref(false);
const saving = ref(false);
const changed = ref(false);

function markChanged() { changed.value = true; }

// ── Thèmes ────────────────────────────────────────────────────
const themes: { id: ThemeName; label: string; color: string }[] = [
  { id: "nitrite-dark",   label: "Nitrite Dark",      color: "#f97316" },
  { id: "cyber-blue",     label: "Cyber Blue",        color: "#3b82f6" },
  { id: "matrix-green",   label: "Matrix Green",      color: "#22c55e" },
  { id: "purple-haze",    label: "Purple Haze",       color: "#a855f7" },
  { id: "red-alert",      label: "Red Alert",         color: "#ef4444" },
  { id: "arctic-light",   label: "Arctic Light",      color: "#0ea5e9" },
  { id: "midnight-gold",  label: "Midnight Gold",     color: "#eab308" },
  { id: "neon-synthwave", label: "Neon Synthwave",    color: "#f0abfc" },
  { id: "ocean-deep",     label: "Ocean Deep",        color: "#06b6d4" },
  { id: "rose-quartz",    label: "Rose Quartz",       color: "#f43f5e" },
  { id: "void-dark",      label: "Void Dark (AMOLED)", color: "#6366f1" },
  { id: "forest-green",   label: "Forest Green",      color: "#16a34a" },
  { id: "copper-rust",    label: "Copper Rust",       color: "#d97706" },
  { id: "slate-steel",    label: "Slate Steel",       color: "#64748b" },
  { id: "inferno",        label: "Inferno",           color: "#ff4500" },
  { id: "aurora",         label: "Aurora Borealis",   color: "#00d4aa" },
  { id: "moonlight",      label: "Moonlight",         color: "#7dd3fc" },
  { id: "ember-glow",     label: "Ember Glow",        color: "#fb923c" },
  { id: "cobalt-night",   label: "Cobalt Night",      color: "#2563eb" },
  { id: "volcanic",       label: "Volcanic",          color: "#f97316" },
  { id: "sakura",         label: "Sakura (Clair)",    color: "#ec4899" },
  { id: "jade-temple",    label: "Jade Temple",       color: "#10b981" },
  { id: "hacker",         label: "Hacker Terminal",   color: "#00ff41" },
  { id: "ice-storm",      label: "Ice Storm (Clair)", color: "#0284c7" },
  { id: "custom",         label: "Custom",            color: "#6b7280" },
];

// ── Chargement config ─────────────────────────────────────────
onMounted(async () => {
  // Charge la config IA depuis Rust → propage dans le store AI
  await aiStore.loadFromConfig();
  try {
    const cfg = await invoke<any>("get_config");
    monitorInterval.value  = cfg.monitor_interval_ms ?? 2000;
    processCount.value     = cfg.process_count       ?? 10;
    sidebarDefault.value   = cfg.sidebar_collapsed   ?? false;
    exportFormat.value     = cfg.export_format       ?? "json";
    if (cfg.font_size) appStore.setFontSize(cfg.font_size);
    if (cfg.show_animations === false) {
      appStore.showAnimations = false;
      document.documentElement.classList.add("no-animations");
    }
  } catch { /* dev fallback */ }
});

// ── Test Ollama ───────────────────────────────────────────────
async function testOllama() {
  ollamaTesting.value = true;
  ollamaStatus.value = "idle";
  try {
    const ok = await invoke<boolean>("ai_check");
    ollamaStatus.value = ok ? "ok" : "error";
  } catch {
    ollamaStatus.value = "error";
  }
  ollamaTesting.value = false;
}

// ── Ouvrir dossier exports ────────────────────────────────────
async function openExportFolder() {
  try {
    const dir = await invoke<string>("get_export_dir");
    await invoke("open_path", { path: dir });
  } catch (e: any) {
    notify.error("Impossible d'ouvrir le dossier", String(e));
  }
}

// ── Sauvegarde ───────────────────────────────────────────────
async function saveSettings() {
  saving.value = true;
  try {
    await invoke("save_config", {
      config: {
        theme:                appStore.theme,
        language:             appStore.language,
        sidebar_collapsed:    sidebarDefault.value,
        ollama_url:           aiStore.ollamaUrl,
        ollama_model:         aiStore.ollamaModel,
        ollama_temperature:   aiStore.temperature,
        monitor_interval_ms:  monitorInterval.value,
        show_animations:      appStore.showAnimations,
        compact_mode:         false,
        notifications_enabled: true,
        process_count:        processCount.value,
        font_size:            appStore.fontSize,
        export_format:        exportFormat.value,
      },
    });
    notify.success("Paramètres sauvegardés");
    changed.value = false;
  } catch (e: any) {
    notify.error("Erreur sauvegarde", String(e));
  }
  saving.value = false;
}

// ── Reset ─────────────────────────────────────────────────────
function resetDefaults() {
  appStore.setTheme("nitrite-dark");
  appStore.setFontSize("normal");
  if (!appStore.showAnimations) appStore.toggleAnimations();
  aiStore.ollamaUrl   = "http://localhost:11434";
  aiStore.ollamaModel = "llama3:8b";
  aiStore.temperature = 0.7;
  monitorInterval.value = 2000;
  processCount.value = 10;
  sidebarDefault.value = false;
  exportFormat.value = "json";
  changed.value = true;
  notify.info("Paramètres réinitialisés — cliquez Save pour confirmer");
}

const tabs: { id: Tab; label: string; icon: any }[] = [
  { id: "interface",   label: "Interface",    icon: Palette  },
  { id: "performance", label: "Performance",  icon: Activity },
  { id: "ai",          label: "IA (Ollama)",  icon: Bot      },
  { id: "export",      label: "Export",       icon: Download },
  { id: "about",       label: "À propos",     icon: Info     },
];
</script>

<template>
  <div class="settings-page">
    <DiagBanner :icon="Settings" title="Paramètres" desc="Interface, performance, IA et exports" color="indigo" />

    <div class="settings-layout">
      <!-- Sidebar onglets -->
      <nav class="settings-nav">
        <button
          v-for="t in tabs"
          :key="t.id"
          class="settings-nav-item"
          :class="{ active: activeTab === t.id }"
          @click="activeTab = t.id"
        >
          <component :is="t.icon" :size="15" />
          <span>{{ t.label }}</span>
        </button>
      </nav>

      <!-- Contenu -->
      <div class="settings-content">

        <!-- ══ INTERFACE ══ -->
        <div v-if="activeTab === 'interface'" class="tab-section">
          <h2 class="tab-title"><Palette :size="16" /> Interface</h2>

          <div class="setting-group">
            <p class="setting-label">Thème</p>
            <div class="theme-grid">
              <button
                v-for="t in themes"
                :key="t.id"
                class="theme-btn"
                :class="{ active: appStore.theme === t.id }"
                @click="appStore.setTheme(t.id); markChanged()"
              >
                <div class="theme-swatch" :style="{ background: t.color, boxShadow: appStore.theme === t.id ? `0 0 10px ${t.color}80` : 'none' }"></div>
                <span>{{ t.label }}</span>
                <span v-if="appStore.theme === t.id" class="theme-check">✓</span>
              </button>
            </div>
          </div>

          <div class="setting-group">
            <p class="setting-label">Taille de police</p>
            <div class="btn-group">
              <button
                v-for="sz in ['small','normal','large'] as const"
                :key="sz"
                class="size-btn"
                :class="{ active: appStore.fontSize === sz }"
                @click="appStore.setFontSize(sz); markChanged()"
              >{{ sz === 'small' ? 'Petit' : sz === 'normal' ? 'Normal' : 'Grand' }}</button>
            </div>
          </div>

          <div class="setting-group">
            <div class="setting-row">
              <div>
                <p class="setting-label">Animations</p>
                <p class="setting-desc">Transitions et effets visuels</p>
              </div>
              <NToggle :modelValue="appStore.showAnimations" label="" @update:modelValue="appStore.toggleAnimations(); markChanged()" />
            </div>
          </div>

          <div class="setting-group">
            <p class="setting-label">Langue</p>
            <div class="btn-group">
              <button
                class="size-btn"
                :class="{ active: appStore.language === 'fr' }"
                @click="appStore.language = 'fr'; markChanged()"
              >Français</button>
              <button
                class="size-btn"
                :class="{ active: appStore.language === 'en' }"
                @click="appStore.language = 'en'; markChanged()"
              >English</button>
            </div>
          </div>
        </div>

        <!-- ══ PERFORMANCE ══ -->
        <div v-else-if="activeTab === 'performance'" class="tab-section">
          <h2 class="tab-title"><Activity :size="16" /> Performance</h2>

          <div class="setting-group">
            <p class="setting-label">Intervalle de monitoring</p>
            <div class="slider-row">
              <input type="range" min="500" max="5000" step="100" v-model.number="monitorInterval" class="slider" @input="markChanged" />
              <span class="slider-value">{{ monitorInterval }}ms</span>
            </div>
          </div>

          <div class="setting-group">
            <p class="setting-label">Nombre de processus affichés</p>
            <div class="slider-row">
              <input type="range" min="5" max="50" step="5" v-model.number="processCount" class="slider" @input="markChanged" />
              <span class="slider-value">{{ processCount }}</span>
            </div>
          </div>

          <div class="setting-group">
            <div class="setting-row">
              <div>
                <p class="setting-label">Sidebar repliée par défaut</p>
                <p class="setting-desc">Fermer le panneau latéral au démarrage</p>
              </div>
              <NToggle v-model="sidebarDefault" label="" @update:modelValue="markChanged" />
            </div>
          </div>

          <div class="setting-group">
            <div class="setting-row">
              <div>
                <p class="setting-label">Auto-save des paramètres</p>
                <p class="setting-desc">Sauvegarder automatiquement à chaque changement</p>
              </div>
              <NToggle v-model="autoSave" label="" @update:modelValue="markChanged" />
            </div>
          </div>
        </div>

        <!-- ══ IA ══ -->
        <div v-else-if="activeTab === 'ai'" class="tab-section">
          <h2 class="tab-title"><Bot :size="16" /> IA (Ollama)</h2>

          <div class="setting-group">
            <label class="setting-label">URL Ollama</label>
            <NInput v-model="ollamaUrl" placeholder="http://localhost:11434" @input="markChanged" />
          </div>

          <div class="setting-group">
            <label class="setting-label">Modèle par défaut</label>
            <NInput v-model="ollamaModel" placeholder="llama3:8b" @input="markChanged" />
          </div>

          <div class="setting-group">
            <label class="setting-label">Température (créativité) — {{ ollamaTemp.toFixed(1) }}</label>
            <input type="range" min="0" max="2" step="0.1" v-model.number="ollamaTemp"
              @input="markChanged" class="range-slider" />
            <div class="range-labels"><span>Précis (0)</span><span>Équilibré (0.7)</span><span>Créatif (2)</span></div>
          </div>

          <div class="setting-group">
            <div class="ai-test-row">
              <NButton variant="ghost" :loading="ollamaTesting" @click="testOllama">
                <Wifi :size="13" /> Tester la connexion
              </NButton>
              <div v-if="ollamaStatus !== 'idle'" class="ai-status" :class="ollamaStatus">
                <CheckCircle v-if="ollamaStatus === 'ok'" :size="14" />
                <XCircle    v-else                        :size="14" />
                <span>{{ ollamaStatus === 'ok' ? 'Connecté' : 'Hors ligne' }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- ══ EXPORT ══ -->
        <div v-else-if="activeTab === 'export'" class="tab-section">
          <h2 class="tab-title"><Download :size="16" /> Export</h2>

          <div class="setting-group">
            <p class="setting-label">Format préféré</p>
            <div class="radio-group">
              <label v-for="fmt in ['json','txt','html','md']" :key="fmt" class="radio-item">
                <input
                  type="radio"
                  :value="fmt"
                  v-model="exportFormat"
                  @change="markChanged"
                />
                <span>.{{ fmt.toUpperCase() }}</span>
              </label>
            </div>
          </div>

          <div class="setting-group">
            <p class="setting-label">Historique des exports</p>
            <NButton variant="ghost" @click="openExportFolder">
              <FolderOpen :size="13" /> Ouvrir le dossier exports
            </NButton>
          </div>
        </div>

        <!-- ══ À PROPOS ══ -->
        <div v-else-if="activeTab === 'about'" class="tab-section">
          <h2 class="tab-title"><Info :size="16" /> À propos</h2>

          <div class="about-card">
            <div class="about-badge">v26.9.0</div>
            <p class="about-name">NiTriTe</p>
            <p class="about-sub">Outil de diagnostic et maintenance Windows</p>
          </div>

          <div class="about-rows">
            <div class="about-row"><span>Stack</span><span>Tauri v2 + Rust + Vue 3 + TypeScript</span></div>
            <div class="about-row"><span>Auteur</span><span>Momo</span></div>
            <div class="about-row"><span>Licence</span><span>MIT</span></div>
          </div>

          <div class="about-actions">
            <NButton variant="ghost" @click="resetDefaults">
              <RotateCcw :size="13" /> Réinitialiser tous les paramètres
            </NButton>
          </div>
        </div>

      </div>
    </div>

    <!-- Bouton Save flottant -->
    <div class="save-fab" :class="{ visible: changed }">
      <NButton variant="primary" :loading="saving" @click="saveSettings">
        <Save :size="14" /> Sauvegarder
      </NButton>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

.settings-layout {
  display: flex;
  gap: 16px;
  flex: 1;
  min-height: 0;
}

/* ── Sidebar ── */
.settings-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
  width: 160px;
  flex-shrink: 0;
}

.settings-nav-item {
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 9px 12px;
  border-radius: var(--radius-md);
  border: 1px solid transparent;
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  color: var(--text-secondary);
  transition: all 0.15s;
  text-align: left;
}
.settings-nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}
.settings-nav-item.active {
  background: var(--bg-secondary);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

/* ── Content ── */
.settings-content {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border);
  padding: 20px;
}

.tab-section {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.tab-title {
  font-size: 15px;
  font-weight: 700;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 8px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border);
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.setting-desc {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 2px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border);
}

/* ── Theme grid ── */
.theme-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 8px;
}
@media (max-width: 900px) {
  .theme-grid { grid-template-columns: repeat(3, 1fr); }
}

.theme-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 9px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  cursor: pointer;
  font-family: inherit;
  font-size: 12px;
  color: var(--text-primary);
  transition: all 0.15s;
  position: relative;
}
.theme-btn:hover { border-color: var(--border-hover); background: var(--bg-elevated); }
.theme-btn.active {
  border-color: var(--accent-primary);
  background: var(--bg-elevated);
  box-shadow: 0 0 0 2px var(--accent-muted);
}
.theme-swatch {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  flex-shrink: 0;
}
.theme-check {
  margin-left: auto;
  color: var(--accent-primary);
  font-size: 11px;
  font-weight: 700;
}

/* ── Button group ── */
.btn-group {
  display: flex;
  gap: 6px;
}
.size-btn {
  padding: 7px 16px;
  border: 1.5px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  cursor: pointer;
  font-family: inherit;
  font-size: 12px;
  color: var(--text-primary);
  transition: all 0.15s;
}
.size-btn:hover { border-color: var(--text-muted); }
.size-btn.active { border-color: var(--accent-primary); background: var(--bg-elevated); color: var(--accent-primary); }

/* ── Slider ── */
.slider-row {
  display: flex;
  align-items: center;
  gap: 14px;
}
.slider {
  flex: 1;
  accent-color: var(--accent-primary);
  height: 6px;
}
.slider-value {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  color: var(--accent-primary);
  min-width: 60px;
  text-align: right;
}

/* ── AI test ── */
.ai-test-row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.ai-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  padding: 5px 10px;
  border-radius: var(--radius-sm);
}
.ai-status.ok    { color: var(--success); background: var(--success-muted); }
.ai-status.error { color: var(--danger);  background: var(--danger-muted);  }

/* ── Radio group ── */
.radio-group {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}
.radio-item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  cursor: pointer;
  padding: 8px 14px;
  border: 1.5px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  transition: all 0.15s;
}
.radio-item:has(input:checked) {
  border-color: var(--accent-primary);
  background: var(--bg-elevated);
  color: var(--accent-primary);
}
.radio-item input { display: none; }

/* ── About ── */
.about-card {
  text-align: center;
  padding: 20px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border);
}
.about-badge {
  display: inline-block;
  background: var(--accent-muted);
  color: var(--accent-primary);
  border: 1px solid var(--accent-primary);
  border-radius: 20px;
  padding: 3px 12px;
  font-size: 11px;
  font-weight: 700;
  margin-bottom: 8px;
}
.about-name {
  font-size: 20px;
  font-weight: 800;
  color: var(--text-primary);
}
.about-sub {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 4px;
}
.about-rows {
  display: flex;
  flex-direction: column;
  gap: 0;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border);
  overflow: hidden;
}
.about-row {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  padding: 8px 14px;
  border-bottom: 1px solid var(--border);
}
.about-row:last-child { border-bottom: none; }
.about-row span:first-child { color: var(--text-muted); }
.about-row span:last-child  { color: var(--text-primary); font-weight: 500; }

.about-actions {
  display: flex;
  gap: 10px;
}

/* ── FAB Save ── */
.save-fab {
  position: fixed;
  bottom: 24px;
  right: 28px;
  opacity: 0;
  pointer-events: none;
  transform: translateY(12px);
  transition: all 0.2s;
  z-index: 100;
}
.save-fab.visible {
  opacity: 1;
  pointer-events: auto;
  transform: translateY(0);
}
</style>
