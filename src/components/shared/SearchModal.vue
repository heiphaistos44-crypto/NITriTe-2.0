<script setup lang="ts">
import { ref, computed, watch, nextTick } from "vue";
import { useRouter } from "vue-router";
import { navigationSections } from "@/data/navigation";
import { Search, ArrowRight, X } from "lucide-vue-next";

const props = defineProps<{ modelValue: boolean }>();
const emit = defineEmits<{ "update:modelValue": [v: boolean] }>();
const router = useRouter();

const query = ref("");
const inputEl = ref<HTMLInputElement | null>(null);
const selectedIndex = ref(0);

interface SearchResult {
  label: string;
  section: string;
  route: string;
  icon: string;
}

const allPages = computed<SearchResult[]>(() => {
  const pages: SearchResult[] = [];
  for (const section of navigationSections) {
    for (const item of section.items) {
      pages.push({
        label: item.label,
        section: section.title,
        route: item.route,
        icon: item.icon,
      });
    }
  }
  pages.push({ label: "Parametres", section: "Configuration", route: "/settings", icon: "settings" });
  return pages;
});

const results = computed(() => {
  if (!query.value.trim()) return allPages.value;
  const q = query.value.toLowerCase();
  return allPages.value.filter(
    (p) =>
      p.label.toLowerCase().includes(q) ||
      p.section.toLowerCase().includes(q)
  );
});

watch(() => props.modelValue, (open) => {
  if (open) {
    query.value = "";
    selectedIndex.value = 0;
    nextTick(() => inputEl.value?.focus());
  }
});

watch(query, () => {
  selectedIndex.value = 0;
});

function close() {
  emit("update:modelValue", false);
}

function navigate(route: string) {
  router.push(route);
  close();
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    close();
  } else if (e.key === "ArrowDown") {
    e.preventDefault();
    selectedIndex.value = Math.min(selectedIndex.value + 1, results.value.length - 1);
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
  } else if (e.key === "Enter") {
    e.preventDefault();
    if (results.value[selectedIndex.value]) {
      navigate(results.value[selectedIndex.value].route);
    }
  }
}
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="modelValue" class="search-overlay" @click.self="close">
        <div class="search-modal" @keydown="handleKeyDown">
          <div class="search-input-row">
            <Search :size="18" class="search-icon" />
            <input
              ref="inputEl"
              v-model="query"
              type="text"
              class="search-input"
              placeholder="Rechercher une page..."
              autocomplete="off"
            />
            <button class="close-btn" @click="close"><X :size="16" /></button>
          </div>

          <div class="search-results">
            <div v-if="results.length === 0" class="no-results">
              Aucun resultat pour "{{ query }}"
            </div>
            <button
              v-for="(item, i) in results"
              :key="item.route"
              class="result-item"
              :class="{ selected: i === selectedIndex }"
              @click="navigate(item.route)"
              @mouseenter="selectedIndex = i"
            >
              <div class="result-info">
                <span class="result-label">{{ item.label }}</span>
                <span class="result-section">{{ item.section }}</span>
              </div>
              <ArrowRight :size="14" class="result-arrow" />
            </button>
          </div>

          <div class="search-footer">
            <span><kbd>↑↓</kbd> naviguer</span>
            <span><kbd>↵</kbd> ouvrir</span>
            <span><kbd>esc</kbd> fermer</span>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.search-overlay {
  position: fixed;
  inset: 0;
  z-index: 9000;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  padding-top: 15vh;
}

.search-modal {
  width: 520px;
  max-height: 460px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.search-input-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 14px 16px;
  border-bottom: 1px solid var(--border);
}

.search-icon { color: var(--text-muted); flex-shrink: 0; }

.search-input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 15px;
}

.search-input::placeholder { color: var(--text-muted); }

.close-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.15s;
}
.close-btn:hover { background: var(--bg-tertiary); color: var(--text-primary); }

.search-results {
  flex: 1;
  overflow-y: auto;
  padding: 6px;
}

.no-results {
  text-align: center;
  padding: 24px;
  color: var(--text-muted);
  font-size: 13px;
}

.result-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 10px 14px;
  border: none;
  background: transparent;
  border-radius: var(--radius-md);
  cursor: pointer;
  font-family: inherit;
  text-align: left;
  transition: background 0.1s;
}

.result-item:hover,
.result-item.selected {
  background: var(--bg-tertiary);
}

.result-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.result-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.result-section {
  font-size: 11px;
  color: var(--text-muted);
}

.result-arrow {
  color: var(--text-muted);
  opacity: 0;
  transition: opacity 0.15s;
}

.result-item.selected .result-arrow,
.result-item:hover .result-arrow {
  opacity: 1;
  color: var(--accent-primary);
}

.search-footer {
  display: flex;
  gap: 16px;
  padding: 8px 16px;
  border-top: 1px solid var(--border);
  font-size: 11px;
  color: var(--text-muted);
}

.search-footer kbd {
  padding: 1px 5px;
  border: 1px solid var(--border);
  border-radius: 3px;
  background: var(--bg-tertiary);
  font-family: inherit;
  font-size: 10px;
  margin-right: 4px;
}

/* Transitions */
.modal-enter-active { transition: opacity 150ms ease; }
.modal-leave-active { transition: opacity 100ms ease; }
.modal-enter-from,
.modal-leave-to { opacity: 0; }
</style>
