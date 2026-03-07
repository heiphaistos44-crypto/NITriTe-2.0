<script setup lang="ts">
withDefaults(
  defineProps<{
    hoverable?: boolean;
    padding?: "none" | "sm" | "md" | "lg";
  }>(),
  { hoverable: false, padding: "md" }
);
</script>

<template>
  <div class="n-card" :class="[`n-card--p-${padding}`, { 'n-card--hoverable': hoverable }]">
    <div v-if="$slots.header" class="n-card__header">
      <slot name="header" />
    </div>
    <div class="n-card__body">
      <slot />
    </div>
    <div v-if="$slots.footer" class="n-card__footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<style scoped>
.n-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-xl);
  transition: all var(--transition-normal);
  overflow: hidden;
}

.n-card--hoverable:hover {
  border-color: var(--border-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg), 0 0 0 1px var(--accent-muted);
}

.n-card--p-none .n-card__body { padding: 0; }
.n-card--p-sm .n-card__body { padding: 12px; }
.n-card--p-md .n-card__body { padding: 18px; }
.n-card--p-lg .n-card__body { padding: 24px; }

.n-card__header {
  padding: 14px 18px;
  border-bottom: 1px solid var(--border);
  font-weight: 600;
  font-size: 14px;
  background: linear-gradient(135deg, var(--bg-tertiary), var(--bg-secondary));
  display: flex;
  align-items: center;
}

.n-card__footer {
  padding: 12px 18px;
  border-top: 1px solid var(--border);
  background: var(--bg-tertiary);
}
</style>
