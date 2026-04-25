<script setup lang="ts">
import { computed } from "vue";
import { useThemeEditorStore } from "@/stores/themeEditor";

const store = useThemeEditorStore();
</script>

<template>
  <div class="tp-scope" :style="store.previewStyle">
    <div class="tp-root">

      <!-- Header mini -->
      <div class="tp-header">
        <div class="tp-breadcrumb">
          <span class="tp-bc-section">Système</span>
          <span class="tp-bc-sep">/</span>
          <span class="tp-bc-page">Tableau de bord</span>
        </div>
        <div class="tp-search">
          <span style="font-size:11px; opacity:0.5">Rechercher... Ctrl+K</span>
        </div>
      </div>

      <!-- Cards row -->
      <div class="tp-row">
        <!-- Default card -->
        <div class="tp-card">
          <div class="tp-card-header">Carte Standard</div>
          <div class="tp-card-body">
            <p class="tp-text-primary">Texte principal</p>
            <p class="tp-text-secondary">Texte secondaire</p>
            <p class="tp-text-muted">Texte atténué</p>
          </div>
        </div>
        <!-- Premium card -->
        <div class="tp-card tp-card--premium">
          <div class="tp-card-header">Carte Premium</div>
          <div class="tp-card-body">
            <p class="tp-text-accent" style="font-weight:700; font-size:18px">42%</p>
            <p class="tp-text-secondary">Utilisation CPU</p>
            <div class="tp-progress-track">
              <div class="tp-progress-bar" style="width:42%"></div>
            </div>
          </div>
        </div>
        <!-- Glass card -->
        <div class="tp-card tp-card--glass">
          <div class="tp-card-header">Carte Glass</div>
          <div class="tp-card-body">
            <div class="tp-badges">
              <span class="tp-badge tp-badge--accent">Accent</span>
              <span class="tp-badge tp-badge--success">OK</span>
              <span class="tp-badge tp-badge--warning">Warn</span>
              <span class="tp-badge tp-badge--danger">Err</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Buttons row -->
      <div class="tp-section-label">Boutons</div>
      <div class="tp-btn-row">
        <button class="tp-btn tp-btn--primary">Principal</button>
        <button class="tp-btn tp-btn--secondary">Secondaire</button>
        <button class="tp-btn tp-btn--ghost">Fantôme</button>
        <button class="tp-btn tp-btn--danger">Danger</button>
        <button class="tp-btn tp-btn--success">Succès</button>
      </div>

      <!-- Progress bars -->
      <div class="tp-section-label">Barres de progression</div>
      <div class="tp-prog-list">
        <div class="tp-prog-item">
          <span class="tp-text-secondary" style="font-size:11px; min-width:40px">CPU</span>
          <div class="tp-progress-track" style="flex:1">
            <div class="tp-progress-bar" style="width:68%"></div>
          </div>
          <span class="tp-text-accent" style="font-size:11px; min-width:32px; text-align:right">68%</span>
        </div>
        <div class="tp-prog-item">
          <span class="tp-text-secondary" style="font-size:11px; min-width:40px">RAM</span>
          <div class="tp-progress-track" style="flex:1">
            <div class="tp-progress-bar tp-progress-bar--success" style="width:45%"></div>
          </div>
          <span class="tp-text-secondary" style="font-size:11px; min-width:32px; text-align:right">45%</span>
        </div>
        <div class="tp-prog-item">
          <span class="tp-text-secondary" style="font-size:11px; min-width:40px">Disque</span>
          <div class="tp-progress-track" style="flex:1">
            <div class="tp-progress-bar tp-progress-bar--warning" style="width:82%"></div>
          </div>
          <span style="color:var(--warning); font-size:11px; min-width:32px; text-align:right">82%</span>
        </div>
      </div>

      <!-- Status bar mini -->
      <div class="tp-statusbar">
        <span class="tp-dot tp-dot--success"></span>
        <span class="tp-text-muted" style="font-size:10px">CPU 42%</span>
        <span class="tp-sep"></span>
        <span class="tp-dot tp-dot--success"></span>
        <span class="tp-text-muted" style="font-size:10px">RAM 55%</span>
        <div style="flex:1"></div>
        <span class="tp-text-accent" style="font-size:10px; font-family:monospace; opacity:0.6">NiTriTe v6.0.0</span>
      </div>

    </div>
  </div>
</template>

<style scoped>
.tp-scope {
  width: 100%;
  height: 100%;
  border-radius: var(--radius-xl);
  overflow: hidden;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: "Inter", system-ui, sans-serif;
  font-size: 13px;
  display: flex;
  flex-direction: column;
}
.tp-root {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 12px;
  overflow-y: auto;
}
.tp-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
}
.tp-breadcrumb { display: flex; align-items: center; gap: 6px; font-size: 12px; }
.tp-bc-section { color: var(--text-muted); }
.tp-bc-sep { color: var(--border-hover); }
.tp-bc-page { color: var(--text-primary); font-weight: 600; }
.tp-search { padding: 4px 10px; border: 1px solid var(--border-hover); border-radius: var(--radius-md); background: var(--bg-tertiary); color: var(--text-muted); }
.tp-row { display: flex; gap: 10px; }
.tp-card {
  flex: 1;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-xl);
  overflow: hidden;
}
.tp-card--premium { border-color: color-mix(in srgb, var(--accent-primary) 40%, transparent); }
.tp-card--glass { background: rgba(255,255,255,0.04); backdrop-filter: blur(12px); border-color: rgba(255,255,255,0.08); }
.tp-card-header { padding: 8px 12px; border-bottom: 1px solid var(--border); font-size: 11px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.06em; background: var(--bg-tertiary); }
.tp-card-body { padding: 10px 12px; display: flex; flex-direction: column; gap: 6px; }
.tp-text-primary { color: var(--text-primary); font-weight: 500; font-size: 12px; }
.tp-text-secondary { color: var(--text-secondary); font-size: 12px; }
.tp-text-muted { color: var(--text-muted); font-size: 12px; }
.tp-text-accent { color: var(--accent-primary); }
.tp-badges { display: flex; gap: 5px; flex-wrap: wrap; }
.tp-badge { display: inline-flex; align-items: center; padding: 2px 7px; border-radius: 99px; font-size: 10px; font-weight: 600; }
.tp-badge--accent  { background: var(--accent-muted); color: var(--accent-primary); }
.tp-badge--success { background: rgba(34,197,94,0.12); color: var(--success); }
.tp-badge--warning { background: rgba(234,179,8,0.12); color: var(--warning); }
.tp-badge--danger  { background: rgba(239,68,68,0.12); color: var(--danger); }
.tp-section-label {
  font-size: 10px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em;
  color: var(--accent-primary); padding-left: 8px; border-left: 2px solid var(--accent-primary);
}
.tp-btn-row { display: flex; gap: 6px; flex-wrap: wrap; }
.tp-btn {
  display: inline-flex; align-items: center; padding: 5px 12px; border-radius: var(--radius-md);
  font-size: 11px; font-weight: 500; cursor: default; border: none; font-family: inherit;
}
.tp-btn--primary { background: var(--accent-primary); color: white; }
.tp-btn--secondary { background: var(--bg-tertiary); color: var(--text-primary); border: 1px solid var(--border-hover); }
.tp-btn--ghost { background: transparent; color: var(--text-secondary); border: 1px solid var(--border); }
.tp-btn--danger { background: var(--danger); color: white; }
.tp-btn--success { background: var(--success); color: white; }
.tp-prog-list { display: flex; flex-direction: column; gap: 7px; }
.tp-prog-item { display: flex; align-items: center; gap: 8px; }
.tp-progress-track {
  height: 5px; background: var(--bg-elevated); border-radius: 99px; overflow: hidden;
  border: 1px solid var(--border);
}
.tp-progress-bar {
  height: 100%; background: var(--accent-primary); border-radius: 99px;
}
.tp-progress-bar--success { background: var(--success); }
.tp-progress-bar--warning { background: var(--warning); }
.tp-statusbar {
  display: flex; align-items: center; gap: 8px; padding: 6px 12px;
  background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-md);
  margin-top: auto;
}
.tp-dot { width: 5px; height: 5px; border-radius: 50%; flex-shrink: 0; }
.tp-dot--success { background: var(--success); }
.tp-sep { width: 1px; height: 10px; background: var(--border-hover); opacity: 0.6; }
</style>
