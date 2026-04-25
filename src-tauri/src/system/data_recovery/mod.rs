/// data_recovery/mod.rs — Re-exports de tous les sous-modules
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub mod shadow_vss;
pub mod disk_recovery;
pub mod backup_folders;

pub use shadow_vss::*;
pub use disk_recovery::*;
pub use backup_folders::*;

// ─── Utilitaire PowerShell partagé ────────────────────────────────────────────

pub(super) fn run_ps(script: &str) -> Option<String> {
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
