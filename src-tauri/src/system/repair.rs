use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SystemHealthStatus {
    pub dism_health: String,
    pub sfc_last_run: String,
    pub sfc_result: String,
    pub pending_reboot: bool,
    pub disk_errors: Vec<String>,
    pub windows_version: String,
    pub cbs_log_size_kb: u64,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RepairResult {
    pub command: String,
    pub success: bool,
    pub output: String,
    pub duration_secs: u64,
}

#[tauri::command]
pub fn check_system_health() -> SystemHealthStatus {
    let ps = r#"
$out = @{}
try {
    $dism = & dism /Online /Cleanup-Image /CheckHealth 2>&1 | Out-String
    if ($dism -match 'repairable') { $out.DismHealth = 'Repairable' }
    elseif ($dism -match 'No component store corruption') { $out.DismHealth = 'Healthy' }
    else { $out.DismHealth = ($dism.Trim())[0..99] -join '' }
} catch { $out.DismHealth = 'Unavailable' }

try {
    $cbsLog = 'C:\Windows\Logs\CBS\CBS.log'
    if (Test-Path $cbsLog) {
        $lines = Get-Content $cbsLog -Tail 300 -ErrorAction SilentlyContinue
        $sfcLine = $lines | Where-Object { $_ -match 'Windows Resource Protection' } | Select-Object -Last 1
        if ($sfcLine -match 'found corrupt files and successfully repaired') { $out.SfcResult = 'Repaired' }
        elseif ($sfcLine -match 'found corrupt files but was unable') { $out.SfcResult = 'Corrupt (not repaired)' }
        elseif ($sfcLine -match 'did not find any integrity violations') { $out.SfcResult = 'No violations' }
        else { $out.SfcResult = 'Not recently run' }
        $tsLine = $lines | Select-String '\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}' | Select-Object -Last 1
        $out.SfcLastRun = if ($tsLine) { $tsLine.Matches[0].Value } else { 'Unknown' }
        $out.CbsLogKb = [long]((Get-Item $cbsLog).Length / 1024)
    } else {
        $out.SfcResult = 'No CBS log'; $out.SfcLastRun = 'N/A'; $out.CbsLogKb = 0
    }
} catch { $out.SfcResult = 'Read error'; $out.SfcLastRun = 'N/A'; $out.CbsLogKb = 0 }

try {
    $rb = $false
    if (Test-Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Component Based Servicing\RebootPending') { $rb = $true }
    if (Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager' -Name PendingFileRenameOperations -ErrorAction SilentlyContinue) { $rb = $true }
    if (Test-Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Auto Update\RebootRequired') { $rb = $true }
    $out.PendingReboot = $rb
} catch { $out.PendingReboot = $false }

try {
    $wv = Get-WmiObject Win32_OperatingSystem -ErrorAction SilentlyContinue
    $out.WinVer = "$($wv.Caption) Build $($wv.BuildNumber)"
} catch { $out.WinVer = '' }

try {
    $de = @(Get-WinEvent -FilterHashtable @{LogName='System';Id=@(7,11,15,157);StartTime=(Get-Date).AddDays(-7)} -MaxEvents 10 -ErrorAction SilentlyContinue |
        ForEach-Object { "$($_.TimeCreated.ToString('yyyy-MM-dd HH:mm')) — $($_.Message.Substring(0,[math]::Min(80,$_.Message.Length)))" })
    $out.DiskErrors = $de
} catch { $out.DiskErrors = @() }

$out | ConvertTo-Json -Depth 3 -Compress
"#;

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps])
            .creation_flags(0x08000000)
            .output();

        if let Ok(o) = output {
            let text = String::from_utf8_lossy(&o.stdout);
            let v: serde_json::Value = match serde_json::from_str(text.trim()) {
                Ok(v) => v,
                Err(_) => return SystemHealthStatus::default(),
            };
            let disk_errors = v["DiskErrors"].as_array().map(|arr| {
                arr.iter().filter_map(|e| e.as_str().map(|s| s.to_string())).collect()
            }).unwrap_or_default();
            return SystemHealthStatus {
                dism_health: v["DismHealth"].as_str().unwrap_or("Unknown").to_string(),
                sfc_last_run: v["SfcLastRun"].as_str().unwrap_or("Unknown").to_string(),
                sfc_result: v["SfcResult"].as_str().unwrap_or("Unknown").to_string(),
                pending_reboot: v["PendingReboot"].as_bool().unwrap_or(false),
                disk_errors,
                windows_version: v["WinVer"].as_str().unwrap_or("").to_string(),
                cbs_log_size_kb: v["CbsLogKb"].as_u64().unwrap_or(0),
            };
        }
    }
    SystemHealthStatus::default()
}

fn repair_cmd_and_label(repair_type: &str) -> Option<(&'static str, String)> {
    match repair_type {
        // Réseau
        "flush_dns"       => Some(("ipconfig /flushdns",             "ipconfig /flushdns".to_string())),
        "winsock"         => Some(("netsh winsock reset",            "netsh winsock reset".to_string())),
        "ip_reset"        => Some(("netsh int ip reset",             "netsh int ip reset".to_string())),
        "register_dns"    => Some(("ipconfig /registerdns",          "ipconfig /registerdns".to_string())),
        "net_reset_all"   => Some(("Reset réseau complet", concat!(
            "ipconfig /release & ipconfig /flushdns & ",
            "netsh winsock reset & netsh int ip reset & ",
            "netsh int ipv6 reset & ipconfig /renew & ipconfig /registerdns"
        ).to_string())),
        "arp_flush"       => Some(("arp -d *",                       "arp -d *".to_string())),
        "reset_tcp"       => Some(("netsh int tcp reset",            "netsh int tcp reset & netsh int udp reset".to_string())),

        // Système Windows
        "sfc"             => Some(("SFC /scannow",                   "sfc /scannow".to_string())),
        "dism_scan"       => Some(("DISM /ScanHealth",               "dism /Online /Cleanup-Image /ScanHealth".to_string())),
        "dism_restore"    => Some(("DISM /RestoreHealth",            "dism /Online /Cleanup-Image /RestoreHealth".to_string())),
        "dism_startcomp"  => Some(("DISM /StartComponentCleanup",   "dism /Online /Cleanup-Image /StartComponentCleanup /ResetBase".to_string())),
        "repair_wmi"      => Some(("Réparer WMI",                   "winmgmt /resetrepository".to_string())),
        "gpupdate"        => Some(("gpupdate /force",                "gpupdate /force".to_string())),
        "bcdedit_check"   => Some(("BCDEdit (lecture)",             "bcdedit /enum all".to_string())),

        // Mise à jour & Sécurité
        "windows_update_reset" => Some(("Reset Windows Update", concat!(
            "net stop wuauserv & net stop cryptSvc & net stop bits & net stop msiserver & ",
            "ren C:\\Windows\\SoftwareDistribution SoftwareDistribution.old & ",
            "ren C:\\Windows\\System32\\catroot2 catroot2.old & ",
            "net start wuauserv & net start cryptSvc & net start bits & net start msiserver"
        ).to_string())),
        "defender_update" => Some(("Màj Defender",                  "powershell -Command Update-MpSignature".to_string())),
        "defender_scan"   => Some(("Scan rapide Defender",          "powershell -Command Start-MpScan -ScanType QuickScan".to_string())),
        "firewall_reset"  => Some(("Reset Pare-feu Windows",        "netsh advfirewall reset".to_string())),

        // Cache & Nettoyage
        "icon_cache"      => Some(("Rebuild cache icônes", concat!(
            "taskkill /F /IM explorer.exe & ",
            "del /F /Q \"%LOCALAPPDATA%\\Microsoft\\Windows\\Explorer\\iconcache*.db\" & ",
            "del /F /Q \"%LOCALAPPDATA%\\Microsoft\\Windows\\Explorer\\thumbcache*.db\" & ",
            "start explorer.exe"
        ).to_string())),
        "thumbnail_cache" => Some(("Vider cache miniatures", concat!(
            "powershell -Command \"",
            "Get-ChildItem -Path $env:LOCALAPPDATA\\Microsoft\\Windows\\Explorer\\thumbcache*.db | ",
            "Remove-Item -Force -ErrorAction SilentlyContinue\""
        ).to_string())),
        "temp_cleanup"    => Some(("Nettoyer fichiers temporaires", concat!(
            "del /F /S /Q \"%TEMP%\\*\" & del /F /S /Q \"C:\\Windows\\Temp\\*\" & ",
            "del /F /S /Q \"C:\\Windows\\Prefetch\\*.pf\""
        ).to_string())),
        "memory_dumps"    => Some(("Supprimer dumps mémoire", concat!(
            "del /F /Q \"C:\\Windows\\Minidump\\*.dmp\" & ",
            "del /F /Q \"C:\\Windows\\MEMORY.DMP\""
        ).to_string())),
        "clear_prefetch"  => Some(("Vider Prefetch",                "del /F /Q C:\\Windows\\Prefetch\\*.pf".to_string())),

        // Services
        "print_spooler"   => Some(("Reset Spouleur impression", concat!(
            "net stop spooler & ",
            "del /F /Q C:\\Windows\\System32\\spool\\PRINTERS\\* & ",
            "net start spooler"
        ).to_string())),
        "search_reindex"  => Some(("Réindexer Windows Search",     "powershell -Command \"net stop WSearch; Remove-Item 'C:\\ProgramData\\Microsoft\\Search\\Data\\Applications\\Windows\\Windows.edb' -Force -EA SilentlyContinue; net start WSearch\"".to_string())),
        "time_sync"       => Some(("Synchroniser l'heure", concat!(
            "net stop w32tm & net start w32tm & ",
            "w32tm /resync /force & w32tm /resync"
        ).to_string())),
        "wsreset"         => Some(("Réparer Windows Store",         "wsreset.exe".to_string())),

        // Disques
        "chkdsk_c"        => Some(("CHKDSK C: /scan",              "chkdsk C: /scan".to_string())),
        "defrag_c"        => Some(("Optimiser/Défragmenter C:",     "defrag C: /U /V".to_string())),

        // Event Logs
        "clear_event_logs" => Some(("Vider journaux d'événements",  "powershell -Command \"wevtutil el | ForEach-Object { wevtutil cl $_ }\"".to_string())),

        // Restauration
        "restore_point"   => Some(("Créer point de restauration",  "powershell -Command \"Checkpoint-Computer -Description 'NiTriTe Backup' -RestorePointType MODIFY_SETTINGS\"".to_string())),

        _ => None,
    }
}

#[tauri::command]
pub fn run_repair_command(repair_type: String) -> RepairResult {
    let Some((label, cmd)) = repair_cmd_and_label(&repair_type) else {
        return RepairResult { command: repair_type, ..Default::default() };
    };

    let start = std::time::Instant::now();
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("cmd")
            .args(["/C", &cmd])
            .creation_flags(0x08000000)
            .output();

        let duration = start.elapsed().as_secs();
        if let Ok(o) = output {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            let stderr = String::from_utf8_lossy(&o.stderr).to_string();
            let combined = if stderr.is_empty() { stdout } else { format!("{}\n{}", stdout, stderr) };
            return RepairResult {
                command: label.to_string(),
                success: o.status.success(),
                output: combined.chars().take(4000).collect(),
                duration_secs: duration,
            };
        }
    }
    RepairResult { command: label.to_string(), ..Default::default() }
}
