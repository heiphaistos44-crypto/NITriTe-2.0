use serde::Serialize;
use std::process::Command;

use crate::error::NiTriTeError;
use crate::utils::paths;

#[derive(Debug, Clone, Serialize)]
pub struct BackupManifest {
    pub timestamp: String,
    pub items: Vec<BackupItem>,
    pub total_items: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupItem {
    pub name: String,
    pub category: String,
    pub data: String,
    pub size_bytes: usize,
}

pub fn create_backup(selected_items: Vec<String>) -> Result<BackupManifest, NiTriTeError> {
    let mut items = Vec::new();
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    for item_id in &selected_items {
        match item_id.as_str() {
            "installed_apps" => {
                if let Ok(data) = collect_installed_apps() {
                    items.push(BackupItem { name: "Applications installees".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "drivers" => {
                if let Ok(data) = collect_drivers() {
                    items.push(BackupItem { name: "Pilotes systeme".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "network_config" => {
                if let Ok(data) = collect_network_config() {
                    items.push(BackupItem { name: "Configuration reseau".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "startup_programs" => {
                if let Ok(data) = collect_startup() {
                    items.push(BackupItem { name: "Programmes demarrage".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "env_variables" => {
                if let Ok(data) = collect_env_vars() {
                    items.push(BackupItem { name: "Variables d'environnement".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "firewall_rules" => {
                if let Ok(data) = collect_firewall_rules() {
                    items.push(BackupItem { name: "Regles pare-feu".into(), category: "Securite".into(), size_bytes: data.len(), data });
                }
            }
            "chrome_bookmarks" => {
                if let Ok(data) = collect_browser_bookmarks("Google\\Chrome") {
                    items.push(BackupItem { name: "Favoris Chrome".into(), category: "Navigateurs".into(), size_bytes: data.len(), data });
                }
            }
            "edge_bookmarks" => {
                if let Ok(data) = collect_browser_bookmarks("Microsoft\\Edge") {
                    items.push(BackupItem { name: "Favoris Edge".into(), category: "Navigateurs".into(), size_bytes: data.len(), data });
                }
            }
            "brave_bookmarks" => {
                if let Ok(data) = collect_browser_bookmarks("BraveSoftware\\Brave-Browser") {
                    items.push(BackupItem { name: "Favoris Brave".into(), category: "Navigateurs".into(), size_bytes: data.len(), data });
                }
            }
            "windows_license" => {
                if let Ok(data) = collect_windows_license() {
                    items.push(BackupItem { name: "Licence Windows".into(), category: "Licences".into(), size_bytes: data.len(), data });
                }
            }
            "bitlocker_keys" => {
                if let Ok(data) = collect_bitlocker_keys() {
                    items.push(BackupItem { name: "Cles BitLocker".into(), category: "Securite".into(), size_bytes: data.len(), data });
                }
            }
            "office_license" => {
                if let Ok(data) = collect_office_license() {
                    items.push(BackupItem { name: "Licence Office".into(), category: "Licences".into(), size_bytes: data.len(), data });
                }
            }
            "installed_fonts" => {
                if let Ok(data) = collect_installed_fonts() {
                    items.push(BackupItem { name: "Polices installees".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "scheduled_tasks" => {
                if let Ok(data) = collect_scheduled_tasks() {
                    items.push(BackupItem { name: "Taches planifiees".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "windows_features" => {
                if let Ok(data) = collect_windows_features() {
                    items.push(BackupItem { name: "Fonctionnalites Windows".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "folder_sizes" => {
                if let Ok(data) = collect_folder_sizes() {
                    items.push(BackupItem { name: "Taille des dossiers C:\\".into(), category: "Diagnostic".into(), size_bytes: data.len(), data });
                }
            }
            "desktop_files" => {
                if let Ok(data) = collect_desktop_files() {
                    items.push(BackupItem { name: "Fichiers Bureau".into(), category: "Utilisateur".into(), size_bytes: data.len(), data });
                }
            }
            "wifi_passwords" => {
                if let Ok(data) = collect_wifi_passwords() {
                    items.push(BackupItem { name: "Mots de passe WiFi".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "registry_export" => {
                if let Ok(data) = collect_registry_export() {
                    items.push(BackupItem { name: "Export registre (HKCU)".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "suspicious_processes" => {
                if let Ok(data) = collect_suspicious_processes() {
                    items.push(BackupItem { name: "Processus suspects".into(), category: "Securite".into(), size_bytes: data.len(), data });
                }
            }
            _ => {}
        }
    }

    let total_items = items.len();
    let manifest = BackupManifest { timestamp: timestamp.clone(), items, total_items };

    // Sauvegarder sur disque
    let backup_dir = paths::backups_dir();
    let backup_file = backup_dir.join(format!("backup_{}.json", timestamp));
    let json = serde_json::to_string_pretty(&manifest)?;
    std::fs::write(&backup_file, &json)?;

    Ok(manifest)
}

pub fn list_backups() -> Result<Vec<String>, NiTriTeError> {
    let backup_dir = paths::backups_dir();
    let mut backups = Vec::new();

    if backup_dir.exists() {
        for entry in std::fs::read_dir(&backup_dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".json") && name.starts_with("backup_") {
                backups.push(name);
            }
        }
    }

    backups.sort();
    backups.reverse();
    Ok(backups)
}

fn collect_installed_apps() -> Result<String, NiTriTeError> {
    let output = Command::new("winget").args(["list", "--accept-source-agreements"]).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_drivers() -> Result<String, NiTriTeError> {
    let output = Command::new("driverquery").args(["/v", "/fo", "csv"]).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_network_config() -> Result<String, NiTriTeError> {
    let output = Command::new("ipconfig").arg("/all").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_startup() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-CimInstance Win32_StartupCommand | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_env_vars() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "[Environment]::GetEnvironmentVariables('Machine') | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_firewall_rules() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-NetFirewallRule | Select-Object -First 50 DisplayName, Direction, Action, Enabled | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_browser_bookmarks(browser_subpath: &str) -> Result<String, NiTriTeError> {
    let local_app = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let bookmarks_path = std::path::PathBuf::from(&local_app)
        .join(browser_subpath)
        .join("User Data").join("Default").join("Bookmarks");

    if bookmarks_path.exists() {
        Ok(std::fs::read_to_string(bookmarks_path)?)
    } else {
        Err(NiTriTeError::System("Fichier bookmarks introuvable".into()))
    }
}

fn collect_windows_license() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "(Get-WmiObject SoftwareLicensingProduct | Where-Object { $_.PartialProductKey } | Select-Object Name, Description, PartialProductKey) | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_bitlocker_keys() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-BitLockerVolume -ErrorAction SilentlyContinue | Select-Object MountPoint, VolumeStatus, EncryptionPercentage, KeyProtector | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_office_license() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Office\\*\\*\\Registration\\*' -ErrorAction SilentlyContinue | Select-Object ProductName, DigitalProductID | ConvertTo-Json; cscript //nologo 'C:\\Program Files\\Microsoft Office\\Office16\\OSPP.VBS' /dstatus 2>$null"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_installed_fonts() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts' | Get-Member -MemberType NoteProperty | Select-Object Name).Name | Sort-Object | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_scheduled_tasks() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ScheduledTask | Where-Object {$_.State -ne 'Disabled'} | Select-Object TaskName, TaskPath, State, Description | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_windows_features() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-WindowsOptionalFeature -Online -ErrorAction SilentlyContinue | Where-Object {$_.State -eq 'Enabled'} | Select-Object FeatureName | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_folder_sizes() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ChildItem C:\\ -Directory -ErrorAction SilentlyContinue | ForEach-Object { $size = (Get-ChildItem $_.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum; [PSCustomObject]@{Folder=$_.Name; SizeMB=[math]::Round($size/1MB,1)} } | Sort-Object SizeMB -Descending | Select-Object -First 30 | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_desktop_files() -> Result<String, NiTriTeError> {
    let desktop = std::env::var("USERPROFILE").unwrap_or_default();
    let desktop_path = std::path::PathBuf::from(&desktop).join("Desktop");
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&desktop_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            files.push(format!("{} ({})", name, format_size(size)));
        }
    }
    Ok(files.join("\n"))
}

fn collect_wifi_passwords() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "(netsh wlan show profiles) | Select-String ':(.+)$' | ForEach-Object { $name = $_.Matches.Groups[1].Value.Trim(); $pass = (netsh wlan show profile name=\"$name\" key=clear 2>$null | Select-String 'Key Content\\s+:\\s+(.+)'); if($pass) { \"$name : $($pass.Matches.Groups[1].Value)\" } else { \"$name : (pas de mot de passe)\" } }"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_registry_export() -> Result<String, NiTriTeError> {
    // Export partiel du registre HKCU (Software uniquement, taille limitee)
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ItemProperty 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -ErrorAction SilentlyContinue | ConvertTo-Json; Get-ItemProperty 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders' -ErrorAction SilentlyContinue | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_suspicious_processes() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-Process | Where-Object { $_.Path -and $_.Path -notmatch 'Windows|Microsoft|System32|SysWOW64|Program Files' } | Select-Object ProcessName, Id, Path, @{N='MemMB';E={[math]::Round($_.WorkingSet64/1MB,1)}} | Sort-Object MemMB -Descending | Select-Object -First 30 | ConvertTo-Json"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}
