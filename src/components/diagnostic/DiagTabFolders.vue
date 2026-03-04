<script setup lang="ts">
import { computed } from "vue";
import NProgress from "@/components/ui/NProgress.vue";
import NBadge from "@/components/ui/NBadge.vue";

const props = defineProps<{
  folders: any[];
}>();

const maxSize = computed(() => Math.max(...props.folders.map(f => f.size_mb), 1));

function sizeStr(mb: number): string {
  if (mb >= 1024) return `${(mb / 1024).toFixed(1)} GB`;
  return `${mb.toFixed(0)} MB`;
}

const sorted = computed(() => [...props.folders].sort((a, b) => b.size_mb - a.size_mb));
const totalMb = computed(() => props.folders.reduce((a, f) => a + f.size_mb, 0));
const totalFiles = computed(() => props.folders.reduce((a, f) => a + f.file_count, 0));
</script>

<template>
  <div v-if="!folders.length" class="diag-empty">Calcul des tailles en cours...</div>
  <template v-else>
    <div style="display:flex;gap:12px;flex-wrap:wrap;margin-bottom:16px">
      <div class="card-block" style="flex:1;min-width:160px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Taille totale analysée</p>
        <span style="font-size:24px;font-weight:700;color:var(--accent)">{{ sizeStr(totalMb) }}</span>
      </div>
      <div class="card-block" style="flex:1;min-width:160px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Fichiers analysés</p>
        <span style="font-size:24px;font-weight:700">{{ totalFiles.toLocaleString() }}</span>
      </div>
    </div>

    <p class="diag-section-label">Tailles par dossier</p>
    <div v-for="(f, i) in sorted" :key="i" class="card-block">
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:6px">
        <div>
          <span style="font-weight:600">{{ f.label }}</span>
          <span class="muted" style="font-size:11px;margin-left:8px">{{ f.path }}</span>
        </div>
        <div style="display:flex;gap:8px;align-items:center">
          <NBadge :variant="f.size_mb > 10240 ? 'danger' : f.size_mb > 1024 ? 'warning' : 'default'">
            {{ sizeStr(f.size_mb) }}
          </NBadge>
          <span class="muted" style="font-size:11px">{{ f.file_count.toLocaleString() }} fichiers</span>
        </div>
      </div>
      <NProgress
        :value="(f.size_mb / maxSize) * 100"
        :variant="f.size_mb > 10240 ? 'danger' : f.size_mb > 1024 ? 'warning' : 'default'"
        size="sm"
      />
    </div>
  </template>
</template>
