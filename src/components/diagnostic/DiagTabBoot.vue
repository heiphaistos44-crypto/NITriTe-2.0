<template>
  <div class="diag-tab-content">
    <div class="diag-section-header"><Settings :size="16" /> Gestionnaire de Démarrage</div>

    <div style="display:flex;gap:8px;margin-bottom:12px;align-items:center">
      <button class="diag-btn diag-btn-primary" :disabled="loading" @click="load">
        <RefreshCw :size="13" /> Actualiser
      </button>
      <span v-if="msg" :class="msgErr ? 'boot-err' : 'boot-ok'">{{ msg }}</span>
    </div>

    <div v-if="loading" class="diag-loading">Lecture de la configuration BCD...</div>

    <div v-else-if="config">
      <!-- Global settings -->
      <div class="boot-settings">
        <div class="boot-setting-row">
          <span class="boot-label">Timeout de démarrage</span>
          <div style="display:flex;gap:8px;align-items:center">
            <input v-model.number="timeout" type="number" class="diag-input" style="width:80px" min="0" max="999" />
            <span style="font-size:12px;opacity:.6">secondes</span>
            <button class="diag-btn" @click="saveTimeout">Appliquer</button>
          </div>
        </div>
        <div class="boot-setting-row">
          <span class="boot-label">Entrée par défaut</span>
          <span class="boot-value">{{ config.default_id || '(non défini)' }}</span>
        </div>
      </div>

      <!-- Boot entries -->
      <div class="diag-section-header" style="margin-top:16px"><List :size="16" /> Entrées de démarrage</div>
      <div class="boot-entries">
        <div v-for="e in config.entries" :key="e.id" class="boot-entry" :class="e.is_default ? 'boot-default' : ''">
          <div class="boot-entry-header">
            <div class="boot-entry-icon">
              <Star v-if="e.is_default" :size="14" style="color:#f59e0b" />
              <Monitor v-else :size="14" style="opacity:.5" />
            </div>
            <div class="boot-entry-info">
              <div class="boot-entry-name">{{ e.description || e.id }}</div>
              <div class="boot-entry-id">{{ e.id }}</div>
              <div v-if="e.path" class="boot-entry-path">{{ e.path }}</div>
              <div v-if="e.device" class="boot-entry-path">{{ e.device }}</div>
            </div>
            <div class="boot-entry-actions">
              <span v-if="e.is_default" class="boot-badge-default">Défaut</span>
              <button v-else class="diag-btn diag-btn-sm" @click="setDefault(e.id)">Définir par défaut</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Danger zone -->
      <div class="diag-section-header" style="margin-top:20px;color:#ef4444"><AlertTriangle :size="16" /> Actions Avancées</div>
      <div class="boot-danger">
        <div class="boot-danger-item">
          <div>
            <div class="boot-danger-title">Redémarrer en mode Récupération</div>
            <div class="boot-danger-desc">Redémarre vers les options avancées de Windows (WinRE)</div>
          </div>
          <button class="diag-btn" style="color:#ef4444;border-color:#ef4444" @click="confirmRecovery = true">
            Redémarrer
          </button>
        </div>
      </div>

      <!-- Confirm modal -->
      <div v-if="confirmRecovery" class="boot-confirm-overlay" @click.self="confirmRecovery = false">
        <div class="boot-confirm-box">
          <AlertTriangle :size="32" style="color:#ef4444;margin-bottom:10px" />
          <div style="font-size:15px;font-weight:600;margin-bottom:6px">Confirmer le redémarrage</div>
          <div style="font-size:12px;opacity:.7;margin-bottom:16px">
            Le PC va redémarrer immédiatement en mode récupération. Sauvegardez votre travail.
          </div>
          <div style="display:flex;gap:8px">
            <button class="diag-btn" @click="confirmRecovery = false">Annuler</button>
            <button class="diag-btn" style="background:#ef4444;color:#fff;border-color:#ef4444" @click="doRecovery">Confirmer</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Settings, RefreshCw, List, Star, Monitor, AlertTriangle } from 'lucide-vue-next'

interface BcdEntry { id: string; description: string; entry_type: string; device: string; path: string; locale: string; is_default: boolean }
interface BootConfig { entries: BcdEntry[]; default_id: string; timeout_secs: number; safe_mode: boolean; debug_mode: boolean }

const loading = ref(false)
const config = ref<BootConfig | null>(null)
const timeout = ref(30)
const msg = ref('')
const msgErr = ref(false)
const confirmRecovery = ref(false)

function showMsg(text: string, err = false) {
  msg.value = text; msgErr.value = err
  setTimeout(() => { msg.value = '' }, 3000)
}

async function load() {
  loading.value = true
  try {
    config.value = await invoke<BootConfig>('get_boot_config')
    timeout.value = config.value.timeout_secs
  } finally { loading.value = false }
}

async function saveTimeout() {
  try {
    const r = await invoke<string>('set_boot_timeout', { seconds: timeout.value })
    showMsg(r)
  } catch(e) { showMsg(String(e), true) }
}

async function setDefault(entryId: string) {
  try {
    const r = await invoke<string>('set_default_boot', { entryId })
    showMsg(r)
    await load()
  } catch(e) { showMsg(String(e), true) }
}

async function doRecovery() {
  confirmRecovery.value = false
  await invoke<string>('boot_to_recovery')
}

onMounted(load)
</script>

<style scoped>
.boot-settings { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; padding: 12px; display: flex; flex-direction: column; gap: 10px; }
.boot-setting-row { display: flex; align-items: center; gap: 12px; }
.boot-label { font-size: 13px; font-weight: 500; min-width: 180px; }
.boot-value { font-size: 12px; opacity: .7; font-family: monospace; }
.boot-entries { display: flex; flex-direction: column; gap: 6px; }
.boot-entry { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; overflow: hidden; }
.boot-default { border-color: #f59e0b !important; background: rgba(245,158,11,.06) !important; }
.boot-entry-header { display: flex; align-items: flex-start; gap: 12px; padding: 12px 14px; }
.boot-entry-icon { padding-top: 2px; }
.boot-entry-info { flex: 1; display: flex; flex-direction: column; gap: 2px; }
.boot-entry-name { font-size: 13px; font-weight: 600; }
.boot-entry-id { font-family: monospace; font-size: 10px; opacity: .5; }
.boot-entry-path { font-size: 11px; opacity: .5; }
.boot-entry-actions { display: flex; align-items: center; }
.boot-badge-default { font-size: 10px; background: rgba(245,158,11,.2); color: #f59e0b; border: 1px solid rgba(245,158,11,.3); border-radius: 4px; padding: 2px 8px; }
.boot-danger { background: rgba(239,68,68,.06); border: 1px solid rgba(239,68,68,.2); border-radius: 8px; padding: 12px; }
.boot-danger-item { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
.boot-danger-title { font-size: 13px; font-weight: 600; }
.boot-danger-desc { font-size: 11px; opacity: .6; margin-top: 2px; }
.boot-confirm-overlay { position: fixed; inset: 0; background: rgba(0,0,0,.6); display: flex; align-items: center; justify-content: center; z-index: 999; }
.boot-confirm-box { background: var(--bg-primary, #13131f); border: 1px solid rgba(239,68,68,.4); border-radius: 12px; padding: 24px; text-align: center; max-width: 360px; }
.boot-ok { color: #22c55e; font-size: 12px; }
.boot-err { color: #ef4444; font-size: 12px; }
</style>
