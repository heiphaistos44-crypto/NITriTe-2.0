use serde::Serialize;
use std::net::UdpSocket;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::error::NiTriTeError;

fn get_hostname() -> String {
    Command::new("hostname")
        .creation_flags(0x08000000)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkOverview {
    pub local_ip: String,
    pub hostname: String,
    pub public_ip: Option<String>,
    pub interfaces: Vec<NetworkInterface>,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip: String,
    pub mac: String,
    pub speed: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionInfo {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
    pub pid: u32,
    pub process_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PingResult {
    pub host: String,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub packet_loss: f64,
    pub success: bool,
}

pub fn get_network_overview() -> Result<NetworkOverview, NiTriTeError> {
    let local_ip = get_local_ip();
    let hostname = get_hostname();
    let interfaces = get_interfaces()?;
    let dns_servers = get_dns_servers();

    Ok(NetworkOverview {
        local_ip,
        hostname,
        public_ip: None, // Sera récupéré côté frontend via HTTP
        interfaces,
        dns_servers,
    })
}

fn get_local_ip() -> String {
    UdpSocket::bind("0.0.0.0:0")
        .and_then(|socket| {
            socket.connect("8.8.8.8:80")?;
            socket.local_addr()
        })
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn get_interfaces() -> Result<Vec<NetworkInterface>, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-NetAdapter | Select-Object Name, Status, MacAddress, LinkSpeed | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let parsed: Result<Vec<serde_json::Value>, _> = serde_json::from_str(&text);

    match parsed {
        Ok(adapters) => Ok(adapters.iter().map(|a| NetworkInterface {
            name: a["Name"].as_str().unwrap_or("").to_string(),
            ip: String::new(),
            mac: a["MacAddress"].as_str().unwrap_or("").to_string(),
            speed: a["LinkSpeed"].as_str().unwrap_or("").to_string(),
            status: a["Status"].as_str().unwrap_or("").to_string(),
        }).collect()),
        Err(_) => {
            // Fallback si une seule interface
            if let Ok(single) = serde_json::from_str::<serde_json::Value>(&text) {
                Ok(vec![NetworkInterface {
                    name: single["Name"].as_str().unwrap_or("").to_string(),
                    ip: String::new(),
                    mac: single["MacAddress"].as_str().unwrap_or("").to_string(),
                    speed: single["LinkSpeed"].as_str().unwrap_or("").to_string(),
                    status: single["Status"].as_str().unwrap_or("").to_string(),
                }])
            } else {
                Ok(vec![])
            }
        }
    }
}

fn get_dns_servers() -> Vec<String> {
    Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "(Get-DnsClientServerAddress -AddressFamily IPv4).ServerAddresses | Select-Object -Unique"])
        .creation_flags(0x08000000)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout)
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string())
            .collect())
        .unwrap_or_default()
}

pub fn get_connections() -> Result<Vec<ConnectionInfo>, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-NetTCPConnection | Select-Object -First 100 LocalAddress, LocalPort, RemoteAddress, RemotePort, State, OwningProcess | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap_or_default();

    // Resoudre les noms de process via sysinfo
    let mut sys = sysinfo::System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    Ok(parsed.iter().map(|c| {
        let pid = c["OwningProcess"].as_u64().unwrap_or(0) as u32;
        let process_name = sys.process(sysinfo::Pid::from_u32(pid))
            .map(|p| p.name().to_string_lossy().to_string())
            .unwrap_or_default();
        ConnectionInfo {
            protocol: "TCP".to_string(),
            local_addr: format!("{}:{}", c["LocalAddress"].as_str().unwrap_or(""), c["LocalPort"].as_u64().unwrap_or(0)),
            remote_addr: format!("{}:{}", c["RemoteAddress"].as_str().unwrap_or(""), c["RemotePort"].as_u64().unwrap_or(0)),
            state: c["State"].as_u64().map(|s| match s { 2 => "Listen", 5 => "Established", 6 => "FinWait1", 8 => "CloseWait", 11 => "TimeWait", _ => "Other" }).unwrap_or("Unknown").to_string(),
            pid,
            process_name,
        }
    }).collect())
}

pub fn ping_host(host: &str) -> Result<PingResult, NiTriTeError> {
    let output = Command::new("ping")
        .args(["-n", "4", host])
        .creation_flags(0x08000000).output()?;

    let text = String::from_utf8_lossy(&output.stdout).to_string();
    let success = output.status.success();

    // Parse basique des résultats
    let mut min = 0.0f64;
    let mut max = 0.0f64;
    let mut avg = 0.0f64;

    for line in text.lines() {
        if line.contains("Minimum") || line.contains("Maximum") {
            // "Minimum = 1ms, Maximum = 3ms, Moyenne = 2ms"
            let parts: Vec<&str> = line.split(',').collect();
            for part in &parts {
                let clean = part.replace("ms", "").replace("Minimum", "").replace("Maximum", "").replace("Moyenne", "").replace("Average", "").replace("=", "").trim().to_string();
                if let Ok(v) = clean.parse::<f64>() {
                    if part.contains("Minimum") || part.contains("Min") { min = v; }
                    else if part.contains("Maximum") || part.contains("Max") { max = v; }
                    else { avg = v; }
                }
            }
        }
    }

    Ok(PingResult {
        host: host.to_string(),
        avg_ms: avg,
        min_ms: min,
        max_ms: max,
        packet_loss: if success { 0.0 } else { 100.0 },
        success,
    })
}
