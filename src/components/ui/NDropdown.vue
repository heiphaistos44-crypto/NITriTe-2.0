<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { ChevronDown } from "lucide-vue-next";

const props = defineProps<{
  options: { value: string; label: string }[];
  modelValue?: string;
  placeholder?: string;
}>();

const emit = defineEmits<{ "update:modelValue": [v: string] }>();

const isOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

const selectedLabel = () => {
  const opt = props.options.find((o) => o.value === props.modelValue);
  return opt?.label ?? props.placeholder ?? "Selectionner...";
};

function toggle() { isOpen.value = !isOpen.value; }

function select(value: string) {
  emit("update:modelValue", value);
  isOpen.value = false;
}

function onClickOutside(e: MouseEvent) {
  if (dropdownRef.value && !dropdownRef.value.contains(e.target as Node)) {
    isOpen.value = false;
  }
}

onMounted(() => document.addEventListener("click", onClickOutside));
onUnmounted(() => document.removeEventListener("click", onClickOutside));
</script>

<template>
  <div class="n-dropdown" ref="dropdownRef">
    <button class="dropdown-trigger" @click="toggle">
      <span>{{ selectedLabel() }}</span>
      <ChevronDown :size="14" class="dropdown-chevron" :class="{ open: isOpen }" />
    </button>
    <Transition name="dropdown">
      <div v-if="isOpen" class="dropdown-menu">
        <button
          v-for="opt in options"
          :key="opt.value"
          class="dropdown-item"
          :class="{ active: opt.value === modelValue }"
          @click="select(opt.value)"
        >
          {{ opt.label }}
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.n-dropdown { position: relative; display: inline-flex; }

.dropdown-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  transition: border-color var(--transition-fast);
}

.dropdown-trigger:hover { border-color: var(--border-hover); }

.dropdown-chevron {
  transition: transform var(--transition-fast);
  color: var(--text-muted);
}

.dropdown-chevron.open { transform: rotate(180deg); }

.dropdown-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 100%;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  z-index: 500;
  overflow: hidden;
}

.dropdown-item {
  display: block;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  text-align: left;
  transition: all var(--transition-fast);
}

.dropdown-item:hover { background: var(--bg-tertiary); color: var(--text-primary); }
.dropdown-item.active { color: var(--accent-primary); background: var(--accent-muted); }

.dropdown-enter-active { animation: scale-in 150ms ease; }
.dropdown-leave-active { animation: scale-in 150ms ease reverse; }
</style>
