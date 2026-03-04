<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NTabs from "@/components/ui/NTabs.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Download, CheckSquare, Square, Package,
  Globe, Shield, Code, Image, MessageSquare,
  FileText, Music, Video, Wrench, RefreshCw,
  Cpu, HardDrive, Monitor, Printer, Archive,
  Bot, Users, Cloud, Star, Lock, Play,
  ChevronDown, ChevronRight,
} from "lucide-vue-next";

const notifications = useNotificationStore();
const search = ref("");
const installing = ref(false);
const currentApp = ref("");
const installProgress = ref(0);
const installTotal = ref(0);
const installIndex = ref(0);
const collapsedCategories = ref<Set<string>>(new Set());

interface AppItem {
  id: string;
  name: string;
  description: string;
  category: string;
  winget_id?: string | null;
  choco_id?: string | null;
  url?: string | null;
  icon?: string;
  checked: boolean;
  installed: boolean;
}

const apps = ref<AppItem[]>([]);

// Categories avec icônes — correspondent aux valeurs exactes de programs.json
const CATEGORIES: { id: string; label: string; icon: any }[] = [
  { id: "Outils Essentiels", label: "Outils Essentiels", icon: Star },
  { id: "Navigateurs", label: "Navigateurs", icon: Globe },
  { id: "Securite", label: "Sécurité", icon: Shield },
  { id: "Antivirus", label: "Antivirus", icon: Shield },
  { id: "Desinstallateurs Antivirus", label: "Désinstallateurs Antivirus", icon: Lock },
  { id: "Developpement", label: "Développement", icon: Code },
  { id: "Multimedia", label: "Multimédia", icon: Video },
  { id: "Streaming Video", label: "Streaming Vidéo", icon: Play },
  { id: "Streaming Audio", label: "Streaming Audio", icon: Music },
  { id: "Communication", label: "Communication", icon: MessageSquare },
  { id: "Reseaux Sociaux", label: "Réseaux Sociaux", icon: Users },
  { id: "Bureautique", label: "Bureautique", icon: FileText },
  { id: "PDF et Documents", label: "PDF & Documents", icon: FileText },
  { id: "Suites Professionnelles", label: "Suites Pro", icon: Cpu },
  { id: "Productivite", label: "Productivité", icon: CheckSquare },
  { id: "IA & Assistants", label: "IA & Assistants", icon: Bot },
  { id: "Utilitaires", label: "Utilitaires", icon: Wrench },
  { id: "Utilitaires Systeme", label: "Utilitaires Système", icon: Monitor },
  { id: "Stockage Cloud", label: "Stockage Cloud", icon: Cloud },
  { id: "Compression", label: "Compression", icon: Archive },
  { id: "Internet", label: "Internet", icon: Globe },
  { id: "Jeux", label: "Jeux", icon: Play },
  { id: "Imprimantes & Scan", label: "Imprimantes & Scan", icon: Printer },
  { id: "Services Apple", label: "Services Apple", icon: Package },
];

const categoryTabs = [
  { id: "all", label: "Tout" },
  ...CATEGORIES.map(c => ({ id: c.id, label: c.label })),
];

const activeCategory = ref("all");

function normalizeStr(s: string) {
  return s.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "");
}

const filteredApps = computed(() => {
  const q = search.value.toLowerCase();
  return apps.value.filter((a) => {
    const matchSearch = !q || a.name.toLowerCase().includes(q) || a.description.toLowerCase().includes(q);
    const matchCat = activeCategory.value === "all" || normalizeStr(a.category) === normalizeStr(activeCategory.value);
    return matchSearch && matchCat;
  });
});

const groupedApps = computed(() => {
  const groups: Record<string, AppItem[]> = {};
  const cats = activeCategory.value === "all"
    ? CATEGORIES.map(c => c.id)
    : [activeCategory.value];

  for (const catId of cats) {
    const items = filteredApps.value.filter((a) => normalizeStr(a.category) === normalizeStr(catId));
    if (items.length > 0) groups[catId] = items;
  }
  return groups;
});

const totalCount = computed(() => apps.value.length);
const selectedCount = computed(() => apps.value.filter((a) => a.checked).length);

function selectAll() {
  filteredApps.value.forEach((a) => (a.checked = true));
}

function deselectAll() {
  apps.value.forEach((a) => (a.checked = false));
}

function toggleApp(app: AppItem) {
  if (!app.installed) app.checked = !app.checked;
}

function toggleCategory(catId: string) {
  if (collapsedCategories.value.has(catId)) {
    collapsedCategories.value.delete(catId);
  } else {
    collapsedCategories.value.add(catId);
  }
  collapsedCategories.value = new Set(collapsedCategories.value);
}

function getCategoryInfo(id: string) {
  return CATEGORIES.find((c) => c.id === id) ?? { id, label: id, icon: Package };
}

function selectCategory(catId: string) {
  const catApps = apps.value.filter((a) => a.category === catId && !a.installed);
  const allChecked = catApps.every((a) => a.checked);
  catApps.forEach((a) => (a.checked = !allChecked));
}

async function installSelection() {
  const selected = apps.value.filter((a) => a.checked && !a.installed);
  if (selected.length === 0) {
    notifications.warning("Aucune application sélectionnée");
    return;
  }

  installing.value = true;
  installTotal.value = selected.length;
  installIndex.value = 0;

  for (const app of selected) {
    installIndex.value++;
    currentApp.value = app.name;
    installProgress.value = Math.round((installIndex.value / installTotal.value) * 100);

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<{ success: boolean; app_id: string; message: string }>("install_app", {
        appId: app.id,
        wingetId: app.winget_id ?? undefined,
      });
      if (!result.success) {
        notifications.warning(`${app.name}: ${result.message}`);
      } else {
        notifications.success(`${app.name} installé`);
      }
      app.installed = true;
      app.checked = false;
    } catch (e: any) {
      notifications.error(`Échec: ${app.name}`, e?.toString());
    }
  }

  installing.value = false;
  currentApp.value = "";
  installProgress.value = 0;
  notifications.success("Installation terminée", `${installIndex.value} application(s) traitée(s)`);
}

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const result = await invoke<any[]>("get_apps");
    apps.value = result.map((a: any) => ({ ...a, checked: false, installed: false }));
  } catch {
    notifications.warning("Impossible de charger la base de données");
  }
});
</script>

<template>
  <div class="master-install">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1>Master Install</h1>
        <p class="page-subtitle">
          Base de données de <strong>{{ totalCount }}</strong> applications — installation groupée via WinGet
        </p>
      </div>
      <div class="header-actions">
        <NButton variant="ghost" size="sm" @click="selectAll">
          <CheckSquare :size="14" />
          Tout sélectionner
        </NButton>
        <NButton variant="ghost" size="sm" @click="deselectAll">
          <Square :size="14" />
          Tout déselectionner
        </NButton>
        <NButton
          variant="primary"
          size="sm"
          :loading="installing"
          :disabled="selectedCount === 0"
          @click="installSelection"
        >
          <Download :size="14" />
          Installer ({{ selectedCount }})
        </NButton>
      </div>
    </div>

    <!-- Progress -->
    <NCard v-if="installing">
      <div class="install-progress">
        <div class="install-status">
          <Package :size="16" class="spin-icon" />
          <span>Installation de <strong>{{ currentApp }}</strong> ({{ installIndex }}/{{ installTotal }})</span>
        </div>
        <NProgress :value="installProgress" size="lg" showLabel />
      </div>
    </NCard>

    <!-- Search -->
    <NSearchBar v-model="search" placeholder="Rechercher une application..." />

    <!-- Category Tabs -->
    <NTabs :tabs="categoryTabs" v-model="activeCategory" wrap>
      <template #default>
        <div v-if="Object.keys(groupedApps).length === 0" class="empty-state">
          <Package :size="40" class="empty-icon" />
          <p>Aucune application trouvée</p>
        </div>

        <div v-else class="categories-list">
          <template v-for="(catApps, catId) in groupedApps" :key="catId">
            <NCard>
              <template #header>
                <div class="section-header" @click="toggleCategory(catId as string)">
                  <component :is="getCategoryInfo(catId as string).icon" :size="16" />
                  <span>{{ getCategoryInfo(catId as string).label }}</span>
                  <NBadge variant="neutral">{{ catApps.length }}</NBadge>
                  <span class="spacer" />
                  <button class="select-cat-btn" @click.stop="selectCategory(catId as string)">
                    Tout sélectionner
                  </button>
                  <component
                    :is="collapsedCategories.has(catId as string) ? ChevronRight : ChevronDown"
                    :size="14"
                    class="collapse-icon"
                  />
                </div>
              </template>

              <div v-if="!collapsedCategories.has(catId as string)" class="apps-grid">
                <div
                  v-for="app in catApps"
                  :key="app.id"
                  class="app-item"
                  :class="{ 'app-item--checked': app.checked, 'app-item--installed': app.installed }"
                  @click="toggleApp(app)"
                >
                  <div class="app-checkbox">
                    <CheckSquare v-if="app.checked || app.installed" :size="18" class="check-on" />
                    <Square v-else :size="18" class="check-off" />
                  </div>
                  <div class="app-info">
                    <span class="app-name">{{ app.name }}</span>
                    <span class="app-desc">{{ app.description }}</span>
                  </div>
                  <NBadge v-if="app.installed" variant="success">Installé</NBadge>
                  <NBadge v-else-if="app.winget_id" variant="info" class="winget-badge">WinGet</NBadge>
                  <NBadge v-else-if="app.url" variant="warning" class="winget-badge">URL</NBadge>
                </div>
              </div>
            </NCard>
          </template>
        </div>
      </template>
    </NTabs>
  </div>
</template>

<style scoped>
.master-install {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 12px;
}

.page-header h1 { font-size: 24px; font-weight: 700; }
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }
.header-actions { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; }

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.spacer { flex: 1; }

.select-cat-btn {
  font-size: 11px;
  color: var(--accent-primary);
  background: none;
  border: none;
  cursor: pointer;
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  transition: background var(--transition-fast);
}
.select-cat-btn:hover { background: var(--accent-muted); }

.collapse-icon { color: var(--text-muted); }

.install-progress {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.install-status {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-secondary);
}

.spin-icon { animation: spin 1s linear infinite; }

@keyframes spin { to { transform: rotate(360deg); } }

.categories-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.apps-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 4px;
}

.app-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.app-item:hover { background: var(--bg-tertiary); }
.app-item--checked { background: var(--accent-muted); }
.app-item--installed { opacity: 0.55; cursor: default; }

.app-checkbox { flex-shrink: 0; display: flex; }
.check-on { color: var(--accent-primary); }
.check-off { color: var(--text-muted); }

.app-info { display: flex; flex-direction: column; gap: 1px; flex: 1; min-width: 0; }

.app-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.app-desc {
  font-size: 11px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.winget-badge { flex-shrink: 0; font-size: 10px; }

.empty-state {
  text-align: center;
  padding: 48px;
  color: var(--text-muted);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.empty-icon { opacity: 0.3; }
</style>
