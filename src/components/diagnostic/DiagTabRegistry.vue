<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import { Database, AlertTriangle, CheckCircle, Key } from "lucide-vue-next";

interface RegEntry {
  hive: string; key: string; name: string; value: string; suspicious: boolean;
}
interface RegistryPersistence {
  run_hklm: RegEntry[]; run_hkcu: RegEntry[]; run_once: RegEntry[];
  appinit_dlls: string[]; ifeo_debuggers: RegEntry[]; winlogon: RegEntry[];
  lsa_packages: string[]; browser_hijack: RegEntry[]; total_suspicious: number;
}

const data = ref<RegistryPersistence | null>(null);
const loading = ref(true);
const error = ref("");

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    data.value = await invoke<RegistryPersistence>("get_registry_persistence");
  } catch (e: any) { error.value = e?.toString() ?? "Erreur"; }
  finally { loading.value = false; }
});

const allRunEntries = computed(() => {
  if (!data.value) return [];
  return [...(data.value.run_hklm || []), ...(data.value.run_hkcu || []), ...(data.value.run_once || [])];
});
</script>

<template>
  <div class="diag-tab-content">
    <DiagBanner :icon="Database" title="Éditeur de Registre" desc="Lecture, recherche et modification du registre Windows" color="amber" />

    <div v-if="loading" class="diag-loading"><div class="diag-spinner"></div> Analyse registre...</div>
    <div v-else-if="error" style="color:var(--error)">⚠ {{ error }}</div>
    <div v-else-if="data" style="display:flex;flex-direction:column;gap:14px">

      <!-- Bilan -->
      <div class="diag-section" :style="{ borderLeft: `3px solid ${data.total_suspicious > 0 ? 'var(--warning)' : 'var(--success)'}` }">
        <p class="diag-section-label" style="margin:0 0 8px 0">Bilan Persistance Registre</p>
        <div v-if="data.total_suspicious === 0" style="color:var(--success);font-size:13px">
          <CheckCircle :size="14" style="display:inline;margin-right:4px" />Aucune entrée suspecte détectée
        </div>
        <div v-else style="color:var(--warning);font-size:13px">
          <AlertTriangle :size="14" style="display:inline;margin-right:4px" />{{ data.total_suspicious }} entrée(s) suspecte(s) détectée(s)
        </div>
        <div class="info-grid" style="margin-top:10px">
          <div class="info-row"><span>HKLM\Run</span><NBadge variant="neutral">{{ data.run_hklm.length }}</NBadge></div>
          <div class="info-row"><span>HKCU\Run</span><NBadge variant="neutral">{{ data.run_hkcu.length }}</NBadge></div>
          <div class="info-row"><span>RunOnce</span><NBadge variant="neutral">{{ data.run_once.length }}</NBadge></div>
          <div class="info-row"><span>AppInit DLLs</span>
            <NBadge :variant="data.appinit_dlls.length > 0 ? 'danger' : 'success'">{{ data.appinit_dlls.length }}</NBadge>
          </div>
          <div class="info-row"><span>IFEO Debuggers</span>
            <NBadge :variant="data.ifeo_debuggers.length > 0 ? 'danger' : 'success'">{{ data.ifeo_debuggers.length }}</NBadge>
          </div>
          <div class="info-row"><span>Winlogon hijack</span>
            <NBadge :variant="data.winlogon.length > 0 ? 'danger' : 'success'">{{ data.winlogon.length }}</NBadge>
          </div>
        </div>
      </div>

      <!-- Entrées Run -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">
          <Key :size="13" style="display:inline;margin-right:4px" />Clés Run / RunOnce ({{ allRunEntries.length }})
        </p>
        <div v-if="!allRunEntries.length" class="muted" style="font-size:13px">Aucune entrée.</div>
        <div v-for="(e, i) in allRunEntries" :key="i"
          style="padding:7px 0;border-bottom:1px solid var(--border)">
          <div style="display:flex;align-items:center;gap:8px;margin-bottom:3px">
            <component :is="e.suspicious ? AlertTriangle : CheckCircle" :size="12"
              :class="e.suspicious ? 'ic-warn' : 'ic-ok'" />
            <code style="font-size:11px;color:var(--accent)">{{ e.name }}</code>
            <NBadge :variant="e.hive === 'HKLM' ? 'info' : 'neutral'" style="font-size:9px">{{ e.hive }}</NBadge>
            <NBadge v-if="e.suspicious" variant="danger" style="font-size:9px">SUSPECT</NBadge>
          </div>
          <div class="muted" style="font-size:11px;padding-left:20px;word-break:break-all">{{ e.value }}</div>
        </div>
      </div>

      <!-- AppInit DLLs (très suspect) -->
      <div v-if="data.appinit_dlls.length" class="diag-section"
        style="border-left:3px solid var(--error)">
        <p class="diag-section-label" style="margin:0 0 8px 0">⚠ AppInit DLLs (malware classique)</p>
        <div v-for="(d, i) in data.appinit_dlls" :key="i"
          style="padding:4px 0;font-size:12px">
          <AlertTriangle :size="12" class="ic-warn" style="display:inline;margin-right:6px" />
          <code>{{ d }}</code>
        </div>
      </div>

      <!-- IFEO Debugger hijacks -->
      <div v-if="data.ifeo_debuggers.length" class="diag-section"
        style="border-left:3px solid var(--error)">
        <p class="diag-section-label" style="margin:0 0 8px 0">⚠ IFEO — Debugger Hijacks</p>
        <div v-for="(e, i) in data.ifeo_debuggers" :key="i" class="list-row">
          <AlertTriangle :size="12" class="ic-warn" style="flex-shrink:0" />
          <code class="list-name">{{ e.key.split('\\').pop() }}</code>
          <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;font-size:11px">→ {{ e.value }}</div>
        </div>
      </div>

      <!-- Winlogon hijacks -->
      <div v-if="data.winlogon.length" class="diag-section"
        style="border-left:3px solid var(--error)">
        <p class="diag-section-label" style="margin:0 0 8px 0">⚠ Winlogon — Modifications suspectes</p>
        <div v-for="(e, i) in data.winlogon" :key="i" class="list-row">
          <AlertTriangle :size="12" class="ic-warn" style="flex-shrink:0" />
          <code class="list-name">{{ e.name }}</code>
          <div class="muted" style="flex:1;font-size:11px">{{ e.value }}</div>
        </div>
      </div>

      <!-- LSA packages hors standard -->
      <div v-if="data.lsa_packages.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">LSA Packages non-standard</p>
        <div style="display:flex;gap:6px;flex-wrap:wrap">
          <code v-for="(p, i) in data.lsa_packages" :key="i"
            style="font-size:11px;background:var(--bg-secondary);padding:2px 6px;border-radius:4px;color:var(--warning)">{{ p }}</code>
        </div>
      </div>

      <!-- Proxy / hijack navigateur -->
      <div v-if="data.browser_hijack.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Proxy Internet détecté</p>
        <div v-for="(e, i) in data.browser_hijack" :key="i" class="list-row">
          <code class="list-name">{{ e.name }}</code>
          <div class="muted" style="flex:1;font-size:11px">{{ e.value }}</div>
        </div>
      </div>
    </div>
  </div>
</template>
