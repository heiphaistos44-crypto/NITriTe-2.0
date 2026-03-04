use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::utils::config::AppConfig;

pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub monitor_running: Arc<AtomicBool>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            monitor_running: Arc::new(AtomicBool::new(false)),
        }
    }
}
