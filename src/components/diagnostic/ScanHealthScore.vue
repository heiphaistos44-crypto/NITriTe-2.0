<script setup lang="ts">
import NBadge from "@/components/ui/NBadge.vue";
import NButton from "@/components/ui/NButton.vue";
defineProps<{
  healthScore: number;
  scoreVariant: "accent" | "success" | "warning" | "danger" | "info" | "neutral" | "default";
  scoreLabel: string;
  solutionsCount: number;
  criticalCount: number;
}>();
const emit = defineEmits<{ export: [] }>();
</script>

<template>
  <div class="diag-section" style="display:flex;align-items:center;gap:20px;padding:16px 20px">
    <div style="position:relative;width:80px;height:80px;flex-shrink:0">
      <svg viewBox="0 0 80 80" style="width:100%;height:100%">
        <circle cx="40" cy="40" r="34" fill="none" stroke="var(--border)" stroke-width="8"/>
        <circle cx="40" cy="40" r="34" fill="none"
          :stroke="healthScore >= 80 ? 'var(--success)' : healthScore >= 60 ? 'var(--warning)' : 'var(--error)'"
          stroke-width="8" stroke-linecap="round"
          :stroke-dasharray="`${2 * Math.PI * 34 * healthScore / 100} ${2 * Math.PI * 34}`"
          stroke-dashoffset="53" transform="rotate(-90 40 40)"/>
        <text x="40" y="44" text-anchor="middle" font-size="18" font-weight="bold"
          :fill="healthScore >= 80 ? 'var(--success)' : healthScore >= 60 ? 'var(--warning)' : 'var(--error)'">
          {{ healthScore }}
        </text>
      </svg>
    </div>
    <div>
      <div style="font-size:20px;font-weight:700;margin-bottom:4px">Score de Santé Système</div>
      <NBadge :variant="scoreVariant" style="font-size:14px;padding:4px 12px">{{ scoreLabel }}</NBadge>
      <div style="font-size:12px;color:var(--text-muted);margin-top:6px">
        {{ solutionsCount }} problème(s) détecté(s) — {{ criticalCount }} critique(s)
      </div>
    </div>
    <div style="margin-left:auto">
      <NButton variant="ghost" size="sm" @click="emit('export')">Exporter le rapport</NButton>
    </div>
  </div>
</template>
