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
    glow?: boolean;
  }>(),
  { value: 0, max: 100, size: "md", color: "auto", indeterminate: false, showLabel: false, glow: false }
);

const percent = computed(() => Math.min(100, Math.max(0, (props.value / props.max) * 100)));

const barColor = computed(() => {
  if (props.color !== "auto") return `var(--${props.color === "accent" ? "accent-primary" : props.color})`;
  if (percent.value > 90) return "var(--danger)";
  if (percent.value > 75) return "var(--warning)";
  return "var(--accent-primary)";
});

const barStyle = computed(() => {
  const style: Record<string, string> = {
    width: props.indeterminate ? "30%" : `${percent.value}%`,
    background: barColor.value,
  };
  if (props.glow) {
    style.boxShadow = `0 0 8px ${barColor.value}, 0 0 4px ${barColor.value}`;
  }
  return style;
});
</script>

<template>
  <div class="n-progress" :class="[`n-progress--${size}`]">
    <div class="n-progress__track">
      <div
        class="n-progress__bar"
        :class="{ 'n-progress__bar--indeterminate': indeterminate }"
        :style="barStyle"
      />
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
  background: var(--bg-elevated);
  overflow: hidden;
  border: 1px solid var(--border);
  position: relative;
}

.n-progress--sm .n-progress__track { height: 5px; }
.n-progress--md .n-progress__track { height: 7px; }
.n-progress--lg .n-progress__track { height: 11px; }

.n-progress__bar {
  height: 100%;
  border-radius: 99px;
  transition: width 350ms cubic-bezier(0.4, 0, 0.2, 1);
}

.n-progress__bar--indeterminate {
  width: 30% !important;
  animation: progress-indeterminate 1.5s ease-in-out infinite;
  position: relative;
}

/* Shimmer on indeterminate */
.n-progress__bar--indeterminate::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(90deg, transparent, rgba(255,255,255,0.25), transparent);
  animation: shimmer-bar 1.5s ease-in-out infinite;
}

@keyframes shimmer-bar {
  0%   { transform: translateX(-100%); }
  100% { transform: translateX(200%); }
}

.n-progress__label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  font-family: "JetBrains Mono", monospace;
  min-width: 38px;
  text-align: right;
}
</style>
