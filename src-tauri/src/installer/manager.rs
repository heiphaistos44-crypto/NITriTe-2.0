use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub winget_id: Option<String>,
    pub choco_id: Option<String>,
    pub url: Option<String>,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntry {
    pub name: String,
    pub description: String,
    pub command: String,
    pub is_url: bool,
    pub section: String,
    pub icon: String,
}

static APPS: LazyLock<Vec<AppEntry>> = LazyLock::new(|| {
    let json = include_str!("../../data/programs.json");
    serde_json::from_str(json).unwrap_or_else(|e| {
        tracing::error!("Erreur chargement programs.json: {}", e);
        Vec::new()
    })
});

static TOOLS: LazyLock<Vec<ToolEntry>> = LazyLock::new(|| {
    let json = include_str!("../../data/tools.json");
    serde_json::from_str(json).unwrap_or_else(|e| {
        tracing::error!("Erreur chargement tools.json: {}", e);
        Vec::new()
    })
});

pub fn get_default_apps() -> Vec<AppEntry> {
    APPS.clone()
}

pub fn get_tools() -> Vec<ToolEntry> {
    TOOLS.clone()
}
