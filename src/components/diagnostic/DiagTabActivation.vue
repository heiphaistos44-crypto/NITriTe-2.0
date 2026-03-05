<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Key, Shield, ShieldCheck, ShieldOff, ExternalLink, AlertTriangle, CheckCircle } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";

interface WinLicense {
  product_name: string; activation_status: string; partial_product_key: string;
  full_product_key: string; office_name: string; office_status: string;
  office_key: string; office_full_key: string;
}

const licenseInfo = ref<WinLicense | null>(null);
const loading = ref(true);
const activating = ref(false);
const statusMsg = ref("");

onMounted(async () => {
  try { licenseInfo.value = await invoke<WinLicense>("get_windows_license"); }
  catch { /* silencieux */ }
  loading.value = false;
});

async function openMas() {
  activating.value = true;
  statusMsg.value = "Ouverture de la fenêtre d'activation MAS...";
  try {
    await invoke("open_mas_window");
    statusMsg.value = "Fenêtre d'activation ouverte. Suivez les instructions à l'écran.";
  } catch (e: any) {
    statusMsg.value = "Erreur : " + e;
  }
  activating.value = false;
}

function actStatus(s: string): "success" | "danger" | "warning" | "default" {
  if (!s) return "default";
  const l = s.toLowerCase();
  if (l.includes("licenci") || l.includes("activ")) return "success";
  if (l.includes("non") || l.includes("invalide") || l.includes("expire")) return "danger";
  return "warning";
}
</script>

<template>
  <!-- Titre -->
  <p class="diag-section-label">État d'activation</p>

  <div v-if="loading" style="display:flex;align-items:center;gap:8px;font-size:13px;padding:12px 0">
    <NSpinner :size="14" /> Chargement...
  </div>
  <template v-else>
    <!-- Statut Windows -->
    <div class="card-block" style="margin-bottom:12px">
      <div class="block-title">
        <Key :size="15" style="color:var(--accent)" />
        <span>Windows</span>
        <NBadge v-if="licenseInfo" :variant="actStatus(licenseInfo.activation_status)">
          {{ licenseInfo.activation_status || "Inconnu" }}
        </NBadge>
      </div>
      <div v-if="licenseInfo" class="info-grid">
        <div class="info-row"><span>Produit</span><span>{{ licenseInfo.product_name }}</span></div>
        <div class="info-row"><span>Clé complète</span>
          <code v-if="licenseInfo.full_product_key" style="color:var(--accent)">{{ licenseInfo.full_product_key }}</code>
          <span v-else class="muted">XXXXX-XXXXX-XXXXX-XXXXX-{{ licenseInfo.partial_product_key }}</span>
        </div>
      </div>
    </div>

    <!-- Statut Office -->
    <div v-if="licenseInfo && licenseInfo.office_name" class="card-block" style="margin-bottom:12px">
      <div class="block-title">
        <Key :size="15" style="color:var(--warning)" />
        <span>{{ licenseInfo.office_name }}</span>
        <NBadge :variant="actStatus(licenseInfo.office_status)">{{ licenseInfo.office_status || "Inconnu" }}</NBadge>
      </div>
      <div class="info-grid">
        <div class="info-row"><span>Clé complète</span>
          <code v-if="licenseInfo.office_full_key" style="color:var(--accent)">{{ licenseInfo.office_full_key }}</code>
          <span v-else-if="licenseInfo.office_key" class="muted">XXXXX-XXXXX-XXXXX-XXXXX-{{ licenseInfo.office_key }}</span>
          <span v-else class="muted">Non disponible</span>
        </div>
      </div>
    </div>
  </template>

  <!-- MAS Activation -->
  <p class="diag-section-label">Activation MAS (Microsoft Activation Scripts)</p>

  <!-- Warning -->
  <div class="card-block" style="border:1px solid var(--warning);margin-bottom:12px">
    <div style="display:flex;gap:8px;align-items:flex-start">
      <AlertTriangle :size="15" style="color:var(--warning);flex-shrink:0;margin-top:2px" />
      <div style="font-size:12px;line-height:1.6;color:var(--text-muted)">
        <strong style="color:var(--text)">MAS</strong> (massgravel) est un projet open-source qui utilise des méthodes
        légitimes reconnues par Microsoft (HWID, KMS38) pour activer Windows et Office.
        L'activation s'effectue dans une <strong>fenêtre PowerShell élevée</strong> séparée — vous verrez exactement
        ce qui se passe. Aucune donnée n'est envoyée à des tiers.
        <br><br>
        Source officielle :
        <code style="font-size:10px">github.com/massgravel/Microsoft-Activation-Scripts</code>
      </div>
    </div>
  </div>

  <!-- Méthodes -->
  <div class="card-block" style="margin-bottom:12px">
    <p style="font-size:12px;color:var(--text-muted);margin-bottom:12px">
      Cliquer sur un bouton ouvre une fenêtre PowerShell en mode administrateur avec le menu MAS interactif.
      Sélectionnez la méthode souhaitée dans le menu affiché.
    </p>

    <div style="display:grid;grid-template-columns:1fr 1fr;gap:10px;margin-bottom:12px">
      <!-- HWID -->
      <div class="card-block" style="margin:0;border:1px solid var(--border-secondary)">
        <div style="display:flex;align-items:center;gap:6px;margin-bottom:6px">
          <ShieldCheck :size="14" style="color:var(--success)" />
          <strong style="font-size:13px">HWID — Windows</strong>
          <NBadge variant="success" style="font-size:10px">Permanent</NBadge>
        </div>
        <p style="font-size:11px;color:var(--text-muted);margin-bottom:10px">
          Licence numérique permanente liée au matériel. Windows 10/11. Nécessite internet.
        </p>
      </div>

      <!-- KMS38 -->
      <div class="card-block" style="margin:0;border:1px solid var(--border-secondary)">
        <div style="display:flex;align-items:center;gap:6px;margin-bottom:6px">
          <Shield :size="14" style="color:var(--info)" />
          <strong style="font-size:13px">KMS38 — Windows</strong>
          <NBadge variant="info" style="font-size:10px">Jusqu'en 2038</NBadge>
        </div>
        <p style="font-size:11px;color:var(--text-muted);margin-bottom:10px">
          Activation KMS valide jusqu'en 2038. Windows 10/11. Sans internet.
        </p>
      </div>

      <!-- Online KMS -->
      <div class="card-block" style="margin:0;border:1px solid var(--border-secondary)">
        <div style="display:flex;align-items:center;gap:6px;margin-bottom:6px">
          <Shield :size="14" style="color:var(--warning)" />
          <strong style="font-size:13px">KMS Online — W+Office</strong>
          <NBadge variant="warning" style="font-size:10px">180 jours</NBadge>
        </div>
        <p style="font-size:11px;color:var(--text-muted);margin-bottom:10px">
          Active Windows ET Office via KMS. Renouvellement auto tous les 180 jours.
        </p>
      </div>

      <!-- Ohook -->
      <div class="card-block" style="margin:0;border:1px solid var(--border-secondary)">
        <div style="display:flex;align-items:center;gap:6px;margin-bottom:6px">
          <ShieldCheck :size="14" style="color:var(--success)" />
          <strong style="font-size:13px">Ohook — Office</strong>
          <NBadge variant="success" style="font-size:10px">Permanent</NBadge>
        </div>
        <p style="font-size:11px;color:var(--text-muted);margin-bottom:10px">
          Activation permanente d'Office (toutes versions). Sans internet.
        </p>
      </div>
    </div>

    <!-- Bouton unique MAS -->
    <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap">
      <NButton variant="primary" :disabled="activating" @click="openMas">
        <NSpinner v-if="activating" :size="13" />
        <Key v-else :size="14" />
        {{ activating ? 'Ouverture...' : 'Ouvrir le menu MAS (admin)' }}
      </NButton>
      <NButton variant="ghost" size="sm" @click="invoke('open_url', { url: 'https://github.com/massgravel/Microsoft-Activation-Scripts' }).catch(() => {})">
        <ExternalLink :size="12" /> GitHub MAS
      </NButton>
    </div>

    <div v-if="statusMsg" style="margin-top:10px;font-size:12px;display:flex;align-items:center;gap:6px"
      :style="{ color: statusMsg.startsWith('Erreur') ? 'var(--error)' : 'var(--success)' }">
      <CheckCircle v-if="!statusMsg.startsWith('Erreur')" :size="13" />
      {{ statusMsg }}
    </div>
  </div>

  <!-- Lien paramètres Windows -->
  <p class="diag-section-label">Paramètres système</p>
  <div class="card-block">
    <div style="display:flex;gap:8px;flex-wrap:wrap">
      <NButton variant="ghost" size="sm"
        @click="invoke('run_system_command', { command: 'start ms-settings:activation' }).catch(() => {})">
        <ExternalLink :size="12" /> Paramètres d'activation Windows
      </NButton>
      <NButton variant="ghost" size="sm"
        @click="invoke('run_system_command', { command: 'slmgr /xpr' }).catch(() => {})">
        <Key :size="12" /> Vérifier expiration licence (slmgr /xpr)
      </NButton>
      <NButton variant="ghost" size="sm"
        @click="invoke('run_system_command', { command: 'slmgr /dlv' }).catch(() => {})">
        <Key :size="12" /> Infos détaillées licence (slmgr /dlv)
      </NButton>
    </div>
  </div>
</template>
