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
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-hover));
  color: white;
  box-shadow: 0 2px 8px rgba(249,115,22,.25);
  letter-spacing: 0.01em;
}
.n-btn--primary:hover:not(:disabled) {
  filter: brightness(1.1);
  box-shadow: var(--accent-glow);
  transform: translateY(-1px);
}

.n-btn--secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border);
}
.n-btn--secondary:hover:not(:disabled) {
  background: var(--bg-elevated);
  border-color: var(--border-hover);
  transform: translateY(-1px);
}

.n-btn--ghost {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid transparent;
}
.n-btn--ghost:hover:not(:disabled) {
  background: var(--bg-tertiary);
  border-color: var(--border);
  color: var(--text-primary);
}

.n-btn--danger {
  background: linear-gradient(135deg, var(--danger), #dc2626);
  color: white;
  box-shadow: 0 2px 8px rgba(239,68,68,.25);
}
.n-btn--danger:hover:not(:disabled) {
  filter: brightness(1.1);
  transform: translateY(-1px);
}

.n-btn--success {
  background: linear-gradient(135deg, var(--success), #16a34a);
  color: white;
  box-shadow: 0 2px 8px rgba(34,197,94,.25);
}
.n-btn--success:hover:not(:disabled) {
  filter: brightness(1.1);
  transform: translateY(-1px);
}
</style>
