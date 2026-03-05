<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import { RefreshCw, Cpu, HardDrive, Wifi } from "lucide-vue-next";

interface DiskPerf { name: string; read_mb: number; write_mb: number; queue_length: number; }
interface NetPerf { name: string; recv_mb: number; sent_mb: number; }
interface PerfSnapshot {
  cpu_percent: number; ram_used_gb: number; ram_total_gb: number; ram_percent: number;
  page_file_used_gb: number; page_file_total_gb: number;
  disks: DiskPerf[]; network: NetPerf[];
  handle_count: number; thread_count: number; process_count: number; uptime_hours: number;
}

const data = ref<PerfSnapshot | null>(null);
const loading = ref(true);
const error = ref("");
const autoRefresh = ref(false);
let timer: ReturnType<typeof setInterval> | null = null;

async function load() {
  loading.value = true;
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    data.value = await invoke<PerfSnapshot>("get_perf_snapshot");
    error.value = "";
  } catch (e: any) { error.value = e?.toString() ?? "Erreur"; }
  finally { loading.value = false; }
}

function toggleAuto() {
  autoRefresh.value = !autoRefresh.value;
  if (autoRefresh.value) {
    timer = setInterval(load, 3000);
  } else {
    if (timer) { clearInterval(timer); timer = null; }
  }
}

function pct(used: number, total: number) {
  if (total === 0) return 0;
  return Math.round(used / total * 100);
}

function barColor(p: number) {
  if (p >= 90) return 'var(--error)';
  if (p >= 70) return 'var(--warning)';
  return 'var(--success)';
}

function uptimeStr(h: number) {
  if (h >= 24) return `${(h / 24).toFixed(1)} jours`;
  return `${h.toFixed(1)} h`;
}

onMounted(load);
onUnmounted(() => { if (timer) clearInterval(timer); });
</script>

<template>
  <div v-if="loading && !data" style="display:flex;align-items:center;gap:10px;color:var(--text-secondary)">
    <NSpinner :size="16" /><span>Collecte des métriques...</span>
  </div>
  <div v-else-if="error && !data" style="color:var(--error)">⚠ {{ error }}</div>
  <div v-else-if="data" style="display:flex;flex-direction:column;gap:14px">

    <!-- Actions -->
    <div style="display:flex;gap:8px;align-items:center">
      <button @click="load" :disabled="loading"
        style="display:flex;align-items:center;gap:6px;padding:6px 12px;border-radius:6px;border:1px solid var(--border);background:var(--bg-secondary);color:var(--text-secondary);font-size:12px;cursor:pointer">
        <RefreshCw :size="13" :style="{animation:loading?'spin 1s linear infinite':''}" />Actualiser
      </button>
      <button @click="toggleAuto"
        :style="{display:'flex',alignItems:'center',gap:'6px',padding:'6px 12px',borderRadius:'6px',border:'1px solid var(--border)',fontSize:'12px',cursor:'pointer',
                 background:autoRefresh?'var(--accent)':'var(--bg-secondary)',color:autoRefresh?'white':'var(--text-secondary)'}">
        {{ autoRefresh ? '⏹ Stop auto (3s)' : '▶ Auto-refresh 3s' }}
      </button>
      <span v-if="loading && data" style="font-size:11px;color:var(--text-muted)">Mise à jour...</span>
    </div>

    <!-- CPU + RAM -->
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:12px">
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0"><Cpu :size="13" style="display:inline;margin-right:4px" />CPU</p>
        <div style="font-size:28px;font-weight:700" :style="{color:barColor(data.cpu_percent)}">{{ data.cpu_percent.toFixed(1) }}%</div>
        <div style="background:var(--bg-secondary);border-radius:4px;height:8px;margin-top:8px;overflow:hidden">
          <div :style="{width:data.cpu_percent+'%',height:'100%',background:barColor(data.cpu_percent),borderRadius:'4px',transition:'width 0.5s'}"></div>
        </div>
      </div>
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">RAM</p>
        <div style="font-size:28px;font-weight:700" :style="{color:barColor(data.ram_percent)}">{{ data.ram_percent.toFixed(1) }}%</div>
        <div style="font-size:11px;color:var(--text-muted);margin-top:2px">{{ data.ram_used_gb.toFixed(1) }} / {{ data.ram_total_gb.toFixed(1) }} GB</div>
        <div style="background:var(--bg-secondary);border-radius:4px;height:8px;margin-top:8px;overflow:hidden">
          <div :style="{width:data.ram_percent+'%',height:'100%',background:barColor(data.ram_percent),borderRadius:'4px',transition:'width 0.5s'}"></div>
        </div>
      </div>
    </div>

    <!-- Stats système -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Informations Système</p>
      <div class="info-grid">
        <div class="info-row"><span>Uptime</span><span>{{ uptimeStr(data.uptime_hours) }}</span></div>
        <div class="info-row"><span>Processus</span><span>{{ data.process_count }}</span></div>
        <div class="info-row"><span>Threads</span><span>{{ data.thread_count.toLocaleString() }}</span></div>
        <div class="info-row"><span>Handles</span><span>{{ data.handle_count.toLocaleString() }}</span></div>
        <div class="info-row"><span>Page file</span><span>{{ data.page_file_used_gb.toFixed(1) }} / {{ data.page_file_total_gb.toFixed(1) }} GB</span></div>
      </div>
    </div>

    <!-- Disques IO -->
    <div v-if="data.disks.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0"><HardDrive :size="13" style="display:inline;margin-right:4px" />I/O Disques (actuels)</p>
      <div v-for="(d, i) in data.disks" :key="i" style="padding:6px 0;border-bottom:1px solid var(--border)">
        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:4px">
          <span style="font-size:12px">{{ d.name }}</span>
          <div style="display:flex;gap:12px;font-size:11px;color:var(--text-muted)">
            <span>↓ {{ d.read_mb.toFixed(2) }} MB/s</span>
            <span>↑ {{ d.write_mb.toFixed(2) }} MB/s</span>
            <span>Queue: {{ d.queue_length.toFixed(1) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Réseau IO -->
    <div v-if="data.network.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0"><Wifi :size="13" style="display:inline;margin-right:4px" />I/O Réseau (actuels)</p>
      <div v-for="(n, i) in data.network" :key="i" style="padding:6px 0;border-bottom:1px solid var(--border)">
        <div style="display:flex;justify-content:space-between;align-items:center">
          <span style="font-size:11px;color:var(--text-secondary);max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ n.name }}</span>
          <div style="display:flex;gap:12px;font-size:11px;color:var(--text-muted)">
            <span>↓ {{ (n.recv_mb * 1000).toFixed(1) }} KB/s</span>
            <span>↑ {{ (n.sent_mb * 1000).toFixed(1) }} KB/s</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }
</style>
