use serde::Serialize;
use std::process::Command;
use tauri::Emitter;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::error::NiTriTeError;
use super::extended_info::{get_bios_info, get_battery_extended};
use super::scan_supplement::{BitlockerVolume, StorageItem, collect_scan_supplement};
use super::scan_extra::{TopProcess, SuspTask, collect_scan_extra};

// === Types ===

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub bios_ok: bool,
    pub bios_info: Option<String>,
    pub battery_present: bool,
    pub battery_health: f64,
    pub battery_cycles: i64,
    pub suspicious_processes: Vec<SuspiciousProcess>,
    pub disk_usage: Vec<DiskUsage>,
    pub winget_upgradable: Vec<String>,
    pub choco_upgradable: Vec<String>,
    pub dism_status: String,
    pub sfc_status: String,
    pub scan_errors: Vec<String>,
    pub uptime_hours: f64,
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_usage_percent: f32,
    pub ram_total_gb: f64,
    pub ram_used_gb: f64,
    pub ram_usage_percent: f32,
    pub windows_version: String,
    pub windows_activation: String,
    pub firewall_enabled: bool,
    pub defender_enabled: bool,
    pub startup_count: usize,
    pub pending_reboot: bool,
    pub recent_errors: Vec<EventEntry>,
    pub network_ok: bool,
    pub open_ports: Vec<u16>,
    // Enrichis sécurité
    pub antivirus_installed: String,
    pub defender_definition_age_days: i64,
    pub last_bsod: String,
    pub last_update_days: i64,
    pub temp_folder_size_mb: f64,
    pub suspicious_services: Vec<SuspiciousService>,
    pub autorun_entries: Vec<AutorunEntry>,
    pub virtual_memory_total_mb: u64,
    pub virtual_memory_available_mb: u64,
    // Enrichis matériel & logiciels
    pub gpu_name: String,
    pub gpu_vram_mb: u64,
    pub screen_resolution: String,
    pub power_plan: String,
    pub installed_software_count: u32,
    pub services_running: u32,
    pub services_stopped: u32,
    pub network_adapters_summary: String,
    pub cpu_temperature: String,
    // Supplément : licences, BitLocker, composants
    pub windows_product_key: String,
    pub office_product_key: String,
    pub office_name: String,
    pub bitlocker_volumes: Vec<BitlockerVolume>,
    pub motherboard: String,
    pub ram_detail: String,
    pub cpu_threads: u32,
    pub cpu_frequency_ghz: f64,
    pub storage_items: Vec<StorageItem>,
    pub monitors_detail: String,
    // Extra : sécurité avancée, BIOS, processus, tâches
    pub tpm_present: bool,
    pub tpm_enabled: bool,
    pub tpm_version: String,
    pub secure_boot: bool,
    pub uac_level: String,
    pub rdp_enabled: bool,
    pub smbv1_enabled: bool,
    pub wmi_subscriptions: u32,
    pub local_admins: Vec<String>,
    pub guest_enabled: bool,
    pub system_manufacturer: String,
    pub system_model: String,
    pub system_serial: String,
    pub bios_manufacturer: String,
    pub bios_version: String,
    pub bios_date: String,
    pub license_type: String,
    pub last_restore_point: String,
    pub pending_updates_cached: i32,
    pub top_cpu: Vec<TopProcess>,
    pub top_ram: Vec<TopProcess>,
    pub susp_tasks_count: u32,
    pub susp_tasks: Vec<SuspTask>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuspiciousService {
    pub name: String,
    pub display_name: String,
    pub state: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AutorunEntry {
    pub name: String,
    pub path: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuspiciousProcess {
    pub name: String,
    pub pid: u32,
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiskUsage {
    pub drive: String,
    pub total_gb: f64,
    pub free_gb: f64,
    pub used_percent: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventEntry {
    pub time: String,
    pub source: String,
    pub message: String,
    pub level: String,
}

#[derive(Debug, Clone, Serialize)]
struct ScanProgressEvent {
    step: String,
    percent: u32,
}

// === Main ===

pub async fn run_total_scan(window: tauri::Window) -> Result<ScanResult, NiTriTeError> {
    let mut result = ScanResult {
        bios_ok: false, bios_info: None,
        battery_present: false, battery_health: 0.0, battery_cycles: -1,
        suspicious_processes: vec![], disk_usage: vec![],
        winget_upgradable: vec![], choco_upgradable: vec![],
        dism_status: String::new(), sfc_status: String::new(), scan_errors: vec![],
        uptime_hours: 0.0, cpu_name: String::new(), cpu_cores: 0, cpu_usage_percent: 0.0,
        ram_total_gb: 0.0, ram_used_gb: 0.0, ram_usage_percent: 0.0,
        windows_version: String::new(), windows_activation: String::new(),
        firewall_enabled: false, defender_enabled: false, startup_count: 0,
        pending_reboot: false, recent_errors: vec![], network_ok: false, open_ports: vec![],
        antivirus_installed: String::new(), defender_definition_age_days: -1,
        last_bsod: String::new(), last_update_days: -1, temp_folder_size_mb: 0.0,
        suspicious_services: vec![], autorun_entries: vec![],
        virtual_memory_total_mb: 0, virtual_memory_available_mb: 0,
        gpu_name: String::new(), gpu_vram_mb: 0, screen_resolution: String::new(),
        power_plan: String::new(), installed_software_count: 0,
        services_running: 0, services_stopped: 0, network_adapters_summary: String::new(),
        cpu_temperature: String::new(),
        windows_product_key: String::new(), office_product_key: String::new(),
        office_name: String::new(), bitlocker_volumes: vec![],
        motherboard: String::new(), ram_detail: String::new(),
        cpu_threads: 0, cpu_frequency_ghz: 0.0,
        storage_items: vec![], monitors_detail: String::new(),
        tpm_present: false, tpm_enabled: false, tpm_version: String::new(),
        secure_boot: false, uac_level: String::new(),
        rdp_enabled: false, smbv1_enabled: false, wmi_subscriptions: 0,
        local_admins: vec![], guest_enabled: false,
        system_manufacturer: String::new(), system_model: String::new(), system_serial: String::new(),
        bios_manufacturer: String::new(), bios_version: String::new(), bios_date: String::new(),
        license_type: String::new(), last_restore_point: String::new(), pending_updates_cached: -1,
        top_cpu: vec![], top_ram: vec![], susp_tasks_count: 0, susp_tasks: vec![],
    };

    // 1. BIOS + Battery (0→10%)
    emit_progress(&window, "BIOS & Batterie...", 5);
    match tokio::task::spawn_blocking(get_bios_info).await {
        Ok(Ok(info)) => {
            result.bios_ok = true;
            result.bios_info = Some(format!("{} {} ({})", info.manufacturer, info.version, info.release_date));
        }
        Ok(Err(e)) => result.scan_errors.push(format!("BIOS: {}", e)),
        Err(_) => {}
    }
    match tokio::task::spawn_blocking(get_battery_extended).await {
        Ok(Ok(Some(bat))) => {
            result.battery_present = true;
            result.battery_health = bat.health_percent;
            result.battery_cycles = bat.estimated_runtime_minutes;
        }
        _ => {}
    }

    // 2. Infos système (CPU, RAM, OS, uptime) (10→20%)
    emit_progress(&window, "Informations système...", 10);
    let sys_info = tokio::task::spawn_blocking(collect_system_summary).await.unwrap_or_default();
    result.uptime_hours = sys_info.uptime_hours;
    result.cpu_name = sys_info.cpu_name;
    result.cpu_cores = sys_info.cpu_cores;
    result.cpu_usage_percent = sys_info.cpu_usage;
    result.ram_total_gb = sys_info.ram_total;
    result.ram_used_gb = sys_info.ram_used;
    result.ram_usage_percent = sys_info.ram_pct;
    result.windows_version = sys_info.win_version;

    // 3. Activation Windows (20→25%)
    emit_progress(&window, "Vérification licence Windows...", 20);
    result.windows_activation = tokio::task::spawn_blocking(check_windows_activation)
        .await.unwrap_or_else(|_| "Inconnu".to_string());

    // 4. Sécurité : Firewall + Defender (25→35%)
    emit_progress(&window, "Sécurité — Firewall & Defender...", 25);
    let (fw, def) = tokio::task::spawn_blocking(check_security).await.unwrap_or((false, false));
    result.firewall_enabled = fw;
    result.defender_enabled = def;

    // 5. Processus suspects (35→50%)
    emit_progress(&window, "Analyse des processus...", 35);
    result.suspicious_processes = tokio::task::spawn_blocking(scan_suspicious_processes).await.unwrap_or_default();

    // 6. Espace disque (50→60%)
    emit_progress(&window, "Vérification espace disque...", 50);
    result.disk_usage = tokio::task::spawn_blocking(get_disk_usage).await.unwrap_or_default();

    // 7. Démarrage + Reboot pending (60→65%)
    emit_progress(&window, "Programmes au démarrage...", 60);
    let (sc, pr) = tokio::task::spawn_blocking(check_startup_and_reboot).await.unwrap_or((0, false));
    result.startup_count = sc;
    result.pending_reboot = pr;

    // 8. Événements d'erreur récents (65→70%)
    emit_progress(&window, "Journal d'événements...", 65);
    result.recent_errors = tokio::task::spawn_blocking(get_recent_errors).await.unwrap_or_default();

    // 9. WinGet upgrades (70→78%)
    emit_progress(&window, "Mises à jour WinGet...", 70);
    result.winget_upgradable = tokio::task::spawn_blocking(list_winget_upgradable).await.unwrap_or_default();

    // 10. Chocolatey (78→82%)
    emit_progress(&window, "Vérification Chocolatey...", 78);
    if is_chocolatey_installed() {
        result.choco_upgradable = tokio::task::spawn_blocking(list_choco_upgradable).await.unwrap_or_default();
    }

    // 11. Connectivité réseau (82→86%)
    emit_progress(&window, "Test connectivité réseau...", 82);
    result.network_ok = tokio::task::spawn_blocking(test_network).await.unwrap_or(false);

    // 12. Ports ouverts (86→88%)
    emit_progress(&window, "Vérification ports...", 86);
    result.open_ports = tokio::task::spawn_blocking(get_listening_ports).await.unwrap_or_default();

    // 13. DISM /CheckHealth (88→94%)
    emit_progress(&window, "Intégrité système (DISM)...", 88);
    result.dism_status = tokio::task::spawn_blocking(run_dism_check).await.unwrap_or_else(|_| "Erreur".to_string());

    // 14. SFC /verifyonly (88→93%)
    emit_progress(&window, "Vérification SFC...", 88);
    result.sfc_status = tokio::task::spawn_blocking(run_sfc_verify).await.unwrap_or_else(|_| "Erreur".to_string());

    // 15. Antivirus + Defender defs + BSOD + KB age + Temp size (93→97%)
    emit_progress(&window, "Antivirus, services & sécurité avancée...", 93);
    let adv = tokio::task::spawn_blocking(collect_advanced_security).await.unwrap_or_default();
    result.antivirus_installed = adv.antivirus;
    result.defender_definition_age_days = adv.def_age_days;
    result.last_bsod = adv.last_bsod;
    result.last_update_days = adv.last_update_days;
    result.temp_folder_size_mb = adv.temp_size_mb;
    result.virtual_memory_total_mb = adv.vmem_total_mb;
    result.virtual_memory_available_mb = adv.vmem_available_mb;

    // 16. Services suspects + Autoruns (94→97%)
    emit_progress(&window, "Services & autoruns suspects...", 94);
    result.suspicious_services = tokio::task::spawn_blocking(scan_suspicious_services).await.unwrap_or_default();
    result.autorun_entries = tokio::task::spawn_blocking(scan_autoruns).await.unwrap_or_default();

    // 17. Matériel étendu : GPU, résolution, plan d'alim, logiciels, services (97→100%)
    emit_progress(&window, "Matériel & logiciels...", 97);
    let hw = tokio::task::spawn_blocking(collect_hw_extended).await.unwrap_or_default();
    result.gpu_name = hw.gpu_name;
    result.gpu_vram_mb = hw.gpu_vram_mb;
    result.screen_resolution = hw.screen_resolution;
    result.power_plan = hw.power_plan;
    result.installed_software_count = hw.installed_software_count;
    result.services_running = hw.services_running;
    result.services_stopped = hw.services_stopped;
    result.network_adapters_summary = hw.network_adapters_summary;
    result.cpu_temperature = hw.cpu_temperature;

    // 18. Licences (clés), BitLocker, composants complets (97→100%)
    emit_progress(&window, "Licences, BitLocker & composants...", 97);
    let sup = tokio::task::spawn_blocking(collect_scan_supplement).await.unwrap_or_default();
    result.windows_product_key = sup.windows_product_key;
    result.office_product_key = sup.office_product_key;
    result.office_name = sup.office_name;
    result.bitlocker_volumes = sup.bitlocker_volumes;
    result.motherboard = sup.motherboard;
    result.ram_detail = sup.ram_detail;
    result.cpu_threads = sup.cpu_threads;
    result.cpu_frequency_ghz = sup.cpu_frequency_ghz;
    result.storage_items = sup.storage_items;
    result.monitors_detail = sup.monitors_detail;

    // 19. Extra : TPM, Secure Boot, UAC, RDP, SMBv1, admins, BIOS, processus top, tâches (97→100%)
    emit_progress(&window, "Sécurité avancée & identité système...", 98);
    let extra = tokio::task::spawn_blocking(collect_scan_extra).await.unwrap_or_default();
    result.tpm_present = extra.tpm_present;
    result.tpm_enabled = extra.tpm_enabled;
    result.tpm_version = extra.tpm_version;
    result.secure_boot = extra.secure_boot;
    result.uac_level = extra.uac_level;
    result.rdp_enabled = extra.rdp_enabled;
    result.smbv1_enabled = extra.smbv1_enabled;
    result.wmi_subscriptions = extra.wmi_subscriptions;
    result.local_admins = extra.local_admins;
    result.guest_enabled = extra.guest_enabled;
    result.system_manufacturer = extra.system_manufacturer;
    result.system_model = extra.system_model;
    result.system_serial = extra.system_serial;
    result.bios_manufacturer = extra.bios_manufacturer;
    result.bios_version = extra.bios_version;
    result.bios_date = extra.bios_date;
    result.license_type = extra.license_type;
    result.last_restore_point = extra.last_restore_point;
    result.pending_updates_cached = extra.pending_updates_cached;
    result.top_cpu = extra.top_cpu;
    result.top_ram = extra.top_ram;
    result.susp_tasks_count = extra.susp_tasks_count;
    result.susp_tasks = extra.susp_tasks;

    emit_progress(&window, "Scan terminé ✓", 100);
    Ok(result)
}

fn emit_progress(window: &tauri::Window, step: &str, percent: u32) {
    let _ = window.emit("scan-progress", ScanProgressEvent { step: step.to_string(), percent });
}

// === System Summary ===

#[derive(Default)]
struct SystemSummary {
    uptime_hours: f64, cpu_name: String, cpu_cores: u32,
    cpu_usage: f32, ram_total: f64, ram_used: f64, ram_pct: f32, win_version: String,
}

fn collect_system_summary() -> SystemSummary {
    use sysinfo::System;
    let mut sys = System::new_all();
    std::thread::sleep(std::time::Duration::from_millis(300));
    sys.refresh_cpu_usage();
    let uptime = System::uptime() as f64 / 3600.0;
    let cpus = sys.cpus();
    let cpu_name = cpus.first().map(|c| c.brand().to_string()).unwrap_or_default();
    let cpu_cores = cpus.len() as u32;
    let cpu_usage = sys.global_cpu_usage();
    let total = sys.total_memory() as f64 / 1_073_741_824.0;
    let used = sys.used_memory() as f64 / 1_073_741_824.0;
    let pct = if total > 0.0 { (used / total * 100.0) as f32 } else { 0.0 };
    let win_version = format!("{} {}", System::name().unwrap_or_else(|| "Windows".into()), System::os_version().unwrap_or_default());
    SystemSummary { uptime_hours: (uptime * 10.0).round() / 10.0, cpu_name, cpu_cores, cpu_usage, ram_total: total, ram_used: used, ram_pct: pct, win_version }
}

// === Activation ===

fn check_windows_activation() -> String {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command",
            r#"try { $s = Get-WmiObject -Query "SELECT LicenseStatus FROM SoftwareLicensingProduct WHERE PartialProductKey IS NOT NULL AND Name LIKE '*Windows*'" -ErrorAction Stop | Select-Object -First 1; switch($s.LicenseStatus){1{"Activé"}2{"Grâce OOB"}5{"Notification"}default{"Non activé"}} } catch { "Inconnu" }"#])
        .creation_flags(0x08000000).output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => "Inconnu".to_string(),
    }
}

// === Security ===

fn check_security() -> (bool, bool) {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command",
            r#"$fw = try { (Get-NetFirewallProfile -ErrorAction SilentlyContinue | Where-Object {$_.Enabled}).Count -gt 0 } catch { $false }; $def = try { (Get-MpComputerStatus -ErrorAction SilentlyContinue).RealTimeProtectionEnabled } catch { $false }; Write-Output "$fw,$def""#])
        .creation_flags(0x08000000).output();
    if let Ok(o) = out {
        let s = String::from_utf8_lossy(&o.stdout);
        let parts: Vec<&str> = s.trim().splitn(2, ',').collect();
        let fw = parts.first().map(|v| *v == "True").unwrap_or(false);
        let def = parts.get(1).map(|v| *v == "True").unwrap_or(false);
        return (fw, def);
    }
    (false, false)
}

// === Suspicious Processes ===

fn scan_suspicious_processes() -> Vec<SuspiciousProcess> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command",
            "Get-Process | Select-Object Id,ProcessName,Path | ConvertTo-Json -Depth 1 -Compress"])
        .output();
    let text = match output { Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(), Err(_) => return vec![] };
    let val: serde_json::Value = match serde_json::from_str(&text) { Ok(v) => v, Err(_) => return vec![] };
    let processes: Vec<serde_json::Value> = match val {
        serde_json::Value::Array(arr) => arr,
        obj @ serde_json::Value::Object(_) => vec![obj],
        _ => return vec![],
    };
    let safe_prefixes = [
        r"C:\Windows\System32", r"C:\Windows\SysWOW64", r"C:\Windows\",
        r"C:\Program Files\", r"C:\Program Files (x86)\", r"C:\ProgramData\Microsoft",
    ];
    let mut suspicious = Vec::new();
    for proc in &processes {
        let name = proc["ProcessName"].as_str().unwrap_or("").to_string();
        let pid = proc["Id"].as_u64().unwrap_or(0) as u32;
        let path = proc["Path"].as_str().unwrap_or("").to_string();
        if path.is_empty() || name.is_empty() { continue; }
        let path_up = path.to_uppercase();
        let is_safe = safe_prefixes.iter().any(|p| path_up.starts_with(&p.to_uppercase()));
        if !is_safe {
            let reason = if path_up.contains("TEMP") || path_up.contains("TMP") {
                "Exécuté depuis un dossier temporaire".to_string()
            } else if path_up.contains("APPDATA\\LOCAL") && !path_up.contains("PROGRAMS") {
                "Exécuté depuis AppData (hors Programs)".to_string()
            } else { "Chemin inhabituel".to_string() };
            suspicious.push(SuspiciousProcess { name, pid, path, reason });
        }
    }
    suspicious
}

// === Disk ===

fn get_disk_usage() -> Vec<DiskUsage> {
    use sysinfo::Disks;
    Disks::new_with_refreshed_list().iter().map(|d| {
        let total = d.total_space() as f64 / 1_073_741_824.0;
        let free = d.available_space() as f64 / 1_073_741_824.0;
        let used = total - free;
        let pct = if total > 0.0 { (used / total * 100.0) as f32 } else { 0.0 };
        DiskUsage {
            drive: d.mount_point().to_string_lossy().to_string(),
            total_gb: (total * 100.0).round() / 100.0,
            free_gb: (free * 100.0).round() / 100.0,
            used_percent: (pct * 10.0).round() / 10.0,
        }
    }).collect()
}

// === Startup + Reboot ===

fn check_startup_and_reboot() -> (usize, bool) {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command",
            r#"$sc = (Get-CimInstance -ClassName Win32_StartupCommand -ErrorAction SilentlyContinue | Measure-Object).Count; $rb = Test-Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Component Based Servicing\RebootPending" -ErrorAction SilentlyContinue; Write-Output "$sc,$rb""#])
        .creation_flags(0x08000000).output();
    if let Ok(o) = out {
        let s = String::from_utf8_lossy(&o.stdout);
        let parts: Vec<&str> = s.trim().splitn(2, ',').collect();
        let sc = parts.first().and_then(|v| v.parse().ok()).unwrap_or(0);
        let rb = parts.get(1).map(|v| *v == "True").unwrap_or(false);
        return (sc, rb);
    }
    (0, false)
}

// === Recent Errors ===

fn get_recent_errors() -> Vec<EventEntry> {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command",
            r#"Get-WinEvent -FilterHashtable @{LogName='System','Application';Level=1,2;StartTime=(Get-Date).AddHours(-48)} -MaxEvents 20 -ErrorAction SilentlyContinue | Select-Object TimeCreated,ProviderName,Message,LevelDisplayName | ConvertTo-Json -Depth 1 -Compress"#])
        .creation_flags(0x08000000).output();
    let text = match out { Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(), Err(_) => return vec![] };
    let val: Vec<serde_json::Value> = serde_json::from_str(text.trim())
        .unwrap_or_else(|_| serde_json::from_str(&format!("[{}]", text.trim())).unwrap_or_default());
    val.iter().map(|v| EventEntry {
        time: v["TimeCreated"].as_str().unwrap_or("").chars().take(16).collect(),
        source: v["ProviderName"].as_str().unwrap_or("").to_string(),
        message: v["Message"].as_str().unwrap_or("").chars().take(120).collect(),
        level: v["LevelDisplayName"].as_str().unwrap_or("").to_string(),
    }).collect()
}

// === Network ===

fn test_network() -> bool {
    Command::new("ping").args(["-n", "1", "-w", "2000", "8.8.8.8"])
        .creation_flags(0x08000000).output()
        .map(|o| o.status.success()).unwrap_or(false)
}

fn get_listening_ports() -> Vec<u16> {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command",
            r#"Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object {$_.LocalAddress -eq '0.0.0.0' -or $_.LocalAddress -eq '::'} | Select-Object -ExpandProperty LocalPort | Sort-Object -Unique | Select-Object -First 30 | ConvertTo-Json -Compress"#])
        .creation_flags(0x08000000).output();
    let text = match out { Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(), Err(_) => return vec![] };
    let val: Vec<serde_json::Value> = serde_json::from_str(text.trim()).unwrap_or_default();
    val.iter().filter_map(|v| v.as_u64().map(|p| p as u16)).collect()
}

// === WinGet / Choco ===

fn list_winget_upgradable() -> Vec<String> {
    let output = Command::new("winget").args(["upgrade", "--include-unknown"]).creation_flags(0x08000000).output();
    match output {
        Ok(o) => {
            let text = String::from_utf8_lossy(&o.stdout).to_string();
            text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('-') && !l.starts_with('N') && !l.starts_with("Name") && !l.contains("packages")).map(|l| l.trim().to_string()).take(20).collect()
        }
        Err(_) => vec![],
    }
}

fn is_chocolatey_installed() -> bool {
    Command::new("choco").arg("--version").creation_flags(0x08000000).output().map(|o| o.status.success()).unwrap_or(false)
}

fn list_choco_upgradable() -> Vec<String> {
    let output = Command::new("choco").args(["outdated", "-r"]).creation_flags(0x08000000).output();
    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).lines().filter(|l| !l.trim().is_empty()).map(|l| l.trim().to_string()).collect(),
        Err(_) => vec![],
    }
}

// === DISM / SFC ===

fn run_dism_check() -> String {
    let output = Command::new("DISM").args(["/Online", "/Cleanup-Image", "/CheckHealth"]).creation_flags(0x08000000).output();
    match output {
        Ok(o) => {
            let text = String::from_utf8_lossy(&o.stdout).to_string();
            if text.contains("No component store corruption") || text.to_lowercase().contains("aucune corruption") { "Sain — Aucune corruption détectée".to_string() }
            else if !o.status.success() { "Avertissement — Vérification requise".to_string() }
            else { "OK".to_string() }
        }
        Err(_) => "DISM non disponible".to_string(),
    }
}

fn run_sfc_verify() -> String {
    let output = Command::new("sfc").args(["/verifyonly"]).creation_flags(0x08000000).output();
    match output {
        Ok(o) => {
            let raw = String::from_utf8_lossy(&o.stdout).to_string();
            let text = if raw.contains('\0') {
                let bytes = o.stdout.clone();
                let utf16: Vec<u16> = bytes.chunks(2).map(|c| u16::from_le_bytes([c[0], c.get(1).copied().unwrap_or(0)])).collect();
                String::from_utf16_lossy(&utf16)
            } else { raw };
            if text.to_lowercase().contains("no integrity violations") { "Intègre — Aucune violation détectée".to_string() }
            else if text.to_lowercase().contains("found corrupt") { "Fichiers corrompus détectés".to_string() }
            else if !o.status.success() { "Avertissement — Relancer en administrateur".to_string() }
            else { "Vérification complète".to_string() }
        }
        Err(_) => "SFC non disponible".to_string(),
    }
}

// === Advanced Security ===

#[derive(Default)]
struct AdvancedSecurityInfo {
    antivirus: String,
    def_age_days: i64,
    last_bsod: String,
    last_update_days: i64,
    temp_size_mb: f64,
    vmem_total_mb: u64,
    vmem_available_mb: u64,
}

fn collect_advanced_security() -> AdvancedSecurityInfo {
    let ps = r#"
$out = [ordered]@{}

# Antivirus tiers via SecurityCenter2
try {
    $av = Get-WmiObject -Namespace root\SecurityCenter2 -Class AntiVirusProduct -ErrorAction SilentlyContinue |
          Where-Object { $_.displayName -notmatch "Windows Defender" } | Select-Object -First 1
    $out['AV'] = if ($av) { $av.displayName } else { "Windows Defender uniquement" }
} catch { $out['AV'] = "Inconnu" }

# Age definitions Defender (jours)
try {
    $mp = Get-MpComputerStatus -ErrorAction SilentlyContinue
    if ($mp) {
        $age = ((Get-Date) - $mp.AntivirusSignatureLastUpdated).Days
        $out['DefAge'] = $age
    } else { $out['DefAge'] = -1 }
} catch { $out['DefAge'] = -1 }

# Dernier BSOD
try {
    $bsod = Get-WinEvent -FilterHashtable @{LogName='System';Id=41} -MaxEvents 1 -ErrorAction SilentlyContinue |
            Select-Object -First 1
    $out['BSOD'] = if ($bsod) { $bsod.TimeCreated.ToString('yyyy-MM-dd HH:mm') } else { "Aucun BSOD recent" }
} catch { $out['BSOD'] = "Aucun BSOD recent" }

# Jours depuis dernier KB
try {
    $kb = Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 1
    if ($kb -and $kb.InstalledOn) {
        $out['KBDays'] = ((Get-Date) - $kb.InstalledOn).Days
    } else { $out['KBDays'] = -1 }
} catch { $out['KBDays'] = -1 }

# Taille dossiers temp
try {
    $t1 = (Get-ChildItem $env:TEMP -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    $t2 = (Get-ChildItem "$env:windir\Temp" -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    $out['TempMB'] = [math]::Round(($t1 + $t2) / 1MB, 1)
} catch { $out['TempMB'] = 0 }

# Memoire virtuelle (pagefile)
try {
    $cs = Get-WmiObject -Class Win32_ComputerSystem -ErrorAction SilentlyContinue
    $out['VMemTotal'] = [math]::Round($cs.TotalVirtualMemorySize / 1024)
    $out['VMemAvail'] = [math]::Round($cs.FreeVirtualMemory / 1024)
} catch { $out['VMemTotal'] = 0; $out['VMemAvail'] = 0 }

$out | ConvertTo-Json -Compress
"#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps])
        .creation_flags(0x08000000)
        .output();

    if let Ok(o) = output {
        let text = String::from_utf8_lossy(&o.stdout);
        let v: serde_json::Value = serde_json::from_str(text.trim()).unwrap_or_default();
        return AdvancedSecurityInfo {
            antivirus: v["AV"].as_str().unwrap_or("Inconnu").to_string(),
            def_age_days: v["DefAge"].as_i64().unwrap_or(-1),
            last_bsod: v["BSOD"].as_str().unwrap_or("").to_string(),
            last_update_days: v["KBDays"].as_i64().unwrap_or(-1),
            temp_size_mb: v["TempMB"].as_f64().unwrap_or(0.0),
            vmem_total_mb: v["VMemTotal"].as_u64().unwrap_or(0),
            vmem_available_mb: v["VMemAvail"].as_u64().unwrap_or(0),
        };
    }
    AdvancedSecurityInfo::default()
}

// === Suspicious Services ===

fn scan_suspicious_services() -> Vec<SuspiciousService> {
    let ps = r#"
Get-WmiObject -Class Win32_Service -ErrorAction SilentlyContinue |
Where-Object { $_.State -eq 'Running' -and $_.PathName -and
    $_.PathName -notmatch 'System32|SysWOW64|Microsoft|Windows\\' -and
    $_.StartMode -ne 'Disabled' } |
Select-Object Name, DisplayName, State, PathName |
Select-Object -First 20 |
ConvertTo-Json -Compress -Depth 1
"#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps])
        .creation_flags(0x08000000)
        .output();
    let text = match output { Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(), Err(_) => return vec![] };
    let val: serde_json::Value = serde_json::from_str(text.trim()).unwrap_or(serde_json::Value::Array(vec![]));
    let arr = match val {
        serde_json::Value::Array(a) => a,
        obj @ serde_json::Value::Object(_) => vec![obj],
        _ => return vec![],
    };
    arr.iter().map(|v| SuspiciousService {
        name: v["Name"].as_str().unwrap_or("").to_string(),
        display_name: v["DisplayName"].as_str().unwrap_or("").to_string(),
        state: v["State"].as_str().unwrap_or("").to_string(),
        path: v["PathName"].as_str().unwrap_or("").chars().take(100).collect(),
    }).collect()
}

// === Autoruns ===

fn scan_autoruns() -> Vec<AutorunEntry> {
    let ps = r#"
$entries = @()
$runKeys = @(
    'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run',
    'HKLM:\Software\Microsoft\Windows\CurrentVersion\Run',
    'HKCU:\Software\Microsoft\Windows\CurrentVersion\RunOnce',
    'HKLM:\Software\Microsoft\Windows\CurrentVersion\RunOnce'
)
foreach ($key in $runKeys) {
    try {
        $props = Get-ItemProperty -Path $key -ErrorAction SilentlyContinue
        if ($props) {
            $props.PSObject.Properties | Where-Object { $_.Name -notmatch '^PS' } | ForEach-Object {
                $val = $_.Value -replace '"',''
                if ($val -notmatch 'Windows|System32|Microsoft') {
                    $entries += [PSCustomObject]@{ Name=$_.Name; Path=$val; Location=$key -replace 'HKCU:','HKCU' -replace 'HKLM:','HKLM' }
                }
            }
        }
    } catch {}
}
$entries | Select-Object -First 25 | ConvertTo-Json -Compress -Depth 1
"#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps])
        .creation_flags(0x08000000)
        .output();
    let text = match output { Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(), Err(_) => return vec![] };
    let val: serde_json::Value = serde_json::from_str(text.trim()).unwrap_or(serde_json::Value::Array(vec![]));
    let arr = match val {
        serde_json::Value::Array(a) => a,
        obj @ serde_json::Value::Object(_) => vec![obj],
        _ => return vec![],
    };
    arr.iter().map(|v| AutorunEntry {
        name: v["Name"].as_str().unwrap_or("").to_string(),
        path: v["Path"].as_str().unwrap_or("").to_string(),
        location: v["Location"].as_str().unwrap_or("").to_string(),
    }).collect()
}

// === Matériel étendu (GPU, résolution, plan, logiciels, services) ===

#[derive(Default)]
struct HwExtended {
    gpu_name: String, gpu_vram_mb: u64, screen_resolution: String,
    power_plan: String, installed_software_count: u32,
    services_running: u32, services_stopped: u32,
    network_adapters_summary: String, cpu_temperature: String,
}

fn collect_hw_extended() -> HwExtended {
    let ps = r#"
$out = @{}
# GPU
try {
    $gpu = Get-CimInstance Win32_VideoController -ErrorAction SilentlyContinue | Select-Object -First 1
    $out.GpuName = if ($gpu) { [string]$gpu.Name } else { "" }
    $out.GpuVram = if ($gpu -and $gpu.AdapterRAM) { [long]$gpu.AdapterRAM } else { 0 }
    $out.Resolution = if ($gpu) { "$($gpu.CurrentHorizontalResolution)x$($gpu.CurrentVerticalResolution) @ $($gpu.CurrentRefreshRate)Hz" } else { "" }
} catch { $out.GpuName = ""; $out.GpuVram = 0; $out.Resolution = "" }
# Plan d'alimentation
try {
    $plan = powercfg /getactivescheme 2>$null
    if ($plan -match '\((.+)\)') { $out.PowerPlan = $matches[1] } else { $out.PowerPlan = $plan -replace '.*:\s*','' }
} catch { $out.PowerPlan = "" }
# Logiciels installés
try {
    $sw = (Get-ItemProperty "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*" -ErrorAction SilentlyContinue |
        Where-Object { $_.DisplayName -and $_.DisplayName -ne "" } | Measure-Object).Count
    $sw += (Get-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*" -ErrorAction SilentlyContinue |
        Where-Object { $_.DisplayName -and $_.DisplayName -ne "" } | Measure-Object).Count
    $out.SoftCount = $sw
} catch { $out.SoftCount = 0 }
# Services Running/Stopped
try {
    $svcs = Get-Service -ErrorAction SilentlyContinue
    $out.SvcRunning = ($svcs | Where-Object {$_.Status -eq 'Running'} | Measure-Object).Count
    $out.SvcStopped = ($svcs | Where-Object {$_.Status -eq 'Stopped'} | Measure-Object).Count
} catch { $out.SvcRunning = 0; $out.SvcStopped = 0 }
# Adaptateurs réseau actifs
try {
    $adapters = Get-NetAdapter -ErrorAction SilentlyContinue | Where-Object { $_.Status -eq 'Up' }
    $names = @()
    foreach ($a in $adapters) {
        $ip = (Get-NetIPAddress -InterfaceIndex $a.InterfaceIndex -AddressFamily IPv4 -ErrorAction SilentlyContinue | Select-Object -First 1).IPAddress
        $names += "$($a.Name) — $ip"
    }
    $out.NetSummary = $names -join " | "
} catch { $out.NetSummary = "" }
# Température CPU (via OpenHardwareMonitor WMI si dispo, sinon vide)
try {
    $temp = Get-WmiObject -Namespace "root\OpenHardwareMonitor" -Class Sensor -ErrorAction SilentlyContinue |
            Where-Object { $_.SensorType -eq 'Temperature' -and $_.Name -like '*CPU*' } |
            Select-Object -First 1
    $out.CpuTemp = if ($temp) { "$($temp.Value)°C" } else { "N/A" }
} catch { $out.CpuTemp = "N/A" }
$out | ConvertTo-Json -Compress
"#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps])
        .creation_flags(0x08000000)
        .output();
    if let Ok(o) = output {
        let text = String::from_utf8_lossy(&o.stdout);
        let v: serde_json::Value = serde_json::from_str(text.trim()).unwrap_or_default();
        return HwExtended {
            gpu_name: v["GpuName"].as_str().unwrap_or("").to_string(),
            gpu_vram_mb: v["GpuVram"].as_u64().unwrap_or(0) / 1_048_576,
            screen_resolution: v["Resolution"].as_str().unwrap_or("").to_string(),
            power_plan: v["PowerPlan"].as_str().unwrap_or("").trim().to_string(),
            installed_software_count: v["SoftCount"].as_u64().unwrap_or(0) as u32,
            services_running: v["SvcRunning"].as_u64().unwrap_or(0) as u32,
            services_stopped: v["SvcStopped"].as_u64().unwrap_or(0) as u32,
            network_adapters_summary: v["NetSummary"].as_str().unwrap_or("").to_string(),
            cpu_temperature: v["CpuTemp"].as_str().unwrap_or("N/A").to_string(),
        };
    }
    HwExtended::default()
}
