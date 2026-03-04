import { computed } from "vue";
import { useAppStore, type ThemeName } from "@/stores/app";

interface ThemeOption {
  name: ThemeName;
  label: string;
}

const themes: ThemeOption[] = [
  { name: "nitrite-dark", label: "Nitrite Dark" },
  { name: "cyber-blue", label: "Cyber Blue" },
  { name: "matrix-green", label: "Matrix Green" },
  { name: "purple-haze", label: "Purple Haze" },
  { name: "red-alert", label: "Red Alert" },
  { name: "arctic-light", label: "Arctic Light" },
  { name: "midnight-gold", label: "Midnight Gold" },
  { name: "custom", label: "Custom" },
];

export function useTheme() {
  const appStore = useAppStore();

  const currentTheme = computed(() => appStore.theme);
  const isDark = computed(() => appStore.theme !== "arctic-light");

  function setTheme(name: ThemeName) {
    appStore.setTheme(name);
  }

  return { currentTheme, themes, setTheme, isDark };
}
