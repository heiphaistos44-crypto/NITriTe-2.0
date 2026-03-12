<script setup lang="ts">
withDefaults(
  defineProps<{
    hoverable?: boolean;
    padding?: "none" | "sm" | "md" | "lg";
    variant?: "default" | "glass" | "premium" | "inset";
  }>(),
  { hoverable: false, padding: "md", variant: "default" }
);
</script>

<template>
  <div
    class="n-card"
    :class="[
      `n-card--p-${padding}`,
      `n-card--${variant}`,
      { 'n-card--hoverable': hoverable },
    ]"
  >
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

/* Variants */
.n-card--glass {
  background: var(--surface-glass);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border-color: rgba(255, 255, 255, 0.08);
}

.n-card--premium {
  border-color: rgba(249, 115, 22, 0.28);
  box-shadow: 0 0 0 1px rgba(249, 115, 22, 0.06), var(--shadow-lg);
}

.n-card--premium .n-card__header {
  background: linear-gradient(135deg, rgba(249, 115, 22, 0.10) 0%, var(--bg-tertiary) 100%);
  border-bottom-color: rgba(249, 115, 22, 0.18);
}

.n-card--inset {
  background: var(--bg-primary);
  box-shadow: var(--shadow-inner);
  border-color: var(--border);
}

/* Hoverable */
.n-card--hoverable {
  cursor: pointer;
}
.n-card--hoverable:hover {
  border-color: var(--border-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg), 0 0 0 1px var(--accent-muted);
}
.n-card--premium.n-card--hoverable:hover {
  border-color: rgba(249, 115, 22, 0.45);
  box-shadow: var(--accent-glow), var(--shadow-lg);
}

/* Padding */
.n-card--p-none .n-card__body { padding: 0; }
.n-card--p-sm  .n-card__body  { padding: 12px; }
.n-card--p-md  .n-card__body  { padding: 18px; }
.n-card--p-lg  .n-card__body  { padding: 24px; }

/* Header */
.n-card__header {
  padding: 14px 18px;
  border-bottom: 1px solid var(--border);
  font-weight: 600;
  font-size: 14px;
  background: linear-gradient(135deg, var(--bg-tertiary) 0%, var(--bg-secondary) 100%);
  display: flex;
  align-items: center;
}

/* Footer */
.n-card__footer {
  padding: 12px 18px;
  border-top: 1px solid var(--border);
  background: var(--bg-tertiary);
}
</style>
