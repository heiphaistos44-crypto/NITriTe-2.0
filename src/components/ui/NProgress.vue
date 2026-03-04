<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    value?: number;
    max?: number;
    size?: "sm" | "md" | "lg";
    color?: "accent" | "success" | "warning" | "danger" | "auto";
    indeterminate?: boolean;
    showLabel?: boolean;
  }>(),
  { value: 0, max: 100, size: "md", color: "auto", indeterminate: false, showLabel: false }
);

const percent = computed(() => Math.min(100, Math.max(0, (props.value / props.max) * 100)));

const barColor = computed(() => {
  if (props.color !== "auto") return `var(--${props.color === "accent" ? "accent-primary" : props.color})`;
  if (percent.value > 90) return "var(--danger)";
  if (percent.value > 75) return "var(--warning)";
  return "var(--accent-primary)";
});
</script>

<template>
  <div class="n-progress" :class="[`n-progress--${size}`]">
    <div class="n-progress__track">
      <div
        v-if="!indeterminate"
        class="n-progress__bar"
        :style="{ width: `${percent}%`, background: barColor }"
      />
      <div v-else class="n-progress__bar n-progress__bar--indeterminate" :style="{ background: barColor }" />
    </div>
    <span v-if="showLabel" class="n-progress__label">{{ Math.round(percent) }}%</span>
  </div>
</template>

<style scoped>
.n-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.n-progress__track {
  flex: 1;
  border-radius: 99px;
  background: var(--bg-tertiary);
  overflow: hidden;
}

.n-progress--sm .n-progress__track { height: 4px; }
.n-progress--md .n-progress__track { height: 6px; }
.n-progress--lg .n-progress__track { height: 10px; }

.n-progress__bar {
  height: 100%;
  border-radius: 99px;
  transition: width 300ms ease;
}

.n-progress__bar--indeterminate {
  width: 30% !important;
  animation: progress-indeterminate 1.5s ease-in-out infinite;
}

.n-progress__label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  font-family: "JetBrains Mono", monospace;
  min-width: 36px;
  text-align: right;
}
</style>
