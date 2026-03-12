<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSkeleton from "@/components/ui/NSkeleton.vue";
import { useNotificationStore } from "@/stores/notifications";
import { Server, Star, Clock, RefreshCw, AlertTriangle, Shield } from "lucide-vue-next";

const notify = useNotificationStore();

interface BcdEntry {
  id: string; description: string; entry_type: string;
  device: string; path: string; locale: string; is_default: boolean;
}
interface BootConfig {
  entries: BcdEntry[]; default_id: string; timeout_secs: number;
  safe_mode: boolean; debug_mode: boolean;
}

const config = ref<BootConfig | null>(null);
const loading = ref(true);
const newTimeout = ref(10);
const savingTimeout = ref(false);
const booting = ref(false);

async function load() {
  loading.value = true;
  try {
    config.value = await invoke<BootConfig>("get_boot_config");
    if (config.value) newTimeout.value = config.value.timeout_secs;
  } catch (e: any) {
    notify.error("Erreur BCD", String(e));
  }
  loading.value = false;
}

async function setDefault(id: string) {
  try {
    await invoke("set_default_boot", { id });
    notify.success("Entrée de démarrage modifiée");
    await load();
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
}

async function saveTimeout() {
  savingTimeout.value = true;
  try {
    await invoke("set_boot_timeout", { secs: newTimeout.value });
    notify.success("Délai de démarrage mis à jour", `${newTimeout.value} secondes`);
    await load();
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  savingTimeout.value = false;
}

async function bootToRecovery() {
  booting.value = true;
  try {
    await invoke("boot_to_recovery");
    notify.info("Redémarrage", "Le système va redémarrer en mode récupération...");
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  booting.value = false;
}

onMounted(load);
</script>

<template>
  <div class="boot-page">
    <div class="page-header">
      <div class="header-icon"><Server :size="22" /></div>
      <div>
        <h1>Gestionnaire de Démarrage</h1>
        <p class="subtitle">Configuration BCD (Boot Configuration Data)</p>
      </div>
      <NButton variant="ghost" size="sm" :loading="loading" @click="load" style="margin-left:auto">
        <RefreshCw :size="13" /> Actualiser
      </NButton>
    </div>

    <div class="warning-banner">
      <AlertTriangle :size="14" />
      <span>Modifier la configuration de démarrage peut rendre le système inopérant. Procédez avec précaution.</span>
    </div>

    <div v-if="loading">
      <NSkeleton v-for="i in 3" :key="i" height="68px" style="margin-bottom:8px" />
    </div>

    <template v-else-if="config">
      <!-- Entrées BCD -->
      <NCard>
        <template #header>
          <div class="section-header"><Server :size="15" /><span>Entrées de démarrage ({{ config.entries.length }})</span></div>
        </template>
        <div class="entries-list">
          <div
            v-for="e in config.entries" :key="e.id"
            class="entry-card"
            :class="{ 'is-default': e.is_default }"
          >
            <div class="entry-left">
              <div class="entry-icon" :class="e.is_default ? 'default' : ''">
                <Star v-if="e.is_default" :size="16" />
                <Server v-else :size="16" />
              </div>
              <div class="entry-info">
                <span class="entry-desc">{{ e.description || 'Windows Boot Manager' }}</span>
                <span class="entry-id">{{ e.id }}</span>
                <span class="entry-meta" v-if="e.device">{{ e.device }}</span>
              </div>
            </div>
            <div class="entry-right">
              <NBadge v-if="e.is_default" variant="success" dot>Défaut</NBadge>
              <NBadge v-if="e.entry_type" variant="neutral">{{ e.entry_type }}</NBadge>
              <NButton v-if="!e.is_default" variant="secondary" size="sm" @click="setDefault(e.id)">
                <Star :size="12" /> Définir par défaut
              </NButton>
            </div>
          </div>
        </div>
      </NCard>

      <!-- Délai de démarrage -->
      <NCard>
        <template #header>
          <div class="section-header"><Clock :size="15" /><span>Délai de démarrage</span></div>
        </template>
        <div class="timeout-row">
          <p class="timeout-desc">Temps d'affichage du menu de démarrage avant boot automatique</p>
          <div class="timeout-ctrl">
            <input type="range" v-model.number="newTimeout" min="0" max="60" step="1" class="timeout-slider" />
            <span class="timeout-val">{{ newTimeout }}s</span>
            <NButton variant="primary" size="sm" :loading="savingTimeout" @click="saveTimeout">Appliquer</NButton>
          </div>
        </div>
        <div class="info-row">
          <span>Mode actuel :</span>
          <NBadge v-if="config.safe_mode" variant="warning">Safe Mode</NBadge>
          <NBadge v-if="config.debug_mode" variant="danger">Debug Mode</NBadge>
          <NBadge v-if="!config.safe_mode && !config.debug_mode" variant="success">Normal</NBadge>
        </div>
      </NCard>

      <!-- Récupération -->
      <NCard>
        <template #header>
          <div class="section-header"><Shield :size="15" /><span>Récupération système</span></div>
        </template>
        <div class="recovery-section">
          <p class="recovery-desc">Redémarre le système en environnement de récupération Windows (WinRE) au prochain démarrage.</p>
          <NButton variant="danger" size="sm" :loading="booting" @click="bootToRecovery">
            <Shield :size="13" /> Redémarrer en récupération
          </NButton>
        </div>
      </NCard>
    </template>
  </div>
</template>

<style scoped>
.boot-page { display: flex; flex-direction: column; gap: 14px; }
.page-header { display: flex; align-items: center; gap: 12px; }
.header-icon { width: 42px; height: 42px; border-radius: var(--radius-lg); background: var(--info-muted, color-mix(in srgb, #60a5fa 15%, transparent)); display: flex; align-items: center; justify-content: center; color: #60a5fa; flex-shrink: 0; }
h1 { font-size: 22px; font-weight: 700; }
.subtitle { font-size: 12px; color: var(--text-muted); }
.warning-banner { display: flex; gap: 10px; align-items: center; padding: 10px 14px; background: var(--danger-muted); border: 1px solid color-mix(in srgb, var(--danger) 30%, transparent); border-radius: var(--radius-md); font-size: 12px; color: var(--danger); }
.section-header { display: flex; align-items: center; gap: 8px; }
.entries-list { display: flex; flex-direction: column; gap: 8px; }
.entry-card { display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 14px; background: var(--bg-tertiary); border-radius: var(--radius-lg); border: 1px solid var(--border); transition: border-color .15s; }
.entry-card.is-default { border-color: var(--success); background: var(--success-muted); }
.entry-card:hover { border-color: var(--text-muted); }
.entry-left { display: flex; align-items: center; gap: 12px; flex: 1; min-width: 0; }
.entry-icon { width: 36px; height: 36px; border-radius: var(--radius-md); display: flex; align-items: center; justify-content: center; flex-shrink: 0; background: var(--bg-elevated); color: var(--text-muted); }
.entry-icon.default { background: var(--success-muted); color: var(--success); }
.entry-info { display: flex; flex-direction: column; gap: 2px; min-width: 0; }
.entry-desc { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.entry-id { font-family: "JetBrains Mono", monospace; font-size: 10px; color: var(--text-muted); }
.entry-meta { font-size: 11px; color: var(--text-secondary); }
.entry-right { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.timeout-row { display: flex; flex-direction: column; gap: 10px; }
.timeout-desc { font-size: 12px; color: var(--text-muted); }
.timeout-ctrl { display: flex; align-items: center; gap: 12px; }
.timeout-slider { flex: 1; accent-color: var(--accent-primary); }
.timeout-val { font-family: "JetBrains Mono", monospace; font-size: 14px; font-weight: 700; color: var(--accent-primary); width: 32px; }
.info-row { display: flex; align-items: center; gap: 8px; margin-top: 10px; font-size: 12px; color: var(--text-muted); }
.recovery-section { display: flex; align-items: flex-start; justify-content: space-between; gap: 14px; }
.recovery-desc { font-size: 12px; color: var(--text-secondary); max-width: 460px; line-height: 1.6; }
</style>
