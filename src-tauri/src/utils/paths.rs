use std::path::PathBuf;

/// Retourne le dossier racine de l'application (a cote de l'exe)
pub fn app_root_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Dossier de configuration portable
pub fn config_dir() -> PathBuf {
    let root = app_root_dir();
    let portable = root.join("config");
    if portable.exists() || std::fs::create_dir_all(&portable).is_ok() {
        portable
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nitrite")
    }
}

/// Dossier de logs
pub fn logs_dir() -> PathBuf {
    let dir = app_root_dir().join(".logs");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Dossier de backups
pub fn backups_dir() -> PathBuf {
    let dir = app_root_dir().join("backups");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Dossier de telechargements
pub fn downloads_dir() -> PathBuf {
    let dir = app_root_dir().join("downloads");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Dossier des logiciels portables
pub fn portables_dir() -> PathBuf {
    let dir = app_root_dir().join("logiciel");
    let _ = std::fs::create_dir_all(&dir);
    dir
}
