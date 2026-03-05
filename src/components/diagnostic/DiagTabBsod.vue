<template>
  <div class="bsod-root">
    <!-- Banner -->
    <div class="bsod-banner" :class="report && report.total_count > 0 ? 'banner-alert' : 'banner-ok'">
      <div class="bsod-banner-icon"><AlertTriangle :size="26" /></div>
      <div class="bsod-banner-text">
        <div class="bsod-banner-title">Analyseur BSOD</div>
        <div class="bsod-banner-desc">
          <span v-if="!report">Analysez les crashs et écrans bleus de votre système</span>
          <span v-else-if="report.total_count === 0">✓ Aucun BSOD détecté — système stable</span>
          <span v-else>{{ report.total_count }} crash(s) détecté(s) — dernier : {{ report.last_bsod }}</span>
        </div>
      </div>
      <button class="bsod-btn bsod-btn-primary" :disabled="loading" @click="load">
        <RefreshCw :size="13" /> Analyser les crashs
      </button>
    </div>

    <div v-if="loading" class="bsod-loading"><div class="bsod-spinner" /> Lecture de l'historique des crashs...</div>

    <div v-else-if="report">
      <!-- Stats -->
      <div class="bsod-stats">
        <div class="bsod-stat" :class="report.total_count > 0 ? 'stat-red' : 'stat-green'">
          <div class="bsod-stat-val">{{ report.total_count }}</div>
          <div class="bsod-stat-lbl">BSOD détectés</div>
        </div>
        <div class="bsod-stat stat-blue">
          <div class="bsod-stat-val">{{ report.dump_count }}</div>
          <div class="bsod-stat-lbl">Fichiers dump</div>
        </div>
        <div class="bsod-stat stat-purple">
          <div class="bsod-stat-val" style="font-size:13px">{{ report.last_bsod }}</div>
          <div class="bsod-stat-lbl">Dernier crash</div>
        </div>
        <div class="bsod-stat stat-gray">
          <div class="bsod-stat-val" style="font-size:11px;opacity:.7">{{ report.dump_folder }}</div>
          <div class="bsod-stat-lbl">Dossier dumps</div>
        </div>
      </div>

      <!-- No BSOD -->
      <div v-if="report.entries.length === 0" class="bsod-empty">
        <div class="bsod-empty-icon">✓</div>
        <div>
          <div style="font-size:15px;font-weight:600;margin-bottom:4px">Système stable</div>
          <div style="font-size:12px;opacity:.6">Aucun BSOD ni rapport de crash détecté dans les journaux Windows</div>
        </div>
      </div>

      <!-- Entries -->
      <div v-else class="bsod-list">
        <div v-for="(e, i) in report.entries" :key="i" class="bsod-entry" :class="{ expanded: selected === i }" @click="selected = selected === i ? null : i">
          <div class="bsod-entry-top">
            <div class="bsod-code-badge">
              <span>{{ e.bug_check_code || e.bug_check_hex || '???' }}</span>
            </div>
            <div class="bsod-entry-info">
              <div class="bsod-entry-desc">{{ e.description }}</div>
              <div v-if="e.module" class="bsod-entry-module">{{ e.module }}</div>
            </div>
            <div class="bsod-entry-right">
              <div class="bsod-entry-time">{{ e.timestamp }}</div>
              <ChevronDown :size="14" :style="{ transform: selected === i ? 'rotate(180deg)' : 'none', transition: '200ms', opacity: .5 }" />
            </div>
          </div>
          <div v-if="selected === i" class="bsod-entry-detail">
            <div v-if="e.bug_check_hex" class="bsod-explain">
              <div class="bsod-explain-title"><BookOpen :size="13" /> Explication</div>
              {{ getDescSync(e.bug_check_hex) }}
            </div>
            <div class="bsod-detail-grid">
              <div v-if="e.dump_file" class="bsod-kv">
                <span class="bsod-k">Fichier dump</span>
                <span class="bsod-v">{{ e.dump_file }} ({{ e.dump_size_kb }} KB)</span>
              </div>
              <div v-if="e.parameters.length" class="bsod-kv">
                <span class="bsod-k">Paramètres</span>
                <span class="bsod-v mono">{{ e.parameters.join(' | ') }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Lookup -->
    <div class="bsod-lookup">
      <div class="bsod-lookup-header"><Search :size="14" /> Recherche par code d'erreur</div>
      <div class="bsod-lookup-body">
        <input v-model="lookupCode" class="bsod-input" placeholder="Ex: 0x3B ou 124 ou DRIVER_IRQL..." @keyup.enter="doLookup" />
        <button class="bsod-btn" @click="doLookup">Rechercher</button>
      </div>
      <div v-if="lookupResult" class="bsod-lookup-result">{{ lookupResult }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { AlertTriangle, RefreshCw, Search, ChevronDown, BookOpen } from 'lucide-vue-next'

interface BsodEntry { timestamp: string; bug_check_code: string; bug_check_hex: string; description: string; parameters: string[]; module: string; dump_file: string; dump_size_kb: number }
interface BsodReport { entries: BsodEntry[]; total_count: number; last_bsod: string; dump_folder: string; dump_count: number }

const loading = ref(false); const report = ref<BsodReport | null>(null)
const selected = ref<number | null>(null); const lookupCode = ref(''); const lookupResult = ref('')
const descCache: Record<string, string> = {}

async function load() {
  loading.value = true
  try { report.value = await invoke<BsodReport>('get_bsod_history') }
  finally { loading.value = false }
}
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
.bsod-root { display: flex; flex-direction: column; gap: 14px; }

/* Banner */
.bsod-banner { display: flex; align-items: center; gap: 16px; padding: 18px 22px; border-radius: 14px; border: 1px solid; }
.banner-alert { background: linear-gradient(135deg,rgba(239,68,68,.15),rgba(220,38,38,.08)); border-color: rgba(239,68,68,.35); }
.banner-ok    { background: linear-gradient(135deg,rgba(34,197,94,.1),rgba(16,185,129,.06)); border-color: rgba(34,197,94,.3); }
.bsod-banner-icon { width: 48px; height: 48px; border-radius: 12px;
  background: linear-gradient(135deg,#ef4444,#b91c1c); display: flex; align-items: center;
  justify-content: center; color: #fff; flex-shrink: 0; box-shadow: 0 4px 14px rgba(239,68,68,.4); }
.bsod-banner-text { flex: 1; }
.bsod-banner-title { font-size: 17px; font-weight: 700; margin-bottom: 3px; }
.bsod-banner-desc { font-size: 12px; opacity: .7; }

/* Stats */
.bsod-stats { display: grid; grid-template-columns: repeat(4,1fr); gap: 10px; }
.bsod-stat { border-radius: 12px; padding: 16px; text-align: center; border: 1px solid transparent; }
.stat-red    { background: rgba(239,68,68,.1);  border-color: rgba(239,68,68,.25); }
.stat-green  { background: rgba(34,197,94,.1);  border-color: rgba(34,197,94,.25); }
.stat-blue   { background: rgba(59,130,246,.1); border-color: rgba(59,130,246,.25); }
.stat-purple { background: rgba(124,58,237,.1); border-color: rgba(124,58,237,.25); }
.stat-gray   { background: var(--bg-secondary); border-color: var(--border); }
.bsod-stat-val { font-size: 24px; font-weight: 700; }
.bsod-stat-lbl { font-size: 10px; opacity: .5; text-transform: uppercase; margin-top: 2px; }

/* Empty */
.bsod-empty { display: flex; align-items: center; gap: 20px; padding: 28px; background: rgba(34,197,94,.07); border: 1px solid rgba(34,197,94,.2); border-radius: 14px; }
.bsod-empty-icon { font-size: 36px; color: #22c55e; background: rgba(34,197,94,.12); width: 56px; height: 56px; border-radius: 50%; display: flex; align-items: center; justify-content: center; flex-shrink: 0; }

/* Entries */
.bsod-list { display: flex; flex-direction: column; gap: 6px; }
.bsod-entry { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: 12px; overflow: hidden; cursor: pointer; transition: border-color 150ms; }
.bsod-entry:hover { border-color: rgba(239,68,68,.4); }
.bsod-entry.expanded { border-color: rgba(239,68,68,.5); }
.bsod-entry-top { display: flex; align-items: center; gap: 12px; padding: 12px 16px; }
.bsod-code-badge { background: linear-gradient(135deg,#7f1d1d,#ef4444); color: #fff; font-family: 'JetBrains Mono',monospace; font-size: 12px; font-weight: 700; padding: 6px 12px; border-radius: 8px; min-width: 80px; text-align: center; }
.bsod-entry-info { flex: 1; min-width: 0; }
.bsod-entry-desc { font-size: 12px; font-weight: 500; }
.bsod-entry-module { font-size: 10px; color: #f59e0b; margin-top: 2px; }
.bsod-entry-right { display: flex; align-items: center; gap: 10px; }
.bsod-entry-time { font-size: 11px; opacity: .5; }
.bsod-entry-detail { padding: 14px 16px; border-top: 1px solid rgba(255,255,255,.07); background: rgba(0,0,0,.15); display: flex; flex-direction: column; gap: 10px; }
.bsod-explain { background: rgba(239,68,68,.08); border: 1px solid rgba(239,68,68,.2); border-radius: 8px; padding: 12px; font-size: 12px; line-height: 1.6; }
.bsod-explain-title { display: flex; align-items: center; gap: 6px; font-weight: 600; font-size: 11px; opacity: .7; margin-bottom: 6px; text-transform: uppercase; }
.bsod-detail-grid { display: flex; flex-direction: column; gap: 6px; }
.bsod-kv { display: flex; gap: 12px; font-size: 12px; }
.bsod-k { min-width: 110px; color: var(--text-muted); flex-shrink: 0; }
.bsod-v { word-break: break-all; }
.mono { font-family: 'JetBrains Mono',monospace; font-size: 11px; }

/* Lookup */
.bsod-lookup { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: 14px; overflow: hidden; }
.bsod-lookup-header { display: flex; align-items: center; gap: 8px; padding: 12px 16px; font-size: 12px; font-weight: 600; opacity: .7; border-bottom: 1px solid var(--border); background: var(--bg-tertiary); }
.bsod-lookup-body { display: flex; gap: 8px; padding: 14px; }
.bsod-input { flex: 1; background: var(--bg-tertiary); border: 1px solid var(--border); border-radius: 8px; padding: 8px 12px; color: var(--text-primary); font-size: 12px; outline: none; }
.bsod-input:focus { border-color: var(--accent-primary); }
.bsod-lookup-result { padding: 0 14px 14px; font-size: 12px; line-height: 1.6; background: rgba(124,58,237,.08); border-top: 1px solid rgba(124,58,237,.2); padding: 12px 16px; color: var(--text-secondary); }

/* Buttons */
.bsod-btn { display: inline-flex; align-items: center; gap: 6px; padding: 8px 16px; border-radius: 8px; border: 1px solid var(--border); background: var(--bg-secondary); color: var(--text-secondary); font-size: 12px; cursor: pointer; transition: all 150ms; font-family: inherit; }
.bsod-btn:hover { color: var(--text-primary); }
.bsod-btn:disabled { opacity: .4; cursor: not-allowed; }
.bsod-btn-primary { background: rgba(239,68,68,.15); color: #ef4444; border-color: rgba(239,68,68,.3); }
.bsod-btn-primary:hover { background: rgba(239,68,68,.25); }

.bsod-loading { display: flex; align-items: center; gap: 10px; padding: 20px; font-size: 13px; color: var(--text-muted); }
.bsod-spinner { width: 16px; height: 16px; border: 2px solid rgba(255,255,255,.15); border-top-color: #ef4444; border-radius: 50%; animation: spin .8s linear infinite; }
@keyframes spin { to { transform: rotate(360deg); } }
</style>
