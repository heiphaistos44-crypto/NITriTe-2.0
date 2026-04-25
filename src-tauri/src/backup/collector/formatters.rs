//! Utilitaires de formatage pour les données de sauvegarde

pub fn format_size(bytes: u64) -> String {
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
pub fn friendly_label(k: &str) -> &str {
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
pub fn fmt_numeric(k: &str, n: &serde_json::Number) -> Option<String> {
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
pub fn json_to_readable(s: &str) -> String {
    let trimmed = s.trim();
    if !trimmed.starts_with('{') && !trimmed.starts_with('[') {
        return s.to_string();
    }
    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(v)  => fmt_val(&v, 0),
        Err(_) => s.to_string(),
    }
}

pub fn fmt_val(v: &serde_json::Value, depth: usize) -> String {
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

