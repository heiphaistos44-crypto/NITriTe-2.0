<script setup lang="ts">
import { ref, onMounted } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NInput from "@/components/ui/NInput.vue";
import NToggle from "@/components/ui/NToggle.vue";
import { useAppStore, type ThemeName } from "@/stores/app";
import { useNotificationStore } from "@/stores/notifications";
import {
  Settings, Palette, Globe, Bot, Activity,
  Save, RotateCcw, Info,
} from "lucide-vue-next";

const appStore = useAppStore();
const notify = useNotificationStore();

const ollamaUrl = ref("http://localhost:11434");
const ollamaModel = ref("llama3.2");
const monitorInterval = ref(1000);
const sidebarDefault = ref(false);
const saving = ref(false);

const themes: { id: ThemeName; label: string; color: string }[] = [
  { id: "nitrite-dark", label: "Nitrite Dark", color: "#f97316" },
  { id: "cyber-blue", label: "Cyber Blue", color: "#3b82f6" },
  { id: "matrix-green", label: "Matrix Green", color: "#22c55e" },
  { id: "purple-haze", label: "Purple Haze", color: "#a855f7" },
  { id: "red-alert", label: "Red Alert", color: "#ef4444" },
  { id: "arctic-light", label: "Arctic Light", color: "#0ea5e9" },
  { id: "midnight-gold", label: "Midnight Gold", color: "#eab308" },
  { id: "custom", label: "Custom", color: "#6b7280" },
];

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const config = await invoke<any>("get_config");
    ollamaUrl.value = config.ollama_url || "http://localhost:11434";
    ollamaModel.value = config.ollama_model || "llama3.2";
    monitorInterval.value = config.monitor_interval_ms || 1000;
    sidebarDefault.value = config.sidebar_collapsed || false;
  } catch {
    // Dev fallback
  }
});

async function saveSettings() {
  saving.value = true;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("save_config", {
      config: {
        theme: appStore.theme,
        language: appStore.language,
        sidebar_collapsed: sidebarDefault.value,
        ollama_url: ollamaUrl.value,
        ollama_model: ollamaModel.value,
        monitor_interval_ms: monitorInterval.value,
      },
    });
    notify.success("Parametres sauvegardes");
  } catch {
    notify.error("Erreur lors de la sauvegarde");
  }
  saving.value = false;
}

function resetDefaults() {
  appStore.setTheme("nitrite-dark");
  ollamaUrl.value = "http://localhost:11434";
  ollamaModel.value = "llama3.2";
  monitorInterval.value = 1000;
  sidebarDefault.value = false;
  notify.info("Parametres reinitialises");
}
</script>

<template>
  <div class="settings-page">
    <div class="page-header">
      <h1><Settings :size="22" /> Parametres</h1>
    </div>

    <!-- Theme -->
    <NCard>
      <template #header>
        <div class="card-header-row"><Palette :size="16" /><span>Theme</span></div>
      </template>
      <div class="theme-grid">
        <button
          v-for="t in themes"
          :key="t.id"
          class="theme-btn"
          :class="{ active: appStore.theme === t.id }"
          @click="appStore.setTheme(t.id)"
        >
          <div class="theme-swatch" :style="{ background: t.color }"></div>
          <span>{{ t.label }}</span>
        </button>
      </div>
    </NCard>

    <!-- Langue -->
    <NCard>
      <template #header>
        <div class="card-header-row"><Globe :size="16" /><span>Langue</span></div>
      </template>
      <div class="lang-row">
        <button
          class="lang-btn"
          :class="{ active: appStore.language === 'fr' }"
          @click="appStore.language = 'fr'"
        >Francais</button>
        <button
          class="lang-btn"
          :class="{ active: appStore.language === 'en' }"
          @click="appStore.language = 'en'"
        >English</button>
      </div>
    </NCard>

    <!-- Ollama -->
    <NCard>
      <template #header>
        <div class="card-header-row"><Bot :size="16" /><span>Ollama (IA)</span></div>
      </template>
      <div class="settings-form">
        <div class="form-row">
          <label>URL Ollama</label>
          <NInput v-model="ollamaUrl" placeholder="http://localhost:11434" />
        </div>
        <div class="form-row">
          <label>Modele par defaut</label>
          <NInput v-model="ollamaModel" placeholder="llama3.2" />
        </div>
      </div>
    </NCard>

    <!-- Monitoring -->
    <NCard>
      <template #header>
        <div class="card-header-row"><Activity :size="16" /><span>Monitoring</span></div>
      </template>
      <div class="settings-form">
        <div class="form-row">
          <label>Intervalle de rafraichissement</label>
          <div class="slider-row">
            <input
              type="range"
              min="500"
              max="5000"
              step="100"
              v-model.number="monitorInterval"
              class="slider"
            />
            <span class="slider-value">{{ monitorInterval }}ms</span>
          </div>
        </div>
        <div class="form-row">
          <label>Sidebar repliee par defaut</label>
          <NToggle v-model="sidebarDefault" label="" />
        </div>
      </div>
    </NCard>

    <!-- Actions -->
    <div class="settings-actions">
      <NButton variant="primary" :loading="saving" @click="saveSettings">
        <Save :size="14" /> Sauvegarder
      </NButton>
      <NButton variant="ghost" @click="resetDefaults">
        <RotateCcw :size="14" /> Reinitialiser
      </NButton>
    </div>

    <!-- Info -->
    <NCard>
      <template #header>
        <div class="card-header-row"><Info :size="16" /><span>A propos</span></div>
      </template>
      <div class="about-info">
        <div class="about-row"><span>Application</span><span>NiTriTe</span></div>
        <div class="about-row"><span>Version</span><span>26.0</span></div>
        <div class="about-row"><span>Stack</span><span>Tauri v2 + Rust + Vue 3</span></div>
        <div class="about-row"><span>Auteur</span><span>Momo</span></div>
      </div>
    </NCard>
  </div>
</template>

<style scoped>
.settings-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.page-header h1 {
  font-size: 22px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 10px;
}
.card-header-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
.theme-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}
@media (max-width: 800px) {
  .theme-grid { grid-template-columns: repeat(2, 1fr); }
}
.theme-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border: 2px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  color: var(--text-primary);
  transition: all 0.2s;
}
.theme-btn:hover { border-color: var(--text-muted); }
.theme-btn.active { border-color: var(--accent-primary); background: var(--bg-secondary); }
.theme-swatch {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  flex-shrink: 0;
}
.lang-row {
  display: flex;
  gap: 10px;
}
.lang-btn {
  padding: 8px 20px;
  border: 2px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  color: var(--text-primary);
  transition: all 0.2s;
}
.lang-btn:hover { border-color: var(--text-muted); }
.lang-btn.active { border-color: var(--accent-primary); background: var(--bg-secondary); }
.settings-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.form-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.form-row label {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 500;
}
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
  font-size: 13px;
  color: var(--accent-primary);
  min-width: 60px;
}
.settings-actions {
  display: flex;
  gap: 10px;
}
.about-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.about-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  padding: 6px 0;
  border-bottom: 1px solid var(--border);
}
.about-row span:first-child { color: var(--text-muted); }
.about-row span:last-child { color: var(--text-primary); font-weight: 500; }
</style>
