pub mod error;
pub mod state;
pub mod system;
pub mod utils;
pub mod installer;
pub mod maintenance;
pub mod backup;
pub mod ai;
pub mod scripts;

use std::sync::atomic::Ordering;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use tauri::Emitter;
use crate::error::NiTriTeError;
use crate::state::AppState;
use crate::system::info::{self, SystemInfo};
use crate::utils::config::AppConfig;
use crate::utils::platform::PlatformInfo;

// === Commandes Systeme ===

#[tauri::command]
async fn get_system_info() -> Result<SystemInfo, NiTriTeError> {
    tokio::task::spawn_blocking(info::collect_system_info)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn get_platform_info() -> Result<PlatformInfo, NiTriTeError> {
    Ok(tokio::task::spawn_blocking(PlatformInfo::detect)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?)
}

// === Monitoring ===

#[tauri::command]
async fn start_monitoring(
    window: tauri::Window,
    state: tauri::State<'_, AppState>,
) -> Result<(), NiTriTeError> {
    let interval = {
        let config = state.config.lock().await;
        config.monitor_interval_ms
    };
    let running = state.monitor_running.clone();
    system::monitor::start_monitoring(window, running, interval);
    Ok(())
}

#[tauri::command]
async fn stop_monitoring(state: tauri::State<'_, AppState>) -> Result<(), NiTriTeError> {
    state.monitor_running.store(false, Ordering::SeqCst);
    Ok(())
}

// === Reseau ===

#[tauri::command]
async fn get_network_overview() -> Result<system::network::NetworkOverview, NiTriTeError> {
    tokio::task::spawn_blocking(system::network::get_network_overview)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn get_connections() -> Result<Vec<system::network::ConnectionInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::network::get_connections)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn ping_host(host: String) -> Result<system::network::PingResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::network::ping_host(&host))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Installer ===

#[tauri::command]
async fn get_apps() -> Result<Vec<installer::manager::AppEntry>, NiTriTeError> {
    Ok(installer::manager::get_default_apps())
}

#[tauri::command]
async fn get_tools() -> Result<Vec<installer::manager::ToolEntry>, NiTriTeError> {
    Ok(installer::manager::get_tools())
}

#[tauri::command]
async fn install_app(app_id: Option<String>, winget_id: Option<String>, window: tauri::Window) -> Result<installer::winget::InstallResult, NiTriTeError> {
    // Resoudre l'ID winget : soit fourni directement, soit lookup depuis programs.json via app_id
    let resolved_id = if let Some(wid) = winget_id.filter(|w| !w.is_empty()) {
        wid
    } else if let Some(aid) = app_id {
        let apps = installer::manager::get_default_apps();
        apps.iter()
            .find(|a| a.id == aid)
            .and_then(|a| a.winget_id.clone())
            .unwrap_or(aid)
    } else {
        return Err(NiTriTeError::System("Aucun identifiant d'application fourni".into()));
    };
    tokio::task::spawn_blocking(move || installer::winget::install_package(&resolved_id, &window))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn check_winget() -> Result<bool, NiTriTeError> {
    Ok(installer::winget::check_winget())
}

#[tauri::command]
async fn list_upgradable() -> Result<Vec<installer::winget::WingetPackage>, NiTriTeError> {
    tokio::task::spawn_blocking(installer::winget::list_upgradable)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn upgrade_all(window: tauri::Window) -> Result<(), NiTriTeError> {
    tokio::task::spawn_blocking(move || installer::winget::upgrade_all(&window))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Terminal multi-shell ===

#[tauri::command]
async fn detect_shells() -> Result<Vec<maintenance::terminal::ShellInfo>, NiTriTeError> {
    Ok(tokio::task::spawn_blocking(maintenance::terminal::detect_shells)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?)
}

#[tauri::command]
async fn run_in_shell(shell_id: String, command: String) -> Result<maintenance::terminal::ShellResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || maintenance::terminal::run_in_shell(&shell_id, &command, 120))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Browser Cleanup ===

#[tauri::command]
async fn get_browser_cache_sizes() -> Result<Vec<maintenance::browser_cleanup::BrowserCacheInfo>, NiTriTeError> {
    Ok(tokio::task::spawn_blocking(maintenance::browser_cleanup::get_browser_cache_sizes)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?)
}

#[tauri::command]
async fn clean_browser_cache(browser_ids: Vec<String>) -> Result<Vec<maintenance::browser_cleanup::CleanupResult>, NiTriTeError> {
    tokio::task::spawn_blocking(move || maintenance::browser_cleanup::clean_browser_cache(browser_ids))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Event Logs ===

#[tauri::command]
async fn get_event_logs(log_name: String, count: u32) -> Result<Vec<system::eventlog::EventLogEntry>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::eventlog::get_event_logs(&log_name, count))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Drivers Recommandes ===

#[tauri::command]
async fn get_recommended_drivers() -> Result<Vec<system::drivers::DriverStatus>, NiTriTeError> {
    tokio::task::spawn_blocking(system::drivers::get_recommended_drivers)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Scripts File Management ===

#[tauri::command]
async fn list_script_files(dir: String) -> Result<Vec<scripts::executor::ScriptFileInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(move || scripts::executor::list_script_files(&dir))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn read_script_file(path: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || scripts::executor::read_script_file(&path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn save_script_file(path: String, content: String) -> Result<(), NiTriTeError> {
    tokio::task::spawn_blocking(move || scripts::executor::save_script_file(&path, &content))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Reports ===

#[tauri::command]
async fn list_reports() -> Result<Vec<ReportInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(|| {
        let backup_dir = utils::paths::backups_dir();
        let mut reports = Vec::new();

        if backup_dir.exists() {
            for entry in std::fs::read_dir(&backup_dir).map_err(NiTriTeError::Io)? {
                let entry = entry.map_err(NiTriTeError::Io)?;
                let meta = entry.metadata().map_err(NiTriTeError::Io)?;
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".json") || name.ends_with(".txt") {
                    let modified_secs = meta.modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let created = chrono::DateTime::from_timestamp(modified_secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                        .unwrap_or_else(|| "Inconnu".to_string());
                    reports.push(ReportInfo {
                        name,
                        path: entry.path().to_string_lossy().to_string(),
                        size_bytes: meta.len(),
                        created,
                    });
                }
            }
        }

        reports.sort_by(|a, b| b.created.cmp(&a.created));
        Ok(reports)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Maintenance ===

#[tauri::command]
async fn empty_recycle_bin() -> Result<maintenance::cleanup::CleanupResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::cleanup::empty_recycle_bin)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn clean_temp_files() -> Result<maintenance::cleanup::CleanupResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::cleanup::clean_temp_files)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn run_disk_cleanup() -> Result<maintenance::cleanup::CleanupResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::cleanup::run_disk_cleanup)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn get_startup_programs() -> Result<Vec<maintenance::cleanup::StartupEntry>, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::cleanup::get_startup_programs)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn run_system_command(cmd: String, args: Vec<String>) -> Result<maintenance::commands::CommandResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        maintenance::commands::execute_system_command(&cmd, &args_refs, 60)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn disable_startup_program(name: String, location: String) -> Result<maintenance::cleanup::CleanupResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || maintenance::cleanup::disable_startup_program(&name, &location))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Backup ===

#[tauri::command]
async fn create_backup(items: Vec<String>) -> Result<backup::collector::BackupManifest, NiTriTeError> {
    tokio::task::spawn_blocking(move || backup::collector::create_backup(items))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn list_backups() -> Result<Vec<String>, NiTriTeError> {
    tokio::task::spawn_blocking(backup::collector::list_backups)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === AI ===

#[tauri::command]
async fn ai_check(state: tauri::State<'_, AppState>) -> Result<bool, NiTriTeError> {
    let url = {
        let config = state.config.lock().await;
        config.ollama_url.clone()
    };
    Ok(ai::ollama::check_ollama(&url).await)
}

#[tauri::command]
async fn ai_list_models(state: tauri::State<'_, AppState>) -> Result<Vec<ai::ollama::OllamaModel>, NiTriTeError> {
    let url = {
        let config = state.config.lock().await;
        config.ollama_url.clone()
    };
    ai::ollama::list_models(&url).await
}

#[tauri::command]
async fn ai_query(
    prompt: String,
    model: Option<String>,
    system_prompt: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<String, NiTriTeError> {
    let (url, default_model, temp) = {
        let config = state.config.lock().await;
        (config.ollama_url.clone(), config.ollama_model.clone(), 0.7)
    };
    let m = model.unwrap_or(default_model);
    ai::ollama::query(&url, &m, &prompt, system_prompt.as_deref(), temp).await
}

#[tauri::command]
async fn ai_execute_command(command: String) -> Result<ai::tool_calling::SafeCommandResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || ai::tool_calling::execute_safe(&command))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Scripts ===

#[tauri::command]
async fn get_builtin_scripts() -> Result<Vec<scripts::executor::ScriptEntry>, NiTriTeError> {
    Ok(scripts::executor::get_builtin_scripts())
}

#[tauri::command]
async fn execute_script(
    content: String,
    script_type: String,
    window: tauri::Window,
) -> Result<scripts::executor::ScriptResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || scripts::executor::execute_script(&content, &script_type, &window))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Logs ===

#[derive(serde::Serialize)]
struct ReportInfo {
    name: String,
    path: String,
    size_bytes: u64,
    created: String,
}

#[derive(serde::Serialize)]
struct AppLogEntry {
    timestamp: String,
    level: String,
    message: String,
}

#[tauri::command]
async fn get_app_logs() -> Result<Vec<AppLogEntry>, NiTriTeError> {
    tokio::task::spawn_blocking(|| {
        let logs_dir = utils::paths::logs_dir();
        let mut entries: Vec<AppLogEntry> = Vec::new();

        // Lire tous les fichiers .log dans le dossier
        let mut log_files: Vec<_> = std::fs::read_dir(&logs_dir)
            .map_err(|e| NiTriTeError::Io(e))?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .to_str()
                    .map(|s| s.contains("nitrite.log"))
                    .unwrap_or(false)
            })
            .collect();

        // Trier par date de modification (plus recent en dernier)
        log_files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());

        // Garder les 3 derniers fichiers max
        let files_to_read: Vec<_> = log_files.into_iter().rev().take(3).collect();

        for entry in files_to_read.into_iter().rev() {
            let content = match std::fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for line in content.lines() {
                // Format tracing: 2026-03-01T10:00:00.123Z  INFO nitrite_lib: message
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Extraire timestamp, level, message
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    entries.push(AppLogEntry {
                        timestamp: String::new(),
                        level: "INFO".to_string(),
                        message: line.to_string(),
                    });
                    continue;
                }

                let timestamp = parts[0].to_string();
                let rest = parts[1].trim();

                // Trouver le level (INFO, WARN, ERROR, DEBUG, TRACE)
                let (level, msg) = if rest.starts_with("INFO") {
                    ("INFO", rest[4..].trim().trim_start_matches(|c: char| !c.is_alphabetic()))
                } else if rest.starts_with("WARN") {
                    ("WARN", rest[4..].trim().trim_start_matches(|c: char| !c.is_alphabetic()))
                } else if rest.starts_with("ERROR") {
                    ("ERROR", rest[5..].trim().trim_start_matches(|c: char| !c.is_alphabetic()))
                } else if rest.starts_with("DEBUG") {
                    ("INFO", rest[5..].trim().trim_start_matches(|c: char| !c.is_alphabetic()))
                } else {
                    ("INFO", rest)
                };

                // Nettoyer le prefix module (ex: "nitrite_lib: ")
                let message = msg
                    .trim_start_matches(|c: char| c.is_alphanumeric() || c == '_' || c == ':')
                    .trim()
                    .to_string();

                let message = if message.is_empty() { msg.to_string() } else { message };

                entries.push(AppLogEntry {
                    timestamp,
                    level: level.to_string(),
                    message,
                });
            }
        }

        // Limiter a 500 entrees
        if entries.len() > 500 {
            entries = entries.split_off(entries.len() - 500);
        }

        Ok(entries)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Portables ===

#[tauri::command]
async fn get_portable_apps() -> Result<Vec<installer::portables::PortableApp>, NiTriTeError> {
    Ok(installer::portables::get_all_portables())
}

#[tauri::command]
async fn check_portable_installed(app_id: String) -> Result<bool, NiTriTeError> {
    let portables_dir = utils::paths::portables_dir();
    let app_dir = portables_dir.join(&app_id);
    Ok(app_dir.exists())
}

#[tauri::command]
async fn launch_portable(app_id: String) -> Result<(), NiTriTeError> {
    let portables = installer::portables::get_all_portables();
    let app = portables.iter().find(|a| a.id == app_id)
        .ok_or_else(|| NiTriTeError::System(format!("Application {} non trouvee", app_id)))?;

    let portables_dir = utils::paths::portables_dir();
    let exe_path = portables_dir.join(&app.folder_name).join(&app.exe_name);

    if !exe_path.exists() {
        return Err(NiTriTeError::System(format!(
            "Executable non trouve: {}. Telechargez d'abord l'application.",
            exe_path.display()
        )));
    }

    std::process::Command::new(&exe_path)
        .spawn()
        .map_err(|e| NiTriTeError::System(format!("Impossible de lancer {}: {}", app.name, e)))?;

    Ok(())
}

#[tauri::command]
async fn open_portables_dir() -> Result<(), NiTriTeError> {
    let dir = utils::paths::portables_dir();
    open::that(&dir).map_err(|e| NiTriTeError::System(e.to_string()))
}

/// Lance un .exe par chemin relatif depuis le dossier logiciel/
#[tauri::command]
async fn launch_exe(relative_path: String) -> Result<(), NiTriTeError> {
    let base = utils::paths::portables_dir();
    let full = base.join(std::path::Path::new(&relative_path));
    if !full.exists() {
        return Err(NiTriTeError::System(format!("Fichier non trouvé: {}", full.display())));
    }
    std::process::Command::new(&full)
        .spawn()
        .map_err(|e| NiTriTeError::System(format!("Lancement impossible: {}", e)))?;
    Ok(())
}

/// Génère un rapport batterie via powercfg et retourne le chemin HTML
#[tauri::command]
async fn run_battery_report() -> Result<String, NiTriTeError> {
    let output_path = std::env::temp_dir().join("nitrite-battery-report.html");
    let output_str = output_path.to_string_lossy().to_string();
    let out = output_str.clone();
    tokio::task::spawn_blocking(move || {
        let _ = std::process::Command::new("powercfg")
            .args(["/batteryreport", "/output", &out])
            .creation_flags(0x08000000)
            .status();
        Ok::<(), NiTriTeError>(())
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))??;
    Ok(output_str)
}

// === Exports (dev + prod) ===

fn nitrite_export_dir() -> Result<std::path::PathBuf, NiTriTeError> {
    let docs = dirs::document_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Documents")))
        .ok_or_else(|| NiTriTeError::System("Dossier Documents introuvable".into()))?;
    let dir = docs.join("NiTriTe").join("exports");
    std::fs::create_dir_all(&dir)
        .map_err(|e| NiTriTeError::System(format!("mkdir exports: {}", e)))?;
    Ok(dir)
}

#[tauri::command]
async fn get_export_dir() -> Result<String, NiTriTeError> {
    let dir = tokio::task::spawn_blocking(nitrite_export_dir)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))??;
    Ok(dir.to_string_lossy().to_string())
}

#[tauri::command]
async fn save_export_file(filename: String, content: String) -> Result<String, NiTriTeError> {
    let file_path = tokio::task::spawn_blocking(move || -> Result<std::path::PathBuf, NiTriTeError> {
        let dir = nitrite_export_dir()?;
        let path = dir.join(&filename);
        std::fs::write(&path, content.as_bytes())
            .map_err(|e| NiTriTeError::System(format!("Écriture {}: {}", filename, e)))?;
        Ok(path)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))??;
    Ok(file_path.to_string_lossy().to_string())
}

// === Config ===

#[tauri::command]
async fn get_config(state: tauri::State<'_, AppState>) -> Result<AppConfig, NiTriTeError> {
    let config = state.config.lock().await;
    Ok(config.clone())
}

#[tauri::command]
async fn save_config(
    config: AppConfig,
    state: tauri::State<'_, AppState>,
) -> Result<(), NiTriTeError> {
    config.save().map_err(|e| NiTriTeError::System(e.to_string()))?;
    let mut current = state.config.lock().await;
    *current = config;
    Ok(())
}

// === Open URL/Path ===

#[tauri::command]
async fn open_url(url: String) -> Result<(), NiTriTeError> {
    open::that(&url).map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn open_path(path: String) -> Result<(), NiTriTeError> {
    open::that(&path).map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Execute tool command (cmd or ms-settings) ===

#[tauri::command]
async fn execute_tool(command: String, is_url: bool) -> Result<(), NiTriTeError> {
    if is_url || command.starts_with("ms-settings:") || command.starts_with("http") {
        open::that(&command).map_err(|e| NiTriTeError::System(e.to_string()))
    } else {
        tokio::task::spawn_blocking(move || {
            std::process::Command::new("cmd")
                .args(["/C", &command])
                .spawn()
                .map_err(|e| NiTriTeError::System(e.to_string()))?;
            Ok(())
        })
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
    }
}

// === Extended Info (BIOS, Batterie, Dossiers) ===

#[tauri::command]
async fn get_bios_info() -> Result<system::extended_info::BiosInfo, NiTriTeError> {
    tokio::task::spawn_blocking(system::extended_info::get_bios_info)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn get_battery_extended() -> Result<Option<system::extended_info::BatteryInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::extended_info::get_battery_extended)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn get_folder_sizes() -> Result<Vec<system::extended_info::FolderSizeEntry>, NiTriTeError> {
    tokio::task::spawn_blocking(system::extended_info::get_folder_sizes)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn run_total_scan(window: tauri::Window) -> Result<system::total_scan::ScanResult, NiTriTeError> {
    system::total_scan::run_total_scan(window).await
}

// === Nouveaux modules Diagnostic ===

#[tauri::command]
async fn get_running_processes() -> Result<Vec<system::processes::ProcessInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::processes::collect_processes)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_windows_services() -> Result<Vec<system::services::ServiceInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::services::collect_services)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn get_security_status() -> Result<system::security::SecurityStatus, NiTriTeError> {
    tokio::task::spawn_blocking(system::security::collect_security_status)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_scheduled_tasks() -> Result<Vec<system::tasks::ScheduledTask>, NiTriTeError> {
    tokio::task::spawn_blocking(system::tasks::collect_scheduled_tasks)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_active_connections() -> Result<Vec<system::connections::TcpConnection>, NiTriTeError> {
    tokio::task::spawn_blocking(system::connections::collect_connections)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_wifi_status() -> Result<Option<system::connections::WifiInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::connections::collect_wifi_info)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_logical_volumes() -> Result<Vec<system::extra::VolumeInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::extra::collect_volumes)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn get_cpu_extended() -> Result<system::extra::CpuExtended, NiTriTeError> {
    tokio::task::spawn_blocking(system::extra::collect_cpu_extended)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn get_os_extended() -> Result<system::extra::OsExtended, NiTriTeError> {
    tokio::task::spawn_blocking(system::extra::collect_os_extended)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn get_folder_sizes_detailed() -> Result<Vec<system::extra::FolderSizeInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::extra::collect_folder_sizes)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_startup_programs_detailed() -> Result<Vec<system::extra::StartupProgram>, NiTriTeError> {
    tokio::task::spawn_blocking(system::extra::collect_startup_programs)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_smart_info() -> Vec<system::extra::SmartDiskInfo> {
    tokio::task::spawn_blocking(system::extra::collect_smart_info)
        .await
        .unwrap_or_default()
}

// === Debloat ===

#[tauri::command]
async fn debloat_disable_telemetry() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::disable_telemetry)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_disable_cortana() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::disable_cortana)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_disable_xbox() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::disable_xbox_services)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_disable_superfetch() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::disable_superfetch)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_disable_tips() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::disable_windows_tips)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_optimize_power() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::optimize_power_plan)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_disable_visual_effects() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::disable_visual_effects)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_clear_event_logs() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::clear_event_logs)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_clear_wu_cache() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::clear_windowsupdate_cache)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_flush_dns() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::flush_dns)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_reset_network() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::reset_network_stack)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_enable_trim() -> Result<maintenance::debloat::DebloatResult, NiTriTeError> {
    tokio::task::spawn_blocking(maintenance::debloat::enable_trim)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn debloat_remove_bloatware(apps: Vec<String>) -> Result<Vec<maintenance::debloat::DebloatResult>, NiTriTeError> {
    tokio::task::spawn_blocking(move || maintenance::debloat::remove_bloatware_uwp(apps))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Chocolatey ===

#[tauri::command]
async fn check_chocolatey() -> bool {
    tokio::task::spawn_blocking(installer::chocolatey::check_chocolatey)
        .await
        .unwrap_or(false)
}

#[tauri::command]
async fn list_chocolatey_upgrades() -> Result<Vec<installer::chocolatey::ChocoPackage>, NiTriTeError> {
    tokio::task::spawn_blocking(installer::chocolatey::list_chocolatey_upgrades)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn upgrade_chocolatey_all() -> Result<installer::chocolatey::ChocoUpgradeResult, NiTriTeError> {
    tokio::task::spawn_blocking(installer::chocolatey::upgrade_chocolatey_all)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Windows Updates ===

#[derive(serde::Serialize, Clone)]
pub struct WinUpdate {
    pub hotfix_id: String,
    pub description: String,
    pub installed_on: String,
}

#[tauri::command]
async fn check_windows_updates() -> Result<Vec<WinUpdate>, NiTriTeError> {
    tokio::task::spawn_blocking(|| {
        let output = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 30 HotFixID,Description,InstalledOn | ConvertTo-Json -Compress",
            ])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| NiTriTeError::System(e.to_string()))?;

        let text = String::from_utf8_lossy(&output.stdout).to_string();
        let json: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap_or_default();

        Ok(json
            .iter()
            .map(|v| WinUpdate {
                hotfix_id: v["HotFixID"].as_str().unwrap_or("").to_string(),
                description: v["Description"].as_str().unwrap_or("").to_string(),
                installed_on: v["InstalledOn"]
                    .as_str()
                    .unwrap_or("")
                    .split('T')
                    .next()
                    .unwrap_or("")
                    .to_string(),
            })
            .collect())
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Windows Updates Pending ===

#[derive(serde::Serialize, Clone)]
pub struct PendingUpdate {
    pub title: String,
    pub kb_ids: String,
    pub severity: String,
    pub size_mb: f64,
    pub is_downloaded: bool,
}

#[tauri::command]
async fn scan_pending_windows_updates() -> Vec<PendingUpdate> {
    tokio::task::spawn_blocking(|| {
        let ps = r#"
try {
    $session = New-Object -ComObject Microsoft.Update.Session -ErrorAction Stop
    $searcher = $session.CreateUpdateSearcher()
    $searcher.Online = $true
    $res = $searcher.Search("IsInstalled=0 and Type='Software'")
    $out = @()
    for ($i = 0; $i -lt $res.Updates.Count; $i++) {
        $u = $res.Updates.Item($i)
        $kbs = @(); for ($k=0;$k -lt $u.KBArticleIDs.Count;$k++) { $kbs += "KB$($u.KBArticleIDs.Item($k))" }
        $out += [PSCustomObject]@{
            title    = [string]$u.Title
            kb_ids   = $kbs -join ","
            severity = if ($u.MsrcSeverity) { [string]$u.MsrcSeverity } else { "Normal" }
            size_mb  = [math]::Round($u.MaxDownloadSize / 1MB, 1)
            dl       = [bool]$u.IsDownloaded
        }
    }
    $out | ConvertTo-Json -Compress -Depth 2
} catch { Write-Output '[]' }
"#;
        use std::io::Read;
        let mut child = match std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps])
            .creation_flags(0x08000000)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn() {
                Ok(c) => c,
                Err(_) => return vec![],
            };
        let timeout = std::time::Duration::from_secs(60);
        let start = std::time::Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_)) => {
                    let mut buf = Vec::new();
                    if let Some(mut out) = child.stdout.take() { let _ = out.read_to_end(&mut buf); }
                    let text = String::from_utf8_lossy(&buf);
                    let t = text.trim();
                    if t.is_empty() || t == "[]" { return vec![]; }
                    let json_text = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
                    return serde_json::from_str::<Vec<serde_json::Value>>(&json_text)
                        .unwrap_or_default()
                        .iter()
                        .map(|v| PendingUpdate {
                            title: v["title"].as_str().unwrap_or("").to_string(),
                            kb_ids: v["kb_ids"].as_str().unwrap_or("").to_string(),
                            severity: v["severity"].as_str().unwrap_or("Normal").to_string(),
                            size_mb: v["size_mb"].as_f64().unwrap_or(0.0),
                            is_downloaded: v["dl"].as_bool().unwrap_or(false),
                        })
                        .collect();
                }
                Ok(None) => {
                    if start.elapsed() > timeout { let _ = child.kill(); let _ = child.wait(); return vec![]; }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
                Err(_) => { let _ = child.kill(); return vec![]; }
            }
        }
    }).await.unwrap_or_default()
}

#[tauri::command]
async fn trigger_windows_update() -> String {
    tokio::task::spawn_blocking(|| {
        // Déclenche le scan et l'install via UsoClient (Windows 10/11)
        let r1 = std::process::Command::new("UsoClient.exe")
            .arg("StartInteractiveScan")
            .creation_flags(0x08000000)
            .output();
        if r1.is_ok() { return "Scan Windows Update déclenché".to_string(); }
        // Fallback: ouvre les paramètres Windows Update
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start ms-settings:windowsupdate"])
            .creation_flags(0x08000000)
            .spawn();
        "Paramètres Windows Update ouverts".to_string()
    }).await.unwrap_or_else(|_| "Erreur".to_string())
}

// === MAS Activation ===

#[tauri::command]
async fn open_mas_window() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command",
                "Start-Process powershell -ArgumentList '-NoExit','-Command','irm https://get.activated.win | iex' -Verb RunAs"])
            .creation_flags(0x08000000)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    Err("Non supporté".to_string())
}

// === Network Extended ===

#[tauri::command]
async fn get_network_extended() -> serde_json::Value {
    tokio::task::spawn_blocking(|| {
        let ps = r#"
$result = @{}
function Do-Ping { param($h)
    try {
        $r = Test-Connection $h -Count 2 -ErrorAction SilentlyContinue
        if ($r) { $times = @($r | Select-Object -ExpandProperty ResponseTime); @{success=$true;avg=[math]::Round(($times|Measure-Object -Average).Average,1);host=$h} }
        else { @{success=$false;avg=0;host=$h} }
    } catch { @{success=$false;avg=0;host=$h} }
}
$gw = (Get-NetRoute -DestinationPrefix '0.0.0.0/0' -ErrorAction SilentlyContinue | Sort-Object RouteMetric | Select-Object -First 1).NextHop
if ($gw) { $result.ping_gateway = Do-Ping $gw } else { $result.ping_gateway = $null }
$result.ping_google = Do-Ping '8.8.8.8'
$result.ping_cloudflare = Do-Ping '1.1.1.1'
try { $result.public_ip = (Invoke-RestMethod -Uri 'https://api.ipify.org' -TimeoutSec 6 -ErrorAction SilentlyContinue) } catch { $result.public_ip = "" }
try {
    $entries = @()
    arp -a 2>$null | ForEach-Object { if ($_ -match '^\s+(\d+\.\d+\.\d+\.\d+)\s+([\w-]+)\s+(\w+)') { $entries += @{ip=$matches[1];mac=$matches[2];type=$matches[3]} } }
    $result.arp_table = $entries
} catch { $result.arp_table = @() }
try {
    $result.routes = @(Get-NetRoute -AddressFamily IPv4 -ErrorAction SilentlyContinue |
        Where-Object { $_.NextHop -ne '0.0.0.0' } | Sort-Object RouteMetric | Select-Object -First 40 |
        ForEach-Object { @{prefix=$_.DestinationPrefix;next_hop=$_.NextHop;metric=[int]$_.RouteMetric;iface=$_.InterfaceAlias} })
} catch { $result.routes = @() }
try {
    $prx = Get-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -ErrorAction SilentlyContinue
    $result.proxy = @{enabled=[bool]$prx.ProxyEnable;server=[string]$prx.ProxyServer;bypass=[string]$prx.ProxyOverride}
} catch { $result.proxy = @{enabled=$false;server="";bypass=""} }
try {
    $fw = Get-NetFirewallProfile -ErrorAction SilentlyContinue
    $result.firewall = @{
        domain=[bool]($fw | Where-Object Name -eq 'Domain' | Select-Object -ExpandProperty Enabled -ErrorAction SilentlyContinue)
        private=[bool]($fw | Where-Object Name -eq 'Private' | Select-Object -ExpandProperty Enabled -ErrorAction SilentlyContinue)
        public=[bool]($fw | Where-Object Name -eq 'Public' | Select-Object -ExpandProperty Enabled -ErrorAction SilentlyContinue)
    }
} catch { $result.firewall = @{domain=$false;private=$false;public=$false} }
try {
    $result.shares = @(Get-SmbShare -ErrorAction SilentlyContinue |
        ForEach-Object { @{name=$_.Name;path=[string]$_.Path;desc=[string]$_.Description} })
} catch { $result.shares = @() }
try {
    $result.stats = @(Get-NetAdapterStatistics -ErrorAction SilentlyContinue |
        ForEach-Object { @{name=$_.Name;recv_bytes=[long]$_.ReceivedBytes;sent_bytes=[long]$_.SentBytes} })
} catch { $result.stats = @() }
try {
    $he = @()
    Get-Content 'C:\Windows\System32\drivers\etc\hosts' -ErrorAction SilentlyContinue | ForEach-Object {
        if ($_ -notmatch '^\s*#' -and $_.Trim() -ne '') {
            $p = $_.Trim() -split '\s+'; if ($p.Count -ge 2) { $he += @{ip=$p[0];host=$p[1]} }
        }
    }
    $result.hosts_entries = $he
} catch { $result.hosts_entries = @() }
try {
    $result.dns_test = @(Resolve-DnsName 'google.com' -ErrorAction SilentlyContinue | Select-Object -First 5 |
        ForEach-Object { @{name=[string]$_.Name;ip=if($_.IPAddress){[string]$_.IPAddress}else{""};type=[string]$_.Type} })
} catch { $result.dns_test = @() }
try {
    $wf = @(); netsh wlan show networks 2>$null | ForEach-Object { if ($_ -match 'SSID\s+\d+\s*:\s*(.+)') { $wf += $matches[1].Trim() } }
    $result.wifi_networks = $wf
} catch { $result.wifi_networks = @() }
$result | ConvertTo-Json -Depth 4 -Compress
"#;
        use std::io::Read;
        let mut child = match std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps])
            .creation_flags(0x08000000)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn() {
                Ok(c) => c,
                Err(_) => return serde_json::Value::Object(serde_json::Map::new()),
            };
        let timeout = std::time::Duration::from_secs(35);
        let start = std::time::Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_)) => {
                    let mut buf = Vec::new();
                    if let Some(mut out) = child.stdout.take() { let _ = out.read_to_end(&mut buf); }
                    return serde_json::from_str(String::from_utf8_lossy(&buf).trim())
                        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
                }
                Ok(None) => {
                    if start.elapsed() > timeout { let _ = child.kill(); let _ = child.wait(); break; }
                    std::thread::sleep(std::time::Duration::from_millis(300));
                }
                Err(_) => { let _ = child.kill(); break; }
            }
        }
        serde_json::Value::Object(serde_json::Map::new())
    }).await.unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
}

// === Scoop ===

#[tauri::command]
async fn check_scoop() -> bool {
    tokio::task::spawn_blocking(|| {
        std::process::Command::new("scoop")
            .arg("--version")
            .creation_flags(0x08000000)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }).await.unwrap_or(false)
}

#[derive(serde::Serialize, Clone)]
pub struct ScoopPackage { pub name: String, pub installed: String, pub available: String }

#[tauri::command]
async fn list_scoop_upgrades() -> Vec<ScoopPackage> {
    tokio::task::spawn_blocking(|| {
        let out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                "scoop status 2>$null | Select-Object -Skip 2 | ConvertFrom-String -PropertyNames Name,Installed,Available | Select-Object Name,Installed,Available | ConvertTo-Json -Compress"])
            .creation_flags(0x08000000)
            .output();
        if let Ok(o) = out {
            let text = String::from_utf8_lossy(&o.stdout);
            let t = text.trim();
            if !t.is_empty() && t != "[]" {
                let json_text = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
                if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&json_text) {
                    return arr.iter().filter_map(|v| {
                        let name = v["Name"].as_str()?.to_string();
                        if name.is_empty() { return None; }
                        Some(ScoopPackage {
                            name,
                            installed: v["Installed"].as_str().unwrap_or("").to_string(),
                            available: v["Available"].as_str().unwrap_or("").to_string(),
                        })
                    }).collect();
                }
            }
        }
        vec![]
    }).await.unwrap_or_default()
}

#[tauri::command]
async fn upgrade_scoop_all(window: tauri::Window) -> Result<(), NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        let out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", "scoop update *; scoop cleanup *"])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| NiTriTeError::System(e.to_string()))?;
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        let _ = window.emit("scoop-upgrade-done", &text);
        Ok(())
    }).await.map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Setup Tauri ===

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    utils::logger::init_logger();
    tracing::info!("Demarrage NiTriTe V26.0");

    let config = AppConfig::load();
    let app_state = AppState::new(config);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Systeme
            get_system_info,
            get_platform_info,
            // Monitoring
            start_monitoring,
            stop_monitoring,
            // Reseau
            get_network_overview,
            get_connections,
            ping_host,
            // Installer
            get_apps,
            get_tools,
            install_app,
            check_winget,
            list_upgradable,
            upgrade_all,
            // Maintenance
            empty_recycle_bin,
            clean_temp_files,
            run_disk_cleanup,
            get_startup_programs,
            run_system_command,
            disable_startup_program,
            // Terminal
            detect_shells,
            run_in_shell,
            // Browser Cleanup
            get_browser_cache_sizes,
            clean_browser_cache,
            // Backup
            create_backup,
            list_backups,
            // AI
            ai_check,
            ai_list_models,
            ai_query,
            ai_execute_command,
            // Scripts
            get_builtin_scripts,
            execute_script,
            // Logs
            get_app_logs,
            // Portables
            get_portable_apps,
            check_portable_installed,
            launch_portable,
            open_portables_dir,
            launch_exe,
            run_battery_report,
            // Config
            get_config,
            save_config,
            // Event Logs
            get_event_logs,
            // Drivers
            get_recommended_drivers,
            // Scripts files
            list_script_files,
            read_script_file,
            save_script_file,
            // Reports
            list_reports,
            // Utils
            open_url,
            open_path,
            execute_tool,
            get_export_dir,
            save_export_file,
            // Extended Info
            get_bios_info,
            get_battery_extended,
            get_folder_sizes,
            run_total_scan,
            // Debloat
            debloat_disable_telemetry,
            debloat_disable_cortana,
            debloat_disable_xbox,
            debloat_disable_superfetch,
            debloat_disable_tips,
            debloat_optimize_power,
            debloat_disable_visual_effects,
            debloat_clear_event_logs,
            debloat_clear_wu_cache,
            debloat_flush_dns,
            debloat_reset_network,
            debloat_enable_trim,
            debloat_remove_bloatware,
            maintenance::debloat::debloat_run_extra,
            // Chocolatey
            check_chocolatey,
            list_chocolatey_upgrades,
            upgrade_chocolatey_all,
            // Windows Updates
            check_windows_updates,
            installer::windows_update::search_pending_updates,
            installer::windows_update::install_windows_updates,
            scan_pending_windows_updates,
            trigger_windows_update,
            open_mas_window,
            get_network_extended,
            check_scoop,
            list_scoop_upgrades,
            upgrade_scoop_all,
            // Detailed Diagnostics
            system::system_detailed::get_motherboard_detailed,
            system::system_detailed::get_gpu_detailed,
            system::system_detailed::get_ram_detailed,
            system::system_detailed::get_audio_devices,
            system::system_detailed::get_usb_devices,
            system::system_detailed::get_battery_detailed,
            system::system_detailed::get_monitor_info,
            system::system_detailed::get_power_plans,
            system::system_detailed::get_printers,
            system::system_detailed::get_environment_variables,
            system::system_detailed::get_windows_license,
            system::system_detailed::get_installed_updates,
            system::system_detailed_extra::get_storage_physical_info,
            system::system_detailed_extra::get_network_adapters_detailed,
            system::system_detailed_extra::get_cpu_cache_info,
            system::system_detailed_extra::get_installed_software,
            // Nouveaux diagnostics
            get_running_processes,
            get_windows_services,
            get_security_status,
            get_scheduled_tasks,
            get_active_connections,
            get_wifi_status,
            get_logical_volumes,
            get_cpu_extended,
            get_os_extended,
            get_folder_sizes_detailed,
            get_startup_programs_detailed,
            get_smart_info,
            // Nouveaux onglets diagnostics
            system::accounts::get_user_accounts,
            system::firewall_rules::get_firewall_rules,
            system::shares::get_network_shares,
            system::registry_persist::get_registry_persistence,
            system::sys_history::get_system_history,
            // Nouveaux modules 10x
            system::sys_drivers::get_sys_drivers_list,
            system::certificates::get_certificates,
            system::perf_snapshot::get_perf_snapshot,
            system::net_tools::run_ping,
            system::net_tools::run_traceroute,
            system::net_tools::run_nslookup,
            system::net_tools::get_ip_config,
            system::net_tools::get_arp_table,
            system::net_tools::get_route_table,
            system::net_tools::scan_ports,
            system::net_tools::get_wifi_networks,
            system::net_tools::get_local_open_ports,
            system::net_tools::check_http,
            system::net_tools::get_net_shares,
            system::net_tools::test_bandwidth,
            system::repair::check_system_health,
            system::repair::run_repair_command,
            system::driver_updater::get_hardware_devices,
            system::driver_updater::scan_driver_folder,
            system::driver_updater::install_driver,
            system::driver_updater::check_driver_updates_winupdate,
            system::driver_updater::get_all_hardware_ids,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de NiTriTe");
}
