use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppStats {
    pub installs_count: u64,
    pub cleanups_count: u64,
    pub scans_count: u64,
    pub scripts_run: u64,
    pub exports_count: u64,
    pub actions_today: u64,
    pub last_action: String,
    pub action_counts: HashMap<String, u64>,
}

fn stats_path() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("NiTriTe");
    let _ = fs::create_dir_all(&base);
    base.join("stats.json")
}

fn load_stats() -> AppStats {
    let path = stats_path();
    if let Ok(raw) = fs::read_to_string(&path) {
        serde_json::from_str(&raw).unwrap_or_default()
    } else {
        AppStats::default()
    }
}

fn save_stats(stats: &AppStats) -> Result<(), String> {
    let json = serde_json::to_string_pretty(stats).map_err(|e| e.to_string())?;
    fs::write(stats_path(), json).map_err(|e| e.to_string())
}

/// Retourne les statistiques d'utilisation
#[tauri::command]
pub fn get_app_stats() -> AppStats {
    load_stats()
}

/// Incrémente un compteur nommé
#[tauri::command]
pub fn log_action(action: String) -> Result<(), String> {
    let mut stats = load_stats();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    stats.last_action = now;

    *stats.action_counts.entry(action.clone()).or_insert(0) += 1;

    match action.as_str() {
        "install" => stats.installs_count += 1,
        "cleanup" => stats.cleanups_count += 1,
        "scan"    => stats.scans_count += 1,
        "script"  => stats.scripts_run += 1,
        "export"  => stats.exports_count += 1,
        _ => {}
    }
    stats.actions_today += 1;

    save_stats(&stats)
}

/// Remet les statistiques à zéro
#[tauri::command]
pub fn reset_stats() -> Result<(), String> {
    save_stats(&AppStats::default())
}
