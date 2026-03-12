use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::paths;

fn default_true() -> bool { true }
fn default_10() -> u64 { 10 }
fn default_normal() -> String { "normal".to_string() }
fn default_json() -> String { "json".to_string() }
fn default_temperature() -> f64 { 0.7 }
fn default_ollama_url() -> String { "http://localhost:11434".to_string() }
fn default_ollama_model() -> String { "llama3:8b".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
    pub sidebar_collapsed: bool,
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,
    #[serde(default = "default_ollama_model")]
    pub ollama_model: String,
    #[serde(default = "default_temperature")]
    pub ollama_temperature: f64,
    pub monitor_interval_ms: u64,
    #[serde(default = "default_true")]
    pub show_animations: bool,
    #[serde(default)]
    pub compact_mode: bool,
    #[serde(default = "default_true")]
    pub notifications_enabled: bool,
    #[serde(default = "default_10")]
    pub process_count: u64,
    #[serde(default = "default_normal")]
    pub font_size: String,
    #[serde(default = "default_json")]
    pub export_format: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "nitrite-dark".to_string(),
            language: "fr".to_string(),
            sidebar_collapsed: false,
            ollama_url: "http://localhost:11434".to_string(),
            ollama_model: "llama3:8b".to_string(),
            ollama_temperature: 0.7,
            monitor_interval_ms: 2000,
            show_animations: true,
            compact_mode: false,
            notifications_enabled: true,
            process_count: 10,
            font_size: "normal".to_string(),
            export_format: "json".to_string(),
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
