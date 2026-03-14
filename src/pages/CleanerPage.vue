<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NSkeleton from "@/components/ui/NSkeleton.vue";
import { useNotificationStore } from "@/stores/notifications";
import { Trash2, RefreshCw, Search, FileText, CheckSquare, Square, HardDrive, Zap } from "lucide-vue-next";

const notify = useNotificationStore();

interface CleanTarget { name: string; path: string; size_mb: number; file_count: number; category: string; }
interface LargeFile { name: string; path: string; mb: number; ext: string; mod: string; }
interface CleanResult { target: string; success: boolean; freed_mb: number; files_deleted: number; message: string; }
interface CleanerProgress { scanned: number; total: number; item: CleanTarget | null; done: boolean; }

const targets    = ref<CleanTarget[]>([]);
const largeFiles = ref<LargeFile[]>([]);

// État de scan streaming
const scanProgress = ref(0);
const scanTotal    = ref(12);
const scanning     = ref(false);

const loadingLarge = ref(false);
const largeScanned = ref(false); // true après premier scan manuel
const selected     = ref<Set<string>>(new Set());
const cleaning     = ref(false);
const cleanProgress = ref(0);
const totalFreed   = ref(0);
const largeMinMb   = ref(100);

let unlistenCleaner: (() => void) | null = null;

const grouped = computed(() => {
  const groups: Record<string, CleanTarget[]> = {};
  for (const t of targets.value) {
    if (!groups[t.category]) groups[t.category] = [];
    groups[t.category].push(t);
  }
  return groups;
});

const selectedSizeMb = computed(() =>
  targets.value.filter(t => selected.value.has(t.name)).reduce((s, t) => s + t.size_mb, 0)
);

function formatMb(mb: number) {
  if (mb < 1) return `${Math.round(mb * 1024)} Ko`;
  if (mb < 1024) return `${mb.toFixed(1)} Mo`;
  return `${(mb / 1024).toFixed(2)} Go`;
}

async function load() {
  // Reset état
  targets.value = [];
  selected.value = new Set();
  scanning.value = true;
  scanProgress.value = 0;

  // Écouter les événements streaming
  if (unlistenCleaner) { unlistenCleaner(); unlistenCleaner = null; }
  unlistenCleaner = await listen<CleanerProgress>("cleaner:progress", (ev) => {
    const p = ev.payload;
    scanProgress.value = p.scanned;
    scanTotal.value = p.total;
    if (p.item) {
      targets.value.push(p.item);
      // Auto-sélection des cibles sûres
      if (["Temp", "Logs", "Cache système"].includes(p.item.category))
        selected.value = new Set([...selected.value, p.item.name]);
    }
    if (p.done) scanning.value = false;
  });

  // Lancer le scan en fire-and-forget (non-bloquant → UI reste fluide)
  invoke("scan_clean_targets_stream").catch(() => { scanning.value = false; });
}

async function loadLargeFiles() {
  loadingLarge.value = true;
  largeScanned.value = true;
  try {
    largeFiles.value = await invoke<LargeFile[]>("get_large_files", {
      folder: "C:\\Users",
      minSizeMb: largeMinMb.value,
    });
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  loadingLarge.value = false;
}

async function clean() {
  const names = [...selected.value];
  if (!names.length) return;
  cleaning.value = true; cleanProgress.value = 0; totalFreed.value = 0;
  for (let i = 0; i < names.length; i++) {
    try {
      const r = await invoke<CleanResult>("clean_target", { targetName: names[i] });
      if (r.success) totalFreed.value += r.freed_mb;
    } catch {}
    cleanProgress.value = Math.round(((i + 1) / names.length) * 100);
  }
  notify.success("Nettoyage terminé", `${formatMb(totalFreed.value)} libérés`);
  cleaning.value = false;
  load(); // Rescan (fire-and-forget)
}

onMounted(() => { load(); });
onUnmounted(() => { if (unlistenCleaner) unlistenCleaner(); });
</script>

<template>
  <div class="cleaner-page">
    <div class="page-header">
      <div class="header-icon"><Trash2 :size="22" /></div>
      <div>
        <h1>Nettoyeur Avancé</h1>
        <p class="subtitle">Libérez de l'espace disque en supprimant les fichiers inutiles</p>
      </div>
      <NButton variant="ghost" size="sm" :loading="scanning" @click="load" style="margin-left:auto">
        <RefreshCw :size="13" /> Analyser
      </NButton>
    </div>

    <!-- Barre d'action -->
    <div class="action-bar">
      <div class="space-info">
        <HardDrive :size="14" />
        <span v-if="selected.size">
          <strong>{{ formatMb(selectedSizeMb) }}</strong> sélectionnés
        </span>
        <span v-if="totalFreed > 0" class="freed-badge">
          <Zap :size="12" /> {{ formatMb(totalFreed) }} libérés
        </span>
      </div>
      <div style="display:flex;gap:8px">
        <NButton variant="ghost" size="sm" @click="selected = new Set(targets.map(t => t.name))">Tout</NButton>
        <NButton variant="ghost" size="sm" @click="selected = new Set()">Aucun</NButton>
        <NButton
          variant="danger" size="sm"
          :loading="cleaning"
          :disabled="selected.size === 0 || cleaning"
          @click="clean"
        >
          <Trash2 :size="13" /> Nettoyer ({{ selected.size }})
        </NButton>
      </div>
    </div>

    <!-- Progression nettoyage -->
    <NProgress v-if="cleaning" :value="cleanProgress" showLabel size="sm" />

    <!-- Barre de progression du scan — ne bloque pas l'UI -->
    <div v-if="scanning" class="scan-progress-bar">
      <div class="scan-label">
        <Search :size="13" style="color:var(--accent-primary)" />
        <span>Analyse en cours… {{ scanProgress }}/{{ scanTotal }}</span>
      </div>
      <NProgress :value="scanProgress" :max="scanTotal" size="sm" style="flex:1" />
    </div>

    <!-- Cibles groupées par catégorie -->
    <div class="targets-sections">
      <NCard v-for="(items, cat) in grouped" :key="cat">
        <template #header>
          <div class="section-header">
            <span class="cat-label">{{ cat }}</span>
            <NBadge variant="neutral">{{ items.length }}</NBadge>
            <span class="cat-size">{{ formatMb(items.reduce((s, i) => s + i.size_mb, 0)) }}</span>
          </div>
        </template>
        <div class="targets-list">
          <button
            v-for="t in items" :key="t.name"
            class="target-row"
            :class="{ checked: selected.has(t.name) }"
            @click="selected.has(t.name) ? selected.delete(t.name) : selected.add(t.name); selected = new Set(selected)"
          >
            <component :is="selected.has(t.name) ? CheckSquare : Square" :size="15" class="check-ic" />
            <div class="target-info">
              <span class="target-label">{{ t.name }}</span>
              <span class="target-desc">{{ t.path || '—' }}</span>
            </div>
            <div class="target-meta">
              <span class="target-size">{{ formatMb(t.size_mb) }}</span>
              <NBadge v-if="t.file_count > 0" variant="neutral" size="sm">{{ t.file_count }} fichiers</NBadge>
            </div>
          </button>
        </div>
      </NCard>
    </div>

    <!-- Gros fichiers — scan manuel uniquement (peut prendre 1-5 min sur C:\Users) -->
    <NCard>
      <template #header>
        <div class="section-header">
          <Search :size="15" /><span>Fichiers volumineux</span>
          <div style="margin-left:auto;display:flex;align-items:center;gap:8px">
            <span style="font-size:11px;color:var(--text-muted)">Min :</span>
            <select v-model="largeMinMb" class="mini-select">
              <option :value="50">50 Mo</option>
              <option :value="100">100 Mo</option>
              <option :value="500">500 Mo</option>
              <option :value="1000">1 Go</option>
            </select>
            <NButton variant="primary" size="sm" :loading="loadingLarge" @click="loadLargeFiles">
              <Search :size="12" /> Analyser
            </NButton>
          </div>
        </div>
      </template>
      <!-- Avant premier scan : invite à lancer manuellement -->
      <div v-if="!largeScanned && !loadingLarge" class="large-not-scanned">
        <Search :size="28" style="opacity:.2" />
        <p>Cliquez sur <strong>Analyser</strong> pour rechercher les fichiers volumineux dans <code>C:\Users</code></p>
        <p class="large-warning">Cette analyse peut prendre 1 à 5 minutes. Vous pouvez continuer à utiliser l'application pendant ce temps.</p>
      </div>
      <div v-else-if="loadingLarge" class="loading-state">
        <div class="large-scan-info">
          <Search :size="16" style="color:var(--accent-primary)" />
          <span>Scan de C:\Users en cours… Vous pouvez naviguer librement.</span>
        </div>
        <NSkeleton v-for="i in 5" :key="i" height="36px" style="margin-bottom:4px" />
      </div>
      <div v-else-if="!largeFiles.length" class="empty-state"><FileText :size="24" style="opacity:.25" /><p>Aucun fichier > {{ largeMinMb }} Mo trouvé</p></div>
      <div v-else class="large-files-list">
        <div v-for="f in largeFiles" :key="f.path" class="large-file-row">
          <span class="file-ext">{{ f.ext || '—' }}</span>
          <span class="file-path">{{ f.path }}</span>
          <span class="file-size">{{ formatMb(f.mb) }}</span>
          <NButton variant="ghost" size="sm" @click="invoke('open_path', { path: f.path.split('\\').slice(0,-1).join('\\') })">Ouvrir</NButton>
        </div>
      </div>
    </NCard>
  </div>
</template>

<style scoped>
.cleaner-page { display: flex; flex-direction: column; gap: 14px; }
.page-header { display: flex; align-items: center; gap: 12px; }
.header-icon { width: 42px; height: 42px; border-radius: var(--radius-lg); background: var(--danger-muted); display: flex; align-items: center; justify-content: center; color: var(--danger); flex-shrink: 0; }
h1 { font-size: 22px; font-weight: 700; }
.subtitle { font-size: 12px; color: var(--text-muted); }
.action-bar { display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 10px 14px; background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-md); }
.space-info { display: flex; align-items: center; gap: 8px; font-size: 13px; color: var(--text-secondary); }
.freed-badge { display: flex; align-items: center; gap: 4px; color: var(--success); font-weight: 700; }
.section-header { display: flex; align-items: center; gap: 8px; width: 100%; }
.cat-label { font-size: 13px; font-weight: 700; color: var(--text-primary); }
.cat-size { margin-left: auto; font-size: 11px; color: var(--text-muted); font-family: "JetBrains Mono", monospace; }
.targets-list { display: flex; flex-direction: column; gap: 2px; }
.target-row { display: flex; align-items: center; gap: 10px; padding: 9px 10px; border: none; border-radius: var(--radius-md); background: transparent; cursor: pointer; font-family: inherit; text-align: left; width: 100%; transition: background var(--transition-fast); }
.target-row:hover { background: var(--bg-tertiary); }
.target-row.checked .check-ic { color: var(--accent-primary); }
.check-ic { color: var(--text-muted); flex-shrink: 0; }
.target-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.target-label { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.target-desc { font-size: 11px; color: var(--text-muted); }
.target-meta { display: flex; align-items: center; gap: 6px; flex-shrink: 0; }
.target-size { font-family: "JetBrains Mono", monospace; font-size: 12px; color: var(--text-secondary); }
.mini-select { padding: 4px 8px; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); font-size: 12px; cursor: pointer; }
.large-files-list { display: flex; flex-direction: column; gap: 2px; }
.large-file-row { display: grid; grid-template-columns: 60px 1fr 80px auto; align-items: center; gap: 10px; padding: 7px 10px; border-bottom: 1px solid var(--border); font-size: 12px; }
.large-file-row:last-child { border-bottom: none; }
.file-ext { font-family: monospace; background: var(--bg-elevated); border-radius: var(--radius-sm); padding: 2px 6px; font-size: 10px; font-weight: 700; color: var(--text-muted); text-align: center; }
.file-path { color: var(--text-secondary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-family: "JetBrains Mono", monospace; font-size: 11px; }
.file-size { font-family: "JetBrains Mono", monospace; color: var(--danger); font-weight: 700; text-align: right; }
.loading-state, .empty-state { display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 30px; color: var(--text-muted); font-size: 13px; }
.scan-progress-bar { display: flex; align-items: center; gap: 10px; padding: 8px 12px; background: color-mix(in srgb,var(--accent-primary) 8%,var(--bg-secondary)); border: 1px solid color-mix(in srgb,var(--accent-primary) 30%,var(--border)); border-radius: var(--radius-md); }
.scan-label { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--accent-primary); white-space: nowrap; font-weight: 500; }
.large-not-scanned { display: flex; flex-direction: column; align-items: center; gap: 10px; padding: 32px 20px; color: var(--text-muted); font-size: 13px; text-align: center; }
.large-not-scanned code { color: var(--accent-primary); background: var(--bg-tertiary); padding: 1px 6px; border-radius: 4px; font-size: 12px; }
.large-warning { font-size: 11px; color: color-mix(in srgb,var(--success) 80%,var(--text-muted)); background: var(--success-muted); padding: 6px 12px; border-radius: 6px; }
.large-scan-info { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--accent-primary); margin-bottom: 8px; }
</style>
