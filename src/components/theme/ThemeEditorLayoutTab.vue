<script setup lang="ts">
import { useLayoutStore, LAYOUT_PRESETS, TAB_STYLE_LABELS, GROUP_NAV_STYLE_LABELS } from "@/stores/layoutStore";
import type { LayoutPresetId, SidebarWidth, UIDensity, ContentMaxWidth, TabStyle, GroupNavStyle } from "@/stores/layoutStore";
import {
  Layout, Sidebar, AlignLeft, AlignRight, Maximize2, Type, Check, Monitor, RotateCcw,
} from "lucide-vue-next";

const layoutStore = useLayoutStore();
</script>

<template>
  <!-- Presets disposition -->
  <div class="te-section">
    <div class="te-section-title"><Layout :size="13" /> Presets de disposition</div>
    <div class="te-presets-grid">
      <button
        v-for="p in LAYOUT_PRESETS" :key="p.id"
        class="te-preset-btn"
        :class="{ active: layoutStore.state.activePreset === p.id }"
        @click="layoutStore.applyPreset(p.id as LayoutPresetId)"
      >
        <span class="te-preset-emoji">{{ p.emoji }}</span>
        <div class="te-preset-info">
          <span class="te-preset-label">{{ p.label }}</span>
          <span class="te-preset-desc">{{ p.description }}</span>
        </div>
        <Check v-if="layoutStore.state.activePreset === p.id" :size="11" class="te-preset-check" />
      </button>
    </div>
  </div>

  <!-- Sidebar -->
  <div class="te-section">
    <div class="te-section-title"><Sidebar :size="13" /> Sidebar</div>
    <div class="te-vars-list">
      <div class="te-var-row">
        <label class="te-var-label">Position</label>
        <div class="te-btn-group">
          <button class="te-seg-btn" :class="{ active: layoutStore.state.sidebarPosition === 'left' }" @click="layoutStore.setField('sidebarPosition', 'left')">
            <AlignLeft :size="12" /> Gauche
          </button>
          <button class="te-seg-btn" :class="{ active: layoutStore.state.sidebarPosition === 'right' }" @click="layoutStore.setField('sidebarPosition', 'right')">
            <AlignRight :size="12" /> Droite
          </button>
        </div>
      </div>
      <div class="te-var-row">
        <label class="te-var-label">Mode</label>
        <div class="te-btn-group">
          <button class="te-seg-btn" :class="{ active: layoutStore.state.sidebarMode === 'icons-text' }" @click="layoutStore.setField('sidebarMode', 'icons-text')">Icônes + texte</button>
          <button class="te-seg-btn" :class="{ active: layoutStore.state.sidebarMode === 'icons-only' }" @click="layoutStore.setField('sidebarMode', 'icons-only')">Icônes seules</button>
        </div>
      </div>
      <div class="te-var-row">
        <label class="te-var-label">Largeur</label>
        <div class="te-btn-group">
          <button v-for="w in (['compact','normal','large'] as SidebarWidth[])" :key="w"
            class="te-seg-btn" :class="{ active: layoutStore.state.sidebarWidth === w }"
            @click="layoutStore.setField('sidebarWidth', w)"
          >{{ w === 'compact' ? '48px' : w === 'normal' ? '240px' : '290px' }}</button>
        </div>
      </div>
    </div>
  </div>

  <!-- Header -->
  <div class="te-section">
    <div class="te-section-title"><Monitor :size="13" /> Header</div>
    <div class="te-vars-list">
      <div class="te-var-row">
        <label class="te-var-label">Visible</label>
        <div class="te-btn-group">
          <button class="te-seg-btn" :class="{ active: layoutStore.state.headerVisible }" @click="layoutStore.setField('headerVisible', true)">Oui</button>
          <button class="te-seg-btn" :class="{ active: !layoutStore.state.headerVisible }" @click="layoutStore.setField('headerVisible', false)">Non</button>
        </div>
      </div>
    </div>
  </div>

  <!-- Contenu -->
  <div class="te-section">
    <div class="te-section-title"><Maximize2 :size="13" /> Contenu</div>
    <div class="te-vars-list">
      <div class="te-var-row">
        <label class="te-var-label">Largeur max</label>
        <select class="te-select" :value="layoutStore.state.contentMaxWidth" @change="layoutStore.setField('contentMaxWidth', ($event.target as HTMLSelectElement).value as ContentMaxWidth)">
          <option value="full">Pleine largeur</option>
          <option value="1400px">1400px</option>
          <option value="1200px">1200px</option>
          <option value="960px">960px</option>
        </select>
      </div>
      <div class="te-var-row te-var-row--radius">
        <label class="te-var-label">Padding ({{ layoutStore.state.contentPadding }}px)</label>
        <div class="te-var-controls">
          <input type="range" class="te-range-input" min="8" max="48"
            :value="layoutStore.state.contentPadding"
            @input="layoutStore.setField('contentPadding', parseInt(($event.target as HTMLInputElement).value))"
          />
        </div>
      </div>
    </div>
  </div>

  <!-- Densité & Typographie -->
  <div class="te-section">
    <div class="te-section-title"><Type :size="13" /> Densité &amp; Typographie</div>
    <div class="te-vars-list">
      <div class="te-var-row">
        <label class="te-var-label">Densité UI</label>
        <div class="te-btn-group">
          <button v-for="d in (['compact','normal','spacious'] as UIDensity[])" :key="d"
            class="te-seg-btn" :class="{ active: layoutStore.state.density === d }"
            @click="layoutStore.setField('density', d)"
          >{{ d === 'compact' ? 'Compact' : d === 'normal' ? 'Normal' : 'Aéré' }}</button>
        </div>
      </div>
      <div class="te-var-row te-var-row--radius">
        <label class="te-var-label">Taille police ({{ layoutStore.state.fontSize }}px)</label>
        <div class="te-var-controls">
          <input type="range" class="te-range-input" min="11" max="17"
            :value="layoutStore.state.fontSize"
            @input="layoutStore.setField('fontSize', parseInt(($event.target as HTMLInputElement).value))"
          />
        </div>
      </div>
    </div>
  </div>

  <!-- Navigation & Onglets -->
  <div class="te-section">
    <div class="te-section-title"><Layout :size="13" /> Navigation &amp; Onglets</div>
    <div class="te-vars-list">
      <div class="te-var-row">
        <label class="te-var-label">Style onglets</label>
        <div class="te-btn-group te-btn-group--wrap">
          <button v-for="(label, key) in TAB_STYLE_LABELS" :key="key"
            class="te-seg-btn" :class="{ active: layoutStore.state.tabStyle === key }"
            @click="layoutStore.setField('tabStyle', key as TabStyle)"
          >{{ label }}</button>
        </div>
      </div>
      <div class="te-var-row">
        <label class="te-var-label">Navigation groupes</label>
        <div class="te-btn-group te-btn-group--wrap">
          <button v-for="(label, key) in GROUP_NAV_STYLE_LABELS" :key="key"
            class="te-seg-btn" :class="{ active: layoutStore.state.groupNavStyle === key }"
            @click="layoutStore.setField('groupNavStyle', key as GroupNavStyle)"
          >{{ label }}</button>
        </div>
      </div>
      <div class="te-var-row te-nav-preview">
        <label class="te-var-label">Aperçu</label>
        <div class="te-tabs-preview" :data-preview-tab="layoutStore.state.tabStyle">
          <button class="tp-tab tp-tab--active">Système</button>
          <button class="tp-tab">Réseau</button>
          <button class="tp-tab">Stockage</button>
        </div>
      </div>
    </div>
  </div>

  <!-- Reset -->
  <button class="te-action-btn" style="width:100%;justify-content:center" @click="layoutStore.reset()">
    <RotateCcw :size="13" /> Réinitialiser le layout
  </button>
</template>
