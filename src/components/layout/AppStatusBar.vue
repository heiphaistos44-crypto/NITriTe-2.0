<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { Cpu, MemoryStick, HardDrive, Wifi } from "lucide-vue-next";

const cpuUsage = ref(0);
const ramUsage = ref(0);
const diskUsage = ref(0);
const networkDown = ref(0);

let monitorInterval: ReturnType<typeof setInterval> | null = null;

onMounted(async () => {
  try {
    const { listen } = await import("@tauri-apps/api/event");
    await listen("system-monitor", (event: any) => {
      const data = event.payload;
      cpuUsage.value = data.cpu_percent ?? 0;
      ramUsage.value = data.ram_percent ?? 0;
      diskUsage.value = data.disk_percent ?? 0;
      networkDown.value = data.network_down_kbs ?? 0;
    });
  } catch {
    // Mode dev sans Tauri : simulation
    monitorInterval = setInterval(() => {
      cpuUsage.value = Math.round(10 + Math.random() * 40);
      ramUsage.value = Math.round(30 + Math.random() * 30);
      diskUsage.value = Math.round(40 + Math.random() * 20);
      networkDown.value = Math.round(Math.random() * 500);
    }, 2000);
  }
});

onUnmounted(() => {
  if (monitorInterval) clearInterval(monitorInterval);
});

function statusColor(value: number): string {
  if (value > 90) return "var(--danger)";
  if (value > 75) return "var(--warning)";
  return "var(--success)";
}
</script>

<template>
  <footer class="status-bar">
    <div class="status-item">
      <Cpu :size="12" />
      <span>CPU</span>
      <span class="status-value" :style="{ color: statusColor(cpuUsage) }">{{ cpuUsage }}%</span>
    </div>
    <div class="status-item">
      <MemoryStick :size="12" />
      <span>RAM</span>
      <span class="status-value" :style="{ color: statusColor(ramUsage) }">{{ ramUsage }}%</span>
    </div>
    <div class="status-item">
      <HardDrive :size="12" />
      <span>Disque</span>
      <span class="status-value" :style="{ color: statusColor(diskUsage) }">{{ diskUsage }}%</span>
    </div>
    <div class="status-item">
      <Wifi :size="12" />
      <span class="status-value">{{ networkDown }} KB/s</span>
    </div>
    <div class="status-spacer"></div>
    <div class="status-item">
      <span class="status-version">NiTriTe v26.0</span>
    </div>
  </footer>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 4px 16px;
  border-top: 1px solid var(--border);
  background: var(--bg-secondary);
  font-size: 11px;
  color: var(--text-muted);
  min-height: 28px;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 4px;
}

.status-value {
  font-weight: 500;
  font-family: "JetBrains Mono", monospace;
  font-size: 11px;
}

.status-spacer {
  flex: 1;
}

.status-version {
  font-size: 10px;
  opacity: 0.6;
}
</style>
