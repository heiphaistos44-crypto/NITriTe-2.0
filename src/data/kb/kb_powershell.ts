import type { KBCategory } from "../knowledgeBase";

export const kbPowershell: KBCategory[] = [
  {
    id: "powershell",
    label: "PowerShell",
    icon: "Terminal",
    items: [
      {
        title: "PowerShell - Introduction et bases",
        symptoms: "Besoin d'automatiser des tâches, remplacer CMD par un outil plus puissant",
        solution: [
          "Ouvrir PowerShell : Win+X > Terminal Windows, ou chercher 'PowerShell' dans Démarrer",
          "Exécuter en admin : Clic droit > Exécuter en tant qu'administrateur",
          "Format des commandes : Verbe-Nom (Get-Process, Set-Service, Remove-Item)",
          "Aide intégrée : Get-Help Get-Process -Examples",
          "Chercher des commandes : Get-Command *service* ou Get-Command -Verb Get",
          "Autoriser scripts : Set-ExecutionPolicy RemoteSigned -Scope CurrentUser",
          "Auto-complétion : Tab pour compléter commandes et chemins",
          "Ctrl+R pour chercher dans l'historique des commandes",
        ],
        command: "Get-ExecutionPolicy",
      },
      {
        title: "PowerShell - Commandes système essentielles",
        solution: [
          "Get-Process : lister processus (comme tasklist mais avec objets filtrables)",
          "Stop-Process -Name chrome -Force : terminer un processus par nom",
          "Get-Service | Where-Object {$_.Status -eq 'Running'} : services actifs",
          "Restart-Service -Name Spooler : redémarrer le service d'impression",
          "Get-NetIPAddress : configuration réseau complète",
          "Test-Connection 8.8.8.8 -Count 4 : ping depuis PowerShell",
          "Get-HotFix | Sort InstalledOn -Desc | Select -First 10 : 10 dernières MAJ",
          "Get-CimInstance Win32_OperatingSystem : informations OS complètes",
        ],
        code: `# Espace disque
Get-PSDrive -PSProvider FileSystem | Select Name, Used, Free

# Processus gourmands RAM
Get-Process | Sort WS -Descending | Select -First 10 Name, WS

# Uptime
(Get-Date) - (Get-CimInstance Win32_OperatingSystem).LastBootUpTime

# Version Windows
(Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion').ProductName`,
      },
      {
        title: "PowerShell - Scripts d'administration utiles",
        solution: [
          "Script de nettoyage disque automatisé (fichiers temp, cache, corbeille)",
          "Script de backup avec rotation (ZIP horodaté, suppression anciens backups)",
          "Script d'inventaire système complet (export HTML avec CPU, RAM, disques)",
          "Script d'installation en batch via Winget (équivalent NiTriTe Master Install)",
          "Script de surveillance processus avec alertes",
          "Créer tâche planifiée PowerShell : Register-ScheduledTask",
          "Export/Import données : Export-Csv, ConvertTo-Json, Import-Csv",
        ],
        code: `# Nettoyage fichiers temp
$paths = @("$env:TEMP\\*", "C:\\Windows\\Temp\\*")
foreach ($path in $paths) {
    Remove-Item $path -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "Nettoyé: $path" -ForegroundColor Green
}

# Installer liste de programmes via Winget
$apps = @("7zip.7zip", "Mozilla.Firefox", "VideoLAN.VLC")
foreach ($app in $apps) {
    winget install --id $app --silent --accept-source-agreements
}`,
      },
    ],
  }
];
