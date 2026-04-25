<script setup lang="ts">
import { ref } from "vue";
import { ScanLine } from "lucide-vue-next";
import NButton from "@/components/ui/NButton.vue";

const emit = defineEmits<{
  launchTotal: [formats: Set<string>];
}>();

const totalFormats = ref<Set<string>>(new Set(["html"]));

function toggleTotalFmt(fmt: string) {
  if (totalFormats.value.has(fmt)) { if (totalFormats.value.size > 1) totalFormats.value.delete(fmt); }
  else { totalFormats.value.add(fmt); }
  totalFormats.value = new Set(totalFormats.value);
}
</script>

<template>
  <div style="display:flex;flex-direction:column;gap:14px">
    <div class="scan-choice-card">
      <div class="scan-choice-header">
        <ScanLine :size="18" style="color:var(--accent-primary);flex-shrink:0" />
        <div>
          <div class="scan-choice-title">Scan Complet du Système</div>
          <div class="scan-choice-desc">Analyse approfondie : DISM/SFC, BitLocker, licences Windows &amp; Office, processus suspects, mises à jour, antivirus. Durée ~3 min.</div>
        </div>
      </div>
      <div class="scan-choice-formats">
        <span class="scan-choice-fmt-label">Formats d'export :</span>
        <button v-for="fmt in ['html','txt','md','json'] as const" :key="fmt"
          class="scan-fmt-btn" :class="{ selected: totalFormats.has(fmt) }"
          @click="toggleTotalFmt(fmt)">{{ fmt.toUpperCase() }}</button>
      </div>
      <NButton variant="primary" size="sm" @click="emit('launchTotal', new Set(totalFormats))">
        <ScanLine :size="13" />Lancer le Scan Complet{{ totalFormats.size > 1 ? ` — ${[...totalFormats].join(', ').toUpperCase()}` : '' }}
      </NButton>
    </div>

  </div>
</template>

<style scoped>
.scan-choice-card {
  background: var(--surface-2, #161625);
  border: 1px solid var(--border, #2e2e33);
  border-radius: 10px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.scan-choice-card--simple { border-color: rgba(74, 222, 128, 0.2); }
.scan-choice-header { display: flex; align-items: flex-start; gap: 12px; }
.scan-choice-title { font-size: 14px; font-weight: 600; color: var(--text-primary, #e2e8f0); margin-bottom: 4px; }
.scan-choice-desc { font-size: 12px; color: var(--text-muted, #64748b); line-height: 1.5; }
.scan-choice-formats { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.scan-choice-fmt-label { font-size: 11px; color: var(--text-muted, #64748b); }
.scan-fmt-btn {
  padding: 4px 10px;
  border: 1px solid var(--border, #2e2e33);
  border-radius: 5px;
  background: var(--surface-1, #0d0d1a);
  color: var(--text-secondary, #94a3b8);
  font-size: 11px;
  font-family: 'Consolas', monospace;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s;
}
.scan-fmt-btn:hover { border-color: var(--accent-primary, #7c9af5); color: var(--text-primary, #e2e8f0); }
.scan-fmt-btn.selected {
  border-color: var(--accent-primary, #7c9af5);
  background: rgba(124, 154, 245, 0.1);
  color: var(--accent-primary, #7c9af5);
}
</style>
