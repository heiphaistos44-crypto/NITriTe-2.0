<script setup lang="ts">
import { Construction, Search, FileQuestion, Inbox } from "lucide-vue-next";

const props = withDefaults(defineProps<{
  title?: string;
  description?: string;
  variant?: "empty" | "search" | "construction" | "error";
}>(), {
  title: "Rien a afficher",
  description: "Aucun element trouve",
  variant: "empty",
});

const iconMap = {
  empty: Inbox,
  search: Search,
  construction: Construction,
  error: FileQuestion,
};
</script>

<template>
  <div class="empty-state">
    <div class="empty-icon" :class="`icon-${variant}`">
      <component :is="iconMap[variant]" :size="40" />
    </div>
    <h3 class="empty-title">{{ title }}</h3>
    <p class="empty-desc">{{ description }}</p>
    <div v-if="$slots.default" class="empty-action">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 48px 24px;
  text-align: center;
}

.empty-icon {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 4px;
}

.icon-empty { background: var(--bg-tertiary); color: var(--text-muted); }
.icon-search { background: var(--info-muted); color: var(--info); }
.icon-construction { background: var(--warning-muted); color: var(--warning); }
.icon-error { background: var(--danger-muted); color: var(--danger); }

.empty-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-desc {
  font-size: 13px;
  color: var(--text-muted);
  max-width: 320px;
  line-height: 1.5;
}

.empty-action {
  margin-top: 8px;
}
</style>
