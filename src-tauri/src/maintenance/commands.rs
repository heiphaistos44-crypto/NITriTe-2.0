use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::error::NiTriTeError;

#[derive(Debug, Clone, Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Execute une commande systeme (avec confirmation cote frontend)
pub fn execute_system_command(cmd: &str, args: &[&str], _timeout_secs: u64) -> Result<CommandResult, NiTriTeError> {
    let output = Command::new(cmd)
        .args(args)
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| NiTriTeError::System(format!("Erreur execution {}: {}", cmd, e)))?;

    Ok(CommandResult {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
    })
}

/// Lance SFC /scannow
pub fn run_sfc() -> Result<CommandResult, NiTriTeError> {
    execute_system_command("sfc", &["/scannow"], 300)
}

/// Lance DISM RestoreHealth
pub fn run_dism_restore() -> Result<CommandResult, NiTriTeError> {
    execute_system_command("DISM", &["/Online", "/Cleanup-Image", "/RestoreHealth"], 600)
}

/// Liste les drivers installes
pub fn list_drivers() -> Result<CommandResult, NiTriTeError> {
    execute_system_command("driverquery", &["/v", "/fo", "csv"], 30)
}
