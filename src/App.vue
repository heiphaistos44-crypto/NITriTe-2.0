<script setup lang="ts">
import { ref, provide, onMounted } from "vue";
import logoUrl from "@/assets/nitrite-logo.jpg";
import AppSidebar from "@/components/layout/AppSidebar.vue";
import AppHeader from "@/components/layout/AppHeader.vue";
import AppStatusBar from "@/components/layout/AppStatusBar.vue";
import NToast from "@/components/ui/NToast.vue";
import SearchModal from "@/components/shared/SearchModal.vue";
import { useAppStore } from "@/stores/app";

const appStore = useAppStore();
const sidebarCollapsed = ref(false);
const searchOpen = ref(false);
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

onMounted(async () => {
  appStore.loadSavedTheme();
  appStore.loadSidebarState();
  sidebarCollapsed.value = appStore.sidebarCollapsed;

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
  });

  // Demarrer le monitoring global pour la status bar
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("start_monitoring");
  } catch {
    // Mode dev
  }

  // Splash screen minimum 800ms
  setTimeout(() => {
    appReady.value = true;
  }, 800);
});
</script>

<template>
  <!-- Splash Screen -->
  <Transition name="splash">
    <div v-if="!appReady" class="splash-screen">
      <div class="splash-content">
        <img :src="logoUrl" class="splash-logo" alt="NiTriTe" />
        <div class="splash-title">NiTriTe</div>
        <div class="splash-version">v26.0</div>
        <div class="splash-loader">
          <div class="splash-bar"></div>
        </div>
      </div>
    </div>
  </Transition>

  <!-- App -->
  <div v-if="appReady" class="app-layout">
    <AppSidebar :collapsed="sidebarCollapsed" @toggle="toggleSidebar" />
    <div class="app-main" :class="{ 'sidebar-collapsed': sidebarCollapsed }">
      <AppHeader @open-search="openSearch" />
      <main class="app-content">
        <router-view v-slot="{ Component }">
          <transition name="page" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
      <AppStatusBar />
    </div>
  </div>
  <NToast />
  <SearchModal v-model="searchOpen" />
</template>

<style scoped>
/* Splash Screen */
.splash-screen {
  position: fixed;
  inset: 0;
  z-index: 99999;
  background: #09090b;
  display: flex;
  align-items: center;
  justify-content: center;
}

.splash-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  animation: splash-fade-in 400ms ease forwards;
}

.splash-logo {
  width: 96px;
  height: 96px;
  border-radius: 20px;
  object-fit: cover;
  box-shadow: 0 0 40px rgba(249, 115, 22, 0.4);
  animation: splash-bounce 600ms ease forwards;
}

@keyframes splash-bounce {
  0% { transform: scale(0.7); opacity: 0; }
  70% { transform: scale(1.08); }
  100% { transform: scale(1); opacity: 1; }
}

.splash-title {
  font-size: 28px;
  font-weight: 700;
  color: #fafafa;
  letter-spacing: -0.5px;
}

.splash-version {
  font-size: 13px;
  color: #71717a;
  margin-top: -8px;
}

.splash-loader {
  width: 120px;
  height: 3px;
  background: #27272a;
  border-radius: 2px;
  overflow: hidden;
  margin-top: 16px;
}

.splash-bar {
  width: 40%;
  height: 100%;
  background: linear-gradient(90deg, #f97316, #fb923c);
  border-radius: 2px;
  animation: splash-progress 800ms ease-in-out forwards;
}

@keyframes splash-fade-in {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

@keyframes splash-progress {
  from { width: 0%; }
  to { width: 100%; }
}

.splash-leave-active {
  transition: opacity 300ms ease;
}
.splash-leave-to {
  opacity: 0;
}

/* App Layout */
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-primary);
  color: var(--text-primary);
  animation: app-fade-in 300ms ease forwards;
}

@keyframes app-fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

.app-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-left: 240px;
  transition: margin-left var(--transition-normal);
  min-width: 0;
}

.app-main.sidebar-collapsed {
  margin-left: 64px;
}

.app-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px;
}

.page-enter-active,
.page-leave-active {
  transition: opacity 200ms ease, transform 200ms ease;
}

.page-enter-from {
  opacity: 0;
  transform: translateX(12px);
}

.page-leave-to {
  opacity: 0;
  transform: translateX(-12px);
}
</style>
