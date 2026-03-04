<script setup lang="ts">
import { ref, computed } from "vue";
import { Search } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";

const props = defineProps<{
  tab: string;
  softwareList: any[];
  envVars: any[];
}>();

const softwareSearch = ref("");
const envSearch = ref("");
const sortBy = ref<"name"|"date"|"size"|"publisher">("name");

const filteredSoftware = computed(() => {
  let list = [...props.softwareList];
  const q = softwareSearch.value.toLowerCase();
  if (q) list = list.filter(s => s.name.toLowerCase().includes(q) || (s.publisher || "").toLowerCase().includes(q));
  if (sortBy.value === "date") list.sort((a, b) => (b.install_date || "").localeCompare(a.install_date || ""));
  else if (sortBy.value === "size") list.sort((a, b) => (b.estimated_size_mb || 0) - (a.estimated_size_mb || 0));
  else if (sortBy.value === "publisher") list.sort((a, b) => (a.publisher || "").localeCompare(b.publisher || ""));
  else list.sort((a, b) => a.name.localeCompare(b.name));
  return list;
});

const filteredEnv = computed(() => {
  const q = envSearch.value.toLowerCase();
  if (!q) return props.envVars;
  return props.envVars.filter(e => e.name.toLowerCase().includes(q) || e.value.toLowerCase().includes(q));
});

function mbStr(v: number): string {
  return v >= 1024 ? `${(v / 1024).toFixed(1)} GB` : `${v.toFixed(0)} MB`;
}

const topBySize = computed(() =>
  [...props.softwareList].filter(s => s.estimated_size_mb > 0)
    .sort((a, b) => b.estimated_size_mb - a.estimated_size_mb).slice(0, 10)
);

const byPublisher = computed(() => {
  const map = new Map<string, number>();
  for (const s of props.softwareList) {
    const p = s.publisher || "Inconnu";
    map.set(p, (map.get(p) || 0) + 1);
  }
  return [...map.entries()].sort((a, b) => b[1] - a[1]).slice(0, 10);
});
</script>

<template>
  <!-- Logiciels installés -->
  <template v-if="tab === 'software'">
    <div style="display:flex;gap:12px;align-items:center;flex-wrap:wrap;margin-bottom:12px">
      <div class="diag-search" style="flex:1;margin-bottom:0">
        <Search :size="14" />
        <input v-model="softwareSearch" placeholder="Rechercher un logiciel ou éditeur..." class="diag-search-input" />
        <span class="muted">{{ filteredSoftware.length }}/{{ softwareList.length }}</span>
      </div>
      <div style="display:flex;gap:4px">
        <NBadge v-for="s in [['name','Nom'],['date','Date'],['size','Taille'],['publisher','Éditeur']]" :key="s[0]"
          :variant="sortBy === s[0] ? 'info' : 'default'"
          style="cursor:pointer" @click="sortBy = s[0] as any">{{ s[1] }}</NBadge>
      </div>
    </div>

    <div style="display:flex;gap:12px;flex-wrap:wrap;margin-bottom:16px">
      <div class="card-block" style="flex:1;min-width:180px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Total installés</p>
        <span style="font-size:28px;font-weight:700;color:var(--accent)">{{ softwareList.length }}</span>
      </div>
      <div class="card-block" style="flex:1;min-width:180px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Taille totale estimée</p>
        <span style="font-size:20px;font-weight:700">{{ mbStr(softwareList.reduce((a, s) => a + (s.estimated_size_mb || 0), 0)) }}</span>
      </div>
    </div>

    <p class="diag-section-label">Top 10 plus volumineux</p>
    <div v-for="(s, i) in topBySize" :key="i" class="list-row">
      <span class="muted" style="min-width:20px;font-size:11px">{{ i + 1 }}.</span>
      <span class="list-name">{{ s.name }}</span>
      <span class="muted" style="font-size:11px">{{ s.publisher || "—" }}</span>
      <NBadge variant="warning">{{ mbStr(s.estimated_size_mb) }}</NBadge>
    </div>

    <p class="diag-section-label" style="margin-top:16px">Top éditeurs ({{ byPublisher.length }})</p>
    <div style="display:flex;gap:6px;flex-wrap:wrap;margin-bottom:16px">
      <NBadge v-for="([pub, count]) in byPublisher" :key="pub" variant="default">
        {{ pub }} <span style="font-size:10px;margin-left:4px;opacity:0.7">×{{ count }}</span>
      </NBadge>
    </div>

    <p class="diag-section-label">Liste complète</p>
    <div class="table-wrap">
      <table class="data-table">
        <thead><tr><th>#</th><th>Nom</th><th>Version</th><th>Éditeur</th><th>Date install.</th><th>Taille</th></tr></thead>
        <tbody>
          <tr v-for="(s, i) in filteredSoftware.slice(0, 300)" :key="i">
            <td class="muted">{{ i + 1 }}</td>
            <td style="font-weight:500;max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ s.name }}</td>
            <td><code>{{ s.version || "—" }}</code></td>
            <td class="muted" style="max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ s.publisher || "—" }}</td>
            <td class="muted">{{ s.install_date || "—" }}</td>
            <td>{{ s.estimated_size_mb > 0 ? mbStr(s.estimated_size_mb) : "—" }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>

  <!-- Variables d'environnement -->
  <template v-else-if="tab === 'env'">
    <div class="diag-search">
      <Search :size="14" />
      <input v-model="envSearch" placeholder="Filtrer variable ou valeur..." class="diag-search-input" />
      <span class="muted">{{ filteredEnv.length }}/{{ envVars.length }}</span>
    </div>
    <div class="table-wrap">
      <table class="data-table">
        <thead><tr><th>Variable</th><th>Valeur</th><th>Portée</th></tr></thead>
        <tbody>
          <tr v-for="(e, i) in filteredEnv" :key="i">
            <td><code style="font-size:11px">{{ e.name }}</code></td>
            <td style="max-width:360px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-family:monospace;font-size:11px;color:var(--text-secondary)">
              {{ e.value }}
            </td>
            <td><NBadge :variant="e.var_type === 'Système' ? 'info' : 'default'" style="font-size:10px">{{ e.var_type }}</NBadge></td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
</template>
