<script setup lang="ts">
import { ref, computed } from "vue";
import { Search, RefreshCw } from "lucide-vue-next";
import NBadge from "@/components/ui/NBadge.vue";
import NProgress from "@/components/ui/NProgress.vue";

const props = defineProps<{
  tab: string;
  processes: any[];
  services: any[];
  startupPrograms: any[];
  tasks: any[];
  onRefresh: () => void;
}>();

const procSearch = ref("");
const procSort = ref<"cpu"|"mem"|"name">("cpu");
const svcSearch = ref("");
const svcFilter = ref("all");
const taskSearch = ref("");

const filteredProcs = computed(() => {
  let list = [...props.processes];
  const q = procSearch.value.toLowerCase();
  if (q) list = list.filter(p => p.name.toLowerCase().includes(q) || (p.path || "").toLowerCase().includes(q));
  if (procSort.value === "cpu") list.sort((a, b) => b.cpu_percent - a.cpu_percent);
  else if (procSort.value === "mem") list.sort((a, b) => b.memory_mb - a.memory_mb);
  else list.sort((a, b) => a.name.localeCompare(b.name));
  return list;
});

const filteredSvcs = computed(() => {
  let list = [...props.services];
  const q = svcSearch.value.toLowerCase();
  if (q) list = list.filter(s => s.display_name.toLowerCase().includes(q) || s.name.toLowerCase().includes(q));
  if (svcFilter.value !== "all") list = list.filter(s => s.state === svcFilter.value);
  return list;
});

const filteredTasks = computed(() => {
  const q = taskSearch.value.toLowerCase();
  if (!q) return props.tasks;
  return props.tasks.filter(t => t.name.toLowerCase().includes(q) || t.path.toLowerCase().includes(q));
});

const totalCpuUsage = computed(() => {
  const sum = props.processes.reduce((a, p) => a + p.cpu_percent, 0);
  return Math.min(100, Math.round(sum * 10) / 10);
});
const totalRamMb = computed(() => props.processes.reduce((a, p) => a + p.memory_mb, 0));
const runningCount = computed(() => props.services.filter(s => s.state === "Running").length);
</script>

<template>
  <!-- Processus actifs -->
  <template v-if="tab === 'processes'">
    <div style="display:flex;gap:12px;flex-wrap:wrap;margin-bottom:12px">
      <div class="card-block" style="flex:1;min-width:140px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Processus actifs</p>
        <span style="font-size:24px;font-weight:700;color:var(--accent)">{{ processes.length }}</span>
      </div>
      <div class="card-block" style="flex:1;min-width:140px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">CPU total</p>
        <span style="font-size:24px;font-weight:700" :class="totalCpuUsage > 80 ? 'text-err' : totalCpuUsage > 50 ? 'text-warn' : 'ic-ok'">{{ totalCpuUsage }}%</span>
      </div>
      <div class="card-block" style="flex:1;min-width:140px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">RAM allouée</p>
        <span style="font-size:24px;font-weight:700">{{ (totalRamMb / 1024).toFixed(1) }} GB</span>
      </div>
    </div>

    <div style="display:flex;gap:8px;align-items:center;margin-bottom:12px;flex-wrap:wrap">
      <div class="diag-search" style="flex:1;margin:0">
        <Search :size="14" />
        <input v-model="procSearch" placeholder="Filtrer par nom ou chemin..." class="diag-search-input" />
        <span class="muted">{{ filteredProcs.length }}/{{ processes.length }}</span>
      </div>
      <div style="display:flex;gap:4px">
        <NBadge v-for="[k,v] in [['cpu','CPU'],['mem','RAM'],['name','Nom']]" :key="k"
          :variant="procSort === k ? 'info' : 'default'" style="cursor:pointer" @click="procSort = k as any">{{ v }}</NBadge>
      </div>
      <button class="diag-search" style="padding:6px 10px;cursor:pointer;border:none" @click="onRefresh">
        <RefreshCw :size="13" style="color:var(--text-secondary)" />
      </button>
    </div>

    <div class="table-wrap">
      <table class="data-table">
        <thead>
          <tr><th>PID</th><th>Nom</th><th>CPU %</th><th>RAM (MB)</th><th>Virt. (MB)</th><th>État</th><th>PPID</th></tr>
        </thead>
        <tbody>
          <tr v-for="p in filteredProcs.slice(0, 300)" :key="p.pid">
            <td class="muted">{{ p.pid }}</td>
            <td style="font-weight:500;max-width:160px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap" :title="p.path">{{ p.name }}</td>
            <td>
              <div style="display:flex;align-items:center;gap:6px;min-width:80px">
                <span style="min-width:36px" :class="p.cpu_percent > 20 ? 'text-warn' : ''">{{ p.cpu_percent.toFixed(1) }}%</span>
                <div style="flex:1;min-width:40px"><NProgress :value="p.cpu_percent" size="sm" :variant="p.cpu_percent > 20 ? 'warning' : 'default'" /></div>
              </div>
            </td>
            <td :class="p.memory_mb > 500 ? 'text-warn' : ''">{{ p.memory_mb.toFixed(0) }}</td>
            <td class="muted">{{ p.virtual_memory_mb.toFixed(0) }}</td>
            <td><NBadge :variant="p.status === 'Running' ? 'success' : p.status === 'Sleeping' ? 'default' : 'warning'" style="font-size:10px">{{ p.status }}</NBadge></td>
            <td class="muted">{{ p.parent_pid || "—" }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>

  <!-- Services Windows -->
  <template v-else-if="tab === 'services'">
    <div style="display:flex;gap:12px;flex-wrap:wrap;margin-bottom:12px">
      <div class="card-block" style="flex:1;min-width:120px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Total services</p>
        <span style="font-size:24px;font-weight:700">{{ services.length }}</span>
      </div>
      <div class="card-block" style="flex:1;min-width:120px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">En cours</p>
        <span style="font-size:24px;font-weight:700;color:var(--success)">{{ runningCount }}</span>
      </div>
      <div class="card-block" style="flex:1;min-width:120px;margin-bottom:0">
        <p class="diag-section-label" style="margin:0 0 4px 0">Arrêtés</p>
        <span style="font-size:24px;font-weight:700;color:var(--text-secondary)">{{ services.length - runningCount }}</span>
      </div>
    </div>

    <div style="display:flex;gap:8px;margin-bottom:12px;flex-wrap:wrap">
      <div class="diag-search" style="flex:1;margin:0">
        <Search :size="14" />
        <input v-model="svcSearch" placeholder="Rechercher un service..." class="diag-search-input" />
        <span class="muted">{{ filteredSvcs.length }}</span>
      </div>
      <div style="display:flex;gap:4px;flex-wrap:wrap">
        <NBadge v-for="[k,v] in [['all','Tous'],['Running','En cours'],['Stopped','Arrêtés']]" :key="k"
          :variant="svcFilter === k ? 'info' : 'default'" style="cursor:pointer" @click="svcFilter = k">{{ v }}</NBadge>
      </div>
    </div>

    <div class="table-wrap">
      <table class="data-table">
        <thead>
          <tr><th>Nom</th><th>Nom affiché</th><th>État</th><th>Démarrage</th><th>Compte</th><th>PID</th></tr>
        </thead>
        <tbody>
          <tr v-for="(s, i) in filteredSvcs.slice(0, 400)" :key="i">
            <td><code style="font-size:11px">{{ s.name }}</code></td>
            <td style="font-weight:500">{{ s.display_name }}</td>
            <td>
              <NBadge
                :variant="s.state === 'Running' ? 'success' : s.state === 'Stopped' ? 'default' : 'warning'"
                style="font-size:10px">{{ s.state }}</NBadge>
            </td>
            <td>
              <NBadge
                :variant="s.start_mode === 'Auto' ? 'info' : s.start_mode === 'Disabled' ? 'danger' : 'default'"
                style="font-size:10px">{{ s.start_mode }}</NBadge>
            </td>
            <td class="muted" style="font-size:11px;max-width:100px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ s.account || "—" }}</td>
            <td class="muted">{{ s.process_id > 0 ? s.process_id : "—" }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>

  <!-- Démarrage -->
  <template v-else-if="tab === 'startup'">
    <p class="diag-section-label">Programmes au démarrage — {{ startupPrograms.length }}</p>
    <div v-if="!startupPrograms.length" class="diag-empty">Aucun programme au démarrage trouvé</div>
    <div class="table-wrap">
      <table class="data-table">
        <thead><tr><th>Nom</th><th>Catégorie</th><th>Source</th><th>Portée</th><th>Commande</th></tr></thead>
        <tbody>
          <tr v-for="(s, i) in startupPrograms" :key="i">
            <td style="font-weight:500">{{ s.name }}</td>
            <td>
              <NBadge
                :variant="s.category === 'Microsoft / Windows' ? 'info' : s.category === 'Sécurité' ? 'success' : s.category === 'Tiers' ? 'warning' : 'default'"
                style="font-size:10px">{{ s.category }}</NBadge>
            </td>
            <td class="muted" style="font-size:11px">{{ s.location }}</td>
            <td><NBadge :variant="s.user === 'Tous les utilisateurs' ? 'neutral' : 'default'" style="font-size:10px">{{ s.user }}</NBadge></td>
            <td class="muted" style="font-size:10px;max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap" :title="s.command">{{ s.command }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>

  <!-- Tâches planifiées -->
  <template v-else-if="tab === 'tasks'">
    <div class="diag-search">
      <Search :size="14" />
      <input v-model="taskSearch" placeholder="Rechercher une tâche..." class="diag-search-input" />
      <span class="muted">{{ filteredTasks.length }}/{{ tasks.length }}</span>
    </div>
    <div class="table-wrap">
      <table class="data-table">
        <thead><tr><th>Nom</th><th>Chemin</th><th>État</th><th>Déclencheur</th><th>Dernier exec.</th><th>Prochain exec.</th><th>Résultat</th></tr></thead>
        <tbody>
          <tr v-for="(t, i) in filteredTasks.slice(0, 300)" :key="i">
            <td style="font-weight:500;max-width:140px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ t.name }}</td>
            <td class="muted" style="font-size:10px;max-width:100px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ t.path }}</td>
            <td>
              <NBadge
                :variant="t.state === 'Ready' ? 'success' : t.state === 'Disabled' ? 'default' : t.state === 'Running' ? 'info' : 'warning'"
                style="font-size:10px">{{ t.state }}</NBadge>
            </td>
            <td class="muted" style="font-size:11px">{{ t.trigger || "—" }}</td>
            <td class="muted" style="font-size:11px">{{ t.last_run_time }}</td>
            <td class="muted" style="font-size:11px">{{ t.next_run_time }}</td>
            <td>
              <NBadge :variant="t.last_task_result === 0 ? 'success' : 'danger'" style="font-size:10px">
                {{ t.last_task_result === 0 ? 'OK' : '0x' + t.last_task_result.toString(16).toUpperCase() }}
              </NBadge>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>
</template>
