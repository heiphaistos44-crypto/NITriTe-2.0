import { defineStore } from "pinia";
import { ref } from "vue";

export type ThemeName =
  | "nitrite-dark"
  | "cyber-blue"
  | "matrix-green"
  | "purple-haze"
  | "red-alert"
  | "arctic-light"
  | "midnight-gold"
  | "neon-synthwave"
  | "ocean-deep"
  | "rose-quartz"
  | "void-dark"
  | "forest-green"
  | "copper-rust"
  | "slate-steel"
  | "custom";

export const useAppStore = defineStore("app", () => {
  const theme = ref<ThemeName>("nitrite-dark");
  const sidebarCollapsed = ref(false);
  const language = ref<"fr" | "en">("fr");
  const fontSize = ref<"small" | "normal" | "large">("normal");
  const showAnimations = ref(true);

  function setTheme(name: ThemeName) {
    document.documentElement.classList.add("theme-transitioning");
    document.documentElement.setAttribute("data-theme", name);
    theme.value = name;
    localStorage.setItem("nitrite-theme", name);
    setTimeout(() => {
      document.documentElement.classList.remove("theme-transitioning");
    }, 350);
  }

  function loadSavedTheme() {
    const saved = localStorage.getItem("nitrite-theme") as ThemeName | null;
    if (saved) setTheme(saved);
  }

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
    localStorage.setItem("nitrite-sidebar", String(sidebarCollapsed.value));
  }

  function loadSidebarState() {
    const saved = localStorage.getItem("nitrite-sidebar");
    if (saved === "true") sidebarCollapsed.value = true;
  }

  function setFontSize(size: "small" | "normal" | "large") {
    fontSize.value = size;
    document.documentElement.setAttribute("data-font-size", size);
    localStorage.setItem("nitrite-font-size", size);
  }

  function loadFontSize() {
    const saved = localStorage.getItem("nitrite-font-size") as "small" | "normal" | "large" | null;
    if (saved) setFontSize(saved);
  }

  function toggleAnimations() {
    showAnimations.value = !showAnimations.value;
    if (showAnimations.value) {
      document.documentElement.classList.remove("no-animations");
    } else {
      document.documentElement.classList.add("no-animations");
    }
    localStorage.setItem("nitrite-animations", String(showAnimations.value));
  }

  function loadAnimations() {
    const saved = localStorage.getItem("nitrite-animations");
    if (saved === "false") {
      showAnimations.value = false;
      document.documentElement.classList.add("no-animations");
    }
  }

  return {
    theme,
    sidebarCollapsed,
    language,
    fontSize,
    showAnimations,
    setTheme,
    loadSavedTheme,
    toggleSidebar,
    loadSidebarState,
    setFontSize,
    loadFontSize,
    toggleAnimations,
    loadAnimations,
  };
});
