use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Child;
use tokio::sync::Mutex;

use crate::utils::config::AppConfig;

/// Cache d'une commande WMI : valeur sérialisée + timestamp d'acquisition
pub struct CacheEntry {
    pub data: serde_json::Value,
    pub acquired_at: Instant,
}

impl CacheEntry {
    pub fn is_fresh(&self, ttl_secs: u64) -> bool {
        self.acquired_at.elapsed().as_secs() < ttl_secs
    }
}

pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub monitor_running: Arc<AtomicBool>,
    /// Processus llama-server en cours (None si arrêté).
    /// Utilise tokio::process::Child pour que wait() soit non-bloquant en contexte async.
    pub llamacpp_process: Arc<Mutex<Option<Child>>>,
    /// Cache WMI — clé = nom commande, valeur = (json, timestamp)
    /// TTL par défaut : 30 secondes
    pub wmi_cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            monitor_running: Arc::new(AtomicBool::new(false)),
            llamacpp_process: Arc::new(Mutex::new(None)),
            wmi_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
