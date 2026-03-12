<script setup lang="ts">
import { computed } from "vue";
import { Cpu } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import NProgress from "@/components/ui/NProgress.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import NCollapse from "@/components/ui/NCollapse.vue";

const props = defineProps<{
  sysInfo: any;
  cpuCache: any;
  cpuExtended: any;
}>();

function kbStr(v: number) {
  return v >= 1024 ? `${(v / 1024).toFixed(0)} MB` : `${v} KB`;
}

const cpuScore = computed(() => {
  if (!props.sysInfo) return null;
  const cpu = props.sysInfo.cpu;
  const score = Math.min(100, (cpu.cores * 10) + (cpu.base_speed_mhz / 100));
  return Math.round(score);
});
</script>

<template>
  <div class="diag-tab-content">
    <DiagBanner
      :icon="Cpu"
      title="Processeur"
      desc="Architecture, fréquences, cache et virtualisation"
      color="orange"
    />
    <template v-if="sysInfo">
      <NCollapse title="Processeur" storageKey="diag-cpu-base" :defaultOpen="true">
        <div class="info-grid">
          <div class="info-row info-full"><span>Modèle complet</span><span style="font-weight:600">{{ sysInfo.cpu.name }}</span></div>
          <div class="info-row"><span>Fabricant</span><span>{{ sysInfo.cpu.manufacturer }}</span></div>
          <div class="info-row"><span>Cœurs physiques</span><NBadge variant="info">{{ sysInfo.cpu.cores }}</NBadge></div>
          <div class="info-row"><span>Threads logiques</span><NBadge variant="info">{{ sysInfo.cpu.threads }}</NBadge></div>
          <div class="info-row"><span>Fréquence de base</span><span>{{ (sysInfo.cpu.base_speed_mhz / 1000).toFixed(2) }} GHz</span></div>
          <div class="info-row"><span>Usage actuel</span><NBadge :variant="sysInfo.cpu.usage_percent > 80 ? 'danger' : sysInfo.cpu.usage_percent > 50 ? 'warning' : 'success'">{{ Math.round(sysInfo.cpu.usage_percent) }}%</NBadge></div>
        </div>
      </NCollapse>

      <p class="diag-section-label">Utilisation CPU</p>
      <div class="diag-stat-row"><span>Global</span><NProgress :value="sysInfo.cpu.usage_percent" size="sm" showLabel /></div>

      <template v-if="cpuExtended">
        <NCollapse title="Informations avancées" storageKey="diag-cpu-advanced" :defaultOpen="true">
          <div class="info-grid">
            <div class="info-row"><span>Socket</span><NBadge variant="info">{{ cpuExtended.socket_designation || "N/A" }}</NBadge></div>
            <div class="info-row"><span>Famille CPU</span><span>{{ cpuExtended.family }}</span></div>
            <div class="info-row"><span>Stepping</span><code>{{ cpuExtended.stepping || "N/A" }}</code></div>
            <div class="info-row"><span>Révision</span><code>{{ cpuExtended.revision || "N/A" }}</code></div>
            <div class="info-row"><span>ID Processeur</span><code style="font-size:10px">{{ cpuExtended.processor_id || "N/A" }}</code></div>
            <div class="info-row"><span>Fréquence max</span><span>{{ cpuExtended.max_clock_speed_mhz > 0 ? (cpuExtended.max_clock_speed_mhz / 1000).toFixed(2) + ' GHz' : 'N/A' }}</span></div>
            <div class="info-row"><span>Horloge externe (FSB)</span><span>{{ cpuExtended.external_clock_mhz > 0 ? cpuExtended.external_clock_mhz + ' MHz' : 'N/A' }}</span></div>
            <div class="info-row"><span>Tension actuelle</span><span>{{ cpuExtended.current_voltage }}</span></div>
            <div class="info-row"><span>Largeur d'adresse</span><span>{{ cpuExtended.address_width }}-bit</span></div>
            <div class="info-row"><span>Largeur de données</span><span>{{ cpuExtended.data_width }}-bit</span></div>
            <div class="info-row"><span>Packages physiques</span><span>{{ cpuExtended.number_of_physical_packages }}</span></div>
            <div class="info-row"><span>Virtualisation (VT-x/AMD-V)</span>
              <NBadge :variant="cpuExtended.virtualization_enabled ? 'success' : 'warning'">
                {{ cpuExtended.virtualization_enabled ? "Activée" : "Désactivée" }}
              </NBadge>
            </div>
          </div>
        </NCollapse>
      </template>

      <template v-if="cpuCache">
        <NCollapse title="Cache CPU" storageKey="diag-cpu-cache" :defaultOpen="true">
          <div class="info-grid">
            <div class="info-row"><span>L1 Instructions</span><NBadge variant="info">{{ kbStr(cpuCache.l1_instruction_kb) }}</NBadge></div>
            <div class="info-row"><span>L1 Données</span><NBadge variant="info">{{ kbStr(cpuCache.l1_data_kb) }}</NBadge></div>
            <div class="info-row"><span>L2</span><NBadge variant="info">{{ kbStr(cpuCache.l2_kb) }}</NBadge></div>
            <div class="info-row"><span>L3</span><NBadge variant="info">{{ kbStr(cpuCache.l3_kb) }}</NBadge></div>
            <div v-if="cpuCache.l4_kb > 0" class="info-row"><span>L4</span><NBadge variant="info">{{ kbStr(cpuCache.l4_kb) }}</NBadge></div>
            <div class="info-row"><span>Cache L2 (WMI)</span><span>{{ cpuExtended?.l2_cache_size_kb > 0 ? kbStr(cpuExtended.l2_cache_size_kb) : 'N/A' }}</span></div>
            <div class="info-row"><span>Cache L3 (WMI)</span><span>{{ cpuExtended?.l3_cache_size_kb > 0 ? kbStr(cpuExtended.l3_cache_size_kb) : 'N/A' }}</span></div>
          </div>
        </NCollapse>
      </template>

      <p class="diag-section-label">Architecture</p>
      <div class="card-block" style="margin-top:0">
        <div style="display:flex;gap:24px;flex-wrap:wrap">
          <div style="text-align:center">
            <div style="font-size:28px;font-weight:700;color:var(--accent)">{{ sysInfo.cpu.cores }}</div>
            <div class="muted" style="font-size:11px">CŒURS</div>
          </div>
          <div style="text-align:center">
            <div style="font-size:28px;font-weight:700;color:var(--accent)">{{ sysInfo.cpu.threads }}</div>
            <div class="muted" style="font-size:11px">THREADS</div>
          </div>
          <div style="text-align:center">
            <div style="font-size:28px;font-weight:700;color:var(--accent)">{{ (sysInfo.cpu.base_speed_mhz / 1000).toFixed(1) }}</div>
            <div class="muted" style="font-size:11px">GHz BASE</div>
          </div>
          <div v-if="cpuExtended?.max_clock_speed_mhz" style="text-align:center">
            <div style="font-size:28px;font-weight:700;color:var(--success)">{{ (cpuExtended.max_clock_speed_mhz / 1000).toFixed(1) }}</div>
            <div class="muted" style="font-size:11px">GHz MAX</div>
          </div>
        </div>
      </div>
    </template>
    <div v-else class="diag-loading"><div class="diag-spinner"></div> Informations CPU non disponibles...</div>
  </div>
</template>
