<script setup lang="ts">
import { CheckCircle, AlertTriangle, Monitor, Cpu, CircuitBoard } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import NProgress from "@/components/ui/NProgress.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";

const props = defineProps<{
  tab: string;
  sysInfo: any;
  biosInfo: any;
  moboInfo: any;
  osExtended: any;
}>();
</script>

<template>
  <!-- OS -->
  <template v-if="tab === 'os'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Monitor"
        title="Système d'exploitation"
        desc="Version, architecture, ressources temps réel et informations avancées"
        color="blue"
      />
      <template v-if="sysInfo">
        <p class="diag-section-label">Système d'exploitation</p>
        <div class="info-grid">
          <div class="info-row"><span>OS</span><span>{{ sysInfo.os.name }} {{ sysInfo.os.version }}</span></div>
          <div class="info-row"><span>Architecture</span><NBadge variant="info">{{ sysInfo.os.architecture }}</NBadge></div>
          <div class="info-row"><span>Hostname</span><code>{{ sysInfo.os.hostname }}</code></div>
          <div class="info-row"><span>Build</span><code>{{ sysInfo.os.build_number }}</code></div>
          <div class="info-row"><span>RAM</span><span>{{ sysInfo.ram.total_gb.toFixed(1) }} GB ({{ Math.round(sysInfo.ram.usage_percent) }}% utilisé)</span></div>
          <div class="info-row"><span>Stockage C:</span>
            <span v-if="sysInfo.disks[0]?.partitions[0]">{{ (sysInfo.disks[0].partitions[0].used_gb || 0).toFixed(0) }} / {{ (sysInfo.disks[0].partitions[0].total_gb || 0).toFixed(0) }} GB</span>
            <span v-else>N/A</span>
          </div>
        </div>

        <p class="diag-section-label">Ressources temps réel</p>
        <div class="diag-stat-row"><span>CPU</span><NProgress :value="sysInfo.cpu.usage_percent" size="sm" showLabel /></div>
        <div class="diag-stat-row"><span>RAM</span><NProgress :value="sysInfo.ram.usage_percent" size="sm" showLabel /></div>
        <div v-if="sysInfo.disks[0]?.partitions[0]" class="diag-stat-row">
          <span>Disque C:</span>
          <NProgress :value="sysInfo.disks[0].partitions[0].usage_percent" :variant="sysInfo.disks[0].partitions[0].usage_percent > 90 ? 'danger' : 'default'" size="sm" showLabel />
        </div>

        <template v-if="osExtended">
          <p class="diag-section-label">Informations avancées</p>
          <div class="info-grid">
            <div class="info-row"><span>Installé le</span><span>{{ osExtended.install_date }}</span></div>
            <div class="info-row"><span>Dernier démarrage</span><span>{{ osExtended.last_boot_time }}</span></div>
            <div class="info-row"><span>Utilisateur enregistré</span><span>{{ osExtended.registered_user || "N/A" }}</span></div>
            <div class="info-row"><span>Organisation</span><span>{{ osExtended.organization || "N/A" }}</span></div>
            <div class="info-row"><span>Langue OS</span><span>{{ osExtended.os_language }}</span></div>
            <div class="info-row"><span>Fuseau horaire</span><span>{{ osExtended.timezone }}</span></div>
            <div class="info-row"><span>Type de produit</span><NBadge variant="info">{{ osExtended.product_type }}</NBadge></div>
            <div class="info-row"><span>Redémarrage en attente</span>
              <NBadge :variant="osExtended.pending_reboot ? 'warning' : 'success'">{{ osExtended.pending_reboot ? "Oui" : "Non" }}</NBadge>
            </div>
            <div class="info-row"><span>Code pays</span><span>{{ osExtended.country_code }}</span></div>
            <div class="info-row"><span>Locale</span><code>{{ osExtended.locale }}</code></div>
          </div>

          <p class="diag-section-label">Répertoires système</p>
          <div class="info-grid">
            <div class="info-row"><span>Windows</span><code class="muted" style="font-size:11px">{{ osExtended.windows_directory }}</code></div>
            <div class="info-row"><span>System32</span><code class="muted" style="font-size:11px">{{ osExtended.system_directory }}</code></div>
            <div class="info-row"><span>Lecteur système</span><span>{{ osExtended.system_drive }}</span></div>
            <div class="info-row"><span>Périphérique de boot</span><code class="muted" style="font-size:11px">{{ osExtended.boot_device }}</code></div>
            <div class="info-row"><span>Fichier de page</span><code class="muted" style="font-size:11px">{{ osExtended.page_file_path }}</code></div>
          </div>

          <p class="diag-section-label">Mémoire virtuelle</p>
          <div class="info-grid">
            <div class="info-row"><span>Mém. virtuelle totale</span><span>{{ osExtended.total_virtual_memory_gb.toFixed(1) }} GB</span></div>
            <div class="info-row"><span>Mém. virtuelle libre</span><span>{{ osExtended.free_virtual_memory_gb.toFixed(1) }} GB</span></div>
            <div class="info-row"><span>Swap total</span><span>{{ osExtended.total_swap_gb > 0 ? osExtended.total_swap_gb.toFixed(1) + ' GB' : 'N/A' }}</span></div>
            <div class="info-row"><span>RAM physique libre</span><span>{{ osExtended.free_physical_gb.toFixed(1) }} GB</span></div>
          </div>

          <p class="diag-section-label">Environnement logiciel</p>
          <div class="info-grid">
            <div class="info-row"><span>PowerShell</span><NBadge variant="info">v{{ osExtended.powershell_version }}</NBadge></div>
            <div class="info-row info-full"><span>.NET Frameworks</span>
              <span class="mono" style="text-align:right">{{ osExtended.dotnet_versions.length ? osExtended.dotnet_versions.join(' · ') : 'N/A' }}</span>
            </div>
          </div>
        </template>
      </template>
      <div v-else class="diag-loading"><div class="diag-spinner"></div> Chargement des informations système...</div>
    </div>
  </template>

  <!-- BIOS -->
  <template v-else-if="tab === 'bios'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Cpu"
        title="BIOS / UEFI"
        desc="Firmware, version SMBIOS, date de sortie et numéro de série système"
        color="amber"
      />
      <template v-if="biosInfo">
        <p class="diag-section-label">BIOS / UEFI</p>
        <div class="info-grid">
          <div class="info-row"><span>Fabricant</span><span>{{ biosInfo.manufacturer }}</span></div>
          <div class="info-row"><span>Version</span><code>{{ biosInfo.version }}</code></div>
          <div class="info-row"><span>Date de sortie</span><span>{{ biosInfo.release_date }}</span></div>
          <div class="info-row"><span>Version SMBIOS</span><NBadge variant="info">{{ biosInfo.smbios_version }}</NBadge></div>
          <div class="info-row info-full"><span>Numéro de série système</span><code>{{ biosInfo.serial_number || "N/A" }}</code></div>
        </div>

        <p class="diag-section-label">Informations complémentaires</p>
        <div class="card-block" style="margin-top:0">
          <div class="diag-badge-row">
            <CheckCircle :size="14" class="ic-ok" />
            <span>Firmware détecté via WMI Win32_BIOS</span>
          </div>
          <div class="diag-badge-row">
            <span class="muted">SMBIOS (System Management BIOS) définit la structure des données matérielles exposées au système d'exploitation.</span>
          </div>
        </div>
      </template>
      <div v-else class="diag-loading"><div class="diag-spinner"></div> Informations BIOS non disponibles...</div>
    </div>
  </template>

  <!-- Carte Mère -->
  <template v-else-if="tab === 'mobo'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="CircuitBoard"
        title="Carte Mère"
        desc="Fabricant, modèle, version et numéro de série du circuit imprimé principal"
        color="teal"
      />
      <template v-if="moboInfo">
        <p class="diag-section-label">Carte Mère</p>
        <div class="info-grid">
          <div class="info-row"><span>Fabricant</span><span>{{ moboInfo.manufacturer }}</span></div>
          <div class="info-row"><span>Modèle</span><span>{{ moboInfo.product }}</span></div>
          <div class="info-row"><span>Version</span><span>{{ moboInfo.version || "N/A" }}</span></div>
          <div class="info-row"><span>Numéro de série</span><code>{{ moboInfo.serial_number || "N/A" }}</code></div>
          <div class="info-row"><span>Statut WMI</span>
            <NBadge :variant="moboInfo.status === 'OK' ? 'success' : 'warning'">{{ moboInfo.status }}</NBadge>
          </div>
        </div>
        <p class="diag-section-label">À propos</p>
        <div class="card-block" style="margin-top:0">
          <p class="muted" style="font-size:12px;line-height:1.6">
            La carte mère est le circuit imprimé principal reliant tous les composants hardware.
            Le numéro de série permet d'identifier la carte pour le support fabricant.
          </p>
        </div>
      </template>
      <div v-else class="diag-loading"><div class="diag-spinner"></div> Informations carte mère non disponibles (droits admin requis)...</div>
    </div>
  </template>
</template>
