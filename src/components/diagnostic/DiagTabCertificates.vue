<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import { Lock, Search, ShieldAlert } from "lucide-vue-next";

interface CertEntry {
  subject: string; issuer: string; thumbprint: string;
  not_before: string; not_after: string; store: string;
  is_expired: boolean; has_private_key: boolean;
}
interface CertsData {
  certs: CertEntry[]; total: number; expired_count: number; expiring_soon_count: number;
}

const data = ref<CertsData | null>(null);
const loading = ref(true);
const error = ref("");
const search = ref("");
const filter = ref<"all"|"expired"|"expiring"|"pk">("all");

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    data.value = await invoke<CertsData>("get_certificates");
  } catch (e: any) { error.value = e?.toString() ?? "Erreur"; }
  finally { loading.value = false; }
});

function cn(subject: string) {
  const m = subject.match(/CN=([^,]+)/);
  return m ? m[1] : subject.split(',')[0];
}

const filtered = computed(() => {
  if (!data.value) return [];
  let list = data.value.certs;
  const today = new Date();
  const soon = new Date(); soon.setDate(soon.getDate() + 30);
  if (filter.value === "expired") list = list.filter(c => c.is_expired);
  if (filter.value === "expiring") list = list.filter(c => !c.is_expired && new Date(c.not_after) < soon);
  if (filter.value === "pk") list = list.filter(c => c.has_private_key);
  const q = search.value.toLowerCase();
  if (q) list = list.filter(c => c.subject.toLowerCase().includes(q) || c.issuer.toLowerCase().includes(q) || c.store.toLowerCase().includes(q));
  return list;
});
</script>

<template>
  <div class="diag-tab-content">
    <DiagBanner :icon="Lock" title="Certificats Numériques" desc="Certificats installés dans les magasins Windows" color="gold" />

    <div v-if="loading" class="diag-loading"><div class="diag-spinner"></div> Chargement des certificats...</div>
    <div v-else-if="error" style="color:var(--error)">⚠ {{ error }}</div>
    <div v-else-if="data" style="display:flex;flex-direction:column;gap:14px">

      <!-- Stats -->
      <div style="display:grid;grid-template-columns:repeat(4,1fr);gap:10px">
        <div class="diag-section" style="text-align:center">
          <div style="font-size:22px;font-weight:700;color:var(--accent)">{{ data.total }}</div>
          <div style="font-size:12px;color:var(--text-secondary)">Total</div>
        </div>
        <div class="diag-section" style="text-align:center">
          <div style="font-size:22px;font-weight:700" :style="{color:data.expired_count>0?'var(--error)':'var(--success)'}">{{ data.expired_count }}</div>
          <div style="font-size:12px;color:var(--text-secondary)">Expirés</div>
        </div>
        <div class="diag-section" style="text-align:center">
          <div style="font-size:22px;font-weight:700" :style="{color:data.expiring_soon_count>0?'var(--warning)':'var(--success)'}">{{ data.expiring_soon_count }}</div>
          <div style="font-size:12px;color:var(--text-secondary)">Expirent bientôt</div>
        </div>
        <div class="diag-section" style="text-align:center">
          <div style="font-size:22px;font-weight:700;color:var(--accent)">{{ data.certs.filter(c=>c.has_private_key).length }}</div>
          <div style="font-size:12px;color:var(--text-secondary)">Avec clé privée</div>
        </div>
      </div>

      <!-- Filtres + Recherche -->
      <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap">
        <div style="position:relative;flex:1;min-width:200px">
          <Search :size="12" style="position:absolute;left:8px;top:50%;transform:translateY(-50%);color:var(--text-secondary)" />
          <input v-model="search" placeholder="Sujet, émetteur, store..." style="width:100%;padding:5px 8px 5px 26px;background:var(--bg-secondary);border:1px solid var(--border);border-radius:6px;color:var(--text-primary);font-size:12px" />
        </div>
        <button v-for="f in [{k:'all',l:'Tous'},{k:'expired',l:'Expirés'},{k:'expiring',l:'30 jours'},{k:'pk',l:'Clé privée'}]" :key="f.k"
          @click="filter = f.k as any"
          :style="{padding:'4px 10px',borderRadius:'6px',border:'1px solid var(--border)',fontSize:'11px',cursor:'pointer',
                   background:filter===f.k?'var(--accent)':'var(--bg-secondary)',
                   color:filter===f.k?'white':'var(--text-secondary)'}">
          {{ f.l }}
        </button>
      </div>

      <!-- Table -->
      <div class="diag-section" style="overflow-x:auto">
        <p class="diag-section-label" style="margin:0 0 8px 0">
          <Lock :size="13" style="display:inline;margin-right:4px" />Certificats ({{ filtered.length }})
        </p>
        <table style="width:100%;border-collapse:collapse;font-size:11px">
          <thead>
            <tr style="background:var(--bg-secondary)">
              <th style="padding:6px 8px;text-align:left;color:var(--text-secondary)">Sujet (CN)</th>
              <th style="padding:6px 8px;text-align:left;color:var(--text-secondary)">Émetteur</th>
              <th style="padding:6px 8px;text-align:left;color:var(--text-secondary)">Store</th>
              <th style="padding:6px 8px;text-align:left;color:var(--text-secondary)">Début</th>
              <th style="padding:6px 8px;text-align:left;color:var(--text-secondary)">Fin</th>
              <th style="padding:6px 8px;text-align:left;color:var(--text-secondary)">Statut</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(c, i) in filtered.slice(0, 150)" :key="i"
              :style="{borderBottom:'1px solid var(--border)',background:c.is_expired?'rgba(239,68,68,0.05)':''}">
              <td style="padding:5px 8px;max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">
                <Lock v-if="c.has_private_key" :size="10" style="color:var(--accent);margin-right:3px;vertical-align:middle" />
                {{ cn(c.subject) }}
              </td>
              <td style="padding:5px 8px;color:var(--text-secondary);max-width:160px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ cn(c.issuer) }}</td>
              <td style="padding:5px 8px"><code style="font-size:10px">{{ c.store }}</code></td>
              <td style="padding:5px 8px;color:var(--text-secondary)">{{ c.not_before }}</td>
              <td style="padding:5px 8px" :style="{color:c.is_expired?'var(--error)':''}">{{ c.not_after }}</td>
              <td style="padding:5px 8px">
                <NBadge :variant="c.is_expired?'danger':'success'" style="font-size:9px">
                  {{ c.is_expired ? 'Expiré' : 'Valide' }}
                </NBadge>
              </td>
            </tr>
          </tbody>
        </table>
        <p v-if="filtered.length > 150" style="font-size:12px;color:var(--text-secondary);margin-top:6px">{{ filtered.length - 150 }} certificats supplémentaires — affinez la recherche.</p>
      </div>
    </div>
  </div>
</template>
