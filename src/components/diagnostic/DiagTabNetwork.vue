<script setup lang="ts">
import { ref, computed } from "vue";
import { Search, Wifi, WifiOff } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";

const props = defineProps<{
  tab: string;
  networkAdapters: any[];
  connections: any[];
  wifiInfo: any;
}>();

const connSearch = ref("");
const connFilter = ref("all");

const filteredConns = computed(() => {
  let list = props.connections;
  if (connFilter.value !== "all") list = list.filter(c => c.state === connFilter.value);
  if (connSearch.value) {
    const q = connSearch.value.toLowerCase();
    list = list.filter(c => c.process_name.toLowerCase().includes(q) || c.local_address.includes(q) || c.remote_address.includes(q));
  }
  return list;
});

const stateVariant = (s: string) => {
  if (s === "Established") return "success";
  if (s === "Listen") return "info";
  if (s === "TimeWait") return "warning";
  return "default";
};
</script>

<template>
  <!-- Adaptateurs réseau -->
  <template v-if="tab === 'network'">
    <div v-if="!networkAdapters.length" class="diag-empty">Aucun adaptateur réseau actif</div>
    <template v-else>
      <!-- WiFi Block -->
      <template v-if="wifiInfo">
        <p class="diag-section-label">WiFi actif</p>
        <div class="card-block" style="border:1px solid var(--accent);margin-bottom:16px">
          <div class="block-title">
            <Wifi :size="16" style="color:var(--accent)" />
            <span>{{ wifiInfo.ssid || "N/A" }}</span>
            <NBadge variant="success">{{ wifiInfo.signal_percent }}%</NBadge>
            <NBadge variant="info">{{ wifiInfo.band }}</NBadge>
            <NBadge variant="default">{{ wifiInfo.authentication }}</NBadge>
          </div>
          <div class="info-grid">
            <div class="info-row"><span>BSSID (MAC Point d'accès)</span><code>{{ wifiInfo.bssid || "N/A" }}</code></div>
            <div class="info-row"><span>Signal</span><span>{{ wifiInfo.signal_percent }}%</span></div>
            <div class="info-row"><span>Bande</span><NBadge variant="info">{{ wifiInfo.band }}</NBadge></div>
            <div class="info-row"><span>Canal</span><span>{{ wifiInfo.channel > 0 ? wifiInfo.channel : 'N/A' }}</span></div>
            <div class="info-row"><span>Protocole radio</span><span>{{ wifiInfo.protocol }}</span></div>
            <div class="info-row"><span>Chiffrement</span><span>{{ wifiInfo.security || "N/A" }}</span></div>
            <div class="info-row"><span>Authentification</span><span>{{ wifiInfo.authentication || "N/A" }}</span></div>
            <div class="info-row"><span>Débit réception</span><span>{{ wifiInfo.receive_rate_mbps > 0 ? wifiInfo.receive_rate_mbps + ' Mbps' : 'N/A' }}</span></div>
            <div class="info-row"><span>Débit émission</span><span>{{ wifiInfo.transmit_rate_mbps > 0 ? wifiInfo.transmit_rate_mbps + ' Mbps' : 'N/A' }}</span></div>
            <div class="info-row"><span>Adaptateur</span><span>{{ wifiInfo.adapter_name }}</span></div>
          </div>
        </div>
      </template>
      <div v-else class="card-block" style="display:flex;align-items:center;gap:8px;margin-bottom:16px;color:var(--text-muted)">
        <WifiOff :size="14" /><span style="font-size:13px">Aucune connexion WiFi active</span>
      </div>

      <p class="diag-section-label">Adaptateurs réseau — {{ networkAdapters.length }}</p>
      <div v-for="(a, i) in networkAdapters" :key="i" class="card-block">
        <div class="block-title">
          <span>{{ a.name }}</span>
          <span class="muted" style="font-size:12px;font-weight:400">{{ a.description }}</span>
          <NBadge :variant="a.status === 'Up' || a.status === 'Connected' ? 'success' : 'warning'">{{ a.status }}</NBadge>
          <NBadge v-if="a.is_physical" variant="info">Physique</NBadge>
        </div>
        <div class="info-grid">
          <div class="info-row"><span>Adresse MAC</span><code>{{ a.mac_address || "N/A" }}</code></div>
          <div class="info-row"><span>Vitesse</span><span>{{ a.speed_mbps > 0 ? a.speed_mbps + ' Mbps' : 'N/A' }}</span></div>
          <div class="info-row info-full"><span>Adresses IP</span>
            <div style="display:flex;flex-wrap:wrap;gap:4px;justify-content:flex-end">
              <code v-for="ip in a.ip_addresses" :key="ip">{{ ip }}</code>
              <span v-if="!a.ip_addresses.length" class="muted">N/A</span>
            </div>
          </div>
          <div class="info-row"><span>Passerelle par défaut</span><code>{{ a.default_gateway.join(", ") || "N/A" }}</code></div>
          <div class="info-row"><span>Serveurs DNS</span>
            <div style="display:flex;flex-wrap:wrap;gap:4px;justify-content:flex-end">
              <code v-for="dns in a.dns_servers.slice(0,3)" :key="dns" style="font-size:11px">{{ dns }}</code>
            </div>
          </div>
          <div class="info-row"><span>DHCP</span>
            <NBadge :variant="a.dhcp_enabled ? 'info' : 'default'">{{ a.dhcp_enabled ? "Activé (auto)" : "Manuel (statique)" }}</NBadge>
          </div>
        </div>
      </div>
    </template>
  </template>

  <!-- Connexions TCP/UDP actives -->
  <template v-else-if="tab === 'connections'">
    <div class="diag-search">
      <Search :size="14" />
      <input v-model="connSearch" placeholder="Filtrer par processus, IP..." class="diag-search-input" />
      <span class="muted">{{ filteredConns.length }}/{{ connections.length }}</span>
    </div>
    <div style="display:flex;gap:6px;margin-bottom:12px;flex-wrap:wrap">
      <NBadge v-for="s in ['all','Established','Listen','TimeWait','CloseWait']" :key="s"
        :variant="connFilter === s ? 'info' : 'default'"
        style="cursor:pointer" @click="connFilter = s">
        {{ s === 'all' ? 'Tous' : s }}
      </NBadge>
    </div>
    <div v-if="!filteredConns.length" class="diag-empty">Aucune connexion trouvée</div>
    <div class="table-wrap">
      <table class="data-table">
        <thead>
          <tr>
            <th>Proto</th><th>Processus</th><th>Adresse locale</th><th>Port local</th>
            <th>Adresse distante</th><th>Port distant</th><th>État</th><th>PID</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(c, i) in filteredConns.slice(0, 300)" :key="i">
            <td><NBadge :variant="c.protocol === 'TCP' ? 'info' : 'default'" style="font-size:10px">{{ c.protocol }}</NBadge></td>
            <td style="font-weight:500;max-width:120px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ c.process_name || "—" }}</td>
            <td><code>{{ c.local_address || "*" }}</code></td>
            <td><code>{{ c.local_port }}</code></td>
            <td><code style="color:var(--text-muted)">{{ c.remote_address || "*" }}</code></td>
            <td><code style="color:var(--text-muted)">{{ c.remote_port || "*" }}</code></td>
            <td><NBadge :variant="stateVariant(c.state)" style="font-size:10px">{{ c.state }}</NBadge></td>
            <td class="muted">{{ c.pid }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
</template>
