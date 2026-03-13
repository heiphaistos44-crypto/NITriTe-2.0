/// Module llama.cpp portable — gère llama-server.exe + modèles GGUF.
/// 100% portable : aucune installation requise. Téléchargement automatique intégré.
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Child;
use crate::error::NiTriTeError;
use tokio::io::AsyncWriteExt;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// ─── Catalogue des modèles recommandés ────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct ModelCatalogEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_gb: f64,
    pub url: String,
    pub filename: String,
    pub recommended: bool,
}

pub fn model_catalog() -> Vec<ModelCatalogEntry> {
    vec![
        ModelCatalogEntry {
            id: "tinyllama".into(),
            name: "TinyLlama 1.1B".into(),
            description: "Ultra-léger. Idéal pour PC avec peu de RAM (4 GB+).".into(),
            size_gb: 0.67,
            url: "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".into(),
            filename: "tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".into(),
            recommended: false,
        },
        ModelCatalogEntry {
            id: "llama32-1b".into(),
            name: "Llama 3.2 1B".into(),
            description: "Modèle Meta récent. Rapide, qualité supérieure à TinyLlama.".into(),
            size_gb: 0.81,
            url: "https://huggingface.co/bartowski/Llama-3.2-1B-Instruct-GGUF/resolve/main/Llama-3.2-1B-Instruct-Q4_K_M.gguf".into(),
            filename: "Llama-3.2-1B-Instruct-Q4_K_M.gguf".into(),
            recommended: false,
        },
        ModelCatalogEntry {
            id: "qwen25-1b5".into(),
            name: "Qwen 2.5 1.5B".into(),
            description: "Excellent en raisonnement technique. Léger et précis.".into(),
            size_gb: 1.12,
            url: "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf".into(),
            filename: "qwen2.5-1.5b-instruct-q4_k_m.gguf".into(),
            recommended: false,
        },
        ModelCatalogEntry {
            id: "gemma2-2b".into(),
            name: "Gemma 2 2B".into(),
            description: "Modèle Google. Très bon pour l'assistance et les explications.".into(),
            size_gb: 1.71,
            url: "https://huggingface.co/bartowski/gemma-2-2b-it-GGUF/resolve/main/gemma-2-2b-it-Q4_K_M.gguf".into(),
            filename: "gemma-2-2b-it-Q4_K_M.gguf".into(),
            recommended: false,
        },
        ModelCatalogEntry {
            id: "phi3-mini".into(),
            name: "Phi-3 Mini 3.8B".into(),
            description: "Meilleur rapport qualité/taille. Recommandé pour l'assistance IT.".into(),
            size_gb: 2.2,
            url: "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf".into(),
            filename: "Phi-3-mini-4k-instruct-q4.gguf".into(),
            recommended: true,
        },
        ModelCatalogEntry {
            id: "mistral-7b".into(),
            name: "Mistral 7B".into(),
            description: "Haute qualité. Nécessite 8 GB RAM. Idéal pour analyses approfondies.".into(),
            size_gb: 4.37,
            url: "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf".into(),
            filename: "mistral-7b-instruct-v0.2.Q4_K_M.gguf".into(),
            recommended: false,
        },
    ]
}

// ─── Modèle GGUF local ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct GgufModel {
    pub name: String,
    pub path: String,
    pub size_gb: f64,
}

/// Scanne un dossier à la recherche de fichiers .gguf
pub fn list_gguf_models(models_dir: &str) -> Vec<GgufModel> {
    let dir = Path::new(models_dir);
    if !dir.exists() { return vec![]; }
    let Ok(entries) = std::fs::read_dir(dir) else { return vec![] };
    let mut models: Vec<GgufModel> = entries.flatten()
        .filter_map(|e| {
            let path = e.path();
            if path.extension().and_then(|x| x.to_str()) == Some("gguf") {
                let size_gb = path.metadata().map(|m| m.len() as f64 / 1_073_741_824.0).unwrap_or(0.0);
                Some(GgufModel {
                    name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                    path: path.to_string_lossy().into_owned(),
                    size_gb,
                })
            } else { None }
        })
        .collect();
    models.sort_by(|a, b| a.name.cmp(&b.name));
    models
}

// ─── Détection llama-server ────────────────────────────────────────────────────

/// Cherche llama-server.exe dans les emplacements standards.
pub fn find_server_binary(exe_dir: &str) -> Option<String> {
    let candidates = [
        format!("{}\\logiciel\\AI\\llama-server.exe", exe_dir),
        format!("{}\\llama-server.exe", exe_dir),
    ];
    for c in &candidates {
        if Path::new(c).exists() { return Some(c.clone()); }
    }
    None
}

/// Retourne le dossier des modèles (à côté de l'exe, dossier "models")
pub fn models_dir(exe_dir: &str) -> String {
    format!("{}\\models", exe_dir)
}

// ─── Téléchargement avec progression ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct DownloadProgress {
    pub name: String,
    pub downloaded_mb: f64,
    pub total_mb: f64,
    pub percent: u8,
    pub done: bool,
    pub error: Option<String>,
}

async fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Nitrite/26.33.0")
        .timeout(std::time::Duration::from_secs(3600))
        .build()
        .unwrap_or_default()
}

/// Télécharge llama-server.exe depuis la dernière release GitHub.
/// Émet des événements de progression via `emit_fn`.
pub async fn download_server(
    exe_dir: &str,
    emit_fn: impl Fn(DownloadProgress),
) -> Result<String, String> {
    let client = http_client().await;
    let dest_dir = format!("{}\\logiciel\\AI", exe_dir);
    std::fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    // 1. Récupérer la dernière release GitHub
    emit_fn(DownloadProgress { name: "llama-server.exe".into(), downloaded_mb: 0.0, total_mb: 0.0, percent: 0, done: false, error: None });
    let release: serde_json::Value = client
        .get("https://api.github.com/repos/ggml-org/llama.cpp/releases/latest")
        .send().await.map_err(|e| format!("GitHub API: {}", e))?
        .json().await.map_err(|e| format!("Parse release: {}", e))?;

    // 2. Trouver l'asset win-cpu-x64.zip
    let assets = release["assets"].as_array().ok_or("Aucun asset dans la release")?;
    let asset = assets.iter()
        .find(|a| a["name"].as_str().map(|n| n.contains("win-cpu-x64")).unwrap_or(false))
        .ok_or("Asset win-cpu-x64 introuvable")?;

    let url       = asset["browser_download_url"].as_str().ok_or("URL invalide")?.to_string();
    let total_size = asset["size"].as_u64().unwrap_or(0);
    let zip_path  = format!("{}\\llama-tmp.zip", dest_dir);

    // 3. Télécharger le ZIP
    download_with_progress(&client, &url, &zip_path, total_size, "llama-server.exe", &emit_fn).await
        .map_err(|e| format!("Téléchargement: {}", e))?;

    // 4. Extraire TOUS les fichiers (exe + DLLs) → 100% portable, zéro dépendance
    extract_all_zip(&zip_path, &dest_dir)
        .map_err(|e| format!("Extraction: {}", e))?;
    let _ = std::fs::remove_file(&zip_path);

    let server_path = format!("{}\\llama-server.exe", dest_dir);
    if !std::path::Path::new(&server_path).exists() {
        return Err("llama-server.exe introuvable après extraction".into());
    }
    emit_fn(DownloadProgress { name: "llama-server.exe".into(), downloaded_mb: 0.0, total_mb: 0.0, percent: 100, done: true, error: None });
    Ok(server_path)
}

/// Télécharge un fichier GGUF vers le dossier models/.
pub async fn download_model_file(
    url: &str,
    filename: &str,
    exe_dir: &str,
    emit_fn: impl Fn(DownloadProgress),
) -> Result<String, String> {
    let client = http_client().await;
    let models = models_dir(exe_dir);
    std::fs::create_dir_all(&models).map_err(|e| e.to_string())?;
    let dest = format!("{}\\{}", models, filename);

    // HEAD pour obtenir la taille
    let total_size = client.head(url).send().await
        .ok()
        .and_then(|r| r.headers().get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok()))
        .unwrap_or(0);

    download_with_progress(&client, url, &dest, total_size, filename, &emit_fn).await
        .map_err(|e| format!("Téléchargement: {}", e))?;

    emit_fn(DownloadProgress { name: filename.into(), downloaded_mb: 0.0, total_mb: 0.0, percent: 100, done: true, error: None });
    Ok(dest)
}

async fn download_with_progress(
    client: &reqwest::Client,
    url: &str,
    dest: &str,
    total_size: u64,
    name: &str,
    emit_fn: &impl Fn(DownloadProgress),
) -> Result<(), String> {
    let mut resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    let mut file = tokio::fs::File::create(dest).await.map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let total_mb = total_size as f64 / 1_048_576.0;

    while let Some(chunk) = resp.chunk().await.map_err(|e| e.to_string())? {
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        let percent = if total_size > 0 { ((downloaded * 100) / total_size).min(99) as u8 } else { 0 };
        emit_fn(DownloadProgress {
            name: name.into(),
            downloaded_mb: downloaded as f64 / 1_048_576.0,
            total_mb,
            percent,
            done: false,
            error: None,
        });
    }
    file.flush().await.map_err(|e| e.to_string())?;
    Ok(())
}

/// Extrait tous les fichiers du ZIP à plat dans dest_dir (ignore les sous-dossiers).
/// Cela garantit que llama-server.exe trouve ses DLLs au même endroit → 100% portable.
fn extract_all_zip(zip_path: &str, dest_dir: &str) -> Result<(), String> {
    let data = std::fs::read(zip_path).map_err(|e| e.to_string())?;
    let reader = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if entry.is_dir() { continue; }
        // Extraire seulement les fichiers utiles (exe, dll, so)
        let name = entry.name().to_string();
        let basename = name.split('/').last().unwrap_or(&name);
        let ext = basename.rsplit('.').next().unwrap_or("").to_lowercase();
        if !["exe", "dll", "so", "dylib"].contains(&ext.as_str()) { continue; }
        let dest = format!("{}\\{}", dest_dir, basename);
        let mut out = std::fs::File::create(&dest).map_err(|e| e.to_string())?;
        std::io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ─── Gestion du serveur ────────────────────────────────────────────────────────

pub fn start_server(server_path: &str, model_path: &str, port: u16) -> Result<Child, String> {
    let mut cmd = std::process::Command::new(server_path);
    cmd.args([
        "--model", model_path,
        "--port", &port.to_string(),
        "--host", "127.0.0.1",
        "--ctx-size", "4096",
        "--threads", "4",
        "--n-gpu-layers", "0",
    ]);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);
    cmd.spawn().map_err(|e| format!("Impossible de démarrer llama-server: {}", e))
}

pub async fn is_server_ready(port: u16) -> bool {
    reqwest::get(format!("http://127.0.0.1:{}/health", port))
        .await.map(|r| r.status().is_success()).unwrap_or(false)
}

// ─── API chat (OpenAI-compatible) ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        .send().await
        .map_err(|e| NiTriTeError::OllamaUnavailable(e.to_string()))?;
    let result: serde_json::Value = resp.json().await?;
    Ok(result["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
}
