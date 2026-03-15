<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import {
  useThemeEditorStore, PRESET_THEMES, THEME_VAR_GROUPS, PRESET_THEME_GROUPS,
} from "@/stores/themeEditor";
import { useLayoutStore, LAYOUT_PRESETS, TAB_STYLE_LABELS, GROUP_NAV_STYLE_LABELS } from "@/stores/layoutStore";
import type { LayoutPresetId, SidebarPosition, SidebarMode, SidebarWidth, UIDensity, ContentMaxWidth, TabStyle, GroupNavStyle } from "@/stores/layoutStore";
import { useNotificationStore } from "@/stores/notifications";
import ThemePreview from "@/components/theme/ThemePreview.vue";
import {
  Palette, Save, Download, Upload, Eye, EyeOff, Trash2,
  RotateCcw, Check, Sparkles, Monitor, Copy, Layout, Sidebar,
  AlignLeft, AlignRight, Minimize2, Maximize2, Type,
} from "lucide-vue-next";

const store = useThemeEditorStore();
const layoutStore = useLayoutStore();
const notify = useNotificationStore();
const activeTab = ref<"theme" | "layout">("theme");

const fileInput = ref<HTMLInputElement | null>(null);

onMounted(() => {
  store.loadSavedThemes();
  // Bloquer le scroll de app-content pour que te-controls gère son propre scroll
  const appContent = document.querySelector(".app-content") as HTMLElement | null;
  if (appContent) {
    appContent.dataset.prevOverflow = appContent.style.overflow;
    appContent.style.overflow = "hidden";
  }
});

onUnmounted(() => {
  if (store.globalPreviewActive) store.toggleGlobalPreview(false);
  // Restaurer le scroll de app-content
  const appContent = document.querySelector(".app-content") as HTMLElement | null;
  if (appContent) {
    appContent.style.overflow = appContent.dataset.prevOverflow ?? "";
  }
});

function selectPreset(id: string) {
  store.loadPreset(id);
}

function handleColorChange(key: string, e: Event) {
  store.setVar(key, (e.target as HTMLInputElement).value);
}

function handleRadiusChange(key: string, e: Event) {
  store.setVar(key, `${(e.target as HTMLInputElement).value}px`);
}

function radiusValue(val: string): number {
  return parseInt(val) || 0;
}

function handleSave() {
  store.saveTheme();
  notify.success("Thème sauvegardé", `"${store.themeName}" ajouté à vos thèmes.`);
}

function handleExport() {
  const json = store.exportTheme();
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${store.themeName.replace(/\s+/g, "_")}.theme.json`;
  a.click();
  URL.revokeObjectURL(url);
  notify.success("Exporté", "Thème exporté en JSON.");
}

function handleImportClick() {
  fileInput.value?.click();
}

function handleFileImport(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = (ev) => {
    try {
      store.importTheme(ev.target?.result as string);
      notify.success("Importé", "Thème chargé avec succès.");
    } catch {
      notify.error("Erreur", "Fichier JSON invalide.");
    }
  };
  reader.readAsText(file);
  (e.target as HTMLInputElement).value = "";
}

function toggleGlobalPreview() {
  store.toggleGlobalPreview(!store.globalPreviewActive);
  notify.info(
    store.globalPreviewActive ? "Aperçu global activé" : "Aperçu global désactivé",
    store.globalPreviewActive ? "L'app entière reflète vos changements." : "Les couleurs d'origine sont restaurées.",
  );
}

function copyCurrentTheme() {
  const json = store.exportTheme();
  navigator.clipboard.writeText(json).then(() => {
    notify.success("Copié", "Thème copié dans le presse-papiers.");
  });
}

const currentPresetId = computed(() =>
  PRESET_THEMES.find(p => JSON.stringify(p.vars) === JSON.stringify(store.editingVars))?.id ?? null
);
</script>

<template>
  <div class="te-page">

    <!-- Header -->
    <div class="te-header">
      <div class="te-header-left">
        <div class="te-title-icon">
          <Palette :size="20" />
        </div>
        <div>
          <h1 class="te-title">Éditeur de Thème</h1>
          <p class="te-subtitle">Personnalisez chaque couleur, rayon et effet de l'interface</p>
        </div>
      </div>
      <div class="te-header-actions">
        <input
          v-model="store.themeName"
          class="te-name-input"
          placeholder="Nom du thème..."
        />
        <button class="te-action-btn" :class="{ active: store.globalPreviewActive }" @click="toggleGlobalPreview" title="Aperçu global">
          <Eye v-if="!store.globalPreviewActive" :size="15" />
          <EyeOff v-else :size="15" />
          {{ store.globalPreviewActive ? "Désactiver aperçu" : "Aperçu global" }}
        </button>
        <button class="te-action-btn" @click="copyCurrentTheme" title="Copier JSON">
          <Copy :size="15" />
        </button>
        <button class="te-action-btn" @click="handleImportClick" title="Importer JSON">
          <Upload :size="15" />
          Importer
        </button>
        <button class="te-action-btn" @click="handleExport" title="Exporter JSON">
          <Download :size="15" />
          Exporter
        </button>
        <button class="te-action-btn te-action-btn--primary" @click="handleSave">
          <Save :size="15" />
          Sauvegarder
        </button>
      </div>
      <input ref="fileInput" type="file" accept=".json" style="display:none" @change="handleFileImport" />
    </div>

    <!-- Body -->
    <div class="te-body">

      <!-- Left panel: controls -->
      <div class="te-controls">

        <!-- Tab switcher -->
        <div class="te-tabs">
          <button class="te-tab" :class="{ active: activeTab === 'theme' }" @click="activeTab = 'theme'">
            <Palette :size="13" /> Thème Couleurs
          </button>
          <button class="te-tab" :class="{ active: activeTab === 'layout' }" @click="activeTab = 'layout'">
            <Layout :size="13" /> Disposition
          </button>
        </div>

        <!-- ===== ONGLET LAYOUT ===== -->
        <template v-if="activeTab === 'layout'">

          <!-- Presets disposition -->
          <div class="te-section">
            <div class="te-section-title"><Layout :size="13" /> Presets de disposition</div>
            <div class="te-presets-grid">
              <button
                v-for="p in LAYOUT_PRESETS" :key="p.id"
                class="te-preset-btn"
                :class="{ active: layoutStore.state.activePreset === p.id }"
                @click="layoutStore.applyPreset(p.id as LayoutPresetId)"
              >
                <span class="te-preset-emoji">{{ p.emoji }}</span>
                <div class="te-preset-info">
                  <span class="te-preset-label">{{ p.label }}</span>
                  <span class="te-preset-desc">{{ p.description }}</span>
                </div>
                <Check v-if="layoutStore.state.activePreset === p.id" :size="11" class="te-preset-check" />
              </button>
            </div>
          </div>

          <!-- Sidebar -->
          <div class="te-section">
            <div class="te-section-title"><Sidebar :size="13" /> Sidebar</div>
            <div class="te-vars-list">
              <div class="te-var-row">
                <label class="te-var-label">Position</label>
                <div class="te-btn-group">
                  <button
                    class="te-seg-btn" :class="{ active: layoutStore.state.sidebarPosition === 'left' }"
                    @click="layoutStore.setField('sidebarPosition', 'left')"
                  ><AlignLeft :size="12" /> Gauche</button>
                  <button
                    class="te-seg-btn" :class="{ active: layoutStore.state.sidebarPosition === 'right' }"
                    @click="layoutStore.setField('sidebarPosition', 'right')"
                  ><AlignRight :size="12" /> Droite</button>
                </div>
              </div>
              <div class="te-var-row">
                <label class="te-var-label">Mode</label>
                <div class="te-btn-group">
                  <button
                    class="te-seg-btn" :class="{ active: layoutStore.state.sidebarMode === 'icons-text' }"
                    @click="layoutStore.setField('sidebarMode', 'icons-text')"
                  >Icônes + texte</button>
                  <button
                    class="te-seg-btn" :class="{ active: layoutStore.state.sidebarMode === 'icons-only' }"
                    @click="layoutStore.setField('sidebarMode', 'icons-only')"
                  >Icônes seules</button>
                </div>
              </div>
              <div class="te-var-row">
                <label class="te-var-label">Largeur</label>
                <div class="te-btn-group">
                  <button v-for="w in (['compact','normal','large'] as SidebarWidth[])" :key="w"
                    class="te-seg-btn" :class="{ active: layoutStore.state.sidebarWidth === w }"
                    @click="layoutStore.setField('sidebarWidth', w)"
                  >{{ w === 'compact' ? '48px' : w === 'normal' ? '240px' : '290px' }}</button>
                </div>
              </div>
            </div>
          </div>

          <!-- Header -->
          <div class="te-section">
            <div class="te-section-title"><Monitor :size="13" /> Header</div>
            <div class="te-vars-list">
              <div class="te-var-row">
                <label class="te-var-label">Visible</label>
                <div class="te-btn-group">
                  <button class="te-seg-btn" :class="{ active: layoutStore.state.headerVisible }" @click="layoutStore.setField('headerVisible', true)">Oui</button>
                  <button class="te-seg-btn" :class="{ active: !layoutStore.state.headerVisible }" @click="layoutStore.setField('headerVisible', false)">Non</button>
                </div>
              </div>
            </div>
          </div>

          <!-- Contenu -->
          <div class="te-section">
            <div class="te-section-title"><Maximize2 :size="13" /> Contenu</div>
            <div class="te-vars-list">
              <div class="te-var-row">
                <label class="te-var-label">Largeur max</label>
                <select class="te-select" :value="layoutStore.state.contentMaxWidth" @change="layoutStore.setField('contentMaxWidth', ($event.target as HTMLSelectElement).value as ContentMaxWidth)">
                  <option value="full">Pleine largeur</option>
                  <option value="1400px">1400px</option>
                  <option value="1200px">1200px</option>
                  <option value="960px">960px</option>
                </select>
              </div>
              <div class="te-var-row te-var-row--radius">
                <label class="te-var-label">Padding ({{ layoutStore.state.contentPadding }}px)</label>
                <div class="te-var-controls">
                  <input type="range" class="te-range-input" min="8" max="48"
                    :value="layoutStore.state.contentPadding"
                    @input="layoutStore.setField('contentPadding', parseInt(($event.target as HTMLInputElement).value))"
                  />
                </div>
              </div>
            </div>
          </div>

          <!-- Densité & Typographie -->
          <div class="te-section">
            <div class="te-section-title"><Type :size="13" /> Densité &amp; Typographie</div>
            <div class="te-vars-list">
              <div class="te-var-row">
                <label class="te-var-label">Densité UI</label>
                <div class="te-btn-group">
                  <button v-for="d in (['compact','normal','spacious'] as UIDensity[])" :key="d"
                    class="te-seg-btn" :class="{ active: layoutStore.state.density === d }"
                    @click="layoutStore.setField('density', d)"
                  >{{ d === 'compact' ? 'Compact' : d === 'normal' ? 'Normal' : 'Aéré' }}</button>
                </div>
              </div>
              <div class="te-var-row te-var-row--radius">
                <label class="te-var-label">Taille police ({{ layoutStore.state.fontSize }}px)</label>
                <div class="te-var-controls">
                  <input type="range" class="te-range-input" min="11" max="17"
                    :value="layoutStore.state.fontSize"
                    @input="layoutStore.setField('fontSize', parseInt(($event.target as HTMLInputElement).value))"
                  />
                </div>
              </div>
            </div>
          </div>

          <!-- Navigation & Onglets -->
          <div class="te-section">
            <div class="te-section-title"><Layout :size="13" /> Navigation &amp; Onglets</div>
            <div class="te-vars-list">

              <!-- Style des onglets -->
              <div class="te-var-row">
                <label class="te-var-label">Style onglets</label>
                <div class="te-btn-group te-btn-group--wrap">
                  <button
                    v-for="(label, key) in TAB_STYLE_LABELS" :key="key"
                    class="te-seg-btn"
                    :class="{ active: layoutStore.state.tabStyle === key }"
                    @click="layoutStore.setField('tabStyle', key as TabStyle)"
                  >{{ label }}</button>
                </div>
              </div>

              <!-- Style nav groupes -->
              <div class="te-var-row">
                <label class="te-var-label">Navigation groupes</label>
                <div class="te-btn-group te-btn-group--wrap">
                  <button
                    v-for="(label, key) in GROUP_NAV_STYLE_LABELS" :key="key"
                    class="te-seg-btn"
                    :class="{ active: layoutStore.state.groupNavStyle === key }"
                    @click="layoutStore.setField('groupNavStyle', key as GroupNavStyle)"
                  >{{ label }}</button>
                </div>
              </div>

              <!-- Prévisualisation style onglets -->
              <div class="te-var-row te-nav-preview">
                <label class="te-var-label">Aperçu</label>
                <div class="te-tabs-preview" :data-preview-tab="layoutStore.state.tabStyle">
                  <button class="tp-tab tp-tab--active">Système</button>
                  <button class="tp-tab">Réseau</button>
                  <button class="tp-tab">Stockage</button>
                </div>
              </div>

            </div>
          </div>

          <!-- Reset -->
          <button class="te-action-btn" style="width:100%;justify-content:center" @click="layoutStore.reset()">
            <RotateCcw :size="13" /> Réinitialiser le layout
          </button>

        </template>

        <!-- ===== ONGLET THEME ===== -->
        <template v-if="activeTab === 'theme'">

        <!-- Preset gallery groupée -->
        <div class="te-section">
          <div class="te-section-title">
            <Sparkles :size="13" />
            Thèmes Prédéfinis
            <span class="te-count">{{ PRESET_THEMES.length }}</span>
          </div>
          <div class="te-presets-grid">
            <template v-for="group in PRESET_THEME_GROUPS" :key="group.label">
              <div class="te-preset-group-label">{{ group.label }}</div>
              <button
                v-for="p in PRESET_THEMES.filter(t => (group.ids as readonly string[]).includes(t.id))"
                :key="p.id"
                class="te-preset-btn"
                :class="{ active: currentPresetId === p.id }"
                @click="selectPreset(p.id)"
                :title="p.label"
              >
                <span class="te-preset-dot" :style="{ background: p.accent }"></span>
                <span class="te-preset-label">{{ p.label }}</span>
                <Check v-if="currentPresetId === p.id" :size="11" class="te-preset-check" />
              </button>
            </template>
          </div>
        </div>

        <!-- Variable groups -->
        <div
          v-for="group in THEME_VAR_GROUPS"
          :key="group.label"
          class="te-section"
        >
          <div class="te-section-title">{{ group.label }}</div>

          <!-- Color vars -->
          <div class="te-vars-list" v-if="group.vars.some((v: any) => v.type === 'color')">
            <div
              v-for="v in group.vars.filter((v: any) => v.type === 'color')"
              :key="v.key"
              class="te-var-row"
            >
              <label class="te-var-label">{{ v.label }}</label>
              <div class="te-var-controls">
                <input
                  type="color"
                  class="te-color-input"
                  :value="store.editingVars[v.key] ?? '#000000'"
                  @input="handleColorChange(v.key, $event)"
                />
                <input
                  type="text"
                  class="te-hex-input"
                  :value="store.editingVars[v.key] ?? '#000000'"
                  @change="handleColorChange(v.key, $event)"
                  maxlength="7"
                />
              </div>
            </div>
          </div>

          <!-- Radius vars -->
          <div class="te-vars-list" v-if="group.vars.some((v: any) => v.type === 'radius')">
            <div
              v-for="v in group.vars.filter((v: any) => v.type === 'radius')"
              :key="v.key"
              class="te-var-row te-var-row--radius"
            >
              <label class="te-var-label">{{ v.label }}</label>
              <div class="te-var-controls">
                <input
                  type="range"
                  class="te-range-input"
                  :min="(v as any).min ?? 0"
                  :max="(v as any).max ?? 32"
                  :value="radiusValue(store.editingVars[v.key] ?? '8px')"
                  @input="handleRadiusChange(v.key, $event)"
                />
                <span class="te-range-val">{{ store.editingVars[v.key] ?? '8px' }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Saved themes -->
        <div class="te-section" v-if="store.savedThemes.length > 0">
          <div class="te-section-title">
            <Save :size="13" />
            Mes Thèmes
            <span class="te-count">{{ store.savedThemes.length }}</span>
          </div>
          <div class="te-saved-list">
            <div
              v-for="t in store.savedThemes"
              :key="t.id"
              class="te-saved-item"
            >
              <button class="te-saved-load" @click="store.loadSavedTheme(t.id)">
                {{ t.name }}
              </button>
              <button class="te-saved-del" @click="store.deleteSavedTheme(t.id)" title="Supprimer">
                <Trash2 :size="12" />
              </button>
            </div>
          </div>
        </div>

        </template> <!-- fin onglet theme -->

      </div>

      <!-- Right panel: live preview -->
      <div class="te-preview-panel">
        <div class="te-preview-header">
          <Monitor :size="14" />
          <span>Prévisualisation en temps réel</span>
          <span class="te-preview-badge">LIVE</span>
        </div>
        <div class="te-preview-content">
          <ThemePreview />
        </div>
      </div>

    </div>
  </div>
</template>

<style scoped>
.te-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: calc(100vh - var(--layout-header-height, 52px) - 26px - calc(var(--layout-content-padding, 24px) * 2));
  max-height: calc(100vh - var(--layout-header-height, 52px) - 26px - calc(var(--layout-content-padding, 24px) * 2));
  overflow: hidden;
}

/* Header */
.te-header {
  display: flex;
  align-items: center;
  gap: 14px;
  flex-wrap: wrap;
  flex-shrink: 0;
}
.te-header-left {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}
.te-title-icon {
  width: 40px; height: 40px;
  border-radius: var(--radius-lg);
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-hover));
  display: flex; align-items: center; justify-content: center;
  color: white;
  box-shadow: var(--accent-glow-sm);
  flex-shrink: 0;
}
.te-title {
  font-size: 20px; font-weight: 800;
  background: linear-gradient(135deg, var(--text-primary) 40%, var(--accent-primary));
  -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
  letter-spacing: -0.3px;
}
.te-subtitle { font-size: 12px; color: var(--text-secondary); margin-top: 1px; }
.te-header-actions { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.te-name-input {
  padding: 7px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-hover);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 13px;
  outline: none;
  width: 180px;
  transition: border-color var(--transition-fast);
}
.te-name-input:focus { border-color: var(--accent-primary); }
.te-action-btn {
  display: inline-flex; align-items: center; gap: 6px;
  padding: 7px 13px;
  border: 1px solid var(--border-hover);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-size: 12px; font-weight: 500; font-family: inherit;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
}
.te-action-btn:hover { background: var(--bg-elevated); color: var(--text-primary); border-color: var(--border-strong); }
.te-action-btn.active {
  background: var(--accent-muted);
  border-color: rgba(249,115,22,0.4);
  color: var(--accent-primary);
}
.te-action-btn--primary {
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-hover));
  color: white; border-color: transparent;
  box-shadow: 0 2px 10px rgba(249,115,22,0.25);
}
.te-action-btn--primary:hover { box-shadow: var(--accent-glow); transform: translateY(-1px); }

/* Body */
.te-body {
  display: flex;
  flex-direction: row;
  gap: 16px;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

/* Controls */
.te-controls {
  width: 340px;
  flex-shrink: 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
  overflow-y: auto;
  padding-right: 2px;
  scrollbar-width: thin;
  scrollbar-color: var(--border-hover) transparent;
}
.te-controls::-webkit-scrollbar { width: 5px; }
.te-controls::-webkit-scrollbar-track { background: transparent; }
.te-controls::-webkit-scrollbar-thumb { background: var(--border-hover); border-radius: 99px; }
.te-section {
  flex-shrink: 0;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-xl);
  overflow: hidden;
}
.te-section-title {
  display: flex;
  align-items: center;
  gap: 7px;
  padding: 10px 14px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
}
.te-count {
  margin-left: auto;
  background: var(--bg-elevated);
  color: var(--text-muted);
  font-size: 10px;
  padding: 1px 6px;
  border-radius: 99px;
}

/* Preset grid */
.te-presets-grid {
  display: flex;
  flex-direction: column;
  gap: 0;
  max-height: 420px;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: var(--border-hover) transparent;
}
.te-presets-grid::-webkit-scrollbar { width: 4px; }
.te-presets-grid::-webkit-scrollbar-thumb { background: var(--border-hover); border-radius: 99px; }
.te-preset-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 8px 14px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 12px;
  font-family: inherit;
  text-align: left;
  transition: all var(--transition-fast);
  border-bottom: 1px solid var(--border);
}
.te-preset-btn:last-child { border-bottom: none; }
.te-preset-btn:hover { background: var(--surface-glass); color: var(--text-primary); }
.te-preset-btn.active {
  background: var(--accent-muted);
  color: var(--accent-primary);
  font-weight: 600;
}
.te-preset-group-label {
  padding: 6px 14px 4px;
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border);
  position: sticky;
  top: 0;
  z-index: 1;
}
.te-preset-dot {
  width: 12px; height: 12px; border-radius: 50%; flex-shrink: 0;
  box-shadow: 0 0 6px currentColor;
}
.te-preset-label { flex: 1; }
.te-preset-check { color: var(--accent-primary); }

/* Variable list */
.te-vars-list { padding: 8px 10px; display: flex; flex-direction: column; gap: 6px; }
.te-var-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 4px 0;
}
.te-var-label { font-size: 12px; color: var(--text-secondary); flex: 1; }
.te-var-controls { display: flex; align-items: center; gap: 6px; }
.te-color-input {
  width: 32px; height: 28px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-hover);
  padding: 2px;
  cursor: pointer;
  background: var(--bg-tertiary);
}
.te-hex-input {
  width: 80px;
  padding: 4px 8px;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 11px;
  font-family: "JetBrains Mono", monospace;
  outline: none;
}
.te-hex-input:focus { border-color: var(--accent-primary); }
.te-var-row--radius .te-var-controls { width: 160px; }
.te-range-input {
  flex: 1;
  accent-color: var(--accent-primary);
  cursor: pointer;
}
.te-range-val {
  font-size: 11px;
  font-family: "JetBrains Mono", monospace;
  color: var(--text-secondary);
  min-width: 30px;
  text-align: right;
}

/* Saved themes */
.te-saved-list { padding: 6px 10px; display: flex; flex-direction: column; gap: 4px; }
.te-saved-item { display: flex; align-items: center; gap: 6px; }
.te-saved-load {
  flex: 1; text-align: left; background: none; border: none;
  color: var(--text-secondary); font-size: 12px; font-family: inherit;
  cursor: pointer; padding: 4px 8px; border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}
.te-saved-load:hover { background: var(--surface-glass); color: var(--text-primary); }
.te-saved-del {
  background: none; border: none; color: var(--text-muted);
  cursor: pointer; padding: 4px; border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}
.te-saved-del:hover { color: var(--danger); background: var(--danger-muted); }

/* Tabs */
.te-tabs {
  display: flex; gap: 4px; padding: 4px;
  background: var(--bg-secondary); border: 1px solid var(--border);
  border-radius: var(--radius-lg); flex-shrink: 0;
}
.te-tab {
  flex: 1; display: flex; align-items: center; justify-content: center; gap: 6px;
  padding: 8px 12px; border: none; border-radius: var(--radius-md);
  background: transparent; color: var(--text-muted); cursor: pointer;
  font-size: 12px; font-weight: 600; font-family: inherit;
  transition: all var(--transition-fast);
}
.te-tab:hover { color: var(--text-secondary); background: var(--bg-tertiary); }
.te-tab.active { background: var(--accent-primary); color: white; }

/* Segment buttons */
.te-btn-group { display: flex; gap: 2px; }
.te-seg-btn {
  display: inline-flex; align-items: center; gap: 4px;
  padding: 5px 10px; border: 1px solid var(--border); border-radius: var(--radius-sm);
  background: var(--bg-tertiary); color: var(--text-muted);
  font-size: 11px; font-family: inherit; cursor: pointer;
  transition: all var(--transition-fast);
}
.te-seg-btn:hover { color: var(--text-secondary); background: var(--bg-elevated); }
.te-seg-btn.active { background: var(--accent-muted); color: var(--accent-primary); border-color: rgba(249,115,22,0.4); font-weight: 600; }

/* Layout preset info */
.te-preset-emoji { font-size: 16px; flex-shrink: 0; line-height: 1; }
.te-preset-info { display: flex; flex-direction: column; gap: 1px; flex: 1; }
.te-preset-desc { font-size: 10px; color: var(--text-muted); }

/* Select */
.te-select {
  padding: 5px 10px; background: var(--bg-tertiary); border: 1px solid var(--border);
  border-radius: var(--radius-sm); color: var(--text-primary); font-size: 12px;
  font-family: inherit; cursor: pointer; outline: none;
}
.te-select:focus { border-color: var(--accent-primary); }

/* Preview panel */
.te-preview-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-xl);
  overflow: hidden;
  min-height: 0;
}
.te-preview-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
}
.te-preview-badge {
  margin-left: auto;
  padding: 1px 8px;
  border-radius: 99px;
  background: rgba(34, 197, 94, 0.15);
  color: var(--success);
  font-size: 9px;
  font-weight: 800;
  letter-spacing: 0.1em;
  animation: dot-pulse 2s ease-in-out infinite;
}
.te-preview-content {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  padding: 10px;
}

/* ── Navigation & Onglets section ──────────────────────────── */
.te-btn-group--wrap {
  flex-wrap: wrap;
  gap: 4px;
}

/* Preview mini tabs */
.te-nav-preview { flex-direction: column; align-items: flex-start; gap: 6px; }
.te-nav-preview .te-var-label { margin-bottom: 0; }
.te-tabs-preview {
  display: flex;
  gap: 2px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  padding: 4px;
  width: 100%;
  border-bottom: 1px solid var(--border);
}
.tp-tab {
  padding: 4px 10px;
  font-size: 11px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  border-radius: var(--radius-sm);
  cursor: default;
  font-family: inherit;
  transition: all 0.12s;
}
.tp-tab--active { color: var(--text-primary); font-weight: 600; }

/* Underline preview */
.te-tabs-preview[data-preview-tab="underline"] { padding: 4px 4px 0; border-bottom: 1px solid var(--border); }
.te-tabs-preview[data-preview-tab="underline"] .tp-tab { border-radius: 0; border-bottom: 2px solid transparent; margin-bottom: -1px; }
.te-tabs-preview[data-preview-tab="underline"] .tp-tab--active { border-bottom-color: var(--accent-primary); color: var(--accent-primary); }

/* Pills preview */
.te-tabs-preview[data-preview-tab="pills"] { border-bottom: none; border-radius: var(--radius-lg); gap: 4px; }
.te-tabs-preview[data-preview-tab="pills"] .tp-tab { border-radius: var(--radius-md); }
.te-tabs-preview[data-preview-tab="pills"] .tp-tab--active { background: var(--bg-elevated); color: var(--accent-primary); box-shadow: 0 1px 3px rgba(0,0,0,0.3); }

/* Cards preview */
.te-tabs-preview[data-preview-tab="cards"] { border-bottom: none; align-items: flex-end; padding: 4px 4px 0; }
.te-tabs-preview[data-preview-tab="cards"] .tp-tab { border: 1px solid var(--border); border-bottom: none; border-radius: var(--radius-sm) var(--radius-sm) 0 0; background: var(--bg-tertiary); }
.te-tabs-preview[data-preview-tab="cards"] .tp-tab--active { background: var(--bg-secondary); border-color: var(--border-hover); color: var(--accent-primary); box-shadow: inset 0 -2px 0 var(--accent-primary); }

/* Minimal preview */
.te-tabs-preview[data-preview-tab="minimal"] { border-bottom: none; padding: 2px; gap: 0; }
.te-tabs-preview[data-preview-tab="minimal"] .tp-tab { font-size: 10.5px; }
.te-tabs-preview[data-preview-tab="minimal"] .tp-tab--active { color: var(--text-primary); font-weight: 700; }

/* Bordered preview */
.te-tabs-preview[data-preview-tab="bordered"] { border-bottom: none; gap: 4px; }
.te-tabs-preview[data-preview-tab="bordered"] .tp-tab { border: 1px solid var(--border); border-radius: var(--radius-sm); }
.te-tabs-preview[data-preview-tab="bordered"] .tp-tab--active { border-color: var(--accent-primary); color: var(--accent-primary); background: rgba(249,115,22,0.06); }

/* Neon preview */
.te-tabs-preview[data-preview-tab="neon"] { padding: 4px 4px 0; border-bottom: 1px solid rgba(249,115,22,0.2); }
.te-tabs-preview[data-preview-tab="neon"] .tp-tab { border-radius: 0; border-bottom: 2px solid transparent; margin-bottom: -1px; }
.te-tabs-preview[data-preview-tab="neon"] .tp-tab--active { border-bottom-color: var(--accent-primary); color: var(--accent-primary); text-shadow: 0 0 8px rgba(249,115,22,0.7); }

/* Gradient preview */
.te-tabs-preview[data-preview-tab="gradient"] { border-bottom: none; gap: 4px; border-radius: var(--radius-lg); }
.te-tabs-preview[data-preview-tab="gradient"] .tp-tab { border-radius: var(--radius-md); }
.te-tabs-preview[data-preview-tab="gradient"] .tp-tab--active { background: linear-gradient(135deg, var(--accent-primary), #e05a00); color: #fff; box-shadow: 0 2px 8px rgba(249,115,22,0.4); }

/* Chip preview */
.te-tabs-preview[data-preview-tab="chip"] { border-bottom: none; gap: 5px; padding: 4px 2px; }
.te-tabs-preview[data-preview-tab="chip"] .tp-tab { border-radius: 999px; border: 1px solid var(--border); background: var(--bg-secondary); padding: 2px 8px 2px 6px; font-size: 10.5px; }
.te-tabs-preview[data-preview-tab="chip"] .tp-tab--active { border-color: var(--accent-primary); color: var(--accent-primary); background: rgba(249,115,22,0.1); }

/* Block preview */
.te-tabs-preview[data-preview-tab="block"] { border-bottom: none; border: 1px solid var(--border); border-radius: var(--radius-md); overflow: hidden; padding: 0; gap: 0; }
.te-tabs-preview[data-preview-tab="block"] .tp-tab { border-radius: 0; border-right: 1px solid var(--border); flex: 1; text-align: center; }
.te-tabs-preview[data-preview-tab="block"] .tp-tab:last-child { border-right: none; }
.te-tabs-preview[data-preview-tab="block"] .tp-tab--active { background: var(--accent-primary); color: #fff; }

/* Retro preview */
.te-tabs-preview[data-preview-tab="retro"] { padding: 4px 4px 0; border-bottom: 1px solid var(--accent-primary); font-family: monospace; gap: 4px; }
.te-tabs-preview[data-preview-tab="retro"] .tp-tab { border-radius: 0; border: 1px solid transparent; border-bottom: none; font-size: 10px; text-transform: uppercase; letter-spacing: 0.05em; font-family: monospace; }
.te-tabs-preview[data-preview-tab="retro"] .tp-tab--active { border-color: var(--accent-primary); border-bottom-color: var(--bg-tertiary); background: var(--bg-tertiary); color: var(--accent-primary); margin-bottom: -1px; }
</style>
