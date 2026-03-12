<script setup lang="ts">
import { Monitor, Music, Usb, Battery, Zap, Printer, RefreshCw, Star, FileText, Settings } from "lucide-vue-next";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NBadge from "@/components/ui/NBadge.vue";
import NProgress from "@/components/ui/NProgress.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import NButton from "@/components/ui/NButton.vue";
import NCollapse from "@/components/ui/NCollapse.vue";

const props = defineProps<{
  tab: string;
  monitors: any[];
  audioDevices: any[];
  usbDevices: any[]
  printers: any[];
  batteries: any[];
  powerPlans: any[];
}>();

const emit = defineEmits(["reload"]);

const actionMsg = ref("");
const actionErr = ref(false);
const loading = ref(false);

function showMsg(msg: string, err = false) {
  actionMsg.value = msg;
  actionErr.value = err;
  setTimeout(() => { actionMsg.value = ""; }, 4000);
}

async function openBatteryReport() {
  loading.value = true;
  try {
    await invoke("open_battery_report_html");
    showMsg("Rapport batterie généré et ouvert !");
  } catch (e: any) {
    showMsg(e || "Erreur rapport batterie", true);
  } finally { loading.value = false; }
}

async function activatePlan(guid: string, name: string) {
  loading.value = true;
  try {
    await invoke("set_power_plan", { guid });
    showMsg(`Plan "${name}" activé !`);
    emit("reload");
  } catch (e: any) {
    showMsg(e || "Erreur activation plan", true);
  } finally { loading.value = false; }
}

async function setDefaultPrinter(printerName: string) {
  loading.value = true;
  try {
    await invoke("set_default_printer", { printerName });
    showMsg(`Imprimante "${printerName}" définie par défaut !`);
    emit("reload");
  } catch (e: any) {
    showMsg(e || "Erreur imprimante par défaut", true);
  } finally { loading.value = false; }
}

async function openDeviceManager() {
  try {
    await invoke("open_device_manager", { deviceClass: "" });
    showMsg("Gestionnaire de périphériques ouvert + scan en cours...");
  } catch (e: any) {
    showMsg(e || "Erreur ouverture gestionnaire", true);
  }
}
</script>

<template>
  <!-- Message d'action global -->
  <teleport to="body">
    <div v-if="actionMsg" :class="['action-toast', actionErr ? 'action-toast-err' : 'action-toast-ok']">
      {{ actionMsg }}
    </div>
  </teleport>

  <!-- Écrans -->
  <template v-if="tab === 'monitors'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Monitor"
        title="Moniteurs"
        desc="Écrans connectés, résolutions, densité PPI et taux de rafraîchissement"
        color="purple"
      />
      <div v-if="!monitors.length" class="diag-loading"><div class="diag-spinner"></div> Aucun écran détecté via WMI...</div>
      <template v-else>
        <NCollapse :title="'Écrans connectés — ' + monitors.length" storageKey="diag-devices-monitors" :defaultOpen="true">
          <div v-for="(m, i) in monitors" :key="i" class="card-block">
            <div class="block-title">
              <span>{{ m.name }}</span>
              <NBadge variant="info">{{ m.screen_width }}×{{ m.screen_height }}</NBadge>
              <NBadge v-if="m.availability === 'Principal'" variant="success">Principal</NBadge>
            </div>
            <div class="info-grid">
              <div class="info-row"><span>Résolution</span><span style="font-weight:600">{{ m.screen_width }}×{{ m.screen_height }}</span></div>
              <div class="info-row"><span>Densité (PPI)</span><span>{{ m.pixels_per_inch > 0 ? m.pixels_per_inch + ' PPI' : 'N/A' }}</span></div>
              <div class="info-row"><span>Taux de rafraîchissement</span>
                <NBadge variant="info">{{ m.refresh_rate_hz > 0 ? m.refresh_rate_hz + ' Hz' : (m.availability === 'Principal' ? 'Voir GPU' : 'N/A') }}</NBadge>
              </div>
              <div class="info-row"><span>Fabricant</span><span>{{ m.manufacturer || "N/A" }}</span></div>
              <div class="info-row"><span>Disponibilité</span><NBadge :variant="m.availability === 'Principal' ? 'success' : 'default'">{{ m.availability || "N/A" }}</NBadge></div>
              <div class="info-row"><span>Rapport d'aspect</span>
                <span>{{ m.screen_width > 0 ? (m.screen_width / (m.screen_height || 1)).toFixed(2) + ':1' : 'N/A' }}</span>
              </div>
              <div class="info-row"><span>Classification PPI</span>
                <NBadge :variant="m.pixels_per_inch >= 200 ? 'success' : m.pixels_per_inch >= 140 ? 'info' : 'default'">
                  {{ m.pixels_per_inch >= 200 ? 'HiDPI / Retina' : m.pixels_per_inch >= 140 ? 'QHD / FHD+' : m.pixels_per_inch >= 90 ? 'FHD standard' : 'Basse résolution' }}
                </NBadge>
              </div>
            </div>
          </div>
        </NCollapse>
      </template>
    </div>
  </template>

  <!-- Audio -->
  <template v-else-if="tab === 'audio'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Music"
        title="Audio"
        desc="Périphériques audio installés"
        color="pink"
      />
      <div style="display:flex;gap:8px;margin-bottom:12px">
        <NButton size="sm" variant="secondary" @click="openDeviceManager">
          <RefreshCw :size="13" style="margin-right:5px" />
          Scanner MAJ pilotes audio
        </NButton>
      </div>
      <div v-if="!audioDevices.length" class="diag-loading"><div class="diag-spinner"></div> Aucun périphérique audio...</div>
      <template v-else>
        <NCollapse :title="'Périphériques audio — ' + audioDevices.length" storageKey="diag-devices-audio" :defaultOpen="true">
          <div v-for="(a, i) in audioDevices" :key="i" class="card-block">
            <div class="block-title">
              <span>{{ a.name }}</span>
              <NBadge :variant="a.status === 'OK' ? 'success' : 'warning'">{{ a.status }}</NBadge>
              <NBadge v-if="a.audio_type" variant="info" style="font-size:10px">{{ a.audio_type === 'Render' ? '🔊 Sortie' : '🎙 Entrée' }}</NBadge>
            </div>
            <div class="info-grid">
              <div class="info-row"><span>Fabricant</span><span>{{ a.manufacturer || "N/A" }}</span></div>
              <div class="info-row"><span>Statut</span><NBadge :variant="a.status === 'OK' ? 'success' : 'warning'">{{ a.status }}</NBadge></div>
              <div class="info-row info-full"><span>Device ID</span><code style="font-size:9px;word-break:break-all">{{ a.device_id || "N/A" }}</code></div>
            </div>
          </div>
        </NCollapse>
      </template>
    </div>
  </template>

  <!-- USB -->
  <template v-else-if="tab === 'usb'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Usb"
        title="USB"
        desc="Périphériques USB connectés"
        color="orange"
      />
      <div style="display:flex;gap:8px;margin-bottom:12px">
        <NButton size="sm" variant="secondary" @click="openDeviceManager">
          <RefreshCw :size="13" style="margin-right:5px" />
          Scanner MAJ pilotes USB
        </NButton>
      </div>
      <div v-if="!usbDevices.length" class="diag-loading"><div class="diag-spinner"></div> Aucun périphérique USB détecté...</div>
      <template v-else>
        <NCollapse :title="'Périphériques USB — ' + usbDevices.length" storageKey="diag-devices-usb" :defaultOpen="true">
          <div class="table-wrap">
            <table class="data-table">
              <thead>
                <tr><th>Nom</th><th>Catégorie</th><th>Fabricant</th><th>Statut</th></tr>
              </thead>
              <tbody>
                <tr v-for="(u, i) in usbDevices" :key="i">
                  <td style="font-weight:500">{{ u.name }}</td>
                  <td><NBadge variant="default" style="font-size:10px">{{ u.pnp_class || "—" }}</NBadge></td>
                  <td class="muted">{{ u.manufacturer || "—" }}</td>
                  <td><NBadge :variant="u.status === 'OK' ? 'success' : 'warning'" style="font-size:10px">{{ u.status }}</NBadge></td>
                </tr>
              </tbody>
            </table>
          </div>
        </NCollapse>
      </template>
    </div>
  </template>

  <!-- Imprimantes -->
  <template v-else-if="tab === 'printers'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Printer"
        title="Imprimantes"
        desc="Imprimantes et périphériques d'impression"
        color="slate"
      />
      <div v-if="!printers.length" class="diag-loading"><div class="diag-spinner"></div> Aucune imprimante installée...</div>
      <template v-else>
        <NCollapse :title="'Imprimantes — ' + printers.length" storageKey="diag-devices-printers" :defaultOpen="true">
          <div v-for="(p, i) in printers" :key="i" class="card-block">
            <div class="block-title">
              <span>{{ p.name }}</span>
              <NBadge v-if="p.is_default" variant="success">⭐ Défaut</NBadge>
              <NBadge v-if="p.is_network" variant="info">Réseau</NBadge>
              <NBadge v-if="p.shared" variant="neutral">Partagée</NBadge>
            </div>
            <div class="info-grid">
              <div class="info-row"><span>Driver</span><span>{{ p.driver_name }}</span></div>
              <div class="info-row"><span>Port</span><code>{{ p.port_name }}</code></div>
              <div class="info-row"><span>Réseau</span><NBadge :variant="p.is_network ? 'info' : 'default'">{{ p.is_network ? "Oui" : "Non" }}</NBadge></div>
              <div class="info-row"><span>Partagée</span><span>{{ p.shared ? "Oui" : "Non" }}</span></div>
              <div class="info-row"><span>Statut</span><NBadge :variant="p.status === 'OK' || p.status === 'Normal' ? 'success' : 'warning'">{{ p.status || "N/A" }}</NBadge></div>
            </div>
            <div style="margin-top:10px">
              <NButton v-if="!p.is_default" size="sm" variant="secondary" :disabled="loading" @click="setDefaultPrinter(p.name)">
                <Star :size="13" style="margin-right:5px" />
                Définir par défaut
              </NButton>
            </div>
          </div>
        </NCollapse>
      </template>
    </div>
  </template>

  <!-- Batterie -->
  <template v-else-if="tab === 'battery'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Battery"
        title="Batterie"
        desc="État, santé, cycles et chimie de la batterie"
        color="green"
      />

      <!-- Bouton rapport batterie -->
      <div style="display:flex;gap:8px;margin-bottom:12px">
        <NButton size="sm" variant="secondary" :disabled="loading" @click="openBatteryReport">
          <FileText :size="13" style="margin-right:5px" />
          {{ loading ? 'Génération...' : 'Rapport complet (HTML)' }}
        </NButton>
      </div>

      <div v-if="!batteries.length" class="diag-loading"><div class="diag-spinner"></div> Pas de batterie détectée (PC de bureau)...</div>
      <div v-for="(b, i) in batteries" :key="i" class="card-block">
        <div class="block-title">{{ b.name }}</div>
        <div style="display:flex;align-items:center;gap:16px;margin-bottom:16px">
          <div style="font-size:36px;font-weight:700;color:var(--accent);min-width:80px">{{ b.estimated_charge_remaining }}%</div>
          <div style="flex:1">
            <NProgress :value="b.estimated_charge_remaining" :variant="b.estimated_charge_remaining > 20 ? 'default' : 'danger'" size="lg" />
            <div style="display:flex;justify-content:space-between;margin-top:4px">
              <span class="muted" style="font-size:11px">{{ b.status }}</span>
              <span class="muted" style="font-size:11px">{{ b.estimated_run_time }}</span>
            </div>
          </div>
        </div>
        <div style="display:flex;gap:16px;margin-bottom:16px;flex-wrap:wrap">
          <div style="text-align:center">
            <div style="font-size:22px;font-weight:700" :class="b.battery_health_percent > 80 ? 'ic-ok' : b.battery_health_percent > 50 ? 'text-warn' : 'text-err'">
              {{ b.battery_health_percent.toFixed(0) }}%
            </div>
            <div class="muted" style="font-size:10px">SANTÉ</div>
          </div>
          <div style="text-align:center">
            <div style="font-size:22px;font-weight:700">{{ b.cycle_count > 0 ? b.cycle_count : 'N/A' }}</div>
            <div class="muted" style="font-size:10px">CYCLES</div>
          </div>
          <div style="text-align:center">
            <div style="font-size:22px;font-weight:700">{{ b.design_capacity > 0 ? b.design_capacity : 'N/A' }}</div>
            <div class="muted" style="font-size:10px">mWh DESIGN</div>
          </div>
          <div style="text-align:center">
            <div style="font-size:22px;font-weight:700">{{ b.full_charge_capacity > 0 ? b.full_charge_capacity : 'N/A' }}</div>
            <div class="muted" style="font-size:10px">mWh ACTUEL</div>
          </div>
        </div>
        <div class="info-grid">
          <div class="info-row"><span>Statut</span><NBadge variant="info">{{ b.status }}</NBadge></div>
          <div class="info-row"><span>Chimie</span><NBadge variant="default">{{ b.chemistry }}</NBadge></div>
          <div class="info-row"><span>Santé</span>
            <NBadge :variant="b.battery_health_percent > 80 ? 'success' : b.battery_health_percent > 50 ? 'warning' : 'danger'">
              {{ b.battery_health_percent.toFixed(1) }}%
            </NBadge>
          </div>
          <div class="info-row"><span>Autonomie estimée</span><span>{{ b.estimated_run_time }}</span></div>
          <div class="info-row"><span>Nombre de cycles</span><span>{{ b.cycle_count > 0 ? b.cycle_count : 'N/A' }}</span></div>
          <div class="info-row"><span>Capacité de conception</span><span>{{ b.design_capacity > 0 ? b.design_capacity + ' mWh' : 'N/A' }}</span></div>
          <div class="info-row"><span>Capacité actuelle (pleine charge)</span><span>{{ b.full_charge_capacity > 0 ? b.full_charge_capacity + ' mWh' : 'N/A' }}</span></div>
          <div class="info-row"><span>Usure (perte de capacité)</span>
            <span v-if="b.design_capacity > 0 && b.full_charge_capacity > 0">
              {{ (b.design_capacity - b.full_charge_capacity) }} mWh perdu(s)
            </span>
            <span v-else>N/A</span>
          </div>
        </div>
      </div>
    </div>
  </template>

  <!-- Plans d'énergie -->
  <template v-else-if="tab === 'power'">
    <div class="diag-tab-content">
      <DiagBanner
        :icon="Zap"
        title="Plans d'alimentation"
        desc="Gestion énergétique Windows — Activer un plan en un clic"
        color="amber"
      />
      <NCollapse :title="'Plans d\'alimentation Windows — ' + powerPlans.length" storageKey="diag-devices-power" :defaultOpen="true">
        <div v-if="!powerPlans.length" class="diag-loading"><div class="diag-spinner"></div> Aucun plan d'énergie...</div>
        <div v-for="(p, i) in powerPlans" :key="i" class="card-block">
          <div class="block-title">
            <span>{{ p.name }}</span>
            <NBadge v-if="p.is_active" variant="success">⚡ ACTIF</NBadge>
          </div>
          <div class="info-row"><span>GUID</span><code style="font-size:10px">{{ p.guid }}</code></div>
          <div style="margin-top:10px">
            <NButton
              v-if="!p.is_active"
              size="sm"
              variant="secondary"
              :disabled="loading"
              @click="activatePlan(p.guid, p.name)"
            >
              <Zap :size="13" style="margin-right:5px" />
              Activer ce plan
            </NButton>
          </div>
        </div>
        <div class="card-block" style="margin-top:8px">
          <p class="muted" style="font-size:12px;line-height:1.6">
            Les plans d'énergie contrôlent la fréquence CPU, la mise en veille des disques,
            et la gestion thermique. Le plan <strong>Haute Performance</strong> désactive les économies d'énergie
            pour des performances maximales.
          </p>
        </div>
      </NCollapse>
    </div>
  </template>
</template>

<style scoped>
.action-toast {
  position: fixed;
  bottom: 24px;
  right: 24px;
  padding: 12px 20px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  z-index: 9999;
  box-shadow: 0 4px 16px rgba(0,0,0,0.3);
}
.action-toast-ok { background: #1a4a1a; border: 1px solid #22c55e; color: #86efac; }
.action-toast-err { background: #4a1a1a; border: 1px solid #ef4444; color: #fca5a5; }
</style>
