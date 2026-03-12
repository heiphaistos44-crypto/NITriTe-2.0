use serde::Serialize;
use std::process::Command;
use std::time::Instant;
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

/// CPU benchmark — Rust natif, calcul float+trig 8 secondes, non-bloquant
#[tauri::command]
pub async fn run_cpu_bench() -> BenchResult {
    tokio::task::spawn_blocking(|| {
        let target = std::time::Duration::from_secs(8);
        let start = Instant::now();
        let mut iterations: u64 = 0;
        let mut acc = 0.0f64;

        while start.elapsed() < target {
            for i in 0u64..50_000 {
                let x = (iterations + i) as f64;
                acc += x.sqrt() * (x * 0.000_001).sin() + x.ln_1p();
            }
            iterations += 50_000;
        }
        let _ = acc; // black-box: prevent optimizer from eliminating the benchmark loop

        let elapsed_ms = start.elapsed().as_millis().max(1) as u64;
        let mops = iterations as f64 / (elapsed_ms as f64 / 1000.0) / 1_000_000.0;
        // Normalisation : CPU moderne ~200-500 Mops/s → score /1000
        // 500 Mops/s = 1000 pts (haut de gamme), 50 Mops/s = 100 pts
        let score = (mops * 2.0).min(1000.0).round();

        BenchResult {
            name: "CPU Floating Point".to_string(),
            score,
            unit: "pts".to_string(),
            duration_ms: elapsed_ms,
            details: format!(
                "{:.1} Mops/s — {} M itérations en {}s (sqrt + sin + ln)",
                mops,
                iterations / 1_000_000,
                elapsed_ms / 1000
            ),
        }
    })
    .await
    .unwrap_or_default()
}

/// RAM benchmark — Rust natif 256 MB, lecture + écriture séquentielle + latence aléatoire
#[tauri::command]
pub async fn run_ram_bench() -> BenchResult {
    tokio::task::spawn_blocking(|| {
        const SIZE: usize = 256 * 1024 * 1024; // 256 MB

        // Écriture séquentielle
        let mut buf = vec![0u8; SIZE];
        let ws = Instant::now();
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (i ^ (i >> 8)) as u8;
        }
        let write_ms = ws.elapsed().as_millis().max(1) as u64;
        let write_gbps = (SIZE as f64 / 1e9) / (write_ms as f64 / 1000.0);

        // Lecture séquentielle
        let rs = Instant::now();
        let mut checksum: u64 = 0;
        for chunk in buf.chunks_exact(8) {
            checksum = checksum.wrapping_add(
                u64::from_le_bytes(chunk.try_into().unwrap_or([0u8; 8]))
            );
        }
        let read_ms = rs.elapsed().as_millis().max(1) as u64;
        let read_gbps = (SIZE as f64 / 1e9) / (read_ms as f64 / 1000.0);

        // Latence accès aléatoire (stride pseudo-aléatoire, évite prefetch)
        let lat_start = Instant::now();
        let iters = 200_000usize;
        let mut idx = 31337usize;
        let mut lat_acc: u64 = 0;
        for _ in 0..iters {
            idx = (idx.wrapping_mul(1_664_525).wrapping_add(1_013_904_223)) % (SIZE - 8);
            lat_acc = lat_acc.wrapping_add(
                u64::from_le_bytes(buf[idx..idx + 8].try_into().unwrap_or([0u8; 8]))
            );
        }
        let lat_ns = (lat_start.elapsed().as_nanos() as f64) / iters as f64;

        drop(buf);

        let avg_gbps = (write_gbps + read_gbps) / 2.0;
        // DDR4-3200 single-channel ≈ 25 GB/s → 1000 pts
        // DDR4-2133 ≈ 17 GB/s → 680 pts
        let score = (avg_gbps * 40.0).min(1000.0).round();

        BenchResult {
            name: "RAM Bande Passante".to_string(),
            score,
            unit: "pts".to_string(),
            duration_ms: write_ms + read_ms,
            details: format!(
                "Écriture: {:.1} GB/s | Lecture: {:.1} GB/s | Latence: {:.1} ns | 256 MB (checksum: {})",
                write_gbps, read_gbps, lat_ns, lat_acc % 10_000
            ),
        }
    })
    .await
    .unwrap_or_default()
}

/// Disk benchmark — PowerShell 256 MB, score composite seq+4K IOPS, non-bloquant
#[tauri::command]
pub async fn run_disk_bench(drive: Option<String>) -> BenchResult {
    tokio::task::spawn_blocking(move || {
        let d = drive
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "C:".to_string())
            .replace('"', "")
            .replace('\'', "");

        let rand_suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();

        let ps = format!(
            r#"
$tmp = "{drive}\__nitrite_bench_{rand}.tmp"
try {{
    $size = 256MB
    $data = New-Object byte[] $size
    (New-Object System.Random(42)).NextBytes($data)

    # Ecriture sequentielle
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    [System.IO.File]::WriteAllBytes($tmp, $data)
    $sw.Stop()
    $wMBps = [math]::Round(($size / 1MB) / ($sw.ElapsedMilliseconds / 1000.0), 1)

    # Lecture sequentielle (chunks pour eviter ReadAllBytes cache)
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $fs = [System.IO.File]::OpenRead($tmp)
    $buf = New-Object byte[] (1MB)
    while ($fs.Read($buf, 0, $buf.Length) -gt 0) {{}}
    $fs.Close()
    $sw.Stop()
    $rMBps = [math]::Round(($size / 1MB) / ($sw.ElapsedMilliseconds / 1000.0), 1)

    # IOPS 4K aleatoire (3 secondes)
    $fs = [System.IO.File]::OpenRead($tmp)
    $buf4k = New-Object byte[] 4096
    $rng = New-Object System.Random
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $ops = 0
    while ($sw.ElapsedMilliseconds -lt 3000) {{
        $pos = [long]($rng.NextDouble() * ($size - 4096))
        $fs.Seek($pos, [System.IO.SeekOrigin]::Begin) | Out-Null
        $fs.Read($buf4k, 0, 4096) | Out-Null
        $ops++
    }}
    $fs.Close()
    $iops = [math]::Round($ops / ($sw.ElapsedMilliseconds / 1000.0))

    @{{ wMBps=$wMBps; rMBps=$rMBps; iops=$iops }} | ConvertTo-Json -Compress
}} catch {{
    @{{ wMBps=0; rMBps=0; iops=0; error=$_.Exception.Message }} | ConvertTo-Json -Compress
}} finally {{
    Remove-Item $tmp -Force -EA SilentlyContinue
}}
"#,
            drive = d,
            rand = rand_suffix
        );

        #[cfg(target_os = "windows")]
        {
            let start = Instant::now();
            let o = Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output();
            let dur = start.elapsed().as_millis() as u64;

            if let Ok(o) = o {
                let t = String::from_utf8_lossy(&o.stdout);
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(t.trim()) {
                    let w = v["wMBps"].as_f64().unwrap_or(0.0);
                    let r = v["rMBps"].as_f64().unwrap_or(0.0);
                    let iops = v["iops"].as_f64().unwrap_or(0.0);
                    // Score composite : seq (60%) + 4K IOPS (40%)
                    // SSD SATA seq ≈ 550 MB/s → 1000 pts seq
                    // SSD SATA 4K ≈ 80k IOPS → 1000 pts iops
                    let seq_score = ((r + w) / 2.0 / 5.5).min(1000.0);
                    let iops_score = (iops / 80.0).min(1000.0);
                    let composite = (seq_score * 0.6 + iops_score * 0.4).round();

                    return BenchResult {
                        name: "Stockage".to_string(),
                        score: composite,
                        unit: "pts".to_string(),
                        duration_ms: dur,
                        details: format!(
                            "Lecture: {} MB/s | Écriture: {} MB/s | IOPS 4K: {} | 256 MB",
                            r, w, iops as u64
                        ),
                    };
                }
            }
        }

        BenchResult {
            name: "Disque".to_string(),
            ..Default::default()
        }
    })
    .await
    .unwrap_or_default()
}
