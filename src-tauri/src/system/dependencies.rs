use serde::Serialize;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize)]
pub struct Dependency {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub installed: bool,
    pub version: String,
    pub winget_id: String,
    pub install_url: String,
}

fn run_cmd(cmd: &str, args: &[&str]) -> (bool, String) {
    let mut command = std::process::Command::new(cmd);
    command.args(args);
    #[cfg(target_os = "windows")]
    command.creation_flags(0x08000000);
    match command.output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let combined = format!("{}{}", stdout, stderr);
            let first_line = combined.lines().next().unwrap_or("").trim().to_string();
            (out.status.success() || !stdout.trim().is_empty(), first_line)
        }
        Err(_) => (false, String::new()),
    }
}

fn check(cmd: &str, version_arg: &str) -> (bool, String) {
    run_cmd(cmd, &[version_arg])
}

fn check_ps(ps_expr: &str) -> (bool, String) {
    let mut command = std::process::Command::new("powershell");
    command.args(["-NoProfile", "-NonInteractive", "-Command", ps_expr]);
    #[cfg(target_os = "windows")]
    command.creation_flags(0x08000000);
    match command.output() {
        Ok(out) => {
            let out_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            (!out_str.is_empty() && out.status.success(), out_str)
        }
        Err(_) => (false, String::new()),
    }
}

pub fn check_all() -> Vec<Dependency> {
    vec![
        // === Outils Dev ===
        {
            let (ok, v) = check("git", "--version");
            Dependency { id: "git".into(), name: "Git".into(), category: "Développement".into(),
                description: "Gestionnaire de versions distribué".into(),
                installed: ok, version: v, winget_id: "Git.Git".into(),
                install_url: "https://git-scm.com/download/win".into() }
        },
        {
            let (ok, v) = check("node", "--version");
            Dependency { id: "nodejs".into(), name: "Node.js".into(), category: "Développement".into(),
                description: "Runtime JavaScript (LTS)".into(),
                installed: ok, version: v, winget_id: "OpenJS.NodeJS.LTS".into(),
                install_url: "https://nodejs.org".into() }
        },
        {
            let (ok, v) = check("python", "--version");
            Dependency { id: "python".into(), name: "Python".into(), category: "Développement".into(),
                description: "Langage de script Python 3".into(),
                installed: ok, version: v, winget_id: "Python.Python.3.12".into(),
                install_url: "https://python.org".into() }
        },
        {
            let (ok, v) = check("rustc", "--version");
            Dependency { id: "rust".into(), name: "Rust (rustc)".into(), category: "Développement".into(),
                description: "Compilateur Rust via rustup".into(),
                installed: ok, version: v, winget_id: "Rustlang.Rustup".into(),
                install_url: "https://rustup.rs".into() }
        },
        {
            let (ok, v) = check("code", "--version");
            Dependency { id: "vscode".into(), name: "Visual Studio Code".into(), category: "Développement".into(),
                description: "Éditeur de code Microsoft".into(),
                installed: ok, version: v.lines().next().unwrap_or("").to_string(),
                winget_id: "Microsoft.VisualStudioCode".into(),
                install_url: "https://code.visualstudio.com".into() }
        },
        {
            let (ok, v) = check("dotnet", "--version");
            Dependency { id: "dotnet".into(), name: ".NET Runtime".into(), category: "Développement".into(),
                description: "Runtime .NET Microsoft".into(),
                installed: ok, version: v, winget_id: "Microsoft.DotNet.Runtime.8".into(),
                install_url: "https://dot.net".into() }
        },
        // === Gestionnaires de paquets ===
        {
            let (ok, v) = check_ps("if (Get-Command winget -EA SilentlyContinue) { (winget --version) } else { exit 1 }");
            Dependency { id: "winget".into(), name: "WinGet".into(), category: "Gestionnaires".into(),
                description: "Gestionnaire de paquets Windows officiel".into(),
                installed: ok, version: v, winget_id: String::new(),
                install_url: "ms-windows-store://pdp/?productid=9NBLGGH4NNS1".into() }
        },
        {
            let (ok, v) = check_ps("if (Get-Command scoop -EA SilentlyContinue) { scoop --version 2>&1 | Select-Object -First 1 } else { exit 1 }");
            Dependency { id: "scoop".into(), name: "Scoop".into(), category: "Gestionnaires".into(),
                description: "Gestionnaire de paquets sans admin".into(),
                installed: ok, version: v, winget_id: String::new(),
                install_url: "https://scoop.sh".into() }
        },
        {
            let (ok, v) = check_ps("if (Get-Command choco -EA SilentlyContinue) { choco --version } else { exit 1 }");
            Dependency { id: "chocolatey".into(), name: "Chocolatey".into(), category: "Gestionnaires".into(),
                description: "Gestionnaire de paquets Windows".into(),
                installed: ok, version: v, winget_id: String::new(),
                install_url: "https://chocolatey.org/install".into() }
        },
        // === Outils Système ===
        {
            let (ok, v) = check("curl", "--version");
            let short = v.lines().next().unwrap_or("").to_string();
            Dependency { id: "curl".into(), name: "curl".into(), category: "Outils Système".into(),
                description: "Outil de transfert HTTP/S en ligne de commande".into(),
                installed: ok, version: short, winget_id: "curl.curl".into(),
                install_url: "https://curl.se".into() }
        },
        {
            let (ok, v) = check("7z", "i");
            let short = v.lines().next().unwrap_or("").to_string();
            Dependency { id: "7zip".into(), name: "7-Zip".into(), category: "Outils Système".into(),
                description: "Archiveur haute compression".into(),
                installed: ok, version: short, winget_id: "7zip.7zip".into(),
                install_url: "https://7-zip.org".into() }
        },
        {
            let (ok, _) = check_ps("if (Test-Path 'C:\\Program Files\\Notepad++\\notepad++.exe') { (& 'C:\\Program Files\\Notepad++\\notepad++.exe' -version 2>&1) } else { exit 1 }");
            Dependency { id: "notepadpp".into(), name: "Notepad++".into(), category: "Outils Système".into(),
                description: "Éditeur de texte avancé".into(),
                installed: ok, version: String::new(), winget_id: "Notepad++.Notepad++".into(),
                install_url: "https://notepad-plus-plus.org".into() }
        },
        {
            let (ok, _) = check_ps("if (Get-Command ssh -EA SilentlyContinue) { 'present' } else { exit 1 }");
            Dependency { id: "openssh".into(), name: "OpenSSH Client".into(), category: "Outils Système".into(),
                description: "Client SSH intégré Windows".into(),
                installed: ok, version: String::new(), winget_id: "Microsoft.OpenSSH.Beta".into(),
                install_url: String::new().into() }
        },
        // === Médias & Productivité ===
        {
            let (ok, _) = check_ps("if (Test-Path 'C:\\Program Files\\VideoLAN\\VLC\\vlc.exe') { 'present' } else { exit 1 }");
            Dependency { id: "vlc".into(), name: "VLC Media Player".into(), category: "Médias".into(),
                description: "Lecteur multimédia universel".into(),
                installed: ok, version: String::new(), winget_id: "VideoLAN.VLC".into(),
                install_url: "https://www.videolan.org".into() }
        },
        {
            let (ok, _) = check_ps("if (Get-ItemProperty HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Where-Object { $_.DisplayName -like '*PowerToys*' }) { 'present' } else { exit 1 }");
            Dependency { id: "powertoys".into(), name: "PowerToys".into(), category: "Productivité".into(),
                description: "Suite d'utilitaires Microsoft pour power users".into(),
                installed: ok, version: String::new(), winget_id: "Microsoft.PowerToys".into(),
                install_url: String::new().into() }
        },
        // === Runtimes ===
        {
            let (ok, v) = check_ps("if (Get-Command java -EA SilentlyContinue) { java -version 2>&1 | Select-Object -First 1 } else { exit 1 }");
            Dependency { id: "java".into(), name: "Java (JRE)".into(), category: "Runtimes".into(),
                description: "Java Runtime Environment".into(),
                installed: ok, version: v, winget_id: "Oracle.JavaRuntimeEnvironment".into(),
                install_url: "https://java.com".into() }
        },
        {
            let (ok, v) = check_ps("if (Get-Command go -EA SilentlyContinue) { go version } else { exit 1 }");
            Dependency { id: "golang".into(), name: "Go".into(), category: "Runtimes".into(),
                description: "Langage Go (runtime + compilateur)".into(),
                installed: ok, version: v, winget_id: "GoLang.Go".into(),
                install_url: "https://go.dev".into() }
        },
    ]
}

pub fn install_via_winget(winget_id: &str) -> Result<String, String> {
    if winget_id.is_empty() {
        return Err("Pas d'ID winget — installez manuellement.".into());
    }
    let mut cmd = std::process::Command::new("winget");
    cmd.args(["install", "--id", winget_id, "--accept-source-agreements", "--accept-package-agreements", "-h"]);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);
    match cmd.output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if out.status.success() {
                Ok(format!("✓ {} installé avec succès", winget_id))
            } else {
                Err(format!("Erreur winget: {}{}", stdout, stderr))
            }
        }
        Err(e) => Err(format!("Impossible de lancer winget: {}", e)),
    }
}
