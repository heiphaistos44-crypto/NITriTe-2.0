use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::paths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
    pub sidebar_collapsed: bool,
    pub ollama_url: String,
    pub ollama_model: String,
    pub monitor_interval_ms: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "nitrite-dark".to_string(),
            language: "fr".to_string(),
            sidebar_collapsed: false,
            ollama_url: "http://localhost:11434".to_string(),
            ollama_model: "llama3:8b".to_string(),
            monitor_interval_ms: 2000,
        }
    }
}

impl AppConfig {
    fn config_file() -> PathBuf {
        paths::config_dir().join("nitrite_config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_file();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            let config = Self::default();
            let _ = config.save();
            config
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_file();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }
}
