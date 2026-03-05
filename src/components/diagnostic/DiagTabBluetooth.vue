<template>
  <div class="diag-tab-content">
    <div class="diag-section-header"><Bluetooth :size="16" /> Bluetooth</div>

    <div style="display:flex;gap:8px;margin-bottom:12px;align-items:center">
      <button class="diag-btn diag-btn-primary" :disabled="loading" @click="load">
        <RefreshCw :size="13" /> Actualiser
      </button>
      <span v-if="msg" :class="msgErr ? 'bt-err' : 'bt-ok'">{{ msg }}</span>
    </div>

    <div v-if="loading" class="diag-loading">Lecture des périphériques Bluetooth...</div>

    <div v-else-if="report">
      <!-- Not available -->
      <div v-if="!report.bt_available" class="bt-unavailable">
        <Bluetooth :size="28" style="opacity:.3" />
        <div>
          <div style="font-weight:600">Bluetooth non disponible</div>
          <div style="font-size:12px;opacity:.6;margin-top:4px">{{ report.error || 'Aucun adaptateur Bluetooth détecté.' }}</div>
        </div>
      </div>

      <div v-else>
        <!-- Adapters -->
        <div class="diag-section-header"><Cpu :size="16" /> Adaptateurs ({{ report.adapters.length }})</div>
        <div class="bt-adapters">
          <div v-for="a in report.adapters" :key="a.name" class="bt-adapter">
            <div class="bt-adapter-icon">
              <div class="bt-dot" :class="a.enabled ? 'bt-on' : 'bt-off'" />
            </div>
            <div class="bt-adapter-info">
              <div class="bt-name">{{ a.name }}</div>
              <div class="bt-meta">
                <span>{{ a.manufacturer || 'Fabricant inconnu' }}</span>
                <span v-if="a.address" style="opacity:.5">{{ a.address }}</span>
                <span :class="a.enabled ? 'bt-on-text' : 'bt-off-text'">{{ a.enabled ? 'Actif' : 'Inactif' }}</span>
              </div>
            </div>
            <div style="display:flex;gap:6px">
              <button class="diag-btn diag-btn-sm" @click="toggleBt(true)">Activer</button>
              <button class="diag-btn diag-btn-sm" @click="toggleBt(false)">Désactiver</button>
            </div>
          </div>
        </div>

        <!-- Devices -->
        <div class="diag-section-header" style="margin-top:16px">
          <Smartphone :size="16" /> Périphériques associés ({{ report.devices.length }})
        </div>
        <div v-if="report.devices.length === 0" style="opacity:.5;font-size:12px;padding:10px">
          Aucun périphérique Bluetooth associé.
        </div>
        <div class="bt-devices">
          <div v-for="d in report.devices" :key="d.address" class="bt-device">
            <div class="bt-device-cat-icon">{{ catIcon(d.category) }}</div>
            <div class="bt-device-info">
              <div class="bt-name">{{ d.name }}</div>
              <div class="bt-meta">
                <span class="bt-cat-badge">{{ d.category }}</span>
                <span v-if="d.manufacturer" style="opacity:.6">{{ d.manufacturer }}</span>
                <span v-if="d.address" style="font-family:monospace;font-size:10px;opacity:.4">{{ d.address }}</span>
              </div>
            </div>
            <div class="bt-device-status">
              <span :class="d.connected ? 'bt-on-text' : 'bt-off-text'">
                {{ d.connected ? '● Connecté' : '○ Déconnecté' }}
              </span>
              <span v-if="d.paired" style="font-size:10px;opacity:.5">Associé</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Bluetooth, RefreshCw, Cpu, Smartphone } from 'lucide-vue-next'

interface BluetoothAdapter { name: string; address: string; enabled: boolean; manufacturer: string; driver_version: string }
interface BluetoothDevice { name: string; address: string; device_class: string; paired: boolean; connected: boolean; trusted: boolean; rssi: number; manufacturer: string; category: string }
interface BluetoothReport { adapters: BluetoothAdapter[]; devices: BluetoothDevice[]; bt_available: boolean; error: string }

const loading = ref(false)
const report = ref<BluetoothReport | null>(null)
const msg = ref(''); const msgErr = ref(false)

function showMsg(text: string, err = false) {
  msg.value = text; msgErr.value = err; setTimeout(() => { msg.value = '' }, 3000)
}

async function load() {
  loading.value = true
  try { report.value = await invoke<BluetoothReport>('get_bluetooth_info') }
  finally { loading.value = false }
}

async function toggleBt(enable: boolean) {
  try {
    const r = await invoke<string>('toggle_bluetooth', { enable })
    showMsg(r); setTimeout(load, 1500)
  } catch(e) { showMsg(String(e), true) }
}

function catIcon(cat: string): string {
  const m: Record<string, string> = { Audio: '🎧', Souris: '🖱️', Clavier: '⌨️', Manette: '🎮', Téléphone: '📱', Autre: '📡' }
  return m[cat] || '📡'
}

onMounted(load)
</script>

<style scoped>
.bt-unavailable { display: flex; gap: 16px; align-items: center; padding: 20px; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; }
.bt-adapters { display: flex; flex-direction: column; gap: 6px; }
.bt-adapter { display: flex; align-items: center; gap: 10px; padding: 10px 14px; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; }
.bt-adapter-icon { display: flex; align-items: center; }
.bt-dot { width: 10px; height: 10px; border-radius: 50%; }
.bt-on { background: #22c55e; box-shadow: 0 0 6px #22c55e; }
.bt-off { background: #6b7280; }
.bt-adapter-info { flex: 1; }
.bt-devices { display: flex; flex-direction: column; gap: 6px; }
.bt-device { display: flex; align-items: center; gap: 10px; padding: 10px 14px; background: var(--bg-secondary, #1e1e2e); border: 1px solid var(--border-color, #333); border-radius: 8px; }
.bt-device-cat-icon { font-size: 20px; min-width: 28px; text-align: center; }
.bt-device-info { flex: 1; }
.bt-name { font-size: 13px; font-weight: 500; }
.bt-meta { display: flex; align-items: center; gap: 10px; font-size: 11px; margin-top: 2px; }
.bt-cat-badge { background: rgba(124,58,237,.15); color: var(--accent, #7c3aed); border: 1px solid rgba(124,58,237,.3); border-radius: 4px; padding: 1px 6px; font-size: 10px; }
.bt-device-status { display: flex; flex-direction: column; align-items: flex-end; gap: 2px; font-size: 12px; }
.bt-on-text { color: #22c55e; }
.bt-off-text { color: #6b7280; }
.bt-ok { color: #22c55e; font-size: 12px; }
.bt-err { color: #ef4444; font-size: 12px; }
</style>
