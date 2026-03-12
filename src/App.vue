<script setup lang="ts">
import { ref, computed, provide, onMounted } from "vue";
import logoUrl from "@/assets/nitrite-logo.jpg";
import AppSidebar from "@/components/layout/AppSidebar.vue";
import AppHeader from "@/components/layout/AppHeader.vue";
import AppStatusBar from "@/components/layout/AppStatusBar.vue";
import NToast from "@/components/ui/NToast.vue";
import SearchModal from "@/components/shared/SearchModal.vue";
import KeyboardShortcutsModal from "@/components/ui/KeyboardShortcutsModal.vue";
import { useAppStore } from "@/stores/app";
import { useLayoutStore } from "@/stores/layoutStore";

const appStore = useAppStore();
const layoutStore = useLayoutStore();

const sidebarCollapsed = ref(false);
const searchOpen = ref(false);
const shortcutsOpen = ref(false);
const appReady = ref(false);

function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value;
}

function openSearch() {
  searchOpen.value = true;
}

provide("sidebarCollapsed", sidebarCollapsed);
provide("toggleSidebar", toggleSidebar);
provide("openSearch", openSearch);

const isRightSidebar = computed(() => layoutStore.state.sidebarPosition === "right");
const currentSidebarWidth = computed(() =>
  sidebarCollapsed.value ? 64 : layoutStore.sidebarWidthPx
);

onMounted(async () => {
  appStore.loadSavedTheme();
  appStore.loadSidebarState();
  sidebarCollapsed.value = appStore.sidebarCollapsed;
  layoutStore.applyToDocument();

  window.addEventListener("keydown", (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      searchOpen.value = !searchOpen.value;
    }
    if ((e.ctrlKey || e.metaKey) && e.key === "b") {
      e.preventDefault();
      toggleSidebar();
      localStorage.setItem("nitrite-sidebar", String(sidebarCollapsed.value));
    }
    if (e.key === "?" && !e.ctrlKey && !e.metaKey && !e.altKey) {
      const tag = (e.target as HTMLElement)?.tagName;
      if (tag !== "INPUT" && tag !== "TEXTAREA" && tag !== "SELECT") {
        e.preventDefault();
        shortcutsOpen.value = !shortcutsOpen.value;
      }
    }
  });

  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("start_monitoring");
  } catch { /* Mode dev */ }

  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const { invoke } = await import("@tauri-apps/api/core");
    const win = getCurrentWindow();
    await win.listen("tauri://close-requested", async () => {
      try {
        await invoke("cleanup_on_exit");
      } catch {
        await win.destroy();
      }
    });
  } catch { /* Mode navigateur / dev */ }

  setTimeout(() => { appReady.value = true; }, 800);
});
</script>

<template>
  <!-- Splash Screen -->
  <Transition name="splash">
    <div v-if="!appReady" class="splash-screen">
      <div class="splash-content">
        <img :src="logoUrl" class="splash-logo" alt="NiTriTe" />
        <div class="splash-title">NiTriTe</div>
        <div class="splash-version">v26.29.0</div>
        <div class="splash-loader">
          <div class="splash-bar"></div>
        </div>
      </div>
    </div>
  </Transition>

  <!-- App -->
  <div
    v-if="appReady"
    class="app-layout"
    :class="[`sidebar-pos-${layoutStore.state.sidebarPosition}`, `density-${layoutStore.state.density}`]"
    :data-density="layoutStore.state.density"
    :data-sidebar-pos="layoutStore.state.sidebarPosition"
  >
    <AppSidebar
      :collapsed="sidebarCollapsed"
      :position="layoutStore.state.sidebarPosition"
      :width="layoutStore.state.sidebarWidth"
      :mode="layoutStore.state.sidebarMode"
      @toggle="toggleSidebar"
    />

    <div
      class="app-main"
      :class="{ 'sidebar-collapsed': sidebarCollapsed }"
      :style="{
        [isRightSidebar ? 'marginRight' : 'marginLeft']: `${currentSidebarWidth}px`,
        [isRightSidebar ? 'marginLeft' : 'marginRight']: '0',
      }"
    >
      <AppHeader v-if="layoutStore.state.headerVisible" @open-search="openSearch" />
      <main
        class="app-content"
        :style="{ padding: `${layoutStore.state.contentPadding}px` }"
      >
        <div
          class="app-content-inner"
          :style="{ maxWidth: layoutStore.state.contentMaxWidth === 'full' ? '100%' : layoutStore.state.contentMaxWidth, margin: '0 auto' }"
        >
          <router-view v-slot="{ Component }">
            <transition name="page" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </div>
      </main>
      <AppStatusBar />
    </div>
  </div>

  <NToast />
  <SearchModal v-model="searchOpen" />
  <KeyboardShortcutsModal v-model="shortcutsOpen" />
</template>

<style scoped>
/* Splash Screen */
.splash-screen {
  position: fixed; inset: 0; z-index: 99999;
  background: #09090b; display: flex; align-items: center; justify-content: center;
}
.splash-content {
  display: flex; flex-direction: column; align-items: center; gap: 12px;
  animation: splash-fade-in 400ms ease forwards;
}
.splash-logo {
  width: 96px; height: 96px; border-radius: 20px; object-fit: cover;
  flex-shrink: 0; display: block;
  box-shadow: 0 0 40px rgba(249, 115, 22, 0.4);
  animation: splash-bounce 600ms ease forwards;
}
@keyframes splash-bounce {
  0% { transform: scale(0.7); opacity: 0; }
  70% { transform: scale(1.08); }
  100% { transform: scale(1); opacity: 1; }
}
.splash-title {
  font-size: 30px; font-weight: 800;
  background: linear-gradient(135deg, #fafafa 40%, #f97316);
  -webkit-background-clip: text; -webkit-text-fill-color: transparent;
  background-clip: text; letter-spacing: -0.5px;
}
.splash-version {
  font-size: 12px; color: #a1a1aa; margin-top: -8px;
  font-family: "JetBrains Mono", monospace; letter-spacing: 0.05em;
}
.splash-loader { width: 120px; height: 3px; background: #27272a; border-radius: 2px; overflow: hidden; margin-top: 16px; }
.splash-bar {
  width: 40%; height: 100%;
  background: linear-gradient(90deg, #f97316, #fb923c);
  border-radius: 2px; animation: splash-progress 800ms ease-in-out forwards;
}
@keyframes splash-fade-in { from { opacity: 0; transform: scale(0.95); } to { opacity: 1; transform: scale(1); } }
@keyframes splash-progress { from { width: 0%; } to { width: 100%; } }
.splash-leave-active { transition: opacity 300ms ease; }
.splash-leave-to { opacity: 0; }

/* App Layout */
.app-layout {
  display: flex; height: 100vh; overflow: hidden;
  background: var(--bg-primary); color: var(--text-primary);
  animation: app-fade-in 300ms ease forwards;
  font-size: var(--layout-font-size, 13px);
}
@keyframes app-fade-in { from { opacity: 0; } to { opacity: 1; } }

/* Sidebar position right : flex-direction inversé */
.sidebar-pos-right { flex-direction: row-reverse; }

.app-main {
  flex: 1; display: flex; flex-direction: column;
  transition: margin var(--transition-normal);
  min-width: 0;
}

.app-content {
  flex: 1; overflow-y: auto; overflow-x: hidden;
}

.app-content-inner {
  width: 100%;
}

/* Transitions page */
.page-enter-active, .page-leave-active {
  transition: opacity 200ms ease, transform 200ms ease;
}
.page-enter-from { opacity: 0; transform: translateX(12px); }
.page-leave-to { opacity: 0; transform: translateX(-12px); }

/* Density global : affecte padding des composants */
:deep(.app-layout.density-compact) .ncard,
.density-compact :deep(.ncard) { padding: calc(var(--layout-density-pad-md, 8px) * 0.72); }

.density-spacious :deep(button.nav-item) { padding: 10px 12px; }
.density-compact :deep(button.nav-item) { padding: 5px 8px; font-size: 11px; }
</style>
