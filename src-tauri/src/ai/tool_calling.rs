use serde::Serialize;
use std::collections::HashSet;
use std::process::Command;

use crate::error::NiTriTeError;

#[derive(Debug, Clone, Serialize)]
pub struct SafeCommandResult {
    pub command: String,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

/// Whitelist de commandes autorisees (lecture seule)
fn safe_commands() -> HashSet<&'static str> {
    [
        "systeminfo", "tasklist", "ipconfig", "netstat", "ping", "tracert",
        "nslookup", "hostname", "ver", "wmic", "driverquery", "whoami",
        "powercfg", "Get-Process", "Get-Service", "Get-NetAdapter",
        "Get-ComputerInfo", "Get-Volume", "cleanmgr", "msinfo32",
    ].into_iter().collect()
}

/// Blacklist absolue
fn blocked_commands() -> HashSet<&'static str> {
    [
        "del", "rm", "rmdir", "format", "fdisk", "diskpart", "shutdown",
        "restart", "reboot", "taskkill", "reg", "Remove-Item", "net",
        "takeown", "icacls", "cipher", "bcdedit", "bcdboot",
    ].into_iter().collect()
}

/// Caracteres suspects
fn has_injection(cmd: &str) -> bool {
    ["&&", "||", "|", ";", ">", ">>", "<", "`", "$(",].iter().any(|c| cmd.contains(c))
}

pub fn is_safe(command: &str) -> Result<(), NiTriTeError> {
    let first_word = command.split_whitespace().next().unwrap_or("").to_lowercase();

    if blocked_commands().iter().any(|b| first_word == b.to_lowercase()) {
        return Err(NiTriTeError::CommandDenied(format!("Commande interdite: {}", first_word)));
    }

    if has_injection(command) {
        return Err(NiTriTeError::CommandDenied("Caracteres d'injection detectes".into()));
    }

    if !safe_commands().iter().any(|s| first_word == s.to_lowercase()) {
        return Err(NiTriTeError::CommandDenied(format!("Commande non autorisee: {}", first_word)));
    }

    Ok(())
}

pub fn execute_safe(command: &str) -> Result<SafeCommandResult, NiTriTeError> {
    is_safe(command)?;

    let output = Command::new("cmd")
        .args(["/C", command])
        .output()
        .map_err(|e| NiTriTeError::System(e.to_string()))?;

    Ok(SafeCommandResult {
        command: command.to_string(),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}
