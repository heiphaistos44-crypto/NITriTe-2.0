<script setup lang="ts">
import { ref, computed, provide, onMounted, nextTick, watch } from "vue";
import logoUrl from "@/assets/nitrite-logo.jpg";
import AppSidebar from "@/components/layout/AppSidebar.vue";
import AppHeader from "@/components/layout/AppHeader.vue";
import AppStatusBar from "@/components/layout/AppStatusBar.vue";
import NToast from "@/components/ui/NToast.vue";
import SearchModal from "@/components/shared/SearchModal.vue";
import KeyboardShortcutsModal from "@/components/ui/KeyboardShortcutsModal.vue";
import { useAppStore } from "@/stores/app";
import { useLayoutStore } from "@/stores/layoutStore";
import { useDataCache } from "@/stores/dataCache";

const appStore    = useAppStore();
const layoutStore = useLayoutStore();
const dataCache   = useDataCache();

const sidebarCollapsed = ref(false);
const searchOpen       = ref(false);
const shortcutsOpen    = ref(false);
const appReady         = ref(false);
const videoMuted       = ref(false); // son activé par défaut
const splashVideo      = ref<HTMLVideoElement | null>(null);

// Sync direct de la propriété live .muted (l'attribut HTML ne suffit pas)
watch(videoMuted, (muted) => {
  if (splashVideo.value) splashVideo.value.muted = muted;
});

function toggleMute() {
  videoMuted.value = !videoMuted.value;
  // Double sécurité : forcer la propriété immédiatement (évite un cycle Vue)
  if (splashVideo.value) splashVideo.value.muted = videoMuted.value;
}

// ── Preloader ──────────────────────────────────────────────────────────────────
interface LoadTask { label: string; status: "pending" | "running" | "done" | "error" }

const loadTasks = ref<LoadTask[]>([
  { label: "Interface & thème",              status: "pending" },
  { label: "Monitoring système",             status: "pending" },
  { label: "Informations CPU",               status: "pending" },
  { label: "Informations mémoire RAM",       status: "pending" },
  { label: "Disques & volumes",              status: "pending" },
  { label: "Réseau & connexions",            status: "pending" },
  { label: "GPU & affichage",                status: "pending" },
  { label: "Comptes utilisateurs",           status: "pending" },
  { label: "Logiciels installés",            status: "pending" },
  { label: "Pilotes système",                status: "pending" },
  { label: "Processus actifs",               status: "pending" },
  { label: "Services Windows",               status: "pending" },
  { label: "Journaux d'événements",          status: "pending" },
  { label: "Pare-feu & règles réseau",       status: "pending" },
  { label: "Licence & activation",           status: "pending" },
  { label: "Historique BSOD",                status: "pending" },
  { label: "Points de restauration",         status: "pending" },
  { label: "Bluetooth & périphériques",      status: "pending" },
  { label: "Partages réseau",                status: "pending" },
  { label: "Certificats système",            status: "pending" },
  { label: "Tâches planifiées",              status: "pending" },
  { label: "Variables d'environnement",      status: "pending" },
  { label: "Informations BIOS & carte mère", status: "pending" },
  { label: "Assistant IA",                   status: "pending" },
]);

const doneCount    = computed(() => loadTasks.value.filter(t => t.status === "done" || t.status === "error").length);
const loadProgress = computed(() => Math.round((doneCount.value / loadTasks.value.length) * 100));
const currentLabel = computed(() => {
  const running = loadTasks.value.find(t => t.status === "running");
  return running?.label ?? (loadProgress.value === 100 ? "Prêt !" : "Chargement…");
});

// ── Layout ────────────────────────────────────────────────────────────────────
function toggleSidebar() { sidebarCollapsed.value = !sidebarCollapsed.value; }
function openSearch()    { searchOpen.value = true; }

provide("sidebarCollapsed", sidebarCollapsed);
provide("toggleSidebar",    toggleSidebar);
provide("openSearch",       openSearch);

const isRightSidebar      = computed(() => layoutStore.state.sidebarPosition === "right");
const currentSidebarWidth = computed(() => sidebarCollapsed.value ? 64 : layoutStore.sidebarWidthPx);

// ── Démarrage ─────────────────────────────────────────────────────────────────
onMounted(async () => {
  // Masquer le préloader HTML natif dès que Vue est prêt
  (window as any).__hideNativeBoot?.();

  // Démarrer la vidéo (WebView2 autorise l'autoplay avec son)
  await nextTick();
  if (splashVideo.value) {
    splashVideo.value.play().catch(() => {
      // Fallback silencieux si autoplay refusé : mute + retry
      videoMuted.value = true;
      splashVideo.value?.play().catch(() => {});
    });
  }

  // ── Tâche 0 : Interface (synchrone) ──
  loadTasks.value[0].status = "running";
  appStore.loadSavedTheme();
  appStore.loadSidebarState();
  sidebarCollapsed.value = appStore.sidebarCollapsed;
  layoutStore.applyToDocument();
  window.addEventListener("keydown", (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "k") { e.preventDefault(); searchOpen.value = !searchOpen.value; }
    if ((e.ctrlKey || e.metaKey) && e.key === "b") { e.preventDefault(); toggleSidebar(); localStorage.setItem("nitrite-sidebar", String(sidebarCollapsed.value)); }
    if (e.key === "?" && !e.ctrlKey && !e.metaKey && !e.altKey) {
      const tag = (e.target as HTMLElement)?.tagName;
      if (tag !== "INPUT" && tag !== "TEXTAREA" && tag !== "SELECT") { e.preventDefault(); shortcutsOpen.value = !shortcutsOpen.value; }
    }
  });
  loadTasks.value[0].status = "done";

  // ── Import Tauri ──
  let inv: ((cmd: string, args?: any) => Promise<any>) | null = null;
  try { const api = await import("@tauri-apps/api/core"); inv = api.invoke; } catch { /* dev */ }

  // ── Wrapper : marque la tâche, invoque, met en cache, marque done ──
  const load = async (idx: number, cmd: string, args?: any) => {
    loadTasks.value[idx].status = "running";
    if (inv) {
      try {
        const key   = args ? `${cmd}::${JSON.stringify(args)}` : cmd;
        const result = await inv(cmd, args);
        dataCache.set(key, result);
      } catch { /* non critique — la page re-fetchera si besoin */ }
      loadTasks.value[idx].status = "done";
    } else {
      await new Promise(r => setTimeout(r, 300));
      loadTasks.value[idx].status = "done";
    }
  };

  // ── Tâche 1 : Monitoring (doit démarrer en premier, fournit les events) ──
  await load(1, "start_monitoring");

  // ── Tâches 2-23 : 3 batches avec yields pour garder le thread libre ──────
  // Batch A — infos système de base (les plus rapides)
  await Promise.allSettled([
    load(2,  "get_system_info"),
    load(3,  "get_ram_detailed"),
    load(4,  "get_storage_physical_info"),
    load(5,  "get_network_overview"),
    load(6,  "get_gpu_detailed"),
    load(7,  "get_user_accounts"),
  ]);
  // Yield → permet aux clics/events utilisateur de passer
  await nextTick();

  // Batch B — données moyennement lourdes
  await Promise.allSettled([
    load(8,  "get_apps"),
    load(9,  "get_sys_drivers_list"),
    load(10, "get_running_processes"),
    load(11, "get_windows_services"),
    load(12, "get_event_logs", { logName: "System", count: 50 }),
    load(13, "get_firewall_rules"),
  ]);
  await nextTick();

  // Batch C — données secondaires / lentes
  await Promise.allSettled([
    load(14, "get_windows_license"),
    load(15, "get_bsod_history"),
    load(16, "list_restore_points_cmd"),
    load(17, "get_bluetooth_info"),
    load(18, "get_network_shares"),
    load(19, "get_certificates"),
    load(20, "get_scheduled_tasks"),
    load(21, "get_environment_variables"),
    load(22, "get_bios_info"),
    load(23, "ai_find_llamacpp_server"),
  ]);

  // ── Handler fermeture ──
  try {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const win = getCurrentWindow();
    await win.listen("tauri://close-requested", async () => {
      try { if (inv) await inv("cleanup_on_exit"); } catch { await win.destroy(); }
    });
  } catch { /* dev */ }

  // Transition vers l'app
  await new Promise(r => setTimeout(r, 400));
  appReady.value = true;
});
</script>

<template>
  <!-- ── Preloader ── -->
  <Transition name="splash">
    <div v-if="!appReady" class="splash-screen">

      <!-- ── Vidéo fond plein écran ── -->
      <video
        ref="splashVideo"
        class="splash-video"
        src="/splash.mp4"
        loop
        playsinline
        :muted="videoMuted"
      />

      <!-- ── Overlay dégradé ── -->
      <div class="splash-overlay" />

      <!-- ── Bouton mute (coin haut droit) ── -->
      <button
        class="splash-mute-btn"
        :title="videoMuted ? 'Activer le son' : 'Couper le son'"
        @click.stop.prevent="toggleMute"
      >
        <!-- Son actif -->
        <svg v-if="!videoMuted" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14"/>
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07"/>
        </svg>
        <!-- Son coupé -->
        <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
          <line x1="23" y1="9" x2="17" y2="15"/>
          <line x1="17" y1="9" x2="23" y2="15"/>
        </svg>
        <span>{{ videoMuted ? 'Son coupé' : 'Son activé' }}</span>
      </button>

      <!-- ── Panel de chargement (bas de l'écran) ── -->
      <div class="splash-content">

        <!-- Logo + titre -->
        <div class="splash-brand">
          <img :src="logoUrl" class="splash-logo" alt="NiTriTe" />
          <div class="splash-brand-text">
            <div class="splash-title">NiTriTe</div>
            <div class="splash-version">v26.41.0</div>
          </div>
        </div>

        <!-- Barre de progression -->
        <div class="splash-progress-wrap">
          <div class="splash-progress-bar">
            <div class="splash-progress-fill" :style="{ width: `${loadProgress}%` }" />
          </div>
          <div class="splash-pct">{{ loadProgress }}%</div>
        </div>

        <!-- Label tâche active -->
        <div class="splash-label">{{ currentLabel }}</div>

        <!-- Grille de tâches (compact) -->
        <div class="splash-grid">
          <div
            v-for="(task, i) in loadTasks"
            :key="i"
            class="splash-task"
            :class="task.status"
          >
            <span class="task-icon">
              <svg v-if="task.status === 'done'" width="10" height="10" viewBox="0 0 10 10">
                <circle cx="5" cy="5" r="5" fill="#22c55e" fill-opacity="0.2"/>
                <path d="M2.5 5l1.7 1.7L7.5 3.3" stroke="#22c55e" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
              </svg>
              <svg v-else-if="task.status === 'running'" class="spin" width="10" height="10" viewBox="0 0 10 10">
                <circle cx="5" cy="5" r="4" stroke="#f97316" stroke-width="1.5" stroke-dasharray="16" stroke-dashoffset="8" stroke-linecap="round" fill="none"/>
              </svg>
              <span v-else class="task-dot" />
            </span>
            <span class="task-label">{{ task.label }}</span>
          </div>
        </div>

        <!-- Compteur -->
        <div class="splash-counter">{{ doneCount }} / {{ loadTasks.length }} modules chargés</div>
      </div>

    </div>
  </Transition>

  <!-- ── Application ── -->
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
        [isRightSidebar ? 'marginLeft'  : 'marginRight']: '0',
      }"
    >
      <AppHeader v-if="layoutStore.state.headerVisible" @open-search="openSearch" />
      <main class="app-content" :style="{ padding: `${layoutStore.state.contentPadding}px` }">
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
/* ── Splash ───────────────────────────────────────────────────────────────── */
.splash-screen {
  position: fixed; inset: 0; z-index: 99999;
  background: #09090b; overflow: hidden;
}

/* Vidéo fond */
.splash-video {
  position: absolute; inset: 0;
  width: 100%; height: 100%;
  object-fit: cover;
  pointer-events: none;
}

/* Overlay dégradé — assombrit les bords pour lisibilité */
.splash-overlay {
  position: absolute; inset: 0;
  background:
    linear-gradient(to top,  rgba(9,9,11,0.92) 0%,  rgba(9,9,11,0.3) 40%, transparent 70%),
    linear-gradient(to bottom, rgba(9,9,11,0.5) 0%, transparent 25%),
    linear-gradient(to right, rgba(9,9,11,0.4) 0%, transparent 30%),
    linear-gradient(to left,  rgba(9,9,11,0.4) 0%, transparent 30%);
  pointer-events: none;
}

/* ── Bouton mute ── */
.splash-mute-btn {
  position: absolute; top: 16px; right: 16px; z-index: 10;
  display: flex; align-items: center; gap: 7px;
  padding: 8px 14px;
  background: rgba(9,9,11,0.65);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 99px;
  color: #e4e4e7; cursor: pointer;
  font-family: inherit; font-size: 12px; font-weight: 500;
  transition: all 200ms ease;
  letter-spacing: 0.02em;
}
.splash-mute-btn:hover {
  background: rgba(249,115,22,0.15);
  border-color: rgba(249,115,22,0.4);
  color: #f97316;
}
.splash-mute-btn svg { flex-shrink: 0; }

/* ── Panel chargement (bas de l'écran) ── */
.splash-content {
  position: absolute; bottom: 0; left: 0; right: 0;
  display: flex; flex-direction: column; align-items: center; gap: 8px;
  padding: 24px 32px 32px;
  animation: splash-in 400ms ease forwards;
}
@keyframes splash-in { from { opacity: 0; transform: translateY(12px); } to { opacity: 1; transform: translateY(0); } }

/* Logo + titre alignés en ligne */
.splash-brand {
  display: flex; align-items: center; gap: 14px; margin-bottom: 4px;
}
.splash-logo {
  width: 52px; height: 52px; border-radius: 12px; object-fit: cover;
  box-shadow: 0 0 24px rgba(249,115,22,0.5);
  animation: bounce 450ms ease forwards;
}
@keyframes bounce { 0% { transform:scale(0.7);opacity:0 } 70% { transform:scale(1.06) } 100% { transform:scale(1);opacity:1 } }

.splash-brand-text { display: flex; flex-direction: column; gap: 2px; }
.splash-title {
  font-size: 22px; font-weight: 800; letter-spacing: -0.5px;
  background: linear-gradient(135deg, #fafafa 40%, #f97316);
  -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
}
.splash-version {
  font-size: 10px; color: #52525b;
  font-family: "JetBrains Mono", monospace; letter-spacing: 0.05em;
}

/* Progress */
.splash-progress-wrap { display:flex; align-items:center; gap:8px; width:100%; max-width:520px; }
.splash-progress-bar  { flex:1; height:3px; background:rgba(255,255,255,0.1); border-radius:99px; overflow:hidden; }
.splash-progress-fill {
  height:100%; border-radius:99px;
  background: linear-gradient(90deg, #ea580c, #f97316, #fb923c);
  box-shadow: 0 0 10px rgba(249,115,22,0.6);
  transition: width 300ms cubic-bezier(0.4,0,0.2,1);
}
.splash-pct { font-size:11px; font-weight:700; color:#f97316; font-family:"JetBrains Mono",monospace; min-width:30px; text-align:right; }
.splash-label { font-size:11px; color:#71717a; min-height:15px; }

/* Grille de tâches */
.splash-grid {
  display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 2px 12px;
  width: 100%; max-width: 620px; margin-top: 4px;
  background: rgba(17,17,19,0.75);
  backdrop-filter: blur(16px);
  border: 1px solid rgba(255,255,255,0.07);
  border-radius: 10px;
  padding: 10px 14px;
}
.splash-task {
  display: flex; align-items: center; gap: 5px;
  opacity: 0.25; transition: opacity 200ms ease;
}
.splash-task.running { opacity: 1; }
.splash-task.done    { opacity: 0.55; }

.task-icon { width:12px; height:12px; display:flex; align-items:center; justify-content:center; flex-shrink:0; }
.task-dot  { width:4px; height:4px; border-radius:50%; background:#3f3f46; display:block; }
.task-label { font-size:10px; color:#a1a1aa; line-height:1.3; }
.splash-task.running .task-label { color:#f97316; font-weight:600; }
.splash-task.done    .task-label { color:#4ade80; }

.spin { animation: spin 0.9s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }

.splash-counter { font-size:10px; color:#52525b; font-family:"JetBrains Mono",monospace; }

.splash-leave-active { transition: opacity 500ms ease; }
.splash-leave-to { opacity: 0; }

/* ── App Layout ─────────────────────────────────────────────────────────── */
.app-layout {
  display:flex; height:100vh; overflow:hidden;
  background:var(--bg-primary); color:var(--text-primary);
  animation: app-in 350ms ease forwards;
  font-size: var(--layout-font-size, 13px);
}
@keyframes app-in { from { opacity:0 } to { opacity:1 } }
.sidebar-pos-right { flex-direction:row-reverse; }
.app-main { flex:1; display:flex; flex-direction:column; transition:margin var(--transition-normal); min-width:0; }
.app-content { flex:1; overflow-y:auto; overflow-x:hidden; }
.app-content-inner { width:100%; }

.page-enter-active, .page-leave-active { transition: opacity 180ms ease, transform 180ms ease; }
.page-enter-from { opacity:0; transform:translateX(10px); }
.page-leave-to   { opacity:0; transform:translateX(-10px); }

.density-compact  :deep(.ncard) { padding: calc(var(--layout-density-pad-md, 8px) * 0.72); }
.density-spacious :deep(button.nav-item) { padding: 10px 12px; }
.density-compact  :deep(button.nav-item) { padding: 5px 8px; font-size: 11px; }
</style>
