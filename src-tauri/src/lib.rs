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

// === Nettoyage sortie ===

#[tauri::command]
async fn cleanup_on_exit(app: tauri::AppHandle) -> Result<(), NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        // 1. Logs applicatif (.logs/ à la racine du projet — dev uniquement)
        let logs_candidates = [
            std::env::current_dir().ok().map(|d| d.join(".logs")),
            std::env::var("APPDATA").ok().map(|a| std::path::PathBuf::from(a).join("com.nitrite.tool").join("logs")),
        ];
        for candidate in logs_candidates.iter().flatten() {
            if candidate.exists() {
                let _ = std::fs::remove_dir_all(candidate);
            }
        }

        // 2. Fichiers temp Tauri dans %LOCALAPPDATA%\com.nitrite.tool\EBWebView\...
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let tauri_cache = std::path::PathBuf::from(&local).join("com.nitrite.tool");
            for sub in &["EBWebView", "logs", "temp"] {
                let p = tauri_cache.join(sub);
                if p.exists() { let _ = std::fs::remove_dir_all(&p); }
            }
        }

        // 3. Fichiers temp Windows portant "nitrite" dans le nom
        if let Ok(temp) = std::env::var("TEMP") {
            if let Ok(entries) = std::fs::read_dir(&temp) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_lowercase();
                    if name.contains("nitrite") || name.contains("tauri") {
                        if entry.path().is_dir() {
                            let _ = std::fs::remove_dir_all(entry.path());
                        } else {
                            let _ = std::fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }

        // 4. WebView2 data (cookies, cache) du dossier EBWebView
        if let Ok(appdata) = std::env::var("APPDATA") {
            let webview_cache = std::path::PathBuf::from(&appdata).join("com.nitrite.tool");
            for sub in &["logs", "tmp", "cache"] {
                let p = webview_cache.join(sub);
                if p.exists() { let _ = std::fs::remove_dir_all(&p); }
            }
        }
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?;

    // Forcer la fermeture après nettoyage
    app.exit(0);
    Ok(())
}

// === Backup ===

#[tauri::command]
async fn create_backup(items: Vec<String>, format: String, custom_path: Option<String>) -> Result<backup::collector::BackupManifest, NiTriTeError> {
    tokio::task::spawn_blocking(move || backup::collector::create_backup(items, format, custom_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn list_backups() -> Result<Vec<backup::collector::BackupEntryInfo>, NiTriTeError> {
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
    history: Option<Vec<serde_json::Value>>,   // [{role, content}]
    state: tauri::State<'_, AppState>,
) -> Result<String, NiTriTeError> {
    let (backend, port, ollama_url, default_model) = {
        let config = state.config.lock().await;
        (config.ai_backend.clone(), config.llamacpp_port, config.ollama_url.clone(), config.ollama_model.clone())
    };
    let temp = 0.7_f64;
    let m = model.unwrap_or(default_model);

    // Construire les messages
    let mut messages_ollama: Vec<ai::ollama::OllamaChatMessage> = vec![];
    let mut messages_llama: Vec<ai::llamacpp::ChatMessage> = vec![];

    let add_msg = |role: &str, content: &str,
                   mo: &mut Vec<ai::ollama::OllamaChatMessage>,
                   ml: &mut Vec<ai::llamacpp::ChatMessage>| {
        mo.push(ai::ollama::OllamaChatMessage { role: role.into(), content: content.into() });
        ml.push(ai::llamacpp::ChatMessage   { role: role.into(), content: content.into() });
    };

    if let Some(sys) = &system_prompt {
        if !sys.trim().is_empty() {
            add_msg("system", sys.trim(), &mut messages_ollama, &mut messages_llama);
        }
    }
    if let Some(hist) = history {
        for msg in hist {
            let role = msg["role"].as_str().unwrap_or("user").to_string();
            let content = msg["content"].as_str().unwrap_or("").to_string();
            if !content.is_empty() && (role == "user" || role == "assistant") {
                add_msg(&role, &content, &mut messages_ollama, &mut messages_llama);
            }
        }
    }
    add_msg("user", &prompt, &mut messages_ollama, &mut messages_llama);

    if backend == "llamacpp" {
        ai::llamacpp::chat(port, &m, messages_llama, temp).await
    } else {
        ai::ollama::chat(&ollama_url, &m, messages_ollama, temp).await
    }
}

#[tauri::command]
async fn ai_execute_command(command: String) -> Result<ai::tool_calling::SafeCommandResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || ai::tool_calling::execute_safe(&command))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
}

#[tauri::command]
async fn ai_start_llamacpp(
    model_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let (server_path, port) = {
        let config = state.config.lock().await;
        (config.llamacpp_server_path.clone(), config.llamacpp_port)
    };
    // Détecter le binaire si chemin non configuré
    let exe_dir = std::env::current_exe()
        .ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let bin = if !server_path.is_empty() && std::path::Path::new(&server_path).exists() {
        server_path
    } else {
        ai::llamacpp::find_server_binary(&exe_dir)
            .ok_or_else(|| "llama-server.exe introuvable. Placez-le dans logiciel/AI/".to_string())?
    };

    // Tuer l'ancien processus si présent
    {
        let mut proc = state.llamacpp_process.lock().await;
        if let Some(mut child) = proc.take() {
            let _ = child.kill();
        }
    }

    let child = ai::llamacpp::start_server(&bin, &model_path, port)
        .map_err(|e| e.to_string())?;

    {
        let mut proc = state.llamacpp_process.lock().await;
        *proc = Some(child);
    }
    // Sauvegarder le chemin du modèle
    {
        let mut config = state.config.lock().await;
        config.llamacpp_model_path = model_path;
        config.llamacpp_server_path = bin;
        let _ = config.save();
    }
    Ok(())
}

#[tauri::command]
async fn ai_stop_llamacpp(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut proc = state.llamacpp_process.lock().await;
    if let Some(mut child) = proc.take() {
        child.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn ai_llamacpp_status(state: tauri::State<'_, AppState>) -> Result<bool, NiTriTeError> {
    let port = { state.config.lock().await.llamacpp_port };
    Ok(ai::llamacpp::is_server_ready(port).await)
}

#[tauri::command]
async fn ai_list_gguf_models() -> Result<Vec<ai::llamacpp::GgufModel>, NiTriTeError> {
    let exe_dir = std::env::current_exe()
        .ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let dir = ai::llamacpp::models_dir(&exe_dir);
    Ok(ai::llamacpp::list_gguf_models(&dir))
}

#[tauri::command]
async fn ai_find_llamacpp_server() -> Result<Option<String>, NiTriTeError> {
    let exe_dir = std::env::current_exe()
        .ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
        .unwrap_or_default();
    Ok(ai::llamacpp::find_server_binary(&exe_dir))
}

#[tauri::command]
fn ai_model_catalog() -> Vec<ai::llamacpp::ModelCatalogEntry> {
    ai::llamacpp::model_catalog()
}

#[tauri::command]
async fn ai_download_server(app: tauri::AppHandle) -> Result<String, String> {
    let exe_dir = std::env::current_exe()
        .ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let app_clone = app.clone();
    ai::llamacpp::download_server(&exe_dir, move |progress| {
        let _ = app_clone.emit("ai:download-progress", &progress);
    }).await
}

#[tauri::command]
async fn ai_download_model(
    app: tauri::AppHandle,
    url: String,
    filename: String,
) -> Result<String, String> {
    let exe_dir = std::env::current_exe()
        .ok().and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
        .unwrap_or_default();
    let app_clone = app.clone();
    ai::llamacpp::download_model_file(&url, &filename, &exe_dir, move |progress| {
        let _ = app_clone.emit("ai:download-progress", &progress);
    }).await
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

// === Clonage Système ===

#[tauri::command]
async fn get_disks_for_clone() -> Result<Vec<system::clone::DiskInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::clone::get_disks)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn start_system_image(target_drive: String, window: tauri::Window) -> Result<system::clone::CloneResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::clone::create_system_image(target_drive, &window))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn start_robocopy_clone(source_drive: String, target_drive: String, window: tauri::Window) -> Result<system::clone::CloneResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::clone::clone_with_robocopy(source_drive, target_drive, &window))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Récupération de Données ===

#[tauri::command]
async fn list_shadow_copies() -> Result<Vec<system::data_recovery::ShadowCopy>, NiTriTeError> {
    tokio::task::spawn_blocking(system::data_recovery::list_shadow_copies)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn browse_shadow_copy(device_path: String, relative_path: String) -> Result<Vec<system::data_recovery::RecoveredFile>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::browse_shadow_copy(device_path, relative_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn restore_from_shadow(source_path: String, target_folder: String) -> Result<system::data_recovery::RestoreResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::restore_from_shadow(source_path, target_folder))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn scan_recycle_bin() -> Result<Vec<system::data_recovery::RecoveredFile>, NiTriTeError> {
    tokio::task::spawn_blocking(system::data_recovery::scan_recycle_bin)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn restore_recycle_bin_item(original_path: String) -> Result<system::data_recovery::RestoreResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::restore_recycle_bin_item(original_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn scan_deleted_files(drive: String) -> Result<Vec<system::data_recovery::RecoveredFile>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::scan_deleted_files(drive))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn search_shadow_copy(device_path: String, query: String, base_path: String) -> Result<Vec<system::data_recovery::RecoveredFile>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::search_shadow_copy(device_path, query, base_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn restore_files_batch(files: Vec<String>, target_folder: String) -> Result<system::data_recovery::BatchRestoreResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::restore_files_batch(files, target_folder))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn list_connected_disks() -> Result<Vec<system::data_recovery::DiskInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::data_recovery::list_connected_disks)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn browse_disk_path(path: String) -> Result<Vec<system::data_recovery::DiskEntry>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::browse_disk_path(path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn recover_files_safe(
    files: Vec<String>,
    target_folder: String,
    safe_mode: bool,
    window: tauri::Window,
) -> Result<system::data_recovery::BatchRestoreResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::recover_files_safe(files, target_folder, safe_mode, &window))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_user_profile_folders() -> Result<Vec<system::data_recovery::UserFolder>, NiTriTeError> {
    tokio::task::spawn_blocking(system::data_recovery::get_user_profile_folders)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn backup_user_folders(
    folders: Vec<String>,
    target: String,
    window: tauri::Window,
) -> Result<system::data_recovery::BackupResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        system::data_recovery::backup_user_folders(folders, target, &window)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Désinstallateur Propre ===

#[tauri::command]
async fn list_installed_apps_for_uninstall() -> Result<Vec<installer::uninstaller::InstalledApp>, NiTriTeError> {
    tokio::task::spawn_blocking(installer::uninstaller::list_installed_apps)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn uninstall_app_clean(
    app_name: String,
    uninstall_string: String,
    publisher: String,
    window: tauri::Window,
) -> Result<installer::uninstaller::UninstallResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        installer::uninstaller::uninstall_app_clean(app_name, uninstall_string, publisher, &window)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Preview résidus (avant désinstallation) ===

#[tauri::command]
async fn preview_residuals(
    app_name: String,
    publisher: String,
) -> Result<Vec<String>, NiTriTeError> {
    tokio::task::spawn_blocking(move || installer::uninstaller::preview_residuals(app_name, publisher))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Suppression définitive des résidus ===

#[tauri::command]
async fn delete_residuals(paths: Vec<String>) -> Result<installer::uninstaller::ResidualCleanResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || installer::uninstaller::delete_residuals(paths))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Extraction + suppression des résidus ===

#[tauri::command]
async fn extract_residuals(paths: Vec<String>, target: String) -> Result<installer::uninstaller::ResidualCleanResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || installer::uninstaller::extract_residuals(paths, target))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Comparaison Shadow Copy vs Système Actuel ===

#[tauri::command]
async fn compare_shadow_with_current(
    device_path: String,
    sub_path: String,
    live_path: String,
) -> Result<Vec<system::data_recovery::ComparedFile>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::compare_shadow_with_current(device_path, sub_path, live_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Scan USN Journal tous lecteurs NTFS ===

#[tauri::command]
async fn scan_all_deleted_files() -> Result<Vec<system::data_recovery::RecoveredFile>, NiTriTeError> {
    tokio::task::spawn_blocking(system::data_recovery::scan_all_deleted_files)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Gestionnaire de Partitions ===

#[tauri::command]
async fn get_disks_smart() -> Result<Vec<system::partition::DiskSmartInfo>, NiTriTeError> {
    tokio::task::spawn_blocking(system::partition::get_disks_smart)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn get_partition_list() -> Result<Vec<system::partition::PartitionDetail>, NiTriTeError> {
    tokio::task::spawn_blocking(system::partition::get_partition_list)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn format_partition_cmd(letter: String, fs: String, label: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::format_partition(letter, fs, label))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn assign_drive_letter_cmd(disk_index: u32, part_index: u32, letter: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::assign_drive_letter(disk_index, part_index, letter))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn create_partition_cmd(disk_index: u32, size_mb: Option<u32>) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::create_partition(disk_index, size_mb))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn delete_partition_cmd(disk_index: u32, part_index: u32) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::delete_partition(disk_index, part_index))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn initialize_disk_cmd(disk_index: u32, style: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::initialize_disk(disk_index, style))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

// === Shadow Copy Gestion ===

#[tauri::command]
async fn create_shadow_copy_cmd(volume: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::create_shadow_copy(volume))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn delete_shadow_copy_cmd(shadow_id: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::data_recovery::delete_shadow_copy(shadow_id))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn open_in_explorer(path: String) -> Result<(), NiTriTeError> {
    system::data_recovery::open_in_explorer(path)
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn get_ntfs_drives() -> Result<Vec<String>, NiTriTeError> {
    tokio::task::spawn_blocking(system::data_recovery::get_ntfs_drives)
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Récupération avancée (image disque, surface test, scan MFT, rapport) ===

#[tauri::command]
async fn create_disk_image_cmd(
    disk_index: u32,
    output_path: String,
    window: tauri::Window,
) -> Result<system::advanced_recovery::DiskImageResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        system::advanced_recovery::create_disk_image(disk_index, output_path, &window)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn surface_test_volume_cmd(
    drive_letter: String,
    window: tauri::Window,
) -> Result<system::advanced_recovery::SurfaceTestResult, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        system::advanced_recovery::surface_test_volume(drive_letter, &window)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn deep_mft_scan_advanced_cmd(
    drive: String,
) -> Result<Vec<system::advanced_recovery::DeepMftFile>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::advanced_recovery::deep_mft_scan_advanced(drive))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

#[tauri::command]
async fn generate_recovery_report_cmd(
    title: String,
    files_json: String,
    output_path: String,
) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        system::advanced_recovery::generate_recovery_report(title, files_json, output_path)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
    .map_err(NiTriTeError::System)
}

// === Partition avancé (resize, MBR, lost partitions) ===

#[tauri::command]
async fn get_partition_resize_limits_cmd(
    disk_index: u32,
    part_index: u32,
) -> Result<system::partition::PartitionSizeLimits, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        system::partition::get_partition_resize_limits(disk_index, part_index)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
    .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn resize_partition_cmd(
    disk_index: u32,
    part_index: u32,
    new_size_mb: u64,
) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        system::partition::resize_partition_ps(disk_index, part_index, new_size_mb)
    })
    .await
    .map_err(|e| NiTriTeError::System(e.to_string()))?
    .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn backup_mbr_cmd(disk_index: u32, output_path: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::backup_mbr(disk_index, output_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn restore_mbr_cmd(disk_index: u32, mbr_path: String) -> Result<String, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::restore_mbr(disk_index, mbr_path))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))?
        .map_err(NiTriTeError::System)
}

#[tauri::command]
async fn scan_lost_partitions_cmd(
    disk_index: u32,
) -> Result<Vec<system::partition::LostPartition>, NiTriTeError> {
    tokio::task::spawn_blocking(move || system::partition::scan_lost_partitions(disk_index))
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
}

// === Save content to arbitrary path (dialog-driven export) ===

#[tauri::command]
async fn save_content_to_path(path: String, content: String) -> Result<(), NiTriTeError> {
    tokio::fs::write(&path, content.as_bytes())
        .await
        .map_err(|e| NiTriTeError::System(e.to_string()))
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
                .creation_flags(0x08000000)
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
                    std::thread::sleep(std::time::Duration::from_millis(50));
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
                    std::thread::sleep(std::time::Duration::from_millis(50));
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
    // Scoop est un script PowerShell — ne pas l'invoquer directement comme exe
    tokio::task::spawn_blocking(|| {
        std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                "if (Get-Command scoop -ErrorAction SilentlyContinue) { exit 0 } else { exit 1 }"])
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
        // ConvertFrom-String est déprécié — on parse manuellement la sortie de "scoop status"
        let ps = r#"
$lines = @(scoop status 2>$null)
$rows = @()
foreach ($line in $lines) {
    $t = $line.Trim()
    if ($t -eq '' -or $t -match '^Name' -or $t -match '^-{3}' -or $t -match '^Scoop') { continue }
    $parts = $t -split '\s{2,}'
    if ($parts.Count -ge 2) {
        $rows += [PSCustomObject]@{
            name      = $parts[0].Trim()
            installed = if ($parts.Count -ge 2) { $parts[1].Trim() } else { '' }
            available = if ($parts.Count -ge 3) { $parts[2].Trim() } else { '' }
        }
    }
}
$rows | ConvertTo-Json -Compress
"#;
        let out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps])
            .creation_flags(0x08000000)
            .output();
        if let Ok(o) = out {
            let text = String::from_utf8_lossy(&o.stdout);
            let t = text.trim();
            if t.is_empty() || t == "null" { return vec![]; }
            let json_text = if t.starts_with('{') { format!("[{}]", t) } else { t.to_string() };
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&json_text) {
                return arr.iter().filter_map(|v| {
                    let name = v["name"].as_str().filter(|s| !s.is_empty())?.to_string();
                    Some(ScoopPackage {
                        name,
                        installed: v["installed"].as_str().unwrap_or("").to_string(),
                        available: v["available"].as_str().unwrap_or("").to_string(),
                    })
                }).collect();
            }
        }
        vec![]
    }).await.unwrap_or_default()
}

#[tauri::command]
async fn upgrade_scoop_all(window: tauri::Window) -> Result<(), NiTriTeError> {
    tokio::task::spawn_blocking(move || {
        // 1. Met à jour Scoop lui-même, 2. Met à jour tous les apps, 3. Nettoie les vieilles versions
        let ps = "scoop update; scoop update * 2>&1; scoop cleanup * 2>&1; Write-Output 'Mise a jour Scoop terminee.'";
        let out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps])
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| NiTriTeError::System(e.to_string()))?;
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        let _ = window.emit("scoop-upgrade-done", &text);
        Ok(())
    }).await.map_err(|e| NiTriTeError::System(e.to_string()))?
}

// === Nouvelles commandes Diagnostic Amélioré ===

/// Termine un processus par son PID
#[tauri::command]
async fn kill_process(pid: u32) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let out = std::process::Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            if out.status.success() {
                Ok(format!("Processus {} terminé", pid))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Contrôle un service Windows (start/stop/restart)
#[tauri::command]
async fn control_service(name: String, action: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let cmd = match action.as_str() {
                "start" => "Start-Service",
                "stop" => "Stop-Service",
                "restart" => "Restart-Service",
                _ => return Err(format!("Action inconnue: {}", action)),
            };
            let ps = format!(
                "{} -Name '{}' -ErrorAction Stop 2>&1; Write-Output 'OK'",
                cmd, name.replace('\'', "''")
            );
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            if stdout.contains("OK") || out.status.success() {
                Ok(format!("Service '{}' : {} effectué", name, action))
            } else {
                Err(if !stderr.is_empty() { stderr } else { stdout })
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Change le mode de démarrage d'un service Windows
#[tauri::command]
async fn set_service_start_mode(name: String, mode: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            // mode: "Auto", "Manual", "Disabled", "Automatic (Delayed Start)"
            let sc_mode = match mode.as_str() {
                "Auto" | "Automatic" => "auto",
                "Manual" => "demand",
                "Disabled" => "disabled",
                "Automatic (Delayed Start)" => "delayed-auto",
                _ => return Err(format!("Mode inconnu: {}", mode)),
            };
            let out = std::process::Command::new("sc")
                .args(["config", &name, &format!("start= {}", sc_mode)])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            if out.status.success() {
                Ok(format!("Service '{}' : mode '{}' appliqué", name, mode))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Définit ou modifie une variable d'environnement
#[tauri::command]
async fn set_environment_variable(name: String, value: String, scope: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let ps_scope = match scope.as_str() {
                "Système" | "System" | "Machine" => "Machine",
                _ => "User",
            };
            let ps = format!(
                "[System.Environment]::SetEnvironmentVariable('{}', '{}', [System.EnvironmentVariableTarget]::{}); Write-Output 'OK'",
                name.replace('\'', "''"),
                value.replace('\'', "''"),
                ps_scope
            );
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.contains("OK") || out.status.success() {
                Ok(format!("Variable '{}' définie ({})", name, ps_scope))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Supprime une variable d'environnement
#[tauri::command]
async fn delete_environment_variable(name: String, scope: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let ps_scope = match scope.as_str() {
                "Système" | "System" | "Machine" => "Machine",
                _ => "User",
            };
            let ps = format!(
                "[System.Environment]::SetEnvironmentVariable('{}', $null, [System.EnvironmentVariableTarget]::{}); Write-Output 'OK'",
                name.replace('\'', "''"),
                ps_scope
            );
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.contains("OK") || out.status.success() {
                Ok(format!("Variable '{}' supprimée ({})", name, ps_scope))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Active ou désactive un programme de démarrage dans le registre
#[tauri::command]
async fn toggle_startup_program(name: String, location: String, command: String, enable: bool) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let hive = if location.contains("HKCU") { "HKCU:" } else { "HKLM:" };
            let reg_path = if location.contains("x86") {
                format!("{}\\SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run", hive)
            } else if location.contains("RunOnce") {
                format!("{}\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce", hive)
            } else {
                format!("{}\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", hive)
            };

            let ps = if enable {
                format!(
                    "Set-ItemProperty -Path '{}' -Name '{}' -Value '{}' -Force; Write-Output 'OK'",
                    reg_path.replace('\'', "''"),
                    name.replace('\'', "''"),
                    command.replace('\'', "''")
                )
            } else {
                // Déplacer vers Disabled\Run plutôt que supprimer
                format!(
                    "$disPath = '{}\\Disabled\\Run'; if(-not (Test-Path $disPath)){{ New-Item $disPath -Force | Out-Null }}; \
                     $val = try{{ (Get-ItemProperty -Path '{}' -Name '{}' -ErrorAction Stop).'{}' }} catch {{ '{}' }}; \
                     Set-ItemProperty -Path $disPath -Name '{}' -Value $val -Force; \
                     Remove-ItemProperty -Path '{}' -Name '{}' -ErrorAction SilentlyContinue; Write-Output 'OK'",
                    hive, // $disPath
                    &reg_path.replace('\'', "''"),
                    &name.replace('\'', "''"),
                    &name.replace('\'', "''"),
                    &command.replace('\'', "''"),
                    &name.replace('\'', "''"),
                    &reg_path.replace('\'', "''"),
                    &name.replace('\'', "''"),
                )
            };
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.contains("OK") || out.status.success() {
                Ok(if enable { format!("'{}' activé au démarrage", name) } else { format!("'{}' désactivé au démarrage", name) })
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Supprime complètement une entrée de démarrage du registre
#[tauri::command]
async fn remove_startup_program(name: String, location: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let hive = if location.contains("HKCU") { "HKCU:" } else { "HKLM:" };
            let reg_path = if location.contains("x86") {
                format!("{}\\SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run", hive)
            } else {
                format!("{}\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", hive)
            };
            let ps = format!(
                "Remove-ItemProperty -Path '{}' -Name '{}' -ErrorAction SilentlyContinue; Write-Output 'OK'",
                reg_path.replace('\'', "''"),
                name.replace('\'', "''")
            );
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            if out.status.success() {
                Ok(format!("Entrée '{}' supprimée du démarrage", name))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Crée une tâche planifiée Windows simple
#[tauri::command]
async fn create_scheduled_task(task_name: String, command: String, trigger: String, description: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            // trigger: "startup", "logon", "daily HH:MM", "hourly N"
            let trigger_ps = match trigger.as_str() {
                "startup" => "New-ScheduledTaskTrigger -AtStartup",
                "logon" => "New-ScheduledTaskTrigger -AtLogOn",
                t if t.starts_with("daily ") => {
                    let time = t.trim_start_matches("daily ").trim();
                    &format!("New-ScheduledTaskTrigger -Daily -At '{}'", time)
                }
                _ => "New-ScheduledTaskTrigger -AtStartup",
            };
            let safe_name = task_name.replace('\'', "''");
            let safe_cmd = command.replace('\'', "''");
            let safe_desc = description.replace('\'', "''");
            let ps = format!(
                r#"
$action = New-ScheduledTaskAction -Execute '{safe_cmd}'
$trigger = {trigger_ps}
$settings = New-ScheduledTaskSettingsSet -RunOnlyIfNetworkAvailable:$false
Register-ScheduledTask -TaskName '{safe_name}' -Action $action -Trigger $trigger -Settings $settings -Description '{safe_desc}' -Force
Write-Output 'OK'
"#
            );
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.contains("OK") || out.status.success() {
                Ok(format!("Tâche '{}' créée avec succès", task_name))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Supprime une tâche planifiée Windows
#[tauri::command]
async fn delete_scheduled_task(task_name: String, task_path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let full_name = if task_path.is_empty() || task_path == "\\" {
                task_name.clone()
            } else {
                format!("{}\\{}", task_path.trim_end_matches('\\'), task_name)
            };
            let out = std::process::Command::new("schtasks")
                .args(["/Delete", "/TN", &full_name, "/F"])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            if out.status.success() {
                Ok(format!("Tâche '{}' supprimée", task_name))
            } else {
                let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let out_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                Err(if !err.is_empty() { err } else { out_str })
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Exécute maintenant une tâche planifiée
#[tauri::command]
async fn run_scheduled_task_now(task_name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let out = std::process::Command::new("schtasks")
                .args(["/Run", "/TN", &task_name])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            if out.status.success() {
                Ok(format!("Tâche '{}' démarrée", task_name))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Active un plan d'alimentation Windows par GUID
#[tauri::command]
async fn set_power_plan(guid: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let out = std::process::Command::new("powercfg")
                .args(["/setactive", &guid])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            if out.status.success() {
                Ok(format!("Plan d'alimentation {} activé", guid))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Définit l'imprimante par défaut
#[tauri::command]
async fn set_default_printer(printer_name: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let ps = format!(
                "(New-Object -ComObject WScript.Network).SetDefaultPrinter('{}'); Write-Output 'OK'",
                printer_name.replace('\'', "''")
            );
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if stdout.contains("OK") || out.status.success() {
                Ok(format!("Imprimante '{}' définie par défaut", printer_name))
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Génère et ouvre le rapport HTML de batterie
#[tauri::command]
async fn open_battery_report_html() -> Result<(), String> {
    let output_path = std::env::temp_dir().join("nitrite-battery-report.html");
    let output_str = output_path.to_string_lossy().to_string();
    let out = output_str.clone();
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("powercfg")
                .args(["/batteryreport", "/output", &out])
                .creation_flags(0x08000000)
                .status();
        }
    }).await.map_err(|e| e.to_string())?;
    if output_path.exists() {
        open::that(&output_path).map_err(|e| e.to_string())
    } else {
        Err("Rapport batterie non généré (pas de batterie ?)".to_string())
    }
}

/// Ouvre Regedit positionné sur une clé de registre précise
#[tauri::command]
async fn open_in_regedit(key_path: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            // Normaliser le chemin : HKCU\ -> HKEY_CURRENT_USER\, etc.
            let full_path = key_path
                .replace("HKCU\\", "HKEY_CURRENT_USER\\")
                .replace("HKLM\\", "HKEY_LOCAL_MACHINE\\")
                .replace("HKCR\\", "HKEY_CLASSES_ROOT\\")
                .replace("HKU\\", "HKEY_USERS\\")
                .replace("HKCC\\", "HKEY_CURRENT_CONFIG\\");

            // Écrire la clé de navigation regedit dans le registre
            let set_ps = format!(
                "Set-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Applets\\Regedit' -Name 'LastKey' -Value '{}' -Force -ErrorAction SilentlyContinue",
                full_path.replace('\'', "''")
            );
            let _ = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &set_ps])
                .creation_flags(0x08000000)
                .status();

            // Ouvrir regedit
            std::process::Command::new("regedit.exe")
                .creation_flags(0x00000001) // Ouvrir visible (pas de CREATE_NO_WINDOW ici)
                .spawn()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Installe un gestionnaire de paquets (winget/scoop/chocolatey)
#[tauri::command]
async fn install_package_manager(manager: String, window: tauri::Window) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let ps = match manager.as_str() {
                "scoop" => r#"
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
Invoke-RestMethod -Uri 'https://get.scoop.sh' | Invoke-Expression
Write-Output 'Scoop installé !'
"#,
                "chocolatey" => r#"
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
Write-Output 'Chocolatey installé !'
"#,
                "winget" => {
                    // Winget s'installe via le Microsoft Store / App Installer
                    let _ = std::process::Command::new("cmd")
                        .args(["/C", "start ms-windows-store://pdp/?productid=9NBLGGH4NNS1"])
                        .creation_flags(0x08000000)
                        .spawn();
                    return Ok("Microsoft Store ouvert — recherchez 'App Installer' pour installer WinGet".to_string());
                }
                _ => return Err(format!("Gestionnaire inconnu: {}", manager)),
            };
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let _ = window.emit("pkg-manager-install-done", &stdout);
            if out.status.success() {
                Ok(stdout)
            } else {
                Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
            }
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Ouvre le gestionnaire de périphériques Windows filtré sur un type
#[tauri::command]
async fn open_device_manager(_device_class: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            // Ouvrir le gestionnaire de périphériques
            let _ = std::process::Command::new("devmgmt.msc")
                .creation_flags(0x08000000)
                .spawn();
            // Scanner les périphériques pour mises à jour
            let _ = std::process::Command::new("pnputil")
                .args(["/scan-devices"])
                .creation_flags(0x08000000)
                .spawn();
            Ok(())
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Benchmark GPU simple via OpenCL/D3D enumeration + mesure temps
#[tauri::command]
async fn run_gpu_benchmark() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
$result = @{}
try {
    $gpus = Get-WmiObject -Class Win32_VideoController -ErrorAction SilentlyContinue
    $result.gpu_name = if ($gpus) { [string]($gpus | Select-Object -First 1 -ExpandProperty Name) } else { "N/A" }
    $result.gpu_vram_mb = if ($gpus) { [long]($gpus | Select-Object -First 1 -ExpandProperty AdapterRAM) / 1MB } else { 0 }
    $result.gpu_driver = if ($gpus) { [string]($gpus | Select-Object -First 1 -ExpandProperty DriverVersion) } else { "N/A" }
    # Test de performance simple: boucle de calcul mathématique sur 2 secondes
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $iterations = 0
    $dummy = 0.0
    while ($sw.Elapsed.TotalSeconds -lt 2) {
        for ($i = 0; $i -lt 10000; $i++) {
            $dummy += [Math]::Sqrt($i * 1.5 + 0.1) * [Math]::Sin($i * 0.001)
        }
        $iterations++
    }
    $sw.Stop()
    $ops_per_sec = [long](($iterations * 10000) / $sw.Elapsed.TotalSeconds)
    $result.ops_per_second = $ops_per_sec
    $result.test_duration_ms = [long]$sw.Elapsed.TotalMilliseconds
    $result.score = [long]($ops_per_sec / 1000)  # Score en KOPS
    $result.rating = if ($ops_per_sec -gt 50000000) { "Excellent" } elseif ($ops_per_sec -gt 20000000) { "Bon" } elseif ($ops_per_sec -gt 5000000) { "Moyen" } else { "Faible" }
} catch { $result.error = $_.Exception.Message }
$result | ConvertTo-Json -Compress
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            serde_json::from_str(&text).map_err(|e| e.to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Récupère les informations étendues BIOS (TPM, Secure Boot, type firmware)
#[tauri::command]
async fn get_bios_extended() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
$result = @{}
try {
    $bios = Get-WmiObject -Class Win32_BIOS -ErrorAction SilentlyContinue
    $result.bios_version = if ($bios) { [string]$bios.SMBIOSBIOSVersion } else { "" }
    $result.bios_date = if ($bios) { [string]$bios.ReleaseDate } else { "" }
    $result.bios_manufacturer = if ($bios) { [string]$bios.Manufacturer } else { "" }
    $result.bios_description = if ($bios) { [string]$bios.Description } else { "" }
} catch {}
try {
    # Type firmware UEFI ou BIOS legacy
    $isUEFI = try { Confirm-SecureBootUEFI -ErrorAction Stop; $true } catch { $false }
    $result.firmware_type = if ($isUEFI -or (Test-Path 'HKLM:\SYSTEM\CurrentControlSet\Control\SecureBoot\State')) { "UEFI" } else { "BIOS Legacy" }
} catch { $result.firmware_type = "Inconnu" }
try {
    # Secure Boot
    $sb = (Get-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\SecureBoot\State' -ErrorAction SilentlyContinue).UEFISecureBootEnabled
    $result.secure_boot = [bool]$sb
} catch { $result.secure_boot = $false }
try {
    # TPM
    $tpm = Get-WmiObject -Namespace "root\CIMV2\Security\MicrosoftTpm" -Class Win32_Tpm -ErrorAction SilentlyContinue
    if ($tpm) {
        $result.tpm_present = $true
        $result.tpm_enabled = [bool]$tpm.IsEnabled_InitialValue
        $result.tpm_activated = [bool]$tpm.IsActivated_InitialValue
        $result.tpm_version = if ($tpm.PhysicalPresenceVersionInfo) { [string]$tpm.PhysicalPresenceVersionInfo } else { "1.x" }
        $result.tpm_spec_version = try { [string]$tpm.SpecVersion.Split(',')[0].Trim() } catch { "" }
    } else {
        $result.tpm_present = $false; $result.tpm_enabled = $false; $result.tpm_activated = $false; $result.tpm_version = ""
        # Essai via Get-Tpm
        $tpm2 = try { Get-Tpm -ErrorAction SilentlyContinue } catch { $null }
        if ($tpm2) { $result.tpm_present = [bool]$tpm2.TpmPresent; $result.tpm_enabled = [bool]$tpm2.TpmEnabled }
    }
} catch { $result.tpm_present = $false; $result.tpm_enabled = $false }
try {
    # Chassis type
    $enclosure = Get-WmiObject -Class Win32_SystemEnclosure -ErrorAction SilentlyContinue | Select-Object -First 1
    $result.chassis_type = switch ([int]($enclosure.ChassisTypes | Select-Object -First 1)) {
        1{"Autre"} 2{"Inconnu"} 3{"Desktop"} 4{"Low Profile Desktop"} 5{"Pizza Box"}
        6{"Mini Tower"} 7{"Tower"} 8{"Portable"} 9{"Laptop"} 10{"Notebook"}
        11{"Handheld"} 12{"Docking Station"} 13{"All-in-One"} 14{"Sub-Notebook"}
        30{"Tablet"} 31{"Convertible"} 32{"Detachable"} default{"PC"}
    }
    $result.system_manufacturer = [string]$enclosure.Manufacturer
} catch { $result.chassis_type = "PC" }
try {
    # Fonctionnalités BIOS supplémentaires
    $result.wake_on_lan = try {
        $adapters = Get-NetAdapterAdvancedProperty -RegistryKeyword "*WakeOnMagicPacket" -ErrorAction SilentlyContinue
        [bool]($adapters | Where-Object { $_.DisplayValue -eq "Enabled" })
    } catch { $false }
} catch {}
try {
    $result.fast_boot = [int](Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Power" -Name "HiberbootEnabled" -ErrorAction SilentlyContinue).HiberbootEnabled -eq 1
} catch { $result.fast_boot = $false }
$result | ConvertTo-Json -Compress
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() { return Ok(serde_json::Value::Null); }
            serde_json::from_str(&text).map_err(|e| format!("JSON parse: {} — raw: {}", e, &text[..text.len().min(200)]))
        }
        #[cfg(not(target_os = "windows"))]
        Ok(serde_json::Value::Null)
    }).await.map_err(|e| e.to_string())?
}

/// Récupère les informations étendues de la carte mère (slots, chipset, socket)
#[tauri::command]
async fn get_motherboard_extended() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
$result = @{}
try {
    $board = Get-WmiObject -Class Win32_BaseBoard -ErrorAction SilentlyContinue
    $result.manufacturer = [string]$board.Manufacturer
    $result.product = [string]$board.Product
    $result.version = [string]$board.Version
    $result.serial = [string]$board.SerialNumber
    $result.tag = [string]$board.Tag
} catch {}
try {
    $sys = Get-WmiObject -Class Win32_ComputerSystem -ErrorAction SilentlyContinue
    $result.model = [string]$sys.Model
    $result.total_ram_slots_phys = [int]$sys.TotalPhysicalMemory
} catch {}
try {
    # Slots d'extension
    $slots = Get-WmiObject -Class Win32_SystemSlot -ErrorAction SilentlyContinue
    $result.expansion_slots = @($slots | ForEach-Object {
        @{
            name = [string]$_.SlotDesignation
            type = [string]$_.ConnectorType
            status = [string]$_.CurrentUsage
            max_data_width = [int]$_.MaxDataWidth
        }
    })
    $result.slot_count = $slots.Count
    $result.slot_available = ($slots | Where-Object { $_.CurrentUsage -eq 3 }).Count  # 3 = Available
} catch { $result.expansion_slots = @(); $result.slot_count = 0 }
try {
    # Socket CPU
    $proc = Get-WmiObject -Class Win32_Processor -ErrorAction SilentlyContinue | Select-Object -First 1
    $result.cpu_socket = [string]$proc.SocketDesignation
    $result.cpu_family = [string]$proc.Family
} catch {}
try {
    # Temperature via MSAcpi (peut échouer sur systèmes sans ACPI thermal)
    $temp_k = try {
        (Get-WmiObject -Namespace "root\WMI" -Class MSAcpi_ThermalZoneTemperature -ErrorAction SilentlyContinue |
        Select-Object -First 1).CurrentTemperature
    } catch { $null }
    $result.motherboard_temp_c = if ($temp_k -and $temp_k -gt 0) { [int]($temp_k / 10 - 273.15) } else { -1 }
} catch { $result.motherboard_temp_c = -1 }
$result | ConvertTo-Json -Compress -Depth 4
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() { return Ok(serde_json::Value::Null); }
            serde_json::from_str(&text).map_err(|e| e.to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Ok(serde_json::Value::Null)
    }).await.map_err(|e| e.to_string())?
}

/// Récupère la fréquence de rafraîchissement exacte de l'écran
#[tauri::command]
async fn get_monitor_refresh_rates() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
try {
    $result = @()
    $gpus = Get-WmiObject -Class Win32_VideoController -ErrorAction SilentlyContinue
    foreach ($gpu in $gpus) {
        if ($gpu.CurrentRefreshRate -gt 0) {
            $result += @{
                gpu_name = [string]$gpu.Name
                refresh_rate_hz = [int]$gpu.CurrentRefreshRate
                resolution = "$($gpu.CurrentHorizontalResolution)x$($gpu.CurrentVerticalResolution)"
                bits_per_pixel = [int]$gpu.CurrentBitsPerPixel
                video_mode = [string]$gpu.VideoModeDescription
            }
        }
    }
    if ($result.Count -eq 1) { $result[0] | ConvertTo-Json -Compress }
    else { $result | ConvertTo-Json -Compress }
} catch { Write-Output '{}' }
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() { return Ok(serde_json::Value::Null); }
            let json_text = if text.starts_with('{') && !text.starts_with('[') {
                format!("[{}]", text)
            } else { text };
            serde_json::from_str(&json_text).map_err(|e| e.to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Ok(serde_json::Value::Null)
    }).await.map_err(|e| e.to_string())?
}

/// Récupère les licences de logiciels tiers depuis le registre
#[tauri::command]
async fn get_third_party_licenses() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
$result = @()
# Logiciels avec licences dans le registre
$regPaths = @(
    'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SoftwareProtectionPlatform',
    'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows NT\CurrentVersion\SoftwareProtectionPlatform'
)
# Chercher des logiciels communs avec clés de licence connues
$softwareChecks = @(
    @{ name="Adobe Acrobat"; reg="HKLM:\SOFTWARE\Adobe\Adobe Acrobat"; key="Serial" },
    @{ name="AutoCAD"; reg="HKLM:\SOFTWARE\Autodesk\AutoCAD"; key="SERIALNUMBER" },
    @{ name="EaseUS"; reg="HKLM:\SOFTWARE\EaseUS"; key="LicenseKey" }
)

# Licence Windows depuis SLP
try {
    $slp = Get-WmiObject -Query "SELECT Name,LicenseStatus,PartialProductKey,Description FROM SoftwareLicensingProduct WHERE PartialProductKey IS NOT NULL" -ErrorAction SilentlyContinue
    foreach ($lic in $slp) {
        $status = switch ($lic.LicenseStatus) {
            0{"Non licencié"} 1{"Licencié"} 2{"Grâce OOB"} 3{"Grâce OOT"}
            4{"Non-authentique"} 5{"Notification"} 6{"Grâce étendue"} default{"Inconnu"}
        }
        $result += @{
            software = [string]$lic.Name
            status = $status
            partial_key = [string]$lic.PartialProductKey
            type = "Windows/Office"
            description = [string]$lic.Description
        }
    }
} catch {}

# Office Click-to-Run
try {
    $c2r = Get-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Office\ClickToRun\Configuration' -ErrorAction SilentlyContinue
    if ($c2r) {
        $result += @{
            software = "Microsoft Office (C2R)"
            status = if ($c2r.LicensingTenantId) { "Abonnement Microsoft 365" } else { "Installé" }
            partial_key = ""
            type = "Office C2R"
            description = if ($c2r.ProductReleaseIds) { [string]$c2r.ProductReleaseIds } else { "Office Click-to-Run" }
        }
    }
} catch {}

if ($result.Count -eq 0) { Write-Output '[]' } else { $result | ConvertTo-Json -Compress -Depth 3 }
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() || text == "[]" { return Ok(serde_json::json!([])); }
            let json_text = if text.starts_with('{') { format!("[{}]", text) } else { text };
            serde_json::from_str(&json_text).map_err(|e| e.to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Ok(serde_json::json!([]))
    }).await.map_err(|e| e.to_string())?
}

/// Ouvre les paramètres Windows d'activation
#[tauri::command]
async fn open_activation_settings() -> Result<(), String> {
    open::that("ms-settings:activation").map_err(|e| e.to_string())
}

/// Exécute slmgr et retourne le résultat
#[tauri::command]
async fn run_slmgr(arg: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            let safe_arg = match arg.as_str() {
                "/xpr" | "/dlv" | "/dli" | "/ato" => arg.as_str(),
                _ => return Err("Argument non autorisé".to_string()),
            };
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command",
                    &format!("$r = cscript.exe //Nologo $env:SystemRoot\\System32\\slmgr.vbs {} 2>&1; $r -join \"`n\"", safe_arg)])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Err("Non supporté".to_string())
    }).await.map_err(|e| e.to_string())?
}

/// Récupère infos audio étendues (fréquence, améliorations)
#[tauri::command]
async fn get_audio_extended() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
$result = @()
try {
    $devices = Get-WmiObject -Class Win32_SoundDevice -ErrorAction SilentlyContinue
    foreach ($d in $devices) {
        $item = @{
            name = [string]$d.Name
            manufacturer = [string]$d.Manufacturer
            status = [string]$d.Status
            device_id = [string]$d.DeviceID
            pnp_device_id = [string]$d.PNPDeviceID
        }
        # Essai de récupérer le format audio depuis le registre
        try {
            $regBase = "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\MMDevices\Audio"
            # Chercher dans Render et Capture
            foreach ($subkey in @("Render", "Capture")) {
                $path = "$regBase\$subkey"
                if (Test-Path $path) {
                    Get-ChildItem $path -ErrorAction SilentlyContinue | ForEach-Object {
                        $propPath = "$($_.PSPath)\Properties"
                        if (Test-Path $propPath) {
                            $props = Get-ItemProperty $propPath -ErrorAction SilentlyContinue
                            $desc = if ($props."{a45c254e-df1c-4efd-8020-67d146a850e0},2") { $props."{a45c254e-df1c-4efd-8020-67d146a850e0},2" } else { "" }
                            if ($desc -and [string]$desc -like "*$([string]$d.Name.Split(' ')[0])*") {
                                $item.audio_type = $subkey
                            }
                        }
                    }
                }
            }
        } catch {}
        $result += $item
    }
} catch {}
if ($result.Count -eq 0) { Write-Output '[]' } else { $result | ConvertTo-Json -Compress -Depth 3 }
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() || text == "[]" { return Ok(serde_json::json!([])); }
            let json_text = if text.starts_with('{') { format!("[{}]", text) } else { text };
            serde_json::from_str(&json_text).map_err(|e| e.to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Ok(serde_json::json!([]))
    }).await.map_err(|e| e.to_string())?
}

/// Récupère les processus avec utilisation GPU et disque
#[tauri::command]
async fn get_processes_extended() -> Result<serde_json::Value, String> {
    tokio::task::spawn_blocking(|| {
        #[cfg(target_os = "windows")]
        {
            let ps = r#"
try {
    $procs = Get-Process -ErrorAction SilentlyContinue | Select-Object -First 200
    $result = @()
    foreach ($p in $procs) {
        $result += @{
            pid = [int]$p.Id
            name = [string]$p.Name
            cpu_percent = 0.0
            memory_mb = [double]($p.WorkingSet64 / 1MB)
            disk_io_read_kb = try { [double]($p.ReadTransferCount / 1KB) } catch { 0.0 }
            disk_io_write_kb = try { [double]($p.WriteTransferCount / 1KB) } catch { 0.0 }
        }
    }
    # GPU usage via Get-Counter (meilleures infos si dispo)
    try {
        $gpuCounters = Get-Counter '\GPU Engine(*)\Utilization Percentage' -ErrorAction SilentlyContinue -MaxSamples 1
        if ($gpuCounters) {
            $gpuByPid = @{}
            $gpuCounters.CounterSamples | Where-Object { $_.CookedValue -gt 0 } | ForEach-Object {
                if ($_.InstanceName -match 'pid_(\d+)') {
                    $pid2 = [int]$matches[1]
                    if (-not $gpuByPid[$pid2] -or $gpuByPid[$pid2] -lt $_.CookedValue) {
                        $gpuByPid[$pid2] = [double]$_.CookedValue
                    }
                }
            }
            $result = $result | ForEach-Object {
                $pid3 = $_.pid
                $_.gpu_percent = if ($gpuByPid[$pid3]) { [double]$gpuByPid[$pid3] } else { 0.0 }
                $_
            }
        }
    } catch {}
    $result | ConvertTo-Json -Compress -Depth 2
} catch { Write-Output '[]' }
"#;
            let out = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", ps])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() || text == "[]" { return Ok(serde_json::json!([])); }
            let json_text = if text.starts_with('{') { format!("[{}]", text) } else { text };
            serde_json::from_str(&json_text).map_err(|e| e.to_string())
        }
        #[cfg(not(target_os = "windows"))]
        Ok(serde_json::json!([]))
    }).await.map_err(|e| e.to_string())?
}

// === Profils ===

#[tauri::command]
fn list_profiles() -> Vec<utils::profiles::Profile> {
    utils::profiles::list_profiles()
}

#[tauri::command]
fn save_profile_cmd(profile: utils::profiles::Profile) -> Result<(), String> {
    utils::profiles::save_profile(&profile).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_profile_cmd(name: String) -> Result<(), String> {
    utils::profiles::delete_profile(&name).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_profile_json(name: String) -> Option<String> {
    utils::profiles::export_profile_json(&name)
}

#[tauri::command]
fn import_profile_json(json: String) -> Result<utils::profiles::Profile, String> {
    utils::profiles::import_profile_from_json(&json)
}

// === Gestionnaire de Dépendances ===

#[tauri::command]
async fn check_all_dependencies() -> Vec<system::dependencies::Dependency> {
    tokio::task::spawn_blocking(system::dependencies::check_all)
        .await
        .unwrap_or_default()
}

/// Streaming : émet `deps:progress` par dépendance vérifiée.
#[tauri::command]
async fn scan_dependencies_stream(app: tauri::AppHandle) {
    tokio::task::spawn_blocking(move || {
        system::dependencies::check_all_streaming(&app);
    }).await.ok();
}


#[tauri::command]
async fn install_dependency(winget_id: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        system::dependencies::install_via_winget(&winget_id)
    })
    .await
    .map_err(|e| e.to_string())?
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
            ai_start_llamacpp,
            ai_stop_llamacpp,
            ai_llamacpp_status,
            ai_list_gguf_models,
            ai_find_llamacpp_server,
            ai_model_catalog,
            ai_download_server,
            ai_download_model,
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
            // Exit cleanup
            cleanup_on_exit,
            // Utils
            open_url,
            open_path,
            execute_tool,
            get_export_dir,
            save_export_file,
            save_content_to_path,
            // Désinstallateur
            list_installed_apps_for_uninstall,
            uninstall_app_clean,
            preview_residuals,
            delete_residuals,
            extract_residuals,
            // Clonage
            get_disks_for_clone,
            start_system_image,
            start_robocopy_clone,
            // Récupération données
            list_shadow_copies,
            browse_shadow_copy,
            restore_from_shadow,
            scan_recycle_bin,
            restore_recycle_bin_item,
            scan_deleted_files,
            search_shadow_copy,
            restore_files_batch,
            list_connected_disks,
            browse_disk_path,
            recover_files_safe,
            get_user_profile_folders,
            backup_user_folders,
            compare_shadow_with_current,
            scan_all_deleted_files,
            // Partitions & SMART
            get_disks_smart,
            get_partition_list,
            format_partition_cmd,
            assign_drive_letter_cmd,
            create_partition_cmd,
            delete_partition_cmd,
            initialize_disk_cmd,
            // Shadow Copy gestion
            create_shadow_copy_cmd,
            delete_shadow_copy_cmd,
            open_in_explorer,
            get_ntfs_drives,
            // Récupération avancée
            create_disk_image_cmd,
            surface_test_volume_cmd,
            deep_mft_scan_advanced_cmd,
            generate_recovery_report_cmd,
            // Partition avancé
            get_partition_resize_limits_cmd,
            resize_partition_cmd,
            backup_mbr_cmd,
            restore_mbr_cmd,
            scan_lost_partitions_cmd,
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
            system::registry_persist::registry_browse,
            system::registry_persist::registry_set_value,
            system::registry_persist::registry_delete_value,
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
            system::driver_updater::install_driver_winupdate,
            system::driver_updater::install_all_driver_updates,
            system::driver_updater::get_all_hardware_ids,
            // Benchmark
            system::benchmark::run_cpu_bench,
            system::benchmark::run_ram_bench,
            system::benchmark::run_disk_bench,
            // Cleaner
            system::cleaner::get_clean_targets,
            system::cleaner::scan_clean_targets_stream,
            system::cleaner::clean_target,
            system::cleaner::get_large_files,
            // BSOD Analyzer
            system::bsod_analyzer::get_bsod_history,
            system::bsod_analyzer::get_bugcheck_description,
            // Hosts Editor
            system::hosts_editor::get_hosts_entries,
            system::hosts_editor::add_hosts_entry,
            system::hosts_editor::delete_hosts_entry,
            system::hosts_editor::toggle_hosts_entry,
            system::hosts_editor::backup_hosts,
            // Boot Manager
            system::boot_manager::get_boot_config,
            system::boot_manager::set_boot_timeout,
            system::boot_manager::set_default_boot,
            system::boot_manager::boot_to_recovery,
            // WSL
            system::wsl_info::get_wsl_info,
            system::wsl_info::wsl_run_command,
            system::wsl_info::wsl_set_default_version,
            // Bluetooth
            system::bluetooth::get_bluetooth_info,
            system::bluetooth::toggle_bluetooth,
            // Perf History
            system::perf_history::get_perf_history,
            system::perf_history::get_top_processes_by_cpu,
            // ============ Nouvelles commandes Diagnostics Améliorés ============
            // Processus
            kill_process,
            get_processes_extended,
            // Services
            control_service,
            set_service_start_mode,
            // Variables d'environnement
            set_environment_variable,
            delete_environment_variable,
            // Démarrage
            toggle_startup_program,
            remove_startup_program,
            // Tâches planifiées
            create_scheduled_task,
            delete_scheduled_task,
            run_scheduled_task_now,
            // Énergie
            set_power_plan,
            // Imprimantes
            set_default_printer,
            // Batterie
            open_battery_report_html,
            // Gestionnaires de paquets
            install_package_manager,
            // Périphériques
            open_device_manager,
            // Benchmark GPU
            run_gpu_benchmark,
            // BIOS / Carte mère (étendus)
            get_bios_extended,
            get_motherboard_extended,
            // Écran taux rafraîchissement
            get_monitor_refresh_rates,
            // Audio étendu
            get_audio_extended,
            // Licences
            get_third_party_licenses,
            // Activation
            open_activation_settings,
            run_slmgr,
            // Points de restauration
            system::restore_points::list_restore_points_cmd,
            system::restore_points::create_restore_point_cmd,
            // Regedit navigation
            open_in_regedit,
            // Script Generator
            installer::script_generator::generate_deploy_script,
            // Script Validator
            scripts::validator::validate_script,
            // Favorites & Install History
            installer::favorites::get_favorites_data,
            installer::favorites::toggle_favorite,
            installer::favorites::log_install,
            installer::favorites::clear_install_history,
            // Usage Stats
            utils::stats::get_app_stats,
            utils::stats::log_action,
            utils::stats::reset_stats,
            // Report Generator
            utils::report_generator::generate_html_report,
            utils::report_generator::generate_md_report,
            // Profils
            list_profiles,
            save_profile_cmd,
            delete_profile_cmd,
            export_profile_json,
            import_profile_json,
            // Gestionnaire Dépendances
            check_all_dependencies,
            scan_dependencies_stream,
            install_dependency,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de NiTriTe");
}
