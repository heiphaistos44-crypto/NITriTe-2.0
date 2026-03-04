<script setup lang="ts">
import { CheckCircle, AlertTriangle, RefreshCw, ScanLine } from "lucide-vue-next";
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
      Le scan complet analyse : système, sécurité, processus suspects, disques, réseau,
      licence, intégrité Windows (DISM/SFC), mises à jour, fichiers temporaires, Autorun,
      journaux d'erreurs, batterie et antivirus.
    </p>
    <NButton variant="primary" @click="onRunScan"><ScanLine :size="14" /> Lancer le Scan Complet</NButton>
  </div>

  <!-- Résultats -->
  <div v-else style="display:flex;flex-direction:column;gap:14px">
    <!-- Bilan -->
    <div class="diag-section" :style="{borderLeft: `3px solid ${scanProblems.length ? 'var(--warning)' : 'var(--success)'}`}">
      <p class="diag-section-label" style="margin:0 0 8px 0">
        {{ scanProblems.length ? `⚠ ${scanProblems.length} problème(s) détecté(s)` : '✓ Aucun problème critique' }}
      </p>
      <div v-if="!scanProblems.length" style="color:var(--success);font-size:13px">Système en bonne santé</div>
      <div v-for="(p, i) in scanProblems" :key="i" style="display:flex;align-items:center;gap:8px;padding:5px 0;border-bottom:1px solid var(--border);font-size:13px">
        <AlertTriangle :size="13" class="ic-warn" />{{ p }}
      </div>
    </div>

    <!-- Résumé -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Résumé Système</p>
      <div class="info-grid">
        <div class="info-row"><span>Windows</span><span>{{ scanResult.windows_version }}</span></div>
        <div class="info-row"><span>Activation</span>
          <NBadge :variant="scanResult.windows_activation === 'Activé' || scanResult.windows_activation === 'Licencié' ? 'success' : 'danger'">
            {{ scanResult.windows_activation || "Inconnu" }}
          </NBadge>
        </div>
        <div class="info-row"><span>Uptime</span><span>{{ scanResult.uptime_hours >= 24 ? `${(scanResult.uptime_hours/24).toFixed(1)} j` : `${scanResult.uptime_hours.toFixed(1)} h` }}</span></div>
        <div class="info-row"><span>Redémarrage requis</span>
          <NBadge :variant="scanResult.pending_reboot ? 'warning' : 'success'">{{ scanResult.pending_reboot ? "Oui" : "Non" }}</NBadge>
        </div>
        <div class="info-row"><span>CPU</span><span>{{ scanResult.cpu_name }} — {{ scanResult.cpu_usage_percent.toFixed(1) }}%</span></div>
        <div class="info-row"><span>Cœurs CPU</span><span>{{ scanResult.cpu_cores }}</span></div>
        <div class="info-row"><span>RAM utilisée</span><span>{{ scanResult.ram_used_gb.toFixed(1) }} / {{ scanResult.ram_total_gb.toFixed(0) }} GB ({{ scanResult.ram_usage_percent.toFixed(0) }}%)</span></div>
        <div class="info-row"><span>Mém. virtuelle</span><span>{{ scanResult.virtual_memory_available_mb > 0 ? (scanResult.virtual_memory_available_mb/1024).toFixed(1)+'GB libres / '+(scanResult.virtual_memory_total_mb/1024).toFixed(1)+'GB' : 'N/A' }}</span></div>
        <div class="info-row"><span>Programmes démarrage</span><span>{{ scanResult.startup_count }}</span></div>
        <div class="info-row"><span>Fichiers %TEMP%</span>
          <NBadge :variant="scanResult.temp_folder_size_mb > 2048 ? 'danger' : scanResult.temp_folder_size_mb > 512 ? 'warning' : 'success'">
            {{ scanResult.temp_folder_size_mb >= 1024 ? (scanResult.temp_folder_size_mb/1024).toFixed(1)+' GB' : scanResult.temp_folder_size_mb.toFixed(0)+' MB' }}
          </NBadge>
        </div>
      </div>
    </div>

    <!-- Sécurité -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Sécurité</p>
      <div v-for="[icon, ok, label, val] in [
        [null, scanResult.firewall_enabled, 'Pare-feu Windows', scanResult.firewall_enabled ? 'Activé' : 'DÉSACTIVÉ'],
        [null, scanResult.defender_enabled, 'Defender (temps réel)', scanResult.defender_enabled ? 'Actif' : 'INACTIF'],
        [null, scanResult.network_ok, 'Connectivité Internet (8.8.8.8)', scanResult.network_ok ? 'OK' : 'Hors ligne'],
        [null, scanResult.suspicious_processes.length === 0, 'Processus suspects', scanResult.suspicious_processes.length === 0 ? 'Aucun' : `${scanResult.suspicious_processes.length} détecté(s)`],
      ]" :key="label" style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="ok ? CheckCircle : AlertTriangle" :size="14" :class="ok ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">{{ label }}</span>
        <span class="mono">{{ val }}</span>
      </div>
      <div v-if="scanResult.open_ports.length" style="display:flex;align-items:center;gap:8px;padding:6px 0;font-size:13px">
        <AlertTriangle :size="14" class="ic-warn" />
        <span style="flex:1">Ports en écoute globale</span>
        <code class="mono" style="font-size:11px">{{ scanResult.open_ports.join(", ") }}</code>
      </div>
      <div style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="!scanResult.last_bsod || scanResult.last_bsod.includes('Aucun') ? CheckCircle : AlertTriangle" :size="14"
          :class="!scanResult.last_bsod || scanResult.last_bsod.includes('Aucun') ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">Dernier BSOD</span>
        <span class="mono">{{ scanResult.last_bsod || "Aucun" }}</span>
      </div>
    </div>

    <!-- Protection avancée -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Antivirus & Protection</p>
      <div class="info-grid">
        <div class="info-row"><span>Antivirus tiers</span><span>{{ scanResult.antivirus_installed || "Aucun" }}</span></div>
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

    <!-- Intégrité Windows -->
    <div class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Intégrité Windows</p>
      <div v-for="[label, status, ok] in [
        ['DISM (Health Store)', scanResult.dism_status, scanResult.dism_status.toLowerCase().includes('sain')],
        ['SFC (System File Checker)', scanResult.sfc_status, scanResult.sfc_status.toLowerCase().includes('intèg') || scanResult.sfc_status.toLowerCase().includes('integ')],
      ]" :key="label" style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="ok ? CheckCircle : AlertTriangle" :size="14" :class="ok ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">{{ label }}</span>
        <span class="mono">{{ status }}</span>
      </div>
      <div style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
        <component :is="scanResult.winget_upgradable.length === 0 ? CheckCircle : AlertTriangle" :size="14" :class="scanResult.winget_upgradable.length === 0 ? 'ic-ok' : 'ic-warn'" />
        <span style="flex:1">WinGet — mises à jour</span>
        <span class="mono">{{ scanResult.winget_upgradable.length }} disponible(s)</span>
      </div>
    </div>

    <!-- Espace disque -->
    <div v-if="scanResult.disk_usage.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Espace Disque</p>
      <div v-for="d in scanResult.disk_usage" :key="d.drive" style="display:flex;align-items:center;gap:12px;margin-bottom:8px;font-size:13px">
        <code style="min-width:40px">{{ d.drive }}</code>
        <NProgress :value="d.used_percent" :variant="d.used_percent > 90 ? 'danger' : d.used_percent > 80 ? 'warning' : 'default'" size="sm" showLabel style="flex:1" />
        <span class="muted" style="min-width:130px;text-align:right;font-size:11px;font-family:monospace">{{ d.free_gb.toFixed(0) }} GB libres / {{ d.total_gb.toFixed(0) }} GB</span>
      </div>
    </div>

    <!-- Batterie -->
    <div v-if="batteries.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Batterie</p>
      <div v-for="(b, i) in batteries" :key="i" class="info-grid">
        <div class="info-row"><span>{{ b.name }}</span>
          <NBadge :variant="b.battery_health_percent > 80 ? 'success' : b.battery_health_percent > 50 ? 'warning' : 'danger'">{{ b.battery_health_percent.toFixed(0) }}% santé</NBadge>
        </div>
        <div class="info-row"><span>Charge / Autonomie</span><span>{{ b.estimated_charge_remaining }}% — {{ b.estimated_run_time }}</span></div>
        <div class="info-row"><span>Cycles</span><span>{{ b.cycle_count > 0 ? b.cycle_count : 'N/A' }}</span></div>
        <div class="info-row"><span>Capacité actuelle</span><span>{{ b.full_charge_capacity > 0 ? b.full_charge_capacity + ' mWh' : 'N/A' }}</span></div>
      </div>
    </div>

    <!-- Processus suspects -->
    <div v-if="scanResult.suspicious_processes.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Processus hors chemins sécurisés ({{ scanResult.suspicious_processes.length }})</p>
      <div v-for="p in scanResult.suspicious_processes.slice(0, 15)" :key="p.pid" class="list-row">
        <code class="list-name">{{ p.name }}</code>
        <NBadge variant="warning" style="flex-shrink:0">{{ p.reason }}</NBadge>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ p.path }}</div>
      </div>
    </div>

    <!-- Services tiers -->
    <div v-if="scanResult.suspicious_services.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Services tiers actifs ({{ scanResult.suspicious_services.length }})</p>
      <div v-for="(s, i) in scanResult.suspicious_services.slice(0, 15)" :key="i" class="list-row">
        <code class="list-name" style="min-width:130px">{{ s.name }}</code>
        <div class="muted" style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ s.display_name }}</div>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ s.path }}</div>
      </div>
    </div>

    <!-- Autoruns -->
    <div v-if="scanResult.autorun_entries.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Entrées Autorun tiers ({{ scanResult.autorun_entries.length }})</p>
      <div v-for="(a, i) in scanResult.autorun_entries.slice(0, 15)" :key="i" class="list-row">
        <code class="list-name" style="min-width:140px">{{ a.name }}</code>
        <NBadge variant="neutral" style="flex-shrink:0;font-size:10px">{{ a.location }}</NBadge>
        <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ a.path }}</div>
      </div>
    </div>

    <!-- Événements récents -->
    <div v-if="scanResult.recent_errors.length" class="diag-section">
      <p class="diag-section-label" style="margin:0 0 8px 0">Événements d'erreur récents (48h) — {{ scanResult.recent_errors.length }}</p>
      <div v-for="(e, i) in scanResult.recent_errors.slice(0, 10)" :key="i" class="list-row">
        <code class="muted" style="min-width:110px;font-size:10px">{{ e.time }}</code>
        <NBadge :variant="e.level.toLowerCase().includes('critical') || e.level.toLowerCase().includes('critique') ? 'danger' : 'warning'" style="flex-shrink:0;font-size:10px">{{ e.level }}</NBadge>
        <div style="flex:1;min-width:0">
          <span style="font-size:12px;font-weight:500">{{ e.source }}</span>
          <div class="muted" style="font-size:11px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis">{{ e.message }}</div>
        </div>
      </div>
    </div>

    <NButton variant="ghost" size="sm" @click="onRunScan"><RefreshCw :size="12" /> Relancer le scan</NButton>
  </div>
</template>
