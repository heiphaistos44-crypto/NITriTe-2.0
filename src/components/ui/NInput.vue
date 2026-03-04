<script setup lang="ts">
defineProps<{
  modelValue?: string;
  label?: string;
  placeholder?: string;
  type?: string;
  error?: string;
  disabled?: boolean;
}>();

defineEmits<{ "update:modelValue": [v: string] }>();
</script>

<template>
  <div class="n-input-group">
    <label v-if="label" class="n-input-label">{{ label }}</label>
    <input
      class="n-input"
      :class="{ 'n-input--error': error }"
      :type="type ?? 'text'"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
    <span v-if="error" class="n-input-error">{{ error }}</span>
  </div>
</template>

<style scoped>
.n-input-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.n-input-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.n-input {
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 13px;
  outline: none;
  transition: border-color var(--transition-fast);
}

.n-input:focus { border-color: var(--accent-primary); }
.n-input--error { border-color: var(--danger); }
.n-input:disabled { opacity: 0.5; cursor: not-allowed; }
.n-input::placeholder { color: var(--text-muted); }

.n-input-error {
  font-size: 12px;
  color: var(--danger);
}
</style>
