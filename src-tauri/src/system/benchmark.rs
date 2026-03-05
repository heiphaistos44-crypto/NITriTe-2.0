use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Default)]
pub struct BenchResult {
    pub name: String,
    pub score: f64,
    pub unit: String,
    pub duration_ms: u64,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct BenchReport {
    pub cpu: BenchResult,
    pub ram: BenchResult,
    pub disk_seq_read: BenchResult,
    pub disk_seq_write: BenchResult,
    pub disk_rand_read: BenchResult,
    pub timestamp: String,
}

#[tauri::command]
pub fn run_cpu_bench() -> BenchResult {
    let ps = r#"
$sw = [System.Diagnostics.Stopwatch]::StartNew()
$iterations = 5000000
$sum = 0.0
for ($i = 1; $i -le $iterations; $i++) {
    $sum += [math]::Sqrt($i) * [math]::Sin($i)
}
$sw.Stop()
$ms = $sw.ElapsedMilliseconds
$score = [math]::Round($iterations / ($ms / 1000.0) / 1000.0, 1)
@{ score=$score; ms=$ms; detail="$iterations iterations; somme=$([math]::Round($sum,2))" } | ConvertTo-Json -Compress
"#;
    #[cfg(target_os = "windows")]
    {
        let start = std::time::Instant::now();
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",ps]).creation_flags(0x08000000).output();
        let dur = start.elapsed().as_millis() as u64;
        if let Ok(o) = o {
            let t = String::from_utf8_lossy(&o.stdout);
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(t.trim()) {
                return BenchResult {
                    name: "CPU Single-thread".to_string(),
                    score: v["score"].as_f64().unwrap_or(0.0),
                    unit: "Kops/s".to_string(),
                    duration_ms: v["ms"].as_u64().unwrap_or(dur),
                    details: v["detail"].as_str().unwrap_or("").to_string(),
                };
            }
        }
    }
    BenchResult { name: "CPU".to_string(), ..Default::default() }
}

#[tauri::command]
pub fn run_ram_bench() -> BenchResult {
    let ps = r#"
$sw = [System.Diagnostics.Stopwatch]::StartNew()
$size = 64MB
$arr = New-Object byte[] $size
$rng = New-Object System.Random
$rng.NextBytes($arr)
$sw.Stop()
$ms = $sw.ElapsedMilliseconds
$gbps = [math]::Round(($size / 1073741824.0) / ($ms / 1000.0), 2)
@{ score=$gbps; ms=$ms; detail="Allocation + remplissage ${size}B" } | ConvertTo-Json -Compress
"#;
    #[cfg(target_os = "windows")]
    {
        let start = std::time::Instant::now();
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",ps]).creation_flags(0x08000000).output();
        let dur = start.elapsed().as_millis() as u64;
        if let Ok(o) = o {
            let t = String::from_utf8_lossy(&o.stdout);
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(t.trim()) {
                return BenchResult {
                    name: "RAM Write (64MB)".to_string(),
                    score: v["score"].as_f64().unwrap_or(0.0),
                    unit: "GB/s".to_string(),
                    duration_ms: v["ms"].as_u64().unwrap_or(dur),
                    details: v["detail"].as_str().unwrap_or("").to_string(),
                };
            }
        }
    }
    BenchResult { name: "RAM".to_string(), ..Default::default() }
}

#[tauri::command]
pub fn run_disk_bench(drive: String) -> Vec<BenchResult> {
    let d = if drive.is_empty() { "C:".to_string() } else { drive.replace('"', "").replace('\'', "") };
    let ps = format!(r#"
$tmp = "{drive}\__nitrite_bench_$([System.IO.Path]::GetRandomFileName()).tmp"
$results = @()
try {{
    $size = 128MB
    $data = New-Object byte[] $size
    (New-Object System.Random).NextBytes($data)

    # Sequential Write
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    [System.IO.File]::WriteAllBytes($tmp, $data)
    $sw.Stop()
    $wMBps = [math]::Round(($size/1MB) / ($sw.ElapsedMilliseconds/1000), 1)
    $results += @{{ name='Seq Write'; score=$wMBps; unit='MB/s'; ms=$sw.ElapsedMilliseconds }}

    # Sequential Read
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $read = [System.IO.File]::ReadAllBytes($tmp)
    $sw.Stop()
    $rMBps = [math]::Round(($size/1MB) / ($sw.ElapsedMilliseconds/1000), 1)
    $results += @{{ name='Seq Read'; score=$rMBps; unit='MB/s'; ms=$sw.ElapsedMilliseconds }}

    # Random Read (4K blocks)
    $fs = [System.IO.File]::OpenRead($tmp)
    $buf = New-Object byte[] 4096
    $rng = New-Object System.Random
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $ops = 0
    while ($sw.ElapsedMilliseconds -lt 2000) {{
        $pos = [long]($rng.NextDouble() * ($size - 4096))
        $fs.Seek($pos, 0) | Out-Null
        $fs.Read($buf, 0, 4096) | Out-Null
        $ops++
    }}
    $fs.Close()
    $sw.Stop()
    $iops = [math]::Round($ops / ($sw.ElapsedMilliseconds/1000), 0)
    $results += @{{ name='Rand Read 4K'; score=$iops; unit='IOPS'; ms=$sw.ElapsedMilliseconds }}
}} finally {{
    Remove-Item $tmp -Force -EA SilentlyContinue
}}
$results | ConvertTo-Json -Compress
"#, drive=d);
    #[cfg(target_os = "windows")]
    {
        let o = Command::new("powershell").args(["-NoProfile","-NonInteractive","-Command",&ps]).creation_flags(0x08000000).output();
        if let Ok(o) = o {
            let t = String::from_utf8_lossy(&o.stdout); let t = t.trim();
            let arr_t = if t.starts_with('{') { format!("[{}]",t) } else { t.to_string() };
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&arr_t) {
                return arr.iter().map(|r| BenchResult {
                    name: r["name"].as_str().unwrap_or("").to_string(),
                    score: r["score"].as_f64().unwrap_or(0.0),
                    unit: r["unit"].as_str().unwrap_or("").to_string(),
                    duration_ms: r["ms"].as_u64().unwrap_or(0),
                    details: format!("Disque {}", d),
                }).collect();
            }
        }
    }
    vec![]
}
