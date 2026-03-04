<script setup lang="ts">
import { Search, X } from "lucide-vue-next";

const props = withDefaults(
  defineProps<{ modelValue?: string; placeholder?: string }>(),
  { modelValue: "", placeholder: "Rechercher..." }
);

const emit = defineEmits<{ "update:modelValue": [v: string] }>();

function onInput(e: Event) {
  emit("update:modelValue", (e.target as HTMLInputElement).value);
}

function clear() {
  emit("update:modelValue", "");
}
</script>

<template>
  <div class="n-search">
    <Search :size="16" class="search-icon" />
    <input
      type="text"
      :value="modelValue"
      :placeholder="placeholder"
      @input="onInput"
      class="search-input"
    />
    <button v-if="modelValue" class="search-clear" @click="clear">
      <X :size="14" />
    </button>
  </div>
</template>

<style scoped>
.n-search {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  transition: border-color var(--transition-fast);
}

.n-search:focus-within {
  border-color: var(--accent-primary);
}

.search-icon { color: var(--text-muted); flex-shrink: 0; }

.search-input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 13px;
}

.search-input::placeholder { color: var(--text-muted); }

.search-clear {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px;
  border-radius: 4px;
  display: flex;
  transition: all var(--transition-fast);
}

.search-clear:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
}
</style>
