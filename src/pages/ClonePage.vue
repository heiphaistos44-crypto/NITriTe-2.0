<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Copy, HardDrive, RefreshCw, Play, AlertTriangle,
  CheckCircle, XCircle, Server, Cpu,
} from "lucide-vue-next";

const notify = useNotificationStore();

interface PartitionInfo {
  letter: string; label: string; size_gb: number; free_gb: number;
  file_system: string; is_system: boolean; is_boot: boolean;
}
interface DiskInfo {
  index: number; label: string; size_gb: number; disk_type: string;
  bus_type: string; partitions: PartitionInfo[];
}
interface CloneProgress { step: string; percent: number; message: string; }
interface CloneResult { success: boolean; method: string; message: string; duration_secs: number; }

// ── État ─────────────────────────────────────────────────────
type Tab = "image" | "robocopy";
const activeTab = ref<Tab>("image");
const disks = ref<DiskInfo[]>([]);
const loadingDisks = ref(false);
const sourceDrive = ref("");
const targetDrive = ref("");
const cloning = ref(false);
const progress = ref(0);
const progressMsg = ref("");
const result = ref<CloneResult | null>(null);
const confirmed = ref(false);

// Reset confirmation quand on change de méthode
watch(activeTab, () => {
  confirmed.value = false;
  targetDrive.value = "";
  result.value = null;
  progress.value = 0;
});

// ── Disques disponibles ───────────────────────────────────────
const drivesWithLetters = computed(() =>
  disks.value.flatMap(d =>
    d.partitions
      .filter(p => p.letter)
      .map(p => ({ ...p, disk_label: d.label, disk_index: d.index }))
  )
);
const availableTargets = computed(() =>
  drivesWithLetters.value.filter(p =>
    !p.is_system && !p.is_boot && p.letter !== sourceDrive.value
  )
);
const systemDrive = computed(() =>
  drivesWithLetters.value.find(p => p.is_boot || p.is_system)
);

async function loadDisks() {
  loadingDisks.value = true;
  disks.value = [];
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    disks.value = await invoke<DiskInfo[]>("get_disks_for_clone");
    if (systemDrive.value) sourceDrive.value = systemDrive.value.letter.replace(":", "");
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  loadingDisks.value = false;
}

// ── Écoute des événements de progression ─────────────────────
onMounted(async () => {
  const { listen } = await import("@tauri-apps/api/event");
  await listen<CloneProgress>("clone-progress", (ev) => {
    progress.value = ev.payload.percent;
    progressMsg.value = ev.payload.message;
  });
  await loadDisks();
});

// ── Clonage Image Système (wbadmin) ─────────────────────────
async function startSystemImage() {
  if (!targetDrive.value || !confirmed.value) return;
  cloning.value = true; result.value = null; progress.value = 5;
  progressMsg.value = "Lancement de Windows Backup (wbadmin)...";
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    result.value = await invoke<CloneResult>("start_system_image", { targetDrive: targetDrive.value });
    if (result.value.success) notify.success("Image système créée", result.value.message);
    else notify.error("Clonage échoué", result.value.message);
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  cloning.value = false;
}

// ── Clonage Robocopy ─────────────────────────────────────────
async function startRobocopy() {
  if (!sourceDrive.value || !targetDrive.value || !confirmed.value) return;
  cloning.value = true; result.value = null; progress.value = 5;
  progressMsg.value = "Démarrage Robocopy...";
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    result.value = await invoke<CloneResult>("start_robocopy_clone", {
      sourceDrive: sourceDrive.value,
      targetDrive: targetDrive.value,
    });
    if (result.value.success) notify.success("Clonage terminé", result.value.message);
    else notify.error("Erreur Robocopy", result.value.message);
  } catch (e: any) {
    notify.error("Erreur", String(e));
  }
  cloning.value = false;
}

function formatSize(gb: number) {
  return gb >= 1000 ? `${(gb / 1000).toFixed(1)} To` : `${gb.toFixed(0)} Go`;
}
</script>

<template>
  <div class="clone-page">
    <DiagBanner :icon="Copy" title="Clonage Système" desc="Image système complète ou copie disque-à-disque pour migration Windows 10/11" color="blue" />

    <!-- Sélecteur de méthode -->
    <div class="method-tabs">
      <button class="method-tab" :class="{ active: activeTab === 'image' }" @click="activeTab = 'image'">
        <Server :size="15" /> Image Système Windows
      </button>
      <button class="method-tab" :class="{ active: activeTab === 'robocopy' }" @click="activeTab = 'robocopy'">
        <Copy :size="15" /> Clone Disque-à-Disque
      </button>
    </div>

    <!-- Refresh disks -->
    <div class="toolbar">
      <NButton variant="ghost" size="sm" :loading="loadingDisks" @click="loadDisks">
        <RefreshCw :size="13" /> Actualiser les disques
      </NButton>
      <span v-if="!loadingDisks" class="count-badge">{{ disks.length }} disque(s) détecté(s)</span>
    </div>

    <!-- Loading -->
    <div v-if="loadingDisks" class="loading-state"><NSpinner :size="22" /><p>Détection des disques...</p></div>

    <template v-else>

      <!-- ══ IMAGE SYSTÈME (wbadmin) ══ -->
      <template v-if="activeTab === 'image'">
        <div class="info-banner warning">
          <AlertTriangle :size="14" />
          <span><strong>Droits administrateur obligatoires.</strong> Si l'opération échoue, faites clic droit sur Nitrite → "Exécuter en tant qu'administrateur" et réessayez.</span>
        </div>
        <div class="info-banner info">
          <AlertTriangle :size="14" />
          <span>wbadmin crée une Image Système Windows complète (OS + données) récupérable depuis le menu de démarrage. Le lecteur cible doit être en NTFS et avoir au moins autant d'espace libre que C:.</span>
        </div>

        <div class="config-grid">
          <div class="config-card">
            <p class="config-label"><Cpu :size="13" /> Système à capturer</p>
            <div class="drive-badge system">
              <HardDrive :size="14" />
              <span>C:\ — Windows (système actuel)</span>
              <span class="badge-sub">+ toutes les partitions critiques</span>
            </div>
          </div>

          <div class="config-card">
            <p class="config-label"><HardDrive :size="13" /> Destination de l'image</p>
            <select v-model="targetDrive" class="drive-select">
              <option value="">— Choisir un lecteur cible —</option>
              <option v-for="p in availableTargets" :key="p.letter" :value="p.letter.replace(':', '')">
                {{ p.letter }} — {{ p.disk_label }} — {{ formatSize(p.size_gb) }} ({{ formatSize(p.free_gb) }} libres)
              </option>
            </select>
            <p class="config-hint">⚠ Le lecteur cible doit avoir plus d'espace que le système source</p>
          </div>
        </div>

        <div class="confirm-row">
          <label class="confirm-label">
            <input type="checkbox" v-model="confirmed" />
            Je confirme que le lecteur cible peut être écrasé et que j'ai vérifié qu'il contient assez d'espace
          </label>
        </div>

        <NButton variant="primary" :disabled="!targetDrive || !confirmed || cloning" :loading="cloning" @click="startSystemImage">
          <Play :size="14" /> Créer l'Image Système
        </NButton>
      </template>

      <!-- ══ CLONE DISQUE-À-DISQUE (Robocopy) ══ -->
      <template v-if="activeTab === 'robocopy'">
        <div class="info-banner info">
          <AlertTriangle :size="14" />
          <span>Robocopy /MIR clone tous les fichiers et dossiers avec attributs et permissions. Idéal pour migrer des données vers un nouveau disque. Ne copie pas le secteur de démarrage (pour migration OS, préférez l'Image Système).</span>
        </div>

        <div class="config-grid">
          <div class="config-card">
            <p class="config-label"><HardDrive :size="13" /> Source</p>
            <select v-model="sourceDrive" class="drive-select">
              <option value="">— Choisir la source —</option>
              <option v-for="p in drivesWithLetters" :key="p.letter" :value="p.letter.replace(':', '')">
                {{ p.letter }} — {{ p.label || p.disk_label }} — {{ formatSize(p.size_gb) }}
                {{ p.is_system ? '(Système)' : '' }}
              </option>
            </select>
          </div>

          <div class="config-card">
            <p class="config-label"><HardDrive :size="13" /> Destination</p>
            <select v-model="targetDrive" class="drive-select">
              <option value="">— Choisir la destination —</option>
              <option
                v-for="p in drivesWithLetters.filter(p => p.letter.replace(':', '') !== sourceDrive)"
                :key="p.letter"
                :value="p.letter.replace(':', '')"
              >
                {{ p.letter }} — {{ p.label || p.disk_label }} — {{ formatSize(p.size_gb) }} ({{ formatSize(p.free_gb) }} libres)
              </option>
            </select>
          </div>
        </div>

        <div class="confirm-row">
          <label class="confirm-label">
            <input type="checkbox" v-model="confirmed" />
            Je confirme que la destination peut être écrasée (les fichiers existants seront supprimés si /MIR)
          </label>
        </div>

        <NButton variant="primary" :disabled="!sourceDrive || !targetDrive || !confirmed || cloning" :loading="cloning" @click="startRobocopy">
          <Play :size="14" /> Lancer le Clone
        </NButton>
      </template>

      <!-- Progression -->
      <div v-if="cloning || progress > 0" class="progress-section">
        <div class="progress-header">
          <NSpinner v-if="cloning" :size="14" />
          <span>{{ progressMsg }}</span>
        </div>
        <NProgress :value="progress" showLabel size="lg" />
        <p class="progress-note">Cette opération peut prendre plusieurs minutes selon la taille du disque.</p>
      </div>

      <!-- Résultat -->
      <div v-if="result" class="result-card" :class="result.success ? 'success' : 'error'">
        <CheckCircle v-if="result.success" :size="20" />
        <XCircle v-else :size="20" />
        <div>
          <p class="result-title">{{ result.success ? 'Opération réussie' : 'Opération échouée' }}</p>
          <p class="result-method">Méthode : {{ result.method }}</p>
          <p class="result-msg">{{ result.message }}</p>
          <p class="result-duration">Durée : {{ Math.floor(result.duration_secs / 60) }}min {{ result.duration_secs % 60 }}s</p>
        </div>
      </div>

      <!-- Disques détectés -->
      <div v-if="disks.length > 0" class="disks-list">
        <p class="section-title">Disques détectés</p>
        <div v-for="disk in disks" :key="disk.index" class="disk-card">
          <div class="disk-header">
            <HardDrive :size="16" style="color:var(--accent-primary)" />
            <span class="disk-name">Disque {{ disk.index }} — {{ disk.label }}</span>
            <span class="disk-meta">{{ formatSize(disk.size_gb) }} · {{ disk.disk_type }} · {{ disk.bus_type }}</span>
          </div>
          <div class="partitions-list">
            <div v-for="p in disk.partitions" :key="p.letter" class="part-row">
              <span class="part-letter">{{ p.letter || 'Sans lettre' }}</span>
              <span class="part-label">{{ p.label || '—' }}</span>
              <span class="part-fs">{{ p.file_system }}</span>
              <span class="part-size">{{ formatSize(p.size_gb) }}</span>
              <span class="part-free" :style="{ color: (p.free_gb / p.size_gb) < 0.1 ? 'var(--danger)' : 'var(--success)' }">
                {{ formatSize(p.free_gb) }} libres
              </span>
              <span v-if="p.is_system" class="badge-sys">SYSTÈME</span>
              <span v-if="p.is_boot" class="badge-boot">BOOT</span>
            </div>
          </div>
        </div>
      </div>

    </template>
  </div>
</template>

<style scoped>
.clone-page { display: flex; flex-direction: column; gap: 14px; }

.method-tabs { display: flex; gap: 6px; }
.method-tab {
  display: flex; align-items: center; gap: 7px;
  padding: 9px 16px; border-radius: var(--radius-md); border: 1.5px solid var(--border);
  background: var(--bg-tertiary); cursor: pointer; font-family: inherit;
  font-size: 13px; color: var(--text-secondary); transition: all 0.15s;
}
.method-tab:hover { border-color: var(--text-muted); color: var(--text-primary); }
.method-tab.active { border-color: var(--accent-primary); color: var(--accent-primary); background: var(--bg-secondary); }

.toolbar { display: flex; align-items: center; gap: 10px; }
.count-badge { font-size: 11px; color: var(--text-muted); font-family: monospace; }

.info-banner {
  display: flex; gap: 8px; align-items: flex-start; padding: 10px 14px;
  border-radius: var(--radius-md); font-size: 12px; line-height: 1.5;
}
.info-banner.info    { background: var(--info-muted);    color: var(--info);    border: 1px solid var(--info); }
.info-banner.warning { background: var(--warning-muted); color: var(--warning); border: 1px solid var(--warning); }

.config-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
@media (max-width: 700px) { .config-grid { grid-template-columns: 1fr; } }
.config-card {
  display: flex; flex-direction: column; gap: 8px; padding: 14px;
  background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-lg);
}
.config-label { font-size: 12px; font-weight: 700; color: var(--text-secondary); display: flex; align-items: center; gap: 6px; }
.drive-badge {
  display: flex; align-items: center; gap: 8px; padding: 10px 12px;
  background: var(--bg-tertiary); border-radius: var(--radius-md); border: 1px solid var(--border); font-size: 13px;
}
.drive-badge.system { border-color: var(--accent-primary); color: var(--accent-primary); }
.badge-sub { font-size: 11px; color: var(--text-muted); margin-left: auto; }
.drive-select {
  padding: 8px 10px; border: 1px solid var(--border); border-radius: var(--radius-md);
  background: var(--bg-tertiary); color: var(--text-primary); font-family: inherit; font-size: 12px;
  outline: none; cursor: pointer; transition: border-color 0.15s;
}
.drive-select:focus { border-color: var(--accent-primary); }
.config-hint { font-size: 11px; color: var(--warning); }

.confirm-row { padding: 10px 0; }
.confirm-label {
  display: flex; align-items: center; gap: 8px; font-size: 12px;
  color: var(--text-secondary); cursor: pointer;
}
.confirm-label input { cursor: pointer; accent-color: var(--accent-primary); }

.progress-section { display: flex; flex-direction: column; gap: 8px; padding: 12px 14px; background: var(--bg-secondary); border-radius: var(--radius-md); border: 1px solid var(--border); }
.progress-header { display: flex; align-items: center; gap: 8px; font-size: 12px; color: var(--text-secondary); }
.progress-note { font-size: 11px; color: var(--text-muted); }

.result-card { display: flex; align-items: flex-start; gap: 12px; padding: 14px; border-radius: var(--radius-lg); border: 1px solid; }
.result-card.success { background: var(--success-muted); border-color: var(--success); color: var(--success); }
.result-card.error { background: var(--danger-muted); border-color: var(--danger); color: var(--danger); }
.result-title { font-weight: 700; font-size: 14px; }
.result-method, .result-msg, .result-duration { font-size: 12px; color: var(--text-secondary); margin-top: 3px; }

.disks-list { display: flex; flex-direction: column; gap: 10px; }
.section-title { font-size: 11px; font-weight: 700; text-transform: uppercase; letter-spacing: .06em; color: var(--text-muted); }
.disk-card { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius-lg); overflow: hidden; }
.disk-header { display: flex; align-items: center; gap: 8px; padding: 10px 14px; background: var(--bg-tertiary); font-size: 13px; font-weight: 600; }
.disk-name { flex: 1; color: var(--text-primary); }
.disk-meta { font-size: 11px; color: var(--text-muted); font-family: monospace; }
.partitions-list { padding: 0; }
.part-row { display: flex; align-items: center; gap: 10px; padding: 6px 14px; font-size: 12px; border-top: 1px solid var(--border); }
.part-letter { font-family: monospace; font-weight: 700; color: var(--accent-primary); min-width: 36px; }
.part-label { flex: 1; color: var(--text-primary); }
.part-fs { color: var(--text-muted); min-width: 60px; }
.part-size { color: var(--text-secondary); min-width: 70px; text-align: right; font-family: monospace; }
.part-free { min-width: 90px; text-align: right; font-family: monospace; font-size: 11px; }
.badge-sys, .badge-boot { font-size: 10px; font-weight: 700; padding: 2px 6px; border-radius: 4px; }
.badge-sys { background: var(--accent-muted); color: var(--accent-primary); }
.badge-boot { background: var(--warning-muted); color: var(--warning); }

.loading-state { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 40px; color: var(--text-muted); font-size: 13px; }
</style>
