use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AppEntry {
    pub name: String,
    pub winget_id: Option<String>,
    pub choco_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GeneratedScript {
    pub content: String,
    pub filename: String,
    pub app_count: usize,
}

/// Génère un script de déploiement (.bat ou .ps1) à partir d'une liste d'apps
#[tauri::command]
pub fn generate_deploy_script(apps: Vec<AppEntry>, format: String) -> GeneratedScript {
    let use_ps = format.to_lowercase() == "ps1";
    let app_count = apps.len();

    let content = if use_ps {
        generate_powershell(&apps)
    } else {
        generate_batch(&apps)
    };

    let filename = if use_ps {
        "deploy_nitrite.ps1".to_string()
    } else {
        "deploy_nitrite.bat".to_string()
    };

    GeneratedScript { content, filename, app_count }
}

fn generate_batch(apps: &[AppEntry]) -> String {
    let mut lines = vec![
        "@echo off".to_string(),
        ":: Script de déploiement généré par NiTriTe".to_string(),
        ":: Nécessite WinGet installé".to_string(),
        "".to_string(),
        "echo === NiTriTe — Installation des logiciels ===".to_string(),
        "echo.".to_string(),
    ];

    for app in apps {
        if let Some(wid) = &app.winget_id {
            lines.push(format!("echo [WinGet] Installation de {}...", app.name));
            lines.push(format!(
                "winget install --id {} --silent --accept-package-agreements --accept-source-agreements",
                wid
            ));
        } else if let Some(cid) = &app.choco_id {
            lines.push(format!("echo [Choco] Installation de {}...", app.name));
            lines.push(format!("choco install {} -y", cid));
        }
        lines.push("".to_string());
    }

    lines.push("echo === Terminé ===".to_string());
    lines.push("pause".to_string());
    lines.join("\r\n")
}

fn generate_powershell(apps: &[AppEntry]) -> String {
    let mut lines = vec![
        "# Script de déploiement généré par NiTriTe".to_string(),
        "# Nécessite WinGet (winget) installé".to_string(),
        "".to_string(),
        "Write-Host '=== NiTriTe — Installation des logiciels ===' -ForegroundColor Cyan".to_string(),
        "".to_string(),
    ];

    for app in apps {
        if let Some(wid) = &app.winget_id {
            lines.push(format!("Write-Host '[WinGet] {}...' -ForegroundColor Yellow", app.name));
            lines.push(format!(
                "winget install --id {} --silent --accept-package-agreements --accept-source-agreements",
                wid
            ));
        } else if let Some(cid) = &app.choco_id {
            lines.push(format!("Write-Host '[Choco] {}...' -ForegroundColor Yellow", app.name));
            lines.push(format!("choco install {} -y", cid));
        }
        lines.push("".to_string());
    }

    lines.push("Write-Host '=== Terminé ===' -ForegroundColor Green".to_string());
    lines.join("\n")
}
