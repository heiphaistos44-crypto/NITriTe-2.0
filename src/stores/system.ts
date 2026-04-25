import { defineStore } from "pinia";
import { invoke } from "@/utils/invoke";
import { ref } from "vue";
import type { SystemInfo, MonitorData } from "@/types/system";

export const useSystemStore = defineStore("system", () => {
  const info = ref<SystemInfo | null>(null);
  const monitor = ref<MonitorData | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchSystemInfo() {
    loading.value = true;
    error.value = null;
    try {
      info.value = await invoke<SystemInfo>("get_system_info");
    } catch (e: any) {
      error.value = e?.toString() ?? "Erreur inconnue";
    } finally {
      loading.value = false;
    }
  }

  function updateMonitorData(data: MonitorData) {
    monitor.value = data;
  }

  return {
    info,
    monitor,
    loading,
    error,
    fetchSystemInfo,
    updateMonitorData,
  };
});
