<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NProgress from "@/components/ui/NProgress.vue";
import { useNotificationStore } from "@/stores/notifications";
import { Cpu, MemoryStick, HardDrive, Play, Zap, Trophy, RefreshCw } from "lucide-vue-next";

const notify = useNotificationStore();

interface BenchResult { score: number; details: string; duration_ms: number; name: string; unit: string; }

const cpuResult = ref<BenchResult | null>(null);
const ramResult = ref<BenchResult | null>(null);
const diskResult = ref<BenchResult | null>(null);
const runningCpu = ref(false);
const runningRam = ref(false);
const runningDisk = ref(false);

const anyRunning = computed(() => runningCpu.value || runningRam.value || runningDisk.value);

const overallScore = computed<number | null>(() => {
  const scores = [cpuResult.value?.score, ramResult.value?.score, diskResult.value?.score]
    .filter((s): s is number => s !== undefined && s !== null && s > 0);
  if (!scores.length) return null;
  return Math.round(scores.reduce((a, b) => a + b, 0) / scores.length);
});

function scoreColor(score: number) {
  if (score > 750) return "var(--success)";
  if (score > 400) return "var(--warning)";
  return "var(--danger)";
}

function scoreLabel(score: number) {
  if (score > 750) return "Excellent";
  if (score > 500) return "Bon";
  if (score > 300) return "Moyen";
  return "Faible";
}

function scoreVariant(score: number): "success" | "warning" | "danger" {
  if (score > 750) return "success";
  if (score > 400) return "warning";
  return "danger";
}

async function runCpu() {
  runningCpu.value = true;
  notify.info("Benchmark CPU", "Calcul float natif (8s)…");
  try {
    cpuResult.value = await invoke<BenchResult>("run_cpu_bench");
    notify.success("CPU terminé", `Score : ${cpuResult.value.score} pts`);
  } catch (e: any) {
    notify.error("Erreur CPU", String(e));
  }
  runningCpu.value = false;
}

async function runRam() {
  runningRam.value = true;
  notify.info("Benchmark RAM", "Test bande passante 256 MB (lecture + écriture)…");
  try {
    ramResult.value = await invoke<BenchResult>("run_ram_bench");
    notify.success("RAM terminé", `Score : ${ramResult.value.score} pts`);
  } catch (e: any) {
    notify.error("Erreur RAM", String(e));
  }
  runningRam.value = false;
}

async function runDisk() {
  runningDisk.value = true;
  notify.info("Benchmark Disque", "Test lecture/écriture 256 MB + IOPS 4K (≈15s)…");
  try {
    diskResult.value = await invoke<BenchResult>("run_disk_bench", { drive: null });
    notify.success("Disque terminé", `Score : ${diskResult.value.score} pts`);
  } catch (e: any) {
    notify.error("Erreur Disque", String(e));
  }
  runningDisk.value = false;
}

async function runAll() {
  await Promise.all([runCpu(), runRam(), runDisk()]);
}
</script>

<template>
  <div class="bench-page">
    <div class="page-header">
      <div class="header-icon"><Zap :size="22" /></div>
      <div>
        <h1>Benchmark Système</h1>
        <p class="subtitle">Évaluation précise CPU (8s natif), RAM (256 MB bande passante) et Disque (256 MB + IOPS)</p>
      </div>
      <NButton variant="primary" size="sm" :loading="anyRunning" :disabled="anyRunning" @click="runAll" style="margin-left:auto">
        <Play :size="13" /> {{ anyRunning ? 'Tests en cours…' : 'Tout lancer' }}
      </NButton>
    </div>

    <!-- Score global -->
    <div v-if="overallScore !== null" class="global-score">
      <Trophy :size="22" style="color:var(--warning)" />
      <span class="score-val">{{ overallScore }}</span>
      <span class="score-label">Score global / 1000</span>
      <NBadge :variant="scoreVariant(overallScore)">
        {{ scoreLabel(overallScore) }}
      </NBadge>
      <NButton variant="ghost" size="sm" @click="cpuResult=null; ramResult=null; diskResult=null" style="margin-left:auto">
        <RefreshCw :size="13" /> Réinitialiser
      </NButton>
    </div>

    <div class="bench-grid">
      <!-- CPU -->
      <NCard>
        <template #header>
          <div class="section-header"><Cpu :size="15" /><span>Processeur (CPU)</span></div>
        </template>
        <div class="bench-card-body">
          <p class="bench-desc">Calcul Rust natif : sqrt + sin + ln sur 8 secondes. Mesure la puissance brute mono-thread.</p>
          <div v-if="cpuResult" class="result-block">
            <div class="score-big" :style="{ color: scoreColor(cpuResult.score) }">{{ cpuResult.score }}</div>
            <div class="score-sub">/ 1000 pts</div>
            <NBadge :variant="scoreVariant(cpuResult.score)">{{ scoreLabel(cpuResult.score) }}</NBadge>
            <NProgress :value="Math.min(cpuResult.score / 10, 100)" :glow="true" size="sm" style="margin-top:8px" />
            <p class="result-detail">{{ cpuResult.details }}</p>
            <p class="result-time">Durée : {{ (cpuResult.duration_ms / 1000).toFixed(1) }}s</p>
          </div>
          <div v-else-if="runningCpu" class="running-state">
            <div class="running-spinner"></div>
            <span>Calcul en cours (8s)…</span>
          </div>
          <NButton variant="secondary" size="sm" :loading="runningCpu" :disabled="runningCpu" @click="runCpu">
            <Play :size="13" /> {{ runningCpu ? 'En cours…' : 'Lancer' }}
          </NButton>
        </div>
      </NCard>

      <!-- RAM -->
      <NCard>
        <template #header>
          <div class="section-header"><MemoryStick :size="15" /><span>Mémoire (RAM)</span></div>
        </template>
        <div class="bench-card-body">
          <p class="bench-desc">Bande passante réelle : écriture + lecture séquentielle 256 MB + latence accès aléatoire. DDR4-3200 ≈ 25 GB/s.</p>
          <div v-if="ramResult" class="result-block">
            <div class="score-big" :style="{ color: scoreColor(ramResult.score) }">{{ ramResult.score }}</div>
            <div class="score-sub">/ 1000 pts</div>
            <NBadge :variant="scoreVariant(ramResult.score)">{{ scoreLabel(ramResult.score) }}</NBadge>
            <NProgress :value="Math.min(ramResult.score / 10, 100)" :glow="true" size="sm" style="margin-top:8px" />
            <p class="result-detail">{{ ramResult.details }}</p>
            <p class="result-time">Durée : {{ (ramResult.duration_ms / 1000).toFixed(1) }}s</p>
          </div>
          <div v-else-if="runningRam" class="running-state">
            <div class="running-spinner"></div>
            <span>Test bande passante 256 MB…</span>
          </div>
          <NButton variant="secondary" size="sm" :loading="runningRam" :disabled="runningRam" @click="runRam">
            <Play :size="13" /> {{ runningRam ? 'En cours…' : 'Lancer' }}
          </NButton>
        </div>
      </NCard>

      <!-- Disque -->
      <NCard>
        <template #header>
          <div class="section-header"><HardDrive :size="15" /><span>Stockage (Disque)</span></div>
        </template>
        <div class="bench-card-body">
          <p class="bench-desc">Lecture/écriture séquentielle 256 MB + IOPS 4K aléatoire 3s. Score composite (60% seq + 40% IOPS).</p>
          <div v-if="diskResult" class="result-block">
            <div class="score-big" :style="{ color: scoreColor(diskResult.score) }">{{ diskResult.score }}</div>
            <div class="score-sub">/ 1000 pts</div>
            <NBadge :variant="scoreVariant(diskResult.score)">{{ scoreLabel(diskResult.score) }}</NBadge>
            <NProgress :value="Math.min(diskResult.score / 10, 100)" :glow="true" size="sm" style="margin-top:8px" />
            <p class="result-detail">{{ diskResult.details }}</p>
            <p class="result-time">Durée : {{ (diskResult.duration_ms / 1000).toFixed(1) }}s</p>
          </div>
          <div v-else-if="runningDisk" class="running-state">
            <div class="running-spinner"></div>
            <span>Test disque en cours (≈15s)…</span>
          </div>
          <NButton variant="secondary" size="sm" :loading="runningDisk" :disabled="runningDisk" @click="runDisk">
            <Play :size="13" /> {{ runningDisk ? 'En cours…' : 'Lancer' }}
          </NButton>
        </div>
      </NCard>
    </div>
  </div>
</template>

<style scoped>
.bench-page { display: flex; flex-direction: column; gap: 16px; }
.page-header { display: flex; align-items: center; gap: 12px; }
.header-icon { width: 42px; height: 42px; border-radius: var(--radius-lg); background: var(--warning-muted); display: flex; align-items: center; justify-content: center; color: var(--warning); flex-shrink: 0; }
h1 { font-size: 22px; font-weight: 700; }
.subtitle { font-size: 12px; color: var(--text-muted); }
.global-score { display: flex; align-items: center; gap: 12px; padding: 14px 20px; background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-xl); }
.score-val { font-size: 28px; font-weight: 900; color: var(--text-primary); font-family: "JetBrains Mono", monospace; }
.score-label { font-size: 13px; color: var(--text-muted); }
.bench-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 14px; }
@media (max-width: 900px) { .bench-grid { grid-template-columns: 1fr; } }
.section-header { display: flex; align-items: center; gap: 8px; }
.bench-card-body { display: flex; flex-direction: column; gap: 12px; }
.bench-desc { font-size: 12px; color: var(--text-muted); line-height: 1.5; }
.result-block { display: flex; flex-direction: column; gap: 4px; padding: 12px; background: var(--bg-tertiary); border-radius: var(--radius-md); }
.score-big { font-size: 36px; font-weight: 900; line-height: 1; font-family: "JetBrains Mono", monospace; }
.score-sub { font-size: 12px; color: var(--text-muted); }
.result-detail { font-size: 11px; color: var(--text-secondary); margin-top: 4px; font-family: "JetBrains Mono", monospace; line-height: 1.5; }
.result-time { font-size: 11px; color: var(--text-muted); }
.running-state { display: flex; align-items: center; gap: 10px; padding: 14px; background: var(--bg-tertiary); border-radius: var(--radius-md); font-size: 12px; color: var(--text-muted); }
.running-spinner { width: 16px; height: 16px; border: 2px solid var(--border); border-top-color: var(--accent-primary); border-radius: 50%; animation: spin 0.8s linear infinite; flex-shrink: 0; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
