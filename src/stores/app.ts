import { defineStore } from "pinia";
import { ref, watch } from "vue";

export type ThemeName =
  | "nitrite-dark"
  | "cyber-blue"
  | "matrix-green"
  | "purple-haze"
  | "red-alert"
  | "arctic-light"
  | "midnight-gold"
  | "custom";

export const useAppStore = defineStore("app", () => {
  const theme = ref<ThemeName>("nitrite-dark");
  const sidebarCollapsed = ref(false);
  const language = ref<"fr" | "en">("fr");

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
    if (saved) {
      setTheme(saved);
    }
  }

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
    localStorage.setItem("nitrite-sidebar", String(sidebarCollapsed.value));
  }

  function loadSidebarState() {
    const saved = localStorage.getItem("nitrite-sidebar");
    if (saved === "true") {
      sidebarCollapsed.value = true;
    }
  }

  return {
    theme,
    sidebarCollapsed,
    language,
    setTheme,
    loadSavedTheme,
    toggleSidebar,
    loadSidebarState,
  };
});
