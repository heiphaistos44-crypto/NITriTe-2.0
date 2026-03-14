import type { KBCategory } from "../knowledgeBase";

export const kbAutomatisation: KBCategory[] = [
  {
    id: "autohotkey",
    label: "AutoHotkey — Automatisation Bureau",
    icon: "Zap",
    items: [
      {
        title: "AutoHotkey — introduction et scripts essentiels",
        solution: [
          "AutoHotkey (AHK) : langage de script pour automatiser les tâches Windows (souris, clavier, GUI)",
          "Télécharger depuis autohotkey.com — v2 (moderne) recommandée pour les nouveaux scripts",
          "Les scripts .ahk s'exécutent en double-cliquant ou se compilent en .exe",
          "Démarrage automatique : placer un raccourci dans shell:startup",
          "Hotkeys : raccourcis clavier déclenchant des actions",
          "Hotstrings : remplacer des abréviations par du texte (correction auto, expansions)",
          "Send : simuler des frappes clavier",
          "Click/MouseMove : simuler des clics et déplacements de souris",
        ],
        code: `; AutoHotkey v2 — script de démarrage essentiel
; Placer dans le dossier Démarrage : shell:startup

; === HOTSTRINGS (expansions de texte) ===
:*:@@::monemail@exemple.com       ; @@ → email
:*:addr::123 Rue Principale, Paris ; addr → adresse
:*:sig::Cordialement,{Enter}Prénom NOM{Enter}+33 6 12 34 56 78

; === HOTKEYS ===
; Win+N → ouvrir Notepad
#n::Run "notepad.exe"

; Ctrl+Alt+C → ouvrir CMD admin
^!c::Run "*RunAs cmd.exe"

; Win+Shift+S → capture d'écran (Snipping Tool)
#+s::Send "#+s"

; Ctrl+Shift+V → coller en texte brut (sans formatage)
^+v:: {
    text := A_Clipboard
    A_Clipboard := text          ; Supprimer le formatage riche
    Sleep 50
    Send "^v"
}

; Renommer une sélection de fichiers dans l'Explorateur
; Appuyer F2 après sélection
$F2:: {
    if WinActive("ahk_class CabinetWClass")  ; Explorateur Windows
        Send "{F2}"
    else
        Send "{F2}"
}

; === GESTIONNAIRE DE FENÊTRES ===
; Win+Flèches = snap personnalisé
#Left::WinMove "A",, 0, 0, A_ScreenWidth//2, A_ScreenHeight
#Right::WinMove "A",, A_ScreenWidth//2, 0, A_ScreenWidth//2, A_ScreenHeight
#Up::WinMaximize "A"

; === RACCOURCIS APPLICATIONS ===
; Ctrl+Alt+T → terminal Windows
^!t::Run "wt.exe"                ; Windows Terminal

; Suspend le script (icône systray)
Pause::Suspend`,
        note: "AutoHotkey v2 est incompatible avec les scripts v1. Utiliser le convertisseur v1→v2 disponible sur le forum AHK.",
      },
      {
        title: "AutoHotkey — automatisation avancée",
        code: `; AHK v2 — automatisations avancées

; === FORMULAIRES AUTOMATIQUES ===
; Remplir un formulaire web (adapté au site)
FillForm() {
    WinActivate "Site Login - Chrome"
    Sleep 500
    ; Cliquer sur le champ email
    Click 400, 300
    Send "user@exemple.com"
    Send "{Tab}"
    Send "MotDePasseSecret"
    Send "{Enter}"
}
F6::FillForm()

; === BATCH RENOMMER DES FICHIERS ===
; Renommer les fichiers sélectionnés dans l'Explorateur
#r:: {
    ; Sauvegarder le presse-papier
    backup := ClipboardAll()
    A_Clipboard := ""

    ; Copier les chemins des fichiers sélectionnés
    Send "^c"
    ClipWait 1

    files := StrSplit(A_Clipboard, "\`n")
    prefix := InputBox("Préfixe à ajouter", "Renommer", "w300 h120").Value
    if prefix = ""
        return

    for file in files {
        file := Trim(file, " \`r\`n")
        if FileExist(file) {
            dir := SubStr(file, 1, InStr(file, "\",,-1))
            name := SubStr(file, InStr(file, "\",,-1)+1)
            FileMove file, dir . prefix . "_" . name
        }
    }

    A_Clipboard := backup
    MsgBox "Renommage terminé!"
}

; === SURVEILLANCE D'ÉCRAN ===
; Cliquer automatiquement quand une couleur apparaît
WatchForColor() {
    Loop {
        ; Chercher un pixel orange (bouton "Accepter") dans une zone
        if PixelSearch(&x, &y, 100, 100, 500, 400, 0xFF6600, 5)
            Click x, y
        Sleep 1000  ; Vérifier toutes les secondes
    }
}`,
      },
      {
        title: "Gestionnaire de tâches planifiées Windows — avancé",
        solution: [
          "Task Scheduler (taskschd.msc) : planifier l'exécution de scripts et programmes",
          "Déclencheurs : à l'heure, au démarrage, à la connexion, sur un événement, en cas d'inactivité",
          "Actions : démarrer un programme, envoyer un email (obsolète), afficher un message",
          "Conditions : AC seulement, réseau disponible, inactivité requise",
          "Paramètres : reprendre si raté, exécuter le plus tôt possible si manqué",
          "Exécuter avec les privilèges les plus élevés : pour les scripts nécessitant admin",
          "schtasks : outil CLI pour gérer les tâches planifiées",
        ],
        code: `# Créer des tâches planifiées via PowerShell

# === TÂCHE SIMPLE : Script quotidien à 3h du matin ===
$action = New-ScheduledTaskAction -Execute "PowerShell.exe" \`
  -Argument "-WindowStyle Hidden -ExecutionPolicy Bypass -File C:\\Scripts\\backup.ps1"

$trigger = New-ScheduledTaskTrigger -Daily -At "03:00"

$settings = New-ScheduledTaskSettingsSet \`
  -ExecutionTimeLimit (New-TimeSpan -Hours 2) \`
  -RunOnlyIfNetworkAvailable \`
  -StartWhenAvailable \`
  -WakeToRun

$principal = New-ScheduledTaskPrincipal -UserId "SYSTEM" -LogonType ServiceAccount -RunLevel Highest

Register-ScheduledTask -TaskName "Backup Nightly" \`
  -Action $action -Trigger $trigger -Settings $settings -Principal $principal -Force

# === TÂCHE : Au démarrage (après connexion réseau) ===
$trigger = New-ScheduledTaskTrigger -AtStartup
$trigger.Delay = "PT2M"  # Délai de 2 minutes après démarrage

Register-ScheduledTask -TaskName "StartupScript" \`
  -Action (New-ScheduledTaskAction -Execute "PowerShell.exe" -Argument "-File C:\\Scripts\\startup.ps1") \`
  -Trigger $trigger \`
  -RunLevel Highest -Force

# === TÂCHE : Sur un événement Windows (BSOD détecté) ===
$eventTrigger = New-ScheduledTaskTrigger -OnEvent \`
  -LogName System -Source "BugCheck" -EventId 1001

Register-ScheduledTask -TaskName "OnBSOD" \`
  -Action (New-ScheduledTaskAction -Execute "PowerShell.exe" -Argument "-File C:\\Scripts\\on-bsod.ps1") \`
  -Trigger $eventTrigger -Force

# === Gestion schtasks ===
schtasks /query /fo LIST /v | Select-String "TaskName|Status|Next Run"
schtasks /run /tn "Backup Nightly"           # Exécuter immédiatement
schtasks /end /tn "Backup Nightly"           # Arrêter
schtasks /delete /tn "Backup Nightly" /f     # Supprimer
schtasks /change /tn "Backup Nightly" /disable  # Désactiver

# Exporter/importer une tâche planifiée
schtasks /query /tn "Backup Nightly" /xml > backup-task.xml
schtasks /create /tn "Backup Nightly" /xml backup-task.xml`,
      },
    ],
  },
  {
    id: "powershell-automation",
    label: "PowerShell — Automatisation",
    icon: "Terminal",
    items: [
      {
        title: "PowerShell — modules et profil",
        code: `# Profil PowerShell (~\\Documents\\PowerShell\\Microsoft.PowerShell_profile.ps1)
# Exécuté au démarrage de chaque session PowerShell

# Voir/créer le fichier profil
$PROFILE                           # Chemin du profil
Test-Path $PROFILE                 # Vérifier s'il existe
New-Item -Path $PROFILE -ItemType File -Force

# Contenu exemple du profil
function prompt {
  $location = (Get-Location).Path -replace "C:\\Users\\$env:USERNAME", "~"
  Write-Host "[" -NoNewline -ForegroundColor DarkGray
  Write-Host $env:USERNAME -NoNewline -ForegroundColor Cyan
  Write-Host "@" -NoNewline -ForegroundColor DarkGray
  Write-Host $env:COMPUTERNAME -NoNewline -ForegroundColor Yellow
  Write-Host "] " -NoNewline -ForegroundColor DarkGray
  Write-Host $location -NoNewline -ForegroundColor Green
  Write-Host " $" -NoNewline -ForegroundColor White
  return " "
}

# Alias utiles
Set-Alias -Name ll -Value Get-ChildItem
Set-Alias -Name grep -Value Select-String
Set-Alias -Name which -Value Get-Command

function uptime {
  (Get-Date) - (gcim Win32_OperatingSystem).LastBootUpTime
}

function weather {
  (Invoke-WebRequest "wttr.in/?format=3").Content
}

# Modules utiles à installer
Install-Module -Name PSReadLine -Scope CurrentUser -Force      # Meilleure ligne de commande
Install-Module -Name Terminal-Icons -Scope CurrentUser -Force  # Icônes dans ls
Install-Module -Name z -Scope CurrentUser -Force               # Navigation rapide cd

# oh-my-posh : thème de terminal magnifique
winget install JanDeDobbeleer.OhMyPosh
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\\agnoster.omp.json" | Invoke-Expression`,
      },
      {
        title: "PowerShell — manipulation fichiers et texte",
        code: `# === LECTURE ET TRAITEMENT DE FICHIERS ===

# Lire un CSV et traiter
$users = Import-Csv "utilisateurs.csv"
$users | Where-Object { $_.Departement -eq "IT" } |
  Select-Object Nom, Email |
  Export-Csv "it-users.csv" -NoTypeInformation

# Lire un JSON
$config = Get-Content "config.json" | ConvertFrom-Json
$config.settings.timeout = 30
$config | ConvertTo-Json | Set-Content "config.json"

# Chercher dans des fichiers
Get-ChildItem -Recurse -Filter "*.log" |
  Select-String -Pattern "ERROR|CRITICAL" |
  Select-Object Filename, LineNumber, Line |
  Export-Csv "errors.csv" -NoTypeInformation

# Chercher et remplacer dans des fichiers
Get-ChildItem -Recurse -Filter "*.config" | ForEach-Object {
  (Get-Content $_.FullName) -replace "ancien-serveur", "nouveau-serveur" |
    Set-Content $_.FullName
}

# === GÉNÉRATION DE RAPPORTS ===
$report = [ordered]@{
  Date = (Get-Date).ToString("yyyy-MM-dd HH:mm")
  PC = $env:COMPUTERNAME
  OS = (Get-WmiObject Win32_OperatingSystem).Caption
  RAM = "$([math]::Round((Get-WmiObject Win32_ComputerSystem).TotalPhysicalMemory/1GB, 1)) Go"
  CPU = (Get-WmiObject Win32_Processor).Name
  Uptime = ((Get-Date) - (Get-WmiObject Win32_OperatingSystem).ConvertToDateTime((Get-WmiObject Win32_OperatingSystem).LastBootUpTime)).ToString("d\j h\h m\m")
}

# Rapport HTML
$html = @"
<!DOCTYPE html><html><head><style>
body{font-family:Arial;} table{border-collapse:collapse;width:100%}
td,th{border:1px solid #ddd;padding:8px} th{background:#4CAF50;color:white}
tr:nth-child(even){background:#f2f2f2}
</style></head><body>
<h2>Rapport Système — $($report.PC)</h2>
<table>
$(($report.GetEnumerator() | ForEach-Object { "<tr><th>$($_.Key)</th><td>$($_.Value)</td></tr>" }) -join "\`n")
</table></body></html>
"@
$html | Out-File "rapport.html"
Start-Process "rapport.html"`,
      },
      {
        title: "PowerShell Remoting — gestion à distance",
        solution: [
          "PowerShell Remoting : exécuter des commandes sur des PC distants via WinRM",
          "Activer sur la cible : Enable-PSRemoting -Force (nécessite admin et réseau privé)",
          "Enter-PSSession : session interactive sur un PC distant (comme SSH)",
          "Invoke-Command : exécuter un script sur un ou plusieurs PC simultanément",
          "New-PSSession : créer une session persistante réutilisable",
          "Sécurité : fonctionne via HTTPS (avec certificat) ou HTTP (réseau de confiance seulement)",
          "Multi-machine : Invoke-Command -ComputerName PC1, PC2, PC3 exécute en parallèle",
          "DCOM/WMI : alternative plus ancienne (Get-WmiObject -ComputerName)",
        ],
        code: `# Activer PowerShell Remoting
Enable-PSRemoting -Force -SkipNetworkProfileCheck

# Ajouter la machine dans la liste de confiance (si réseau public)
Set-Item WSMan:\\localhost\\Client\\TrustedHosts -Value "192.168.1.*" -Force

# Session interactive
Enter-PSSession -ComputerName PC-FINANCE-01 -Credential (Get-Credential)
# Dans la session distante :
Get-Process | Where-Object { $_.CPU -gt 10 }
exit

# Exécuter une commande sur plusieurs PC
$pcs = "PC-FINANCE-01", "PC-RH-02", "PC-COMPTA-03"
Invoke-Command -ComputerName $pcs -ScriptBlock {
  Get-WmiObject Win32_OperatingSystem | Select CSName, Caption, LastBootUpTime
}

# Exécuter un script local sur des PC distants
Invoke-Command -ComputerName $pcs -FilePath "C:\\Scripts\\audit.ps1"

# Session persistante (réutilisable)
$session = New-PSSession -ComputerName PC-FINANCE-01 -Credential (Get-Credential)
Invoke-Command -Session $session -ScriptBlock { Get-Process }
Invoke-Command -Session $session -ScriptBlock { Stop-Service "Fax" }
Remove-PSSession $session

# Copier des fichiers via PSRemoting
$session = New-PSSession -ComputerName PC-FINANCE-01
Copy-Item "C:\\Scripts\\deploy.ps1" -ToSession $session -Destination "C:\\Temp\\"
Copy-Item "C:\\Logs\\finance.log" -FromSession $session -Destination "C:\\Logs\\remote\\"

# Inventaire de parc en masse
$pcs = Get-ADComputer -Filter {OperatingSystem -like "*Windows*"} | Select -ExpandProperty Name
Invoke-Command -ComputerName $pcs -ErrorAction SilentlyContinue -ScriptBlock {
  [PSCustomObject]@{
    PC = $env:COMPUTERNAME
    OS = (Get-WmiObject Win32_OperatingSystem).Caption
    RAM = [math]::Round((Get-WmiObject Win32_ComputerSystem).TotalPhysicalMemory/1GB, 0)
    CPU = (Get-WmiObject Win32_Processor).Name
    Uptime = ([datetime]::Now - (Get-WmiObject Win32_OperatingSystem).ConvertToDateTime((Get-WmiObject Win32_OperatingSystem).LastBootUpTime)).Days
  }
} | Export-Csv "inventaire-parc.csv" -NoTypeInformation`,
      },
      {
        title: "WMI / CIM — interroger le système en profondeur",
        code: `# CIM (WMI moderne) — classes essentielles
# Système
Get-CimInstance Win32_ComputerSystem | Select Name, Manufacturer, Model, TotalPhysicalMemory
Get-CimInstance Win32_OperatingSystem | Select Caption, Version, BuildNumber, LastBootUpTime
Get-CimInstance Win32_BIOS | Select SMBIOSBIOSVersion, Manufacturer, ReleaseDate

# CPU
Get-CimInstance Win32_Processor | Select Name, NumberOfCores, MaxClockSpeed, CurrentVoltage, LoadPercentage

# RAM
Get-CimInstance Win32_PhysicalMemory | Select BankLabel, Capacity, Speed, MemoryType, Manufacturer

# Stockage
Get-CimInstance Win32_DiskDrive | Select Model, Size, InterfaceType, MediaType, SerialNumber
Get-CimInstance Win32_LogicalDisk | Select DeviceID, FileSystem, Size, FreeSpace, VolumeName

# Réseau
Get-CimInstance Win32_NetworkAdapterConfiguration | Where-Object { $_.IPEnabled } |
  Select Description, IPAddress, DefaultIPGateway, DNSServerSearchOrder, MACAddress

# Processus
Get-CimInstance Win32_Process | Select Name, ProcessId, WorkingSetSize, CreationDate |
  Sort-Object WorkingSetSize -Descending | Select -First 20

# Services
Get-CimInstance Win32_Service | Where-Object { $_.State -eq "Running" } |
  Select Name, DisplayName, StartMode, State

# Logiciels installés
Get-CimInstance Win32_Product | Select Name, Version, Vendor, InstallDate |
  Sort-Object Name

# Inventaire hardware complet vers JSON
$inventory = @{
  Date = (Get-Date).ToString("o")
  Computer = (Get-CimInstance Win32_ComputerSystem).Name
  OS = (Get-CimInstance Win32_OperatingSystem).Caption
  BIOS = (Get-CimInstance Win32_BIOS).SMBIOSBIOSVersion
  CPU = (Get-CimInstance Win32_Processor).Name
  RAM = (Get-CimInstance Win32_PhysicalMemory | Measure-Object Capacity -Sum).Sum / 1GB
  Disks = Get-CimInstance Win32_DiskDrive | ForEach-Object { @{ Model = $_.Model; SizeGB = [math]::Round($_.Size/1GB) } }
}
$inventory | ConvertTo-Json -Depth 3 | Out-File "inventory-$($inventory.Computer).json"`,
      },
    ],
  },
  {
    id: "batch-scripts",
    label: "Scripts Batch Avancés",
    icon: "Terminal",
    items: [
      {
        title: "Batch Windows — techniques avancées",
        solution: [
          "Les scripts .bat/.cmd sont toujours utiles pour la compatibilité maximale (aucune dépendance PowerShell)",
          "@echo off : masquer les commandes dans la sortie",
          "setlocal enabledelayedexpansion : activer les variables dans les boucles (!var! au lieu de %var%)",
          "errorlevel : code de retour de la dernière commande (0 = succès)",
          "goto : saut conditionnel (goto :EOF pour quitter)",
          "call : appeler une sous-routine ou un autre script batch",
          "for /f : parser la sortie de commandes ou le contenu de fichiers",
          "choice : menu interactif avec timeout",
        ],
        code: `@echo off
setlocal enabledelayedexpansion

:: === TEMPLATE SCRIPT BATCH PROFESSIONNEL ===
:: Vérification des droits admin
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo ERREUR: Ce script necessite les droits administrateur.
    echo Clic droit ^> Executer en tant qu'administrateur
    pause & exit /b 1
)

:: Variables
set "SCRIPT_NAME=%~n0"
set "SCRIPT_DIR=%~dp0"
set "LOG_FILE=%SCRIPT_DIR%.logs\%SCRIPT_NAME%_%DATE:~-4,4%%DATE:~-7,2%%DATE:~-10,2%.log"
set "VERSION=1.0.0"

:: Créer le dossier de logs
if not exist "%SCRIPT_DIR%.logs" mkdir "%SCRIPT_DIR%.logs"

:: Fonction de log
:log
    echo [%DATE% %TIME%] %~1
    echo [%DATE% %TIME%] %~1 >> "%LOG_FILE%"
    goto :eof

:: Traitement des arguments
:parse_args
    if "%~1"=="" goto :main
    if /i "%~1"=="/help" goto :help
    if /i "%~1"=="/version" ( echo Version: %VERSION% & exit /b 0 )
    shift
    goto :parse_args

:main
call :log "Démarrage de %SCRIPT_NAME% v%VERSION%"

:: Menu interactif
:menu
echo.
echo  ==========================================
echo   %SCRIPT_NAME% — Menu Principal
echo  ==========================================
echo   [1] Nettoyage système
echo   [2] Vérification des services
echo   [3] Rapport système
echo   [Q] Quitter
echo  ==========================================
choice /c 123Q /m "Votre choix" /t 30 /d Q

if errorlevel 4 goto :end
if errorlevel 3 goto :rapport
if errorlevel 2 goto :services
if errorlevel 1 goto :nettoyage

:nettoyage
call :log "Nettoyage en cours..."
del /f /q "%TEMP%\\*" 2>nul
for /d %%i in ("%TEMP%\\*") do rd /s /q "%%i" 2>nul
call :log "Nettoyage terminé"
goto :menu

:services
call :log "Vérification des services..."
for %%s in (Spooler BITS wuauserv) do (
    sc query "%%s" | findstr /i "running" >nul
    if !errorlevel! equ 0 (
        call :log "Service %%s: RUNNING [OK]"
    ) else (
        call :log "Service %%s: STOPPED [ATTENTION]"
    )
)
goto :menu

:rapport
call :log "Génération du rapport..."
systeminfo > "%SCRIPT_DIR%rapport_systeme.txt"
call :log "Rapport sauvegardé: %SCRIPT_DIR%rapport_systeme.txt"
goto :menu

:end
call :log "Fin du script %SCRIPT_NAME%"
exit /b 0

:help
echo Usage: %SCRIPT_NAME% [/help] [/version]
exit /b 0`,
      },
    ],
  },
];
