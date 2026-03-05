<script setup lang="ts">
import { ref, onMounted } from "vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import { Wrench, AlertTriangle, CheckCircle, RefreshCw } from "lucide-vue-next";

interface SystemHealthStatus {
  dism_health: string; sfc_last_run: string; sfc_result: string;
  pending_reboot: boolean; disk_errors: string[];
  windows_version: string; cbs_log_size_kb: number;
}
interface RepairResult { command: string; success: boolean; output: string; duration_secs: number; }

const health = ref<SystemHealthStatus | null>(null);
const healthLoading = ref(true);
const repairLoading = ref<string | null>(null);
const repairResult = ref<RepairResult | null>(null);
const repairError = ref("");

const REPAIR_GROUPS = [
  {
    label: "Réseau",
    icon: "🌐",
    actions: [
      { key: "flush_dns",    label: "Vider cache DNS",      desc: "ipconfig /flushdns",              fast: true  },
      { key: "winsock",      label: "Reset Winsock",        desc: "netsh winsock reset",             fast: true  },
      { key: "ip_reset",     label: "Reset pile IP",        desc: "netsh int ip reset",              fast: true  },
      { key: "register_dns", label: "Enregistrer DNS",      desc: "ipconfig /registerdns",           fast: true  },
      { key: "arp_flush",    label: "Vider cache ARP",      desc: "arp -d *",                        fast: true  },
      { key: "reset_tcp",    label: "Reset TCP/UDP",        desc: "netsh int tcp/udp reset",         fast: true  },
      { key: "net_reset_all",label: "Reset réseau complet", desc: "Release + flush + winsock + renew",fast: false },
    ],
  },
  {
    label: "Intégrité Système",
    icon: "🛡️",
    actions: [
      { key: "sfc",          label: "SFC /scannow",         desc: "Analyse intégrité (2-5 min)",     fast: false },
      { key: "dism_scan",    label: "DISM ScanHealth",      desc: "Analyse store composants (3-5min)",fast: false },
      { key: "dism_restore", label: "DISM RestoreHealth",   desc: "Répare store WU (5-15 min)",      fast: false },
      { key: "dism_startcomp",label:"DISM /StartComponent", desc: "Nettoie composants (5-20 min)",   fast: false },
      { key: "repair_wmi",   label: "Réparer WMI",          desc: "winmgmt /resetrepository",        fast: true  },
      { key: "gpupdate",     label: "gpupdate /force",      desc: "Refresh stratégies groupe",       fast: true  },
      { key: "bcdedit_check",label: "Lire BCD",             desc: "bcdedit /enum all (lecture seule)",fast: true },
    ],
  },
  {
    label: "Mises à jour & Sécurité",
    icon: "🔄",
    actions: [
      { key: "windows_update_reset",label:"Reset Windows Update",desc:"Stop services + rename SD.old + restart",fast:false },
      { key: "defender_update",     label:"Màj Defender",        desc:"Update-MpSignature",           fast: false },
      { key: "defender_scan",       label:"Scan rapide Defender", desc:"Start-MpScan -QuickScan",     fast: false },
      { key: "firewall_reset",      label:"Reset Pare-feu",       desc:"netsh advfirewall reset",     fast: true  },
    ],
  },
  {
    label: "Cache & Nettoyage",
    icon: "🧹",
    actions: [
      { key: "icon_cache",      label: "Rebuild icônes",       desc: "Supprime IconCache.db + restart Explorer", fast: true  },
      { key: "thumbnail_cache", label: "Vider miniatures",     desc: "Supprime thumbcache_*.db",     fast: true  },
      { key: "temp_cleanup",    label: "Nettoyer %TEMP%",      desc: "%TEMP% + Windows\\Temp + Prefetch",fast: true },
      { key: "memory_dumps",    label: "Supprimer dumps",      desc: "Minidump + MEMORY.DMP",        fast: true  },
      { key: "clear_prefetch",  label: "Vider Prefetch",       desc: "C:\\Windows\\Prefetch\\*.pf",  fast: true  },
      { key: "clear_event_logs",label: "Vider journaux",       desc: "wevtutil — tous les logs",     fast: true  },
    ],
  },
  {
    label: "Services",
    icon: "⚙️",
    actions: [
      { key: "print_spooler",  label: "Reset Spouleur",        desc: "Stop + clear PRINTERS + start", fast: true  },
      { key: "search_reindex", label: "Réindexer Search",      desc: "Stop WSearch + rm edb + restart",fast: false },
      { key: "time_sync",      label: "Sync heure Windows",    desc: "w32tm /resync /force",          fast: true  },
      { key: "wsreset",        label: "Réparer Store",         desc: "wsreset.exe",                   fast: true  },
    ],
  },
  {
    label: "Disques",
    icon: "💾",
    actions: [
      { key: "chkdsk_c",      label: "CHKDSK C: /scan",        desc: "Analyse disque système",        fast: false },
      { key: "defrag_c",      label: "Optimiser C:",            desc: "defrag C: /U /V",               fast: false },
    ],
  },
  {
    label: "Sauvegarde",
    icon: "💿",
    actions: [
      { key: "restore_point", label: "Créer point restauration", desc: "Checkpoint-Computer",         fast: false },
    ],
  },
];

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    health.value = await invoke<SystemHealthStatus>("check_system_health");
  } catch {}
  finally { healthLoading.value = false; }
});

async function runRepair(key: string) {
  repairLoading.value = key;
  repairResult.value = null;
  repairError.value = "";
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    repairResult.value = await invoke<RepairResult>("run_repair_command", { repairType: key });
  } catch (e: any) { repairError.value = e?.toString() ?? "Erreur"; }
  finally { repairLoading.value = null; }
}

function healthColor(s: string) {
  if (!s) return 'neutral';
  const low = s.toLowerCase();
  if (low.includes('healthy') || low.includes('no violations') || low.includes('repaired')) return 'success';
  if (low.includes('repairable') || low.includes('corrupt')) return 'danger';
  return 'neutral';
}
</script>

<template>
  <div style="display:flex;flex-direction:column;gap:14px">

    <!-- État santé système -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">
        <CheckCircle :size="13" style="display:inline;margin-right:4px" />État de santé système
      </p>
      <div v-if="healthLoading" style="display:flex;align-items:center;gap:8px;color:var(--text-secondary)">
        <NSpinner :size="14" /> Analyse en cours (DISM CheckHealth)...
      </div>
      <div v-else-if="health" class="info-grid">
        <div class="info-row"><span>Windows</span><code style="font-size:11px">{{ health.windows_version || 'Inconnu' }}</code></div>
        <div class="info-row"><span>DISM CheckHealth</span>
          <NBadge :variant="healthColor(health.dism_health)" style="font-size:10px">{{ health.dism_health || 'Inconnu' }}</NBadge>
        </div>
        <div class="info-row"><span>Dernier SFC</span><code style="font-size:11px">{{ health.sfc_last_run || 'N/A' }}</code></div>
        <div class="info-row"><span>Résultat SFC</span>
          <NBadge :variant="healthColor(health.sfc_result)" style="font-size:10px">{{ health.sfc_result || 'N/A' }}</NBadge>
        </div>
        <div class="info-row"><span>Redémarrage requis</span>
          <NBadge :variant="health.pending_reboot?'warning':'success'" style="font-size:10px">{{ health.pending_reboot ? 'Oui' : 'Non' }}</NBadge>
        </div>
        <div class="info-row"><span>Taille CBS.log</span><span>{{ (health.cbs_log_size_kb / 1024).toFixed(1) }} MB</span></div>
      </div>
      <div v-if="health?.disk_errors?.length" style="margin-top:10px">
        <p style="font-size:11px;color:var(--error);margin-bottom:6px">
          <AlertTriangle :size="11" style="display:inline;margin-right:4px" />Erreurs disque récentes (7j) :
        </p>
        <div v-for="(e, i) in health.disk_errors" :key="i"
          style="font-size:10px;color:var(--text-secondary);padding:2px 0;border-bottom:1px solid var(--border);font-family:monospace;white-space:nowrap;overflow:hidden;text-overflow:ellipsis">
          {{ e }}
        </div>
      </div>
    </div>

    <!-- Actions de réparation — groupées -->
    <div v-for="group in REPAIR_GROUPS" :key="group.label" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">
        <Wrench :size="12" style="display:inline;margin-right:4px" />{{ group.icon }} {{ group.label }}
      </p>
      <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:6px">
        <button v-for="action in group.actions" :key="action.key"
          @click="runRepair(action.key)"
          :disabled="repairLoading !== null"
          style="display:flex;flex-direction:column;align-items:flex-start;padding:8px 12px;background:var(--bg-secondary);border:1px solid var(--border);border-radius:7px;cursor:pointer;text-align:left;gap:3px;transition:border-color 0.15s"
          :style="{borderColor:repairLoading===action.key?'var(--accent)':'',opacity:repairLoading!==null&&repairLoading!==action.key?'0.6':'1'}"
        >
          <div style="display:flex;align-items:center;gap:6px">
            <NSpinner v-if="repairLoading===action.key" :size="11" />
            <span style="font-size:11px;font-weight:600;color:var(--text-primary)">{{ action.label }}</span>
            <NBadge v-if="!action.fast" variant="warning" style="font-size:8px">Long</NBadge>
          </div>
          <span style="font-size:9px;color:var(--text-secondary)">{{ action.desc }}</span>
        </button>
      </div>
    </div>

    <!-- Résultat -->
    <div v-if="repairLoading" class="diag-section" style="display:flex;align-items:center;gap:10px;color:var(--text-secondary)">
      <NSpinner :size="16" /> Exécution en cours... (peut prendre plusieurs minutes)
    </div>
    <div v-else-if="repairResult" class="diag-section">
      <div style="display:flex;align-items:center;gap:8px;margin-bottom:8px">
        <NBadge :variant="repairResult.success?'success':'danger'">{{ repairResult.success ? 'Succès' : 'Erreur' }}</NBadge>
        <strong style="font-size:12px">{{ repairResult.command }}</strong>
        <span style="font-size:12px;color:var(--text-secondary)">{{ repairResult.duration_secs }}s</span>
      </div>
      <pre style="font-size:10px;color:var(--text-secondary);background:var(--bg-secondary);padding:8px;border-radius:6px;overflow-x:auto;white-space:pre-wrap;max-height:240px;overflow-y:auto">{{ repairResult.output || '(Aucune sortie)' }}</pre>
    </div>
    <div v-else-if="repairError" class="diag-section" style="color:var(--error)">⚠ {{ repairError }}</div>

  </div>
</template>
