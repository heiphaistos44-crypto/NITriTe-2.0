<script setup lang="ts">
import { CheckCircle, AlertTriangle, XCircle, ShieldCheck, ShieldOff, Shield, Key, RefreshCw } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import NCollapse from "@/components/ui/NCollapse.vue";

const props = defineProps<{
  tab: string;
  securityInfo: any;
  licenseInfo: any;
  updatesHistory: any[];
}>();

function secIcon(ok: boolean) { return ok ? CheckCircle : AlertTriangle; }
function secClass(ok: boolean) { return ok ? "ic-ok" : "ic-warn"; }
</script>

<template>
  <!-- Sécurité & Protection -->
  <template v-if="tab === 'security'">
    <div class="diag-tab-content">
      <DiagBanner :icon="Shield" title="Sécurité Windows" desc="Defender, pare-feu, TPM, Secure Boot et protection avancée" color="red" />
      <div v-if="!securityInfo" class="diag-empty">Informations de sécurité non disponibles (droits admin recommandés)</div>
      <template v-else>
        <!-- Score global -->
        <div class="card-block" style="background:linear-gradient(135deg,#1a1a2e,var(--bg-tertiary));border:1px solid var(--border)">
          <div style="display:flex;align-items:center;gap:16px;flex-wrap:wrap">
            <component :is="[securityInfo.secure_boot.includes('Activé'),securityInfo.tpm_enabled,securityInfo.uac_enabled,securityInfo.firewall_private,securityInfo.windows_defender_realtime].filter(Boolean).length >= 4 ? ShieldCheck : ShieldOff"
              :size="40" :style="{color: [securityInfo.secure_boot.includes('Activé'),securityInfo.tpm_enabled,securityInfo.uac_enabled,securityInfo.firewall_private,securityInfo.windows_defender_realtime].filter(Boolean).length >= 4 ? 'var(--success)' : 'var(--warning)'}" />
            <div>
              <div style="font-size:18px;font-weight:700">Posture de sécurité Windows</div>
              <div class="muted" style="font-size:12px">{{ securityInfo.antivirus_name }} — {{ securityInfo.antivirus_state }}</div>
            </div>
          </div>
        </div>

        <NCollapse title="Démarrage sécurisé & Virtualisation" storageKey="diag-security-boot" :defaultOpen="true">
          <div class="diag-section">
            <div class="diag-badge-row">
              <component :is="securityInfo.secure_boot.includes('Activé') ? CheckCircle : AlertTriangle" :size="14" :class="securityInfo.secure_boot.includes('Activé') ? 'ic-ok' : 'ic-warn'" />
              <span>Secure Boot</span>
              <span class="mono">{{ securityInfo.secure_boot }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.tpm_enabled)" :size="14" :class="secClass(securityInfo.tpm_enabled)" />
              <span>TPM</span>
              <span class="mono">{{ securityInfo.tpm_enabled ? 'Activé' : 'Désactivé' }} — v{{ securityInfo.tpm_version }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.tpm_activated)" :size="14" :class="secClass(securityInfo.tpm_activated)" />
              <span>TPM Activé</span>
              <span class="mono">{{ securityInfo.tpm_activated ? 'Oui' : 'Non' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.vbs_enabled)" :size="14" :class="secClass(securityInfo.vbs_enabled)" />
              <span>Virtualization Based Security (VBS)</span>
              <span class="mono">{{ securityInfo.vbs_enabled ? 'Activée' : 'Désactivée' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.hvci_enabled)" :size="14" :class="secClass(securityInfo.hvci_enabled)" />
              <span>Hypervisor Protected Code Integrity (HVCI)</span>
              <span class="mono">{{ securityInfo.hvci_enabled ? 'Activé' : 'Désactivé' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.credential_guard)" :size="14" :class="secClass(securityInfo.credential_guard)" />
              <span>Credential Guard</span>
              <span class="mono">{{ securityInfo.credential_guard ? 'Actif' : 'Inactif' }}</span>
            </div>
          </div>
        </NCollapse>

        <NCollapse title="Antivirus & Defender" storageKey="diag-security-antivirus" :defaultOpen="true">
          <div class="diag-section">
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.windows_defender_realtime)" :size="14" :class="secClass(securityInfo.windows_defender_realtime)" />
              <span>Windows Defender (Protection temps réel)</span>
              <span class="mono">{{ securityInfo.windows_defender_realtime ? 'Actif' : 'INACTIF ⚠' }}</span>
            </div>
            <div class="diag-badge-row">
              <CheckCircle :size="14" class="ic-ok" />
              <span>Antivirus détecté</span>
              <span class="mono">{{ securityInfo.antivirus_name }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="securityInfo.antivirus_state === 'À jour' ? CheckCircle : AlertTriangle" :size="14" :class="securityInfo.antivirus_state === 'À jour' ? 'ic-ok' : 'ic-warn'" />
              <span>État des définitions antivirus</span>
              <span class="mono">{{ securityInfo.antivirus_state }}</span>
            </div>
            <div class="info-row" style="padding:8px 0">
              <span>Dernière mise à jour définitions Defender</span>
              <span>{{ securityInfo.defender_definitions_date }}</span>
            </div>
            <div class="info-row" style="padding:8px 0">
              <span>Dernier scan complet</span>
              <span>{{ securityInfo.last_full_scan }}</span>
            </div>
          </div>
        </NCollapse>

        <NCollapse title="Pare-feu Windows" storageKey="diag-security-firewall" :defaultOpen="true">
          <div class="diag-section">
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.firewall_domain)" :size="14" :class="secClass(securityInfo.firewall_domain)" />
              <span>Profil Domaine</span>
              <span class="mono">{{ securityInfo.firewall_domain ? 'Activé' : 'DÉSACTIVÉ' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.firewall_private)" :size="14" :class="secClass(securityInfo.firewall_private)" />
              <span>Profil Privé</span>
              <span class="mono">{{ securityInfo.firewall_private ? 'Activé' : 'DÉSACTIVÉ' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.firewall_public)" :size="14" :class="secClass(securityInfo.firewall_public)" />
              <span>Profil Public</span>
              <span class="mono">{{ securityInfo.firewall_public ? 'Activé' : 'DÉSACTIVÉ' }}</span>
            </div>
          </div>
        </NCollapse>

        <NCollapse title="Contrôle & Accès" storageKey="diag-security-access" :defaultOpen="true">
          <div class="diag-section">
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.uac_enabled)" :size="14" :class="secClass(securityInfo.uac_enabled)" />
              <span>Contrôle de compte d'utilisateur (UAC)</span>
              <span class="mono">{{ securityInfo.uac_enabled ? 'Activé' : 'DÉSACTIVÉ' }}</span>
            </div>
            <div class="info-row" style="padding:8px 0">
              <span>Niveau UAC</span><span>{{ securityInfo.uac_level }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.lsa_protection)" :size="14" :class="secClass(securityInfo.lsa_protection)" />
              <span>Protection LSA (LSASS.exe)</span>
              <span class="mono">{{ securityInfo.lsa_protection ? 'Activée (PPL)' : 'Standard' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="secIcon(securityInfo.smartscreen_enabled)" :size="14" :class="secClass(securityInfo.smartscreen_enabled)" />
              <span>Windows SmartScreen</span>
              <span class="mono">{{ securityInfo.smartscreen_enabled ? 'Activé' : 'Désactivé' }}</span>
            </div>
            <div class="diag-badge-row">
              <component :is="securityInfo.windows_hello ? CheckCircle : XCircle" :size="14" :class="securityInfo.windows_hello ? 'ic-ok' : 'muted'" />
              <span>Windows Hello</span>
              <span class="mono">{{ securityInfo.windows_hello ? 'Détecté' : 'Non détecté' }}</span>
            </div>
          </div>
        </NCollapse>

        <NCollapse
          :title="securityInfo.bitlocker_drives.length > 0 ? 'BitLocker — ' + securityInfo.bitlocker_drives.length + ' volume(s)' : 'BitLocker'"
          storageKey="diag-security-bitlocker"
          :defaultOpen="true"
        >
          <template v-if="securityInfo.bitlocker_drives.length > 0">
            <div v-for="(d, i) in securityInfo.bitlocker_drives" :key="i" class="card-block">
              <div class="block-title">
                <code>{{ d.drive_letter }}</code>
                <NBadge :variant="d.protection_status === 'On' ? 'success' : 'warning'">
                  {{ d.protection_status === 'On' ? 'Chiffré' : d.protection_status === 'Off' ? 'Non chiffré' : d.protection_status }}
                </NBadge>
              </div>
              <div class="info-grid">
                <div class="info-row"><span>Protection</span><span>{{ d.protection_status }}</span></div>
                <div class="info-row"><span>Méthode de chiffrement</span><span>{{ d.encryption_method }}</span></div>
                <div class="info-row"><span>État de conversion</span><span>{{ d.conversion_status }}</span></div>
                <div class="info-row"><span>Statut de verrouillage</span><span>{{ d.lock_status }}</span></div>
              </div>
            </div>
          </template>
          <div v-else class="card-block">
            <div class="diag-badge-row">
              <AlertTriangle :size="14" class="ic-warn" />
              <span>BitLocker non activé sur les volumes détectés</span>
            </div>
          </div>
        </NCollapse>
      </template>
    </div>
  </template>

  <!-- Licence Windows -->
  <template v-else-if="tab === 'license'">
    <div class="diag-tab-content">
      <DiagBanner :icon="Key" title="Licence Windows & Office" desc="Clés produit, activation et informations de licence" color="gold" />
      <div v-if="licenseInfo">
        <NCollapse title="Licence Windows" storageKey="diag-license-windows" :defaultOpen="true">
          <div class="info-grid mb-12">
            <div class="info-row info-full"><span>Produit</span><span style="font-size:11px;text-align:right;max-width:300px;overflow:hidden;text-overflow:ellipsis">{{ licenseInfo.product_name }}</span></div>
            <div class="info-row"><span>Statut d'activation</span>
              <NBadge :variant="licenseInfo.activation_status === 'Licencié' || licenseInfo.activation_status === 'Activé' ? 'success' : 'danger'">
                {{ licenseInfo.activation_status }}
              </NBadge>
            </div>
            <div class="info-row info-full">
              <span>Clé produit</span>
              <code style="font-size:12px;letter-spacing:1px;color:var(--accent)">
                {{ licenseInfo.full_product_key || ('XXXXX-XXXXX-XXXXX-XXXXX-' + licenseInfo.partial_product_key) }}
              </code>
            </div>
            <div v-if="!licenseInfo.full_product_key" class="info-row info-full">
              <span></span><span class="muted" style="font-size:11px">⚠ Clé complète non disponible — licence numérique ou droits insuffisants</span>
            </div>
            <div class="info-row"><span>Statut de licence</span><span>{{ licenseInfo.license_status }}</span></div>
            <div class="info-row"><span>Famille de licence</span><span>{{ licenseInfo.license_family || "N/A" }}</span></div>
          </div>
        </NCollapse>
        <template v-if="licenseInfo.office_name">
          <NCollapse title="Microsoft Office / 365" storageKey="diag-license-office" :defaultOpen="true">
            <div class="info-grid">
              <div class="info-row info-full"><span>Produit</span><span style="font-size:11px;text-align:right">{{ licenseInfo.office_name }}</span></div>
              <div class="info-row"><span>Statut</span>
                <NBadge :variant="licenseInfo.office_status === 'Licencié' || licenseInfo.office_status === 'Activé' ? 'success' : 'warning'">
                  {{ licenseInfo.office_status }}
                </NBadge>
              </div>
              <div class="info-row"><span>Clé partielle</span><code>XXXXX-XXXXX-XXXXX-XXXXX-{{ licenseInfo.office_key }}</code></div>
            </div>
          </NCollapse>
        </template>
        <div v-else class="diag-empty" style="padding:12px 4px;text-align:left">Office / Microsoft 365 non détecté</div>
      </div>
      <div v-else class="diag-empty">Informations de licence non disponibles</div>
    </div>
  </template>

  <!-- Mises à jour -->
  <template v-else-if="tab === 'updates'">
    <div class="diag-tab-content">
      <DiagBanner :icon="RefreshCw" title="Mises à Jour Windows" desc="Historique des mises à jour installées" color="green" />
      <NCollapse :title="'Historique des mises à jour installées — ' + updatesHistory.length" storageKey="diag-security-updates-history" :defaultOpen="true">
        <div v-if="!updatesHistory.length" class="diag-empty">Aucune mise à jour dans l'historique</div>
        <div class="table-wrap">
          <table class="data-table">
            <thead><tr><th>KB ID</th><th>Description</th><th>Installé le</th><th>Par</th></tr></thead>
            <tbody>
              <tr v-for="(u, i) in updatesHistory.slice(0, 200)" :key="i">
                <td><code>{{ u.hotfix_id }}</code></td>
                <td>{{ u.description }}</td>
                <td class="muted">{{ u.installed_on }}</td>
                <td class="muted">{{ u.installed_by || "—" }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </NCollapse>
    </div>
  </template>
</template>
