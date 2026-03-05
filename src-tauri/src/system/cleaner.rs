use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CleanTarget {
    pub name: String,
    pub path: String,
    pub size_mb: f64,
    pub file_count: u32,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CleanResult {
    pub target: String,
    pub freed_mb: f64,
    pub files_deleted: u32,
    pub success: bool,
    pub message: String,
}

fn ps_run(ps: &str) -> Option<serde_json::Value> {
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",ps]).creation_flags(0x08000000).output().ok()?;
        let t = String::from_utf8_lossy(&o.stdout);
        serde_json::from_str(t.trim()).ok()
    }
    #[cfg(not(target_os = "windows"))]
    None
}

#[tauri::command]
pub fn get_clean_targets() -> Vec<CleanTarget> {
    let ps = r#"
$items = @(
    @{ name='%TEMP%'; path=$env:TEMP; cat='Temp' },
    @{ name='Windows\Temp'; path='C:\Windows\Temp'; cat='Temp' },
    @{ name='Prefetch'; path='C:\Windows\Prefetch'; cat='Système' },
    @{ name='Dumps mémoire'; path='C:\Windows\Minidump'; cat='Système' },
    @{ name='Logs CBS'; path='C:\Windows\Logs\CBS'; cat='Logs' },
    @{ name='Windows Error Reports'; path="$env:LOCALAPPDATA\Microsoft\Windows\WER\ReportArchive"; cat='Logs' },
    @{ name='Corbeille'; path=''; cat='Corbeille' },
    @{ name='Chrome Cache'; path="$env:LOCALAPPDATA\Google\Chrome\User Data\Default\Cache"; cat='Navigateurs' },
    @{ name='Edge Cache'; path="$env:LOCALAPPDATA\Microsoft\Edge\User Data\Default\Cache"; cat='Navigateurs' },
    @{ name='Firefox Cache'; path="$env:APPDATA\Mozilla\Firefox\Profiles"; cat='Navigateurs' },
    @{ name='Windows Update Cache'; path='C:\Windows\SoftwareDistribution\Download'; cat='Windows Update' },
    @{ name='Thumbnails DB'; path="$env:LOCALAPPDATA\Microsoft\Windows\Explorer"; cat='Cache système' }
)

@($items | ForEach-Object {
    $p = $_.path
    $sz = 0.0; $cnt = 0
    if ($p -and (Test-Path $p)) {
        try {
            $files = @(Get-ChildItem $p -Recurse -File -EA SilentlyContinue)
            $cnt = $files.Count
            $bytes = ($files | Measure-Object -Property Length -Sum -EA SilentlyContinue).Sum
            $sz = if($bytes){[math]::Round($bytes/1MB,2)}else{0}
        } catch {}
    } elseif ($_.cat -eq 'Corbeille') {
        try {
            $shell = New-Object -ComObject Shell.Application
            $rb = $shell.Namespace(0xA)
            $cnt = @($rb.Items()).Count
            $sz = 0.1 * $cnt
        } catch {}
    }
    @{ name=$_.name; path=$p; mb=$sz; count=$cnt; cat=$_.cat }
}) | ConvertTo-Json -Compress
"#;
    if let Some(v) = ps_run(ps) {
        let arr = match v.as_array() {
            Some(a) => a.clone(),
            None => if v.is_object() { vec![v] } else { vec![] }
        };
        return arr.iter().map(|r| CleanTarget {
            name: r["name"].as_str().unwrap_or("").to_string(),
            path: r["path"].as_str().unwrap_or("").to_string(),
            size_mb: r["mb"].as_f64().unwrap_or(0.0),
            file_count: r["count"].as_u64().unwrap_or(0) as u32,
            category: r["cat"].as_str().unwrap_or("").to_string(),
        }).collect();
    }
    vec![]
}

#[tauri::command]
pub fn clean_target(target_name: String) -> CleanResult {
    let ps = match target_name.as_str() {
        "%TEMP%"               => format!("$b=0;$c=0;@(Get-ChildItem $env:TEMP -Recurse -File -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}};@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Windows\\Temp"        => format!("$b=0;$c=0;@(Get-ChildItem 'C:\\Windows\\Temp' -Recurse -File -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}};@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Prefetch"             => format!("$b=0;$c=0;@(Get-ChildItem 'C:\\Windows\\Prefetch\\*.pf' -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}};@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Dumps mémoire"        => format!("$b=0;$c=0;@(Get-ChildItem 'C:\\Windows\\Minidump\\*.dmp' -EA SilentlyContinue)+@(Get-Item 'C:\\Windows\\MEMORY.DMP' -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}};@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Corbeille"            => format!("Clear-RecycleBin -Force -EA SilentlyContinue;@{{freed=0;count=0;ok=$true}}|ConvertTo-Json -Compress"),
        "Chrome Cache"         => format!("$p=\"$env:LOCALAPPDATA\\Google\\Chrome\\User Data\\Default\\Cache\";$b=0;$c=0;if(Test-Path $p){{@(Get-ChildItem $p -Recurse -File -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}}}};@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Edge Cache"           => format!("$p=\"$env:LOCALAPPDATA\\Microsoft\\Edge\\User Data\\Default\\Cache\";$b=0;$c=0;if(Test-Path $p){{@(Get-ChildItem $p -Recurse -File -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}}}};@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Windows Update Cache" => format!("net stop wuauserv 2>&1|Out-Null;$b=0;$c=0;@(Get-ChildItem 'C:\\Windows\\SoftwareDistribution\\Download' -Recurse -File -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}};net start wuauserv 2>&1|Out-Null;@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        "Thumbnails DB"        => format!("Stop-Process -Name explorer -Force -EA SilentlyContinue;Start-Sleep 1;$p=\"$env:LOCALAPPDATA\\Microsoft\\Windows\\Explorer\";$b=0;$c=0;@(Get-ChildItem $p -Filter 'thumbcache_*.db' -EA SilentlyContinue)|ForEach-Object{{$b+=$_.Length;$c++;Remove-Item $_.FullName -Force -EA SilentlyContinue}};Start-Process explorer;@{{freed=[math]::Round($b/1MB,2);count=$c;ok=$true}}|ConvertTo-Json -Compress"),
        _ => return CleanResult { target: target_name, success: false, message: "Cible inconnue".to_string(), ..Default::default() },
    };
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",&ps]).creation_flags(0x08000000).output();
        if let Ok(o) = o {
            let t = String::from_utf8_lossy(&o.stdout);
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(t.trim()) {
                return CleanResult {
                    target: target_name,
                    freed_mb: v["freed"].as_f64().unwrap_or(0.0),
                    files_deleted: v["count"].as_u64().unwrap_or(0) as u32,
                    success: v["ok"].as_bool().unwrap_or(false),
                    message: String::new(),
                };
            }
        }
    }
    CleanResult { target: target_name, success: false, message: "Erreur".to_string(), ..Default::default() }
}

#[tauri::command]
pub fn get_large_files(folder: String, min_size_mb: f64) -> Vec<serde_json::Value> {
    let f = folder.replace('"', "").replace('\'', "");
    let min_bytes = (min_size_mb * 1048576.0) as u64;
    let ps = format!(r#"
@(Get-ChildItem '{folder}' -Recurse -File -EA SilentlyContinue |
    Where-Object {{ $_.Length -ge {min} }} |
    Sort-Object Length -Descending |
    Select-Object -First 100 |
    ForEach-Object {{ @{{ name=$_.Name; path=$_.FullName; mb=[math]::Round($_.Length/1MB,1); ext=$_.Extension; mod=[string]$_.LastWriteTime.ToString('yyyy-MM-dd') }} }}) | ConvertTo-Json -Compress
"#, folder=f, min=min_bytes);
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",&ps]).creation_flags(0x08000000).output();
        if let Ok(o) = o {
            let t = String::from_utf8_lossy(&o.stdout); let t = t.trim();
            let arr_t = if t.starts_with('{') { format!("[{}]",t) } else { t.to_string() };
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&arr_t) { return arr; }
        }
    }
    vec![]
}
