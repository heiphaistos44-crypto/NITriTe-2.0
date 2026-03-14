import type { KBCategory } from "../knowledgeBase";

export const kbErreursSysteme: KBCategory[] = [
  {
    id: "codes-erreurs-windows",
    label: "Codes d'Erreur Windows",
    icon: "AlertTriangle",
    items: [
      {
        title: "Codes d'erreur système courants — référence complète",
        solution: [
          "Code 0 (ERROR_SUCCESS) : succès — pas une erreur",
          "Code 1 (ERROR_INVALID_FUNCTION) : fonction invalide",
          "Code 2 (ERROR_FILE_NOT_FOUND) : fichier introuvable",
          "Code 3 (ERROR_PATH_NOT_FOUND) : chemin introuvable",
          "Code 5 (ERROR_ACCESS_DENIED) : accès refusé — droits insuffisants",
          "Code 32 (ERROR_SHARING_VIOLATION) : fichier utilisé par un autre processus",
          "Code 87 (ERROR_INVALID_PARAMETER) : paramètre invalide",
          "Code 112 (ERROR_DISK_FULL) : disque plein",
          "Code 1168 (ERROR_NOT_FOUND) : élément introuvable (souvent registre)",
          "Code 1223 (ERROR_CANCELLED) : opération annulée par l'utilisateur",
        ],
        code: `# Décoder un code d'erreur Windows
$code = 5  # Exemple : ERROR_ACCESS_DENIED
$msg = [System.ComponentModel.Win32Exception]::new($code).Message
Write-Host "Code $code : $msg"

# Fonction utilitaire pour décoder n'importe quel code
function Get-WindowsError {
  param([int]$Code)
  $err = [System.ComponentModel.Win32Exception]::new($Code)
  [PSCustomObject]@{
    Code = $Code
    HexCode = "0x{0:X8}" -f $Code
    Message = $err.Message
  }
}

# Exemples
Get-WindowsError 5    # Access Denied
Get-WindowsError 1392  # File or directory is corrupted

# Via net helpmsg (CMD)
net helpmsg 5
net helpmsg 87

# Erreurs HRESULT (COM, .NET)
# 0x80070005 = ACCESS_DENIED (0x8007 = Win32, 0005 = code 5)
# 0x80070002 = FILE_NOT_FOUND
# 0x8007000D = INVALID_DATA
# 0x80004005 = Unspecified error
# 0xC0000005 = Access Violation (crash dump)

# Décoder HRESULT
function Get-HRESULT {
  param([string]$HResult)
  $code = [Convert]::ToInt32($HResult, 16)
  $win32Code = $code -band 0xFFFF
  Get-WindowsError $win32Code
}
Get-HRESULT "0x80070005"`,
      },
      {
        title: "Codes BSOD — référence et solutions",
        solution: [
          "0x0000007E SYSTEM_THREAD_EXCEPTION_NOT_HANDLED : driver corrompu ou incompatible",
          "0x0000000A IRQL_NOT_LESS_OR_EQUAL : driver accède à une zone mémoire invalide",
          "0x00000050 PAGE_FAULT_IN_NONPAGED_AREA : défaut de page en mémoire non paginée — RAM défectueuse ou driver",
          "0x0000007F UNEXPECTED_KERNEL_MODE_TRAP : hardware défaillant (RAM, CPU)",
          "0x00000101 CLOCK_WATCHDOG_TIMEOUT : processeur ne répond plus (overclocking instable, refroidissement)",
          "0x0000003B SYSTEM_SERVICE_EXCEPTION : exception dans un appel système — drivers",
          "0x000000EF CRITICAL_PROCESS_DIED : processus critique de Windows s'est arrêté",
          "0xC000021A STATUS_SYSTEM_PROCESS_TERMINATED : smss.exe ou csrss.exe mort — corruption système",
        ],
        code: `# Analyser un crash dump via PowerShell
# Les minidumps sont dans C:\\Windows\\Minidump

# Lister les crashs récents
Get-EventLog -LogName System -EntryType Error -Source "BugCheck" -Newest 10 |
  Select-Object TimeGenerated, Message

# Identifier le code BSOD depuis l'événement
$event = Get-WinEvent -FilterHashtable @{LogName='System'; Id=1001; ProviderName='Microsoft-Windows-WER-SystemErrorReporting'} |
  Select-Object -First 1
$event.Message

# Analyser le crash dump avec WinDbg (Windows Debugging Tools)
# Installer : winget install Microsoft.WinDbg
# Ouvrir le dump : WinDbg > File > Open Crash Dump > C:\\Windows\\MEMORY.DMP
# Commandes WinDbg :
# !analyze -v       → analyse automatique complète
# !thread           → thread en cours
# lmvm nom_driver  → infos sur un driver
# .symfix           → configurer les symboles Microsoft (serveur public)
# .reload           → recharger les symboles

# Script d'analyse automatique des dumps
Get-ChildItem "C:\\Windows\\Minidump\\*.dmp" | Sort-Object LastWriteTime -Descending |
  Select-Object -First 5 | ForEach-Object {
    Write-Host "Crash: $($_.LastWriteTime)" -ForegroundColor Yellow
    Write-Host "Fichier: $($_.FullName)"
    Write-Host ""
  }`,
        note: "WhoCrashed (free) : analyse automatique des crash dumps et propose des causes lisibles pour les non-experts.",
      },
      {
        title: "Event Viewer — analyser les journaux système",
        solution: [
          "Event Viewer (eventvwr.msc) : outil principal pour diagnostiquer les problèmes Windows",
          "Journaux Windows > Application : erreurs des applications (crashes, erreurs .NET)",
          "Journaux Windows > Système : erreurs hardware, drivers, services",
          "Journaux Windows > Sécurité : connexions, modifications des droits, audit",
          "Journaux Applications et Services > Microsoft > Windows : logs détaillés par composant",
          "Niveau : Critique (rouge), Erreur (rouge), Avertissement (jaune), Informations (bleu), Détail (gris)",
          "Créer une vue personnalisée : filtrer par source, niveau, période",
          "Attacher une tâche à un événement : déclencheur automatique sur un événement spécifique",
        ],
        code: `# Analyser les journaux via PowerShell
# Crashs applications (dernières 24h)
Get-EventLog -LogName Application -EntryType Error -After (Get-Date).AddDays(-1) |
  Select-Object TimeGenerated, Source, EventID, Message | Format-Table -Wrap

# Erreurs système critiques
Get-WinEvent -FilterHashtable @{
  LogName = 'System'
  Level = 1,2          # 1=Critique, 2=Erreur
  StartTime = (Get-Date).AddDays(-7)
} | Select-Object TimeCreated, Id, ProviderName, Message |
  Sort-Object TimeCreated -Descending | Select-Object -First 50

# Trouver les redémarrages non planifiés
Get-EventLog -LogName System -Source "EventLog" -EventId 6008 |
  Select-Object TimeGenerated, Message

# Trouver les connexions échouées (Sécurité — ID 4625)
Get-WinEvent -FilterHashtable @{LogName='Security'; Id=4625} -MaxEvents 20 |
  ForEach-Object {
    [PSCustomObject]@{
      Time = $_.TimeCreated
      User = $_.Properties[5].Value
      Domain = $_.Properties[6].Value
      IP = $_.Properties[19].Value
      Reason = $_.Properties[8].Value
    }
  } | Format-Table

# Exporter les journaux pour analyse
wevtutil epl System "C:\\Logs\\System.evtx"         # Exporter journal Système
wevtutil epl Application "C:\\Logs\\Application.evtx"  # Exporter journal Application

# Effacer un journal (Admin)
Clear-EventLog -LogName Application
wevtutil cl System

# Activer un journal désactivé (ex: Security auditing)
wevtutil set-log "Microsoft-Windows-TaskScheduler/Operational" /enabled:true`,
      },
      {
        title: "Process Monitor & Process Explorer — débogage avancé",
        solution: [
          "Process Monitor (ProcMon) : outil Sysinternals, capture TOUS les accès fichiers, registre, réseau et processus",
          "Process Explorer : gestionnaire de tâches avancé, remplace complètement le gestionnaire intégré",
          "Télécharger : learn.microsoft.com/sysinternals ou winget install Microsoft.Sysinternals.ProcessMonitor",
          "ProcMon — Filtres utiles : Filter > Process Name (contient) > nom_du_process.exe",
          "ProcMon — chercher les erreurs ACCESS DENIED : Filter > Result is 'ACCESS DENIED'",
          "ProcMon — diagnostiquer les crashs : Filter > Category is 'Process' + Event Type 'Process Exit'",
          "Process Explorer — vérifier les DLLs chargées : clic droit > Properties > DLLs",
          "Process Explorer — vérifier les signatures : Options > VirusTotal.com > Check VirusTotal.com",
        ],
        code: `# Lancer Process Monitor via ligne de commande
# Télécharger Procmon.exe depuis Sysinternals
procmon.exe /Quiet /Minimized /BackingFile "C:\\Logs\\procmon.pml"  # Capture silencieuse
# Attendre... puis arrêter
procmon.exe /Terminate

# Analyser une capture PML
procmon.exe /OpenLog "C:\\Logs\\procmon.pml"

# Alternative PowerShell : surveiller les fichiers créés/modifiés
$watcher = New-Object System.IO.FileSystemWatcher
$watcher.Path = "C:\\Program Files"
$watcher.IncludeSubdirectories = $true
$watcher.EnableRaisingEvents = $true

$action = {
  Write-Host "[$($event.TimeGenerated)] $($event.SourceEventArgs.ChangeType): $($event.SourceEventArgs.FullPath)"
}

Register-ObjectEvent $watcher "Created" -Action $action
Register-ObjectEvent $watcher "Changed" -Action $action
Register-ObjectEvent $watcher "Deleted" -Action $action

# Attendre et observer...
Start-Sleep 30
Get-EventSubscriber | Unregister-Event`,
      },
    ],
  },
  {
    id: "erreurs-reseau-windows",
    label: "Erreurs Réseau Courantes",
    icon: "Wifi",
    items: [
      {
        title: "Erreurs réseau — codes et solutions",
        solution: [
          "Erreur 0x800704CF 'Ressource réseau introuvable' : découverte réseau désactivée ou pare-feu",
          "Erreur 0x80070035 'Chemin réseau introuvable' : problème SMB, name resolution, ou partage inexistant",
          "Erreur 1231 'Le protocole réseau n'est pas disponible' : pile TCP/IP corrompue — netsh reset",
          "Erreur 53 'Le chemin d'accès réseau est introuvable' : DNS/NetBIOS ne résout pas le nom",
          "Erreur 58 'Le système réseau distant n'est pas compatible' : version SMB incompatible",
          "Erreur 64 'Le nom réseau spécifié n'est plus disponible' : connexion SMB interrompue",
          "Erreur 1326 'Le nom d'utilisateur ou le mot de passe est incorrect' : credentials réseau incorrects",
          "Gestionnaire de credentials : Panneau de config > Gestionnaire d'identification > Windows Credentials",
        ],
        code: `# Diagnostiquer les erreurs réseau
# Tester la résolution DNS
Resolve-DnsName google.com
Resolve-DnsName "serveur-local" -Type ANY
nslookup serveur-local 192.168.1.10   # Tester un DNS spécifique

# Tester la connectivité
Test-NetConnection -ComputerName "192.168.1.100" -Port 445  # SMB
Test-NetConnection -ComputerName "192.168.1.100" -Port 3389 # RDP
Test-NetConnection -ComputerName "192.168.1.100" -TraceRoute # Traceroute

# Résoudre l'erreur 0x80070035 (chemin réseau introuvable)
# 1. Vérifier que SMB est activé
Get-SmbClientConfiguration
Enable-WindowsOptionalFeature -Online -FeatureName "SMB1Protocol"  # À éviter si possible (sécurité)
# 2. Vérifier que la découverte réseau est activée
Get-NetFirewallRule -DisplayGroup "Network Discovery" | Select DisplayName, Enabled

# Activer la découverte réseau
netsh advfirewall firewall set rule group="Network Discovery" new enable=Yes

# Résoudre les problèmes de credentials réseau
cmdkey /list                          # Lister les credentials stockés
cmdkey /add:serveur-local /user:user /pass:password  # Ajouter
cmdkey /delete:serveur-local          # Supprimer

# Tester l'accès SMB en spécifiant les credentials
net use \\\\192.168.1.100\\Partage /user:DOMAINE\\utilisateur password
net use * /delete /yes                # Déconnecter tous les partages

# Erreur SMBv1 (Windows 10/11 le désactive par défaut)
# Vérifier si le serveur distant utilise SMBv1 seulement
Test-NetConnection -ComputerName "vieux-serveur" -Port 445
Get-SmbServerConfiguration | Select EnableSMB1Protocol, EnableSMB2Protocol`,
      },
      {
        title: "Réinitialisation Winsock et pile TCP/IP",
        symptoms: "Internet ne fonctionne pas après désinstallation d'un VPN ou antivirus, connexion instable",
        solution: [
          "Winsock : interface entre les applications Windows et les protocoles réseau",
          "Un VPN, antivirus ou malware peut corrompre le catalogue Winsock",
          "netsh winsock reset : réinitialise Winsock aux paramètres par défaut",
          "netsh int ip reset : réinitialise la pile TCP/IP",
          "ipconfig /flushdns : vide le cache DNS local",
          "Redémarrage obligatoire après ces commandes",
          "Si problème persiste : réinitialisation complète via Paramètres > Réseau > Réinitialisation du réseau",
        ],
        code: `# Réinitialisation complète de la pile réseau (exécuter en Admin dans l'ordre)
# ÉTAPE 1 : Winsock
netsh winsock reset catalog

# ÉTAPE 2 : TCP/IP
netsh int ip reset "C:\\Logs\\ip-reset.log"
netsh int ipv4 reset
netsh int ipv6 reset

# ÉTAPE 3 : DNS
ipconfig /flushdns
ipconfig /registerdns

# ÉTAPE 4 : Libérer et renouveler l'IP
ipconfig /release
ipconfig /renew

# ÉTAPE 5 : Réinitialiser les règles pare-feu Windows
netsh advfirewall reset

# ÉTAPE 6 : Redémarrer les adaptateurs réseau
Get-NetAdapter | Restart-NetAdapter

Write-Host "Réinitialisation terminée. REDÉMARRER OBLIGATOIRE." -ForegroundColor Green
shutdown /r /t 10 /c "Redémarrage requis après réinitialisation réseau"

# Si problème VPN résiduel : supprimer les adaptateurs virtuels résiduels
# Gestionnaire de périphériques > Vue > Afficher les périphériques cachés
# Cartes réseau > supprimer les adaptateurs TAP, WAN Miniport orphelins

# Alternative rapide : réinitialisation via Paramètres Windows
# Paramètres > Réseau et Internet > Paramètres réseau avancés > Réinitialisation du réseau`,
      },
    ],
  },
  {
    id: "erreurs-demarrage",
    label: "Erreurs au Démarrage",
    icon: "Power",
    items: [
      {
        title: "Erreurs de démarrage Windows — diagnostic",
        solution: [
          "Winload.exe introuvable : BCD (Boot Configuration Data) corrompu — corriger avec bootrec/bcdedit",
          "BOOTMGR missing : MBR ou VBR corrompu — corriger depuis Windows PE (bootrec /fixmbr)",
          "Fichier HAL.DLL corrompu : corriger depuis Windows Recovery Environment (SFC /SCANNOW en WinRE)",
          "Winlogon.exe ne peut pas démarrer : corruption du profil utilisateur ou virus",
          "smss.exe ne peut pas démarrer : corruption des fichiers système (DISM + SFC)",
          "Boucle de réparation automatique : Paramètres avancés > Invité de commandes > bootrec /rebuildbcd",
          "Écran bleu au démarrage : voir les crashs dumps dans WinRE > Invite de commandes > C:\\Windows\\Minidump",
          "Safe Mode : F8 pendant le boot (Windows 10/11 : souvent nécessite bcdedit pour l'activer)",
        ],
        code: `# Activer le menu F8 (mode sans échec au démarrage)
bcdedit /set {default} bootmenupolicy legacy
# Pour le désactiver :
bcdedit /set {default} bootmenupolicy standard

# Démarrer en mode sans échec directement
bcdedit /set {default} safeboot minimal    # Mode sans échec
bcdedit /set {default} safeboot network    # Mode sans échec avec réseau
# Annuler le mode sans échec forcé :
bcdedit /deletevalue {default} safeboot

# Via msconfig (GUI)
msconfig
# > Démarrer > Mode sans échec > Minimal

# Réparer le BCD depuis Windows PE/WinRE
# Booter sur USB Windows 11 > Réparer l'ordinateur > Invité de commandes
bootrec /fixmbr          # Réparer le MBR
bootrec /fixboot         # Réparer le VBR (Volume Boot Record)
bootrec /scanos          # Scanner les installations Windows
bootrec /rebuildbcd      # Reconstruire le BCD

# Cas spécial : GPT (UEFI) — réparer EFI
diskpart
list disk
select disk 0
list partition
# Identifier la partition EFI (type System, ~100Mo)
select partition 1
assign letter=Z
exit

# Réparer le BCD EFI
bcdboot C:\\Windows /s Z: /f UEFI
Z:
cd EFI\\Microsoft\\Boot
bootrec /fixboot`,
      },
      {
        title: "Réparation Windows — SFC, DISM, WinRE",
        code: `# === RÉPARATION EN LIGNE (Windows fonctionne) ===

# SFC (System File Checker) — vérifier et réparer les fichiers système
sfc /scannow
# Rapport : C:\\Windows\\Logs\\CBS\\CBS.log

# SFC hors ligne (si Windows ne démarre pas) — depuis WinRE
sfc /scannow /offbootdir=C:\\ /offwindir=C:\\Windows

# DISM — réparer l'image Windows
# 1. Vérifier la santé
DISM /Online /Cleanup-Image /CheckHealth
# 2. Scanner
DISM /Online /Cleanup-Image /ScanHealth
# 3. Réparer (télécharge depuis Windows Update)
DISM /Online /Cleanup-Image /RestoreHealth

# Réparer depuis un ISO Windows (si pas d'accès Internet)
# Monter l'ISO et trouver le fichier install.wim ou install.esd
DISM /Online /Cleanup-Image /RestoreHealth /Source:D:\\sources\\install.wim /LimitAccess

# Vérifier l'état de l'image après réparation
DISM /Online /Cleanup-Image /CheckHealth

# === DEPUIS WINRE (Windows ne démarre pas) ===
# Booter en WinRE : Paramètres > Système > Récupération > Démarrage avancé
# Ou : F8 pendant le boot > Résoudre les problèmes > Options avancées > Invite de commandes

# Identifier la lettre du volume Windows (peut être différent de C: en WinRE)
diskpart
list volume
exit
# Note la lettre (ex: D:)

# Réparer depuis WinRE
sfc /scannow /offbootdir=D:\\ /offwindir=D:\\Windows
DISM /Image:D:\\ /Cleanup-Image /RestoreHealth

# Réinitialiser Windows 11 (dernier recours, garde les fichiers)
# WinRE > Résoudre les problèmes > Réinitialiser ce PC > Conserver mes fichiers`,
        note: "Toujours exécuter DISM avant SFC — DISM répare le magasin de fichiers de Windows Update que SFC utilise pour ses corrections.",
      },
    ],
  },
];
