<script setup lang="ts">
import { computed } from "vue";
import { Loader2 } from "lucide-vue-next";

const props = withDefaults(
  defineProps<{
    variant?: "primary" | "secondary" | "ghost" | "danger" | "success";
    size?: "sm" | "md" | "lg";
    loading?: boolean;
    disabled?: boolean;
    fullWidth?: boolean;
  }>(),
  {
    variant: "primary",
    size: "md",
    loading: false,
    disabled: false,
    fullWidth: false,
  }
);

defineEmits<{ click: [e: MouseEvent] }>();

const classes = computed(() => [
  "n-btn",
  `n-btn--${props.variant}`,
  `n-btn--${props.size}`,
  { "n-btn--full": props.fullWidth, "n-btn--loading": props.loading },
]);
</script>

<template>
  <button
    :class="classes"
    :disabled="disabled || loading"
    @click="$emit('click', $event)"
  >
    <Loader2 v-if="loading" :size="16" class="animate-spin" />
    <slot />
  </button>
</template>

<style scoped>
.n-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border: none;
  border-radius: var(--radius-md);
  font-family: inherit;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
}

.n-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Sizes */
.n-btn--sm { padding: 6px 12px; font-size: 12px; }
.n-btn--md { padding: 8px 16px; font-size: 13px; }
.n-btn--lg { padding: 10px 20px; font-size: 14px; }
.n-btn--full { width: 100%; }

/* Variants */
.n-btn--primary {
  background: var(--accent-primary);
  color: white;
}
.n-btn--primary:hover:not(:disabled) {
  background: var(--accent-hover);
  box-shadow: var(--accent-glow);
}

.n-btn--secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border);
}
.n-btn--secondary:hover:not(:disabled) {
  background: var(--bg-elevated);
  border-color: var(--border-hover);
}

.n-btn--ghost {
  background: transparent;
  color: var(--text-secondary);
}
.n-btn--ghost:hover:not(:disabled) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.n-btn--danger {
  background: var(--danger);
  color: white;
}
.n-btn--danger:hover:not(:disabled) {
  opacity: 0.9;
}

.n-btn--success {
  background: var(--success);
  color: white;
}
.n-btn--success:hover:not(:disabled) {
  opacity: 0.9;
}
</style>
