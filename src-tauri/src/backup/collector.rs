use serde::Serialize;
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use crate::error::NiTriTeError;
use crate::utils::paths;

/// Écrit le script PS dans un fichier .ps1 temporaire et l'exécute.
/// Évite les limites de taille de -Command et les problèmes d'échappement.
fn run_ps_temp(script: &str) -> Result<String, NiTriTeError> {
    let tmp = std::env::temp_dir().join("nitrite_backup_ps.ps1");
    std::fs::write(&tmp, script.as_bytes())
        .map_err(|e| NiTriTeError::System(e.to_string()))?;
    let out = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File",
               tmp.to_str().unwrap_or("")])
        .creation_flags(0x08000000)
        .output()
        .map_err(|e| NiTriTeError::System(e.to_string()))?;
    let _ = std::fs::remove_file(&tmp);
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupManifest {
    pub timestamp: String,
    pub items: Vec<BackupItem>,
    pub total_items: usize,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupItem {
    pub name: String,
    pub category: String,
    pub data: String,
    pub size_bytes: usize,
}

pub fn create_backup(selected_items: Vec<String>, format: String, custom_path: Option<String>) -> Result<BackupManifest, NiTriTeError> {
    let mut items = Vec::new();
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    for item_id in &selected_items {
        match item_id.as_str() {
            "installed_apps" => {
                if let Ok(data) = collect_installed_apps() {
                    items.push(BackupItem { name: "Applications installees".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "drivers" => {
                if let Ok(data) = collect_drivers() {
                    items.push(BackupItem { name: "Pilotes systeme".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "network_config" => {
                if let Ok(data) = collect_network_config() {
                    items.push(BackupItem { name: "Configuration reseau".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "startup_programs" => {
                if let Ok(data) = collect_startup() {
                    items.push(BackupItem { name: "Programmes demarrage".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "env_variables" => {
                if let Ok(data) = collect_env_vars() {
                    items.push(BackupItem { name: "Variables d'environnement".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "firewall_rules" => {
                if let Ok(data) = collect_firewall_rules() {
                    items.push(BackupItem { name: "Regles pare-feu".into(), category: "Securite".into(), size_bytes: data.len(), data });
                }
            }
            "chrome_bookmarks" => {
                if let Ok(data) = collect_browser_bookmarks("Google\\Chrome") {
                    items.push(BackupItem { name: "Favoris Chrome".into(), category: "Navigateurs".into(), size_bytes: data.len(), data });
                }
            }
            "edge_bookmarks" => {
                if let Ok(data) = collect_browser_bookmarks("Microsoft\\Edge") {
                    items.push(BackupItem { name: "Favoris Edge".into(), category: "Navigateurs".into(), size_bytes: data.len(), data });
                }
            }
            "brave_bookmarks" => {
                if let Ok(data) = collect_browser_bookmarks("BraveSoftware\\Brave-Browser") {
                    items.push(BackupItem { name: "Favoris Brave".into(), category: "Navigateurs".into(), size_bytes: data.len(), data });
                }
            }
            "windows_license" => {
                if let Ok(data) = collect_windows_license() {
                    items.push(BackupItem { name: "Licence Windows".into(), category: "Licences".into(), size_bytes: data.len(), data });
                }
            }
            "bitlocker_keys" => {
                if let Ok(data) = collect_bitlocker_keys() {
                    items.push(BackupItem { name: "Cles BitLocker".into(), category: "Securite".into(), size_bytes: data.len(), data });
                }
            }
            "office_license" => {
                if let Ok(data) = collect_office_license() {
                    items.push(BackupItem { name: "Licence Office".into(), category: "Licences".into(), size_bytes: data.len(), data });
                }
            }
            "installed_fonts" => {
                if let Ok(data) = collect_installed_fonts() {
                    items.push(BackupItem { name: "Polices installees".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "scheduled_tasks" => {
                if let Ok(data) = collect_scheduled_tasks() {
                    items.push(BackupItem { name: "Taches planifiees".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "windows_features" => {
                if let Ok(data) = collect_windows_features() {
                    items.push(BackupItem { name: "Fonctionnalites Windows".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "folder_sizes" => {
                if let Ok(data) = collect_folder_sizes() {
                    items.push(BackupItem { name: "Taille des dossiers C:\\".into(), category: "Diagnostic".into(), size_bytes: data.len(), data });
                }
            }
            "desktop_files" => {
                if let Ok(data) = collect_desktop_files() {
                    items.push(BackupItem { name: "Fichiers Bureau".into(), category: "Utilisateur".into(), size_bytes: data.len(), data });
                }
            }
            "wifi_passwords" => {
                if let Ok(data) = collect_wifi_passwords() {
                    items.push(BackupItem { name: "Mots de passe WiFi".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "registry_export" => {
                if let Ok(data) = collect_registry_export() {
                    items.push(BackupItem { name: "Export registre (HKCU)".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "suspicious_processes" => {
                if let Ok(data) = collect_suspicious_processes() {
                    items.push(BackupItem { name: "Processus suspects".into(), category: "Securite".into(), size_bytes: data.len(), data });
                }
            }
            "winget_export" => {
                if let Ok(data) = collect_winget_export() {
                    items.push(BackupItem { name: "WinGet Export JSON".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "network_shares" => {
                if let Ok(data) = collect_network_shares() {
                    items.push(BackupItem { name: "Partages reseau".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "hosts_file" => {
                if let Ok(data) = collect_hosts_file() {
                    items.push(BackupItem { name: "Fichier hosts".into(), category: "Reseau".into(), size_bytes: data.len(), data });
                }
            }
            "ssh_keys" => {
                if let Ok(data) = collect_ssh_keys() {
                    items.push(BackupItem { name: "Cles SSH".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "pip_packages" => {
                if let Ok(data) = collect_pip_packages() {
                    items.push(BackupItem { name: "Packages Python (pip)".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "vscode_extensions" => {
                if let Ok(data) = collect_vscode_extensions() {
                    items.push(BackupItem { name: "Extensions VSCode".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "wsl_config" => {
                if let Ok(data) = collect_wsl_config() {
                    items.push(BackupItem { name: "Config WSL".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "powershell_profile" => {
                if let Ok(data) = collect_powershell_profile() {
                    items.push(BackupItem { name: "Profil PowerShell".into(), category: "Dev".into(), size_bytes: data.len(), data });
                }
            }
            "power_plans" => {
                if let Ok(data) = collect_power_plans() {
                    items.push(BackupItem { name: "Plans alimentation".into(), category: "Systeme".into(), size_bytes: data.len(), data });
                }
            }
            "printer_config" => {
                if let Ok(data) = collect_printer_config() {
                    items.push(BackupItem { name: "Imprimantes".into(), category: "Materiel".into(), size_bytes: data.len(), data });
                }
            }
            "system_components" => {
                if let Ok(data) = collect_system_components() {
                    items.push(BackupItem { name: "Composants PC".into(), category: "Materiel".into(), size_bytes: data.len(), data });
                }
            }
            _ => {}
        }
    }

    // Tri par catégorie pour éviter les sections dupliquées dans le rendu
    items.sort_by(|a, b| a.category.cmp(&b.category));
    let total_items = items.len();

    let backup_dir = if let Some(ref p) = custom_path {
        std::path::PathBuf::from(p)
    } else {
        paths::backups_dir()
    };
    std::fs::create_dir_all(&backup_dir)
        .map_err(|e| NiTriTeError::System(format!("Impossible de créer le dossier backups: {}", e)))?;

    let ext = match format.as_str() {
        "html" => "html",
        "md"   => "md",
        "txt"  => "txt",
        _      => "json",
    };
    let backup_file = backup_dir.join(format!("backup_{}.{}", timestamp, ext));
    let path_str = backup_file.to_string_lossy().to_string();

    let manifest = BackupManifest {
        timestamp: timestamp.clone(),
        items: items.clone(),
        total_items,
        path: path_str.clone(),
    };

    let content = match format.as_str() {
        "txt"  => render_txt(&manifest),
        "html" => render_html(&manifest),
        "md"   => render_md(&manifest),
        _      => serde_json::to_string_pretty(&manifest)
            .map_err(|e| NiTriTeError::System(format!("Sérialisation échouée: {}", e)))?,
    };

    std::fs::write(&backup_file, content.as_bytes())
        .map_err(|e| NiTriTeError::System(format!("Écriture fichier échouée ({}): {}", path_str, e)))?;

    Ok(manifest)
}

fn render_txt(m: &BackupManifest) -> String {
    let width = 72usize;
    let border = "═".repeat(width);
    let thin   = "─".repeat(width);

    let mut out = String::new();

    // En-tête principal
    out.push_str(&format!("╔{}╗\n", border));
    let title = "N i T r i T e  —  R a p p o r t  d e  S a u v e g a r d e";
    let pad = (width.saturating_sub(title.chars().count())) / 2;
    out.push_str(&format!("║{}{}{:>pad2$}║\n",
        " ".repeat(pad), title,
        " ",
        pad2 = width.saturating_sub(pad).saturating_sub(title.chars().count())));
    out.push_str(&format!("╠{}╣\n", border));
    out.push_str(&format!("║  Date      : {:<width$}║\n", m.timestamp, width = width - 15));
    out.push_str(&format!("║  Éléments  : {:<width$}║\n", m.total_items, width = width - 15));
    out.push_str(&format!("╚{}╝\n\n", border));

    // Grouper par catégorie
    let mut current_cat = String::new();
    for item in &m.items {
        // Nouveau groupe catégorie
        if item.category != current_cat {
            current_cat = item.category.clone();
            out.push_str(&format!("┌{}┐\n", thin));
            let cat_label = format!("  {}  ", current_cat.to_uppercase());
            let cat_pad = width.saturating_sub(cat_label.chars().count());
            out.push_str(&format!("│ {:<width$}│\n", format!("▶  {}", current_cat.to_uppercase()), width = width - 1));
            out.push_str(&format!("└{}┘\n", thin));
            let _ = cat_pad;
            let _ = cat_label;
        }

        // Bloc item
        out.push_str(&format!("\n  ┌─ {} {}\n", item.name, "─".repeat(width.saturating_sub(item.name.chars().count() + 4))));

        // Données — indenter chaque ligne (convertir JSON si nécessaire)
        let readable_data = json_to_readable(&item.data);
        for line in readable_data.lines().take(200) {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                out.push_str("  │\n");
            } else {
                // Couper les lignes trop longues
                let max_line = width - 4;
                if trimmed.len() <= max_line {
                    out.push_str(&format!("  │  {}\n", trimmed));
                } else {
                    let mut remaining = trimmed;
                    let mut first = true;
                    while !remaining.is_empty() {
                        // safe char boundary
                        let cut = remaining.char_indices().nth(max_line).map(|(i, _)| i).unwrap_or(remaining.len());
                        let (chunk, rest) = remaining.split_at(cut);
                        if first {
                            out.push_str(&format!("  │  {}\n", chunk));
                            first = false;
                        } else {
                            out.push_str(&format!("  │     {}\n", chunk));
                        }
                        remaining = rest;
                    }
                }
            }
        }
        if readable_data.lines().count() > 200 {
            out.push_str(&format!("  │  … ({} lignes supplémentaires tronquées)\n",
                readable_data.lines().count() - 200));
        }
        out.push_str(&format!("  └{}\n", "─".repeat(width - 2)));
    }

    // Pied de page
    out.push('\n');
    out.push_str(&format!("╔{}╗\n", border));
    out.push_str(&format!("║  Généré par NiTriTe {:<width$}║\n",
        format!("— {} éléments exportés", m.total_items), width = width - 22));
    out.push_str(&format!("╚{}╝\n", border));

    out
}

fn render_md(m: &BackupManifest) -> String {
    let mut out = format!("# NiTriTe — Rapport de Sauvegarde\n\n**Date :** `{}`  \n**Total :** {} éléments\n\n---\n\n",
        m.timestamp, m.total_items);
    let mut current_cat = String::new();
    for item in &m.items {
        if item.category != current_cat {
            out.push_str(&format!("## {}\n\n", item.category));
            current_cat = item.category.clone();
        }
        out.push_str(&format!("### {}\n\n```\n{}\n```\n\n", item.name, json_to_readable(&item.data)));
    }
    out
}

fn render_html(m: &BackupManifest) -> String {
    // Comptage des items par catégorie
    let mut cat_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for item in &m.items {
        *cat_counts.entry(item.category.clone()).or_insert(0) += 1;
    }

    // Ordre d'apparition des catégories
    let mut cats_seen: Vec<String> = Vec::new();
    for item in &m.items {
        if !cats_seen.contains(&item.category) {
            cats_seen.push(item.category.clone());
        }
    }

    // Liens de navigation avec compteur
    let nav_links: String = cats_seen.iter().map(|c| {
        let slug = c.to_lowercase().replace(' ', "-");
        let count = cat_counts.get(c).copied().unwrap_or(0);
        format!(
            "<a href='#cat-{slug}' class='nav-link'>{c}<span class='nav-count'>{count}</span></a>",
            slug = slug, c = html_escape(c), count = count
        )
    }).collect::<Vec<_>>().join("\n");

    // Sections collapsibles (<details>/<summary> = natif, 0 JS requis)
    let mut sections = String::new();
    let mut current_cat = String::new();
    for item in &m.items {
        if item.category != current_cat {
            if !current_cat.is_empty() {
                sections.push_str("</div></details>\n");
            }
            current_cat = item.category.clone();
            let slug = current_cat.to_lowercase().replace(' ', "-");
            let count = cat_counts.get(&current_cat).copied().unwrap_or(0);
            sections.push_str(&format!(
                "<details class='section' id='cat-{slug}' open>\
<summary class='section-header'>\
<span class='section-arrow'>▶</span>\
<h2>{cat}</h2>\
<span class='section-count'>{count} élément{s}</span>\
</summary>\
<div class='cards'>\n",
                slug  = slug,
                cat   = html_escape(&current_cat),
                count = count,
                s     = if count > 1 { "s" } else { "" },
            ));
        }
        let data_html = html_escape(&json_to_readable(&item.data));
        sections.push_str(&format!(
            "<details class='card' open>\
<summary class='card-title'>\
<span class='card-arrow'>▶</span>{name}\
</summary>\
<pre class='card-data'>{data}</pre>\
</details>\n",
            name = html_escape(&item.name),
            data = data_html,
        ));
    }
    if !current_cat.is_empty() {
        sections.push_str("</div></details>\n");
    }

    format!(r#"<!DOCTYPE html>
<html lang="fr">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>NiTriTe — Rapport de Sauvegarde — {ts}</title>
<style>
  *{{box-sizing:border-box;margin:0;padding:0}}
  :root{{
    --bg:#09090b;--bg2:#111113;--bg3:#18181b;--border:#27272a;--border2:#3f3f46;
    --accent:#f97316;--text:#e4e4e7;--text2:#a1a1aa;--text3:#52525b;
    --success:#22c55e;--radius:10px;
  }}
  body{{font-family:Inter,system-ui,sans-serif;background:var(--bg);color:var(--text);
    display:flex;min-height:100vh;font-size:13px}}

  /* ── Sidebar ── */
  nav{{width:230px;flex-shrink:0;background:var(--bg2);border-right:1px solid var(--border);
    padding:0;position:sticky;top:0;height:100vh;overflow-y:auto;display:flex;flex-direction:column}}
  .nav-brand{{padding:20px;border-bottom:1px solid var(--border);flex-shrink:0}}
  .nav-brand-title{{font-size:18px;font-weight:800;
    background:linear-gradient(135deg,#fafafa 40%,#f97316);
    -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}}
  .nav-brand-sub{{font-size:10px;color:var(--text3);font-family:'Courier New',monospace;margin-top:3px}}
  .nav-actions{{padding:10px 12px;border-bottom:1px solid var(--border);
    display:flex;gap:6px;flex-shrink:0}}
  .nav-btn{{flex:1;padding:5px 8px;border:1px solid var(--border);border-radius:6px;
    background:var(--bg3);color:var(--text2);cursor:pointer;font-size:11px;
    font-family:inherit;transition:all .15s;text-align:center}}
  .nav-btn:hover{{border-color:var(--accent);color:var(--accent)}}
  .nav-label{{padding:8px 16px 4px;font-size:9px;font-weight:700;color:var(--text3);
    text-transform:uppercase;letter-spacing:.12em}}
  .nav-link{{display:flex;align-items:center;justify-content:space-between;
    padding:7px 16px;font-size:12px;color:var(--text2);text-decoration:none;
    border-left:2px solid transparent;transition:all .15s}}
  .nav-link:hover{{color:var(--accent);border-left-color:var(--accent);
    background:rgba(249,115,22,.06)}}
  .nav-count{{font-size:10px;background:var(--bg3);border:1px solid var(--border);
    border-radius:99px;padding:1px 7px;color:var(--text3);font-weight:500}}

  /* ── Main ── */
  main{{flex:1;padding:32px 40px;min-width:0;max-width:1200px}}

  /* ── Header ── */
  .report-header{{margin-bottom:28px}}
  .report-title{{font-size:28px;font-weight:800;margin-bottom:10px;
    background:linear-gradient(135deg,#fafafa 40%,#f97316);
    -webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}}
  .report-meta{{display:flex;gap:10px;flex-wrap:wrap}}
  .meta-pill{{display:inline-flex;align-items:center;gap:6px;padding:5px 12px;
    background:var(--bg3);border:1px solid var(--border);border-radius:99px;
    font-size:11px;color:var(--text2)}}
  .meta-pill strong{{color:var(--accent)}}

  /* ── Sections collapsibles ── */
  details.section{{margin-bottom:20px;scroll-margin-top:16px;
    border:1px solid var(--border);border-radius:var(--radius);overflow:hidden}}
  details.section > summary.section-header{{
    display:flex;align-items:center;gap:12px;
    padding:14px 18px;cursor:pointer;list-style:none;
    background:var(--bg3);border-bottom:1px solid transparent;
    user-select:none;transition:background .15s}}
  details.section > summary.section-header::-webkit-details-marker{{display:none}}
  details.section > summary:hover{{background:var(--bg2)}}
  details.section[open] > summary{{border-bottom-color:var(--border)}}
  .section-arrow{{color:var(--accent);font-size:9px;display:inline-block;
    transition:transform .2s;flex-shrink:0}}
  details.section[open] > summary .section-arrow{{transform:rotate(90deg)}}
  details.section > summary h2{{font-size:13px;font-weight:700;text-transform:uppercase;
    letter-spacing:.06em;color:var(--text2);flex:1}}
  .section-count{{font-size:10px;background:rgba(249,115,22,.12);color:var(--accent);
    border:1px solid rgba(249,115,22,.25);border-radius:99px;padding:2px 10px;
    font-weight:600;flex-shrink:0}}

  /* ── Cards collapsibles dans la section ── */
  .cards{{display:flex;flex-direction:column;gap:0;padding:12px;gap:8px}}
  details.card{{background:var(--bg2);border:1px solid var(--border);
    border-radius:8px;overflow:hidden;transition:border-color .2s}}
  details.card:hover{{border-color:var(--border2)}}
  details.card[open]{{border-color:var(--border2)}}
  details.card > summary.card-title{{
    display:flex;align-items:center;gap:10px;
    padding:10px 14px;cursor:pointer;list-style:none;
    font-size:12px;font-weight:600;color:var(--text2);
    background:var(--bg3);user-select:none;transition:color .15s;
    border-bottom:1px solid transparent}}
  details.card > summary.card-title::-webkit-details-marker{{display:none}}
  details.card > summary:hover{{color:var(--text)}}
  details.card[open] > summary{{border-bottom-color:var(--border)}}
  .card-arrow{{font-size:8px;color:var(--text3);display:inline-block;
    transition:transform .2s;flex-shrink:0}}
  details.card[open] > summary .card-arrow{{transform:rotate(90deg)}}

  /* ── Contenu des cards (pre) ── */
  pre.card-data{{
    padding:14px 16px;
    font-size:11.5px;font-family:'JetBrains Mono','Courier New',monospace;
    color:var(--text);white-space:pre-wrap;word-break:break-word;
    line-height:1.7;
    /* PAS de max-height — tout le contenu est visible */
    overflow-x:auto;
    scrollbar-width:thin;scrollbar-color:var(--border2) transparent;
  }}
  pre.card-data::-webkit-scrollbar{{height:4px}}
  pre.card-data::-webkit-scrollbar-thumb{{background:var(--border2);border-radius:99px}}

  /* ── Footer ── */
  .report-footer{{margin-top:40px;padding-top:16px;border-top:1px solid var(--border);
    font-size:11px;color:var(--text3);text-align:center}}

  @media print{{
    nav{{display:none}}
    main{{padding:16px;max-width:100%}}
    details.section,details.card{{border:1px solid #ccc}}
    details.section > summary,details.card > summary{{background:#f5f5f5;color:#333}}
    pre.card-data{{color:#111;font-size:10px}}
  }}
</style>
</head>
<body>

<nav>
  <div class="nav-brand">
    <div class="nav-brand-title">NiTriTe</div>
    <div class="nav-brand-sub">Rapport de sauvegarde</div>
  </div>
  <div class="nav-actions">
    <button class="nav-btn" onclick="document.querySelectorAll('details').forEach(d=>d.open=true)">Tout ouvrir</button>
    <button class="nav-btn" onclick="document.querySelectorAll('details').forEach(d=>d.open=false)">Tout fermer</button>
  </div>
  <div class="nav-label">Catégories</div>
  {nav_links}
</nav>

<main>
  <div class="report-header">
    <div class="report-title">Rapport de Sauvegarde</div>
    <div class="report-meta">
      <div class="meta-pill">📅 Date&nbsp;<strong>{ts}</strong></div>
      <div class="meta-pill">📦 Éléments&nbsp;<strong>{total}</strong></div>
      <div class="meta-pill">🛡 NiTriTe&nbsp;<strong>v26.36</strong></div>
    </div>
  </div>

  {sections}

  <div class="report-footer">
    Généré par NiTriTe &mdash; {ts} &mdash; {total} éléments exportés
  </div>
</main>

</body></html>"#,
        ts        = html_escape(&m.timestamp),
        total     = m.total_items,
        nav_links = nav_links,
        sections  = sections,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[derive(Debug, Clone, Serialize)]
pub struct BackupEntryInfo {
    pub filename: String,
    pub date: String,
    pub size: String,
    pub items_count: usize,
}

pub fn list_backups() -> Result<Vec<BackupEntryInfo>, NiTriTeError> {
    let backup_dir = paths::backups_dir();
    let mut backups = Vec::new();

    if backup_dir.exists() {
        for entry in std::fs::read_dir(&backup_dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            let is_backup = name.starts_with("backup_")
                && (name.ends_with(".json") || name.ends_with(".txt")
                    || name.ends_with(".html") || name.ends_with(".md"));
            if !is_backup { continue; }

            let meta = entry.metadata().unwrap_or_else(|_| {
                std::fs::metadata(entry.path()).unwrap()
            });
            let size_bytes = meta.len();
            let size = if size_bytes < 1_048_576 {
                format!("{:.1} Ko", size_bytes as f64 / 1024.0)
            } else {
                format!("{:.1} Mo", size_bytes as f64 / 1_048_576.0)
            };

            // Parse date from filename: backup_2026-03-09_14-30-00.txt
            let stem = name.strip_prefix("backup_").unwrap_or(&name);
            let stem = stem.split('.').next().unwrap_or(stem);
            let date = stem.replace('_', " ").replace('-', "/");

            backups.push(BackupEntryInfo { filename: name, date, size, items_count: 0 });
        }
    }

    backups.sort_by(|a, b| b.filename.cmp(&a.filename));
    Ok(backups)
}

fn collect_installed_apps() -> Result<String, NiTriTeError> {
    let output = Command::new("winget").args(["list", "--accept-source-agreements"]).creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_drivers() -> Result<String, NiTriTeError> {
    let script = r#"
$cats = @{
    'Bluetooth'    = 'Bluetooth'
    'Net'          = 'Cartes Reseau'
    'Display'      = 'Affichage / GPU'
    'DiskDrive'    = 'Disques / SSD'
    'USB'          = 'Controleurs USB'
    'AudioEndpoint'= 'Audio'
    'Media'        = 'Multimedia'
    'MEDIA'        = 'Multimedia'
    'HDC'          = 'Controleurs disque'
    'SCSIAdapter'  = 'Controleurs SCSI / NVMe'
    'Mouse'        = 'Souris'
    'Keyboard'     = 'Clavier'
    'HIDClass'     = 'Peripheriques HID'
    'Camera'       = 'Cameras'
    'System'       = 'Systeme'
    'Processor'    = 'Processeur'
    'Battery'      = 'Batterie'
    'Monitor'      = 'Ecrans'
    'PrintQueue'   = 'Imprimantes'
    'Biometric'    = 'Biometrique'
    'UCM'          = 'USB Type-C'
}
$all = Get-WmiObject Win32_PnPSignedDriver -ErrorAction SilentlyContinue |
    Where-Object { $_.DeviceName -and $_.DeviceName.Trim() -ne '' -and $_.DriverVersion } |
    Group-Object DeviceClass | Sort-Object Name

foreach ($group in $all) {
    $cat = if ($cats[$group.Name]) { $cats[$group.Name] }
           elseif ($group.Name)    { $group.Name }
           else                    { 'Autres' }
    Write-Output ""
    Write-Output "=== $cat ($($group.Count) pilote(s)) ==="
    foreach ($drv in ($group.Group | Sort-Object DeviceName)) {
        Write-Output "  Peripherique : $($drv.DeviceName)"
        if ($drv.Manufacturer -and $drv.Manufacturer -ne $drv.DeviceName) {
            Write-Output "  Fabricant    : $($drv.Manufacturer)"
        }
        Write-Output "  Version      : $($drv.DriverVersion)"
        if ($drv.DriverDate -and $drv.DriverDate.Length -ge 8) {
            try {
                $df = [datetime]::ParseExact($drv.DriverDate.Substring(0,8),'yyyyMMdd',$null)
                Write-Output "  Date         : $($df.ToString('dd/MM/yyyy'))"
            } catch {}
        }
        Write-Output ""
    }
}
"#;
    run_ps_temp(script)
}


fn collect_network_config() -> Result<String, NiTriTeError> {
    let output = Command::new("ipconfig").arg("/all").creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_startup() -> Result<String, NiTriTeError> {
    let script = r#"
$items = Get-CimInstance Win32_StartupCommand -ErrorAction SilentlyContinue |
    Sort-Object Location, Name

$prev_loc = ""
foreach ($item in $items) {
    if ($item.Location -ne $prev_loc) {
        Write-Output ""
        Write-Output "=== $($item.Location) ==="
        $prev_loc = $item.Location
    }
    Write-Output "  Nom         : $($item.Name)"
    Write-Output "  Commande    : $($item.Command)"
    Write-Output "  Utilisateur : $($item.User)"
    Write-Output ""
}
"#;
    run_ps_temp(script)
}

fn collect_env_vars() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "[Environment]::GetEnvironmentVariables('Machine') | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_firewall_rules() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-NetFirewallRule | Select-Object -First 50 DisplayName, Direction, Action, Enabled | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_browser_bookmarks(browser_subpath: &str) -> Result<String, NiTriTeError> {
    let local_app = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let bookmarks_path = std::path::PathBuf::from(&local_app)
        .join(browser_subpath)
        .join("User Data").join("Default").join("Bookmarks");

    if bookmarks_path.exists() {
        Ok(std::fs::read_to_string(bookmarks_path)?)
    } else {
        Err(NiTriTeError::System("Fichier bookmarks introuvable".into()))
    }
}

fn collect_windows_license() -> Result<String, NiTriTeError> {
    // Méthode 1 : clé OEM gravée dans le BIOS/UEFI (laptops OEM)
    let oem_out = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "$k=(Get-WmiObject SoftwareLicensingService -EA SilentlyContinue).OA3xOriginalProductKey; if($k -and $k.Trim().Length -gt 0){$k.Trim()} else {''}"])
        .creation_flags(0x08000000).output()?;
    let oem_key = String::from_utf8_lossy(&oem_out.stdout).trim().to_string();

    // Méthode 2 : décode DigitalProductId depuis le registre (fonctionne retail, volume, KMS/Massgrave)
    let decode_script = concat!(
        "$m='BCDFGHJKMPQRTVWXY2346789';",
        "$d=Get-ItemPropertyValue 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion' -Name DigitalProductId -EA SilentlyContinue;",
        "if($d -and $d.Count -ge 67){",
          "$k=$d[52..66]; $r='';",
          "for($i=24;$i -ge 0;$i--){",
            "$n=0;",
            "for($j=14;$j -ge 0;$j--){$n=$n*256 -bxor $k[$j];$k[$j]=[math]::Floor($n/24);$n=$n%24};",
            "$r=$m[$n]+$r;",
            "if((24-$i)%5 -eq 0 -and $i -ne 0 -and $i -ne 24){$r='-'+$r}",
          "};",
          "$r",
        "}else{''}"
    );
    let reg_out = Command::new("powershell")
        .args(["-NoProfile", "-Command", decode_script])
        .creation_flags(0x08000000).output()?;
    let reg_key = String::from_utf8_lossy(&reg_out.stdout).trim().to_string();

    // Méthode 3 : slmgr /dli — statut lisible (fonctionne avec Massgrave HWID/KMS38)
    let slmgr_out = Command::new("cscript")
        .args(["//nologo", "C:\\Windows\\System32\\slmgr.vbs", "/dli"])
        .creation_flags(0x08000000).output();
    let slmgr = if let Ok(o) = slmgr_out {
        String::from_utf8_lossy(&o.stdout).trim().to_string()
    } else { String::new() };

    // Méthode 4 : informations WMI (canal, clé partielle)
    let wmi_out = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-WmiObject SoftwareLicensingProduct -EA SilentlyContinue | Where-Object {$_.PartialProductKey -and $_.Name -like '*Windows*'} | Select-Object -First 1 | ForEach-Object { \"Edition         : \" + $_.Name; \"Cle partielle   : ...\" + $_.PartialProductKey; \"Canal           : \" + $_.LicenseFamily; \"Statut          : \" + $(switch($_.LicenseStatus){0{'Non licence'} 1{'ACTIVE'} 2{'Grace period'} 3{'Modifie (tampered)'} 4{'Notification'} 5{'Grace etendue'} default{'Inconnu'}}) }"])
        .creation_flags(0x08000000).output()?;
    let wmi = String::from_utf8_lossy(&wmi_out.stdout).trim().to_string();

    let mut r = String::new();
    r.push_str("╔══════════════════════════════════════╗\n");
    r.push_str("║     LICENCE WINDOWS — INFORMATIONS   ║\n");
    r.push_str("╚══════════════════════════════════════╝\n\n");

    if !oem_key.is_empty() {
        r.push_str(&format!("Cle OEM (BIOS)  : {}\n", oem_key));
    }
    if !reg_key.is_empty() && reg_key != "DigitalProductId introuvable" {
        r.push_str(&format!("Cle registre    : {}\n", reg_key));
    }
    if oem_key.is_empty() && (reg_key.is_empty() || reg_key == "DigitalProductId introuvable") {
        r.push_str("Cle produit     : Non disponible (activation par hardware ID ou KMS)\n");
    }
    if !wmi.is_empty() {
        r.push('\n');
        r.push_str(&wmi);
        r.push('\n');
    }
    if !slmgr.is_empty() {
        r.push_str("\n─── Détail complet (slmgr) ───\n");
        r.push_str(&slmgr);
    }
    Ok(r)
}

fn collect_bitlocker_keys() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-BitLockerVolume -ErrorAction SilentlyContinue | Select-Object MountPoint, VolumeStatus, EncryptionPercentage, KeyProtector | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_office_license() -> Result<String, NiTriTeError> {
    // Méthode 1 : DigitalProductId dans les chemins MSI classiques
    let decode_script = concat!(
        "$m='BCDFGHJKMPQRTVWXY2346789';",
        "$paths=@('HKLM:\\SOFTWARE\\Microsoft\\Office\\*\\Registration','HKLM:\\SOFTWARE\\WOW6432Node\\Microsoft\\Office\\*\\Registration');",
        "$found=$false;",
        "foreach($p in $paths){",
          "Get-ChildItem $p -EA SilentlyContinue | ForEach-Object {",
            "$reg=Get-ItemProperty $_.PSPath -EA SilentlyContinue;",
            "$name=if($reg.ProductName){$reg.ProductName}else{'Microsoft Office'};",
            "$dpid=$reg.DigitalProductID;",
            "if($dpid -and $dpid.Count -ge 67){",
              "$k=$dpid[52..66]; $r='';",
              "for($i=24;$i -ge 0;$i--){",
                "$n=0;",
                "for($j=14;$j -ge 0;$j--){$n=$n*256 -bxor $k[$j];$k[$j]=[math]::Floor($n/24);$n=$n%24};",
                "$r=$m[$n]+$r;",
                "if((24-$i)%5 -eq 0 -and $i -ne 0 -and $i -ne 24){$r='-'+$r}",
              "};",
              "\"Produit  : $name\";\"Cle      : $r\";'';$found=$true",
            "}",
          "}",
        "};",
        "if(-not $found){'(Aucune cle MSI trouvee — voir section Click-to-Run ci-dessous)'}"
    );
    let reg_out = Command::new("powershell")
        .args(["-NoProfile", "-Command", decode_script])
        .creation_flags(0x08000000).output()?;
    let reg_keys = String::from_utf8_lossy(&reg_out.stdout).trim().to_string();

    // Méthode 2 : Click-to-Run / Office 365 / Massgrave
    let c2r_out = Command::new("powershell")
        .args(["-NoProfile", "-Command", concat!(
            "$c2r=Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Office\\ClickToRun\\Configuration' -EA SilentlyContinue;",
            "if($c2r){",
              "\"Produit C2R    : \" + $c2r.ProductReleaseIds;",
              "\"Canal          : \" + $c2r.CDNBaseUrl;",
              "\"Version        : \" + $c2r.VersionToReport;",
              "$lic=Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Office\\ClickToRun\\REGISTRY\\MACHINE\\SOFTWARE\\Microsoft\\Office\\16.0\\Common\\Licensing' -EA SilentlyContinue;",
              "if($lic.LastAcknowledgedLicenseToken){'Licence token  : presente (abonnement/KMS actif)'}",
            "}else{'(Office Click-to-Run non detecte)'}"
        )])
        .creation_flags(0x08000000).output()?;
    let c2r = String::from_utf8_lossy(&c2r_out.stdout).trim().to_string();

    // Méthode 3 : OSPP.VBS statut d'activation
    let ospp_dirs = [
        r"C:\Program Files\Microsoft Office\Office16",
        r"C:\Program Files (x86)\Microsoft Office\Office16",
        r"C:\Program Files\Microsoft Office\Office15",
    ];
    let mut ospp_status = String::new();
    for dir in &ospp_dirs {
        let vbs = format!("{}\\OSPP.VBS", dir);
        if std::path::Path::new(&vbs).exists() {
            if let Ok(o) = Command::new("cscript")
                .args(["//nologo", &vbs, "/dstatus"])
                .creation_flags(0x08000000).output()
            {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if !s.is_empty() { ospp_status = s; break; }
            }
        }
    }

    let mut r = String::new();
    r.push_str("╔══════════════════════════════════════╗\n");
    r.push_str("║     LICENCE OFFICE — INFORMATIONS    ║\n");
    r.push_str("╚══════════════════════════════════════╝\n\n");
    r.push_str("─── Clé produit (registre MSI) ───\n");
    r.push_str(&reg_keys);
    r.push_str("\n\n─── Office Click-to-Run / Office 365 ───\n");
    r.push_str(&c2r);
    if !ospp_status.is_empty() {
        r.push_str("\n\n─── Statut d'activation (OSPP) ───\n");
        r.push_str(&ospp_status);
    }
    Ok(r)
}

fn collect_installed_fonts() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts' | Get-Member -MemberType NoteProperty | Select-Object Name).Name | Sort-Object | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_scheduled_tasks() -> Result<String, NiTriTeError> {
    let script = r#"
$tasks = Get-ScheduledTask -ErrorAction SilentlyContinue |
    Where-Object { $_.State -ne 'Disabled' -and $_.TaskPath -notmatch '\\Microsoft\\' } |
    Sort-Object TaskPath, TaskName

$prev_path = ""
foreach ($t in $tasks) {
    $path = if ($t.TaskPath -and $t.TaskPath -ne '\') { $t.TaskPath.TrimEnd('\') } else { "Racine" }
    if ($path -ne $prev_path) {
        Write-Output ""
        Write-Output "=== Dossier : $path ==="
        $prev_path = $path
    }
    $etat = switch ($t.State) {
        'Ready'   { 'Pret' }
        'Running' { 'En cours' }
        'Queued'  { 'En attente' }
        default   { $t.State }
    }
    Write-Output "  Tache       : $($t.TaskName)"
    Write-Output "  Etat        : $etat"
    if ($t.Description -and $t.Description.Trim() -ne '') {
        $desc = $t.Description.Trim() -replace '\r?\n', ' '
        if ($desc.Length -gt 120) { $desc = $desc.Substring(0,120) + '...' }
        Write-Output "  Description : $desc"
    }
    # Déclencheur simplifié
    $trig = $t.Triggers | Select-Object -First 1
    if ($trig) {
        $type = $trig.CimClass.CimClassName -replace 'MSFT_Task','` -replace 'Trigger',''
        Write-Output "  Declencheur : $type"
    }
    Write-Output ""
}
if (-not $tasks) { Write-Output "Aucune tache planifiee active (hors Microsoft) trouvee." }
"#;
    run_ps_temp(script)
}

fn collect_windows_features() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-WindowsOptionalFeature -Online -ErrorAction SilentlyContinue | Where-Object {$_.State -eq 'Enabled'} | Select-Object FeatureName | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_folder_sizes() -> Result<String, NiTriTeError> {
    // Sortie directement formatée en Mo/Go — pas de JSON brut
    let script = concat!(
        "Get-ChildItem C:\\ -Directory -EA SilentlyContinue",
        " | ForEach-Object {",
        "   $bytes = (Get-ChildItem $_.FullName -Recurse -EA SilentlyContinue | Measure-Object Length -Sum).Sum;",
        "   $bytes = if($bytes){[long]$bytes}else{0};",
        "   $taille = if($bytes -ge 1073741824){ \"{0:N2} Go\" -f ($bytes/1GB) }",
        "             elseif($bytes -ge 1048576){ \"{0:N0} Mo\" -f ($bytes/1MB) }",
        "             elseif($bytes -ge 1024){ \"{0:N0} Ko\" -f ($bytes/1KB) }",
        "             else{ \"$bytes o\" };",
        "   [PSCustomObject]@{Dossier=$_.Name; Taille=$taille; TailleBytes=$bytes}",
        " }",
        " | Sort-Object TailleBytes -Descending",
        " | Select-Object -First 30",
        " | ForEach-Object { \"{0,-40} {1}\" -f $_.Dossier, $_.Taille }"
    );
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_desktop_files() -> Result<String, NiTriTeError> {
    let desktop = std::env::var("USERPROFILE").unwrap_or_default();
    let desktop_path = std::path::PathBuf::from(&desktop).join("Desktop");
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&desktop_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            files.push(format!("{} ({})", name, format_size(size)));
        }
    }
    Ok(files.join("\n"))
}

fn collect_system_components() -> Result<String, NiTriTeError> {
    let script = r#"
# === SYSTEME ===
$cs = Get-WmiObject Win32_ComputerSystem -EA SilentlyContinue
$os = Get-WmiObject Win32_OperatingSystem -EA SilentlyContinue
Write-Output "=== INFORMATIONS SYSTEME ==="
if ($cs) {
    Write-Output "  Fabricant PC    : $($cs.Manufacturer)"
    Write-Output "  Modele PC       : $($cs.Model)"
    Write-Output "  Nom machine     : $($cs.Name)"
}
if ($os) {
    Write-Output "  Systeme         : $($os.Caption) $($os.OSArchitecture)"
    Write-Output "  Build           : $($os.BuildNumber)"
}
Write-Output ""

# === CPU ===
$cpu = Get-WmiObject Win32_Processor -EA SilentlyContinue | Select-Object -First 1
Write-Output "=== PROCESSEUR (CPU) ==="
if ($cpu) {
    Write-Output "  Modele          : $($cpu.Name.Trim())"
    Write-Output "  Coeurs          : $($cpu.NumberOfCores)"
    Write-Output "  Threads         : $($cpu.NumberOfLogicalProcessors)"
    $ghz = [math]::Round($cpu.MaxClockSpeed / 1000.0, 2)
    Write-Output "  Frequence       : $($cpu.MaxClockSpeed) MHz ($ghz GHz)"
    if ($cpu.L2CacheSize -gt 0) {
        $l2 = [math]::Round($cpu.L2CacheSize / 1024.0, 1)
        Write-Output "  Cache L2        : $l2 Mo"
    }
    if ($cpu.L3CacheSize -gt 0) {
        $l3 = [math]::Round($cpu.L3CacheSize / 1024.0, 1)
        Write-Output "  Cache L3        : $l3 Mo"
    }
    Write-Output "  Socket          : $($cpu.SocketDesignation)"
    $arch = switch ($cpu.Architecture) { 0 { 'x86' } 9 { 'x64 (AMD64)' } 12 { 'ARM64' } default { $cpu.Architecture } }
    Write-Output "  Architecture    : $arch"
}
Write-Output ""

# === GPU ===
Write-Output "=== CARTE(S) GRAPHIQUE(S) ==="
$gpus = Get-WmiObject Win32_VideoController -EA SilentlyContinue
foreach ($gpu in $gpus) {
    Write-Output "  Modele          : $($gpu.Name)"
    if ($gpu.AdapterRAM -and $gpu.AdapterRAM -gt 0) {
        $mb = [math]::Round($gpu.AdapterRAM / 1MB)
        $vram = if ($mb -ge 1024) { "$([math]::Round($mb/1024.0,1)) Go" } else { "$mb Mo" }
        Write-Output "  VRAM            : $vram"
    }
    Write-Output "  Version pilote  : $($gpu.DriverVersion)"
    if ($gpu.CurrentHorizontalResolution) {
        Write-Output "  Resolution      : $($gpu.CurrentHorizontalResolution) x $($gpu.CurrentVerticalResolution)"
    }
    Write-Output ""
}

# === RAM ===
Write-Output "=== MEMOIRE RAM ==="
$ramList = Get-WmiObject Win32_PhysicalMemory -EA SilentlyContinue
$ramTotal = ($ramList | Measure-Object Capacity -Sum).Sum
if ($ramTotal) {
    Write-Output "  Total           : $([math]::Round($ramTotal/1GB,1)) Go"
}
foreach ($stick in $ramList) {
    $slot = if ($stick.DeviceLocator) { $stick.DeviceLocator } else { 'Slot inconnu' }
    $cap  = [math]::Round($stick.Capacity / 1GB, 1)
    $type = switch ($stick.SMBIOSMemoryType) {
        26 { 'DDR4' } 34 { 'DDR5' } 24 { 'DDR3' } 21 { 'DDR2' } 20 { 'DDR' } default { 'RAM' }
    }
    $speed = if ($stick.ConfiguredClockSpeed -gt 0) { $stick.ConfiguredClockSpeed }
             elseif ($stick.Speed -gt 0) { $stick.Speed } else { 0 }
    $speedStr = if ($speed -gt 0) { " @ $speed MHz" } else { '' }
    Write-Output "  $slot          : $cap Go $type$speedStr"
}
Write-Output ""

# === STOCKAGE ===
Write-Output "=== STOCKAGE (SSD / HDD) ==="
# Essai Get-PhysicalDisk (Storage module), repli sur Win32_DiskDrive
$disks = $null
try { $disks = Get-PhysicalDisk -ErrorAction Stop } catch {}
if ($disks) {
    foreach ($d in ($disks | Sort-Object DeviceId)) {
        $type = switch ($d.MediaType) {
            'SSD' { 'SSD' } 'HDD' { 'Disque dur (HDD)' } 'SCM' { 'Storage Class Memory' } default { $d.MediaType }
        }
        $size = if ($d.Size -ge 1GB) { "$([math]::Round($d.Size/1GB,1)) Go" } else { "$([math]::Round($d.Size/1MB,0)) Mo" }
        Write-Output "  $($d.FriendlyName)"
        Write-Output "    Type            : $type"
        Write-Output "    Capacite        : $size"
        Write-Output "    Sante           : $($d.HealthStatus)"
        Write-Output ""
    }
} else {
    foreach ($d in (Get-WmiObject Win32_DiskDrive -EA SilentlyContinue | Sort-Object Index)) {
        $size = if ($d.Size -ge 1GB) { "$([math]::Round($d.Size/1GB,1)) Go" } else { "$([math]::Round($d.Size/1MB,0)) Mo" }
        Write-Output "  $($d.Model)"
        Write-Output "    Interface       : $($d.InterfaceType)"
        Write-Output "    Capacite        : $size"
        Write-Output ""
    }
}

# === CARTE MERE ===
Write-Output "=== CARTE MERE ==="
$mb = Get-WmiObject Win32_BaseBoard -EA SilentlyContinue
if ($mb) {
    Write-Output "  Fabricant       : $($mb.Manufacturer)"
    Write-Output "  Modele          : $($mb.Product)"
    $sn = if ($mb.SerialNumber -and $mb.SerialNumber -notmatch 'Default|None|To Be|N/A|^\s*$') { $mb.SerialNumber } else { 'Non disponible' }
    Write-Output "  Numero de serie : $sn"
}
Write-Output ""

# === BIOS ===
Write-Output "=== BIOS / UEFI ==="
$bios = Get-WmiObject Win32_BIOS -EA SilentlyContinue
if ($bios) {
    Write-Output "  Fabricant       : $($bios.Manufacturer)"
    Write-Output "  Version         : $($bios.SMBIOSBIOSVersion)"
    if ($bios.ReleaseDate -and $bios.ReleaseDate.Length -ge 8) {
        try {
            $bd = [datetime]::ParseExact($bios.ReleaseDate.Substring(0,8), 'yyyyMMdd', $null)
            Write-Output "  Date            : $($bd.ToString('dd/MM/yyyy'))"
        } catch {}
    }
}
"#;
    run_ps_temp(script)
}

fn collect_wifi_passwords() -> Result<String, NiTriTeError> {
    let script = r#"
$raw = netsh wlan show profiles 2>$null
if (-not $raw) {
    Write-Output "Wi-Fi desactive ou aucun profil sauvegarde."
    return
}
$profiles = @()
foreach ($line in $raw) {
    if ($line -match 'All User Profile\s*:\s*(.+)' -or $line -match 'User Profile\s*:\s*(.+)') {
        $profiles += $Matches[1].Trim()
    }
}
if ($profiles.Count -eq 0) {
    Write-Output "Aucun profil WiFi sauvegarde."
    return
}
foreach ($name in $profiles) {
    $detail = netsh wlan show profile name="$name" key=clear 2>$null
    $passLine = $detail | Where-Object { $_ -match 'Key Content\s*:\s*(.+)' }
    Write-Output "Reseau       : $name"
    if ($passLine) {
        $pw = ($passLine -replace '.*Key Content\s*:\s*', '').Trim()
        Write-Output "Mot de passe : $pw"
    } else {
        Write-Output "Mot de passe : (aucun / reseau ouvert ou securise autrement)"
    }
    # Infos supplementaires
    $auth = $detail | Where-Object { $_ -match 'Authentication\s*:\s*(.+)' }
    if ($auth) {
        $a = ($auth -replace '.*Authentication\s*:\s*', '').Trim()
        Write-Output "Securite     : $a"
    }
    Write-Output ""
}
"#;
    run_ps_temp(script)
}

fn collect_registry_export() -> Result<String, NiTriTeError> {
    // Export partiel du registre HKCU (Software uniquement, taille limitee)
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-ItemProperty 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -ErrorAction SilentlyContinue | ConvertTo-Json; Get-ItemProperty 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders' -ErrorAction SilentlyContinue | ConvertTo-Json"])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_suspicious_processes() -> Result<String, NiTriTeError> {
    // Sortie formatée directement (plus besoin de json_to_readable pour ce collecteur)
    let script = concat!(
        "Get-Process -EA SilentlyContinue",
        " | Where-Object { $_.Path -and $_.Path -notmatch 'Windows|Microsoft|System32|SysWOW64|Program Files' }",
        " | ForEach-Object {",
        "   $mem = $_.WorkingSet64;",
        "   $memStr = if($mem -ge 1073741824){ \"{0:N2} Go\" -f ($mem/1GB) }",
        "             elseif($mem -ge 1048576){ \"{0:N0} Mo\" -f ($mem/1MB) }",
        "             else{ \"{0:N0} Ko\" -f ($mem/1KB) };",
        "   [PSCustomObject]@{Processus=$_.ProcessName; PID=$_.Id; Memoire=$memStr; Chemin=$_.Path}",
        " }",
        " | Sort-Object {$_.Memoire} -Descending",
        " | Select-Object -First 30",
        " | ForEach-Object { \"$($_.Processus) (PID $($_.PID)) — $($_.Memoire)`n  Chemin : $($_.Chemin)\n\" }"
    );
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .creation_flags(0x08000000).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

// Champs PowerShell internes à ignorer
const PS_SKIP: &[&str] = &[
    "PSPath","PSParentPath","PSChildName","PSProvider",
    "PSComputerName","RunspaceId","PSShowComputerName",
];

// Traduction des clés techniques → labels lisibles
fn friendly_label(k: &str) -> &str {
    match k {
        "Name"|"TaskName"|"DisplayName"   => "Nom",
        "ProductName"                      => "Produit",
        "Description"                      => "Description",
        "State"                            => "Etat",
        "Status"                           => "Statut",
        "Path"|"TaskPath"|"FullName"       => "Chemin",
        "Id"|"ProcessId"                   => "PID",
        "ProcessName"                      => "Processus",
        "SizeMB"|"SizeGB"                  => "Taille",
        "Folder"|"Dossier"                 => "Dossier",
        "LicenseStatus"                    => "Statut licence",
        "PartialProductKey"                => "Cle partielle",
        "LicenseFamily"                    => "Canal",
        "MemMB"                            => "Memoire",
        "Username"|"UserName"              => "Utilisateur",
        "Version"|"DisplayVersion"         => "Version",
        "Publisher"|"InstallLocation"      => "Editeur",
        "InstallDate"                      => "Date installation",
        "FeatureName"                      => "Fonctionnalite",
        "MountPoint"                       => "Lecteur",
        "VolumeStatus"                     => "Statut volume",
        "EncryptionPercentage"             => "Chiffrement (%)",
        "KeyProtector"                     => "Protecteur cle",
        "MemoryMB"                         => "Memoire",
        other                              => other,
    }
}

// Formate une valeur numérique selon le nom de la clé (Mo, Go, %)
fn fmt_numeric(k: &str, n: &serde_json::Number) -> Option<String> {
    let kl = k.to_lowercase();
    let f = n.as_f64()?;
    if kl.ends_with("mb") || kl == "sizemb" || kl == "memmb" || kl == "memorymb" {
        return Some(if f >= 1024.0 {
            format!("{:.2} Go", f / 1024.0)
        } else {
            format!("{:.0} Mo", f)
        });
    }
    if kl.ends_with("gb") || kl == "sizegb" {
        return Some(format!("{:.2} Go", f));
    }
    if kl.contains("bytes") || (kl.contains("size") && !kl.contains("mb") && !kl.contains("gb")) {
        if let Some(u) = n.as_u64() { return Some(format_size(u)); }
    }
    if kl.contains("percent") { return Some(format!("{}%", f)); }
    None
}

/// Convertit du JSON en texte lisible pour un non-informaticien
fn json_to_readable(s: &str) -> String {
    let trimmed = s.trim();
    if !trimmed.starts_with('{') && !trimmed.starts_with('[') {
        return s.to_string();
    }
    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(v)  => fmt_val(&v, 0),
        Err(_) => s.to_string(),
    }
}

fn fmt_val(v: &serde_json::Value, depth: usize) -> String {
    let pad = "  ".repeat(depth);
    match v {
        serde_json::Value::Array(arr) => {
            arr.iter().enumerate().map(|(i, item)| {
                match item {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        format!("{}── Entrée {} ──\n{}", pad, i + 1, fmt_val(item, depth))
                    }
                    _ => fmt_val(item, depth),
                }
            }).collect::<Vec<_>>().join("\n\n")
        }
        serde_json::Value::Object(map) => {
            map.iter()
                .filter(|(k, v)| !v.is_null() && !PS_SKIP.contains(&k.as_str()))
                .map(|(k, v)| {
                    let label = friendly_label(k);
                    let val = match v {
                        serde_json::Value::String(s) if s.is_empty() => return String::new(),
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Bool(b) => if *b { "Oui".into() } else { "Non".into() },
                        serde_json::Value::Number(n) => {
                            fmt_numeric(k, n).unwrap_or_else(|| n.to_string())
                        }
                        other => fmt_val(other, depth + 1),
                    };
                    if val.is_empty() { return String::new(); }
                    format!("{}  {:<26} {}", pad, format!("{}:", label), val)
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("\n")
        }
        serde_json::Value::String(s) => format!("{}{}", pad, s),
        serde_json::Value::Null       => String::new(),
        _                             => format!("{}{}", pad, v),
    }
}

fn collect_winget_export() -> Result<String, NiTriTeError> {
    let tmp = std::env::temp_dir().join("nitrite_winget_export.json");
    let _ = Command::new("winget")
        .args(["export", "-o", tmp.to_str().unwrap_or(""), "--accept-source-agreements"])
        .creation_flags(0x08000000)
        .output();
    if tmp.exists() {
        let content = std::fs::read_to_string(&tmp)?;
        let _ = std::fs::remove_file(&tmp);
        Ok(content)
    } else {
        Err(NiTriTeError::System("winget export échoué".into()))
    }
}

fn collect_network_shares() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-SmbShare | Select-Object Name, Path, Description | Format-Table -AutoSize | Out-String"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_hosts_file() -> Result<String, NiTriTeError> {
    let hosts = std::path::Path::new(r"C:\Windows\System32\drivers\etc\hosts");
    if hosts.exists() {
        Ok(std::fs::read_to_string(hosts)?)
    } else {
        Err(NiTriTeError::System("Fichier hosts introuvable".into()))
    }
}

fn collect_ssh_keys() -> Result<String, NiTriTeError> {
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    let ssh_dir = std::path::PathBuf::from(&home).join(".ssh");
    let mut lines = vec![format!("Dossier : {}", ssh_dir.display())];
    if ssh_dir.exists() {
        for entry in std::fs::read_dir(&ssh_dir).into_iter().flatten().flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            lines.push(format!("  {} ({})", name, format_size(size)));
        }
    } else {
        lines.push("Dossier .ssh introuvable".into());
    }
    Ok(lines.join("\n"))
}

fn collect_pip_packages() -> Result<String, NiTriTeError> {
    let output = Command::new("pip")
        .args(["freeze"])
        .creation_flags(0x08000000)
        .output()
        .or_else(|_| Command::new("pip3").args(["freeze"]).creation_flags(0x08000000).output())?;
    let s = String::from_utf8_lossy(&output.stdout).to_string();
    if s.trim().is_empty() {
        Err(NiTriTeError::System("pip non disponible ou aucun package installé".into()))
    } else {
        Ok(s)
    }
}

fn collect_vscode_extensions() -> Result<String, NiTriTeError> {
    let output = Command::new("code")
        .args(["--list-extensions"])
        .creation_flags(0x08000000)
        .output()?;
    let s = String::from_utf8_lossy(&output.stdout).to_string();
    if s.trim().is_empty() {
        Err(NiTriTeError::System("VSCode non disponible ou aucune extension".into()))
    } else {
        Ok(s)
    }
}

fn collect_wsl_config() -> Result<String, NiTriTeError> {
    let output = Command::new("wsl")
        .args(["--list", "--verbose"])
        .creation_flags(0x08000000)
        .output()?;
    let mut content = String::from_utf8_lossy(&output.stdout).to_string();
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    let wslconfig = std::path::PathBuf::from(&home).join(".wslconfig");
    if wslconfig.exists() {
        if let Ok(cfg) = std::fs::read_to_string(&wslconfig) {
            content.push_str("\n\n--- .wslconfig ---\n");
            content.push_str(&cfg);
        }
    }
    Ok(content)
}

fn collect_powershell_profile() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "if (Test-Path $PROFILE) { \"=== \" + $PROFILE + \" ===\"; Get-Content $PROFILE } else { 'Profil inexistant : ' + $PROFILE }"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_power_plans() -> Result<String, NiTriTeError> {
    let output = Command::new("powercfg")
        .args(["/list"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_printer_config() -> Result<String, NiTriTeError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command",
            "Get-Printer | Select-Object Name, PortName, DriverName, PrinterStatus | Format-Table -AutoSize | Out-String"])
        .creation_flags(0x08000000)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
