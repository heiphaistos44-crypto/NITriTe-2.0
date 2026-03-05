<script setup lang="ts">
import { ref, watch } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NTabs from "@/components/ui/NTabs.vue";
import NButton from "@/components/ui/NButton.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import DiagnosticToolsGrid from "@/components/shared/DiagnosticToolsGrid.vue";
import DiagTabSystem from "@/components/diagnostic/DiagTabSystem.vue";
import DiagTabCPU from "@/components/diagnostic/DiagTabCPU.vue";
import DiagTabGPU from "@/components/diagnostic/DiagTabGPU.vue";
import DiagTabRAM from "@/components/diagnostic/DiagTabRAM.vue";
import DiagTabStorage from "@/components/diagnostic/DiagTabStorage.vue";
import DiagTabNetwork from "@/components/diagnostic/DiagTabNetwork.vue";
import DiagTabDevices from "@/components/diagnostic/DiagTabDevices.vue";
import DiagTabSoftware from "@/components/diagnostic/DiagTabSoftware.vue";
import DiagTabProcesses from "@/components/diagnostic/DiagTabProcesses.vue";
import DiagTabSecurity from "@/components/diagnostic/DiagTabSecurity.vue";
import DiagTabUpdates from "@/components/diagnostic/DiagTabUpdates.vue";
import DiagTabActivation from "@/components/diagnostic/DiagTabActivation.vue";
import DiagTabFolders from "@/components/diagnostic/DiagTabFolders.vue";
import DiagTabAccounts from "@/components/diagnostic/DiagTabAccounts.vue";
import DiagTabFirewall from "@/components/diagnostic/DiagTabFirewall.vue";
import DiagTabShares from "@/components/diagnostic/DiagTabShares.vue";
import DiagTabRegistry from "@/components/diagnostic/DiagTabRegistry.vue";
import DiagTabHistory from "@/components/diagnostic/DiagTabHistory.vue";
import DiagTabSysDrivers from "@/components/diagnostic/DiagTabSysDrivers.vue";
import DiagTabDriverUpdater from "@/components/diagnostic/DiagTabDriverUpdater.vue";
import DiagTabCertificates from "@/components/diagnostic/DiagTabCertificates.vue";
import DiagTabPerf from "@/components/diagnostic/DiagTabPerf.vue";
import DiagTabNetTools from "@/components/diagnostic/DiagTabNetTools.vue";
import DiagTabRepair from "@/components/diagnostic/DiagTabRepair.vue";
import DiagTabScan from "@/components/diagnostic/DiagTabScan.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Monitor, Cpu, MemoryStick, HardDrive, Globe, Headphones,
  Usb, Battery, Package, Play, Zap, Printer, Key,
  RefreshCw, ScanLine, FileJson, FileText, FileCode, FolderOpen,
  CircuitBoard, Wifi, Server, Shield, Activity, FolderTree, Layers,
  Users, Clock, FileDown, History, Lock, Wrench,
} from "lucide-vue-next";

const notify = useNotificationStore();

// ============= Types =============
interface SysInfo { os: any; cpu: any; ram: any; gpus: any[]; disks: any[]; motherboard: any; }
interface BiosInfo { manufacturer: string; version: string; release_date: string; serial_number: string; smbios_version: string; }
interface MoboDetailed { manufacturer: string; product: string; serial_number: string; version: string; status: string; }
interface GpuDetailed { name: string; adapter_ram_mb: number; driver_version: string; driver_date: string; video_processor: string; video_mode: string; current_resolution: string; current_refresh_rate: number; status: string; pnp_device_id: string; adapter_dac_type: string; }
interface RamSlot { bank_label: string; device_locator: string; manufacturer: string; capacity_gb: number; speed_mhz: number; configured_speed_mhz: number; memory_type: string; form_factor: string; serial_number: string; part_number: string; data_width: number; }
interface RamDetailed { total_slots: number; used_slots: number; total_capacity_gb: number; slots: RamSlot[]; }
interface StoragePhysical { model: string; serial_number: string; firmware_revision: string; size_bytes: number; size_gb: number; interface_type: string; media_type: string; status: string; pnp_device_id: string; partitions: number; }
interface NetworkAdapter { name: string; description: string; mac_address: string; ip_addresses: string[]; subnet_masks: string[]; default_gateway: string[]; dns_servers: string[]; dhcp_enabled: boolean; dhcp_server: string; speed_mbps: number; net_connection_id: string; is_physical: boolean; status: string; }
interface CpuCache { l1_instruction_kb: number; l1_data_kb: number; l2_kb: number; l3_kb: number; l4_kb: number; }
interface MonitorDetail { name: string; screen_width: number; screen_height: number; pixels_per_inch: number; manufacturer: string; availability: string; }
interface AudioDevice { name: string; manufacturer: string; status: string; device_id: string; }
interface UsbDevice { name: string; device_id: string; manufacturer: string; status: string; pnp_class: string; }
interface BatteryDetailed { name: string; status: string; estimated_charge_remaining: number; estimated_run_time: string; design_capacity: number; full_charge_capacity: number; battery_health_percent: number; chemistry: string; cycle_count: number; }
interface InstalledSoftware { name: string; version: string; publisher: string; install_date: string; install_location: string; estimated_size_mb: number; }
interface StartupProgram { name: string; command: string; location: string; user: string; category: string; }
interface PowerPlan { name: string; is_active: boolean; guid: string; }
interface PrinterDetail { name: string; driver_name: string; port_name: string; is_default: boolean; is_network: boolean; status: string; shared: boolean; }
interface EnvVar { name: string; value: string; var_type: string; }
interface WinLicense { product_name: string; activation_status: string; partial_product_key: string; full_product_key: string; license_status: string; license_family: string; office_name: string; office_status: string; office_key: string; office_full_key: string; }
interface InstalledUpdate { title: string; hotfix_id: string; description: string; installed_on: string; installed_by: string; }
interface ScanResult {
  bios_ok: boolean; bios_info: string | null;
  battery_present: boolean; battery_health: number; battery_cycles: number;
  suspicious_processes: { name: string; pid: number; path: string; reason: string }[];
  disk_usage: { drive: string; total_gb: number; free_gb: number; used_percent: number }[];
  winget_upgradable: string[]; choco_upgradable: string[];
  dism_status: string; sfc_status: string; scan_errors: string[];
  uptime_hours: number; cpu_name: string; cpu_cores: number; cpu_usage_percent: number;
  ram_total_gb: number; ram_used_gb: number; ram_usage_percent: number;
  windows_version: string; windows_activation: string;
  firewall_enabled: boolean; defender_enabled: boolean;
  startup_count: number; pending_reboot: boolean;
  recent_errors: { time: string; source: string; message: string; level: string }[];
  network_ok: boolean; open_ports: number[];
  antivirus_installed: string; defender_definition_age_days: number;
  last_bsod: string; last_update_days: number; temp_folder_size_mb: number;
  suspicious_services: { name: string; display_name: string; state: string; path: string }[];
  autorun_entries: { name: string; path: string; location: string }[];
  virtual_memory_total_mb: number; virtual_memory_available_mb: number;
  gpu_name: string; gpu_vram_mb: number; screen_resolution: string;
  power_plan: string; installed_software_count: number;
  services_running: number; services_stopped: number;
  network_adapters_summary: string; cpu_temperature: string;
  // Supplément
  windows_product_key: string; office_product_key: string; office_name: string;
  bitlocker_volumes: { drive: string; protection_status: string; encryption_percent: number; recovery_password: string; protectors: string[] }[];
  motherboard: string; ram_detail: string; cpu_threads: number; cpu_frequency_ghz: number;
  storage_items: { model: string; size_gb: number; media_type: string; interface_type: string; health: string }[];
  monitors_detail: string;
  // Extra
  tpm_present: boolean; tpm_enabled: boolean; tpm_version: string;
  secure_boot: boolean; uac_level: string; rdp_enabled: boolean;
  smbv1_enabled: boolean; wmi_subscriptions: number;
  local_admins: string[]; guest_enabled: boolean;
  system_manufacturer: string; system_model: string; system_serial: string;
  bios_manufacturer: string; bios_version: string; bios_date: string;
  license_type: string; last_restore_point: string; pending_updates_cached: number;
  top_cpu: { name: string; pid: number; value: number }[];
  top_ram: { name: string; pid: number; value: number }[];
  susp_tasks_count: number;
  susp_tasks: { name: string; path: string; exec: string }[];
}

// ============= Tabs =============
const TABS = [
  { id: "os",          label: "Système",      icon: Monitor },
  { id: "bios",        label: "BIOS",         icon: CircuitBoard },
  { id: "mobo",        label: "Carte Mère",   icon: CircuitBoard },
  { id: "cpu",         label: "Processeur",   icon: Cpu },
  { id: "gpu",         label: "GPU",          icon: Monitor },
  { id: "ram",         label: "RAM",          icon: MemoryStick },
  { id: "disks",       label: "Disques",      icon: HardDrive },
  { id: "volumes",     label: "Volumes",      icon: Layers },
  { id: "network",     label: "Réseau",       icon: Wifi },
  { id: "connections", label: "Connexions",   icon: Activity },
  { id: "monitors",    label: "Écrans",       icon: Monitor },
  { id: "audio",       label: "Audio",        icon: Headphones },
  { id: "usb",         label: "USB",          icon: Usb },
  { id: "battery",     label: "Batterie",     icon: Battery },
  { id: "power",       label: "Énergie",      icon: Zap },
  { id: "printers",    label: "Imprimantes",  icon: Printer },
  { id: "software",    label: "Logiciels",    icon: Package },
  { id: "env",         label: "Variables",    icon: Server },
  { id: "processes",   label: "Processus",    icon: Activity },
  { id: "services",    label: "Services",     icon: Server },
  { id: "startup",     label: "Démarrage",    icon: Play },
  { id: "tasks",       label: "Tâches",       icon: RefreshCw },
  { id: "security",    label: "Sécurité",     icon: Shield },
  { id: "license",     label: "Licence",      icon: Key },
  { id: "updates",     label: "MAJ Windows",  icon: RefreshCw },
  { id: "folders",     label: "Dossiers",     icon: FolderTree },
  { id: "activation",  label: "Activation",   icon: Key },
  { id: "comptes",     label: "Comptes",      icon: Users },
  { id: "parefeu",     label: "Pare-feu",     icon: Shield },
  { id: "partages",    label: "Partages",     icon: FolderOpen },
  { id: "registre",    label: "Registre",     icon: Key },
  { id: "historique",  label: "Historique",   icon: History },
  { id: "pilotes",     label: "Pilotes",      icon: HardDrive },
  { id: "driver-update", label: "MAJ Pilotes",  icon: HardDrive },
  { id: "certificats", label: "Certificats",  icon: Lock },
  { id: "performances",label: "Performances", icon: Activity },
  { id: "outils-reseau",label: "Outils Réseau",icon: Wifi },
  { id: "reparation",  label: "Réparation",   icon: Wrench },
  { id: "tools",       label: "Outils",       icon: Globe },
  { id: "scan",        label: "Scan Total",   icon: ScanLine },
];

const activeTab = ref("os");
const loadedTabs = ref<Set<string>>(new Set());
const loadingTab = ref<string | null>(null);
const tabError = ref<Record<string, string>>({});

// ============= Data refs =============
const sysInfo = ref<SysInfo | null>(null);
const biosInfo = ref<BiosInfo | null>(null);
const moboInfo = ref<MoboDetailed | null>(null);
const cpuCache = ref<CpuCache | null>(null);
const cpuExtended = ref<any>(null);
const osExtended = ref<any>(null);
const gpuList = ref<GpuDetailed[]>([]);
const ramData = ref<RamDetailed | null>(null);
const storageList = ref<StoragePhysical[]>([]);
const volumes = ref<any[]>([]);
const networkAdapters = ref<NetworkAdapter[]>([]);
const connections = ref<any[]>([]);
const wifiInfo = ref<any>(null);
const monitors = ref<MonitorDetail[]>([]);
const audioDevices = ref<AudioDevice[]>([]);
const usbDevices = ref<UsbDevice[]>([]);
const batteries = ref<BatteryDetailed[]>([]);
const powerPlans = ref<PowerPlan[]>([]);
const printers = ref<PrinterDetail[]>([]);
const softwareList = ref<InstalledSoftware[]>([]);
const envVars = ref<EnvVar[]>([]);
const processes = ref<any[]>([]);
const services = ref<any[]>([]);
const startupPrograms = ref<StartupProgram[]>([]);
const scheduledTasks = ref<any[]>([]);
const securityInfo = ref<any>(null);
const licenseInfo = ref<WinLicense | null>(null);
const updatesHistory = ref<InstalledUpdate[]>([]);
const folders = ref<any[]>([]);
const smartData = ref<any[]>([]);
const scanning = ref(false);
const scanProgress = ref(0);
const scanStep = ref("");
const scanResult = ref<ScanResult | null>(null);
const scanProblems = ref<string[]>([]);

// ============= Loaders =============
async function loadTab(tab: string, force = false) {
  if (!force && loadedTabs.value.has(tab)) return;
  loadedTabs.value.add(tab);
  loadingTab.value = tab;
  tabError.value[tab] = "";
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    switch (tab) {
      case "os":
        if (!sysInfo.value) sysInfo.value = await invoke("get_system_info");
        if (!osExtended.value) osExtended.value = await invoke("get_os_extended").catch(() => null);
        break;
      case "bios":   biosInfo.value = await invoke("get_bios_info"); break;
      case "mobo":   moboInfo.value = await invoke("get_motherboard_detailed"); break;
      case "cpu":
        if (!sysInfo.value) sysInfo.value = await invoke("get_system_info");
        [cpuCache.value, cpuExtended.value] = await Promise.all([
          invoke("get_cpu_cache_info"),
          invoke("get_cpu_extended").catch(() => null),
        ]); break;
      case "gpu":         gpuList.value = await invoke("get_gpu_detailed"); break;
      case "ram":         ramData.value = await invoke("get_ram_detailed"); break;
      case "disks":
        // Chargement séquentiel : disques d'abord (affichage rapide), puis SMART
        storageList.value = await invoke<any[]>("get_storage_physical_info").catch(() => []);
        invoke<any[]>("get_smart_info").then(v => { smartData.value = v; }).catch(() => {});
        break;
      case "volumes":     volumes.value = await invoke("get_logical_volumes"); break;
      case "network":     networkAdapters.value = await invoke("get_network_adapters_detailed"); break;
      case "connections":
        [connections.value, wifiInfo.value] = await Promise.all([
          invoke("get_active_connections"),
          invoke("get_wifi_status").catch(() => null),
        ]); break;
      case "monitors":  monitors.value = await invoke("get_monitor_info"); break;
      case "audio":     audioDevices.value = await invoke("get_audio_devices"); break;
      case "usb":       usbDevices.value = await invoke("get_usb_devices"); break;
      case "battery":   batteries.value = await invoke("get_battery_detailed"); break;
      case "power":     powerPlans.value = await invoke("get_power_plans"); break;
      case "printers":  printers.value = await invoke("get_printers"); break;
      case "software":  softwareList.value = await invoke("get_installed_software"); break;
      case "env":       envVars.value = await invoke("get_environment_variables"); break;
      case "processes": processes.value = await invoke("get_running_processes"); break;
      case "services":  services.value = await invoke("get_windows_services"); break;
      case "startup":   startupPrograms.value = await invoke("get_startup_programs_detailed"); break;
      case "tasks":     scheduledTasks.value = await invoke("get_scheduled_tasks"); break;
      case "security":  securityInfo.value = await invoke("get_security_status").catch(() => null); break;
      case "license":   licenseInfo.value = await invoke("get_windows_license"); break;
      case "updates":    updatesHistory.value = await invoke("get_installed_updates"); break;
      case "activation":  break; // auto-chargé dans DiagTabActivation
      case "comptes":     break; // auto-chargé dans DiagTabAccounts
      case "parefeu":     break; // auto-chargé dans DiagTabFirewall
      case "partages":    break; // auto-chargé dans DiagTabShares
      case "registre":    break; // auto-chargé dans DiagTabRegistry
      case "historique":    break; // auto-chargé dans DiagTabHistory
      case "pilotes":       break; // auto-chargé dans DiagTabSysDrivers
      case "driver-update": break; // auto-chargé dans DiagTabDriverUpdater
      case "certificats":   break; // auto-chargé dans DiagTabCertificates
      case "performances":  break; // auto-chargé dans DiagTabPerf
      case "outils-reseau": break; // auto-chargé dans DiagTabNetTools
      case "reparation":    break; // auto-chargé dans DiagTabRepair
      case "folders":     folders.value = await invoke("get_folder_sizes_detailed"); break;
    }
  } catch (e: any) {
    tabError.value[tab] = e?.toString() ?? "Erreur inconnue";
    loadedTabs.value.delete(tab);
  } finally { loadingTab.value = null; }
}

watch(activeTab, (tab) => {
  if (tab !== "tools" && tab !== "scan") loadTab(tab);
});

async function refreshTab() {
  const tab = activeTab.value;
  loadedTabs.value.delete(tab);
  await loadTab(tab);
}

// ============= Scan =============
function computeProblems(sr: ScanResult) {
  const p: string[] = [];
  if (!sr.firewall_enabled) p.push("🛡 Pare-feu Windows désactivé");
  if (!sr.defender_enabled) p.push("🛡 Windows Defender (temps réel) désactivé ou inconnu");
  if (!sr.network_ok) p.push("🌐 Pas de connectivité internet (8.8.8.8 injoignable)");
  if (sr.pending_reboot) p.push("🔄 Redémarrage Windows en attente");
  if (sr.suspicious_processes.length > 0)
    p.push(`⚠ ${sr.suspicious_processes.length} processus suspect(s) hors chemins sécurisés`);
  if (sr.winget_upgradable.length > 0)
    p.push(`🔄 ${sr.winget_upgradable.length} mise(s) à jour WinGet disponible(s)`);
  if (sr.choco_upgradable.length > 0)
    p.push(`🔄 ${sr.choco_upgradable.length} mise(s) à jour Chocolatey disponible(s)`);
  if (!sr.dism_status.toLowerCase().includes("sain") && sr.dism_status !== "OK")
    p.push(`🔧 DISM: ${sr.dism_status}`);
  if (!sr.sfc_status.toLowerCase().includes("intèg") && !sr.sfc_status.toLowerCase().includes("integ"))
    p.push(`🔧 SFC: ${sr.sfc_status}`);
  for (const d of sr.disk_usage) {
    if (d.used_percent > 90) p.push(`💾 Disque ${d.drive}: espace critique (${d.used_percent.toFixed(0)}%)`);
    else if (d.used_percent > 80) p.push(`💾 Disque ${d.drive}: espace faible (${d.used_percent.toFixed(0)}%)`);
  }
  if (sr.ram_usage_percent > 85) p.push(`🧠 RAM critique: ${sr.ram_usage_percent.toFixed(0)}% utilisé`);
  for (const b of batteries.value) {
    if (b.battery_health_percent > 0 && b.battery_health_percent < 80)
      p.push(`🔋 Batterie "${b.name}": santé faible (${b.battery_health_percent.toFixed(0)}%)`);
    if (b.cycle_count > 400)
      p.push(`🔋 Batterie "${b.name}": ${b.cycle_count} cycles — remplacement recommandé`);
  }
  const winAct = sr.windows_activation || (licenseInfo.value?.activation_status ?? "");
  if (winAct && winAct !== "Activé" && winAct !== "Licencié")
    p.push(`🔑 Windows non activé: ${winAct}`);
  if (sr.recent_errors.length > 5)
    p.push(`📋 ${sr.recent_errors.length} erreurs/critiques dans les journaux (48h)`);
  if (sr.defender_definition_age_days > 7)
    p.push(`🛡 Définitions Defender: ${sr.defender_definition_age_days} jours sans mise à jour`);
  if (sr.last_update_days > 60)
    p.push(`🔄 Dernier KB Windows: il y a ${sr.last_update_days} jours`);
  if (sr.temp_folder_size_mb > 2048)
    p.push(`🗑 Fichiers temporaires volumineux: ${(sr.temp_folder_size_mb / 1024).toFixed(1)} GB`);
  if (sr.suspicious_services.length > 5)
    p.push(`⚙ ${sr.suspicious_services.length} services tiers en cours hors chemins sécurisés`);
  if (sr.autorun_entries.length > 8)
    p.push(`🚀 ${sr.autorun_entries.length} entrées Autorun tiers détectées`);
  if (sr.last_bsod && !sr.last_bsod.includes("Aucun"))
    p.push(`💥 Dernier BSOD détecté: ${sr.last_bsod}`);
  scanProblems.value = p;
}

async function runTotalScan() {
  scanning.value = true; scanProgress.value = 0; scanStep.value = "Démarrage...";
  scanResult.value = null; scanProblems.value = [];
  const { invoke } = await import("@tauri-apps/api/core");
  const { listen } = await import("@tauri-apps/api/event");
  try {
    scanStep.value = "Chargement composants..."; scanProgress.value = 5;
    await Promise.allSettled([
      batteries.value.length === 0
        ? invoke<BatteryDetailed[]>("get_battery_detailed").then(v => { batteries.value = v; loadedTabs.value.add("battery"); })
        : Promise.resolve(),
      !licenseInfo.value
        ? invoke<WinLicense>("get_windows_license").then(v => { licenseInfo.value = v; loadedTabs.value.add("license"); }).catch(() => {})
        : Promise.resolve(),
      storageList.value.length === 0
        ? invoke<StoragePhysical[]>("get_storage_physical_info").then(v => { storageList.value = v; loadedTabs.value.add("disks"); })
        : Promise.resolve(),
      !sysInfo.value
        ? invoke<SysInfo>("get_system_info").then(v => { sysInfo.value = v; })
        : Promise.resolve(),
    ]);
    scanProgress.value = 15;
    const unlisten = await listen<{ step: string; percent: number }>("scan-progress", (e) => {
      scanProgress.value = 15 + Math.round(e.payload.percent * 0.85);
      scanStep.value = e.payload.step;
    });
    try { scanResult.value = await invoke<ScanResult>("run_total_scan"); }
    finally { unlisten(); }
    if (scanResult.value) computeProblems(scanResult.value);
  } catch { notify.error("Scan échoué"); }
  finally { scanning.value = false; }
}

// ============= Exports =============
function kbStr(v: number) { return v >= 1024 ? `${(v / 1024).toFixed(0)} MB` : `${v} KB`; }
function mbStr(v: number) { return v >= 1024 ? `${(v / 1024).toFixed(1)} GB` : `${v.toFixed(0)} MB`; }

async function writeExport(filename: string, content: string) {
  const { invoke } = await import("@tauri-apps/api/core");
  const path = await invoke<string>("save_export_file", { filename, content });
  notify.success("Export sauvegardé", path);
}

async function openExportFolder() {
  const { invoke } = await import("@tauri-apps/api/core");
  await invoke("open_path", { path: await invoke<string>("get_export_dir") });
}

async function exportJson() {
  const payload: Record<string, any> = {
    generated: new Date().toISOString(),
    system: sysInfo.value, bios: biosInfo.value, motherboard: moboInfo.value,
    cpu_cache: cpuCache.value, cpu_extended: cpuExtended.value, os_extended: osExtended.value,
    gpus: gpuList.value, ram: ramData.value, storage: storageList.value, volumes: volumes.value,
    network: networkAdapters.value, connections: connections.value, wifi: wifiInfo.value,
    monitors: monitors.value, audio: audioDevices.value, usb: usbDevices.value,
    batteries: batteries.value, power_plans: powerPlans.value, printers: printers.value,
    software: softwareList.value, startup: startupPrograms.value, env_vars: envVars.value,
    processes: processes.value.slice(0, 100), services: services.value,
    scheduled_tasks: scheduledTasks.value, security: securityInfo.value,
    license: licenseInfo.value, updates: updatesHistory.value, folders: folders.value,
    scan: scanResult.value, problems: scanProblems.value,
  };
  await writeExport("diagnostic.json", JSON.stringify(payload, null, 2));
}

function buildTxtReport(): string {
  const si = sysInfo.value;
  const lines: string[] = [
    "═══════════════════════════════════════════",
    "       RAPPORT DIAGNOSTIC — NiTriTe",
    `       Généré le ${new Date().toLocaleString()}`,
    "═══════════════════════════════════════════", "",
  ];
  if (si) {
    lines.push("[ SYSTÈME ]",
      `  OS         : ${si.os.name} ${si.os.version} (${si.os.architecture})`,
      `  Hostname   : ${si.os.hostname}`, `  Build      : ${si.os.build_number}`, "");
    lines.push("[ PROCESSEUR ]", `  Modèle     : ${si.cpu.name}`,
      `  Fabricant  : ${si.cpu.manufacturer}`,
      `  Cœurs      : ${si.cpu.cores} cores / ${si.cpu.threads} threads`,
      `  Fréquence  : ${(si.cpu.base_speed_mhz / 1000).toFixed(2)} GHz`, "");
    if (cpuCache.value) {
      const c = cpuCache.value;
      lines.push("[ CACHE CPU ]",
        `  L1i: ${kbStr(c.l1_instruction_kb)}  L1d: ${kbStr(c.l1_data_kb)}  L2: ${kbStr(c.l2_kb)}  L3: ${kbStr(c.l3_kb)}`, "");
    }
    lines.push("[ MÉMOIRE RAM ]",
      `  Total      : ${si.ram.total_gb.toFixed(1)} GB`,
      `  Utilisée   : ${si.ram.used_gb.toFixed(1)} GB (${Math.round(si.ram.usage_percent)}%)`, "");
  }
  if (ramData.value) {
    lines.push("[ SLOTS RAM ]", `  Slots: ${ramData.value.used_slots}/${ramData.value.total_slots} utilisés`);
    for (const s of ramData.value.slots)
      lines.push(`  ${s.device_locator} : ${s.capacity_gb.toFixed(0)} GB ${s.memory_type}-${s.speed_mhz} (${s.manufacturer}) [${s.form_factor}] P/N: ${s.part_number || "N/A"}`);
    lines.push("");
  }
  if (biosInfo.value) {
    const b = biosInfo.value;
    lines.push("[ BIOS ]", `  Fabricant  : ${b.manufacturer}`, `  Version    : ${b.version}`,
      `  Date       : ${b.release_date}`, `  SMBIOS     : ${b.smbios_version}`,
      `  N° Série   : ${b.serial_number || "N/A"}`, "");
  }
  if (moboInfo.value) {
    const m = moboInfo.value;
    lines.push("[ CARTE MÈRE ]", `  Fabricant  : ${m.manufacturer}`, `  Modèle     : ${m.product}`,
      `  Version    : ${m.version}`, `  N° Série   : ${m.serial_number || "N/A"}`, "");
  }
  if (gpuList.value.length) {
    lines.push("[ GPU ]");
    for (const g of gpuList.value)
      lines.push(`  ${g.name}`, `    VRAM: ${g.adapter_ram_mb >= 1024 ? (g.adapter_ram_mb / 1024).toFixed(0) + "GB" : g.adapter_ram_mb + "MB"} | Driver: ${g.driver_version} (${g.driver_date}) | ${g.current_resolution} @${g.current_refresh_rate}Hz`);
    lines.push("");
  }
  if (storageList.value.length) {
    lines.push("[ STOCKAGE ]");
    for (const d of storageList.value)
      lines.push(`  ${d.model} | ${d.size_gb.toFixed(0)} GB | ${d.interface_type} | ${d.media_type} | S/N: ${d.serial_number || "N/A"} | FW: ${d.firmware_revision || "N/A"} | Statut: ${d.status}`);
    lines.push("");
  }
  if (networkAdapters.value.length) {
    lines.push("[ RÉSEAU ]");
    for (const a of networkAdapters.value)
      lines.push(`  ${a.name} | MAC: ${a.mac_address} | IP: ${a.ip_addresses.join(",")} | ${a.speed_mbps} Mbps | GW: ${a.default_gateway.join(",")} | DNS: ${a.dns_servers.join(",")}`);
    lines.push("");
  }
  if (batteries.value.length) {
    lines.push("[ BATTERIE ]");
    for (const b of batteries.value)
      lines.push(`  ${b.name} | Charge: ${b.estimated_charge_remaining}% | Autonomie: ${b.estimated_run_time}`,
        `    Cap. originale: ${b.design_capacity} mWh | Cap. actuelle: ${b.full_charge_capacity} mWh`,
        `    Santé: ${b.battery_health_percent.toFixed(1)}% | Cycles: ${b.cycle_count} | Chimie: ${b.chemistry}`);
    lines.push("");
  }
  if (licenseInfo.value) {
    const l = licenseInfo.value;
    lines.push("[ LICENCE WINDOWS ]", `  Produit    : ${l.product_name}`,
      `  Statut     : ${l.activation_status}`,
      `  Clé        : XXXXX-XXXXX-XXXXX-XXXXX-${l.partial_product_key}`, "");
  }
  if (monitors.value.length) {
    lines.push("[ ÉCRANS ]");
    for (const m of monitors.value)
      lines.push(`  ${m.name} — ${m.screen_width}x${m.screen_height} — ${m.pixels_per_inch} PPI — ${m.manufacturer || "N/A"}`);
    lines.push("");
  }
  if (audioDevices.value.length) {
    lines.push("[ AUDIO ]");
    for (const a of audioDevices.value) lines.push(`  ${a.name} | ${a.manufacturer || "—"} | ${a.status}`);
    lines.push("");
  }
  if (usbDevices.value.length) {
    lines.push(`[ USB (${usbDevices.value.length} périphériques) ]`);
    for (const u of usbDevices.value) lines.push(`  ${u.name} | ${u.pnp_class || "—"} | ${u.manufacturer || "—"}`);
    lines.push("");
  }
  if (printers.value.length) {
    lines.push("[ IMPRIMANTES ]");
    for (const p of printers.value)
      lines.push(`  ${p.name}${p.is_default ? " [DÉFAUT]" : ""} | Driver: ${p.driver_name} | Port: ${p.port_name} | Réseau: ${p.is_network ? "Oui" : "Non"}`);
    lines.push("");
  }
  if (powerPlans.value.length) {
    lines.push("[ PLANS D'ÉNERGIE ]");
    for (const p of powerPlans.value) lines.push(`  ${p.name}${p.is_active ? " [ACTIF]" : ""} | ${p.guid}`);
    lines.push("");
  }
  if (startupPrograms.value.length) {
    lines.push("[ DÉMARRAGE ]");
    for (const s of startupPrograms.value) lines.push(`  ${s.name} | ${s.command} | ${s.location} | ${s.category}`);
    lines.push("");
  }
  if (softwareList.value.length) {
    const sorted = [...softwareList.value].sort((a, b) => (b.install_date || "").localeCompare(a.install_date || "")).slice(0, 100);
    lines.push(`[ LOGICIELS — ${softwareList.value.length} total, 100 derniers ]`);
    for (const s of sorted)
      lines.push(`  ${s.name.padEnd(48)} | v${(s.version || "—").padEnd(18)} | ${(s.publisher || "—").padEnd(28)} | ${s.install_date || "—"}`);
    lines.push("");
  }
  if (envVars.value.length) {
    const sysVars = envVars.value.filter(e => e.var_type === "Système");
    if (sysVars.length) {
      lines.push("[ VARIABLES D'ENVIRONNEMENT SYSTÈME ]");
      for (const e of sysVars) lines.push(`  ${e.name.padEnd(28)} = ${e.value}`);
      lines.push("");
    }
  }
  if (updatesHistory.value.length) {
    lines.push(`[ MISES À JOUR — ${updatesHistory.value.length} total ]`);
    for (const u of updatesHistory.value.slice(0, 30))
      lines.push(`  ${u.hotfix_id.padEnd(14)} | ${(u.description || "").padEnd(18)} | ${u.installed_on || "—"} | ${u.installed_by || "—"}`);
    lines.push("");
  }
  if (scanResult.value) {
    const sr = scanResult.value;
    lines.push("[ SCAN TOTAL SYSTÈME ]",
      `  DISM              : ${sr.dism_status}`,
      `  SFC               : ${sr.sfc_status}`,
      `  Pare-feu          : ${sr.firewall_enabled ? "Activé" : "DÉSACTIVÉ ⚠"}`,
      `  Defender          : ${sr.defender_enabled ? "Actif" : "INACTIF ⚠"}`,
      `  Antivirus tiers   : ${sr.antivirus_installed || "Aucun détecté"}`,
      `  Connectivité      : ${sr.network_ok ? "OK" : "HORS LIGNE ⚠"}`,
      `  Reboot requis     : ${sr.pending_reboot ? "OUI ⚠" : "Non"}`,
      `  Uptime            : ${sr.uptime_hours >= 24 ? (sr.uptime_hours / 24).toFixed(1) + " jours" : sr.uptime_hours.toFixed(1) + " h"}`,
      `  Activation        : ${sr.windows_activation || "Inconnu"}`,
      `  Dernier BSOD      : ${sr.last_bsod || "Aucun"}`,
      `  Dernier KB        : il y a ${sr.last_update_days >= 0 ? sr.last_update_days : "?"} jours`,
      `  Dossier %TEMP%    : ${(sr.temp_folder_size_mb / 1024).toFixed(1)} GB`,
      `  WinGet MAJ        : ${sr.winget_upgradable.length}`,
      `  Processus suspects: ${sr.suspicious_processes.length}`,
      `  Services tiers    : ${sr.suspicious_services.length}`,
      `  Ports ouverts     : ${sr.open_ports.join(", ") || "Aucun"}`,
      `  Mém. virtuelle    : ${sr.virtual_memory_available_mb} MB libres / ${sr.virtual_memory_total_mb} MB`, "");
    if (sr.disk_usage.length) {
      lines.push("  — Espace disque :");
      for (const d of sr.disk_usage) lines.push(`    ${d.drive}: ${d.used_percent.toFixed(0)}% (${d.free_gb.toFixed(0)} GB libres / ${d.total_gb.toFixed(0)} GB)`);
      lines.push("");
    }
    if (sr.suspicious_processes.length) {
      lines.push("  — Processus suspects :");
      for (const p of sr.suspicious_processes) lines.push(`    [${p.pid}] ${p.name} — ${p.path} — ${p.reason}`);
      lines.push("");
    }
  }
  if (scanProblems.value.length) {
    lines.push("[ PROBLÈMES DÉTECTÉS ]");
    for (const p of scanProblems.value) lines.push(`  ${p}`);
    lines.push("");
  } else if (scanResult.value) {
    lines.push("[ ÉTAT GLOBAL ]", "  ✓ Aucun problème critique détecté", "");
  }
  lines.push("═══════════════════════════════════════════",
    "  Fin du rapport — NiTriTe",
    `  Généré le ${new Date().toLocaleString()}`,
    "═══════════════════════════════════════════");
  return lines.join("\n");
}

async function exportTxt() { await writeExport("diagnostic.txt", buildTxtReport()); }

function badge(v: boolean, ok = "Activé", ko = "DÉSACTIVÉ") { return v ? ok : ko; }
function cls(v: boolean, good = true) { return (v === good) ? 'ok' : 'warn'; }

async function exportHtml() {
  const si = sysInfo.value;
  const sr = scanResult.value;
  const CSS = `*{box-sizing:border-box;margin:0;padding:0}body{font-family:'Segoe UI',Arial,sans-serif;background:#0d0d1a;color:#c9d1e0;padding:24px;line-height:1.5}h1{color:#7c9af5;font-size:22px;margin-bottom:4px}h2{color:#a78bfa;font-size:15px;margin:18px 0 8px;border-bottom:1px solid #2a2a3e;padding-bottom:6px}h3{color:#94a3b8;font-size:12px;font-weight:600;margin:12px 0 5px}.header{text-align:center;background:#161625;padding:20px;border-radius:10px;margin-bottom:20px}.header p{color:#64748b;font-size:12px;margin-top:5px}.section{background:#161625;border-radius:8px;padding:16px;margin-bottom:12px;border:1px solid #1e1e35}table{width:100%;border-collapse:collapse;margin-top:6px}th{background:#1a1a2e;color:#7c9af5;padding:6px 10px;text-align:left;font-size:11px;font-weight:600}td{padding:5px 10px;border-bottom:1px solid #1e1e35;font-size:12px}tr:last-child td{border-bottom:none}.ok{background:#052e16;color:#4ade80;padding:1px 7px;border-radius:4px;font-size:11px;display:inline-block}.warn{background:#3b1f00;color:#fb923c;padding:1px 7px;border-radius:4px;font-size:11px;display:inline-block}.danger{background:#3b0000;color:#f87171;padding:1px 7px;border-radius:4px;font-size:11px;display:inline-block}.info{background:#0c1f3b;color:#60a5fa;padding:1px 7px;border-radius:4px;font-size:11px;display:inline-block}.neutral{background:#1e1e35;color:#94a3b8;padding:1px 7px;border-radius:4px;font-size:11px;display:inline-block}code{font-family:Consolas,monospace;background:#1a1a2e;padding:1px 5px;border-radius:3px;font-size:11px;color:#7c9af5}.grid2{display:grid;grid-template-columns:1fr 1fr;gap:14px}.problems li{padding:5px 0;border-bottom:1px solid #1e1e35;font-size:12px}.no-prob{color:#4ade80;font-size:13px}.kv{display:flex;padding:4px 0;border-bottom:1px solid #1e1e35;font-size:12px}.kv .k{color:#94a3b8;min-width:200px;flex-shrink:0}.kv .v{word-break:break-all}`;

  let h = `<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>Rapport NiTriTe — ${new Date().toLocaleDateString()}</title><style>${CSS}</style></head><body>`;
  h += `<div class="header"><h1>🖥 Rapport Diagnostic — NiTriTe</h1><p>Généré le ${new Date().toLocaleString()}</p></div>`;

  // Problèmes
  if (scanProblems.value.length) {
    h += `<div class="section"><h2>⚠ Problèmes Détectés (${scanProblems.value.length})</h2><ul class="problems">${scanProblems.value.map(p => `<li>${p}</li>`).join('')}</ul></div>`;
  } else if (sr) {
    h += `<div class="section"><p class="no-prob">✅ Aucun problème critique détecté</p></div>`;
  }

  // Système
  if (si) {
    h += `<div class="section"><h2>Système</h2>
<div class="kv"><span class="k">OS</span><span class="v">${si.os.name} ${si.os.version} (${si.os.architecture})</span></div>
<div class="kv"><span class="k">Hostname</span><span class="v">${si.os.hostname}</span></div>
<div class="kv"><span class="k">Build</span><span class="v">${si.os.build_number}</span></div>
<h3>Processeur</h3>
<div class="kv"><span class="k">Modèle</span><span class="v">${si.cpu.name}</span></div>
<div class="kv"><span class="k">Cœurs / Threads</span><span class="v">${si.cpu.cores} / ${si.cpu.threads}</span></div>
<div class="kv"><span class="k">Fréquence</span><span class="v">${(si.cpu.base_speed_mhz / 1000).toFixed(2)} GHz</span></div>
<h3>RAM</h3>
<div class="kv"><span class="k">Total</span><span class="v">${si.ram.total_gb.toFixed(1)} GB</span></div>
<div class="kv"><span class="k">Utilisée</span><span class="v">${si.ram.used_gb.toFixed(1)} GB (${Math.round(si.ram.usage_percent)}%)</span></div></div>`;
  }

  // BIOS
  if (biosInfo.value) {
    const b = biosInfo.value;
    h += `<div class="section"><h2>BIOS</h2>
<div class="kv"><span class="k">Fabricant</span><span class="v">${b.manufacturer}</span></div>
<div class="kv"><span class="k">Version</span><span class="v">${b.version}</span></div>
<div class="kv"><span class="k">Date</span><span class="v">${b.release_date}</span></div>
<div class="kv"><span class="k">SMBIOS</span><span class="v">${b.smbios_version}</span></div>
<div class="kv"><span class="k">N° Série</span><span class="v">${b.serial_number || 'N/A'}</span></div></div>`;
  }

  // Carte mère
  if (moboInfo.value) {
    const m = moboInfo.value;
    h += `<div class="section"><h2>Carte Mère</h2>
<div class="kv"><span class="k">Fabricant</span><span class="v">${m.manufacturer}</span></div>
<div class="kv"><span class="k">Modèle</span><span class="v">${m.product}</span></div>
<div class="kv"><span class="k">N° Série</span><span class="v">${m.serial_number || 'N/A'}</span></div></div>`;
  }

  // GPU
  if (gpuList.value.length) {
    h += `<div class="section"><h2>GPU (${gpuList.value.length})</h2><table><tr><th>Modèle</th><th>VRAM</th><th>Driver</th><th>Date driver</th><th>Résolution</th></tr>`;
    for (const g of gpuList.value) {
      h += `<tr><td>${g.name}</td><td>${g.adapter_ram_mb >= 1024 ? (g.adapter_ram_mb/1024).toFixed(0)+'GB' : g.adapter_ram_mb+'MB'}</td><td>${g.driver_version}</td><td>${g.driver_date}</td><td>${g.current_resolution} @${g.current_refresh_rate}Hz</td></tr>`;
    }
    h += `</table></div>`;
  }

  // RAM slots
  if (ramData.value) {
    h += `<div class="section"><h2>RAM — ${ramData.value.used_slots}/${ramData.value.total_slots} slots — ${ramData.value.total_capacity_gb.toFixed(0)} GB</h2><table><tr><th>Slot</th><th>Capacité</th><th>Type</th><th>Vitesse</th><th>Fabricant</th><th>P/N</th></tr>`;
    for (const s of ramData.value.slots) {
      h += `<tr><td>${s.device_locator}</td><td>${s.capacity_gb.toFixed(0)} GB</td><td>${s.memory_type}</td><td>${s.speed_mhz} MHz</td><td>${s.manufacturer}</td><td>${s.part_number || '—'}</td></tr>`;
    }
    h += `</table></div>`;
  }

  // Stockage
  if (storageList.value.length) {
    h += `<div class="section"><h2>Stockage Physique</h2><table><tr><th>Modèle</th><th>Taille</th><th>Interface</th><th>Type</th><th>Serial</th><th>Statut</th></tr>`;
    for (const d of storageList.value) {
      h += `<tr><td>${d.model}</td><td>${d.size_gb.toFixed(0)} GB</td><td>${d.interface_type}</td><td>${d.media_type}</td><td><code>${d.serial_number || '—'}</code></td><td>${d.status}</td></tr>`;
    }
    h += `</table></div>`;
  }

  // Réseau
  if (networkAdapters.value.length) {
    h += `<div class="section"><h2>Adaptateurs Réseau</h2><table><tr><th>Nom</th><th>MAC</th><th>IP</th><th>Vitesse</th><th>DNS</th></tr>`;
    for (const a of networkAdapters.value.slice(0, 10)) {
      h += `<tr><td>${a.name}</td><td><code>${a.mac_address}</code></td><td>${a.ip_addresses.join(', ')}</td><td>${a.speed_mbps} Mbps</td><td>${a.dns_servers.slice(0,2).join(', ')}</td></tr>`;
    }
    h += `</table></div>`;
  }

  // Batterie
  if (batteries.value.length) {
    h += `<div class="section"><h2>Batterie</h2>`;
    for (const b of batteries.value) {
      h += `<div class="kv"><span class="k">${b.name}</span><span class="v"><span class="${b.battery_health_percent > 80 ? 'ok' : b.battery_health_percent > 50 ? 'warn' : 'danger'}">${b.battery_health_percent.toFixed(0)}% santé</span> — ${b.estimated_charge_remaining}% charge — ${b.cycle_count} cycles — ${b.chemistry}</span></div>`;
    }
    h += `</div>`;
  }

  // Licence
  if (licenseInfo.value) {
    const l = licenseInfo.value;
    h += `<div class="section"><h2>Licence Windows</h2>
<div class="kv"><span class="k">Produit</span><span class="v">${l.product_name}</span></div>
<div class="kv"><span class="k">Statut activation</span><span class="v"><span class="${l.activation_status === 'Activé' ? 'ok' : 'danger'}">${l.activation_status}</span></span></div>
<div class="kv"><span class="k">Clé partielle</span><span class="v"><code>XXXXX-XXXXX-XXXXX-XXXXX-${l.partial_product_key}</code></span></div>
${l.office_name ? `<div class="kv"><span class="k">${l.office_name}</span><span class="v"><span class="${l.office_status === 'Activé' ? 'ok' : 'warn'}">${l.office_status}</span></span></div>` : ''}</div>`;
  }

  // Scan Total
  if (sr) {
    h += `<div class="section"><h2>Scan Total — Résultats</h2><div class="grid2">`;

    // Sécurité de base
    h += `<div><h3>Sécurité</h3>
<div class="kv"><span class="k">Activation</span><span class="v"><span class="${cls(sr.windows_activation === 'Activé')}">${sr.windows_activation || 'Inconnu'}</span></span></div>
<div class="kv"><span class="k">Pare-feu</span><span class="v"><span class="${cls(sr.firewall_enabled)}">${badge(sr.firewall_enabled)}</span></span></div>
<div class="kv"><span class="k">Defender</span><span class="v"><span class="${cls(sr.defender_enabled)}">${badge(sr.defender_enabled, 'Actif', 'INACTIF')}</span></span></div>
<div class="kv"><span class="k">BSOD récent</span><span class="v">${sr.last_bsod || 'Aucun'}</span></div>
<div class="kv"><span class="k">DISM</span><span class="v">${sr.dism_status}</span></div>
<div class="kv"><span class="k">SFC</span><span class="v">${sr.sfc_status}</span></div>
<div class="kv"><span class="k">Connectivité</span><span class="v"><span class="${cls(sr.network_ok)}">${sr.network_ok ? 'OK' : 'Hors ligne'}</span></span></div>
<div class="kv"><span class="k">Redémarrage requis</span><span class="v"><span class="${sr.pending_reboot ? 'warn' : 'ok'}">${sr.pending_reboot ? 'Oui' : 'Non'}</span></span></div></div>`;

    // Sécurité avancée (scan_extra)
    h += `<div><h3>Sécurité Avancée</h3>
<div class="kv"><span class="k">TPM</span><span class="v"><span class="${cls(sr.tpm_present)}">${sr.tpm_present ? (sr.tpm_enabled ? 'Présent & Activé' : 'Présent (désactivé)') : 'Absent'}</span> ${sr.tpm_version || ''}</span></div>
<div class="kv"><span class="k">Secure Boot</span><span class="v"><span class="${cls(sr.secure_boot)}">${badge(sr.secure_boot, 'Activé', 'Désactivé')}</span></span></div>
<div class="kv"><span class="k">UAC</span><span class="v">${sr.uac_level || 'Inconnu'}</span></div>
<div class="kv"><span class="k">RDP</span><span class="v"><span class="${sr.rdp_enabled ? 'warn' : 'ok'}">${badge(sr.rdp_enabled)}</span></span></div>
<div class="kv"><span class="k">SMBv1</span><span class="v"><span class="${sr.smbv1_enabled ? 'danger' : 'ok'}">${sr.smbv1_enabled ? 'Activé ⚠' : 'Désactivé'}</span></span></div>
<div class="kv"><span class="k">WMI Abonnements</span><span class="v"><span class="${sr.wmi_subscriptions > 0 ? 'danger' : 'ok'}">${sr.wmi_subscriptions ?? 0}</span></span></div>
<div class="kv"><span class="k">Compte Invité</span><span class="v"><span class="${sr.guest_enabled ? 'warn' : 'ok'}">${sr.guest_enabled ? 'Activé ⚠' : 'Désactivé'}</span></span></div>
<div class="kv"><span class="k">MAJ en attente</span><span class="v">${sr.pending_updates_cached >= 0 ? sr.pending_updates_cached : 'N/A'}</span></div></div>`;
    h += `</div>`;

    // Identité système
    if (sr.system_manufacturer || sr.bios_manufacturer) {
      h += `<h3>Identité Système & BIOS</h3><div class="grid2">`;
      h += `<div>
<div class="kv"><span class="k">Fabricant</span><span class="v">${sr.system_manufacturer || '—'}</span></div>
<div class="kv"><span class="k">Modèle</span><span class="v">${sr.system_model || '—'}</span></div>
<div class="kv"><span class="k">N° Série</span><span class="v"><code>${sr.system_serial || 'N/A'}</code></span></div></div>`;
      h += `<div>
<div class="kv"><span class="k">BIOS Fabricant</span><span class="v">${sr.bios_manufacturer || '—'}</span></div>
<div class="kv"><span class="k">BIOS Version</span><span class="v"><code>${sr.bios_version || '—'}</code></span></div>
<div class="kv"><span class="k">BIOS Date</span><span class="v">${sr.bios_date || '—'}</span></div>
<div class="kv"><span class="k">Type de licence</span><span class="v">${sr.license_type || '—'}</span></div></div>`;
      h += `</div>`;
    }

    // Clés
    h += `<h3>Licences</h3>
<div class="kv"><span class="k">Clé Windows</span><span class="v"><code>${sr.windows_product_key || 'N/A'}</code></span></div>`;
    if (sr.office_product_key) h += `<div class="kv"><span class="k">${sr.office_name || 'Office'}</span><span class="v"><code>${sr.office_product_key}</code></span></div>`;
    h += `<div class="kv"><span class="k">Point de restauration</span><span class="v">${sr.last_restore_point || '—'}</span></div>`;

    // BitLocker
    if (sr.bitlocker_volumes?.length) {
      h += `<h3>BitLocker</h3><table><tr><th>Volume</th><th>Statut</th><th>Clé de récupération</th></tr>`;
      for (const bv of sr.bitlocker_volumes) {
        const on = bv.protection_status === 'On' || bv.protection_status === '1';
        h += `<tr><td><code>${bv.drive}</code></td><td><span class="${on ? 'ok' : 'warn'}">${on ? 'Protégé' : 'Non protégé'}</span></td><td><code style="font-size:10px;color:#fb923c">${bv.recovery_password || 'N/A'}</code></td></tr>`;
      }
      h += `</table>`;
    }

    // Disques
    if (sr.disk_usage?.length) {
      h += `<h3>Espace Disque</h3><table><tr><th>Volume</th><th>Utilisé %</th><th>Libre</th><th>Total</th></tr>`;
      for (const d of sr.disk_usage) {
        h += `<tr><td><code>${d.drive}</code></td><td><span class="${d.used_percent > 90 ? 'danger' : d.used_percent > 80 ? 'warn' : 'ok'}">${d.used_percent.toFixed(0)}%</span></td><td>${d.free_gb.toFixed(0)} GB</td><td>${d.total_gb.toFixed(0)} GB</td></tr>`;
      }
      h += `</table>`;
    }

    // Top processus
    if (sr.top_cpu?.length || sr.top_ram?.length) {
      h += `<h3>Top Processus</h3><div class="grid2">`;
      if (sr.top_cpu?.length) {
        h += `<div><table><tr><th>PID</th><th>CPU Top 5</th><th>Sec.</th></tr>${sr.top_cpu.map(p => `<tr><td>${p.pid}</td><td>${p.name}</td><td>${p.value}s</td></tr>`).join('')}</table></div>`;
      }
      if (sr.top_ram?.length) {
        h += `<div><table><tr><th>PID</th><th>RAM Top 5</th><th>MB</th></tr>${sr.top_ram.map(p => `<tr><td>${p.pid}</td><td>${p.name}</td><td>${p.value}</td></tr>`).join('')}</table></div>`;
      }
      h += `</div>`;
    }

    // Admins locaux
    if (sr.local_admins?.length) {
      h += `<h3>Administrateurs locaux</h3><p style="font-size:12px">${sr.local_admins.map(a => `<code>${a}</code>`).join(' ')}</p>`;
    }

    // Processus suspects
    if (sr.suspicious_processes?.length) {
      h += `<h3>Processus Suspects (${sr.suspicious_processes.length})</h3><table><tr><th>PID</th><th>Nom</th><th>Raison</th><th>Chemin</th></tr>`;
      for (const p of sr.suspicious_processes.slice(0, 20)) {
        h += `<tr><td>${p.pid}</td><td>${p.name}</td><td><span class="warn">${p.reason}</span></td><td style="font-size:10px">${p.path}</td></tr>`;
      }
      h += `</table>`;
    }

    // Tâches suspectes
    if (sr.susp_tasks?.length) {
      h += `<h3>Tâches Planifiées Suspectes (${sr.susp_tasks_count})</h3><table><tr><th>Nom</th><th>Chemin</th><th>Exécutable</th></tr>`;
      for (const t of sr.susp_tasks) {
        h += `<tr><td>${t.name}</td><td>${t.path}</td><td style="font-size:10px">${t.exec}</td></tr>`;
      }
      h += `</table>`;
    }

    // Antivirus
    h += `<h3>Antivirus & Protection</h3>
<div class="kv"><span class="k">Antivirus tiers</span><span class="v">${sr.antivirus_installed || 'Aucun (Defender)'}</span></div>
<div class="kv"><span class="k">Defs Defender</span><span class="v">${sr.defender_definition_age_days >= 0 ? sr.defender_definition_age_days + ' jours' : 'N/A'}</span></div>
<div class="kv"><span class="k">Dernier KB</span><span class="v">il y a ${sr.last_update_days >= 0 ? sr.last_update_days : '?'} jours</span></div>`;

    // Ports
    if (sr.open_ports?.length) {
      h += `<h3>Ports en écoute globale</h3><p style="font-size:12px">${sr.open_ports.map(p => `<code>${p}</code>`).join(' ')}</p>`;
    }

    h += `</div>`;
  }

  // Logiciels (top 50)
  if (softwareList.value.length) {
    const sorted = [...softwareList.value].sort((a, b) => (b.install_date || '').localeCompare(a.install_date || '')).slice(0, 50);
    h += `<div class="section"><h2>Logiciels installés (${softwareList.value.length} total — 50 récents)</h2><table><tr><th>Nom</th><th>Version</th><th>Éditeur</th><th>Date</th></tr>`;
    for (const s of sorted) h += `<tr><td>${s.name}</td><td>${s.version || '—'}</td><td>${s.publisher || '—'}</td><td>${s.install_date || '—'}</td></tr>`;
    h += `</table></div>`;
  }

  // Mises à jour
  if (updatesHistory.value.length) {
    h += `<div class="section"><h2>Mises à jour Windows (${updatesHistory.value.length})</h2><table><tr><th>KB</th><th>Description</th><th>Installé le</th><th>Par</th></tr>`;
    for (const u of updatesHistory.value.slice(0, 30)) h += `<tr><td><code>${u.hotfix_id}</code></td><td>${u.description || '—'}</td><td>${u.installed_on || '—'}</td><td>${u.installed_by || '—'}</td></tr>`;
    h += `</table></div>`;
  }

  h += `<div style="text-align:center;margin-top:20px;color:#475569;font-size:11px">Rapport généré par NiTriTe — ${new Date().toLocaleString()}</div></body></html>`;

  await writeExport("diagnostic.html", h);
}

async function exportMd() {
  const si = sysInfo.value;
  const lines = ["# Rapport Diagnostic — NiTriTe", `> ${new Date().toLocaleString()}`, ""];
  if (si) {
    lines.push("## Système", `| | |`, `|---|---|`,
      `| OS | ${si.os.name} ${si.os.version} (${si.os.architecture}) |`,
      `| Hostname | ${si.os.hostname} |`, `| Build | ${si.os.build_number} |`, "");
    lines.push("## Processeur",
      `**${si.cpu.name}** — ${si.cpu.cores}c/${si.cpu.threads}t @ ${(si.cpu.base_speed_mhz / 1000).toFixed(2)} GHz — ${Math.round(si.cpu.usage_percent)}% usage`, "");
  }
  if (cpuCache.value) {
    const c = cpuCache.value;
    lines.push(`Cache : L1i ${kbStr(c.l1_instruction_kb)} | L1d ${kbStr(c.l1_data_kb)} | L2 ${kbStr(c.l2_kb)} | L3 ${kbStr(c.l3_kb)}`, "");
  }
  if (ramData.value) {
    lines.push("## RAM",
      `${ramData.value.used_slots}/${ramData.value.total_slots} slots — ${ramData.value.total_capacity_gb.toFixed(0)} GB total`, "",
      `| Slot | GB | Type | Vitesse | Fabricant | P/N |`, `|---|---|---|---|---|---|`,
      ...ramData.value.slots.map(s => `| ${s.device_locator} | ${s.capacity_gb.toFixed(0)} | ${s.memory_type} | ${s.speed_mhz} MHz | ${s.manufacturer} | ${s.part_number || "—"} |`), "");
  }
  if (biosInfo.value) {
    const b = biosInfo.value;
    lines.push("## BIOS", `${b.manufacturer} — ${b.version} (${b.release_date}) — SMBIOS ${b.smbios_version}`, "");
  }
  if (moboInfo.value) {
    const m = moboInfo.value;
    lines.push("## Carte Mère", `${m.manufacturer} ${m.product} — ${m.version}`, "");
  }
  if (gpuList.value.length) {
    lines.push("## GPU", `| Modèle | VRAM | Driver | Résolution | Hz |`, `|---|---|---|---|---|`,
      ...gpuList.value.map(g => `| ${g.name} | ${g.adapter_ram_mb >= 1024 ? (g.adapter_ram_mb / 1024).toFixed(0) + "GB" : g.adapter_ram_mb + "MB"} | ${g.driver_version} | ${g.current_resolution} | ${g.current_refresh_rate} |`), "");
  }
  if (storageList.value.length) {
    lines.push("## Stockage", `| Modèle | Taille | Interface | Type | Serial | Statut |`, `|---|---|---|---|---|---|`,
      ...storageList.value.map(d => `| ${d.model} | ${d.size_gb.toFixed(0)} GB | ${d.interface_type} | ${d.media_type} | ${d.serial_number || "—"} | ${d.status} |`), "");
  }
  if (batteries.value.length) {
    lines.push("## Batterie", `| Nom | Charge | Santé | Cycles | Autonomie |`, `|---|---|---|---|---|`,
      ...batteries.value.map(b => `| ${b.name} | ${b.estimated_charge_remaining}% | ${b.battery_health_percent.toFixed(1)}% | ${b.cycle_count} | ${b.estimated_run_time} |`), "");
  }
  if (licenseInfo.value) {
    const l = licenseInfo.value;
    lines.push("## Licence Windows", `| | |`, `|---|---|`,
      `| Produit | ${l.product_name} |`, `| Statut | ${l.activation_status} |`, "");
  }
  if (softwareList.value.length) {
    lines.push("## Logiciels installés (50 plus récents)",
      `| Nom | Version | Éditeur | Date |`, `|---|---|---|---|`,
      ...[...softwareList.value].sort((a, b) => (b.install_date || "").localeCompare(a.install_date || "")).slice(0, 50)
        .map(s => `| ${s.name} | ${s.version || "—"} | ${s.publisher || "—"} | ${s.install_date || "—"} |`), "");
  }
  if (scanProblems.value.length) {
    lines.push("## ⚠ Problèmes détectés", ...scanProblems.value.map(p => `- ${p}`), "");
  } else if (scanResult.value) {
    lines.push("## ✅ Aucun problème critique", "");
  }
  await writeExport("diagnostic.md", lines.join("\n"));
}

// Init
loadTab("os");
</script>

<template>
  <div class="page-content">
    <!-- Header -->
    <div style="display:flex;align-items:center;justify-content:space-between;gap:12px;flex-wrap:wrap;margin-bottom:12px">
      <h2 style="margin:0;font-size:16px;font-weight:700">Diagnostic Système</h2>
      <div style="display:flex;gap:6px;flex-wrap:wrap">
        <NButton variant="ghost" size="sm" @click="exportJson"><FileJson :size="13" /> JSON</NButton>
        <NButton variant="ghost" size="sm" @click="exportTxt"><FileText :size="13" /> TXT</NButton>
        <NButton variant="ghost" size="sm" @click="exportMd"><FileCode :size="13" /> MD</NButton>
        <NButton variant="ghost" size="sm" @click="exportHtml"><FileDown :size="13" /> HTML</NButton>
        <NButton variant="ghost" size="sm" @click="openExportFolder"><FolderOpen :size="13" /> Exports</NButton>
      </div>
    </div>

    <NTabs :tabs="TABS" v-model="activeTab" wrap />

    <NCard style="margin-top:12px;padding:16px">
      <!-- Loading -->
      <div v-if="loadingTab === activeTab"
           style="display:flex;align-items:center;gap:10px;padding:24px 0;color:var(--text-secondary)">
        <NSpinner :size="18" /><span>Chargement {{ activeTab }}...</span>
      </div>
      <!-- Erreur -->
      <div v-else-if="tabError[activeTab]" style="color:var(--error);padding:16px 0;font-size:13px">
        ⚠ {{ tabError[activeTab] }}
        <NButton variant="ghost" size="sm" style="margin-left:12px" @click="refreshTab">Réessayer</NButton>
      </div>
      <!-- Contenu dispatché vers les composants enfants -->
      <template v-else>
        <DiagTabSystem
          v-if="activeTab === 'os' || activeTab === 'bios' || activeTab === 'mobo'"
          :tab="activeTab" :sysInfo="sysInfo" :biosInfo="biosInfo"
          :moboInfo="moboInfo" :osExtended="osExtended"
        />
        <DiagTabCPU
          v-else-if="activeTab === 'cpu'"
          :sysInfo="sysInfo" :cpuCache="cpuCache" :cpuExtended="cpuExtended"
        />
        <DiagTabGPU
          v-else-if="activeTab === 'gpu'"
          :gpuList="gpuList"
        />
        <DiagTabRAM
          v-else-if="activeTab === 'ram'"
          :ramData="ramData" :sysInfo="sysInfo"
        />
        <DiagTabStorage
          v-else-if="activeTab === 'disks' || activeTab === 'volumes'"
          :tab="activeTab" :storageList="storageList" :volumes="volumes" :smartData="smartData"
        />
        <DiagTabNetwork
          v-else-if="activeTab === 'network' || activeTab === 'connections'"
          :tab="activeTab" :networkAdapters="networkAdapters"
          :connections="connections" :wifiInfo="wifiInfo"
        />
        <DiagTabDevices
          v-else-if="activeTab === 'monitors' || activeTab === 'audio' || activeTab === 'usb' || activeTab === 'battery' || activeTab === 'power' || activeTab === 'printers'"
          :tab="activeTab" :monitors="monitors" :audioDevices="audioDevices"
          :usbDevices="usbDevices" :batteries="batteries"
          :powerPlans="powerPlans" :printers="printers"
        />
        <DiagTabSoftware
          v-else-if="activeTab === 'software' || activeTab === 'env'"
          :tab="activeTab" :softwareList="softwareList" :envVars="envVars"
        />
        <DiagTabProcesses
          v-else-if="activeTab === 'processes' || activeTab === 'services' || activeTab === 'startup' || activeTab === 'tasks'"
          :tab="activeTab" :processes="processes" :services="services"
          :startupPrograms="startupPrograms" :tasks="scheduledTasks"
          :onRefresh="refreshTab"
        />
        <DiagTabSecurity
          v-else-if="activeTab === 'security' || activeTab === 'license'"
          :tab="activeTab" :securityInfo="securityInfo"
          :licenseInfo="licenseInfo" :updatesHistory="updatesHistory"
        />
        <DiagTabUpdates
          v-else-if="activeTab === 'updates'"
          :updatesHistory="updatesHistory"
        />
        <DiagTabActivation
          v-else-if="activeTab === 'activation'"
        />
        <DiagTabAccounts   v-else-if="activeTab === 'comptes'" />
        <DiagTabFirewall   v-else-if="activeTab === 'parefeu'" />
        <DiagTabShares     v-else-if="activeTab === 'partages'" />
        <DiagTabRegistry   v-else-if="activeTab === 'registre'" />
        <DiagTabHistory      v-else-if="activeTab === 'historique'" />
        <DiagTabSysDrivers     v-else-if="activeTab === 'pilotes'" />
        <DiagTabDriverUpdater  v-else-if="activeTab === 'driver-update'" />
        <DiagTabCertificates v-else-if="activeTab === 'certificats'" />
        <DiagTabPerf         v-else-if="activeTab === 'performances'" />
        <DiagTabNetTools     v-else-if="activeTab === 'outils-reseau'" />
        <DiagTabRepair       v-else-if="activeTab === 'reparation'" />
        <DiagTabFolders
          v-else-if="activeTab === 'folders'"
          :folders="folders"
        />
        <DiagnosticToolsGrid
          v-else-if="activeTab === 'tools'"
        />
        <DiagTabScan
          v-else-if="activeTab === 'scan'"
          :scanning="scanning" :scanProgress="scanProgress" :scanStep="scanStep"
          :scanResult="scanResult" :scanProblems="scanProblems"
          :batteries="batteries" :onRunScan="runTotalScan"
        />
      </template>
    </NCard>
  </div>
</template>
