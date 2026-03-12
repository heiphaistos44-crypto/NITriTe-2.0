<script setup lang="ts">
defineProps<{
  width?: string;
  height?: string;
  rounded?: boolean;
  circle?: boolean;
  lines?: number;
}>();
</script>

<template>
  <div v-if="lines && lines > 1" class="skeleton-lines">
    <div
      v-for="i in lines"
      :key="i"
      class="skeleton"
      :style="{ width: i === lines ? '70%' : '100%', height: height || '14px' }"
    />
  </div>
  <div
    v-else
    class="skeleton"
    :class="{ 'skeleton-circle': circle, 'skeleton-rounded': rounded }"
    :style="{
      width: circle ? (height || '40px') : (width || '100%'),
      height: height || '14px',
    }"
  />
</template>

<style scoped>
.skeleton {
  background: linear-gradient(
    90deg,
    var(--bg-tertiary) 25%,
    color-mix(in srgb, var(--bg-tertiary) 70%, var(--border)) 50%,
    var(--bg-tertiary) 75%
  );
  background-size: 400% 100%;
  animation: shimmer 1.4s ease-in-out infinite;
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}
.skeleton-circle { border-radius: 50%; }
.skeleton-rounded { border-radius: var(--radius-xl); }
.skeleton-lines { display: flex; flex-direction: column; gap: 8px; width: 100%; }
@keyframes shimmer {
  0% { background-position: 100% 50%; }
  100% { background-position: -100% 50%; }
}
</style>
