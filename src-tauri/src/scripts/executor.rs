use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use tauri::Emitter;

use crate::error::NiTriTeError;

#[derive(Debug, Clone, Serialize)]
pub struct ScriptResult {
    pub success: bool,
    pub output: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScriptEntry {
    pub name: String,
    pub description: String,
    pub category: String,
    pub script_type: String, // "cmd" | "powershell"
    pub content: String,
    pub requires_admin: bool,
}

pub fn execute_script(
    content: &str,
    script_type: &str,
    window: &tauri::Window,
) -> Result<ScriptResult, NiTriTeError> {
    let (cmd, args) = match script_type {
        "powershell" => ("powershell", vec!["-NoProfile", "-Command", content]),
        _ => ("cmd", vec!["/C", content]),
    };

    let mut child = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()?;

    let mut output_text = String::new();

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            output_text.push_str(&line);
            output_text.push('\n');
            let _ = window.emit("script-output", &line);
        }
    }

    let status = child.wait()?;

    Ok(ScriptResult {
        success: status.success(),
        output: output_text,
        exit_code: status.code().unwrap_or(-1),
    })
}

pub fn get_builtin_scripts() -> Vec<ScriptEntry> {
    vec![
        // === Nettoyage ===
        script("Nettoyer fichiers temp", "Supprime les fichiers temporaires Windows", "Nettoyage", "cmd",
            "del /q /f /s %TEMP%\\* 2>nul & del /q /f /s C:\\Windows\\Temp\\* 2>nul", false),
        script("Vider prefetch", "Supprime les fichiers prefetch", "Nettoyage", "cmd",
            "del /q /f /s C:\\Windows\\Prefetch\\* 2>nul", true),
        script("Vider cache thumbnails", "Reinitialise le cache des miniatures", "Nettoyage", "cmd",
            "del /q /f /s %LOCALAPPDATA%\\Microsoft\\Windows\\Explorer\\thumbcache_*.db 2>nul", false),
        script("Vider fichiers recents", "Supprime l'historique des fichiers recents", "Nettoyage", "cmd",
            "del /q /f %APPDATA%\\Microsoft\\Windows\\Recent\\* 2>nul", false),
        script("Nettoyage WinSxS", "Nettoie le dossier composants Windows", "Nettoyage", "cmd",
            "DISM /Online /Cleanup-Image /StartComponentCleanup", true),
        // === Reseau ===
        script("Vider cache DNS", "Reinitialise le cache DNS", "Reseau", "cmd",
            "ipconfig /flushdns", false),
        script("Reset Winsock", "Reinitialise la pile reseau", "Reseau", "cmd",
            "netsh winsock reset", true),
        script("Reset TCP/IP", "Reinitialise la config TCP/IP", "Reseau", "cmd",
            "netsh int ip reset", true),
        script("Renouveler IP", "Release et renew de l'adresse IP", "Reseau", "cmd",
            "ipconfig /release & ipconfig /renew", false),
        script("Afficher profils WiFi", "Liste tous les profils WiFi enregistres", "Reseau", "cmd",
            "netsh wlan show profiles", false),
        script("Mot de passe WiFi actuel", "Affiche le mot de passe du WiFi connecte", "Reseau", "powershell",
            "$p=(netsh wlan show interfaces | Select-String 'Profile').ToString().Split(':')[1].Trim(); netsh wlan show profile name=$p key=clear | Select-String 'Key Content'", false),
        // === Reparation ===
        script("SFC Scannow", "Verifie et repare les fichiers systeme", "Reparation", "cmd",
            "sfc /scannow", true),
        script("DISM Restore Health", "Repare l'image Windows avec DISM", "Reparation", "cmd",
            "DISM /Online /Cleanup-Image /RestoreHealth", true),
        script("CHKDSK C:", "Verifie l'integrite du disque C:", "Reparation", "cmd",
            "chkdsk C: /f", true),
        script("Reset Windows Update", "Reinitialise les composants Windows Update", "Reparation", "cmd",
            "net stop wuauserv & net stop cryptSvc & net stop bits & ren C:\\Windows\\SoftwareDistribution SoftwareDistribution.old & net start wuauserv & net start cryptSvc & net start bits", true),
        script("Re-enregistrer DLLs systeme", "Re-enregistre toutes les DLLs systeme", "Reparation", "cmd",
            "for /f %s in ('dir /b /s %windir%\\system32\\*.dll') do regsvr32 /s %s", true),
        // === Performance ===
        script("Defragmenter C:", "Optimise le disque C:", "Performance", "cmd",
            "defrag C: /O", true),
        script("Vider RAM standby", "Libere la memoire en standby", "Performance", "powershell",
            "Clear-RecycleBin -Force -ErrorAction SilentlyContinue; [System.GC]::Collect()", false),
        script("Desactiver indexation", "Arrete le service d'indexation Windows Search", "Performance", "cmd",
            "sc stop WSearch & sc config WSearch start=disabled", true),
        script("Activer TRIM SSD", "Active la commande TRIM pour les SSD", "Performance", "cmd",
            "fsutil behavior set disabledeletenotify 0", true),
        // === Diagnostic ===
        script("Lister services actifs", "Affiche les services en cours d'execution", "Diagnostic", "powershell",
            "Get-Service | Where-Object {$_.Status -eq 'Running'} | Format-Table Name, DisplayName -AutoSize", false),
        script("Infos batterie", "Genere un rapport batterie sur le Bureau", "Diagnostic", "cmd",
            "powercfg /batteryreport /output %USERPROFILE%\\Desktop\\battery-report.html", false),
        script("Rapport energie", "Analyse la consommation energetique", "Diagnostic", "cmd",
            "powercfg /energy /output %USERPROFILE%\\Desktop\\energy-report.html", true),
        script("Lister programmes installes", "Liste via WinGet", "Diagnostic", "cmd",
            "winget list", false),
        script("Espace disque par dossier", "Top 20 des plus gros dossiers sur C:", "Diagnostic", "powershell",
            "Get-ChildItem C:\\ -Directory -ErrorAction SilentlyContinue | ForEach-Object { $size = (Get-ChildItem $_.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object Length -Sum).Sum; [PSCustomObject]@{Folder=$_.Name; SizeMB=[math]::Round($size/1MB,1)} } | Sort-Object SizeMB -Descending | Select-Object -First 20 | Format-Table -AutoSize", false),
        script("Verifier sante disque", "Affiche le statut SMART des disques", "Diagnostic", "cmd",
            "wmic diskdrive get status,model,size", false),
        // === Tweaks ===
        script("Desactiver telemetrie", "Desactive la telemetrie Windows", "Tweaks", "powershell",
            "Set-ItemProperty -Path 'HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection' -Name 'AllowTelemetry' -Value 0 -Type DWord -Force", true),
        script("Activer mode sombre", "Active le mode sombre Windows", "Tweaks", "powershell",
            "Set-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize' -Name 'AppsUseLightTheme' -Value 0; Set-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize' -Name 'SystemUsesLightTheme' -Value 0", false),
        script("Desactiver Cortana", "Desactive Cortana via registre", "Tweaks", "powershell",
            "Set-ItemProperty -Path 'HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search' -Name 'AllowCortana' -Value 0 -Type DWord -Force", true),
        script("Restaurer menu classique W11", "Restaure le menu contextuel classique", "Tweaks", "powershell",
            "reg add 'HKCU\\Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32' /f /ve", false),
    ]
}

// === Script File Management ===

#[derive(Debug, Clone, Serialize)]
pub struct ScriptFileInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub script_type: String,
}

pub fn list_script_files(dir: &str) -> Result<Vec<ScriptFileInfo>, NiTriTeError> {
    let path = std::path::Path::new(dir);
    if !path.exists() || !path.is_dir() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let ep = entry.path();
        if ep.is_file() {
            let ext = ep.extension().and_then(|e| e.to_str()).unwrap_or("");
            let script_type = match ext {
                "ps1" => "powershell",
                "bat" | "cmd" => "cmd",
                "sh" => "bash",
                "py" => "python",
                _ => continue,
            };
            files.push(ScriptFileInfo {
                name: ep.file_name().unwrap_or_default().to_string_lossy().to_string(),
                path: ep.to_string_lossy().to_string(),
                size_bytes: entry.metadata().map(|m| m.len()).unwrap_or(0),
                script_type: script_type.to_string(),
            });
        }
    }

    files.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(files)
}

pub fn read_script_file(path: &str) -> Result<String, NiTriTeError> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err(NiTriTeError::System(format!("Fichier introuvable: {}", path)));
    }
    // Limiter a 100KB
    let meta = std::fs::metadata(p)?;
    if meta.len() > 100_000 {
        return Err(NiTriTeError::System("Fichier trop volumineux (max 100KB)".into()));
    }
    Ok(std::fs::read_to_string(p)?)
}

pub fn save_script_file(path: &str, content: &str) -> Result<(), NiTriTeError> {
    // Valider l'extension
    let p = std::path::Path::new(path);
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
    if !["ps1", "bat", "cmd", "sh", "py"].contains(&ext) {
        return Err(NiTriTeError::System(format!("Extension non autorisee: .{}", ext)));
    }
    std::fs::write(p, content)?;
    Ok(())
}

fn script(name: &str, desc: &str, cat: &str, stype: &str, content: &str, admin: bool) -> ScriptEntry {
    ScriptEntry {
        name: name.to_string(),
        description: desc.to_string(),
        category: cat.to_string(),
        script_type: stype.to_string(),
        content: content.to_string(),
        requires_admin: admin,
    }
}
