<template>
  <div class="diag-tab-content">
    <div class="diag-section-header"><Activity :size="16" /> Historique des Performances</div>

    <!-- Config -->
    <div class="perf-config">
      <label style="font-size:12px;opacity:.7">Échantillons :</label>
      <select v-model.number="samples" class="diag-input" style="width:80px">
        <option :value="10">10</option>
        <option :value="20">20</option>
        <option :value="30">30</option>
        <option :value="60">60</option>
      </select>
      <label style="font-size:12px;opacity:.7">Intervalle :</label>
      <select v-model.number="interval" class="diag-input" style="width:90px">
        <option :value="1">1 sec</option>
        <option :value="2">2 sec</option>
        <option :value="5">5 sec</option>
      </select>
      <button class="diag-btn diag-btn-primary" :disabled="loading" @click="runHistory">
        <Play :size="13" /> Lancer ({{ samples * interval }}s)
      </button>
      <span v-if="loading" class="perf-running">
        <span class="perf-spinner" /> Collecte {{ progressPts }}/{{ samples }} échantillons...
      </span>
    </div>

    <div v-if="history">
      <!-- Summary cards -->
      <div class="perf-summary">
        <div class="perf-card">
          <div class="perf-card-label">CPU Moyen</div>
          <div class="perf-card-val" :style="{color: pctColor(history.avg_cpu)}">{{ history.avg_cpu.toFixed(1) }}%</div>
        </div>
        <div class="perf-card">
          <div class="perf-card-label">CPU Pic</div>
          <div class="perf-card-val" :style="{color: pctColor(history.peak_cpu)}">{{ history.peak_cpu.toFixed(1) }}%</div>
        </div>
        <div class="perf-card">
          <div class="perf-card-label">RAM Moyenne</div>
          <div class="perf-card-val">{{ fmtMb(history.avg_ram_mb) }}</div>
        </div>
        <div class="perf-card">
          <div class="perf-card-label">RAM Pic</div>
          <div class="perf-card-val">{{ fmtMb(history.peak_ram_mb) }}</div>
        </div>
        <div class="perf-card">
          <div class="perf-card-label">Durée</div>
          <div class="perf-card-val" style="font-size:14px">{{ history.duration_secs }}s</div>
        </div>
      </div>

      <!-- CPU Chart -->
      <div class="perf-chart-wrap">
        <div class="perf-chart-title">CPU % dans le temps</div>
        <div class="perf-chart">
          <div class="perf-grid-lines">
            <div class="perf-grid-line" v-for="l in [75,50,25]" :key="l" :style="{bottom: l+'%'}">
              <span class="perf-grid-label">{{ l }}%</span>
            </div>
          </div>
          <div class="perf-bars">
            <div v-for="(p, i) in history.points" :key="i" class="perf-bar-col" :title="`${p.timestamp}\nCPU: ${p.cpu_percent}%`">
              <div class="perf-bar-inner" :style="{height: p.cpu_percent+'%', background: pctColor(p.cpu_percent)}" />
            </div>
          </div>
        </div>
        <div class="perf-x-axis">
          <span v-for="(p, i) in history.points" :key="i"
                v-show="i % Math.max(1, Math.floor(history.points.length/8)) === 0"
                class="perf-x-label">{{ p.timestamp }}</span>
        </div>
      </div>

      <!-- RAM Chart -->
      <div class="perf-chart-wrap">
        <div class="perf-chart-title">RAM utilisée (MB)</div>
        <div class="perf-chart">
          <div class="perf-grid-lines">
            <div class="perf-grid-line" v-for="l in [75,50,25]" :key="l" :style="{bottom: l+'%'}">
              <span class="perf-grid-label">{{ Math.round(maxRam * l / 100) }}MB</span>
            </div>
          </div>
          <div class="perf-bars">
            <div v-for="(p, i) in history.points" :key="i" class="perf-bar-col"
                 :title="`${p.timestamp}\nRAM: ${p.ram_used_mb}MB / ${p.ram_total_mb}MB`">
              <div class="perf-bar-inner"
                   :style="{height: (p.ram_used_mb/maxRam*100)+'%', background: '#3b82f6'}" />
            </div>
          </div>
        </div>
      </div>

      <!-- Data table -->
      <div class="diag-section-header" style="margin-top:16px"><Table :size="16" /> Données brutes</div>
      <div style="max-height:250px;overflow-y:auto">
        <table class="diag-table">
          <thead>
            <tr>
              <th>Heure</th><th>CPU %</th><th>RAM MB</th>
              <th>Disque R</th><th>Disque W</th><th>Réseau R</th><th>Réseau E</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(p, i) in history.points" :key="i">
              <td>{{ p.timestamp }}</td>
              <td :style="{color: pctColor(p.cpu_percent)}">{{ p.cpu_percent.toFixed(1) }}%</td>
              <td>{{ p.ram_used_mb }}</td>
              <td>{{ p.disk_read_mbps.toFixed(2) }}</td>
              <td>{{ p.disk_write_mbps.toFixed(2) }}</td>
              <td>{{ p.net_recv_mbps.toFixed(3) }}</td>
              <td>{{ p.net_send_mbps.toFixed(3) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Top Processes -->
    <div class="diag-section-header" style="margin-top:20px"><Cpu :size="16" /> Top Processus par CPU</div>
    <div style="display:flex;gap:8px;margin-bottom:8px">
      <button class="diag-btn" :disabled="topLoading" @click="loadTop">
        <RefreshCw :size="13" /> Actualiser
      </button>
    </div>
    <div v-if="topLoading" class="diag-loading">Chargement...</div>
    <table v-else-if="topProcs.length > 0" class="diag-table">
      <thead><tr><th>Processus</th><th>PID</th><th>CPU (s total)</th><th>RAM (MB)</th></tr></thead>
      <tbody>
        <tr v-for="p in topProcs" :key="p.pid">
          <td>{{ p.name }}</td>
          <td>{{ p.pid }}</td>
          <td>{{ p.cpu_percent.toFixed(1) }}</td>
          <td>{{ p.ram_mb }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Activity, Play, Cpu, RefreshCw } from 'lucide-vue-next'

interface PerfPoint {
  timestamp: string; cpu_percent: number; ram_used_mb: number; ram_total_mb: number
  disk_read_mbps: number; disk_write_mbps: number; net_recv_mbps: number; net_send_mbps: number
}
interface PerfHistory {
  points: PerfPoint[]; sample_interval_secs: number; duration_secs: number
  avg_cpu: number; peak_cpu: number; avg_ram_mb: number; peak_ram_mb: number
}
interface TopProcess { name: string; pid: number; cpu_percent: number; ram_mb: number; disk_mbps: number }

const loading = ref(false)
const history = ref<PerfHistory | null>(null)
const samples = ref(20)
const interval = ref(2)
const progressPts = ref(0)
const topLoading = ref(false)
const topProcs = ref<TopProcess[]>([])

const maxRam = computed(() => {
  if (!history.value) return 1
  return Math.max(...history.value.points.map(p => p.ram_total_mb), 1)
})

async function runHistory() {
  loading.value = true; history.value = null; progressPts.value = 0
  try {
    history.value = await invoke<PerfHistory>('get_perf_history', { samples: samples.value, intervalSecs: interval.value })
  } finally { loading.value = false }
}

async function loadTop() {
  topLoading.value = true
  try { topProcs.value = await invoke<TopProcess[]>('get_top_processes_by_cpu', { limit: 20 }) }
  finally { topLoading.value = false }
}

function pctColor(pct: number): string {
  if (pct > 80) return '#ef4444'
  if (pct > 50) return '#f59e0b'
  return '#22c55e'
}

function fmtMb(mb: number): string {
  if (mb >= 1024) return (mb / 1024).toFixed(1) + ' GB'
  return mb + ' MB'
}

onMounted(loadTop)
</script>

<style scoped>
.perf-config { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; margin-bottom: 16px; padding: 12px; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; }
.perf-running { display: flex; align-items: center; gap: 8px; font-size: 12px; opacity: .7; }
.perf-spinner { display: inline-block; width: 14px; height: 14px; border: 2px solid rgba(255,255,255,.2); border-top-color: var(--accent, #7c3aed); border-radius: 50%; animation: spin .8s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
.perf-summary { display: grid; grid-template-columns: repeat(5, 1fr); gap: 10px; margin-bottom: 16px; }
.perf-card { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; padding: 12px; text-align: center; }
.perf-card-label { font-size: 10px; opacity: .5; text-transform: uppercase; margin-bottom: 4px; }
.perf-card-val { font-size: 20px; font-weight: 700; }
.perf-chart-wrap { margin-bottom: 16px; }
.perf-chart-title { font-size: 12px; font-weight: 600; opacity: .7; margin-bottom: 6px; }
.perf-chart { position: relative; height: 160px; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; overflow: hidden; padding: 8px; }
.perf-grid-lines { position: absolute; inset: 0; pointer-events: none; }
.perf-grid-line { position: absolute; left: 0; right: 0; border-top: 1px dashed rgba(255,255,255,.07); }
.perf-grid-label { font-size: 9px; opacity: .4; margin-left: 4px; }
.perf-bars { display: flex; align-items: flex-end; height: 100%; gap: 1px; padding-bottom: 2px; }
.perf-bar-col { flex: 1; height: 100%; display: flex; align-items: flex-end; min-width: 2px; }
.perf-bar-inner { width: 100%; border-radius: 2px 2px 0 0; transition: height .3s ease; min-height: 2px; }
.perf-x-axis { display: flex; justify-content: space-between; margin-top: 2px; }
.perf-x-label { font-size: 9px; opacity: .4; }
</style>
