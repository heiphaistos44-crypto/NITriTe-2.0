<template>
  <div class="diag-tab-content">
    <div class="diag-section-header"><FileText :size="16" /> Éditeur Hosts</div>

    <div style="display:flex;gap:8px;margin-bottom:12px;flex-wrap:wrap">
      <button class="diag-btn diag-btn-primary" :disabled="loading" @click="loadEntries">
        <RefreshCw :size="13" /> Actualiser
      </button>
      <button class="diag-btn" @click="doBackup">
        <Save :size="13" /> Sauvegarder (.bak)
      </button>
      <span v-if="msg" :class="msgErr ? 'hosts-err' : 'hosts-ok'">{{ msg }}</span>
    </div>

    <div v-if="loading" class="diag-loading">Lecture du fichier hosts...</div>

    <div v-else>
      <!-- Entries table -->
      <table class="diag-table" v-if="entries.length > 0">
        <thead>
          <tr>
            <th style="width:40px">Actif</th>
            <th>IP</th>
            <th>Hostname</th>
            <th>Commentaire</th>
            <th>Ligne</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="e in entries" :key="e.line_number" :class="!e.active ? 'row-inactive' : ''">
            <td>
              <div class="hosts-toggle" :class="e.active ? 'tog-on' : 'tog-off'"
                   @click="toggleEntry(e.line_number, !e.active)" title="Activer/Désactiver">
                {{ e.active ? '●' : '○' }}
              </div>
            </td>
            <td style="font-family:monospace;font-size:12px">{{ e.ip }}</td>
            <td style="font-size:12px">{{ e.hostname }}</td>
            <td style="font-size:11px;opacity:.6">{{ e.comment }}</td>
            <td style="font-size:11px;opacity:.4">{{ e.line_number }}</td>
            <td>
              <button class="diag-btn diag-btn-sm" style="color:#ef4444;border-color:#ef4444"
                @click="deleteEntry(e.line_number)">Supprimer</button>
            </td>
          </tr>
        </tbody>
      </table>
      <div v-else style="opacity:.5;font-size:13px;padding:12px">Aucune entrée trouvée.</div>

      <!-- Add entry -->
      <div class="diag-section-header" style="margin-top:20px"><Plus :size="16" /> Ajouter une entrée</div>
      <div class="hosts-add-form">
        <input v-model="newIp" class="diag-input" placeholder="IP (ex: 127.0.0.1)" style="width:160px" />
        <input v-model="newHost" class="diag-input" placeholder="Hostname" style="flex:1" />
        <input v-model="newComment" class="diag-input" placeholder="Commentaire (optionnel)" style="flex:1" />
        <button class="diag-btn diag-btn-primary" :disabled="!newIp || !newHost" @click="addEntry">
          <Plus :size="13" /> Ajouter
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { FileText, RefreshCw, Save, Plus } from 'lucide-vue-next'

interface HostsEntry { ip: string; hostname: string; comment: string; active: boolean; line_number: number }

const entries = ref<HostsEntry[]>([])
const loading = ref(false)
const msg = ref('')
const msgErr = ref(false)
const newIp = ref('')
const newHost = ref('')
const newComment = ref('')

function showMsg(text: string, err = false) {
  msg.value = text; msgErr.value = err
  setTimeout(() => { msg.value = '' }, 3000)
}

async function loadEntries() {
  loading.value = true
  try { entries.value = await invoke<HostsEntry[]>('get_hosts_entries') }
  finally { loading.value = false }
}

async function addEntry() {
  try {
    const r = await invoke<string>('add_hosts_entry', { ip: newIp.value, hostname: newHost.value, comment: newComment.value })
    showMsg(r)
    newIp.value = ''; newHost.value = ''; newComment.value = ''
    await loadEntries()
  } catch(e) { showMsg(String(e), true) }
}

async function deleteEntry(lineNumber: number) {
  try {
    await invoke<string>('delete_hosts_entry', { lineNumber })
    showMsg('Entrée supprimée')
    await loadEntries()
  } catch(e) { showMsg(String(e), true) }
}

async function toggleEntry(lineNumber: number, enable: boolean) {
  try {
    await invoke<string>('toggle_hosts_entry', { lineNumber, enable })
    await loadEntries()
  } catch(e) { showMsg(String(e), true) }
}

async function doBackup() {
  try {
    const r = await invoke<string>('backup_hosts')
    showMsg(r)
  } catch(e) { showMsg(String(e), true) }
}

onMounted(loadEntries)
</script>

<style scoped>
.row-inactive { opacity: .45; }
.hosts-toggle { cursor: pointer; font-size: 16px; text-align: center; transition: color .15s; }
.tog-on { color: #22c55e; }
.tog-off { color: #6b7280; }
.hosts-add-form { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }
.hosts-ok { color: #22c55e; font-size: 12px; }
.hosts-err { color: #ef4444; font-size: 12px; }
</style>
