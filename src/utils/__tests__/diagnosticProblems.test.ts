/**
 * Tests unitaires pour la logique computeProblems (détection de problèmes scan).
 * Ces fonctions sont extraites du composable pour être testables de façon isolée.
 */
import { describe, it, expect } from "vitest";
import type { ScanResult } from "@/types/diagnostic";

// Fonction extraite pour test — même logique que dans useDiagnosticExport
function computeProblems(sr: ScanResult, batteries: { battery_health_percent: number; cycle_count: number; name: string }[] = []): string[] {
  const p: string[] = [];
  if (!sr.firewall_enabled)   p.push("🛡 Pare-feu Windows désactivé");
  if (!sr.defender_enabled)   p.push("🛡 Windows Defender (temps réel) désactivé ou inconnu");
  if (!sr.network_ok)         p.push("🌐 Pas de connectivité internet (8.8.8.8 injoignable)");
  if (sr.pending_reboot)      p.push("🔄 Redémarrage Windows en attente");
  if (sr.suspicious_processes.length > 0)
    p.push(`⚠ ${sr.suspicious_processes.length} processus suspect(s) hors chemins sécurisés`);
  if (sr.winget_upgradable.length > 0)
    p.push(`🔄 ${sr.winget_upgradable.length} mise(s) à jour WinGet disponible(s)`);
  for (const d of sr.disk_usage) {
    if (d.used_percent > 90) p.push(`💾 Disque ${d.drive}: espace critique (${d.used_percent.toFixed(0)}%)`);
    else if (d.used_percent > 80) p.push(`💾 Disque ${d.drive}: espace faible (${d.used_percent.toFixed(0)}%)`);
  }
  if (sr.ram_usage_percent > 85) p.push(`🧠 RAM critique: ${sr.ram_usage_percent.toFixed(0)}% utilisé`);
  for (const b of batteries) {
    if (b.battery_health_percent > 0 && b.battery_health_percent < 80)
      p.push(`🔋 Batterie "${b.name}": santé faible (${b.battery_health_percent.toFixed(0)}%)`);
    if (b.cycle_count > 400)
      p.push(`🔋 Batterie "${b.name}": ${b.cycle_count} cycles — remplacement recommandé`);
  }
  if (sr.last_update_days > 60) p.push(`🔄 Dernier KB Windows: il y a ${sr.last_update_days} jours`);
  if (sr.temp_folder_size_mb > 2048) p.push(`🗑 Fichiers temporaires volumineux: ${(sr.temp_folder_size_mb / 1024).toFixed(1)} GB`);
  if (sr.last_bsod && !sr.last_bsod.includes("Aucun")) p.push(`💥 Dernier BSOD détecté: ${sr.last_bsod}`);
  return p;
}

function makeScan(overrides: Partial<ScanResult> = {}): ScanResult {
  return {
    bios_ok: true, bios_info: null,
    battery_present: false, battery_health: 100, battery_cycles: 0,
    suspicious_processes: [], disk_usage: [], winget_upgradable: [], choco_upgradable: [],
    dism_status: "OK", sfc_status: "OK", scan_errors: [],
    uptime_hours: 2, cpu_name: "Intel i9", cpu_cores: 16, cpu_usage_percent: 20,
    ram_total_gb: 32, ram_used_gb: 8, ram_usage_percent: 25,
    windows_version: "11", windows_activation: "Activé",
    firewall_enabled: true, defender_enabled: true,
    startup_count: 5, pending_reboot: false,
    recent_errors: [], network_ok: true, open_ports: [],
    antivirus_installed: "Defender", defender_definition_age_days: 1,
    last_bsod: "Aucun BSOD", last_update_days: 5, temp_folder_size_mb: 100,
    suspicious_services: [], autorun_entries: [],
    virtual_memory_total_mb: 65536, virtual_memory_available_mb: 32768,
    gpu_name: "RTX 4090", gpu_vram_mb: 24576, screen_resolution: "2560x1440",
    power_plan: "Haute performance", installed_software_count: 120,
    services_running: 80, services_stopped: 20,
    network_adapters_summary: "OK", cpu_temperature: "45°C",
    windows_product_key: "", office_product_key: "", office_name: "",
    bitlocker_volumes: [], motherboard: "", ram_detail: "",
    cpu_threads: 32, cpu_frequency_ghz: 3.8, storage_items: [], monitors_detail: "",
    tpm_present: true, tpm_enabled: true, tpm_version: "2.0",
    secure_boot: true, uac_level: "2", rdp_enabled: false,
    smbv1_enabled: false, wmi_subscriptions: 0,
    local_admins: ["Momo"], guest_enabled: false,
    system_manufacturer: "ASUS", system_model: "ProArt", system_serial: "ABC123",
    bios_manufacturer: "AMI", bios_version: "3702", bios_date: "2024-01",
    license_type: "OEM", last_restore_point: "2026-03-30", pending_updates_cached: 0,
    top_cpu: [], top_ram: [], susp_tasks_count: 0, susp_tasks: [],
    ...overrides,
  };
}

describe("computeProblems — détection de problèmes système", () => {
  it("retourne [] si tout est sain", () => {
    expect(computeProblems(makeScan())).toHaveLength(0);
  });

  it("détecte pare-feu désactivé", () => {
    const p = computeProblems(makeScan({ firewall_enabled: false }));
    expect(p).toContain("🛡 Pare-feu Windows désactivé");
  });

  it("détecte Defender désactivé", () => {
    const p = computeProblems(makeScan({ defender_enabled: false }));
    expect(p.some(x => x.includes("Defender"))).toBe(true);
  });

  it("détecte pas de connectivité", () => {
    const p = computeProblems(makeScan({ network_ok: false }));
    expect(p.some(x => x.includes("connectivité"))).toBe(true);
  });

  it("détecte reboot en attente", () => {
    const p = computeProblems(makeScan({ pending_reboot: true }));
    expect(p.some(x => x.includes("Redémarrage"))).toBe(true);
  });

  it("détecte disque critique (>90%)", () => {
    const p = computeProblems(makeScan({
      disk_usage: [{ drive: "C:", total_gb: 500, free_gb: 45, used_percent: 91 }],
    }));
    expect(p.some(x => x.includes("critique"))).toBe(true);
  });

  it("détecte disque faible (>80% mais <90%)", () => {
    const p = computeProblems(makeScan({
      disk_usage: [{ drive: "D:", total_gb: 200, free_gb: 35, used_percent: 83 }],
    }));
    expect(p.some(x => x.includes("faible"))).toBe(true);
  });

  it("détecte RAM critique (>85%)", () => {
    const p = computeProblems(makeScan({ ram_usage_percent: 92 }));
    expect(p.some(x => x.includes("RAM critique"))).toBe(true);
  });

  it("ne signale pas RAM si <85%", () => {
    const p = computeProblems(makeScan({ ram_usage_percent: 70 }));
    expect(p.some(x => x.includes("RAM"))).toBe(false);
  });

  it("détecte batterie avec santé faible (<80%)", () => {
    const p = computeProblems(makeScan(), [{ name: "Batterie principale", battery_health_percent: 65, cycle_count: 200 }]);
    expect(p.some(x => x.includes("santé faible"))).toBe(true);
  });

  it("détecte batterie avec trop de cycles (>400)", () => {
    const p = computeProblems(makeScan(), [{ name: "Batterie principale", battery_health_percent: 90, cycle_count: 450 }]);
    expect(p.some(x => x.includes("cycles"))).toBe(true);
  });

  it("détecte MAJ Windows trop anciennes (>60 jours)", () => {
    const p = computeProblems(makeScan({ last_update_days: 75 }));
    expect(p.some(x => x.includes("Dernier KB"))).toBe(true);
  });

  it("détecte dossier TEMP volumineux (>2GB)", () => {
    const p = computeProblems(makeScan({ temp_folder_size_mb: 3500 }));
    expect(p.some(x => x.includes("temporaires"))).toBe(true);
  });

  it("détecte BSOD récent", () => {
    const p = computeProblems(makeScan({ last_bsod: "MEMORY_MANAGEMENT — 2026-03-30" }));
    expect(p.some(x => x.includes("BSOD"))).toBe(true);
  });

  it("compte N processus suspects", () => {
    const p = computeProblems(makeScan({
      suspicious_processes: [
        { name: "evil.exe", pid: 1337, path: "C:\\Temp", reason: "hors chemin sécurisé" },
        { name: "sus.exe", pid: 666, path: "C:\\Temp", reason: "hors chemin sécurisé" },
      ],
    }));
    expect(p.some(x => x.includes("2 processus suspect"))).toBe(true);
  });

  it("détecte les mises à jour WinGet disponibles", () => {
    const p = computeProblems(makeScan({
      winget_upgradable: [
        { name: "Firefox", id: "Mozilla.Firefox", current_version: "120", available_version: "125" },
      ],
    }));
    expect(p.some(x => x.includes("WinGet"))).toBe(true);
  });

  it("accumule plusieurs problèmes simultanément", () => {
    const p = computeProblems(makeScan({
      firewall_enabled: false,
      defender_enabled: false,
      pending_reboot: true,
      ram_usage_percent: 90,
    }));
    expect(p.length).toBeGreaterThanOrEqual(4);
  });
});
