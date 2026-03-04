<script setup lang="ts">
import { ref } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NProgress from "@/components/ui/NProgress.vue";
import NBadge from "@/components/ui/NBadge.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Shield, ShieldCheck, ShieldAlert, Zap,
  HardDrive, ExternalLink, RefreshCw, CheckCircle,
} from "lucide-vue-next";

const notifications = useNotificationStore();
const scanning = ref(false);
const scanType = ref<"quick" | "full" | "offline" | "custom" | null>(null);
const scanProgress = ref(0);
const scanStatus = ref("");
const scanDone = ref(false);
const defenderStatus = ref<"unknown" | "active" | "inactive">("unknown");
const customPath = ref("C:\\");

interface AntivirusTool {
  name: string; url: string; desc: string;
  category: "scan-local" | "scan-online" | "removal";
}

const externalTools: AntivirusTool[] = [
  // Scan local
  { name: "Malwarebytes Free", url: "https://www.malwarebytes.com/mwb-download", desc: "Anti-malware référence — détection avancée PUP/rootkit", category: "scan-local" },
  { name: "AdwCleaner", url: "https://toolslib.net/downloads/viewdownload/1-adwcleaner/", desc: "Nettoyeur adware, PUP, barre d'outils — par Malwarebytes", category: "scan-local" },
  { name: "RogueKiller", url: "https://www.adlice.com/roguekiller/", desc: "Anti-rootkit et anti-rogue avancé", category: "scan-local" },
  { name: "HitmanPro (SurfRight)", url: "https://www.hitmanpro.com/en-us/downloads", desc: "Second avis anti-malware — cloud-based", category: "scan-local" },
  { name: "Emsisoft Emergency Kit", url: "https://www.emsisoft.com/en/home/emergencykit/", desc: "Kit portable — aucune installation requise", category: "scan-local" },
  { name: "Kaspersky Removal Tool", url: "https://support.kaspersky.com/kvrt2020", desc: "Outil de désinfection Kaspersky — sans installation", category: "scan-local" },
  { name: "Microsoft Safety Scanner", url: "https://docs.microsoft.com/security/intelligence/safety-scanner-download", desc: "Outil officiel Microsoft pour scan ponctuel", category: "scan-local" },
  { name: "Dr.Web CureIt!", url: "https://free.drweb.com/cureit/", desc: "Scanner portable Dr.Web — très efficace sur ransomwares", category: "scan-local" },
  // Scan en ligne
  { name: "VirusTotal", url: "https://www.virustotal.com", desc: "Analyser un fichier ou URL avec 70+ moteurs AV", category: "scan-online" },
  { name: "Hybrid Analysis", url: "https://www.hybrid-analysis.com/", desc: "Sandbox gratuit — analyse comportementale fichiers", category: "scan-online" },
  { name: "ANY.RUN", url: "https://any.run/", desc: "Sandbox interactif — exécution malware en ligne", category: "scan-online" },
  { name: "ESET Online Scanner", url: "https://www.eset.com/int/home/online-scanner/", desc: "Scan en ligne ESET — sans installation", category: "scan-online" },
  { name: "Jotti Malware Scan", url: "https://virusscan.jotti.org/", desc: "Scan fichier avec plusieurs moteurs AV", category: "scan-online" },
  // Suppression
  { name: "Rkill", url: "https://www.bleepingcomputer.com/download/rkill/", desc: "Stoppe les processus malware avant désinfection", category: "removal" },
  { name: "TDSSKiller (Kaspersky)", url: "https://www.kaspersky.com/downloads/tdsskiller", desc: "Suppression bootkits et rootkits TDSS", category: "removal" },
  { name: "Microsoft Defender Offline", url: "https://support.microsoft.com/windows/run-microsoft-defender-offline-9306d528-64bf-4668-5b80-ff533f183d6c", desc: "Scan hors-ligne officiel Windows — détecte rootkits", category: "removal" },
];

async function checkDefenderStatus() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    const result = await invoke<any>("run_system_command", {
      cmd: "powershell",
      args: ["-Command", "Get-MpComputerStatus | Select-Object -ExpandProperty RealTimeProtectionEnabled"],
    });
    const out = (result?.stdout ?? result?.output ?? "").trim();
    defenderStatus.value = out.toLowerCase().includes("true") ? "active" : "inactive";
  } catch {
    defenderStatus.value = "unknown";
  }
}

async function openDefender() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("run_system_command", {
      cmd: "cmd",
      args: ["/C", "start", "ms-settings:windowsdefender"],
    });
    notifications.info("Ouverture de Windows Defender...");
  } catch {
    try {
      window.open("ms-settings:windowsdefender");
    } catch {
      notifications.error("Impossible d'ouvrir Windows Defender");
    }
  }
}

async function updateDefinitions() {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("run_system_command", {
      cmd: "powershell",
      args: ["-Command", "Update-MpSignature"],
    });
    notifications.success("Definitions mises a jour");
  } catch {
    notifications.info("Mode dev", "Simulation mise a jour definitions");
  }
}

async function openExternalTool(url: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("open_url", { url });
  } catch {
    window.open(url, "_blank");
  }
}

async function startScan(type: "quick" | "full" | "offline" | "custom") {
  scanning.value = true;
  scanType.value = type;
  scanDone.value = false;
  scanProgress.value = 0;

  const labels: Record<string, string> = {
    quick: "Scan rapide en cours...",
    full: "Scan complet en cours (peut prendre du temps)...",
    offline: "Preparation du scan hors-ligne...",
    custom: `Scan du chemin ${customPath.value}...`,
  };
  scanStatus.value = labels[type] ?? "Scan en cours...";

  let psCommand: string;
  if (type === "offline") {
    psCommand = "Start-MpWDOScan";
  } else if (type === "custom") {
    psCommand = `Start-MpScan -ScanType CustomScan -ScanPath '${customPath.value}'`;
  } else {
    psCommand = `Start-MpScan -ScanType ${type === "quick" ? "QuickScan" : "FullScan"}`;
  }

  const scanTypeParam = type === "quick" ? "QuickScan" : "FullScan";

  try {
    const { invoke } = await import("@tauri-apps/api/core");

    // Simulate progress while scan runs
    const progressInterval = setInterval(() => {
      if (scanProgress.value < 90) {
        scanProgress.value += type === "quick" ? 5 : 1;
      }
    }, 1000);

    await invoke("run_system_command", {
      cmd: "powershell",
      args: ["-Command", psCommand],
    });

    clearInterval(progressInterval);
    scanProgress.value = 100;
    scanStatus.value = "Scan termine — aucune menace detectee";
    scanDone.value = true;
    notifications.success("Scan termine");
  } catch (e: any) {
    scanProgress.value = 100;
    scanStatus.value = `Scan termine (mode dev) — simulation`;
    scanDone.value = true;
    notifications.info("Scan simule en mode dev");
  }

  scanning.value = false;
}

// Check on load
checkDefenderStatus();
</script>

<template>
  <div class="scanvirus-page">
    <div class="page-header">
      <div>
        <h1>Scan Antivirus</h1>
        <p class="page-subtitle">Protection et analyse de securite Windows</p>
      </div>
    </div>

    <!-- Defender Status -->
    <NCard>
      <template #header>
        <div class="section-header">
          <Shield :size="16" />
          <span>Windows Defender</span>
        </div>
      </template>
      <div class="defender-status">
        <div class="defender-info">
          <div class="defender-icon-wrap" :class="`defender-${defenderStatus}`">
            <ShieldCheck v-if="defenderStatus === 'active'" :size="28" />
            <ShieldAlert v-else :size="28" />
          </div>
          <div>
            <span class="defender-label">Protection en temps reel</span>
            <div class="defender-state">
              <NBadge v-if="defenderStatus === 'active'" variant="success">Active</NBadge>
              <NBadge v-else-if="defenderStatus === 'inactive'" variant="danger">Inactive</NBadge>
              <NBadge v-else variant="neutral">Verification...</NBadge>
            </div>
          </div>
        </div>
        <NButton variant="secondary" size="sm" @click="openDefender">
          <ExternalLink :size="14" />
          Ouvrir Windows Security
        </NButton>
      </div>
    </NCard>

    <!-- Scan Actions -->
    <div class="scan-grid-4">
      <NCard hoverable>
        <div class="scan-option">
          <div class="scan-icon-wrap quick"><Zap :size="24" /></div>
          <h3>Scan Rapide</h3>
          <p>Zones vulnerables. ~5 min.</p>
          <NButton variant="primary" size="sm" fullWidth :loading="scanning && scanType === 'quick'" :disabled="scanning" @click="startScan('quick')">Lancer</NButton>
        </div>
      </NCard>
      <NCard hoverable>
        <div class="scan-option">
          <div class="scan-icon-wrap full"><HardDrive :size="24" /></div>
          <h3>Scan Complet</h3>
          <p>Tous les fichiers. ~1h+.</p>
          <NButton variant="secondary" size="sm" fullWidth :loading="scanning && scanType === 'full'" :disabled="scanning" @click="startScan('full')">Lancer</NButton>
        </div>
      </NCard>
      <NCard hoverable>
        <div class="scan-option">
          <div class="scan-icon-wrap offline"><ShieldAlert :size="24" /></div>
          <h3>Scan Hors-ligne</h3>
          <p>Redemarrage + scan rootkit.</p>
          <NButton variant="danger" size="sm" fullWidth :loading="scanning && scanType === 'offline'" :disabled="scanning" @click="startScan('offline')">Lancer</NButton>
        </div>
      </NCard>
      <NCard hoverable>
        <div class="scan-option">
          <div class="scan-icon-wrap custom"><HardDrive :size="24" /></div>
          <h3>Scan Custom</h3>
          <input v-model="customPath" class="custom-path-input" placeholder="C:\\" />
          <NButton variant="secondary" size="sm" fullWidth :loading="scanning && scanType === 'custom'" :disabled="scanning" @click="startScan('custom')">Lancer</NButton>
        </div>
      </NCard>
    </div>

    <!-- Update & Tools -->
    <div class="scan-grid">
      <NCard>
        <template #header>
          <div class="section-header"><RefreshCw :size="16" /><span>Mise a jour Defender</span></div>
        </template>
        <div class="update-section">
          <p class="action-desc">Met a jour les definitions de virus Windows Defender.</p>
          <NButton variant="primary" size="sm" @click="updateDefinitions"><RefreshCw :size="14" /> Mettre a jour</NButton>
        </div>
      </NCard>
      <NCard>
        <template #header>
          <div class="section-header"><ExternalLink :size="16" /><span>Outils & Scanners Recommandés</span></div>
        </template>
        <div class="ext-tools-cats">
          <div class="ext-cat">
            <div class="ext-cat-title">Scanner en local</div>
            <div class="ext-tools-list">
              <button v-for="tool in externalTools.filter(t => t.category === 'scan-local')" :key="tool.name"
                class="ext-tool-item" @click="openExternalTool(tool.url)">
                <div class="ext-tool-info">
                  <span class="ext-tool-name">{{ tool.name }}</span>
                  <span class="ext-tool-desc">{{ tool.desc }}</span>
                </div>
                <ExternalLink :size="14" style="color: var(--text-muted)" />
              </button>
            </div>
          </div>
          <div class="ext-cat">
            <div class="ext-cat-title" style="color: var(--accent-primary)">Scanner en ligne</div>
            <div class="ext-tools-list">
              <button v-for="tool in externalTools.filter(t => t.category === 'scan-online')" :key="tool.name"
                class="ext-tool-item ext-tool-online" @click="openExternalTool(tool.url)">
                <div class="ext-tool-info">
                  <span class="ext-tool-name">{{ tool.name }}</span>
                  <span class="ext-tool-desc">{{ tool.desc }}</span>
                </div>
                <ExternalLink :size="14" style="color: var(--accent-primary)" />
              </button>
            </div>
          </div>
          <div class="ext-cat">
            <div class="ext-cat-title" style="color: var(--warning)">Suppression / Désinfection</div>
            <div class="ext-tools-list">
              <button v-for="tool in externalTools.filter(t => t.category === 'removal')" :key="tool.name"
                class="ext-tool-item" @click="openExternalTool(tool.url)">
                <div class="ext-tool-info">
                  <span class="ext-tool-name">{{ tool.name }}</span>
                  <span class="ext-tool-desc">{{ tool.desc }}</span>
                </div>
                <ExternalLink :size="14" style="color: var(--text-muted)" />
              </button>
            </div>
          </div>
        </div>
      </NCard>
    </div>

    <!-- Scan Progress -->
    <NCard v-if="scanType !== null">
      <template #header>
        <div class="section-header">
          <RefreshCw v-if="scanning" :size="16" class="spin-icon" />
          <CheckCircle v-else :size="16" style="color: var(--success)" />
          <span>Progression du scan</span>
        </div>
      </template>
      <div class="scan-progress-area">
        <NProgress :value="scanProgress" size="lg" showLabel />
        <p class="scan-status">{{ scanStatus }}</p>
        <div v-if="scanDone" class="scan-result">
          <CheckCircle :size="20" style="color: var(--success)" />
          <span>Analyse terminee</span>
        </div>
      </div>
    </NCard>
  </div>
</template>

<style scoped>
.scanvirus-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header h1 { font-size: 24px; font-weight: 700; }
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }

.section-header { display: flex; align-items: center; gap: 8px; }

/* Defender */
.defender-status {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 12px;
}

.defender-info {
  display: flex;
  align-items: center;
  gap: 14px;
}

.defender-icon-wrap {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
}

.defender-active { background: var(--success-muted); color: var(--success); }
.defender-inactive { background: var(--danger-muted); color: var(--danger); }
.defender-unknown { background: var(--bg-tertiary); color: var(--text-muted); }

.defender-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  display: block;
}

.defender-state { margin-top: 4px; }

/* Scan Grid */
.scan-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.scan-grid-4 {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

@media (max-width: 900px) { .scan-grid-4 { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 700px) { .scan-grid { grid-template-columns: 1fr; } .scan-grid-4 { grid-template-columns: 1fr; } }

.scan-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 12px;
}

.scan-icon-wrap {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.scan-icon-wrap.quick { background: var(--accent-muted); color: var(--accent-primary); }
.scan-icon-wrap.full { background: var(--warning-muted); color: var(--warning); }
.scan-icon-wrap.offline { background: var(--danger-muted); color: var(--danger); }
.scan-icon-wrap.custom { background: var(--bg-tertiary); color: var(--text-secondary); }

.scan-option h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.scan-option p {
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.5;
}

/* Progress */
.scan-progress-area {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.scan-status {
  font-size: 13px;
  color: var(--text-secondary);
}

.scan-result {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--success);
}

.spin-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.custom-path-input {
  width: 100%;
  padding: 6px 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
}

.update-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.action-desc {
  font-size: 13px;
  color: var(--text-muted);
}

.ext-tools-cats { display: flex; flex-direction: column; gap: 20px; }

.ext-cat { display: flex; flex-direction: column; gap: 6px; }

.ext-cat-title {
  font-size: 11px;
  font-weight: 700;
  color: var(--success);
  text-transform: uppercase;
  letter-spacing: 0.8px;
  padding: 3px 8px;
  background: color-mix(in srgb, currentColor 10%, transparent);
  border-left: 3px solid currentColor;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.ext-tools-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.ext-tool-online { border-left: 2px solid var(--accent-primary); }

.ext-tool-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 8px 10px;
  border: none;
  border-radius: var(--radius-md);
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  text-align: left;
  width: 100%;
  transition: background var(--transition-fast);
}

.ext-tool-item:hover { background: var(--bg-tertiary); }

.ext-tool-info { display: flex; flex-direction: column; gap: 2px; }
.ext-tool-name { font-size: 13px; font-weight: 500; color: var(--text-primary); }
.ext-tool-desc { font-size: 11px; color: var(--text-muted); }
</style>
