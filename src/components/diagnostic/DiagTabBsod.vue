<template>
  <div class="diag-tab-content">
    <div class="diag-section-header">
      <AlertTriangle :size="16" /> Analyseur BSOD
    </div>

    <button class="diag-btn diag-btn-primary" :disabled="loading" style="margin-bottom:12px" @click="load">
      <RefreshCw :size="13" /> Analyser les crashs
    </button>

    <div v-if="loading" class="diag-loading">Lecture de l'historique des crashs...</div>

    <div v-else-if="report">
      <!-- Stats -->
      <div class="bsod-stats">
        <div class="bsod-stat" :class="report.total_count > 0 ? 'stat-warn' : 'stat-ok'">
          <div class="stat-value">{{ report.total_count }}</div>
          <div class="stat-label">BSOD détectés</div>
        </div>
        <div class="bsod-stat">
          <div class="stat-value">{{ report.dump_count }}</div>
          <div class="stat-label">Fichiers dump</div>
        </div>
        <div class="bsod-stat">
          <div class="stat-value" style="font-size:13px">{{ report.last_bsod }}</div>
          <div class="stat-label">Dernier crash</div>
        </div>
        <div class="bsod-stat">
          <div class="stat-value" style="font-size:11px;opacity:.6">{{ report.dump_folder }}</div>
          <div class="stat-label">Dossier dumps</div>
        </div>
      </div>

      <div v-if="report.entries.length === 0" class="bsod-empty">
        <CheckCircle :size="24" style="color:#22c55e" />
        <span>Aucun BSOD détecté — système stable</span>
      </div>

      <!-- Entries -->
      <div v-else class="bsod-list">
        <div v-for="(e, i) in report.entries" :key="i" class="bsod-entry" @click="selected = selected === i ? null : i">
          <div class="bsod-entry-header">
            <div class="bsod-code" :title="e.bug_check_hex">{{ e.bug_check_code || e.bug_check_hex || '—' }}</div>
            <div class="bsod-desc">{{ e.description }}</div>
            <div class="bsod-ts">{{ e.timestamp }}</div>
          </div>
          <div v-if="selected === i" class="bsod-detail">
            <div v-if="e.module"><b>Module :</b> {{ e.module }}</div>
            <div v-if="e.dump_file"><b>Dump :</b> {{ e.dump_file }} ({{ e.dump_size_kb }} KB)</div>
            <div v-if="e.bug_check_hex" class="bsod-explain">
              <b>Explication :</b><br/>{{ getDescription(e.bug_check_hex) }}
            </div>
            <div v-if="e.parameters.length > 0"><b>Paramètres :</b> {{ e.parameters.join(', ') }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Code lookup -->
    <div class="diag-section-header" style="margin-top:20px"><Search :size="16" /> Recherche par code</div>
    <div style="display:flex;gap:8px;align-items:center">
      <input v-model="lookupCode" class="diag-input" style="width:140px" placeholder="Ex: 0x3B ou 3B" />
      <button class="diag-btn" @click="doLookup">Rechercher</button>
    </div>
    <div v-if="lookupResult" class="bsod-explain" style="margin-top:8px">{{ lookupResult }}</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { AlertTriangle, RefreshCw, CheckCircle, Search } from 'lucide-vue-next'

interface BsodEntry {
  timestamp: string; bug_check_code: string; bug_check_hex: string
  description: string; parameters: string[]; module: string; dump_file: string; dump_size_kb: number
}
interface BsodReport {
  entries: BsodEntry[]; total_count: number; last_bsod: string; dump_folder: string; dump_count: number
}

const loading = ref(false)
const report = ref<BsodReport | null>(null)
const selected = ref<number | null>(null)
const lookupCode = ref('')
const lookupResult = ref('')

async function load() {
  loading.value = true
  try {
    report.value = await invoke<BsodReport>('get_bsod_history')
  } finally {
    loading.value = false
  }
}

async function getDescription(code: string): Promise<string> {
  try { return await invoke<string>('get_bugcheck_description', { code }) } catch { return '' }
}

// Sync lookup for display in template
const descCache: Record<string, string> = {}
function getDescSync(code: string): string {
  if (!code) return ''
  if (descCache[code]) return descCache[code]
  invoke<string>('get_bugcheck_description', { code }).then(d => { descCache[code] = d })
  return 'Chargement...'
}

async function doLookup() {
  if (!lookupCode.value.trim()) return
  lookupResult.value = await invoke<string>('get_bugcheck_description', { code: lookupCode.value.trim() })
}
</script>

<style scoped>
.bsod-stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin-bottom: 16px; }
.bsod-stat { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; padding: 12px; text-align: center; }
.stat-warn { border-color: #ef4444 !important; }
.stat-ok { border-color: #22c55e !important; }
.stat-value { font-size: 22px; font-weight: 700; }
.stat-label { font-size: 10px; opacity: .5; text-transform: uppercase; margin-top: 2px; }
.bsod-empty { display: flex; align-items: center; gap: 10px; padding: 20px; font-size: 14px; opacity: .8; }
.bsod-list { display: flex; flex-direction: column; gap: 6px; }
.bsod-entry { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; overflow: hidden; cursor: pointer; transition: border-color .15s; }
.bsod-entry:hover { border-color: var(--accent, #7c3aed); }
.bsod-entry-header { display: flex; align-items: center; gap: 12px; padding: 10px 14px; }
.bsod-code { font-family: monospace; font-size: 13px; font-weight: 700; color: #ef4444; min-width: 80px; }
.bsod-desc { flex: 1; font-size: 12px; opacity: .8; }
.bsod-ts { font-size: 11px; opacity: .5; }
.bsod-detail { padding: 10px 14px; border-top: 1px solid rgba(255,255,255,.07); font-size: 12px; display: flex; flex-direction: column; gap: 6px; background: rgba(0,0,0,.2); }
.bsod-explain { font-size: 12px; background: rgba(239,68,68,.08); border: 1px solid rgba(239,68,68,.2); border-radius: 6px; padding: 8px; line-height: 1.5; }
</style>
