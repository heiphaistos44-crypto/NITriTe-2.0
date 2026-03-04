<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import { navigationSections } from "@/data/navigation";
import { Search } from "lucide-vue-next";

const emit = defineEmits<{ "open-search": [] }>();
const route = useRoute();

const currentPage = computed(() => {
  for (const section of navigationSections) {
    for (const item of section.items) {
      if (item.route === route.path) {
        return { label: item.label, section: section.title };
      }
    }
  }
  if (route.path === "/settings") {
    return { label: "Parametres", section: "" };
  }
  return { label: "Tableau de bord", section: "Systeme" };
});
</script>

<template>
  <header class="app-header">
    <div class="breadcrumb">
      <span v-if="currentPage.section" class="breadcrumb-section">{{ currentPage.section }}</span>
      <span v-if="currentPage.section" class="breadcrumb-sep">/</span>
      <span class="breadcrumb-page">{{ currentPage.label }}</span>
    </div>
    <div class="header-actions">
      <button class="search-btn" title="Recherche globale (Ctrl+K)" @click="emit('open-search')">
        <Search :size="16" />
        <span class="search-label">Rechercher...</span>
        <kbd class="search-kbd">Ctrl+K</kbd>
      </button>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-primary);
  min-height: 48px;
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
}

.breadcrumb-section {
  color: var(--text-muted);
}

.breadcrumb-sep {
  color: var(--text-muted);
}

.breadcrumb-page {
  color: var(--text-primary);
  font-weight: 500;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.search-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-secondary);
  color: var(--text-muted);
  cursor: pointer;
  font-size: 13px;
  font-family: inherit;
  transition: all var(--transition-fast);
}

.search-btn:hover {
  border-color: var(--border-hover);
  color: var(--text-secondary);
}

.search-label {
  min-width: 100px;
  text-align: left;
}

.search-kbd {
  font-size: 10px;
  padding: 2px 5px;
  border: 1px solid var(--border);
  border-radius: 4px;
  background: var(--bg-tertiary);
  color: var(--text-muted);
  font-family: inherit;
}
</style>
