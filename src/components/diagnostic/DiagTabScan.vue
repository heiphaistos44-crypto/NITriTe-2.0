<script setup lang="ts">
import {
  CheckCircle, AlertTriangle, RefreshCw, ScanLine, Lock, LockOpen,
  Key, HardDrive, Cpu, MemoryStick, Monitor, Battery,
  FileDown, FileText, FileCode,
} from "lucide-vue-next";
import NProgress from "@/components/ui/NProgress.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";

const props = defineProps<{
  scanning: boolean;
  scanProgress: number;
  scanStep: string;
  scanResult: any;
  scanProblems: string[];
  batteries: any[];
  onRunScan: () => void;
}>();

function kbStr(v: number) { return v >= 1024 ? `${(v / 1024).toFixed(0)} MB` : `${v} KB`; }

async function exportScanTxt() {
  if (!props.scanResult) return;
  const sr = props.scanResult;
  const lines: string[] = [
    "═══════════════════════════════════════════════════════",
    "              RAPPORT SCAN TOTAL — NiTriTe",
    `              Généré le ${new Date().toLocaleString()}`,
    "═══════════════════════════════════════════════════════", "",
    "[ SÉCURITÉ ]",
    `  Activation        : ${sr.windows_activation || 'Inconnu'}`,
    `  Pare-feu          : ${sr.firewall_enabled ? 'Activé' : 'DÉSACTIVÉ ⚠'}`,
    `  Defender          : ${sr.defender_enabled ? 'Actif' : 'INACTIF ⚠'}`,
    `  Antivirus tiers   : ${sr.antivirus_installed || 'Aucun (Defender)'}`,
    `  DISM              : ${sr.dism_status}`,
    `  SFC               : ${sr.sfc_status}`,
    `  Dernier BSOD      : ${sr.last_bsod || 'Aucun'}`,
    `  Connectivité      : ${sr.network_ok ? 'OK' : 'HORS LIGNE ⚠'}`,
    `  Reboot requis     : ${sr.pending_reboot ? 'OUI ⚠' : 'Non'}`,
    `  Dernier KB        : il y a ${sr.last_update_days >= 0 ? sr.last_update_days : '?'} jours`,
    `  Defs Defender     : ${sr.defender_definition_age_days >= 0 ? sr.defender_definition_age_days + ' jours' : 'N/A'}`, "",
    "[ SÉCURITÉ AVANCÉE ]",
    `  TPM               : ${sr.tpm_present ? (sr.tpm_enabled ? 'Présent & Activé' : 'Présent (désactivé)') : 'Absent'} ${sr.tpm_version || ''}`,
    `  Secure Boot       : ${sr.secure_boot ? 'Activé' : 'Désactivé'}`,
    `  UAC               : ${sr.uac_level || 'Inconnu'}`,
    `  RDP               : ${sr.rdp_enabled ? 'Activé ⚠' : 'Désactivé'}`,
    `  SMBv1             : ${sr.smbv1_enabled ? 'Activé ⚠' : 'Désactivé'}`,
    `  WMI Abonnements   : ${sr.wmi_subscriptions ?? 0}`,
    `  Compte Invité     : ${sr.guest_enabled ? 'Activé ⚠' : 'Désactivé'}`,
    `  MAJ en attente    : ${sr.pending_updates_cached >= 0 ? sr.pending_updates_cached : 'N/A'}`,
    `  Point restauration: ${sr.last_restore_point || 'N/A'}`,
    `  Admins locaux     : ${sr.local_admins?.join(', ') || 'N/A'}`, "",
    "[ IDENTITÉ SYSTÈME & BIOS ]",
    `  Fabricant         : ${sr.system_manufacturer || 'N/A'}`,
    `  Modèle            : ${sr.system_model || 'N/A'}`,
    `  N° Série          : ${sr.system_serial || 'N/A'}`,
    `  BIOS Fabricant    : ${sr.bios_manufacturer || 'N/A'}`,
    `  BIOS Version      : ${sr.bios_version || 'N/A'}`,
    `  BIOS Date         : ${sr.bios_date || 'N/A'}`,
    `  Type de licence   : ${sr.license_type || 'N/A'}`, "",
    "[ LICENCES & CHIFFREMENT ]",
    `  Clé Windows       : ${sr.windows_product_key || 'N/A'}`,
    `  Office            : ${sr.office_name || 'N/A'} — ${sr.office_product_key || 'N/A'}`,
  ];
  if (sr.bitlocker_volumes?.length) {
    lines.push("  BitLocker :");
    for (const bv of sr.bitlocker_volumes) {
      lines.push(`    ${bv.drive}: ${bv.protection_status === 'On' || bv.protection_status === '1' ? 'Protégé' : 'Non protégé'}`);
      if (bv.recovery_password) lines.push(`    Clé: ${bv.recovery_password}`);
    }
  }
  lines.push("", "[ COMPOSANTS ]",
    `  CPU               : ${sr.cpu_name} — ${sr.cpu_cores} cœurs / ${sr.cpu_threads || '?'} threads @ ${sr.cpu_frequency_ghz || '?'} GHz`,
    `  RAM               : ${sr.ram_used_gb.toFixed(1)} / ${sr.ram_total_gb.toFixed(0)} GB (${sr.ram_usage_percent.toFixed(0)}%) — ${sr.ram_detail || ''}`,
    `  GPU               : ${sr.gpu_name || 'N/A'} — VRAM: ${sr.gpu_vram_mb >= 1024 ? (sr.gpu_vram_mb/1024).toFixed(0)+'GB' : sr.gpu_vram_mb+'MB'}`,
    `  Carte mère        : ${sr.motherboard || 'N/A'}`,
    `  Écrans            : ${sr.monitors_detail || sr.screen_resolution || 'N/A'}`,
    `  Plan alimentation : ${sr.power_plan || 'N/A'}`,
    `  Logiciels         : ${sr.installed_software_count}`,
    `  Services          : ${sr.services_running} actifs / ${sr.services_stopped} arrêtés`,
  );
  lines.push("", "[ ESPACE DISQUE ]");
  for (const d of sr.disk_usage || []) lines.push(`  ${d.drive}: ${d.used_percent.toFixed(0)}% (${d.free_gb.toFixed(0)} GB libres / ${d.total_gb.toFixed(0)} GB)`);
  if (sr.storage_items?.length) {
    lines.push("", "[ STOCKAGE PHYSIQUE ]");
    for (const s of sr.storage_items) lines.push(`  ${s.model} — ${s.size_gb} GB ${s.media_type} ${s.interface_type} — ${s.health}`);
  }
  if (sr.top_cpu?.length || sr.top_ram?.length) {
    lines.push("", "[ TOP PROCESSUS ]");
    if (sr.top_cpu?.length) { lines.push("  CPU :"); for (const p of sr.top_cpu) lines.push(`    [${p.pid}] ${p.name} — ${p.value}s`); }
    if (sr.top_ram?.length) { lines.push("  RAM :"); for (const p of sr.top_ram) lines.push(`    [${p.pid}] ${p.name} — ${p.value} MB`); }
  }
  if (sr.suspicious_processes?.length) {
    lines.push("", `[ PROCESSUS SUSPECTS (${sr.suspicious_processes.length}) ]`);
    for (const p of sr.suspicious_processes.slice(0, 20)) lines.push(`  [${p.pid}] ${p.name} — ${p.reason} — ${p.path}`);
  }
  if (sr.susp_tasks?.length) {
    lines.push("", `[ TÂCHES SUSPECTES (${sr.susp_tasks_count}) ]`);
    for (const t of sr.susp_tasks) lines.push(`  ${t.name} (${t.path}) → ${t.exec}`);
  }
  if (props.scanProblems?.length) {
    lines.push("", "[ PROBLÈMES DÉTECTÉS ]");
    for (const p of props.scanProblems) lines.push(`  ${p}`);
  }
  lines.push("", "═══════════════════════════════════════════════════════", `  Fin du rapport — NiTriTe — ${new Date().toLocaleString()}`, "═══════════════════════════════════════════════════════");
  const { invoke } = await import("@tauri-apps/api/core");
  const path = await invoke<string>("save_export_file", { filename: "scan_total.txt", content: lines.join("\n") });
  const { useNotificationStore } = await import("@/stores/notifications");
  useNotificationStore().success("Scan exporté", path);
}

async function exportScanJson() {
  if (!props.scanResult) return;
  const { invoke } = await import("@tauri-apps/api/core");
  const path = await invoke<string>("save_export_file", {
    filename: "scan_total.json",
    content: JSON.stringify({ generated: new Date().toISOString(), problems: props.scanProblems, scan: props.scanResult }, null, 2)
  });
  const { useNotificationStore } = await import("@/stores/notifications");
  useNotificationStore().success("Scan exporté (JSON)", path);
}
</script>

<template>
  <!-- En cours -->
  <div v-if="scanning" class="scan-progress" style="display:flex;flex-direction:column;gap:10px">
    <div style="display:flex;align-items:center;gap:8px;font-size:13px;color:var(--text-secondary)">
      <NSpinner :size="16" /><span>{{ scanStep }}</span>
    </div>
    <NProgress :value="scanProgress" showLabel size="lg" />
    <p class="muted" style="font-size:12px">Analyse complète en cours — cela peut prendre 1 à 3 minutes...</p>
  </div>

  <!-- Aucun scan -->
  <div v-else-if="!scanResult">
    <p class="muted" style="margin-bottom:16px;font-size:13px;line-height:1.6">
      Le scan complet analyse : système, CPU/RAM/GPU/disques/carte mère, sécurité,
      licences Windows & Office en clair, clés BitLocker, processus suspects,
      réseau, DISM/SFC, mises à jour, batterie complète, journaux d'erreurs et antivirus.
    </p>
    <NButton variant="primary" @click="onRunScan"><ScanLine :size="14" /> Lancer le Scan Complet</NButton>
  </div>

  <!-- Résultats -->
  <div v-else style="display:flex;flex-direction:column;gap:14px">

    <!-- ===== BILAN ===== -->
    <div class="diag-section" :style="{borderLeft: `3px solid ${scanProblems.length ? 'var(--warning)' : 'var(--success)'}`}">
      <p class="diag-section-label" style="margin:0 0 8px 0">
        {{ scanProblems.length ? `⚠ ${scanProblems.length} problème(s) détecté(s)` : '✓ Aucun problème critique' }}
      </p>
      <div v-if="!scanProblems.length" style="color:var(--success);font-size:13px">Système en bonne santé</div>
      <div v-for="(p, i) in scanProblems" :key="i"
        style="display:flex;align-items:center;gap:8px;padding:5px 0;border-bottom:1px solid var(--border);font-size:13px">
        <AlertTriangle :size="13" class="ic-warn" />{{ p }}
      </div>
    </div>

    <!-- ===== SYSTÈME GÉNÉRAL ===== -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Résumé Système</p>
      <div class="info-grid">
        <div class="info-row"><span>Windows</span><span>{{ scanResult.windows_version }}</span></div>
        <div class="info-row"><span>Activation</span>
          <NBadge :variant="scanResult.windows_activation === 'Activé' || scanResult.windows_activation === 'Licencié' ? 'success' : 'danger'">
            {{ scanResult.windows_activation || "Inconnu" }}
          </NBadge>
        </div>
        <div class="info-row"><span>Uptime</span>
          <span>{{ scanResult.uptime_hours >= 24 ? `${(scanResult.uptime_hours/24).toFixed(1)} j` : `${scanResult.uptime_hours.toFixed(1)} h` }}</span>
        </div>
        <div class="info-row"><span>Redémarrage requis</span>
          <NBadge :variant="scanResult.pending_reboot ? 'warning' : 'success'">{{ scanResult.pending_reboot ? "Oui" : "Non" }}</NBadge>
        </div>
        <div class="info-row"><span>Démarrage auto</span><span>{{ scanResult.startup_count }} programmes</span></div>
        <div class="info-row"><span>Fichiers %TEMP%</span>
          <NBadge :variant="scanResult.temp_folder_size_mb > 2048 ? 'danger' : scanResult.temp_folder_size_mb > 512 ? 'warning' : 'success'">
            {{ scanResult.temp_folder_size_mb >= 1024 ? (scanResult.temp_folder_size_mb/1024).toFixed(1)+' GB' : scanResult.temp_folder_size_mb.toFixed(0)+' MB' }}
          </NBadge>
        </div>
        <div class="info-row"><span>Logiciels installés</span><span>{{ scanResult.installed_software_count }}</span></div>
        <div class="info-row"><span>Services actifs / arrêtés</span>
          <span><span style="color:var(--success)">{{ scanResult.services_running }}</span> / <span class="muted">{{ scanResult.services_stopped }}</span></span>
        </div>
        <div class="info-row"><span>Mém. virtuelle</span>
          <span>{{ scanResult.virtual_memory_available_mb > 0 ? (scanResult.virtual_memory_available_mb/1024).toFixed(1)+'GB libres / '+(scanResult.virtual_memory_total_mb/1024).toFixed(1)+'GB' : 'N/A' }}</span>
        </div>
        <div class="info-row" v-if="scanResult.plan_alimentation || scanResult.power_plan">
          <span>Plan d'alimentation</span><span>{{ scanResult.power_plan }}</span>
        </div>
      </div>
    </div>

    <!-- ===== COMPOSANTS MATÉRIELS ===== -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Composants matériels</p>
      <div class="info-grid">
        <!-- CPU -->
        <div class="info-row">
          <span style="display:flex;align-items:center;gap:4px"><Cpu :size="12" /> Processeur</span>
          <span>{{ scanResult.cpu_name }}</span>
        </div>
        <div class="info-row"><span>Cœurs / Threads</span>
          <span>{{ scanResult.cpu_cores }} cœurs / {{ scanResult.cpu_threads > 0 ? scanResult.cpu_threads : '—' }} threads</span>
        </div>
        <div class="info-row"><span>Fréquence max</span>
          <span>{{ scanResult.cpu_frequency_ghz > 0 ? scanResult.cpu_frequency_ghz + ' GHz' : '—' }}</span>
        </div>
        <div class="info-row"><span>Utilisation CPU</span>
          <NBadge :variant="scanResult.cpu_usage_percent > 80 ? 'danger' : scanResult.cpu_usage_percent > 50 ? 'warning' : 'success'">
            {{ scanResult.cpu_usage_percent.toFixed(1) }}%
          </NBadge>
        </div>
        <div v-if="scanResult.cpu_temperature && scanResult.cpu_temperature !== 'N/A'" class="info-row">
          <span>Température CPU</span>
          <NBadge :variant="parseInt(scanResult.cpu_temperature) > 80 ? 'danger' : parseInt(scanResult.cpu_temperature) > 65 ? 'warning' : 'success'">
            {{ scanResult.cpu_temperature }}
          </NBadge>
        </div>
        <!-- RAM -->
        <div class="info-row">
          <span style="display:flex;align-items:center;gap:4px"><MemoryStick :size="12" /> RAM utilisée</span>
          <NBadge :variant="scanResult.ram_usage_percent > 85 ? 'danger' : scanResult.ram_usage_percent > 65 ? 'warning' : 'success'">
            {{ scanResult.ram_used_gb.toFixed(1) }} / {{ scanResult.ram_total_gb.toFixed(0) }} GB ({{ scanResult.ram_usage_percent.toFixed(0) }}%)
          </NBadge>
        </div>
        <div v-if="scanResult.ram_detail" class="info-row"><span>Configuration RAM</span><span>{{ scanResult.ram_detail }}</span></div>
        <!-- GPU -->
        <div v-if="scanResult.gpu_name" class="info-row">
          <span style="display:flex;align-items:center;gap:4px"><Monitor :size="12" /> GPU</span>
          <span>{{ scanResult.gpu_name }}</span>
        </div>
        <div v-if="scanResult.gpu_vram_mb > 0" class="info-row">
          <span>VRAM GPU</span>
          <span>{{ scanResult.gpu_vram_mb >= 1024 ? (scanResult.gpu_vram_mb/1024).toFixed(1)+' GB' : scanResult.gpu_vram_mb+' MB' }}</span>
        </div>
        <!-- Carte mère -->
        <div v-if="scanResult.motherboard" class="info-row"><span>Carte mère</span><span>{{ scanResult.motherboard }}</span></div>
        <!-- Écrans -->
        <div v-if="scanResult.monitors_detail" class="info-row info-full">
          <span style="display:flex;align-items:center;gap:4px"><Monitor :size="12" /> Écran(s)</span>
          <span style="font-size:11px">{{ scanResult.monitors_detail }}</span>
        </div>
        <div v-else-if="scanResult.screen_resolution" class="info-row"><span>Résolution</span><span>{{ scanResult.screen_resolution }}</span></div>
        <!-- Réseau -->
        <div v-if="scanResult.network_adapters_summary" class="info-row info-full">
          <span>Interfaces réseau actives</span><span style="font-size:11px">{{ scanResult.network_adapters_summary }}</span>
        </div>
      </div>
    </div>

    <!-- ===== STOCKAGE PHYSIQUE ===== -->
    <div v-if="scanResult.storage_items?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Stockage physique ({{ scanResult.storage_items.length }} disque(s))</p>
      <div v-for="(s, i) in scanResult.storage_items" :key="i"
        style="display:flex;align-items:center;gap:10px;padding:6px 0;border-bottom:1px solid var(--border);font-size:12px;flex-wrap:wrap">
        <HardDrive :size="13" style="color:var(--accent);flex-shrink:0" />
        <span style="font-weight:500;flex:1;min-width:160px">{{ s.model || "Disque inconnu" }}</span>
        <NBadge :variant="s.media_type === 'SSD' || s.media_type === 'NVMe' ? 'info' : 'default'">{{ s.media_type }}</NBadge>
        <NBadge variant="neutral">{{ s.interface_type }}</NBadge>
        <span class="muted">{{ s.size_gb > 0 ? s.size_gb + ' GB' : '—' }}</span>
        <NBadge :variant="s.health === 'Healthy' || s.health === 'Sain' ? 'success' : s.health ? 'warning' : 'neutral'">
          {{ s.health || "—" }}
        </NBadge>
      </div>
    </div>

    <!-- ===== ESPACE DISQUE ===== -->
    <div v-if="scanResult.disk_usage?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Espace disque (volumes)</p>
      <div v-for="d in scanResult.disk_usage" :key="d.drive"
        style="display:flex;align-items:center;gap:12px;margin-bottom:8px;font-size:13px">
        <code style="min-width:40px">{{ d.drive }}</code>
        <NProgress :value="d.used_percent"
          :variant="d.used_percent > 90 ? 'danger' : d.used_percent > 80 ? 'warning' : 'default'"
          size="sm" showLabel style="flex:1" />
        <span class="muted" style="min-width:130px;text-align:right;font-size:11px;font-family:monospace">
          {{ d.free_gb.toFixed(0) }} GB libres / {{ d.total_gb.toFixed(0) }} GB
        </span>
      </div>
    </div>

    <!-- ===== LICENCES & CHIFFREMENT ===== -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Licences & Chiffrement</p>
      <div class="info-grid">
        <!-- Clé Windows -->
        <div class="info-row info-full">
          <span style="display:flex;align-items:center;gap:4px"><Key :size="12" /> Clé Windows</span>
          <code v-if="scanResult.windows_product_key" style="color:var(--accent);font-size:12px">
            {{ scanResult.windows_product_key }}
          </code>
          <span v-else class="muted" style="font-size:11px">Non disponible (BIOS/UEFI sans clé embarquée)</span>
        </div>
        <!-- Clé Office -->
        <div v-if="scanResult.office_name || scanResult.office_product_key" class="info-row info-full">
          <span style="display:flex;align-items:center;gap:4px"><Key :size="12" /> {{ scanResult.office_name || "Office" }}</span>
          <code v-if="scanResult.office_product_key" style="color:var(--success);font-size:12px">
            {{ scanResult.office_product_key }}
          </code>
          <span v-else class="muted" style="font-size:11px">Clé non disponible dans le registre</span>
        </div>
      </div>

      <!-- BitLocker -->
      <div v-if="scanResult.bitlocker_volumes?.length" style="margin-top:10px">
        <p style="font-size:12px;font-weight:600;margin-bottom:6px;color:var(--text-muted)">BitLocker</p>
        <div v-for="(bv, i) in scanResult.bitlocker_volumes" :key="i"
          style="border:1px solid var(--border);border-radius:6px;padding:10px;margin-bottom:8px">
          <div style="display:flex;align-items:center;gap:8px;margin-bottom:6px">
            <component :is="bv.protection_status === 'On' || bv.protection_status === '1' ? Lock : LockOpen"
              :size="14" :style="{ color: bv.protection_status === 'On' || bv.protection_status === '1' ? 'var(--success)' : 'var(--warning)' }" />
            <strong>{{ bv.drive }}</strong>
            <NBadge :variant="bv.protection_status === 'On' || bv.protection_status === '1' ? 'success' : 'warning'">
              {{ bv.protection_status === 'On' || bv.protection_status === '1' ? 'Protégé' : 'Non protégé' }}
            </NBadge>
            <span v-if="bv.encryption_percent < 100 && bv.encryption_percent > 0" class="muted" style="font-size:11px">
              {{ bv.encryption_percent }}% chiffré
            </span>
            <div style="display:flex;gap:4px;flex-wrap:wrap">
              <NBadge v-for="(p, pi) in bv.protectors" :key="pi" variant="neutral" style="font-size:10px">{{ p }}</NBadge>
            </div>
          </div>
          <div v-if="bv.recovery_password" class="info-row" style="background:var(--bg-secondary);padding:6px 8px;border-radius:4px">
            <span style="font-size:11px;color:var(--text-muted)">Clé de récupération</span>
            <code style="font-size:11px;color:var(--warning);word-break:break-all">{{ bv.recovery_password }}</code>
          </div>
          <div v-else class="muted" style="font-size:11px">Aucune clé de récupération disponible</div>
        </div>
      </div>
      <div v-else class="muted" style="font-size:12px;margin-top:8px">
        BitLocker non configuré (ou non détectable sans droits admin)
      </div>
    </div>

    <!-- ===== BATTERIE ===== -->
    <div v-if="batteries?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Batterie</p>
      <div v-for="(b, i) in batteries" :key="i">
        <div style="display:flex;align-items:center;gap:8px;margin-bottom:8px">
          <Battery :size="14" style="color:var(--accent)" />
          <strong>{{ b.name }}</strong>
          <NBadge :variant="b.battery_health_percent > 80 ? 'success' : b.battery_health_percent > 50 ? 'warning' : 'danger'">
            {{ b.battery_health_percent.toFixed(0) }}% santé
          </NBadge>
          <NBadge :variant="b.status === 'Charging' || b.status === 'En charge' ? 'success' : 'info'">
            {{ b.status || "—" }}
          </NBadge>
        </div>
        <div class="info-grid">
          <div class="info-row"><span>Charge actuelle</span>
            <NBadge :variant="b.estimated_charge_remaining < 20 ? 'danger' : b.estimated_charge_remaining < 40 ? 'warning' : 'success'">
              {{ b.estimated_charge_remaining }}%
            </NBadge>
          </div>
          <div class="info-row"><span>Autonomie estimée</span><span>{{ b.estimated_run_time || "—" }}</span></div>
          <div class="info-row"><span>Cycles de charge</span>
            <NBadge :variant="b.cycle_count > 500 ? 'danger' : b.cycle_count > 300 ? 'warning' : 'success'">
              {{ b.cycle_count > 0 ? b.cycle_count : 'N/A' }}
            </NBadge>
          </div>
          <div class="info-row"><span>Capacité de conception</span>
            <span>{{ b.design_capacity > 0 ? b.design_capacity + ' mWh' : 'N/A' }}</span>
          </div>
          <div class="info-row"><span>Capacité actuelle</span>
            <span>{{ b.full_charge_capacity > 0 ? b.full_charge_capacity + ' mWh' : 'N/A' }}</span>
          </div>
          <div v-if="b.design_capacity > 0 && b.full_charge_capacity > 0" class="info-row">
            <span>Usure capacité</span>
            <code style="color:var(--warning)">
              -{{ ((1 - b.full_charge_capacity / b.design_capacity) * 100).toFixed(1) }}%
            </code>
          </div>
          <div class="info-row"><span>Chimie</span><span>{{ b.chemistry || "N/A" }}</span></div>
        </div>
      </div>
    </div>

    <!-- ===== SÉCURITÉ ===== -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Sécurité</p>
      <div v-for="[ok, label, val] in [
        [scanResult.firewall_enabled, 'Pare-feu Windows', scanResult.firewall_enabled ? 'Activé' : 'DÉSACTIVÉ ⚠'],
        [scanResult.defender_enabled, 'Defender (temps réel)', scanResult.defender_enabled ? 'Actif' : 'INACTIF ⚠'],
        [scanResult.network_ok, 'Connectivité Internet (8.8.8.8)', scanResult.network_ok ? 'OK' : 'Hors ligne'],
        [scanResult.suspicious_processes?.length === 0, 'Processus suspects', scanResult.suspicious_processes?.length === 0 ? 'Aucun' : `${scanResult.suspicious_processes?.length} détecté(s)`],
      ]" :key="label" style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="ok ? CheckCircle : AlertTriangle" :size="14" :class="ok ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">{{ label }}</span>
        <span class="mono">{{ val }}</span>
      </div>
      <div v-if="scanResult.open_ports?.length" style="display:flex;align-items:center;gap:8px;padding:6px 0;font-size:13px">
        <AlertTriangle :size="14" class="ic-warn" />
        <span style="flex:1">Ports en écoute globale</span>
        <code class="mono" style="font-size:11px">{{ scanResult.open_ports.join(", ") }}</code>
      </div>
      <div style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="!scanResult.last_bsod || scanResult.last_bsod.includes('Aucun') ? CheckCircle : AlertTriangle"
          :size="14" :class="!scanResult.last_bsod || scanResult.last_bsod.includes('Aucun') ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">Dernier BSOD</span>
        <span class="mono">{{ scanResult.last_bsod || "Aucun" }}</span>
      </div>
    </div>

    <!-- ===== ANTIVIRUS ===== -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Antivirus & Protection</p>
      <div class="info-grid">
        <div class="info-row"><span>Antivirus tiers</span><span>{{ scanResult.antivirus_installed || "Aucun (Defender)" }}</span></div>
        <div class="info-row"><span>Définitions Defender</span>
          <NBadge :variant="scanResult.defender_definition_age_days < 0 ? 'neutral' : scanResult.defender_definition_age_days <= 3 ? 'success' : scanResult.defender_definition_age_days <= 7 ? 'warning' : 'danger'">
            {{ scanResult.defender_definition_age_days >= 0 ? scanResult.defender_definition_age_days + ' j' : 'N/A' }}
          </NBadge>
        </div>
        <div class="info-row"><span>Dernier KB Windows</span>
          <NBadge :variant="scanResult.last_update_days < 0 ? 'neutral' : scanResult.last_update_days <= 30 ? 'success' : scanResult.last_update_days <= 60 ? 'warning' : 'danger'">
            {{ scanResult.last_update_days >= 0 ? 'il y a ' + scanResult.last_update_days + ' j' : 'N/A' }}
          </NBadge>
        </div>
      </div>
    </div>

    <!-- ===== INTÉGRITÉ WINDOWS ===== -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Intégrité Windows</p>
      <div v-for="[label, status, ok] in [
        ['DISM (Health Store)', scanResult.dism_status, scanResult.dism_status?.toLowerCase().includes('sain')],
        ['SFC (System File Checker)', scanResult.sfc_status, scanResult.sfc_status?.toLowerCase().includes('intèg') || scanResult.sfc_status?.toLowerCase().includes('integ')],
      ]" :key="label" style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="ok ? CheckCircle : AlertTriangle" :size="14" :class="ok ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">{{ label }}</span>
        <span class="mono">{{ status }}</span>
      </div>
      <div style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="scanResult.winget_upgradable?.length === 0 ? CheckCircle : AlertTriangle"
          :size="14" :class="scanResult.winget_upgradable?.length === 0 ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">WinGet — mises à jour</span>
        <span class="mono">{{ scanResult.winget_upgradable?.length || 0 }} disponible(s)</span>
      </div>
    </div>

    <!-- ===== PROCESSUS SUSPECTS ===== -->
    <div v-if="scanResult.suspicious_processes?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Processus hors chemins sécurisés ({{ scanResult.suspicious_processes.length }})</p>
      <div v-for="p in scanResult.suspicious_processes.slice(0, 20)" :key="p.pid" class="list-row">
        <code class="list-name">{{ p.name }}</code>
        <NBadge variant="warning" style="flex-shrink:0">{{ p.reason }}</NBadge>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ p.path }}</div>
      </div>
    </div>

    <!-- ===== SERVICES TIERS ===== -->
    <div v-if="scanResult.suspicious_services?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Services tiers actifs ({{ scanResult.suspicious_services.length }})</p>
      <div v-for="(s, i) in scanResult.suspicious_services.slice(0, 15)" :key="i" class="list-row">
        <code class="list-name" style="min-width:130px">{{ s.name }}</code>
        <div class="muted" style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ s.display_name }}</div>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ s.path }}</div>
      </div>
    </div>

    <!-- ===== AUTORUNS ===== -->
    <div v-if="scanResult.autorun_entries?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Entrées Autorun tiers ({{ scanResult.autorun_entries.length }})</p>
      <div v-for="(a, i) in scanResult.autorun_entries.slice(0, 20)" :key="i" class="list-row">
        <code class="list-name" style="min-width:140px">{{ a.name }}</code>
        <NBadge variant="neutral" style="flex-shrink:0;font-size:10px">{{ a.location }}</NBadge>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ a.path }}</div>
      </div>
    </div>

    <!-- ===== ÉVÉNEMENTS D'ERREUR ===== -->
    <div v-if="scanResult.recent_errors?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">
        Événements d'erreur récents (48h) — {{ scanResult.recent_errors.length }}
      </p>
      <div v-for="(e, i) in scanResult.recent_errors.slice(0, 25)" :key="i" class="list-row">
        <code class="muted" style="min-width:110px;font-size:10px">{{ e.time }}</code>
        <NBadge :variant="e.level?.toLowerCase().includes('critical') || e.level?.toLowerCase().includes('critique') ? 'danger' : 'warning'"
          style="flex-shrink:0;font-size:10px">{{ e.level }}</NBadge>
        <div style="flex:1;min-width:0">
          <span style="font-size:12px;font-weight:500">{{ e.source }}</span>
          <div class="muted" style="font-size:11px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis">{{ e.message }}</div>
        </div>
      </div>
    </div>

    <!-- ===== SÉCURITÉ AVANCÉE ===== -->
    <div v-if="scanResult.tpm_present !== undefined" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Sécurité Avancée</p>
      <div class="info-grid">
        <div class="info-row"><span>TPM</span>
          <NBadge :variant="scanResult.tpm_present ? 'success' : 'warning'">
            {{ scanResult.tpm_present ? (scanResult.tpm_enabled ? 'Présent & Activé' : 'Présent (désactivé)') : 'Absent' }}
          </NBadge>
        </div>
        <div v-if="scanResult.tpm_present && scanResult.tpm_version" class="info-row">
          <span>Version TPM</span><span>{{ scanResult.tpm_version }}</span>
        </div>
        <div class="info-row"><span>Secure Boot</span>
          <NBadge :variant="scanResult.secure_boot ? 'success' : 'warning'">{{ scanResult.secure_boot ? 'Activé' : 'Désactivé' }}</NBadge>
        </div>
        <div class="info-row"><span>Niveau UAC</span><span>{{ scanResult.uac_level || 'Inconnu' }}</span></div>
        <div class="info-row"><span>Bureau à distance (RDP)</span>
          <NBadge :variant="scanResult.rdp_enabled ? 'warning' : 'success'">{{ scanResult.rdp_enabled ? 'Activé' : 'Désactivé' }}</NBadge>
        </div>
        <div class="info-row"><span>SMBv1 (obsolète)</span>
          <NBadge :variant="scanResult.smbv1_enabled ? 'danger' : 'success'">{{ scanResult.smbv1_enabled ? 'Activé ⚠' : 'Désactivé' }}</NBadge>
        </div>
        <div class="info-row"><span>Abonnements WMI (indicateur malware)</span>
          <NBadge :variant="scanResult.wmi_subscriptions > 0 ? 'danger' : 'success'">{{ scanResult.wmi_subscriptions ?? 0 }}</NBadge>
        </div>
        <div class="info-row"><span>Compte Invité</span>
          <NBadge :variant="scanResult.guest_enabled ? 'warning' : 'success'">{{ scanResult.guest_enabled ? 'Activé ⚠' : 'Désactivé' }}</NBadge>
        </div>
        <div v-if="scanResult.pending_updates_cached >= 0" class="info-row">
          <span>MAJ Windows en attente (cache)</span>
          <NBadge :variant="scanResult.pending_updates_cached > 10 ? 'danger' : scanResult.pending_updates_cached > 0 ? 'warning' : 'success'">
            {{ scanResult.pending_updates_cached }}
          </NBadge>
        </div>
        <div v-if="scanResult.last_restore_point" class="info-row">
          <span>Dernier point de restauration</span>
          <NBadge :variant="scanResult.last_restore_point.includes('Aucun') ? 'warning' : 'success'">
            {{ scanResult.last_restore_point }}
          </NBadge>
        </div>
      </div>
      <div v-if="scanResult.local_admins?.length" style="margin-top:8px">
        <p style="font-size:11px;color:var(--text-muted);margin:0 0 4px 0">Administrateurs locaux ({{ scanResult.local_admins.length }})</p>
        <div style="display:flex;gap:6px;flex-wrap:wrap">
          <code v-for="(a, i) in scanResult.local_admins" :key="i"
            style="font-size:11px;background:var(--bg-secondary);padding:2px 6px;border-radius:4px">{{ a }}</code>
        </div>
      </div>
    </div>

    <!-- ===== IDENTITÉ SYSTÈME & BIOS ===== -->
    <div v-if="scanResult.system_manufacturer || scanResult.bios_manufacturer" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Identité Système & BIOS</p>
      <div class="info-grid">
        <div v-if="scanResult.system_manufacturer" class="info-row"><span>Fabricant</span><span>{{ scanResult.system_manufacturer }}</span></div>
        <div v-if="scanResult.system_model" class="info-row"><span>Modèle</span><span>{{ scanResult.system_model }}</span></div>
        <div v-if="scanResult.system_serial && scanResult.system_serial !== 'N/A'" class="info-row">
          <span>N° Série carte mère</span><code style="font-size:11px">{{ scanResult.system_serial }}</code>
        </div>
        <div v-if="scanResult.bios_manufacturer" class="info-row"><span>BIOS Fabricant</span><span>{{ scanResult.bios_manufacturer }}</span></div>
        <div v-if="scanResult.bios_version" class="info-row"><span>BIOS Version</span><code style="font-size:11px">{{ scanResult.bios_version }}</code></div>
        <div v-if="scanResult.bios_date" class="info-row"><span>BIOS Date</span><span>{{ scanResult.bios_date }}</span></div>
        <div v-if="scanResult.license_type" class="info-row"><span>Type de licence Windows</span>
          <NBadge :variant="scanResult.license_type === 'OEM' ? 'info' : scanResult.license_type === 'Retail' ? 'success' : 'neutral'">
            {{ scanResult.license_type }}
          </NBadge>
        </div>
      </div>
    </div>

    <!-- ===== TOP PROCESSUS ===== -->
    <div v-if="scanResult.top_cpu?.length || scanResult.top_ram?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Top 5 Processus (snapshot)</p>
      <div style="display:flex;gap:16px;flex-wrap:wrap">
        <div v-if="scanResult.top_cpu?.length" style="flex:1;min-width:200px">
          <p style="font-size:11px;font-weight:600;color:var(--text-muted);margin:0 0 6px 0"><Cpu :size="11" style="display:inline;margin-right:4px" />CPU — temps cumulé (sec.)</p>
          <div v-for="(p, i) in scanResult.top_cpu" :key="i"
            style="display:flex;align-items:center;gap:8px;padding:4px 0;border-bottom:1px solid var(--border);font-size:12px">
            <code class="muted" style="min-width:34px;font-size:10px">{{ p.pid }}</code>
            <span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ p.name }}</span>
            <code style="color:var(--accent);font-size:11px;flex-shrink:0">{{ p.value }}s</code>
          </div>
        </div>
        <div v-if="scanResult.top_ram?.length" style="flex:1;min-width:200px">
          <p style="font-size:11px;font-weight:600;color:var(--text-muted);margin:0 0 6px 0"><MemoryStick :size="11" style="display:inline;margin-right:4px" />RAM (MB)</p>
          <div v-for="(p, i) in scanResult.top_ram" :key="i"
            style="display:flex;align-items:center;gap:8px;padding:4px 0;border-bottom:1px solid var(--border);font-size:12px">
            <code class="muted" style="min-width:34px;font-size:10px">{{ p.pid }}</code>
            <span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ p.name }}</span>
            <code style="color:var(--success);font-size:11px;flex-shrink:0">{{ p.value }} MB</code>
          </div>
        </div>
      </div>
    </div>

    <!-- ===== TÂCHES PLANIFIÉES SUSPECTES ===== -->
    <div v-if="scanResult.susp_tasks?.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Tâches planifiées suspectes ({{ scanResult.susp_tasks_count }})</p>
      <div v-for="(t, i) in scanResult.susp_tasks" :key="i" class="list-row">
        <code class="list-name" style="min-width:140px">{{ t.name }}</code>
        <NBadge variant="warning" style="flex-shrink:0;font-size:10px">{{ t.path }}</NBadge>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ t.exec }}</div>
      </div>
    </div>

    <!-- Export scan -->
    <div style="display:flex;gap:8px;flex-wrap:wrap;align-items:center;padding-top:4px">
      <NButton variant="ghost" size="sm" @click="onRunScan"><RefreshCw :size="12" /> Relancer</NButton>
      <NButton variant="ghost" size="sm" @click="exportScanTxt"><FileText :size="12" /> Export .txt</NButton>
      <NButton variant="ghost" size="sm" @click="exportScanJson"><FileDown :size="12" /> Export .json</NButton>
    </div>
  </div>
</template>
