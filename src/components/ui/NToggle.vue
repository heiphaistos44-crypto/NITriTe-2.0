<script setup lang="ts">
defineProps<{ modelValue: boolean; label?: string; disabled?: boolean }>();
defineEmits<{ "update:modelValue": [v: boolean] }>();
</script>

<template>
  <label class="n-toggle" :class="{ disabled }">
    <button
      class="toggle-track"
      :class="{ active: modelValue }"
      :disabled="disabled"
      role="switch"
      :aria-checked="modelValue"
      @click="$emit('update:modelValue', !modelValue)"
    >
      <span class="toggle-thumb" />
    </button>
    <span v-if="label" class="toggle-label">{{ label }}</span>
  </label>
</template>

<style scoped>
.n-toggle {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.n-toggle.disabled { opacity: 0.5; cursor: not-allowed; }

.toggle-track {
  width: 36px;
  height: 20px;
  border-radius: 10px;
  background: var(--bg-elevated);
  border: none;
  cursor: inherit;
  position: relative;
  transition: background var(--transition-fast);
  padding: 0;
}

.toggle-track.active {
  background: var(--accent-primary);
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: white;
  transition: transform var(--transition-fast);
}

.toggle-track.active .toggle-thumb {
  transform: translateX(16px);
}

.toggle-label {
  font-size: 13px;
  color: var(--text-secondary);
}
</style>
