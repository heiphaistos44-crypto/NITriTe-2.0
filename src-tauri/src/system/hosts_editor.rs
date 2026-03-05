use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Default)]
pub struct HostsEntry {
    pub ip: String,
    pub hostname: String,
    pub comment: String,
    pub active: bool,
    pub line_number: u32,
}

const HOSTS_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";

#[tauri::command]
pub fn get_hosts_entries() -> Vec<HostsEntry> {
    let content = match std::fs::read_to_string(HOSTS_PATH) {
        Ok(c) => c,
        Err(_) => {
            // Try PowerShell read
            #[cfg(target_os = "windows")]
            {
                let o = Command::new("powershell")
                    .args(["-NoProfile","-NonInteractive","-Command",
                           &format!("Get-Content '{}' -Raw", HOSTS_PATH)])
                    .creation_flags(0x08000000).output().ok();
                if let Some(o) = o {
                    String::from_utf8_lossy(&o.stdout).to_string()
                } else { return vec![]; }
            }
            #[cfg(not(target_os = "windows"))]
            return vec![];
        }
    };

    let mut entries = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let active = !trimmed.starts_with('#');

        // Parse active entry: "ip hostname # optional comment"
        let parse_line = if active { trimmed } else {
            trimmed.trim_start_matches('#').trim()
        };

        let parts: Vec<&str> = parse_line.splitn(2, |c: char| c.is_whitespace()).collect();
        if parts.len() >= 1 {
            let ip = parts[0].trim();
            let rest = parts.get(1).unwrap_or(&"").trim();

            // Separate hostname and inline comment
            let (hostname, comment) = if let Some(ci) = rest.find('#') {
                (rest[..ci].trim(), rest[ci+1..].trim())
            } else {
                (rest, "")
            };

            if !ip.is_empty() && !hostname.is_empty() {
                entries.push(HostsEntry {
                    ip: ip.to_string(),
                    hostname: hostname.split_whitespace().next().unwrap_or("").to_string(),
                    comment: comment.to_string(),
                    active,
                    line_number: i as u32 + 1,
                });
            }
        }
    }
    entries
}

#[tauri::command]
pub fn add_hosts_entry(ip: String, hostname: String, comment: String) -> Result<String, String> {
    let ip_c = ip.trim().to_string();
    let host_c = hostname.trim().to_string();
    if ip_c.is_empty() || host_c.is_empty() {
        return Err("IP et hostname requis".to_string());
    }
    let line = if comment.is_empty() {
        format!("\n{}\t{}", ip_c, host_c)
    } else {
        format!("\n{}\t{}\t# {}", ip_c, host_c, comment)
    };
    let ps = format!(r#"Add-Content -Path '{}' -Value '{}' -Encoding UTF8"#, HOSTS_PATH, line.replace('\'', ""));
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",&ps]).creation_flags(0x08000000).output().map_err(|e| e.to_string())?;
        if o.status.success() {
            return Ok(format!("Entrée ajoutée : {} -> {}", ip_c, host_c));
        }
        return Err(String::from_utf8_lossy(&o.stderr).to_string());
    }
    #[cfg(not(target_os = "windows"))]
    Err("Non disponible".to_string())
}

#[tauri::command]
pub fn delete_hosts_entry(line_number: u32) -> Result<String, String> {
    let ps = format!(r#"
$lines = @(Get-Content '{}')
$new = @($lines | Select-Object -SkipIndex {})
$new | Set-Content '{}' -Encoding UTF8
"#, HOSTS_PATH, line_number - 1, HOSTS_PATH);
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",&ps]).creation_flags(0x08000000).output().map_err(|e| e.to_string())?;
        if o.status.success() { return Ok("Entrée supprimée".to_string()); }
        return Err(String::from_utf8_lossy(&o.stderr).to_string());
    }
    #[cfg(not(target_os = "windows"))]
    Err("Non disponible".to_string())
}

#[tauri::command]
pub fn toggle_hosts_entry(line_number: u32, enable: bool) -> Result<String, String> {
    let ps = format!(r#"
$lines = @(Get-Content '{}')
$idx = {}
if ($idx -ge 0 -and $idx -lt $lines.Count) {{
    $line = $lines[$idx]
    if ({}) {{
        $lines[$idx] = $line.TrimStart('#').TrimStart()
    }} else {{
        if (-not $line.StartsWith('#')) {{ $lines[$idx] = '# ' + $line }}
    }}
    $lines | Set-Content '{}' -Encoding UTF8
    "Modifié"
}} else {{ throw "Ligne introuvable" }}
"#, HOSTS_PATH, line_number - 1, if enable { "$true" } else { "$false" }, HOSTS_PATH);
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",&ps]).creation_flags(0x08000000).output().map_err(|e| e.to_string())?;
        if o.status.success() { return Ok("Modifié".to_string()); }
        return Err(String::from_utf8_lossy(&o.stderr).to_string());
    }
    #[cfg(not(target_os = "windows"))]
    Err("Non disponible".to_string())
}

#[tauri::command]
pub fn backup_hosts() -> Result<String, String> {
    let backup = format!("{}.bak", HOSTS_PATH);
    std::fs::copy(HOSTS_PATH, &backup).map(|_| format!("Sauvegarde : {}", backup)).map_err(|e| e.to_string())
}
