use serde::Serialize;
use tauri::Emitter;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize)]
pub struct ShadowCopy {
    pub id: String,
    pub volume: String,
    pub creation_time: String,
    pub provider: String,
    pub device_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecoveredFile {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub deleted_date: String,
    pub source: String, // "recycle_bin" | "shadow_copy" | "mft"
    pub is_dir: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestoreResult {
    pub success: bool,
    pub message: String,
    pub restored_path: String,
}

/// Liste les Shadow Copies (Points de restauration VSS)
pub fn list_shadow_copies() -> Vec<ShadowCopy> {
    let ps = r#"
try {
    $shadows = Get-WmiObject Win32_ShadowCopy -ErrorAction Stop
    $result = @()
    foreach ($s in $shadows) {
        $result += @{
            id            = $s.ID
            volume        = $s.VolumeName
            creation_time = $s.InstallDate
            provider      = $s.ProviderID
            device_path   = $s.DeviceObject
        }
    }
    $result | ConvertTo-Json -Compress -Depth 2
} catch { Write-Output '[]' }
"#;
    run_ps(ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(ShadowCopy {
                id: v["id"].as_str()?.to_string(),
                volume: v["volume"].as_str().unwrap_or("").to_string(),
                creation_time: v["creation_time"].as_str().unwrap_or("").to_string(),
                provider: v["provider"].as_str().unwrap_or("").to_string(),
                device_path: v["device_path"].as_str().unwrap_or("").to_string(),
            })
        }).collect())
        .unwrap_or_default()
}

/// Normalise un chemin shadow copy : device_path inclut déjà \\?\GLOBALROOT
fn shadow_path(device_path: &str, sub_path: &str) -> String {
    let base = device_path.trim_end_matches('\\');
    if sub_path.is_empty() {
        base.to_string()
    } else if sub_path.starts_with("\\\\?\\") || sub_path.starts_with("\\\\") {
        // Déjà un chemin UNC complet (ex: depuis FullName de PowerShell)
        sub_path.trim_end_matches('\\').to_string()
    } else {
        format!("{}\\{}", base, sub_path.trim_matches('\\'))
    }
}

/// Lister le contenu du dossier dans une Shadow Copy
pub fn browse_shadow_copy(device_path: String, relative_path: String) -> Vec<RecoveredFile> {
    // device_path vient de Win32_ShadowCopy.DeviceObject :
    // \\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyX  (préfixe déjà présent)
    let path = shadow_path(&device_path, &relative_path);

    let ps = format!(r#"
$p = '{}'
try {{
    $items = @()
    Get-ChildItem -LiteralPath $p -Force -ErrorAction SilentlyContinue | ForEach-Object {{
        try {{
            $items += @{{
                name       = $_.Name
                path       = $_.FullName
                size_bytes = if ($_.PSIsContainer) {{ 0 }} else {{ try {{ $_.Length }} catch {{ 0 }} }}
                is_dir     = $_.PSIsContainer
                modified   = try {{ $_.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss') }} catch {{ '' }}
            }}
        }} catch {{}}
    }}
    if ($items.Count -eq 0) {{ Write-Output '[]' }}
    elseif ($items.Count -eq 1) {{ Write-Output "[$($items | ConvertTo-Json -Compress -Depth 2)]" }}
    else {{ $items | ConvertTo-Json -Compress -Depth 2 }}
}} catch {{ Write-Output '[]' }}
"#, path.replace('\'', "''"));

    run_ps(&ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(RecoveredFile {
                name: v["name"].as_str()?.to_string(),
                path: v["path"].as_str().unwrap_or("").to_string(),
                size_bytes: v["size_bytes"].as_u64().unwrap_or(0),
                deleted_date: v["modified"].as_str().unwrap_or("").to_string(),
                source: "shadow_copy".to_string(),
                is_dir: v["is_dir"].as_bool().unwrap_or(false),
            })
        }).collect())
        .unwrap_or_default()
}

/// Chercher des fichiers dans une Shadow Copy (récursif, filtre par nom)
pub fn search_shadow_copy(device_path: String, query: String, base_path: String) -> Vec<RecoveredFile> {
    let shadow_root = shadow_path(&device_path, &base_path);
    let ps = format!(r#"
$root = '{}'
$q = '{}'
try {{
    $items = @()
    Get-ChildItem -LiteralPath $root -Recurse -ErrorAction SilentlyContinue |
        Where-Object {{ $_.Name -like "*$q*" }} |
        Select-Object -First 500 |
        ForEach-Object {{
            try {{
                $items += @{{
                    name       = $_.Name
                    path       = $_.FullName
                    size_bytes = if ($_.PSIsContainer) {{ 0 }} else {{ try {{ $_.Length }} catch {{ 0 }} }}
                    is_dir     = $_.PSIsContainer
                    modified   = try {{ $_.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss') }} catch {{ '' }}
                }}
            }} catch {{}}
        }}
    if ($items.Count -eq 0) {{ Write-Output '[]' }}
    elseif ($items.Count -eq 1) {{ Write-Output "[$($items | ConvertTo-Json -Compress -Depth 2)]" }}
    else {{ $items | ConvertTo-Json -Compress -Depth 2 }}
}} catch {{ Write-Output '[]' }}
"#, shadow_root.replace('\'', "''"), query.replace('\'', "''"));

    run_ps(&ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(RecoveredFile {
                name: v["name"].as_str()?.to_string(),
                path: v["path"].as_str().unwrap_or("").to_string(),
                size_bytes: v["size_bytes"].as_u64().unwrap_or(0),
                deleted_date: v["modified"].as_str().unwrap_or("").to_string(),
                source: "shadow_copy".to_string(),
                is_dir: v["is_dir"].as_bool().unwrap_or(false),
            })
        }).collect())
        .unwrap_or_default()
}

/// Restaurer un lot de fichiers depuis une Shadow Copy vers un dossier cible
#[derive(Debug, Clone, Serialize)]
pub struct BatchRestoreResult {
    pub success: bool,
    pub restored_count: usize,
    pub failed_count: usize,
    pub message: String,
}

pub fn restore_files_batch(files: Vec<String>, target_folder: String) -> BatchRestoreResult {
    if files.is_empty() {
        return BatchRestoreResult { success: false, restored_count: 0, failed_count: 0, message: "Aucun fichier sélectionné".into() };
    }
    let files_array = files.iter()
        .map(|f| format!("'{}'", f.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(",");
    let ps = format!(r#"
$target = '{}'
$files = @({})
if (-not (Test-Path $target)) {{ New-Item -ItemType Directory -Path $target -Force | Out-Null }}
$ok = 0; $fail = 0; $errs = @()
foreach ($src in $files) {{
    try {{
        Copy-Item -Path $src -Destination $target -Recurse -Force -ErrorAction Stop
        $ok++
    }} catch {{
        $fail++
        $errs += $_.Exception.Message
    }}
}}
@{{ ok=$ok; fail=$fail; errors=($errs | Select-Object -First 5) -join '|' }} | ConvertTo-Json -Compress
"#, target_folder.replace('\'', "''"), files_array);

    match run_ps(&ps) {
        Some(t) => {
            let t = t.trim();
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(t) {
                let ok = v["ok"].as_u64().unwrap_or(0) as usize;
                let fail = v["fail"].as_u64().unwrap_or(0) as usize;
                let errs = v["errors"].as_str().unwrap_or("").to_string();
                let msg = if fail == 0 {
                    format!("{} fichier(s) restauré(s) dans {}", ok, target_folder)
                } else {
                    format!("{} restauré(s), {} échec(s){}", ok, fail,
                        if errs.is_empty() { String::new() } else { format!(": {}", &errs[..errs.len().min(200)]) })
                };
                BatchRestoreResult { success: fail == 0 || ok > 0, restored_count: ok, failed_count: fail, message: msg }
            } else {
                BatchRestoreResult { success: false, restored_count: 0, failed_count: files.len(), message: "Erreur parse résultat".into() }
            }
        }
        None => BatchRestoreResult { success: false, restored_count: 0, failed_count: files.len(), message: "Erreur PowerShell".into() },
    }
}

/// Restaurer un fichier depuis une Shadow Copy vers un dossier cible
pub fn restore_from_shadow(source_path: String, target_folder: String) -> RestoreResult {
    let target = format!("{}\\", target_folder.trim_end_matches('\\'));

    let ps = format!(r#"
try {{
    $src = '{}'
    $dst = '{}'
    if (-not (Test-Path $dst)) {{ New-Item -ItemType Directory -Path $dst -Force | Out-Null }}
    Copy-Item -Path $src -Destination $dst -Recurse -Force -ErrorAction Stop
    Write-Output 'OK'
}} catch {{ Write-Output "ERR:$($_.Exception.Message)" }}
"#,
        source_path.replace('\'', "''"),
        target.replace('\'', "''")
    );

    match run_ps(&ps) {
        Some(t) if t.trim() == "OK" => RestoreResult {
            success: true,
            message: "Fichier restauré avec succès".into(),
            restored_path: target,
        },
        Some(t) => RestoreResult {
            success: false,
            message: t.trim().trim_start_matches("ERR:").to_string(),
            restored_path: String::new(),
        },
        None => RestoreResult {
            success: false,
            message: "Impossible de lancer PowerShell".into(),
            restored_path: String::new(),
        },
    }
}

/// Scanner la Corbeille et retourner les fichiers récupérables
pub fn scan_recycle_bin() -> Vec<RecoveredFile> {
    let ps = r#"
try {
    $shell = New-Object -ComObject Shell.Application
    $recycle = $shell.Namespace(0xA)  # CSIDL_BITBUCKET
    $items = @()
    foreach ($item in $recycle.Items()) {
        $items += @{
            name       = $item.Name
            path       = $item.Path
            size_bytes = $item.Size
            deleted    = $item.ExtendedProperty("System.Recycle.DateDeleted")
        }
    }
    $items | ConvertTo-Json -Compress -Depth 2
} catch { Write-Output '[]' }
"#;
    run_ps(ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(RecoveredFile {
                name: v["name"].as_str()?.to_string(),
                path: v["path"].as_str().unwrap_or("").to_string(),
                size_bytes: v["size_bytes"].as_u64().unwrap_or(0),
                deleted_date: v["deleted"].as_str().unwrap_or("").to_string(),
                source: "recycle_bin".to_string(),
                is_dir: false,
            })
        }).collect())
        .unwrap_or_default()
}

/// Restaurer un élément de la Corbeille (déplace vers le dossier cible)
pub fn restore_recycle_bin_item(original_path: String) -> RestoreResult {
    let ps = format!(r#"
try {{
    $shell = New-Object -ComObject Shell.Application
    $recycle = $shell.Namespace(0xA)
    foreach ($item in $recycle.Items()) {{
        if ($item.Path -eq '{}') {{
            $item.InvokeVerb('Restore')
            Write-Output 'OK'
            exit
        }}
    }}
    Write-Output 'ERR:Élément introuvable dans la Corbeille'
}} catch {{ Write-Output "ERR:$($_.Exception.Message)" }}
"#, original_path.replace('\'', "''"));

    match run_ps(&ps) {
        Some(t) if t.trim() == "OK" => RestoreResult {
            success: true,
            message: "Fichier restauré à son emplacement d'origine".into(),
            restored_path: original_path,
        },
        Some(t) => RestoreResult {
            success: false,
            message: t.trim().trim_start_matches("ERR:").to_string(),
            restored_path: String::new(),
        },
        None => RestoreResult {
            success: false,
            message: "Erreur PowerShell".into(),
            restored_path: String::new(),
        },
    }
}

/// Scan MFT pour fichiers récemment supprimés (dernières 48h) via fsutil
pub fn scan_deleted_files(drive: String) -> Vec<RecoveredFile> {
    // Utilise les journaux NTFS via fsutil pour détecter les suppressions récentes
    let ps = format!(r#"
try {{
    $drive = '{}'
    # Cherche dans l'USN Journal les suppressions récentes
    $result = fsutil usn readjournal $drive maxcount=500 2>$null
    if (-not $result) {{ Write-Output '[]'; exit }}

    $deleted = @()
    $lines = $result -split "`n"
    $current = @{{}}
    foreach ($line in $lines) {{
        if ($line -match '^\s*File name\s*:\s*(.+)') {{ $current.name = $matches[1].Trim() }}
        if ($line -match '^\s*Reason\s*:\s*(.+)') {{ $current.reason = $matches[1].Trim() }}
        if ($line -match '^\s*Time stamp\s*:\s*(.+)') {{ $current.time = $matches[1].Trim() }}
        if ($line -match '^\s*$' -and $current.name -and $current.reason -like '*Delete*') {{
            $deleted += @{{ name=$current.name; time=$current.time; reason=$current.reason }}
            $current = @{{}}
        }}
    }}
    $deleted | Select-Object -First 200 | ConvertTo-Json -Compress
}} catch {{ Write-Output '[]' }}
"#, drive.trim_end_matches(':'));

    run_ps(&ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(RecoveredFile {
                name: v["name"].as_str()?.to_string(),
                path: format!("{}:\\[supprimé]\\{}", drive.trim_end_matches(':'), v["name"].as_str().unwrap_or("")),
                size_bytes: 0,
                deleted_date: v["time"].as_str().unwrap_or("").to_string(),
                source: "mft_journal".to_string(),
                is_dir: false,
            })
        }).collect())
        .unwrap_or_default()
}

// ════════════════════════════════════════════════════════
//  Récupération Disque Externe / Défaillant
// ════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub letter: String,
    pub label: String,
    pub total_gb: f64,
    pub free_gb: f64,
    pub disk_type: String,
    pub file_system: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiskEntry {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub modified: String,
    pub is_dir: bool,
}

/// Liste les lecteurs connectés (fixe, amovible, réseau)
pub fn list_connected_disks() -> Vec<DiskInfo> {
    let ps = r#"
try {
    Get-WmiObject Win32_LogicalDisk -ErrorAction SilentlyContinue | ForEach-Object {
        @{
            letter      = $_.DeviceID
            label       = if ($_.VolumeName) { $_.VolumeName } else { '' }
            total_gb    = if ($_.Size) { [math]::Round($_.Size / 1GB, 1) } else { 0 }
            free_gb     = if ($_.FreeSpace) { [math]::Round($_.FreeSpace / 1GB, 1) } else { 0 }
            disk_type   = switch ($_.DriveType) {
                2 { 'Amovible' } 3 { 'Fixe' } 4 { 'Réseau' } 5 { 'CD/DVD' } default { 'Inconnu' }
            }
            file_system = if ($_.FileSystem) { $_.FileSystem } else { 'NTFS' }
        }
    } | ConvertTo-Json -Compress -Depth 2
} catch { Write-Output '[]' }
"#;
    run_ps(ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(DiskInfo {
                letter:      v["letter"].as_str()?.to_string(),
                label:       v["label"].as_str().unwrap_or("").to_string(),
                total_gb:    v["total_gb"].as_f64().unwrap_or(0.0),
                free_gb:     v["free_gb"].as_f64().unwrap_or(0.0),
                disk_type:   v["disk_type"].as_str().unwrap_or("Fixe").to_string(),
                file_system: v["file_system"].as_str().unwrap_or("NTFS").to_string(),
            })
        }).collect())
        .unwrap_or_default()
}

/// Liste le contenu d'un chemin disque (sans préfixe shadow)
pub fn browse_disk_path(path: String) -> Vec<DiskEntry> {
    let ps = format!(r#"
$p = '{}'
try {{
    $items = @()
    Get-ChildItem -LiteralPath $p -Force -ErrorAction SilentlyContinue | ForEach-Object {{
        try {{
            $items += @{{
                name       = $_.Name
                path       = $_.FullName
                size_bytes = if ($_.PSIsContainer) {{ 0 }} else {{ try {{ $_.Length }} catch {{ 0 }} }}
                modified   = try {{ $_.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss') }} catch {{ '' }}
                is_dir     = $_.PSIsContainer
            }}
        }} catch {{}}
    }}
    if ($items.Count -eq 0) {{ Write-Output '[]' }}
    elseif ($items.Count -eq 1) {{ Write-Output "[$($items | ConvertTo-Json -Compress -Depth 2)]" }}
    else {{ $items | ConvertTo-Json -Compress -Depth 2 }}
}} catch {{ Write-Output '[]' }}
"#, path.replace('\'', "''"));

    run_ps(&ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(DiskEntry {
                name:       v["name"].as_str()?.to_string(),
                path:       v["path"].as_str().unwrap_or("").to_string(),
                size_bytes: v["size_bytes"].as_u64().unwrap_or(0),
                modified:   v["modified"].as_str().unwrap_or("").to_string(),
                is_dir:     v["is_dir"].as_bool().unwrap_or(false),
            })
        }).collect())
        .unwrap_or_default()
}

/// Récupère des fichiers/dossiers depuis un disque vers une destination.
/// safe_mode=true : lecture par chunks 64 KB via FileStream (FileShare.ReadWrite),
/// mauvais secteurs → écriture de zéros pour maintenir la structure (style ddrescue).
pub fn recover_files_safe(
    files: Vec<String>,
    target_folder: String,
    safe_mode: bool,
    window: &tauri::Window,
) -> BatchRestoreResult {
    use std::path::Path;
    let total = files.len();
    if total == 0 {
        return BatchRestoreResult { success: false, restored_count: 0, failed_count: 0, message: "Aucun fichier sélectionné".into() };
    }
    if let Err(e) = std::fs::create_dir_all(&target_folder) {
        return BatchRestoreResult { success: false, restored_count: 0, failed_count: total, message: format!("Impossible de créer la destination: {}", e) };
    }
    let mut ok = 0usize;
    let mut fail = 0usize;
    let mut notices: Vec<String> = Vec::new();

    for (i, src) in files.iter().enumerate() {
        let name = Path::new(src).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let pct = ((i as f32 / total as f32) * 90.0) as u32 + 5;
        let _ = window.emit("recover-disk-progress", serde_json::json!({
            "percent": pct,
            "message": format!("({}/{}) {}", i + 1, total, name)
        }));

        let is_dir = Path::new(src).is_dir();
        let dst = format!("{}\\{}", target_folder.trim_end_matches('\\'), name);

        if is_dir {
            #[cfg(target_os = "windows")]
            {
                // Robocopy en mode backup — /R:0 sans retry sur disque défaillant
                let (retry, wait) = if safe_mode { ("0", "0") } else { ("3", "5") };
                let status = std::process::Command::new("robocopy")
                    .args([src.as_str(), &dst, "/E", &format!("/R:{retry}"), &format!("/W:{wait}"),
                           "/B", "/COPYALL", "/NP", "/NFL", "/NDL", "/NJH", "/NJS"])
                    .creation_flags(0x08000000).status();
                match status {
                    Ok(s) if s.code().map(|c| c < 8).unwrap_or(false) => { ok += 1; }
                    Ok(s) => {
                        let code = s.code().unwrap_or(-1);
                        if code == 8 { ok += 1; notices.push(format!("{}: quelques fichiers inaccessibles ignorés", name)); }
                        else { fail += 1; notices.push(format!("{}: robocopy code {}", name, code)); }
                    }
                    Err(e) => { fail += 1; notices.push(format!("{}: {}", name, e)); }
                }
            }
            #[cfg(not(target_os = "windows"))]
            { fail += 1; notices.push(format!("{}: plateforme non supportée", name)); }
        } else if safe_mode {
            // ── Mode ddrescue : lecture par chunks 64 KB, zéros sur mauvais secteurs ──
            let ps = format!(r#"
$src = '{src}'
$dst = '{dst}'
try {{
    $dstDir = Split-Path $dst -Parent
    if (-not (Test-Path $dstDir)) {{ New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }}
    $chunkSize = 65536
    $fs = [System.IO.File]::Open($src, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read, [System.IO.FileShare]::ReadWrite)
    $fd = [System.IO.File]::Open($dst, [System.IO.FileMode]::Create, [System.IO.FileAccess]::Write, [System.IO.FileShare]::None)
    $buf   = New-Object byte[] $chunkSize
    $zeros = New-Object byte[] $chunkSize
    $recovered = [long]0
    $skipped   = [long]0
    while ($true) {{
        $read = 0
        try {{
            $read = $fs.Read($buf, 0, $chunkSize)
            if ($read -eq 0) {{ break }}
            $fd.Write($buf, 0, $read)
            $recovered += $read
        }} catch {{
            # Mauvais secteur : écrire des zéros pour maintenir la structure
            $fd.Write($zeros, 0, $chunkSize)
            $skipped += $chunkSize
            try {{ [void]$fs.Seek($chunkSize, [System.IO.SeekOrigin]::Current) }} catch {{}}
        }}
    }}
    $fs.Close(); $fd.Close()
    Write-Output "OK:$recovered`:$skipped"
}} catch {{
    try {{ if ($fs) {{ $fs.Close() }} }} catch {{}}
    try {{ if ($fd) {{ $fd.Close() }} }} catch {{}}
    Write-Output "ERR:$($_.Exception.Message)"
}}
"#,
                src = src.replace('\'', "''"),
                dst = dst.replace('\'', "''")
            );
            match run_ps(&ps) {
                Some(t) => {
                    let t = t.trim();
                    if t.starts_with("OK:") {
                        let parts: Vec<&str> = t[3..].splitn(2, ':').collect();
                        let skipped: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                        ok += 1;
                        if skipped > 0 {
                            let kb = skipped / 1024;
                            notices.push(format!("{}: {} Ko de secteurs défaillants (remplacés par zéros)", name, kb));
                        }
                    } else {
                        fail += 1;
                        notices.push(format!("{}: {}", name, t.trim_start_matches("ERR:")));
                    }
                }
                None => { fail += 1; notices.push(format!("{}: erreur PowerShell", name)); }
            }
        } else {
            // ── Mode normal : copie directe ──
            let ps = format!(r#"
try {{
    $dstDir = Split-Path '{}' -Parent
    if (-not (Test-Path $dstDir)) {{ New-Item -ItemType Directory -Path $dstDir -Force | Out-Null }}
    [System.IO.File]::Copy('{}', '{}', $true)
    Write-Output 'OK'
}} catch {{ Write-Output "ERR:$($_.Exception.Message)" }}
"#, dst.replace('\'', "''"), src.replace('\'', "''"), dst.replace('\'', "''"));
            match run_ps(&ps) {
                Some(t) if t.trim() == "OK" => { ok += 1; }
                Some(t) => { fail += 1; notices.push(format!("{}: {}", name, t.trim().trim_start_matches("ERR:"))); }
                None => { fail += 1; notices.push(format!("{}: erreur PowerShell", name)); }
            }
        }
    }
    let _ = window.emit("recover-disk-progress", serde_json::json!({
        "percent": 100,
        "message": format!("{}/{} éléments récupérés", ok, total)
    }));
    let msg = if notices.is_empty() {
        format!("{} élément(s) récupéré(s) avec succès dans «{}»", ok, target_folder)
    } else if fail == 0 {
        format!("{} récupéré(s) — {} avertissement(s): {}", ok, notices.len(), notices[..notices.len().min(3)].join("; "))
    } else {
        format!("{} récupéré(s), {} échec(s): {}", ok, fail, notices[..notices.len().min(3)].join("; "))
    };
    BatchRestoreResult { success: ok > 0, restored_count: ok, failed_count: fail, message: msg }
}

// ════════════════════════════════════════════════════════
//  Comparaison Shadow Copy vs Système Actuel
// ════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize)]
pub struct ComparedFile {
    pub name: String,
    pub shadow_path: String,
    pub live_path: String,
    pub status: String, // "deleted" | "modified" | "added"
    pub shadow_size: u64,
    pub live_size: u64,
    pub shadow_modified: String,
    pub live_modified: String,
}

pub fn compare_shadow_with_current(device_path: String, sub_path: String, live_path: String) -> Vec<ComparedFile> {
    let shadow_full = shadow_path(&device_path, &sub_path);
    let ps = format!(r#"
$shadowPath = '{sp}'
$livePath   = '{lp}'
try {{
    $shadowItems = @{{}}
    if (Test-Path -LiteralPath $shadowPath) {{
        Get-ChildItem -LiteralPath $shadowPath -Force -ErrorAction SilentlyContinue | ForEach-Object {{
            $shadowItems[$_.Name] = @{{
                size     = if ($_.PSIsContainer) {{ 0 }} else {{ try {{ $_.Length }} catch {{ 0 }} }}
                modified = try {{ $_.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss') }} catch {{ '' }}
                is_dir   = $_.PSIsContainer
                full     = $_.FullName
            }}
        }}
    }}
    $liveItems = @{{}}
    if (Test-Path -LiteralPath $livePath) {{
        Get-ChildItem -LiteralPath $livePath -Force -ErrorAction SilentlyContinue | ForEach-Object {{
            $liveItems[$_.Name] = @{{
                size     = if ($_.PSIsContainer) {{ 0 }} else {{ try {{ $_.Length }} catch {{ 0 }} }}
                modified = try {{ $_.LastWriteTime.ToString('yyyy-MM-dd HH:mm:ss') }} catch {{ '' }}
                is_dir   = $_.PSIsContainer
                full     = $_.FullName
            }}
        }}
    }}
    $result = @()
    foreach ($name in $shadowItems.Keys) {{
        $s = $shadowItems[$name]
        if (-not $liveItems.ContainsKey($name)) {{
            $result += @{{ name=$name; shadow_path=$s.full; live_path=''; status='deleted';
                          shadow_size=$s.size; live_size=0; shadow_modified=$s.modified; live_modified='' }}
        }} else {{
            $l = $liveItems[$name]
            if (-not $s.is_dir -and ($s.size -ne $l.size -or $s.modified -ne $l.modified)) {{
                $result += @{{ name=$name; shadow_path=$s.full; live_path=$l.full; status='modified';
                              shadow_size=$s.size; live_size=$l.size; shadow_modified=$s.modified; live_modified=$l.modified }}
            }}
        }}
    }}
    foreach ($name in $liveItems.Keys) {{
        if (-not $shadowItems.ContainsKey($name)) {{
            $l = $liveItems[$name]
            $result += @{{ name=$name; shadow_path=''; live_path=$l.full; status='added';
                          shadow_size=0; live_size=$l.size; shadow_modified=''; live_modified=$l.modified }}
        }}
    }}
    if ($result.Count -eq 0) {{ Write-Output '[]' }}
    elseif ($result.Count -eq 1) {{ Write-Output "[$($result | ConvertTo-Json -Compress -Depth 2)]" }}
    else {{ $result | Select-Object -First 300 | ConvertTo-Json -Compress -Depth 2 }}
}} catch {{ Write-Output '[]' }}
"#,
        sp = shadow_full.replace('\'', "''"),
        lp = live_path.replace('\'', "''")
    );

    run_ps(&ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(ComparedFile {
                name:            v["name"].as_str()?.to_string(),
                shadow_path:     v["shadow_path"].as_str().unwrap_or("").to_string(),
                live_path:       v["live_path"].as_str().unwrap_or("").to_string(),
                status:          v["status"].as_str().unwrap_or("").to_string(),
                shadow_size:     v["shadow_size"].as_u64().unwrap_or(0),
                live_size:       v["live_size"].as_u64().unwrap_or(0),
                shadow_modified: v["shadow_modified"].as_str().unwrap_or("").to_string(),
                live_modified:   v["live_modified"].as_str().unwrap_or("").to_string(),
            })
        }).collect())
        .unwrap_or_default()
}

// ════════════════════════════════════════════════════════
//  Scan USN Journal — Tous les lecteurs NTFS
// ════════════════════════════════════════════════════════

pub fn scan_all_deleted_files() -> Vec<RecoveredFile> {
    let ps = r#"
try {
    $drives = Get-WmiObject Win32_LogicalDisk -ErrorAction SilentlyContinue |
        Where-Object { $_.FileSystem -eq 'NTFS' } |
        Select-Object -ExpandProperty DeviceID
    $deleted = @()
    foreach ($d in $drives) {
        $result = & fsutil usn readjournal $d maxcount=300 2>$null
        if (-not $result) { continue }
        $lines = $result -split "`n"
        $current = @{}
        foreach ($line in $lines) {
            if ($line -match '^\s*File name\s*:\s*(.+)')  { $current.name   = $matches[1].Trim() }
            if ($line -match '^\s*Reason\s*:\s*(.+)')     { $current.reason = $matches[1].Trim() }
            if ($line -match '^\s*Time stamp\s*:\s*(.+)') { $current.time   = $matches[1].Trim() }
            if ($line -match '^\s*$' -and $current.name -and $current.reason -like '*Delete*') {
                $deleted += @{ name=$current.name; time=$current.time; drive=$d }
                $current = @{}
            }
        }
    }
    if ($deleted.Count -eq 0) { Write-Output '[]' }
    else { $deleted | Select-Object -First 500 | ConvertTo-Json -Compress }
} catch { Write-Output '[]' }
"#;

    run_ps(ps)
        .and_then(|t| {
            let t = t.trim();
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            let drive = v["drive"].as_str().unwrap_or("?:");
            let name  = v["name"].as_str()?.to_string();
            Some(RecoveredFile {
                name:         name.clone(),
                path:         format!("{}\\[supprimé]\\{}", drive, name),
                size_bytes:   0,
                deleted_date: v["time"].as_str().unwrap_or("").to_string(),
                source:       "mft_journal".to_string(),
                is_dir:       false,
            })
        }).collect())
        .unwrap_or_default()
}

fn run_ps(script: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let o = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", script])
            .creation_flags(0x08000000)
            .output().ok()?;
        Some(String::from_utf8_lossy(&o.stdout).to_string())
    }
    #[cfg(not(target_os = "windows"))]
    None
}

// ════════════════════════════════════════════════════════
//  Sauvegarde Dossier Utilisateur
// ════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize)]
pub struct UserFolder {
    pub name: String,
    pub path: String,           // Chemin absolu (ex: C:\Users\John\Documents)
    pub size_mb: u64,
    pub shadow_relative: String, // Chemin relatif VSS (ex: Users\John\Documents)
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub message: String,
    pub duration_secs: u64,
    pub folders_count: usize,
}

/// Retourne les dossiers standard du profil utilisateur avec leur taille et chemin VSS relatif
pub fn get_user_profile_folders() -> Vec<UserFolder> {
    let ps = r#"
try {
    $result = @()
    $keys = @(
        @{ name='Documents';       path=[Environment]::GetFolderPath('Personal') },
        @{ name='Bureau';          path=[Environment]::GetFolderPath('Desktop') },
        @{ name='Images';          path=[Environment]::GetFolderPath('MyPictures') },
        @{ name='Videos';          path=[Environment]::GetFolderPath('MyVideos') },
        @{ name='Musique';         path=[Environment]::GetFolderPath('MyMusic') },
        @{ name='Telechargements'; path=(Join-Path ([Environment]::GetFolderPath('UserProfile')) 'Downloads') }
    )
    foreach ($k in $keys) {
        if (Test-Path $k.path) {
            $sum = 0
            try {
                $items = Get-ChildItem $k.path -Recurse -Force -ErrorAction SilentlyContinue
                if ($items) {
                    $measure = $items | Measure-Object -Property Length -Sum -ErrorAction SilentlyContinue
                    if ($measure.Sum) { $sum = $measure.Sum }
                }
            } catch {}
            # Chemin relatif VSS : strip "X:\" prefix
            $rel = $k.path -replace '^[A-Za-z]:\\', ''
            $result += @{
                name            = $k.name
                path            = $k.path
                size_mb         = [math]::Round($sum / 1MB, 0)
                shadow_relative = $rel
            }
        }
    }
    if ($result.Count -eq 0) { Write-Output '[]' }
    else { $result | ConvertTo-Json -Compress -Depth 2 }
} catch { Write-Output '[]' }
"#;
    run_ps(ps)
        .and_then(|t| {
            let t = t.trim();
            if t.is_empty() || t == "[]" { return Some(vec![]); }
            let j = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            serde_json::from_str::<Vec<serde_json::Value>>(&j).ok()
        })
        .map(|vals| vals.into_iter().filter_map(|v| {
            Some(UserFolder {
                name:            v["name"].as_str()?.to_string(),
                path:            v["path"].as_str().unwrap_or("").to_string(),
                size_mb:         v["size_mb"].as_u64().unwrap_or(0),
                shadow_relative: v["shadow_relative"].as_str().unwrap_or("").to_string(),
            })
        }).collect())
        .unwrap_or_default()
}

/// Sauvegarde les dossiers sélectionnés vers un dossier cible via Robocopy.
/// Continue sur erreur individuelle et retourne un rapport complet.
pub fn backup_user_folders(
    folders: Vec<String>,
    target: String,
    window: &tauri::Window,
) -> BackupResult {
    let start = std::time::Instant::now();
    let total = folders.len();

    if total == 0 {
        return BackupResult {
            success: false,
            message: "Aucun dossier sélectionné.".into(),
            duration_secs: 0,
            folders_count: 0,
        };
    }

    // ── Validation : créer + tester l'écriture en destination ──
    if let Err(e) = std::fs::create_dir_all(&target) {
        return BackupResult {
            success: false,
            message: format!(
                "Impossible de créer le dossier de destination: {}. \
                 Vérifiez le chemin et les permissions.",
                e
            ),
            duration_secs: 0,
            folders_count: 0,
        };
    }
    let test_file = std::path::Path::new(&target).join(".nitrite_write_test");
    if let Err(e) = std::fs::write(&test_file, b"ok") {
        return BackupResult {
            success: false,
            message: format!(
                "Destination non accessible en écriture ({}). \
                 Vérifiez les droits sur le dossier cible.",
                e
            ),
            duration_secs: 0,
            folders_count: 0,
        };
    }
    let _ = std::fs::remove_file(&test_file);

    // ── Copie dossier par dossier — continue sur erreur ─────────
    let mut succeeded = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for (i, folder_path) in folders.iter().enumerate() {
        let folder_name = std::path::Path::new(folder_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("dossier_{}", i));

        // Progression : répartie sur 5%→95%
        let pct = ((i as f32 / total as f32) * 88.0) as u32 + 5;
        let _ = window.emit("backup-profile-progress", serde_json::json!({
            "folder": &folder_name,
            "percent": pct,
            "message": format!("({}/{}) Copie de {}...", i + 1, total, folder_name)
        }));

        // Vérification source avant lancement
        if !std::path::Path::new(folder_path).exists() {
            errors.push(format!("{} : dossier source introuvable", folder_name));
            continue;
        }

        let dst = format!("{}\\{}", target.trim_end_matches('\\'), folder_name);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            let status = std::process::Command::new("robocopy")
                .args([
                    folder_path.as_str(),
                    dst.as_str(),
                    "/E",        // Récursif y compris dossiers vides
                    "/B",        // Mode backup (contourne les ACL)
                    "/DCOPY:DA", // Attributs + timestamps dossiers
                    "/COPY:DAT", // Données + attributs + timestamps fichiers
                    "/R:1", "/W:2",
                    "/MT:4",
                    "/NFL", "/NDL", "/NP",
                ])
                .creation_flags(0x08000000)
                .status();

            match status {
                Ok(s) if s.code().map(|c| c < 8).unwrap_or(false) => {
                    succeeded += 1;
                }
                Ok(s) => {
                    let code = s.code().unwrap_or(-1);
                    // Code 8 = certains fichiers impossibles à copier (tolerable)
                    if code == 8 {
                        succeeded += 1;
                        errors.push(format!("{} : copié avec avertissements (fichiers verrouillés ignorés)", folder_name));
                    } else {
                        errors.push(format!("{} : échec robocopy code {}", folder_name, code));
                    }
                }
                Err(e) => {
                    errors.push(format!("{} : {}", folder_name, e));
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            errors.push(format!("{} : non supporté hors Windows", folder_name));
        }
    }

    let _ = window.emit("backup-profile-progress", serde_json::json!({
        "folder": "done",
        "percent": 100,
        "message": if errors.is_empty() {
            format!("{}/{} dossier(s) sauvegardé(s) avec succès ✓", succeeded, total)
        } else {
            format!("{}/{} succès — {} avertissement(s)/erreur(s)", succeeded, total, errors.len())
        }
    }));

    let all_failed = succeeded == 0 && !errors.is_empty();
    BackupResult {
        success: !all_failed,
        message: if errors.is_empty() {
            format!("{} dossier(s) sauvegardé(s) dans «{}».", succeeded, target)
        } else {
            format!(
                "{}/{} succès. Détails : {}",
                succeeded, total,
                errors.join(" | ")
            )
        },
        duration_secs: start.elapsed().as_secs(),
        folders_count: succeeded,
    }
}

// ════════════════════════════════════════════════════════
//  Shadow Copy — Création & Suppression
// ════════════════════════════════════════════════════════

/// Crée une nouvelle Shadow Copy VSS sur le volume indiqué (ex: "C:")
pub fn create_shadow_copy(volume: String) -> Result<String, String> {
    let v = volume.trim_end_matches('\\').trim_end_matches(':');
    let vol_path = format!("{}:\\", v.to_uppercase());
    let ps = format!(
        r#"$r = (Get-WmiObject -List Win32_ShadowCopy).Create('{}', 'ClientAccessible'); if ($r.ReturnValue -eq 0) {{ 'OK:' + $r.ShadowID }} else {{ 'ERR:Code ' + $r.ReturnValue }}"#,
        vol_path
    );
    match run_ps(&ps) {
        Some(t) if t.trim().starts_with("OK:") => Ok(t.trim()[3..].to_string()),
        Some(t) => Err(t.trim().trim_start_matches("ERR:").to_string()),
        None => Err("Erreur PowerShell".into()),
    }
}

/// Supprime une Shadow Copy par son ID VSS
pub fn delete_shadow_copy(shadow_id: String) -> Result<String, String> {
    let ps = format!(
        r#"$s = Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq '{}' }}; if ($s) {{ $s.Delete(); 'OK' }} else {{ 'ERR:Shadow copy introuvable' }}"#,
        shadow_id.replace('\'', "''")
    );
    match run_ps(&ps) {
        Some(t) if t.trim() == "OK" => Ok("Shadow copy supprimée".into()),
        Some(t) => Err(t.trim().trim_start_matches("ERR:").to_string()),
        None => Err("Erreur PowerShell".into()),
    }
}

/// Ouvre un dossier dans l'Explorateur Windows
pub fn open_in_explorer(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "windows"))]
    { let _ = path; Err("Non supporté hors Windows".into()) }
}

/// Retourne les lettres des volumes NTFS connectés (ex: ["C:", "D:"])
pub fn get_ntfs_drives() -> Vec<String> {
    let ps = r#"Get-WmiObject Win32_LogicalDisk -ErrorAction SilentlyContinue | Where-Object { $_.FileSystem -eq 'NTFS' } | Select-Object -ExpandProperty DeviceID"#;
    run_ps(ps)
        .map(|t| t.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect())
        .unwrap_or_default()
}
