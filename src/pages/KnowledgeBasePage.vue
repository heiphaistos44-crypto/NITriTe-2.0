<script setup lang="ts">
import { ref, computed, markRaw } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import { knowledgeBase } from "@/data/knowledgeBase";
import type { KBCategory } from "@/data/knowledgeBase";
import {
  BookOpen, Wifi, Zap, Shield, HardDrive,
  Monitor, Volume2, Usb, ChevronDown, ChevronRight,
  Terminal, Copy, AlertTriangle, Settings, Cpu,
  CheckCircle,
} from "lucide-vue-next";

// Map icon name → composant
const iconMap: Record<string, any> = {
  Wifi:          markRaw(Wifi),
  Zap:           markRaw(Zap),
  Shield:        markRaw(Shield),
  HardDrive:     markRaw(HardDrive),
  Monitor:       markRaw(Monitor),
  Volume2:       markRaw(Volume2),
  Usb:           markRaw(Usb),
  AlertTriangle: markRaw(AlertTriangle),
  Settings:      markRaw(Settings),
  Cpu:           markRaw(Cpu),
  Terminal:      markRaw(Terminal),
  BookOpen:      markRaw(BookOpen),
  CheckCircle:   markRaw(CheckCircle),
};

// Enrichissement : ajouter les composants icône à la volée
const categories = computed<(KBCategory & { iconComponent: any })[]>(() =>
  knowledgeBase.map((cat) => ({
    ...cat,
    iconComponent: iconMap[cat.icon] ?? iconMap.Settings,
  }))
);

const search             = ref("");
const expandedCategory   = ref<string | null>(null);
const expandedItem       = ref<string | null>(null);
const copiedCmd          = ref<string | null>(null);

function toggleCategory(id: string) {
  expandedCategory.value = expandedCategory.value === id ? null : id;
  expandedItem.value = null;
}

function toggleItem(title: string) {
  expandedItem.value = expandedItem.value === title ? null : title;
}

const filteredCategories = computed(() => {
  const cats = categories.value;
  if (!search.value.trim()) return cats;
  const q = search.value.toLowerCase();
  return cats
    .map((cat) => ({
      ...cat,
      items: cat.items.filter(
        (item) =>
          item.title.toLowerCase().includes(q) ||
          (item.symptoms ?? "").toLowerCase().includes(q) ||
          (item.solution ?? []).some((s) => s.toLowerCase().includes(q)) ||
          (item.code ?? "").toLowerCase().includes(q)
      ),
    }))
    .filter((cat) => cat.items.length > 0);
});

// Compteurs globaux
const totalCategories = computed(() => knowledgeBase.length);
const totalArticles   = computed(() => knowledgeBase.reduce((n, c) => n + c.items.length, 0));

async function runCommand(cmd: string) {
  try {
    await invoke("run_system_command", { cmd: "cmd", args: ["/C", cmd] });
  } catch { /* dev */ }
}

async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    copiedCmd.value = text;
    setTimeout(() => { copiedCmd.value = null; }, 2000);
  } catch { /* dev */ }
}
</script>

<template>
  <div class="kb-page">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1><BookOpen :size="22" /> Base de Connaissances</h1>
        <p class="page-subtitle">{{ totalCategories }} catégories · {{ totalArticles }} articles · Solutions aux problèmes courants</p>
      </div>
    </div>

    <!-- Recherche -->
    <NSearchBar v-model="search" placeholder="Rechercher un problème, une commande, un symptôme…" />

    <!-- Grille de catégories -->
    <div class="categories">
      <div v-for="cat in filteredCategories" :key="cat.id" class="category">
        <!-- En-tête catégorie -->
        <button class="cat-header" @click="toggleCategory(cat.id)">
          <component :is="cat.iconComponent" :size="18" class="cat-icon" />
          <span class="cat-label">{{ cat.label }}</span>
          <span class="cat-count">{{ cat.items.length }} article{{ cat.items.length > 1 ? 's' : '' }}</span>
          <ChevronDown v-if="expandedCategory === cat.id" :size="16" class="chevron" />
          <ChevronRight v-else :size="16" class="chevron" />
        </button>

        <!-- Liste des articles -->
        <div v-if="expandedCategory === cat.id" class="cat-items">
          <div v-for="item in cat.items" :key="item.title" class="kb-item">
            <!-- En-tête article -->
            <button class="item-header" @click="toggleItem(item.title)">
              <span class="item-title">{{ item.title }}</span>
              <ChevronDown v-if="expandedItem === item.title" :size="14" />
              <ChevronRight v-else :size="14" />
            </button>

            <!-- Contenu article -->
            <div v-if="expandedItem === item.title" class="item-content">
              <!-- Symptômes -->
              <div v-if="item.symptoms" class="symptoms">
                <AlertTriangle :size="13" style="color: var(--warning); flex-shrink:0" />
                <span><strong>Symptômes :</strong> {{ item.symptoms }}</span>
              </div>

              <!-- Solutions -->
              <div class="solution">
                <strong>Solution :</strong>
                <ol>
                  <li v-for="(step, i) in item.solution" :key="i">{{ step }}</li>
                </ol>
              </div>

              <!-- Bloc code -->
              <div v-if="item.code" class="code-block">
                <div class="code-header">
                  <Terminal :size="12" />
                  <span>Script / Commandes</span>
                  <button class="copy-btn-sm" @click="copyToClipboard(item.code!)" title="Copier le code">
                    <Copy :size="11" />
                    {{ copiedCmd === item.code ? 'Copié !' : 'Copier' }}
                  </button>
                </div>
                <pre class="code-content">{{ item.code }}</pre>
              </div>

              <!-- Commande rapide -->
              <div v-if="item.command" class="command-block">
                <code>{{ item.command }}</code>
                <button
                  class="copy-btn"
                  :class="{ copied: copiedCmd === item.command }"
                  @click="copyToClipboard(item.command!)"
                  title="Copier"
                >
                  <Copy :size="12" />
                </button>
                <button class="run-btn" @click="runCommand(item.command!)">
                  <Terminal :size="12" /> Exécuter
                </button>
              </div>

              <!-- Note -->
              <div v-if="item.note" class="note-block">
                💡 {{ item.note }}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Aucun résultat -->
      <div v-if="filteredCategories.length === 0" class="empty-state">
        <BookOpen :size="36" style="opacity: 0.2" />
        <p>Aucun résultat pour <strong>"{{ search }}"</strong></p>
        <p class="empty-hint">Essayez un terme plus général (ex: "réseau", "lent", "BSOD")</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.kb-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header h1 {
  font-size: 22px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 10px;
}

.page-subtitle {
  color: var(--text-muted);
  font-size: 13px;
  margin-top: 4px;
}

/* ── Catégories ── */
.categories { display: flex; flex-direction: column; gap: 6px; }

.category {
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--bg-secondary);
  transition: border-color 0.15s;
}
.category:has(.cat-header:hover) { border-color: var(--border-hover); }

.cat-header {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 14px 16px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  transition: background 0.15s;
}
.cat-header:hover { background: var(--bg-tertiary); }

.cat-icon { color: var(--accent-primary); flex-shrink: 0; }
.cat-count {
  margin-left: auto;
  font-size: 11px;
  color: var(--text-muted);
  font-weight: 400;
  background: var(--bg-tertiary);
  padding: 2px 8px;
  border-radius: 99px;
  border: 1px solid var(--border);
}
.chevron { color: var(--text-muted); flex-shrink: 0; }

/* ── Articles ── */
.cat-items { border-top: 1px solid var(--border); }

.kb-item { border-bottom: 1px solid var(--border); }
.kb-item:last-child { border-bottom: none; }

.item-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 11px 16px 11px 44px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  color: var(--text-secondary);
  transition: background 0.15s, color 0.15s;
  text-align: left;
  gap: 8px;
}
.item-header:hover { background: var(--bg-tertiary); color: var(--text-primary); }

.item-title { flex: 1; text-align: left; }

/* ── Contenu ── */
.item-content {
  padding: 0 16px 16px 44px;
  font-size: 13px;
  color: var(--text-secondary);
  display: flex;
  flex-direction: column;
  gap: 10px;
  animation: fade-in-up 200ms ease forwards;
}

@keyframes fade-in-up {
  from { opacity: 0; transform: translateY(4px); }
  to   { opacity: 1; transform: translateY(0); }
}

.symptoms {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 8px 12px;
  background: rgba(234, 179, 8, 0.07);
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--warning);
  font-size: 12px;
  color: var(--text-secondary);
}

.solution ol {
  margin: 6px 0 0 18px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  line-height: 1.5;
}

/* ── Bloc code (scripts multi-lignes) ── */
.code-block {
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--bg-primary);
}
.code-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border);
  font-size: 11px;
  color: var(--text-muted);
  font-weight: 500;
}
.copy-btn-sm {
  margin-left: auto;
  display: flex; align-items: center; gap: 4px;
  padding: 3px 8px;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-muted);
  cursor: pointer;
  font-family: inherit;
  font-size: 10px;
  transition: all 0.15s;
}
.copy-btn-sm:hover { border-color: var(--accent-primary); color: var(--accent-primary); }
.code-content {
  margin: 0;
  padding: 12px 14px;
  font-family: "JetBrains Mono", "Courier New", monospace;
  font-size: 11px;
  color: var(--text-secondary);
  white-space: pre;
  overflow-x: auto;
  line-height: 1.6;
  scrollbar-width: thin;
  scrollbar-color: var(--border) transparent;
  max-height: 320px;
  overflow-y: auto;
}

/* ── Commande rapide ── */
.command-block {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
  flex-wrap: wrap;
}
.command-block code {
  font-family: "JetBrains Mono", monospace;
  font-size: 11px;
  color: var(--accent-primary);
  flex: 1;
  min-width: 0;
  word-break: break-all;
}
.copy-btn {
  display: flex; align-items: center; justify-content: center;
  padding: 4px 8px;
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: var(--bg-tertiary);
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.15s;
  flex-shrink: 0;
}
.copy-btn:hover, .copy-btn.copied { border-color: var(--accent-primary); color: var(--accent-primary); }

.run-btn {
  display: flex; align-items: center; gap: 4px;
  padding: 4px 10px;
  border: 1px solid var(--accent-primary);
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--accent-primary);
  cursor: pointer;
  font-family: inherit;
  font-size: 11px;
  transition: all 0.15s;
  flex-shrink: 0;
}
.run-btn:hover { background: var(--accent-primary); color: #fff; }

/* ── Note ── */
.note-block {
  padding: 8px 12px;
  background: rgba(249, 115, 22, 0.07);
  border-left: 3px solid var(--accent-primary);
  border-radius: var(--radius-sm);
  font-size: 12px;
  color: var(--text-secondary);
}

/* ── État vide ── */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 48px;
  color: var(--text-muted);
  font-size: 14px;
  text-align: center;
}
.empty-hint { font-size: 12px; color: var(--text-muted); opacity: 0.7; }
</style>
