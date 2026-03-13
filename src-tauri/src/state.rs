use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::utils::config::AppConfig;

pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub monitor_running: Arc<AtomicBool>,
    /// Processus llama-server en cours (None si arrêté)
    pub llamacpp_process: Arc<Mutex<Option<std::process::Child>>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            monitor_running: Arc::new(AtomicBool::new(false)),
            llamacpp_process: Arc::new(Mutex::new(None)),
        }
    }
}
