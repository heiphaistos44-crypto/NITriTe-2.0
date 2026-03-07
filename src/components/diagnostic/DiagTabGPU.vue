<script setup lang="ts">
import { Monitor } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";

const props = defineProps<{
  gpuList: any[];
}>();

function vramStr(mb: number): string {
  return mb >= 1024 ? `${(mb / 1024).toFixed(0)} GB` : `${mb} MB`;
}
</script>

<template>
  <div class="diag-tab-content">
    <DiagBanner
      :icon="Monitor"
      title="Carte(s) Graphique(s)"
      desc="VRAM, drivers, résolution et processeur vidéo"
      color="purple"
    />
    <div v-if="!gpuList.length" class="diag-loading"><div class="diag-spinner"></div> Aucun GPU détecté...</div>
    <template v-else>
      <p class="diag-section-label">Carte(s) graphique(s) — {{ gpuList.length }} détectée(s)</p>
      <div v-for="(g, i) in gpuList" :key="i" class="card-block">
        <div class="block-title">
          <span>{{ g.name }}</span>
          <NBadge :variant="g.status === 'OK' ? 'success' : 'warning'">{{ g.status }}</NBadge>
        </div>
        <div class="info-grid">
          <div class="info-row"><span>Mémoire vidéo (VRAM)</span><NBadge variant="info">{{ vramStr(g.adapter_ram_mb) }}</NBadge></div>
          <div class="info-row"><span>Résolution actuelle</span><span>{{ g.current_resolution || "N/A" }}</span></div>
          <div class="info-row"><span>Fréquence de rafraîchissement</span><span>{{ g.current_refresh_rate > 0 ? g.current_refresh_rate + ' Hz' : 'N/A' }}</span></div>
          <div class="info-row"><span>Version du driver</span><code>{{ g.driver_version || "N/A" }}</code></div>
          <div class="info-row"><span>Date du driver</span><span>{{ g.driver_date || "N/A" }}</span></div>
          <div class="info-row"><span>Processeur vidéo</span><span>{{ g.video_processor || "N/A" }}</span></div>
          <div class="info-row"><span>Mode vidéo</span><span class="muted" style="font-size:11px">{{ g.video_mode || "N/A" }}</span></div>
          <div class="info-row"><span>Type DAC</span><span>{{ g.adapter_dac_type || "N/A" }}</span></div>
          <div class="info-row info-full"><span>PNP Device ID</span><code style="font-size:9px;word-break:break-all">{{ g.pnp_device_id || "N/A" }}</code></div>
        </div>
      </div>
    </template>
  </div>
</template>
