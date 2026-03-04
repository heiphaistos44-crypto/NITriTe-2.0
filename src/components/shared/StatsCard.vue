<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  title: string;
  subtitle?: string;
  value: string | number;
  icon: any;
  color?: "accent" | "success" | "warning" | "danger" | "info";
  progress?: number;
  trend?: { value: number; label: string };
}>();

const colorVar = computed(() => {
  const map: Record<string, string> = {
    accent: "var(--accent-primary)",
    success: "var(--success)",
    warning: "var(--warning)",
    danger: "var(--danger)",
    info: "var(--info)",
  };
  return map[props.color ?? "accent"];
});

const colorMuted = computed(() => {
  const map: Record<string, string> = {
    accent: "var(--accent-muted)",
    success: "var(--success-muted)",
    warning: "var(--warning-muted)",
    danger: "var(--danger-muted)",
    info: "var(--info-muted)",
  };
  return map[props.color ?? "accent"];
});

const progressColor = computed(() => {
  if (props.progress === undefined) return "";
  if (props.progress > 90) return "var(--danger)";
  if (props.progress > 75) return "var(--warning)";
  return colorVar.value;
});
</script>

<template>
  <div class="stats-card">
    <div class="stats-header">
      <div class="stats-icon-wrap" :style="{ background: colorMuted }">
        <component :is="icon" :size="20" :style="{ color: colorVar }" />
      </div>
      <div class="stats-info">
        <div class="stats-title">{{ title }}</div>
        <div v-if="subtitle" class="stats-subtitle">{{ subtitle }}</div>
      </div>
    </div>
    <div class="stats-value">{{ value }}</div>
    <div v-if="progress !== undefined" class="stats-progress">
      <div class="stats-progress-track">
        <div class="stats-progress-bar" :style="{ width: `${Math.min(100, progress)}%`, background: progressColor }" />
      </div>
    </div>
    <div v-if="trend" class="stats-trend" :class="{ 'trend-up': trend.value > 0, 'trend-down': trend.value < 0 }">
      <span>{{ trend.value > 0 ? "+" : "" }}{{ trend.value }}%</span>
      <span class="trend-label">{{ trend.label }}</span>
    </div>
  </div>
</template>

<style scoped>
.stats-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 16px;
  transition: all var(--transition-normal);
}

.stats-card:hover {
  border-color: var(--border-hover);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.stats-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 12px;
}

.stats-icon-wrap {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stats-info { min-width: 0; }

.stats-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.stats-subtitle {
  font-size: 11px;
  color: var(--text-muted);
}

.stats-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  font-family: "JetBrains Mono", monospace;
  line-height: 1.2;
}

.stats-progress {
  margin-top: 10px;
}

.stats-progress-track {
  height: 4px;
  border-radius: 99px;
  background: var(--bg-tertiary);
  overflow: hidden;
}

.stats-progress-bar {
  height: 100%;
  border-radius: 99px;
  transition: width 500ms ease;
}

.stats-trend {
  margin-top: 8px;
  font-size: 11px;
  display: flex;
  align-items: center;
  gap: 4px;
}

.trend-up { color: var(--success); }
.trend-down { color: var(--danger); }
.trend-label { color: var(--text-muted); }
</style>
