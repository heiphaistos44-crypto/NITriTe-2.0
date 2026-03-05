<template>
  <div class="diag-tab-content">
    <div class="diag-section-header">
      <Trash2 :size="16" /> Nettoyeur Système
    </div>

    <div style="display:flex;gap:8px;margin-bottom:12px;align-items:center">
      <button class="diag-btn diag-btn-primary" :disabled="loading || cleaning !== null" @click="loadTargets">
        <RefreshCw :size="13" /> Analyser
      </button>
      <button v-if="selectedTargets.size > 0" class="diag-btn" style="color:#ef4444;border-color:#ef4444"
        :disabled="cleaning !== null" @click="cleanSelected">
        <Trash2 :size="13" /> Nettoyer sélection ({{ selectedTargets.size }})
      </button>
      <span style="font-size:11px;opacity:.6">Total sélectionné : {{ totalSelectedMb.toFixed(1) }} MB</span>
    </div>

    <div v-if="loading" class="diag-loading">Analyse en cours...</div>

    <!-- Targets by category -->
    <div v-else-if="targets.length > 0">
      <div v-for="cat in categories" :key="cat" class="clean-category">
        <div class="clean-cat-header">{{ cat }}</div>
        <div class="clean-items">
          <div v-for="t in targetsByCategory(cat)" :key="t.name"
               class="clean-item" :class="{ 'clean-item-selected': selectedTargets.has(t.name), 'clean-item-done': results[t.name]?.success }">
            <label class="clean-checkbox">
              <input type="checkbox" :checked="selectedTargets.has(t.name)" @change="toggleTarget(t.name)" :disabled="cleaning === t.name" />
            </label>
            <div class="clean-item-info">
              <span class="clean-item-name">{{ t.name }}</span>
              <span class="clean-item-path" :title="t.path">{{ t.path || 'Corbeille' }}</span>
            </div>
            <div class="clean-item-size">
              <span :class="t.size_mb > 100 ? 'size-big' : t.size_mb > 10 ? 'size-mid' : ''">
                {{ t.size_mb.toFixed(1) }} MB
              </span>
              <span style="font-size:10px;opacity:.5">{{ t.file_count }} fichiers</span>
            </div>
            <div class="clean-item-action">
              <div v-if="cleaning === t.name" class="clean-spinner" />
              <span v-else-if="results[t.name]?.success" class="clean-ok">
                ✓ {{ results[t.name].freed_mb.toFixed(1) }}MB libérés
              </span>
              <button v-else class="diag-btn diag-btn-sm" :disabled="cleaning !== null" @click="cleanOne(t.name)">
                Nettoyer
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Large Files finder -->
    <div class="diag-section-header" style="margin-top:20px"><Search :size="16" /> Grands Fichiers</div>
    <div style="display:flex;gap:8px;align-items:center;margin-bottom:8px">
      <input v-model="largeFolder" class="diag-input" style="flex:1" placeholder="Dossier (ex: C:\Users)" />
      <input v-model.number="minSizeMb" type="number" class="diag-input" style="width:80px" placeholder="MB min" />
      <button class="diag-btn" :disabled="largLoading" @click="findLarge">Chercher</button>
    </div>
    <div v-if="largLoading" class="diag-loading">Recherche...</div>
    <div v-else-if="largeFiles.length > 0">
      <table class="diag-table">
        <thead><tr><th>Nom</th><th>Taille</th><th>Extension</th><th>Modifié</th><th>Chemin</th></tr></thead>
        <tbody>
          <tr v-for="f in largeFiles" :key="f.path">
            <td>{{ f.name }}</td>
            <td>{{ f.mb }} MB</td>
            <td>{{ f.ext }}</td>
            <td>{{ f.mod }}</td>
            <td style="font-size:10px;opacity:.6;max-width:200px;overflow:hidden;text-overflow:ellipsis">{{ f.path }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Trash2, RefreshCw, Search } from 'lucide-vue-next'

interface CleanTarget { name: string; path: string; size_mb: number; file_count: number; category: string }
interface CleanResult { target: string; freed_mb: number; files_deleted: number; success: boolean; message: string }
interface LargeFile { name: string; path: string; mb: number; ext: string; mod: string }

const targets = ref<CleanTarget[]>([])
const loading = ref(false)
const cleaning = ref<string | null>(null)
const selectedTargets = ref(new Set<string>())
const results = ref<Record<string, CleanResult>>({})
const largeFolder = ref('C:\\Users')
const minSizeMb = ref(100)
const largLoading = ref(false)
const largeFiles = ref<LargeFile[]>([])

const categories = computed(() => [...new Set(targets.value.map(t => t.category))])

const totalSelectedMb = computed(() =>
  targets.value.filter(t => selectedTargets.value.has(t.name)).reduce((s, t) => s + t.size_mb, 0)
)

function targetsByCategory(cat: string) {
  return targets.value.filter(t => t.category === cat)
}

async function loadTargets() {
  loading.value = true
  try {
    targets.value = await invoke<CleanTarget[]>('get_clean_targets')
  } finally {
    loading.value = false
  }
}

function toggleTarget(name: string) {
  if (selectedTargets.value.has(name)) selectedTargets.value.delete(name)
  else selectedTargets.value.add(name)
}

async function cleanOne(name: string) {
  cleaning.value = name
  try {
    const r = await invoke<CleanResult>('clean_target', { targetName: name })
    results.value[name] = r
    await loadTargets()
  } finally {
    cleaning.value = null
  }
}

async function cleanSelected() {
  for (const name of Array.from(selectedTargets.value)) {
    await cleanOne(name)
  }
  selectedTargets.value.clear()
}

async function findLarge() {
  largLoading.value = true
  largeFiles.value = []
  try {
    largeFiles.value = await invoke<LargeFile[]>('get_large_files', { folder: largeFolder.value, minSizeMb: minSizeMb.value })
  } finally {
    largLoading.value = false
  }
}

onMounted(loadTargets)
</script>

<style scoped>
.clean-category { margin-bottom: 16px; }
.clean-cat-header { font-size: 11px; font-weight: 600; text-transform: uppercase; opacity: .5; letter-spacing: .05em; margin-bottom: 6px; padding-bottom: 4px; border-bottom: 1px solid rgba(255,255,255,.07); }
.clean-items { display: flex; flex-direction: column; gap: 4px; }
.clean-item { display: flex; align-items: center; gap: 10px; padding: 8px 10px; border-radius: 6px; background: var(--bg-secondary, #1e1e2e); border: 1px solid transparent; transition: border-color .15s; }
.clean-item:hover { border-color: var(--border-color, #333); }
.clean-item-selected { border-color: var(--accent, #7c3aed) !important; background: rgba(124,58,237,.08) !important; }
.clean-item-done { opacity: .6; }
.clean-checkbox input { cursor: pointer; accent-color: var(--accent, #7c3aed); }
.clean-item-info { flex: 1; display: flex; flex-direction: column; min-width: 0; }
.clean-item-name { font-size: 13px; font-weight: 500; }
.clean-item-path { font-size: 10px; opacity: .5; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.clean-item-size { display: flex; flex-direction: column; align-items: flex-end; min-width: 70px; font-size: 12px; }
.size-big { color: #ef4444; font-weight: 700; }
.size-mid { color: #f59e0b; font-weight: 600; }
.clean-item-action { min-width: 110px; display: flex; justify-content: flex-end; align-items: center; }
.clean-spinner { width: 16px; height: 16px; border: 2px solid rgba(255,255,255,.2); border-top-color: var(--accent, #7c3aed); border-radius: 50%; animation: spin .8s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
.clean-ok { font-size: 11px; color: #22c55e; }
</style>
