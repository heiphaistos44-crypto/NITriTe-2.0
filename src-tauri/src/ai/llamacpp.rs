/// Module llama.cpp portable — gère llama-server.exe + modèles GGUF.
/// Pas d'installation requise : un binaire + un fichier .gguf suffisent.
use serde::Serialize;
use std::path::Path;
use std::process::Child;
use crate::error::NiTriTeError;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// ─── Modèle GGUF ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct GgufModel {
    pub name: String,
    pub path: String,
    pub size_gb: f64,
}

/// Scanne un dossier (récursif 1 niveau) à la recherche de fichiers .gguf
pub fn list_gguf_models(models_dir: &str) -> Vec<GgufModel> {
    let dir = Path::new(models_dir);
    if !dir.exists() {
        return vec![];
    }
    let mut models = vec![];
    let Ok(entries) = std::fs::read_dir(dir) else { return vec![] };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("gguf") {
            let size_gb = path.metadata().map(|m| m.len() as f64 / 1_073_741_824.0).unwrap_or(0.0);
            models.push(GgufModel {
                name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                path: path.to_string_lossy().into_owned(),
                size_gb,
            });
        }
    }
    models.sort_by(|a, b| a.name.cmp(&b.name));
    models
}

// ─── Détection llama-server ────────────────────────────────────────────────────

/// Cherche llama-server.exe dans les emplacements standards.
/// Retourne le chemin absolu ou None.
pub fn find_server_binary(exe_dir: &str) -> Option<String> {
    let candidates = vec![
        format!("{}\\logiciel\\AI\\llama-server.exe", exe_dir),
        format!("{}\\llama-server.exe", exe_dir),
        "llama-server".to_string(),   // dans le PATH
        "llama-server.exe".to_string(),
    ];
    for c in candidates {
        let p = Path::new(&c);
        if p.exists() {
            return Some(c);
        }
        // test PATH via where
        if !c.ends_with(".exe") || !c.contains('\\') {
            let mut cmd = std::process::Command::new("where");
            cmd.arg(&c);
            #[cfg(target_os = "windows")]
            cmd.creation_flags(0x08000000);
            if cmd.output().map(|o| o.status.success()).unwrap_or(false) {
                return Some(c);
            }
        }
    }
    None
}

// ─── Gestion du serveur ────────────────────────────────────────────────────────

/// Démarre llama-server avec le modèle GGUF donné.
/// Retourne le processus enfant pour pouvoir le tuer plus tard.
pub fn start_server(server_path: &str, model_path: &str, port: u16) -> Result<Child, String> {
    let mut cmd = std::process::Command::new(server_path);
    cmd.args([
        "--model", model_path,
        "--port", &port.to_string(),
        "--host", "127.0.0.1",
        "--ctx-size", "4096",
        "--threads", "4",
        "--n-gpu-layers", "0",   // CPU only par défaut (safe)
    ]);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    cmd.spawn().map_err(|e| format!("Impossible de démarrer llama-server: {}", e))
}

/// Vérifie si le serveur répond (HTTP GET /health)
pub async fn is_server_ready(port: u16) -> bool {
    let url = format!("http://127.0.0.1:{}/health", port);
    reqwest::get(&url).await.map(|r| r.status().is_success()).unwrap_or(false)
}

// ─── API chat (compatible OpenAI /v1/chat/completions) ────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub async fn chat(
    port: u16,
    model: &str,
    messages: Vec<ChatMessage>,
    temperature: f64,
) -> Result<String, NiTriTeError> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": false,
        "temperature": temperature,
    });
    let resp = client
        .post(format!("http://127.0.0.1:{}/v1/chat/completions", port))
        .json(&body)
        .timeout(std::time::Duration::from_secs(300))
        .send()
        .await
        .map_err(|e| NiTriTeError::OllamaUnavailable(e.to_string()))?;

    let result: serde_json::Value = resp.json().await?;
    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    Ok(content)
}
