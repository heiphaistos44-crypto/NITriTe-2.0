/**
 * [7] DiagnosticStore — Pinia store pour les données système partagées.
 * Évite les fetches redondants entre DiagnosticPage, Dashboard, Monitoring, etc.
 * Utilise cachedInvoke pour la déduplication automatique.
 */
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { cachedInvoke, refreshCached } from "@/composables/useCachedInvoke";
import type {
  SysInfo, BatteryDetailed, GpuDetailed, NetworkAdapter,
  StoragePhysical, InstalledSoftware,
} from "@/types/diagnostic";

export const useDiagnosticStore = defineStore("diagnostic", () => {
  // ── État ──────────────────────────────────────────────────────────────────
  const sysInfo         = ref<SysInfo | null>(null);
  const batteries       = ref<BatteryDetailed[]>([]);
  const gpuList         = ref<GpuDetailed[]>([]);
  const networkAdapters = ref<NetworkAdapter[]>([]);
  const storageList     = ref<StoragePhysical[]>([]);
  const softwareList    = ref<InstalledSoftware[]>([]);

  const loadingKeys = ref<Set<string>>(new Set());
  const errors      = ref<Record<string, string>>({});

  // ── Computed ──────────────────────────────────────────────────────────────
  const isLoading = computed(() => loadingKeys.value.size > 0);
  const hasBattery = computed(() => batteries.value.length > 0);
  const primaryGpu = computed(() => gpuList.value[0] ?? null);

  // ── Actions ───────────────────────────────────────────────────────────────
  async function fetchSysInfo(force = false) {
    if (!force && sysInfo.value) return sysInfo.value;
    loadingKeys.value.add("sysInfo");
    try {
      const fn = force ? refreshCached : cachedInvoke;
      sysInfo.value = await fn<SysInfo>("get_system_info");
      delete errors.value["sysInfo"];
    } catch (e: unknown) {
      errors.value["sysInfo"] = String(e);
    } finally {
      loadingKeys.value.delete("sysInfo");
    }
    return sysInfo.value;
  }

  async function fetchBatteries(force = false) {
    if (!force && batteries.value.length) return batteries.value;
    loadingKeys.value.add("batteries");
    try {
      const fn = force ? refreshCached : cachedInvoke;
      batteries.value = await fn<BatteryDetailed[]>("get_battery_detailed");
      delete errors.value["batteries"];
    } catch (e: unknown) {
      errors.value["batteries"] = String(e);
    } finally {
      loadingKeys.value.delete("batteries");
    }
    return batteries.value;
  }

  async function fetchGpuList(force = false) {
    if (!force && gpuList.value.length) return gpuList.value;
    loadingKeys.value.add("gpuList");
    try {
      const fn = force ? refreshCached : cachedInvoke;
      gpuList.value = await fn<GpuDetailed[]>("get_gpu_detailed");
      delete errors.value["gpuList"];
    } catch (e: unknown) {
      errors.value["gpuList"] = String(e);
    } finally {
      loadingKeys.value.delete("gpuList");
    }
    return gpuList.value;
  }

  async function fetchNetworkAdapters(force = false) {
    if (!force && networkAdapters.value.length) return networkAdapters.value;
    loadingKeys.value.add("network");
    try {
      const fn = force ? refreshCached : cachedInvoke;
      networkAdapters.value = await fn<NetworkAdapter[]>("get_network_adapters_detailed");
      delete errors.value["network"];
    } catch (e: unknown) {
      errors.value["network"] = String(e);
    } finally {
      loadingKeys.value.delete("network");
    }
    return networkAdapters.value;
  }

  async function fetchStorageList(force = false) {
    if (!force && storageList.value.length) return storageList.value;
    loadingKeys.value.add("storage");
    try {
      const fn = force ? refreshCached : cachedInvoke;
      storageList.value = await fn<StoragePhysical[]>("get_storage_physical_info");
      delete errors.value["storage"];
    } catch (e: unknown) {
      errors.value["storage"] = String(e);
    } finally {
      loadingKeys.value.delete("storage");
    }
    return storageList.value;
  }

  async function fetchSoftwareList(force = false) {
    if (!force && softwareList.value.length) return softwareList.value;
    loadingKeys.value.add("software");
    try {
      const fn = force ? refreshCached : cachedInvoke;
      softwareList.value = await fn<InstalledSoftware[]>("get_installed_software");
      delete errors.value["software"];
    } catch (e: unknown) {
      errors.value["software"] = String(e);
    } finally {
      loadingKeys.value.delete("software");
    }
    return softwareList.value;
  }

  /** Précharge toutes les données critiques en parallèle (appelé au splash) */
  async function prefetchAll() {
    await Promise.allSettled([
      fetchSysInfo(),
      fetchBatteries(),
      fetchGpuList(),
      fetchNetworkAdapters(),
      fetchStorageList(),
    ]);
  }

  /** Recharge tout (après un changement hardware ou forçage utilisateur) */
  async function refreshAll() {
    await Promise.allSettled([
      fetchSysInfo(true),
      fetchBatteries(true),
      fetchGpuList(true),
      fetchNetworkAdapters(true),
      fetchStorageList(true),
    ]);
  }

  return {
    // State
    sysInfo, batteries, gpuList, networkAdapters, storageList, softwareList,
    loadingKeys, errors,
    // Computed
    isLoading, hasBattery, primaryGpu,
    // Actions
    fetchSysInfo, fetchBatteries, fetchGpuList, fetchNetworkAdapters,
    fetchStorageList, fetchSoftwareList, prefetchAll, refreshAll,
  };
});
