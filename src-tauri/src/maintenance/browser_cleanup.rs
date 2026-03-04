use serde::Serialize;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::error::NiTriTeError;

#[derive(Debug, Clone, Serialize)]
pub struct BrowserCacheInfo {
    pub id: String,
    pub name: String,
    pub detected: bool,
    pub cache_size_mb: f64,
    pub cache_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CleanupResult {
    pub browser_id: String,
    pub freed_mb: f64,
    pub files_deleted: usize,
    pub errors: Vec<String>,
}

/// Detecte les navigateurs installes et calcule la taille de leurs caches
pub fn get_browser_cache_sizes() -> Vec<BrowserCacheInfo> {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();

    let browsers = vec![
        ("chrome", "Google Chrome", vec![
            format!("{}\\Google\\Chrome\\User Data\\Default\\Cache", local),
            format!("{}\\Google\\Chrome\\User Data\\Default\\Code Cache", local),
            format!("{}\\Google\\Chrome\\User Data\\Default\\Service Worker\\CacheStorage", local),
        ]),
        ("edge", "Microsoft Edge", vec![
            format!("{}\\Microsoft\\Edge\\User Data\\Default\\Cache", local),
            format!("{}\\Microsoft\\Edge\\User Data\\Default\\Code Cache", local),
            format!("{}\\Microsoft\\Edge\\User Data\\Default\\Service Worker\\CacheStorage", local),
        ]),
        ("firefox", "Mozilla Firefox", {
            let mut paths = Vec::new();
            let profiles_dir = format!("{}\\Mozilla\\Firefox\\Profiles", local);
            if let Ok(entries) = std::fs::read_dir(&profiles_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        paths.push(format!("{}\\cache2", p.display()));
                    }
                }
            }
            paths
        }),
        ("brave", "Brave Browser", vec![
            format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Cache", local),
            format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Code Cache", local),
        ]),
        ("opera", "Opera", vec![
            format!("{}\\Opera Software\\Opera Stable\\Cache", local),
            format!("{}\\Opera Software\\Opera Stable\\Code Cache", local),
        ]),
        ("vivaldi", "Vivaldi", vec![
            format!("{}\\Vivaldi\\User Data\\Default\\Cache", local),
            format!("{}\\Vivaldi\\User Data\\Default\\Code Cache", local),
        ]),
    ];

    browsers
        .into_iter()
        .map(|(id, name, paths)| {
            let existing_paths: Vec<String> = paths
                .iter()
                .filter(|p| PathBuf::from(p).exists())
                .cloned()
                .collect();

            let detected = !existing_paths.is_empty();
            let cache_size_mb = if detected {
                existing_paths.iter().map(|p| dir_size_mb(p)).sum()
            } else {
                0.0
            };

            BrowserCacheInfo {
                id: id.to_string(),
                name: name.to_string(),
                detected,
                cache_size_mb,
                cache_paths: existing_paths,
            }
        })
        .collect()
}

/// Nettoie les caches des navigateurs specifies
pub fn clean_browser_cache(browser_ids: Vec<String>) -> Result<Vec<CleanupResult>, NiTriTeError> {
    let all_browsers = get_browser_cache_sizes();
    let mut results = Vec::new();

    for browser in all_browsers {
        if !browser_ids.contains(&browser.id) || !browser.detected {
            continue;
        }

        let mut freed: u64 = 0;
        let mut deleted = 0usize;
        let mut errors = Vec::new();

        for cache_path in &browser.cache_paths {
            let path = PathBuf::from(cache_path);
            if !path.exists() {
                continue;
            }

            // Supprimer les fichiers dans le dossier cache
            for entry in WalkDir::new(&path).min_depth(1).into_iter().flatten() {
                let ep = entry.path();
                if ep.is_file() {
                    let size = ep.metadata().map(|m| m.len()).unwrap_or(0);
                    match std::fs::remove_file(ep) {
                        Ok(_) => {
                            freed += size;
                            deleted += 1;
                        }
                        Err(e) => {
                            // Fichier verrouille = normal si navigateur ouvert
                            if errors.len() < 5 {
                                errors.push(format!("{}: {}", ep.display(), e));
                            }
                        }
                    }
                }
            }
        }

        results.push(CleanupResult {
            browser_id: browser.id,
            freed_mb: freed as f64 / 1_048_576.0,
            files_deleted: deleted,
            errors,
        });
    }

    Ok(results)
}

fn dir_size_mb(path: &str) -> f64 {
    let total: u64 = WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum();
    total as f64 / 1_048_576.0
}
