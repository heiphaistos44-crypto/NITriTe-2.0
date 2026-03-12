<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: "accent" | "success" | "warning" | "danger" | "info" | "neutral" | "default";
    dot?: boolean;
  }>(),
  { variant: "neutral", dot: false }
);
</script>

<template>
  <span class="n-badge" :class="`n-badge--${variant}`">
    <span v-if="dot" class="n-badge__dot" />
    <slot />
  </span>
</template>

<style scoped>
.n-badge {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 2px 8px;
  border-radius: 99px;
  font-size: 11px;
  font-weight: 600;
  line-height: 1.4;
  letter-spacing: 0.01em;
}

.n-badge__dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  background: currentColor;
  flex-shrink: 0;
}

.n-badge--accent  { background: var(--accent-muted);   color: var(--accent-primary); border: 1px solid rgba(249, 115, 22, 0.2); }
.n-badge--success { background: var(--success-muted);  color: var(--success);        border: 1px solid rgba(34, 197, 94, 0.2); }
.n-badge--warning { background: var(--warning-muted);  color: var(--warning);        border: 1px solid rgba(234, 179, 8, 0.2); }
.n-badge--danger  { background: var(--danger-muted);   color: var(--danger);         border: 1px solid rgba(239, 68, 68, 0.2); }
.n-badge--info    { background: var(--info-muted);     color: var(--info);           border: 1px solid rgba(59, 130, 246, 0.2); }
.n-badge--neutral { background: var(--bg-tertiary);    color: var(--text-secondary); border: 1px solid var(--border); }
.n-badge--default { background: var(--bg-elevated);    color: var(--text-secondary); border: 1px solid var(--border-hover); }
</style>
