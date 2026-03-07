use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use tauri::Emitter;

use crate::error::NiTriTeError;

#[derive(Debug, Clone, Serialize)]
pub struct WingetPackage {
    pub name: String,
    pub id: String,
    pub version: String,
    pub available: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstallResult {
    pub success: bool,
    pub app_id: String,
    pub message: String,
}

pub fn check_winget() -> bool {
    Command::new("winget").arg("--version")
        .stdout(Stdio::null()).stderr(Stdio::null())
        .creation_flags(0x08000000)
        .status().is_ok()
}

pub fn list_upgradable() -> Result<Vec<WingetPackage>, NiTriTeError> {
    let output = Command::new("winget")
        .args(["upgrade", "--accept-source-agreements"])
        .creation_flags(0x08000000).output()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();

    let lines: Vec<&str> = text.lines().collect();
    let header_idx = lines.iter().position(|l| l.contains("----"));

    if let Some(idx) = header_idx {
        for line in &lines[idx + 1..] {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.contains("upgrade") { continue; }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 {
                packages.push(WingetPackage {
                    name: parts[..parts.len()-3].join(" "),
                    id: parts[parts.len()-3].to_string(),
                    version: parts[parts.len()-2].to_string(),
                    available: parts[parts.len()-1].to_string(),
                    source: "winget".to_string(),
                });
            }
        }
    }

    Ok(packages)
}

pub fn install_package(
    package_id: &str,
    window: &tauri::Window,
) -> Result<InstallResult, NiTriTeError> {
    let mut child = Command::new("winget")
        .args([
            "install", "--id", package_id, "--exact", "--silent",
            "--accept-source-agreements", "--accept-package-agreements",
            "--disable-interactivity",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let _ = window.emit("install-log", serde_json::json!({
                "app_id": package_id,
                "line": line,
                "level": if line.contains("error") || line.contains("Error") { "error" }
                         else if line.contains("Successfully") { "success" }
                         else { "info" },
            }));
        }
    }

    let status = child.wait()?;

    Ok(InstallResult {
        success: status.success(),
        app_id: package_id.to_string(),
        message: if status.success() { "Installation reussie".into() } else { format!("Code: {}", status.code().unwrap_or(-1)) },
    })
}

pub fn upgrade_all(window: &tauri::Window) -> Result<(), NiTriTeError> {
    let mut child = Command::new("winget")
        .args(["upgrade", "--all", "--silent", "--accept-source-agreements", "--accept-package-agreements"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            let _ = window.emit("upgrade-log", &line);
        }
    }

    child.wait()?;
    Ok(())
}

pub fn search_packages(query: &str) -> Result<Vec<WingetPackage>, NiTriTeError> {
    let output = Command::new("winget")
        .args(["search", query, "--accept-source-agreements"])
        .creation_flags(0x08000000).output()?;

    let text = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();

    let lines: Vec<&str> = text.lines().collect();
    let header_idx = lines.iter().position(|l| l.contains("----"));

    if let Some(idx) = header_idx {
        for line in &lines[idx + 1..] {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                packages.push(WingetPackage {
                    name: parts[..parts.len()-2].join(" "),
                    id: parts[parts.len()-2].to_string(),
                    version: parts[parts.len()-1].to_string(),
                    available: String::new(),
                    source: "winget".to_string(),
                });
            }
        }
    }

    Ok(packages)
}
