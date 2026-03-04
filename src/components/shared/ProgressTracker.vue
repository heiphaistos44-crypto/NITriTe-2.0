<script setup lang="ts">
import { CheckCircle, Circle, Loader } from "lucide-vue-next";

export interface Step {
  label: string;
  status: "pending" | "active" | "done" | "error";
}

defineProps<{ steps: Step[] }>();
</script>

<template>
  <div class="progress-tracker">
    <div v-for="(step, i) in steps" :key="i" class="step" :class="`step-${step.status}`">
      <div class="step-indicator">
        <CheckCircle v-if="step.status === 'done'" :size="18" />
        <Loader v-else-if="step.status === 'active'" :size="18" class="spin" />
        <Circle v-else :size="18" />
      </div>
      <span class="step-label">{{ step.label }}</span>
      <div v-if="i < steps.length - 1" class="step-line" :class="{ completed: step.status === 'done' }"></div>
    </div>
  </div>
</template>

<style scoped>
.progress-tracker {
  display: flex;
  align-items: center;
  gap: 0;
  padding: 8px 0;
}

.step {
  display: flex;
  align-items: center;
  gap: 6px;
  position: relative;
}

.step-indicator {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.step-pending { color: var(--text-muted); }
.step-active { color: var(--accent-primary); }
.step-done { color: var(--success); }
.step-error { color: var(--danger); }

.step-label {
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
}

.step-line {
  width: 24px;
  height: 2px;
  background: var(--border);
  margin: 0 6px;
  flex-shrink: 0;
}

.step-line.completed { background: var(--success); }

.spin { animation: spin 1s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
