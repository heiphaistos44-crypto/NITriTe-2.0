import { defineStore } from "pinia";
import { ref, computed, watch } from "vue";

export type SidebarPosition = "left" | "right";
export type SidebarMode = "icons-only" | "icons-text";
export type SidebarWidth = "compact" | "normal" | "large";
export type UIDensity = "compact" | "normal" | "spacious";
export type ContentMaxWidth = "full" | "1400px" | "1200px" | "960px";

export type LayoutPresetId =
  | "default" | "minimal" | "compact" | "wide" | "developer"
  | "zen" | "focus" | "analyst" | "ultracompact" | "notebook"
  | "pro" | "studio" | "presentation" | "sysadmin" | "gaming"
  | "reading" | "dashboard" | "fullscreen" | "hybrid" | "editorial"
  | "command" | "widescreen" | "tablet" | "monitor4k" | "stream";

export interface LayoutState {
  sidebarPosition: SidebarPosition;
  sidebarWidth: SidebarWidth;
  sidebarMode: SidebarMode;
  headerVisible: boolean;
  density: UIDensity;
  contentMaxWidth: ContentMaxWidth;
  contentPadding: number;
  fontSize: number;
  activePreset: LayoutPresetId | "custom";
}

export interface LayoutPreset {
  id: LayoutPresetId;
  label: string;
  description: string;
  emoji: string;
  state: Omit<LayoutState, "activePreset">;
}

const SIDEBAR_WIDTHS: Record<SidebarWidth, number> = {
  compact: 48,
  normal: 240,
  large: 290,
};

export const LAYOUT_PRESETS: LayoutPreset[] = [
  // ── ESSENTIELS ──────────────────────────────────────────────────────
  {
    id: "default",
    label: "Défaut",
    description: "Mise en page équilibrée, usage général",
    emoji: "⚖️",
    state: {
      sidebarPosition: "left", sidebarWidth: "normal", sidebarMode: "icons-text",
      headerVisible: true, density: "normal", contentMaxWidth: "full",
      contentPadding: 24, fontSize: 13,
    },
  },
  {
    id: "minimal",
    label: "Minimal",
    description: "Sidebar icônes seules, focus sur le contenu",
    emoji: "🪶",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "normal", contentMaxWidth: "full",
      contentPadding: 20, fontSize: 13,
    },
  },
  {
    id: "compact",
    label: "Compact",
    description: "Maximum d'info à l'écran, densité élevée",
    emoji: "📦",
    state: {
      sidebarPosition: "left", sidebarWidth: "normal", sidebarMode: "icons-text",
      headerVisible: true, density: "compact", contentMaxWidth: "1400px",
      contentPadding: 14, fontSize: 12,
    },
  },
  {
    id: "wide",
    label: "Large",
    description: "Sidebar étendue, espacement généreux",
    emoji: "🖥️",
    state: {
      sidebarPosition: "left", sidebarWidth: "large", sidebarMode: "icons-text",
      headerVisible: true, density: "spacious", contentMaxWidth: "full",
      contentPadding: 32, fontSize: 14,
    },
  },
  {
    id: "developer",
    label: "Développeur",
    description: "Sidebar droite, sans header, compact",
    emoji: "🛠️",
    state: {
      sidebarPosition: "right", sidebarWidth: "normal", sidebarMode: "icons-text",
      headerVisible: false, density: "compact", contentMaxWidth: "full",
      contentPadding: 16, fontSize: 12,
    },
  },

  // ── PRODUCTIVITÉ ─────────────────────────────────────────────────────
  {
    id: "pro",
    label: "Pro",
    description: "Sidebar large, contenu 1400px, police 14px",
    emoji: "💼",
    state: {
      sidebarPosition: "left", sidebarWidth: "large", sidebarMode: "icons-text",
      headerVisible: true, density: "normal", contentMaxWidth: "1400px",
      contentPadding: 28, fontSize: 14,
    },
  },
  {
    id: "focus",
    label: "Focus",
    description: "Contenu centré 1200px, sidebar icônes, aéré",
    emoji: "🎯",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "spacious", contentMaxWidth: "1200px",
      contentPadding: 36, fontSize: 14,
    },
  },
  {
    id: "notebook",
    label: "Notebook",
    description: "Format document centré 960px, lecture confortable",
    emoji: "📓",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "spacious", contentMaxWidth: "960px",
      contentPadding: 40, fontSize: 15,
    },
  },
  {
    id: "presentation",
    label: "Présentation",
    description: "Plein écran, sans header, grand texte",
    emoji: "📽️",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "spacious", contentMaxWidth: "full",
      contentPadding: 48, fontSize: 16,
    },
  },

  // ── TECHNIQUE ────────────────────────────────────────────────────────
  {
    id: "analyst",
    label: "Analyste",
    description: "Sidebar droite icônes, contenu 1400px, compact",
    emoji: "📊",
    state: {
      sidebarPosition: "right", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "compact", contentMaxWidth: "1400px",
      contentPadding: 18, fontSize: 12,
    },
  },
  {
    id: "sysadmin",
    label: "Sysadmin",
    description: "Sidebar droite, sans header, ultra dense, mono 11px",
    emoji: "⚙️",
    state: {
      sidebarPosition: "right", sidebarWidth: "normal", sidebarMode: "icons-text",
      headerVisible: false, density: "compact", contentMaxWidth: "full",
      contentPadding: 12, fontSize: 11,
    },
  },
  {
    id: "ultracompact",
    label: "Ultra-Compact",
    description: "Densité maximale, tout visible, 11px",
    emoji: "🔬",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "compact", contentMaxWidth: "full",
      contentPadding: 10, fontSize: 11,
    },
  },

  // ── STYLE ────────────────────────────────────────────────────────────
  {
    id: "studio",
    label: "Studio",
    description: "Sidebar gauche icônes, plein écran, aéré",
    emoji: "🎨",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "spacious", contentMaxWidth: "full",
      contentPadding: 32, fontSize: 14,
    },
  },
  {
    id: "gaming",
    label: "Gaming",
    description: "Sidebar large gauche, grand texte, style immersif",
    emoji: "🎮",
    state: {
      sidebarPosition: "left", sidebarWidth: "large", sidebarMode: "icons-text",
      headerVisible: true, density: "spacious", contentMaxWidth: "full",
      contentPadding: 36, fontSize: 15,
    },
  },
  {
    id: "zen",
    label: "Zen",
    description: "Rien que l'essentiel, sidebar droite icônes, centré",
    emoji: "🧘",
    state: {
      sidebarPosition: "right", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "spacious", contentMaxWidth: "1200px",
      contentPadding: 48, fontSize: 15,
    },
  },

  // ── LECTURE & DOCUMENT ───────────────────────────────────────────────────
  {
    id: "reading",
    label: "Lecture",
    description: "Colonne étroite 800px, grande police, très aéré",
    emoji: "📖",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "spacious", contentMaxWidth: "960px",
      contentPadding: 56, fontSize: 16,
    },
  },
  {
    id: "editorial",
    label: "Éditorial",
    description: "Style magazine, 1200px, sidebar gauche icônes",
    emoji: "📰",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "spacious", contentMaxWidth: "1200px",
      contentPadding: 44, fontSize: 15,
    },
  },

  // ── PRODUCTIVITÉ AVANCÉE ─────────────────────────────────────────────────
  {
    id: "dashboard",
    label: "Dashboard",
    description: "Sidebar compacte, plein écran, compact, idéal tableaux",
    emoji: "📈",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "compact", contentMaxWidth: "full",
      contentPadding: 12, fontSize: 12,
    },
  },
  {
    id: "fullscreen",
    label: "Plein écran",
    description: "Aucune sidebar, aucun header, contenu max",
    emoji: "⛶",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "normal", contentMaxWidth: "full",
      contentPadding: 20, fontSize: 13,
    },
  },
  {
    id: "hybrid",
    label: "Hybride",
    description: "Sidebar icônes gauche + header, densité normale",
    emoji: "⚡",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "normal", contentMaxWidth: "full",
      contentPadding: 20, fontSize: 13,
    },
  },
  {
    id: "command",
    label: "Command",
    description: "Sidebar droite texte, sans header, compact, 12px mono",
    emoji: "💻",
    state: {
      sidebarPosition: "right", sidebarWidth: "normal", sidebarMode: "icons-text",
      headerVisible: false, density: "compact", contentMaxWidth: "full",
      contentPadding: 14, fontSize: 12,
    },
  },

  // ── ÉCRANS SPÉCIAUX ──────────────────────────────────────────────────────
  {
    id: "widescreen",
    label: "Ultrawide",
    description: "Optimisé 21:9, sidebar large, contenu 1400px",
    emoji: "🖥️",
    state: {
      sidebarPosition: "left", sidebarWidth: "large", sidebarMode: "icons-text",
      headerVisible: true, density: "normal", contentMaxWidth: "1400px",
      contentPadding: 40, fontSize: 14,
    },
  },
  {
    id: "tablet",
    label: "Tablette",
    description: "Compact, icônes seules, contenu 960px",
    emoji: "📱",
    state: {
      sidebarPosition: "left", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: true, density: "compact", contentMaxWidth: "960px",
      contentPadding: 16, fontSize: 13,
    },
  },
  {
    id: "monitor4k",
    label: "Moniteur 4K",
    description: "Grande police 15px, sidebar large, aéré, contenu large",
    emoji: "🖵",
    state: {
      sidebarPosition: "left", sidebarWidth: "large", sidebarMode: "icons-text",
      headerVisible: true, density: "spacious", contentMaxWidth: "full",
      contentPadding: 48, fontSize: 15,
    },
  },
  {
    id: "stream",
    label: "Stream / Obs",
    description: "Sidebar droite icônes, sans header, plein écran, 14px",
    emoji: "🎙️",
    state: {
      sidebarPosition: "right", sidebarWidth: "compact", sidebarMode: "icons-only",
      headerVisible: false, density: "normal", contentMaxWidth: "full",
      contentPadding: 24, fontSize: 14,
    },
  },
];

const STORAGE_KEY = "nitrite-layout";

const DEFAULT_STATE: LayoutState = {
  ...LAYOUT_PRESETS[0].state,
  activePreset: "default",
};

export const useLayoutStore = defineStore("layout", () => {
  const state = ref<LayoutState>({ ...DEFAULT_STATE });

  function load() {
    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed = JSON.parse(saved) as Partial<LayoutState>;
        state.value = { ...DEFAULT_STATE, ...parsed };
      }
    } catch { /* ignore */ }
    applyToDocument();
  }

  function save() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state.value));
  }

  function applyPreset(id: LayoutPresetId) {
    const preset = LAYOUT_PRESETS.find(p => p.id === id);
    if (!preset) return;
    state.value = { ...preset.state, activePreset: id };
    save();
    applyToDocument();
  }

  function setField<K extends keyof LayoutState>(key: K, value: LayoutState[K]) {
    (state.value as any)[key] = value;
    state.value.activePreset = "custom";
    save();
    applyToDocument();
  }

  function reset() {
    state.value = { ...DEFAULT_STATE };
    save();
    applyToDocument();
  }

  const sidebarWidthPx = computed(() => SIDEBAR_WIDTHS[state.value.sidebarWidth]);

  const cssVars = computed(() => {
    const sw = sidebarWidthPx.value;
    const densityFactor =
      state.value.density === "compact" ? 0.72 :
      state.value.density === "spacious" ? 1.4 : 1;
    return {
      "--layout-sidebar-width": `${sw}px`,
      "--layout-sidebar-collapsed-width": "64px",
      "--layout-sidebar-position": state.value.sidebarPosition,
      "--layout-header-height": state.value.headerVisible ? "52px" : "0px",
      "--layout-content-padding": `${state.value.contentPadding}px`,
      "--layout-content-max-width": state.value.contentMaxWidth === "full" ? "100%" : state.value.contentMaxWidth,
      "--layout-font-size": `${state.value.fontSize}px`,
      "--layout-density": `${densityFactor}`,
      "--layout-density-pad-sm": `${Math.round(4 * densityFactor)}px`,
      "--layout-density-pad-md": `${Math.round(8 * densityFactor)}px`,
      "--layout-density-pad-lg": `${Math.round(16 * densityFactor)}px`,
    };
  });

  function applyToDocument() {
    const vars = cssVars.value;
    const root = document.documentElement;
    for (const [k, v] of Object.entries(vars)) {
      root.style.setProperty(k, v as string);
    }
    document.body.style.fontSize = `${state.value.fontSize}px`;
  }

  watch(cssVars, applyToDocument, { deep: true });

  load();

  return { state, cssVars, sidebarWidthPx, applyPreset, setField, reset, load, applyToDocument };
});
