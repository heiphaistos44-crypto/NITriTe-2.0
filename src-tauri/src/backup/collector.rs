use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::error::NiTriTeError;
use crate::utils::paths;

#[derive(Debug, Clone, Serialize)]
pub struct BackupManifest {
    pub timestamp: String,
    pub items: Vec<BackupItem>,
    pub total_items: usize,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupItem {
    pub name: String,
    pub category: String,
    pub data: String,
    pub size_bytes: usize,
}

pub fn create_backup(selected_items: Vec<String>, format: String, custom_path: Option<String>) -> Result<BackupManifest, NiTriTeError> {
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
            "winget_export" => {
                if let Ok(data) = collect_winget_export() {
                    items.push(BackupItem { name: "WinGet Export JSON".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "network_shares" => {
                if let Ok(data) = collect_network_shares() {
                    items.push(BackupItem { name: "Partages reseau".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "hosts_file" => {
                if let Ok(data) = collect_hosts_file() {
                    items.push(BackupItem { name: "Fichier hosts".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "ssh_keys" => {
                if let Ok(data) = collect_ssh_keys() {
                    items.push(BackupItem { name: "Cles SSH".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "pip_packages" => {
                if let Ok(data) = collect_pip_packages() {
                    items.push(BackupItem { name: "Packages Python (pip)".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "vscode_extensions" => {
                if let Ok(data) = collect_vscode_extensions() {
                    items.push(BackupItem { name: "Extensions VSCode".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "wsl_config" => {
                if let Ok(data) = collect_wsl_config() {
                    items.push(BackupItem { name: "Config WSL".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "powershell_profile" => {
                if let Ok(data) = collect_powershell_profile() {
                    items.push(BackupItem { name: "Profil PowerShell".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "power_plans" => {
                if let Ok(data) = collect_power_plans() {
                    items.push(BackupItem { name: "Plans alimentation".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "printer_config" => {
                if let Ok(data) = collect_printer_config() {
                    items.push(BackupItem { name: "Imprimantes".into(), category: "Materiel".into(), size_bytes: data.len(), data });
                }
            }
            _ => {}
        }
    }

    let total_items = items.len();

    let backup_dir = if let Some(ref p) = custom_path {
        std::path::PathBuf::from(p)
    } else {
        paths::backups_dir()
    };
    std::fs::create_dir_all(&backup_dir)
        .map_err(|e| NiTriTeError::System(format!("Impossible de créer le dossier backups: {}", e)))?;

    let ext = match format.as_str() {
        "html" => "html",
        "md"   => "md",
        "txt"  => "txt",
        _      => "json",
    };
    let backup_file = backup_dir.join(format!("backup_{}.{}", timestamp, ext));
    let path_str = backup_file.to_string_lossy().to_string();

    let manifest = BackupManifest {
        timestamp: timestamp.clone(),
        items: items.clone(),
        total_items,
        path: path_str.clone(),
    };

    let content = match format.as_str() {
        "txt"  => render_txt(&manifest),
        "html" => render_html(&manifest),
        "md"   => render_md(&manifest),
        _      => serde_json::to_string_pretty(&manifest)
            .map_err(|e| NiTriTeError::System(format!("Sérialisation échouée: {}", e)))?,
    };

    std::fs::write(&backup_file, content.as_bytes())
        .map_err(|e| NiTriTeError::System(format!("Écriture fichier échouée ({}): {}", path_str, e)))?;

    Ok(manifest)
}

fn render_txt(m: &BackupManifest) -> String {
    let width = 72usize;
    let border = "═".repeat(width);
    let thin   = "─".repeat(width);

    let mut out = String::new();

    // En-tête principal
    out.push_str(&format!("╔{}╗\n", border));
    let title = "N i T r i T e  —  R a p p o r t  d e  S a u v e g a r d e";
    let pad = (width.saturating_sub(title.chars().count())) / 2;
    out.push_str(&format!("║{}{}{:>pad2$}║\n",
        " ".repeat(pad), title,
        " ",
        pad2 = width.saturating_sub(pad).saturating_sub(title.chars().count())));
    out.push_str(&format!("╠{}╣\n", border));
    out.push_str(&format!("║  Date      : {:<width$}║\n", m.timestamp, width = width - 15));
    out.push_str(&format!("║  Éléments  : {:<width$}║\n", m.total_items, width = width - 15));
    out.push_str(&format!("╚{}╝\n\n", border));

    // Grouper par catégorie
    let mut current_cat = String::new();
    for item in &m.items {
        // Nouveau groupe catégorie
        if item.category != current_cat {
            current_cat = item.category.clone();
            out.push_str(&format!("┌{}┐\n", thin));
            let cat_label = format!("  {}  ", current_cat.to_uppercase());
            let cat_pad = width.saturating_sub(cat_label.chars().count());
            out.push_str(&format!("│ {:<width$}│\n", format!("▶  {}", current_cat.to_uppercase()), width = width - 1));
            out.push_str(&format!("└{}┘\n", thin));
            let _ = cat_pad;
            let _ = cat_label;
        }

        // Bloc item
        out.push_str(&format!("\n  ┌─ {} {}\n", item.name, "─".repeat(width.saturating_sub(item.name.chars().count() + 4))));

        // Données — indenter chaque ligne (convertir JSON si nécessaire)
        let readable_data = json_to_readable(&item.data);
        for line in readable_data.lines().take(200) {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                out.push_str("  │\n");
            } else {
                // Couper les lignes trop longues
                let max_line = width - 4;
                if trimmed.len() <= max_line {
                    out.push_str(&format!("  │  {}\n", trimmed));
                } else {
                    let mut remaining = trimmed;
                    let mut first = true;
                    while !remaining.is_empty() {
                        // safe char boundary
                        let cut = remaining.char_indices().nth(max_line).map(|(i, _)| i).unwrap_or(remaining.len());
                        let (chunk, rest) = remaining.split_at(cut);
                        if first {
                            out.push_str(&format!("  │  {}\n", chunk));
                            first = false;
                        } else {
                            out.push_str(&format!("  │     {}\n", chunk));
                        }
                        remaining = rest;
                    }
                }
            }
        }
        if readable_data.lines().count() > 200 {
            out.push_str(&format!("  │  … ({} lignes supplémentaires tronquées)\n",
                readable_data.lines().count() - 200));
        }
        out.push_str(&format!("  └{}\n", "─".repeat(width - 2)));
    }

    // Pied de page
    out.push('\n');
    out.push_str(&format!("╔{}╗\n", border));
    out.push_str(&format!("║  Généré par NiTriTe {:<width$}║\n",
        format!("— {} éléments exportés", m.total_items), width = width - 22));
    out.push_str(&format!("╚{}╝\n", border));

    out
}

fn render_md(m: &BackupManifest) -> String {
    let mut out = format!("# NiTriTe — Rapport de Sauvegarde\n\n**Date :** `{}`  \n**Total :** {} éléments\n\n---\n\n",
        m.timestamp, m.total_items);
    let mut current_cat = String::new();
    for item in &m.items {
        if item.category != current_cat {
            out.push_str(&format!("## {}\n\n", item.category));
            current_cat = item.category.clone();
        }
        out.push_str(&format!("### {}\n\n```\n{}\n```\n\n", item.name, json_to_readable(&item.data)));
    }
    out
}

fn render_html(m: &BackupManifest) -> String {
    let mut rows = String::new();
    let mut current_cat = String::new();
    for item in &m.items {
        if item.category != current_cat {
            rows.push_str(&format!("<tr><th colspan='2' class='cat'>{}</th></tr>", html_escape(&item.category)));
            current_cat = item.category.clone();
        }
        rows.push_str(&format!(
            "<tr><td class='name'>{}</td><td class='data'><pre>{}</pre></td></tr>",
            html_escape(&item.name), html_escape(&item.data)
        ));
    }
    format!(r#"<!DOCTYPE html>
<html lang="fr"><head><meta charset="UTF-8">
<title>NiTriTe Backup {ts}</title>
<style>
  body{{font-family:system-ui,sans-serif;background:#0f0f10;color:#e5e5e5;margin:0;padding:20px}}
  h1{{color:#f97316;border-bottom:2px solid #f97316;padding-bottom:8px}}
  .meta{{color:#888;font-size:13px;margin-bottom:20px}}
  table{{width:100%;border-collapse:collapse;background:#1a1a1f;border-radius:8px;overflow:hidden}}
  th,td{{padding:10px 14px;text-align:left;border-bottom:1px solid #2e2e33}}
  .cat{{background:#1e1e28;color:#f97316;font-size:12px;text-transform:uppercase;letter-spacing:.08em}}
  .name{{color:#a1a1aa;font-weight:600;white-space:nowrap;width:200px}}
  .data pre{{margin:0;font-size:12px;color:#e5e5e5;white-space:pre-wrap;word-break:break-all;max-height:200px;overflow:auto}}
</style></head><body>
<h1>NiTriTe — Rapport de Sauvegarde</h1>
<p class="meta">Date : {ts} &nbsp;|&nbsp; {total} éléments exportés</p>
<table>{rows}</table>
</body></html>"#,
        ts = html_escape(&m.timestamp),
        total = m.total_items,
        rows = rows
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupEntryInfo {
    pub filename: String,
    pub date: String,
    pub size: String,
    pub items_count: usize,
}

pub fn list_backups() -> Result<Vec<BackupEntryInfo>, NiTriTeError> {
    let backup_dir = paths::backups_dir();
    let mut backups = Vec::new();

    if backup_dir.exists() {
        for entry in std::fs::read_dir(&backup_dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            let is_backup = name.starts_with("backup_")
                && (name.ends_with(".json") || name.ends_with(".txt")
                    || name.ends_with(".html") || name.ends_with(".md"));
            if !is_backup { continue; }

            let meta = entry.metadata().unwrap_or_else(|_| {
                std::fs::metadata(entry.path()).unwrap()
            });
            let size_bytes = meta.len();
            let size = if size_bytes < 1_048_576 {
                format!("{:.1} Ko", size_bytes as f64 / 1024.0)
            } else {
                format!("{:.1} Mo", size_bytes as f64 / 1_048_576.0)
            };

            // Parse date from filename: backup_2026-03-09_14-30-00.txt
            let stem = name.strip_prefix("backup_").unwrap_or(&name);
            let stem = stem.split('.').next().unwrap_or(stem);
            let date = stem.replace('_', " ").replace('-', "/");

            backups.push(BackupEntryInfo { filename: name, date, size, items_count: 0 });
        }
    }

    backups.sort_by(|a, b| b.filename.cmp(&a.filename));
    Ok(backups)
}

fn collect_installed_apps() -> Result<String, NiTriTeError> {
    let output = Command::new("winget").args(["list", "--accept-source-agreements"]).creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_drivers() -> Result<String, NiTriTeError> {
    let output = Command::new("driverquery").args(["/v", "/fo", "csv"]).creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_network_config() -> Result<String, NiTriTeError> {
    let output = Command::new("ipconfig").arg("/all").creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_startup() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-CimInstance Win32_StartupCommand | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_env_vars() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "[Environment]::GetEnvironmentVariables('Machine') | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_firewall_rules() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-NetFirewallRule | Select-Object -First 50 DisplayName, Direction, Action, Enabled | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
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
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_bitlocker_keys() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-BitLockerVolume -ErrorAction SilentlyContinue | Select-Object MountPoint, VolumeStatus, EncryptionPercentage, KeyProtector | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_office_license() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Office\\*\\*\\Registration\\*' -ErrorAction SilentlyContinue | Select-Object ProductName, DigitalProductID | ConvertTo-Json; cscript //nologo 'C:\\Program Files\\Microsoft Office\\Office16\\OSPP.VBS' /dstatus 2>$null"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_installed_fonts() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts' | Get-Member -MemberType NoteProperty | Select-Object Name).Name | Sort-Object | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_scheduled_tasks() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ScheduledTask | Where-Object {$_.State -ne 'Disabled'} | Select-Object TaskName, TaskPath, State, Description | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_windows_features() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-WindowsOptionalFeature -Online -ErrorAction SilentlyContinue | Where-Object {$_.State -eq 'Enabled'} | Select-Object FeatureName | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_folder_sizes() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ChildItem C:\\ -Directory -ErrorAction SilentlyContinue | ForEach-Object { $size = (Get-ChildItem $_.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum; [PSCustomObject]@{Folder=$_.Name; SizeMB=[math]::Round($size/1MB,1)} } | Sort-Object SizeMB -Descending | Select-Object -First 30 | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
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
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_registry_export() -> Result<String, NiTriTeError> {
    // Export partiel du registre HKCU (Software uniquement, taille limitee)
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ItemProperty 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -ErrorAction SilentlyContinue | ConvertTo-Json; Get-ItemProperty 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders' -ErrorAction SilentlyContinue | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_suspicious_processes() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-Process | Where-Object { $_.Path -and $_.Path -notmatch 'Windows|Microsoft|System32|SysWOW64|Program Files' } | Select-Object ProcessName, Id, Path, @{N='MemMB';E={[math]::Round($_.WorkingSet64/1MB,1)}} | Sort-Object MemMB -Descending | Select-Object -First 30 | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
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

/// Convertit du JSON en texte lisible pour les formats .txt et .md
fn json_to_readable(s: &str) -> String {
    let trimmed = s.trim();
    if !trimmed.starts_with('{') && !trimmed.starts_with('[') {
        return s.to_string();
    }
    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(v) => fmt_val(&v, 0),
        Err(_) => s.to_string(),
    }
}

fn fmt_val(v: &serde_json::Value, depth: usize) -> String {
    let pad = "  ".repeat(depth);
    match v {
        serde_json::Value::Array(arr) => arr
            .iter()
            .enumerate()
            .map(|(i, item)| match item {
                serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                    format!("{}[{}]\n{}", pad, i + 1, fmt_val(item, depth))
                }
                _ => fmt_val(item, depth),
            })
            .collect::<Vec<_>>()
            .join("\n"),
        serde_json::Value::Object(map) => map
            .iter()
            .filter(|(_, v)| !v.is_null())
            .map(|(k, v)| {
                let val = match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Number(n) => n.to_string(),
                    other => fmt_val(other, depth + 1),
                };
                format!("{}  {:<32} {}", pad, format!("{}:", k), val)
            })
            .collect::<Vec<_>>()
            .join("\n"),
        serde_json::Value::String(s) => format!("{}{}", pad, s),
        serde_json::Value::Null => format!("{}(null)", pad),
        _ => format!("{}{}", pad, v),
    }
}

fn collect_winget_export() -> Result<String, NiTriTeError> {
    let tmp = std::env::temp_dir().join("nitrite_winget_export.json");
    let _ = Command::new("winget")
        .args(["export", "-o", tmp.to_str().unwrap_or(""), "--accept-source-agreements"])
        .creation_flags(0x08000000)
        .output();
    if tmp.exists() {
        let content = std::fs::read_to_string(&tmp)?;
        let _ = std::fs::remove_file(&tmp);
        Ok(content)
    } else {
        Err(NiTriTeError::System("winget export échoué".into()))
    }
}

fn collect_network_shares() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-SmbShare | Select-Object Name, Path, Description | Format-Table -AutoSize | Out-String"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_hosts_file() -> Result<String, NiTriTeError> {
    let hosts = std::path::Path::new(r"C:\Windows\System32\drivers\etc\hosts");
    if hosts.exists() {
        Ok(std::fs::read_to_string(hosts)?)
    } else {
        Err(NiTriTeError::System("Fichier hosts introuvable".into()))
    }
}

fn collect_ssh_keys() -> Result<String, NiTriTeError> {
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    let ssh_dir = std::path::PathBuf::from(&home).join(".ssh");
    let mut lines = vec![format!("Dossier : {}", ssh_dir.display())];
    if ssh_dir.exists() {
        for entry in std::fs::read_dir(&ssh_dir).into_iter().flatten().flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            lines.push(format!("  {} ({})", name, format_size(size)));
        }
    } else {
        lines.push("Dossier .ssh introuvable".into());
    }
    Ok(lines.join("\n"))
}

fn collect_pip_packages() -> Result<String, NiTriTeError> {
    let output = Command::new("pip")
        .args(["freeze"])
        .creation_flags(0x08000000)
        .output()
        .or_else(|_| Command::new("pip3").args(["freeze"]).creation_flags(0x08000000).output())?;
    let s = String::from_utf8_lossy(&output.stdout).to_string();
    if s.trim().is_empty() {
        Err(NiTriTeError::System("pip non disponible ou aucun package installé".into()))
    } else {
        Ok(s)
    }
}

fn collect_vscode_extensions() -> Result<String, NiTriTeError> {
    let output = Command::new("code")
        .args(["--list-extensions"])
        .creation_flags(0x08000000)
        .output()?;
    let s = String::from_utf8_lossy(&output.stdout).to_string();
    if s.trim().is_empty() {
        Err(NiTriTeError::System("VSCode non disponible ou aucune extension".into()))
    } else {
        Ok(s)
    }
}

fn collect_wsl_config() -> Result<String, NiTriTeError> {
    let output = Command::new("wsl")
        .args(["--list", "--verbose"])
        .creation_flags(0x08000000)
        .output()?;
    let mut content = String::from_utf8_lossy(&output.stdout).to_string();
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    let wslconfig = std::path::PathBuf::from(&home).join(".wslconfig");
    if wslconfig.exists() {
        if let Ok(cfg) = std::fs::read_to_string(&wslconfig) {
            content.push_str("\n\n--- .wslconfig ---\n");
            content.push_str(&cfg);
        }
    }
    Ok(content)
}

fn collect_powershell_profile() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "if (Test-Path $PROFILE) { \"=== \" + $PROFILE + \" ===\"; Get-Content $PROFILE } else { 'Profil inexistant : ' + $PROFILE }"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_power_plans() -> Result<String, NiTriTeError> {
    let output = Command::new("powercfg")
        .args(["/list"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_printer_config() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-Printer | Select-Object Name, PortName, DriverName, PrinterStatus | Format-Table -AutoSize | Out-String"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
