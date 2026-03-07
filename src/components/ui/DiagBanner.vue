<template>
  <div class="diag-bann" :class="`bann-${color}`">
    <div class="diag-bann-icon">
      <component :is="icon" :size="24" />
    </div>
    <div class="diag-bann-body">
      <div class="diag-bann-title">{{ title }}</div>
      <div class="diag-bann-desc">{{ desc }}</div>
    </div>
    <div v-if="$slots.default" class="diag-bann-actions">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Component } from 'vue'
defineProps<{ icon: Component; title: string; desc: string; color?: string }>()
</script>

<style scoped>
.diag-bann {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 18px 22px;
  border-radius: 16px;
  border: 1px solid;
  flex-wrap: wrap;
}
.diag-bann-icon {
  width: 52px; height: 52px;
  border-radius: 14px;
  display: flex; align-items: center; justify-content: center;
  color: #fff; flex-shrink: 0;
}
.diag-bann-body { flex: 1; min-width: 160px; }
.diag-bann-title { font-size: 18px; font-weight: 700; margin-bottom: 3px; color: var(--text-primary); }
.diag-bann-desc  { font-size: 12px; color: var(--text-secondary); }
.diag-bann-actions { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }

/* ---- Color variants ---- */
.bann-blue    { background: linear-gradient(135deg,rgba(59,130,246,.13),rgba(37,99,235,.06));  border-color: rgba(59,130,246,.3); }
.bann-blue    .diag-bann-icon { background: linear-gradient(135deg,#3b82f6,#1d4ed8); box-shadow: 0 4px 16px rgba(59,130,246,.4); }

.bann-purple  { background: linear-gradient(135deg,rgba(139,92,246,.13),rgba(109,40,217,.06)); border-color: rgba(139,92,246,.3); }
.bann-purple  .diag-bann-icon { background: linear-gradient(135deg,#8b5cf6,#6d28d9); box-shadow: 0 4px 16px rgba(139,92,246,.4); }

.bann-green   { background: linear-gradient(135deg,rgba(34,197,94,.13),rgba(21,128,61,.06));   border-color: rgba(34,197,94,.3); }
.bann-green   .diag-bann-icon { background: linear-gradient(135deg,#22c55e,#15803d); box-shadow: 0 4px 16px rgba(34,197,94,.4); }

.bann-red     { background: linear-gradient(135deg,rgba(239,68,68,.13),rgba(185,28,28,.06));   border-color: rgba(239,68,68,.3); }
.bann-red     .diag-bann-icon { background: linear-gradient(135deg,#ef4444,#b91c1c); box-shadow: 0 4px 16px rgba(239,68,68,.4); }

.bann-orange  { background: linear-gradient(135deg,rgba(249,115,22,.13),rgba(194,65,12,.06)); border-color: rgba(249,115,22,.3); }
.bann-orange  .diag-bann-icon { background: linear-gradient(135deg,#f97316,#c2410c); box-shadow: 0 4px 16px rgba(249,115,22,.4); }

.bann-amber   { background: linear-gradient(135deg,rgba(245,158,11,.13),rgba(180,83,9,.06));  border-color: rgba(245,158,11,.3); }
.bann-amber   .diag-bann-icon { background: linear-gradient(135deg,#f59e0b,#b45309); box-shadow: 0 4px 16px rgba(245,158,11,.4); }

.bann-teal    { background: linear-gradient(135deg,rgba(20,184,166,.13),rgba(15,118,110,.06)); border-color: rgba(20,184,166,.3); }
.bann-teal    .diag-bann-icon { background: linear-gradient(135deg,#14b8a6,#0f766e); box-shadow: 0 4px 16px rgba(20,184,166,.4); }

.bann-cyan    { background: linear-gradient(135deg,rgba(6,182,212,.13),rgba(14,116,144,.06)); border-color: rgba(6,182,212,.3); }
.bann-cyan    .diag-bann-icon { background: linear-gradient(135deg,#06b6d4,#0e7490); box-shadow: 0 4px 16px rgba(6,182,212,.4); }

.bann-indigo  { background: linear-gradient(135deg,rgba(99,102,241,.13),rgba(67,56,202,.06)); border-color: rgba(99,102,241,.3); }
.bann-indigo  .diag-bann-icon { background: linear-gradient(135deg,#6366f1,#4338ca); box-shadow: 0 4px 16px rgba(99,102,241,.4); }

.bann-pink    { background: linear-gradient(135deg,rgba(236,72,153,.13),rgba(157,23,77,.06)); border-color: rgba(236,72,153,.3); }
.bann-pink    .diag-bann-icon { background: linear-gradient(135deg,#ec4899,#9d174d); box-shadow: 0 4px 16px rgba(236,72,153,.4); }

.bann-gold    { background: linear-gradient(135deg,rgba(234,179,8,.13),rgba(161,98,7,.06));  border-color: rgba(234,179,8,.3); }
.bann-gold    .diag-bann-icon { background: linear-gradient(135deg,#eab308,#a16207); box-shadow: 0 4px 16px rgba(234,179,8,.4); }

.bann-slate   { background: linear-gradient(135deg,rgba(100,116,139,.13),rgba(51,65,85,.06)); border-color: rgba(100,116,139,.3); }
.bann-slate   .diag-bann-icon { background: linear-gradient(135deg,#64748b,#334155); box-shadow: 0 4px 16px rgba(100,116,139,.4); }

.bann-emerald { background: linear-gradient(135deg,rgba(16,185,129,.13),rgba(5,150,105,.06)); border-color: rgba(16,185,129,.3); }
.bann-emerald .diag-bann-icon { background: linear-gradient(135deg,#10b981,#059669); box-shadow: 0 4px 16px rgba(16,185,129,.4); }
</style>
