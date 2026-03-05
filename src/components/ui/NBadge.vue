<script setup lang="ts">
withDefaults(
  defineProps<{ variant?: "accent" | "success" | "warning" | "danger" | "info" | "neutral" | "default" }>(),
  { variant: "neutral" }
);
</script>

<template>
  <span class="n-badge" :class="`n-badge--${variant}`">
    <slot />
  </span>
</template>

<style scoped>
.n-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 99px;
  font-size: 11px;
  font-weight: 500;
  line-height: 1.4;
}

.n-badge--accent { background: var(--accent-muted); color: var(--accent-primary); }
.n-badge--success { background: var(--success-muted); color: var(--success); }
.n-badge--warning { background: var(--warning-muted); color: var(--warning); }
.n-badge--danger { background: var(--danger-muted); color: var(--danger); }
.n-badge--info { background: var(--info-muted); color: var(--info); }
.n-badge--neutral  { background: var(--bg-tertiary);  color: var(--text-secondary); }
.n-badge--default  { background: var(--bg-elevated);  color: var(--text-secondary); border: 1px solid var(--border-hover); }
</style>
