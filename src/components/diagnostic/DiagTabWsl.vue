<template>
  <div class="diag-tab-content">
    <div class="diag-section-header"><Terminal :size="16" /> WSL — Windows Subsystem for Linux</div>

    <button class="diag-btn diag-btn-primary" :disabled="loading" style="margin-bottom:12px" @click="load">
      <RefreshCw :size="13" /> Actualiser
    </button>

    <div v-if="loading" class="diag-loading">Lecture des informations WSL...</div>

    <div v-else-if="info">
      <!-- Not installed -->
      <div v-if="!info.installed" class="wsl-not-installed">
        <Terminal :size="32" style="opacity:.3" />
        <div>
          <div style="font-weight:600;margin-bottom:4px">WSL non disponible</div>
          <div style="font-size:12px;opacity:.6">{{ info.error || 'WSL n\'est pas installé sur ce système.' }}</div>
          <div style="font-size:12px;margin-top:8px;opacity:.7">Pour installer WSL : <code>wsl --install</code> dans un terminal administrateur</div>
        </div>
      </div>

      <div v-else>
        <!-- Info cards -->
        <div class="wsl-cards">
          <div class="wsl-card">
            <div class="wsl-card-label">Version WSL</div>
            <div class="wsl-card-val">{{ info.wsl_version || 'N/A' }}</div>
          </div>
          <div class="wsl-card">
            <div class="wsl-card-label">Version Kernel</div>
            <div class="wsl-card-val" style="font-size:13px">{{ info.kernel_version || 'N/A' }}</div>
          </div>
          <div class="wsl-card">
            <div class="wsl-card-label">WSL par défaut</div>
            <div class="wsl-card-val">WSL {{ info.default_version }}</div>
          </div>
          <div class="wsl-card">
            <div class="wsl-card-label">Distributions</div>
            <div class="wsl-card-val">{{ info.distros.length }}</div>
          </div>
        </div>

        <!-- Default version changer -->
        <div style="display:flex;gap:8px;align-items:center;margin:12px 0">
          <span style="font-size:12px;opacity:.7">Version par défaut :</span>
          <button class="diag-btn diag-btn-sm" :class="info.default_version===1?'btn-active':''" @click="setVersion(1)">WSL 1</button>
          <button class="diag-btn diag-btn-sm" :class="info.default_version===2?'btn-active':''" @click="setVersion(2)">WSL 2</button>
          <span v-if="versionMsg" class="wsl-ok">{{ versionMsg }}</span>
        </div>

        <!-- Distros -->
        <div class="diag-section-header"><List :size="16" /> Distributions installées</div>
        <div v-if="info.distros.length === 0" style="opacity:.5;font-size:12px;padding:10px">Aucune distribution installée.</div>
        <div class="wsl-distros">
          <div v-for="d in info.distros" :key="d.name" class="wsl-distro" :class="d.is_default ? 'wsl-default' : ''">
            <div class="wsl-distro-name">
              <Star v-if="d.is_default" :size="12" style="color:#f59e0b" />
              {{ d.name }}
              <span v-if="d.is_default" style="font-size:10px;color:#f59e0b"> (défaut)</span>
            </div>
            <div class="wsl-distro-meta">
              <span :class="d.state === 'Running' ? 'wsl-running' : 'wsl-stopped'">{{ d.state }}</span>
              <span style="opacity:.5">WSL {{ d.version }}</span>
            </div>
          </div>
        </div>

        <!-- Terminal -->
        <div class="diag-section-header" style="margin-top:16px"><Terminal :size="16" /> Terminal WSL</div>
        <div style="display:flex;gap:8px;margin-bottom:8px;flex-wrap:wrap">
          <select v-model="selectedDistro" class="diag-input" style="width:180px">
            <option value="">Distribution par défaut</option>
            <option v-for="d in info.distros" :key="d.name" :value="d.name">{{ d.name }}</option>
          </select>
          <input v-model="wslCmd" class="diag-input" style="flex:1" placeholder="Commande (ex: uname -a)" @keyup.enter="runCmd" />
          <button class="diag-btn" :disabled="cmdLoading || !wslCmd" @click="runCmd">
            <Play :size="13" /> Exécuter
          </button>
        </div>
        <div v-if="cmdLoading" class="diag-loading">Exécution...</div>
        <pre v-else-if="cmdOutput" class="wsl-output">{{ cmdOutput }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Terminal, RefreshCw, List, Star, Play } from 'lucide-vue-next'

interface WslDistro { name: string; state: string; version: number; is_default: boolean }
interface WslInfo { installed: boolean; default_version: number; distros: WslDistro[]; kernel_version: string; wsl_version: string; error: string }

const loading = ref(false)
const info = ref<WslInfo | null>(null)
const selectedDistro = ref('')
const wslCmd = ref('')
const cmdLoading = ref(false)
const cmdOutput = ref('')
const versionMsg = ref('')

async function load() {
  loading.value = true
  try { info.value = await invoke<WslInfo>('get_wsl_info') }
  finally { loading.value = false }
}

async function setVersion(v: number) {
  try {
    await invoke<string>('wsl_set_default_version', { version: v })
    versionMsg.value = `Version WSL ${v} définie`
    setTimeout(() => { versionMsg.value = '' }, 2000)
    await load()
  } catch(e) { versionMsg.value = String(e) }
}

async function runCmd() {
  if (!wslCmd.value.trim()) return
  cmdLoading.value = true; cmdOutput.value = ''
  try {
    cmdOutput.value = await invoke<string>('wsl_run_command', { distro: selectedDistro.value, command: wslCmd.value })
  } catch(e) { cmdOutput.value = String(e) }
  finally { cmdLoading.value = false }
}

onMounted(load)
</script>

<style scoped>
.wsl-not-installed { display: flex; gap: 16px; align-items: flex-start; padding: 20px; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; }
.wsl-cards { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin-bottom: 12px; }
.wsl-card { background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; padding: 12px; text-align: center; }
.wsl-card-label { font-size: 10px; opacity: .5; text-transform: uppercase; margin-bottom: 4px; }
.wsl-card-val { font-size: 18px; font-weight: 700; }
.wsl-distros { display: flex; flex-direction: column; gap: 4px; }
.wsl-distro { display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; background: var(--bg-secondary, #1e1e2e); border: 1px solid transparent; border-radius: 6px; }
.wsl-default { border-color: rgba(245,158,11,.3) !important; background: rgba(245,158,11,.05) !important; }
.wsl-distro-name { display: flex; align-items: center; gap: 6px; font-size: 13px; font-weight: 500; }
.wsl-distro-meta { display: flex; gap: 12px; font-size: 11px; }
.wsl-running { color: #22c55e; }
.wsl-stopped { color: #6b7280; }
.wsl-output { background: #0d0d1a; border: 1px solid var(--border-color, #333); border-radius: 6px; padding: 12px; font-size: 11px; font-family: monospace; white-space: pre-wrap; max-height: 300px; overflow-y: auto; }
.wsl-ok { color: #22c55e; font-size: 12px; }
.btn-active { background: var(--accent, #7c3aed) !important; color: #fff !important; }
</style>
