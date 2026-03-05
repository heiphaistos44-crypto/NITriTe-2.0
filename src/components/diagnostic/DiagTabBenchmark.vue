<template>
  <div class="diag-tab-content">
    <div class="diag-section-header">
      <Gauge :size="16" /> Benchmark Système
    </div>

    <div class="bench-grid">
      <!-- CPU -->
      <div class="bench-card">
        <div class="bench-card-title"><Cpu :size="14" /> CPU Single-thread</div>
        <div v-if="cpu.loading" class="bench-running">
          <div class="bench-spinner" /> Calcul en cours...
        </div>
        <div v-else-if="cpu.score !== null" class="bench-result">
          <div class="bench-score">{{ cpu.score.toFixed(1) }} <span class="bench-unit">Kops/s</span></div>
          <div class="bench-detail">{{ cpu.details }}</div>
          <div class="bench-time">{{ cpu.duration_ms }}ms</div>
          <div class="bench-bar-wrap">
            <div class="bench-bar" :style="{width: Math.min(cpu.score/200*100,100)+'%', background: scoreColor(cpu.score,200)}" />
          </div>
        </div>
        <div v-else class="bench-idle">En attente</div>
        <button class="diag-btn" :disabled="anyRunning" @click="runCpu">Lancer</button>
      </div>

      <!-- RAM -->
      <div class="bench-card">
        <div class="bench-card-title"><MemoryStick :size="14" /> RAM Write (64MB)</div>
        <div v-if="ram.loading" class="bench-running"><div class="bench-spinner" /> Test RAM...</div>
        <div v-else-if="ram.score !== null" class="bench-result">
          <div class="bench-score">{{ ram.score.toFixed(2) }} <span class="bench-unit">GB/s</span></div>
          <div class="bench-detail">{{ ram.details }}</div>
          <div class="bench-time">{{ ram.duration_ms }}ms</div>
          <div class="bench-bar-wrap">
            <div class="bench-bar" :style="{width: Math.min(ram.score/50*100,100)+'%', background: scoreColor(ram.score,50)}" />
          </div>
        </div>
        <div v-else class="bench-idle">En attente</div>
        <button class="diag-btn" :disabled="anyRunning" @click="runRam">Lancer</button>
      </div>

      <!-- Disk -->
      <div class="bench-card bench-card-wide">
        <div class="bench-card-title"><HardDrive :size="14" /> Disque</div>
        <div class="bench-disk-drive">
          <label>Lecteur :</label>
          <input v-model="drive" class="diag-input" style="width:80px" placeholder="C:" />
        </div>
        <div v-if="disk.loading" class="bench-running"><div class="bench-spinner" /> Test disque (peut prendre 30s)...</div>
        <div v-else-if="disk.results.length > 0" class="bench-disk-results">
          <div v-for="r in disk.results" :key="r.name" class="bench-disk-row">
            <span class="bench-disk-name">{{ r.name }}</span>
            <span class="bench-score-sm">{{ r.score.toFixed(0) }} {{ r.unit }}</span>
            <div class="bench-bar-wrap" style="flex:1">
              <div class="bench-bar" :style="{width: diskBarPct(r)+'%', background: diskColor(r)}" />
            </div>
          </div>
        </div>
        <div v-else class="bench-idle">En attente</div>
        <button class="diag-btn" :disabled="anyRunning" @click="runDisk">Lancer</button>
      </div>
    </div>

    <!-- Run All -->
    <div style="margin-top:16px;display:flex;gap:8px;align-items:center">
      <button class="diag-btn diag-btn-primary" :disabled="anyRunning" @click="runAll">
        <Play :size="13" /> Tout tester
      </button>
      <button class="diag-btn" :disabled="anyRunning" @click="reset">Réinitialiser</button>
      <span v-if="anyRunning" style="font-size:11px;opacity:.7">Test en cours...</span>
    </div>

    <!-- Summary -->
    <div v-if="hasResults" class="bench-summary">
      <div class="diag-section-header" style="margin-top:20px">Résumé</div>
      <table class="diag-table">
        <thead><tr><th>Test</th><th>Score</th><th>Unité</th><th>Durée</th></tr></thead>
        <tbody>
          <tr v-if="cpu.score !== null"><td>CPU Single-thread</td><td>{{ cpu.score.toFixed(1) }}</td><td>Kops/s</td><td>{{ cpu.duration_ms }}ms</td></tr>
          <tr v-if="ram.score !== null"><td>RAM Write</td><td>{{ ram.score.toFixed(2) }}</td><td>GB/s</td><td>{{ ram.duration_ms }}ms</td></tr>
          <template v-for="r in disk.results" :key="r.name">
            <tr><td>Disque — {{ r.name }}</td><td>{{ r.score.toFixed(0) }}</td><td>{{ r.unit }}</td><td>{{ r.duration_ms }}ms</td></tr>
          </template>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Gauge, Cpu, MemoryStick, HardDrive, Play } from 'lucide-vue-next'

interface BenchResult { name: string; score: number; unit: string; duration_ms: number; details: string }
interface BenchState { loading: boolean; score: number | null; unit: string; duration_ms: number; details: string }

const drive = ref('C:')
const cpu = ref<BenchState>({ loading: false, score: null, unit: 'Kops/s', duration_ms: 0, details: '' })
const ram = ref<BenchState>({ loading: false, score: null, unit: 'GB/s', duration_ms: 0, details: '' })
const disk = ref<{ loading: boolean; results: BenchResult[] }>({ loading: false, results: [] })

const anyRunning = computed(() => cpu.value.loading || ram.value.loading || disk.value.loading)
const hasResults = computed(() => cpu.value.score !== null || ram.value.score !== null || disk.value.results.length > 0)

async function runCpu() {
  cpu.value.loading = true
  try {
    const r = await invoke<BenchResult>('run_cpu_bench')
    cpu.value = { loading: false, score: r.score, unit: r.unit, duration_ms: r.duration_ms, details: r.details }
  } catch { cpu.value.loading = false }
}

async function runRam() {
  ram.value.loading = true
  try {
    const r = await invoke<BenchResult>('run_ram_bench')
    ram.value = { loading: false, score: r.score, unit: r.unit, duration_ms: r.duration_ms, details: r.details }
  } catch { ram.value.loading = false }
}

async function runDisk() {
  disk.value.loading = true
  disk.value.results = []
  try {
    const r = await invoke<BenchResult[]>('run_disk_bench', { drive: drive.value })
    disk.value = { loading: false, results: r }
  } catch { disk.value.loading = false }
}

async function runAll() {
  await Promise.all([runCpu(), runRam(), runDisk()])
}

function reset() {
  cpu.value = { loading: false, score: null, unit: 'Kops/s', duration_ms: 0, details: '' }
  ram.value = { loading: false, score: null, unit: 'GB/s', duration_ms: 0, details: '' }
  disk.value = { loading: false, results: [] }
}

function scoreColor(score: number, max: number): string {
  const pct = score / max
  if (pct > 0.7) return '#22c55e'
  if (pct > 0.4) return '#f59e0b'
  return '#ef4444'
}

function diskBarPct(r: BenchResult): number {
  if (r.unit === 'MB/s') return Math.min(r.score / 5000 * 100, 100)
  return Math.min(r.score / 100000 * 100, 100)
}

function diskColor(r: BenchResult): string {
  if (r.unit === 'MB/s') return scoreColor(r.score, 5000)
  return scoreColor(r.score, 100000)
}
</script>

<style scoped>
.bench-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
.bench-card { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; padding: 14px; display: flex; flex-direction: column; gap: 8px; }
.bench-card-wide { grid-column: 1 / -1; }
.bench-card-title { font-size: 12px; font-weight: 600; opacity: .8; display: flex; align-items: center; gap: 6px; }
.bench-score { font-size: 28px; font-weight: 700; color: var(--accent, #7c3aed); }
.bench-unit { font-size: 14px; font-weight: 400; opacity: .7; }
.bench-score-sm { font-size: 13px; font-weight: 600; min-width: 90px; }
.bench-detail { font-size: 11px; opacity: .6; }
.bench-time { font-size: 11px; opacity: .5; }
.bench-bar-wrap { height: 6px; background: rgba(255,255,255,.08); border-radius: 3px; overflow: hidden; }
.bench-bar { height: 100%; border-radius: 3px; transition: width .4s ease; }
.bench-running { display: flex; align-items: center; gap: 8px; font-size: 12px; opacity: .7; }
.bench-idle { font-size: 12px; opacity: .4; }
.bench-spinner { width: 14px; height: 14px; border: 2px solid rgba(255,255,255,.2); border-top-color: var(--accent, #7c3aed); border-radius: 50%; animation: spin .8s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
.bench-disk-drive { display: flex; align-items: center; gap: 8px; font-size: 12px; }
.bench-disk-results { display: flex; flex-direction: column; gap: 8px; }
.bench-disk-row { display: flex; align-items: center; gap: 10px; }
.bench-disk-name { font-size: 12px; min-width: 120px; opacity: .8; }
.bench-summary { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; padding: 12px; }
</style>
