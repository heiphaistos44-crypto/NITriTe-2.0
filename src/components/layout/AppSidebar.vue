<script setup lang="ts">
import { ref, reactive, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { navigationSections } from "@/data/navigation";
import {
  LayoutDashboard, Stethoscope, Activity, Zap, LayoutGrid, Wrench,
  Download, Package, HardDrive, RefreshCw, Cpu, Scan, Save, Shield,
  Wifi, Terminal, FileCode, Bot, BookOpen, FileText, ScrollText,
  BarChart3, Settings, ChevronLeft, ChevronRight, ChevronDown,
} from "lucide-vue-next";

const props = defineProps<{ collapsed: boolean }>();
const emit = defineEmits<{ toggle: [] }>();

const route = useRoute();
const router = useRouter();

// Sections retractables — persistance localStorage
const collapsedSections = reactive<Record<string, boolean>>({});

function loadSectionStates() {
  try {
    const saved = localStorage.getItem("nitrite-sections");
    if (saved) {
      const parsed = JSON.parse(saved);
      Object.assign(collapsedSections, parsed);
    }
  } catch {}
}

function saveSectionStates() {
  localStorage.setItem("nitrite-sections", JSON.stringify(collapsedSections));
}

function toggleSection(title: string) {
  collapsedSections[title] = !collapsedSections[title];
  saveSectionStates();
}

function isSectionCollapsed(title: string) {
  return collapsedSections[title] === true;
}

// Auto-expand la section qui contient la page active
function expandActiveSection() {
  for (const section of navigationSections) {
    for (const item of section.items) {
      if (item.route === route.path) {
        collapsedSections[section.title] = false;
        saveSectionStates();
        return;
      }
    }
  }
}

loadSectionStates();

watch(() => route.path, expandActiveSection);

const iconMap: Record<string, any> = {
  "layout-dashboard": LayoutDashboard,
  stethoscope: Stethoscope,
  activity: Activity,
  zap: Zap,
  "layout-grid": LayoutGrid,
  wrench: Wrench,
  download: Download,
  package: Package,
  "hard-drive": HardDrive,
  "refresh-cw": RefreshCw,
  cpu: Cpu,
  scan: Scan,
  save: Save,
  shield: Shield,
  wifi: Wifi,
  terminal: Terminal,
  "file-code": FileCode,
  bot: Bot,
  "book-open": BookOpen,
  "file-text": FileText,
  "scroll-text": ScrollText,
  "bar-chart-3": BarChart3,
  settings: Settings,
};

function getIcon(name: string) {
  return iconMap[name] ?? LayoutDashboard;
}

function isActive(itemRoute: string) {
  return route.path === itemRoute;
}

function sectionHasActive(section: typeof navigationSections[0]) {
  return section.items.some((item) => item.route === route.path);
}

function navigate(itemRoute: string) {
  router.push(itemRoute);
}
</script>

<template>
  <aside class="sidebar" :class="{ collapsed }">
    <!-- Header -->
    <div class="sidebar-header">
      <div class="logo-area">
        <div class="logo-icon">N</div>
        <transition name="fade">
          <div v-if="!collapsed" class="logo-text">
            <span class="logo-title">NiTriTe</span>
            <span class="logo-version">v26.0</span>
          </div>
        </transition>
      </div>
      <button class="collapse-btn" @click="emit('toggle')" :title="collapsed ? 'Ouvrir' : 'Replier'">
        <ChevronLeft v-if="!collapsed" :size="16" />
        <ChevronRight v-else :size="16" />
      </button>
    </div>

    <!-- Navigation -->
    <nav class="sidebar-nav">
      <div v-for="section in navigationSections" :key="section.title" class="nav-section">
        <!-- Section header cliquable (retractable) -->
        <button
          v-if="!collapsed"
          class="section-header"
          :class="{ 'has-active': sectionHasActive(section) }"
          @click="toggleSection(section.title)"
        >
          <span class="section-title-text">{{ section.title }}</span>
          <ChevronDown
            :size="12"
            class="section-chevron"
            :class="{ rotated: isSectionCollapsed(section.title) }"
          />
        </button>
        <div v-else class="section-divider"></div>

        <!-- Items avec animation collapse -->
        <transition name="section-collapse">
          <div
            v-show="!isSectionCollapsed(section.title) || collapsed"
            class="section-items"
          >
            <button
              v-for="item in section.items"
              :key="item.id"
              class="nav-item"
              :class="{ active: isActive(item.route) }"
              @click="navigate(item.route)"
              :title="collapsed ? item.label : undefined"
            >
              <component :is="getIcon(item.icon)" :size="18" class="nav-icon" />
              <transition name="fade">
                <span v-if="!collapsed" class="nav-label">{{ item.label }}</span>
              </transition>
              <span v-if="item.badge && !collapsed" class="nav-badge">{{ item.badge }}</span>
            </button>
          </div>
        </transition>
      </div>
    </nav>

    <!-- Footer -->
    <div class="sidebar-footer">
      <button
        class="nav-item"
        :class="{ active: isActive('/settings') }"
        @click="navigate('/settings')"
        title="Parametres"
      >
        <Settings :size="18" class="nav-icon" />
        <transition name="fade">
          <span v-if="!collapsed" class="nav-label">Parametres</span>
        </transition>
      </button>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  position: fixed;
  left: 0;
  top: 0;
  bottom: 0;
  width: 240px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  z-index: 100;
  transition: width var(--transition-normal);
  overflow: hidden;
}

.sidebar.collapsed {
  width: 64px;
}

/* Header */
.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 12px;
  border-bottom: 1px solid var(--border);
  min-height: 64px;
}

.logo-area {
  display: flex;
  align-items: center;
  gap: 10px;
  overflow: hidden;
}

.logo-icon {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  background: var(--accent-primary);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 18px;
  flex-shrink: 0;
}

.logo-text {
  display: flex;
  flex-direction: column;
  white-space: nowrap;
}

.logo-title {
  font-weight: 700;
  font-size: 16px;
  color: var(--text-primary);
  line-height: 1.2;
}

.logo-version {
  font-size: 11px;
  color: var(--text-muted);
}

.collapse-btn {
  width: 28px;
  height: 28px;
  border-radius: var(--radius-sm);
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  transition: all var(--transition-fast);
}

.collapse-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

/* Navigation */
.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 8px;
}

.nav-section {
  margin-bottom: 2px;
}

/* Section header retractable */
.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 10px 8px 5px;
  border: none;
  background: transparent;
  cursor: pointer;
  transition: all var(--transition-fast);
  border-radius: var(--radius-sm);
}

.section-header:hover {
  background: var(--bg-tertiary);
}

.section-header:hover .section-title-text {
  color: var(--text-secondary);
}

.section-header.has-active .section-title-text {
  color: var(--accent-primary);
}

.section-title-text {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  white-space: nowrap;
  transition: color var(--transition-fast);
}

.section-chevron {
  color: var(--text-muted);
  transition: transform var(--transition-normal);
  flex-shrink: 0;
}

.section-chevron.rotated {
  transform: rotate(-90deg);
}

.section-divider {
  height: 1px;
  background: var(--border);
  margin: 8px 8px;
}

/* Section items container */
.section-items {
  overflow: hidden;
}

/* Transition collapse */
.section-collapse-enter-active,
.section-collapse-leave-active {
  transition: all 200ms ease;
  max-height: 500px;
  opacity: 1;
}

.section-collapse-enter-from,
.section-collapse-leave-to {
  max-height: 0;
  opacity: 0;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 7px 10px;
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  font-family: inherit;
  text-align: left;
  transition: all var(--transition-fast);
  position: relative;
  white-space: nowrap;
}

.nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--accent-muted);
  color: var(--accent-primary);
}

.nav-item.active::before {
  content: "";
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  border-radius: 0 2px 2px 0;
  background: var(--accent-primary);
}

.nav-icon {
  flex-shrink: 0;
}

.nav-label {
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-badge {
  margin-left: auto;
  background: var(--accent-primary);
  color: white;
  font-size: 10px;
  font-weight: 600;
  padding: 1px 6px;
  border-radius: 10px;
  min-width: 18px;
  text-align: center;
}

/* Footer */
.sidebar-footer {
  border-top: 1px solid var(--border);
  padding: 8px;
}

/* Transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 150ms ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
