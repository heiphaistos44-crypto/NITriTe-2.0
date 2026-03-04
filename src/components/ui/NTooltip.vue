<script setup lang="ts">
withDefaults(
  defineProps<{ text: string; position?: "top" | "bottom" | "left" | "right" }>(),
  { position: "top" }
);
</script>

<template>
  <div class="n-tooltip-wrap" :class="`tooltip-${position}`">
    <slot />
    <div class="n-tooltip" role="tooltip">{{ text }}</div>
  </div>
</template>

<style scoped>
.n-tooltip-wrap {
  position: relative;
  display: inline-flex;
}

.n-tooltip {
  position: absolute;
  padding: 4px 8px;
  background: var(--bg-elevated);
  color: var(--text-primary);
  font-size: 12px;
  border-radius: var(--radius-sm);
  white-space: nowrap;
  pointer-events: none;
  opacity: 0;
  transition: opacity var(--transition-fast);
  z-index: 1000;
  box-shadow: var(--shadow-md);
}

.n-tooltip-wrap:hover .n-tooltip { opacity: 1; }

.tooltip-top .n-tooltip { bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 6px; }
.tooltip-bottom .n-tooltip { top: 100%; left: 50%; transform: translateX(-50%); margin-top: 6px; }
.tooltip-left .n-tooltip { right: 100%; top: 50%; transform: translateY(-50%); margin-right: 6px; }
.tooltip-right .n-tooltip { left: 100%; top: 50%; transform: translateY(-50%); margin-left: 6px; }
</style>
