<script setup lang="ts">
import { ref, watch, onMounted, type Component } from "vue";
import { ChevronDown } from "lucide-vue-next";

const props = withDefaults(defineProps<{
  title?: string;
  storageKey?: string;
  defaultOpen?: boolean;
  icon?: Component;
  count?: number;
  variant?: "default" | "card" | "subtle";
  disabled?: boolean;
}>(), {
  defaultOpen: true,
  variant: "default",
  disabled: false,
});

const emit = defineEmits<{ toggle: [isOpen: boolean] }>();

const isOpen = ref(props.defaultOpen);

onMounted(() => {
  if (props.storageKey) {
    const saved = localStorage.getItem(props.storageKey);
    if (saved !== null) isOpen.value = saved === "true";
  }
});

watch(isOpen, (val) => {
  if (props.storageKey) localStorage.setItem(props.storageKey, String(val));
  emit("toggle", val);
});

function toggle() {
  if (!props.disabled) isOpen.value = !isOpen.value;
}

defineExpose({ isOpen, toggle });
</script>

<template>
  <div class="ncollapse" :class="[`ncollapse--${variant}`, { 'ncollapse--open': isOpen, 'ncollapse--disabled': disabled }]">
    <button class="ncollapse-header" @click="toggle" :disabled="disabled" type="button">
      <div class="ncollapse-header-left">
        <component v-if="icon" :is="icon" :size="14" class="ncollapse-icon" />
        <span class="ncollapse-title">{{ title }}</span>
        <span v-if="count !== undefined" class="ncollapse-count">{{ count }}</span>
      </div>
      <div class="ncollapse-header-right">
        <slot name="header-extra" />
        <ChevronDown :size="14" class="ncollapse-chevron" :class="{ rotated: !isOpen }" />
      </div>
    </button>
    <transition name="ncollapse-anim">
      <div v-show="isOpen" class="ncollapse-body">
        <slot />
      </div>
    </transition>
  </div>
</template>

<style scoped>
.ncollapse { display: flex; flex-direction: column; }

/* Header */
.ncollapse-header {
  display: flex; align-items: center; justify-content: space-between;
  width: 100%; padding: 9px 12px;
  background: transparent; border: none; cursor: pointer;
  font-family: inherit; border-radius: var(--radius-md);
  transition: background var(--transition-fast);
  user-select: none;
}
.ncollapse-header:hover { background: var(--bg-tertiary); }
.ncollapse-header:disabled { cursor: default; opacity: .5; }
.ncollapse-header-left { display: flex; align-items: center; gap: 8px; }
.ncollapse-header-right { display: flex; align-items: center; gap: 6px; }
.ncollapse-title {
  font-size: 12px; font-weight: 700; text-transform: uppercase;
  letter-spacing: .07em; color: var(--text-secondary);
}
.ncollapse-icon { color: var(--text-muted); flex-shrink: 0; }
.ncollapse-count {
  font-size: 10px; font-weight: 600; color: var(--text-muted);
  background: var(--bg-elevated); border: 1px solid var(--border);
  border-radius: 99px; padding: 1px 6px; line-height: 1.4;
}
.ncollapse-chevron {
  color: var(--text-muted); transition: transform var(--transition-normal); flex-shrink: 0;
}
.ncollapse-chevron.rotated { transform: rotate(-90deg); }

/* Body */
.ncollapse-body { overflow: hidden; }

/* Variants */
.ncollapse--card > .ncollapse-header {
  border-bottom: 1px solid var(--border);
  background: var(--bg-secondary);
  border-radius: var(--radius-md) var(--radius-md) 0 0;
  padding: 10px 14px;
}
.ncollapse--card { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-md); }
.ncollapse--card.ncollapse--open > .ncollapse-header { border-bottom: 1px solid var(--border); }
.ncollapse--card:not(.ncollapse--open) > .ncollapse-header { border-radius: var(--radius-md); border-bottom: none; }
.ncollapse--card > .ncollapse-body { padding: 12px 14px; }

.ncollapse--subtle > .ncollapse-header { padding: 6px 8px; }
.ncollapse--subtle .ncollapse-title { font-size: 11px; }

/* Animation */
.ncollapse-anim-enter-active, .ncollapse-anim-leave-active {
  transition: max-height 220ms ease, opacity 200ms ease;
  max-height: 2000px; opacity: 1;
}
.ncollapse-anim-enter-from, .ncollapse-anim-leave-to {
  max-height: 0; opacity: 0;
}
</style>
