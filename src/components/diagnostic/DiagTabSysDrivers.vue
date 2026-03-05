<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import { Search, AlertTriangle, Shield } from "lucide-vue-next";

interface PnpDriver {
  name: string; provider: string; version: string; date: string;
  class: string; inf: string; signed: boolean; status: string; config_error: number;
}
interface SysDriversData {
  drivers: PnpDriver[]; total: number; unsigned_count: number; error_count: number;
}

const data = ref<SysDriversData | null>(null);
const loading = ref(true);
const error = ref("");
const search = ref("");
const filter = ref<"all"|"errors"|"unsigned">("all");

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    data.value = await invoke<SysDriversData>("get_sys_drivers_list");
  } catch (e: any) { error.value = e?.toString() ?? "Erreur"; }
  finally { loading.value = false; }
});

const filtered = computed(() => {
  if (!data.value) return [];
  let list = data.value.drivers;
  if (filter.value === "errors") list = list.filter(d => d.config_error !== 0);
  if (filter.value === "unsigned") list = list.filter(d => !d.signed);
  const q = search.value.toLowerCase();
  if (q) list = list.filter(d => d.name.toLowerCase().includes(q) || d.provider.toLowerCase().includes(q) || d.class.toLowerCase().includes(q));
  return list;
});
</script>

<template>
  <div v-if="loading" style="display:flex;align-items:center;gap:10px;color:var(--text-secondary)">
    <NSpinner :size="16" /><span>Chargement des pilotes...</span>
  </div>
  <div v-else-if="error" style="color:var(--error)">⚠ {{ error }}</div>
  <div v-else-if="data" style="display:flex;flex-direction:column;gap:14px">

    <!-- Stats -->
    <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:10px">
      <div class="diag-section" style="text-align:center">
        <div style="font-size:24px;font-weight:700;color:var(--accent)">{{ data.total }}</div>
        <div style="font-size:11px;color:var(--text-muted)">Pilotes total</div>
      </div>
      <div class="diag-section" style="text-align:center">
        <div style="font-size:24px;font-weight:700" :style="{color: data.unsigned_count>0?'var(--warning)':'var(--success)'}">{{ data.unsigned_count }}</div>
        <div style="font-size:11px;color:var(--text-muted)">Non signés</div>
      </div>
      <div class="diag-section" style="text-align:center">
        <div style="font-size:24px;font-weight:700" :style="{color: data.error_count>0?'var(--error)':'var(--success)'}">{{ data.error_count }}</div>
        <div style="font-size:11px;color:var(--text-muted)">Erreurs</div>
      </div>
    </div>

    <!-- Filtres -->
    <div style="display:flex;gap:8px;align-items:center;flex-wrap:wrap">
      <div style="position:relative;flex:1;min-width:200px">
        <Search :size="12" style="position:absolute;left:8px;top:50%;transform:translateY(-50%);color:var(--text-muted)" />
        <input v-model="search" placeholder="Rechercher..." style="width:100%;padding:5px 8px 5px 26px;background:var(--bg-secondary);border:1px solid var(--border);border-radius:6px;color:var(--text-primary);font-size:12px" />
      </div>
      <button v-for="f in [{k:'all',l:'Tous'},{k:'errors',l:'Erreurs'},{k:'unsigned',l:'Non signés'}]" :key="f.k"
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
        <Shield :size="13" style="display:inline;margin-right:4px" />Pilotes ({{ filtered.length }})
      </p>
      <table style="width:100%;border-collapse:collapse;font-size:11px">
        <thead>
          <tr style="background:var(--bg-secondary)">
            <th style="padding:6px 8px;text-align:left;color:var(--text-muted)">Nom</th>
            <th style="padding:6px 8px;text-align:left;color:var(--text-muted)">Fournisseur</th>
            <th style="padding:6px 8px;text-align:left;color:var(--text-muted)">Classe</th>
            <th style="padding:6px 8px;text-align:left;color:var(--text-muted)">Version</th>
            <th style="padding:6px 8px;text-align:left;color:var(--text-muted)">Signé</th>
            <th style="padding:6px 8px;text-align:left;color:var(--text-muted)">Statut</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(d, i) in filtered.slice(0, 200)" :key="i"
            :style="{borderBottom:'1px solid var(--border)', background:d.config_error!==0?'rgba(239,68,68,0.05)':d.signed?'':'rgba(234,179,8,0.05)'}">
            <td style="padding:5px 8px;max-width:220px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">
              <AlertTriangle v-if="d.config_error!==0" :size="11" style="color:var(--error);margin-right:4px;vertical-align:middle" />
              {{ d.name }}
            </td>
            <td style="padding:5px 8px;color:var(--text-muted);max-width:160px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ d.provider || '—' }}</td>
            <td style="padding:5px 8px;color:var(--text-muted)">{{ d.class || '—' }}</td>
            <td style="padding:5px 8px"><code style="font-size:10px">{{ d.version || '—' }}</code></td>
            <td style="padding:5px 8px">
              <NBadge :variant="d.signed?'success':'warning'" style="font-size:9px">{{ d.signed ? 'Oui' : 'Non' }}</NBadge>
            </td>
            <td style="padding:5px 8px">
              <NBadge :variant="d.config_error===0?'neutral':'danger'" style="font-size:9px">
                {{ d.config_error===0 ? (d.status||'OK') : `Erreur ${d.config_error}` }}
              </NBadge>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-if="filtered.length > 200" style="font-size:11px;color:var(--text-muted);margin-top:6px">{{ filtered.length - 200 }} entrées supplémentaires — affinez la recherche.</p>
    </div>
  </div>
</template>
