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
  border-radius: var(--radius-lg);
  transition: all var(--transition-normal);
}

.n-card--hoverable:hover {
  border-color: var(--border-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.n-card--p-none .n-card__body { padding: 0; }
.n-card--p-sm .n-card__body { padding: 12px; }
.n-card--p-md .n-card__body { padding: 16px; }
.n-card--p-lg .n-card__body { padding: 24px; }

.n-card__header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--border);
  font-weight: 500;
  font-size: 14px;
}

.n-card__footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border);
}
</style>
