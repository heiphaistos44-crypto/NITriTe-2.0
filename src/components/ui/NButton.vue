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
    <Loader2 v-if="loading" :size="14" class="animate-spin" />
    <slot />
  </button>
</template>

<style scoped>
.n-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 7px;
  border: none;
  border-radius: var(--radius-md);
  font-family: inherit;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  white-space: nowrap;
  position: relative;
  overflow: hidden;
  letter-spacing: 0.01em;
}

.n-btn:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

/* Sizes */
.n-btn--sm { padding: 5px 12px; font-size: 12px; }
.n-btn--md { padding: 8px 16px; font-size: 13px; }
.n-btn--lg { padding: 10px 22px; font-size: 14px; letter-spacing: 0.02em; }
.n-btn--full { width: 100%; }

/* === Primary === */
.n-btn--primary {
  background: linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-hover) 100%);
  color: white;
  box-shadow: 0 2px 10px rgba(249,115,22,.28), inset 0 1px 0 rgba(255,255,255,0.15);
}
/* Shine sweep on hover */
.n-btn--primary::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(105deg, transparent 40%, rgba(255,255,255,0.18) 50%, transparent 60%);
  transform: translateX(-100%);
  transition: transform 500ms ease;
}
.n-btn--primary:hover:not(:disabled)::after {
  transform: translateX(100%);
}
.n-btn--primary:hover:not(:disabled) {
  box-shadow: var(--accent-glow), 0 4px 14px rgba(249,115,22,.35);
  transform: translateY(-1px);
}
.n-btn--primary:active:not(:disabled) {
  transform: translateY(0);
  box-shadow: 0 1px 6px rgba(249,115,22,.2);
}

/* === Secondary === */
.n-btn--secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-hover);
}
.n-btn--secondary:hover:not(:disabled) {
  background: var(--bg-elevated);
  border-color: var(--border-strong);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}
.n-btn--secondary:active:not(:disabled) {
  transform: translateY(0);
}

/* === Ghost === */
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

/* === Danger === */
.n-btn--danger {
  background: linear-gradient(135deg, var(--danger), #dc2626);
  color: white;
  box-shadow: 0 2px 10px rgba(239,68,68,.22), inset 0 1px 0 rgba(255,255,255,0.1);
}
.n-btn--danger:hover:not(:disabled) {
  box-shadow: 0 0 16px rgba(239,68,68,.4), 0 4px 12px rgba(239,68,68,.3);
  transform: translateY(-1px);
}

/* === Success === */
.n-btn--success {
  background: linear-gradient(135deg, var(--success), #16a34a);
  color: white;
  box-shadow: 0 2px 10px rgba(34,197,94,.22), inset 0 1px 0 rgba(255,255,255,0.1);
}
.n-btn--success:hover:not(:disabled) {
  box-shadow: var(--success-glow), 0 4px 12px rgba(34,197,94,.3);
  transform: translateY(-1px);
}
</style>
