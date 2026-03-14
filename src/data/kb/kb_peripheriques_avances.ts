import type { KBCategory } from "../knowledgeBase";

export const kbPeripheriquesAvances: KBCategory[] = [
  {
    id: "imprimantes-avance",
    label: "Imprimantes — Dépannage Avancé",
    icon: "Printer",
    items: [
      {
        title: "Spooler d'impression — diagnostic et réparation",
        symptoms: "Impression bloquée, la file d'impression ne se vide pas, imprimante 'hors ligne'",
        solution: [
          "Le spooler d'impression (Print Spooler) gère la file d'impression — peut se bloquer",
          "Réinitialiser le spooler : arrêter le service, vider la file, redémarrer le service",
          "File d'impression coincée : C:\\Windows\\System32\\spool\\PRINTERS (vider ce dossier)",
          "Imprimante en ligne : clic droit sur l'imprimante > Voir les documents > Imprimante > Décocher 'Utiliser l'imprimante hors connexion'",
          "Supprimer et réajouter l'imprimante si rien ne fonctionne",
          "PrintBrm.exe : outil de sauvegarde/restauration des imprimantes",
          "Journaux d'événements : Applications > Microsoft-Windows-PrintService",
        ],
        code: `# Réinitialiser le spooler d'impression
net stop spooler
# Vider la file d'impression
Remove-Item "C:\\Windows\\System32\\spool\\PRINTERS\\*" -Recurse -Force -ErrorAction SilentlyContinue
net start spooler

Write-Host "Spooler réinitialisé" -ForegroundColor Green

# Vérifier l'état du spooler
Get-Service Spooler | Select Name, Status, StartType

# Supprimer une imprimante bloquée
Get-Printer | Select Name, DriverName, PortName
Remove-Printer -Name "HP LaserJet Pro M15w"

# Supprimer tous les travaux d'impression d'une imprimante
Get-PrintJob -PrinterName "Mon Imprimante" | Remove-PrintJob

# Voir les travaux en cours
Get-PrintJob -PrinterName "Mon Imprimante" | Select ID, DocumentName, JobStatus, Size, TimeSubmitted

# Ajouter une imprimante par IP
Add-PrinterPort -Name "IP_192.168.1.50" -PrinterHostAddress "192.168.1.50"
Add-Printer -Name "Imprimante Bureau" -DriverName "HP Universal Printing PCL 6" -PortName "IP_192.168.1.50"

# Sauvegarder les imprimantes (migration)
sdbinst -u "C:\\Backup\\printers.cab"
PrintBrm.exe -B -F "C:\\Backup\\printers.printerExport"
# Restaurer sur un autre PC
PrintBrm.exe -R -F "C:\\Backup\\printers.printerExport"`,
      },
      {
        title: "Drivers d'impression — gestion avancée",
        solution: [
          "Gestionnaire des pilotes : Gestion de l'imprimante (printmanagement.msc) > Pilotes",
          "Un driver corrompus peut bloquer TOUS les impressions — supprimer proprement",
          "Architecture : les drivers x86 et x64 sont séparés — pour les clients 32 bits en réseau",
          "pnputil : outil pour gérer les drivers du store Windows",
          "Mettre à jour le driver : via Windows Update ou depuis le site du fabricant",
          "Mode de compatibilité : un vieux driver peut fonctionner sur Windows 11 en forçant la compatibilité",
          "Driver universel PCL/PS : HP, Ricoh, Xerox proposent des drivers universels compatibles de nombreux modèles",
          "Netprinter (outils Sysinternals) : voir tous les mappings d'impression",
        ],
        code: `# Gestion des pilotes d'impression
# Lister les pilotes installés
Get-PrinterDriver | Select Name, InfPath, DriverVersion

# Supprimer un pilote (le service Spooler doit être arrêté ou l'imprimante non utilisée)
Remove-PrinterDriver -Name "HP Universal Printing PCL 6"

# Forcer la suppression d'un pilote récalcitrant
$PrinterName = "HP LaserJet 1320"
$DriverName = "HP LaserJet 1320 PCL 6"

# 1. Supprimer toutes les imprimantes utilisant ce driver
Get-Printer | Where-Object { $_.DriverName -eq $DriverName } | Remove-Printer

# 2. Arrêter le spooler
net stop spooler

# 3. Supprimer le driver du store
pnputil /delete-driver "oem12.inf" /uninstall  # Trouver l'inf via printmanagement.msc

# 4. Supprimer via registre si toujours bloqué
reg delete "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Print\\Environments\\Windows x64\\Drivers\\Version-3\\$DriverName" /f

# 5. Redémarrer le spooler
net start spooler

# Installer un driver depuis un INF
pnputil /add-driver "C:\\Drivers\\HP\\hpcu160u.inf" /install`,
      },
      {
        title: "Impression depuis PowerShell",
        code: `# Imprimer un fichier directement
Start-Process -FilePath "C:\\document.pdf" -Verb Print

# Imprimer sur une imprimante spécifique
$imprimante = "HP LaserJet Pro"
Start-Process -FilePath "C:\\rapport.pdf" -Verb PrintTo -ArgumentList $imprimante

# Imprimer du texte
[System.Drawing.Printing.PrintDocument]$doc = New-Object System.Drawing.Printing.PrintDocument
$doc.PrinterSettings.PrinterName = "HP LaserJet Pro"
$doc.add_PrintPage({
  param($sender, $e)
  $font = New-Object System.Drawing.Font("Arial", 12)
  $e.Graphics.DrawString("Texte à imprimer", $font, [System.Drawing.Brushes]::Black, 100, 100)
})
$doc.Print()

# Lister les imprimantes disponibles
[System.Drawing.Printing.PrinterSettings]::InstalledPrinters | ForEach-Object { $_ }

# Tester si une imprimante est accessible en réseau
$imprimante = "\\\\192.168.1.100\\Imprimante"
Test-Path $imprimante

# Récupérer les statistiques d'impression
Get-WmiObject Win32_Printer | Select Name, Status, PrinterStatus, Default |
  Where-Object { $_.Status -ne "Error" }`,
      },
    ],
  },
  {
    id: "audio-avance",
    label: "Audio — Configuration Avancée",
    icon: "Volume2",
    items: [
      {
        title: "Windows Audio — dépannage avancé",
        symptoms: "Pas de son, son crachotant, device audio non reconnu, microphone ne fonctionne pas",
        solution: [
          "Service Windows Audio : doit être en cours d'exécution (services.msc)",
          "Réinitialiser l'audio : clic droit sur l'icône volume > Résoudre les problèmes audio",
          "Périphérique par défaut : clic droit icône volume > Paramètres du son > choisir le bon périphérique",
          "Driver audio : DDU (Display Driver Uninstaller) peut aussi supprimer les drivers audio — réinstaller depuis le site du fabricant",
          "Realtek HD Audio : le driver le plus courant — peut être réinstallé depuis Realtek.com",
          "Spatial Sound (Dolby/DTS) : peut causer des problèmes — désactiver dans les propriétés du périphérique",
          "Exclusive mode : certaines apps monopolisent l'audio — décocher 'Autoriser les applications à prendre le contrôle exclusif' dans les propriétés avancées",
          "ASIO4ALL : driver ASIO gratuit pour réduire la latence audio (musique/streaming)",
        ],
        code: `# Diagnostiquer les problèmes audio
# Voir les périphériques audio
Get-PnpDevice -Class AudioEndpoint | Select FriendlyName, Status
Get-WmiObject Win32_SoundDevice | Select Name, Status, Manufacturer

# Redémarrer le service audio
Restart-Service AudioSrv, AudioEndpointBuilder -Force

# Voir le périphérique audio par défaut
Get-WmiObject -Namespace root\cimv2 -Class Win32_SoundDevice | Select Name, Status

# Lister les applications qui utilisent l'audio
Get-Process | Where-Object { $_.MainWindowTitle -ne "" } | ForEach-Object {
  $handles = $_.Handles
  if ($handles -gt 0) {
    $name = $_.Name
    # Vérifier si le processus a des connexions audio (approximatif)
    Get-AudioDevice -Verbose 2>$null
  }
}

# AudioDeviceCmdlets (module PowerShell pour l'audio)
Install-Module -Name AudioDeviceCmdlets -Scope CurrentUser
Get-AudioDevice -List                          # Lister les périphériques
Set-AudioDevice -ID "{GUID-DU-DEVICE}"        # Changer le périphérique par défaut
Set-AudioDevice -Index 1                       # Par index
Set-AudioDevice -InputIndex 2                  # Microphone par défaut

# Réinitialiser les permissions audio (microphone)
# Paramètres > Confidentialité et sécurité > Microphone > Autoriser l'accès

# Désactiver l'amélioration audio (peut causer des problèmes)
# Clic droit icône volume > Paramètres du son > Propriétés > Améliorations > Désactiver toutes les améliorations`,
      },
      {
        title: "Voicemeeter — routage audio virtuel",
        solution: [
          "Voicemeeter (Banana / Potato) : mixeur audio virtuel gratuit pour Windows (vb-audio.com)",
          "Permet de router l'audio de plusieurs sources vers plusieurs destinations",
          "Use cases : mixer micro + musique de fond pour streaming, séparer le son du jeu et du Discord",
          "Virtual Cable (inclus) : crée un câble audio virtuel entre applications",
          "Voicemeeter Banana (2 canaux virtuels) : pour la plupart des streamers",
          "Voicemeeter Potato (3 canaux virtuels) : pour les configurations complexes",
          "ASIO support : permet une latence ultra-faible pour la production musicale",
          "Patch Insert : connecter des plugins VST en temps réel dans la chaîne audio",
        ],
        code: `# Configuration Voicemeeter pour le streaming

# Schéma de routage typique :
# Micro physique → Voicemeeter Input 1
# Jeu/Applications → Virtual Cable (B1)
# Voicemeeter Output → Casque physique

# Installation :
# 1. Installer Voicemeeter Banana
# 2. Redémarrer
# 3. Dans Windows :
#    - Périphérique de lecture par défaut = Voicemeeter Input
#    - Périphérique d'enregistrement = Voicemeeter Output (écoute du jeu pour OBS)

# Dans OBS :
# Source Audio Desktop : Voicemeeter Output
# Source Microphone : Voicemeeter VAIO (micro traité)

# PowerShell — changer le périphérique par défaut vers Voicemeeter
Install-Module AudioDeviceCmdlets
$voicemeeter = Get-AudioDevice -List | Where-Object { $_.Name -like "*VoiceMeeter*" } | Select-Object -First 1
Set-AudioDevice -ID $voicemeeter.ID

# EarTrumpet (app Microsoft Store) : meilleur contrôle du volume par application`,
      },
      {
        title: "Microphones — configuration et amélioration",
        solution: [
          "Gain du microphone : clic droit icône volume > Sons > Enregistrement > Propriétés > Niveaux",
          "Boost microphone : jusqu'à +30dB (mais ajoute du bruit de fond)",
          "Noise suppression (réduction de bruit) : NVIDIA RTX Voice, RTX Broadcast (GPU Nvidia), Krisp.ai (tout GPU)",
          "Commutation rapide micro (OBS) : raccourci clavier pour mute/unmute",
          "Problème d'écho : désactiver 'Annulation d'écho acoustique' si cela cause des problèmes",
          "Microphone trop faible : vérifier le gain physique sur l'interface audio, le driver et le système",
          "NVIDIA RTX Voice : fonctionne aussi sur les GPU non-RTX avec un tweak registre",
          "Test du microphone : Paramètres > Système > Son > Tester votre microphone",
        ],
        code: `# Configurer le microphone via PowerShell
# Voir les niveaux du microphone
$mic = Get-AudioDevice -List | Where-Object { $_.Type -eq "Capture" -and $_.Default -eq $true }
Write-Host "Microphone actif : $($mic.Name)"

# Régler le volume du microphone à 80%
# (via nircmd ou AudioDeviceCmdlets)
nircmd.exe setsysvolume 52428 default_record  # 80% de 65535

# NVIDIA RTX Voice sur GPU non-RTX (hack registre)
reg add "HKLM\\SOFTWARE\\NVIDIA Corporation\\NGXCore" /v FullFeatureSupported /t REG_DWORD /d 1 /f

# Activer les améliorations audio système du microphone
# Panneau de config > Sons > Enregistrement > Microphone > Propriétés > Améliorations
# Cocher : Suppression du bruit, Annulation de l'écho acoustique

# Testeur de microphone PowerShell
Add-Type -AssemblyName System.Speech
$recognizer = New-Object System.Speech.Recognition.SpeechRecognitionEngine
$recognizer.SetInputToDefaultAudioDevice()
Write-Host "Parlez... (test microphone actif)"
# Utiliser des apps de test (windows.microsoft.com/recorder ou l'app Magnétophone)`,
      },
    ],
  },
  {
    id: "ecrans-moniteurs",
    label: "Écrans & Moniteurs",
    icon: "Monitor",
    items: [
      {
        title: "Configuration multi-moniteurs — avancé",
        solution: [
          "Disposition : Paramètres > Système > Affichage > cliquer-glisser les moniteurs pour correspondre à la disposition physique",
          "Taux de rafraîchissement : Paramètres avancés de l'affichage > Taux de rafraîchissement (sélectionner le max pris en charge)",
          "HDR : Paramètres > Système > Affichage > activer HDR (nécessite moniteur + câble compatible)",
          "Profil ICC/ICM : calibration des couleurs — Gestion des couleurs (colorcpl.exe)",
          "Display Port 1.4 / HDMI 2.1 : requis pour 4K@144Hz ou 8K@60Hz",
          "DSC (Display Stream Compression) : permet d'atteindre des résolutions/taux élevés sur câbles DP 1.4",
          "Variable Refresh Rate (VRR) : G-Sync (Nvidia) ou FreeSync (AMD) pour éliminer le déchirement d'image",
          "Little Biggy / DisplayFusion / FancyZones : gestion avancée des fenêtres multi-écran",
        ],
        code: `# Informations sur les moniteurs
Get-CimInstance Win32_DesktopMonitor | Select Name, ScreenHeight, ScreenWidth, MonitorType, PNPDeviceID
Get-CimInstance Win32_VideoController | Select Name, VideoModeDescription, CurrentRefreshRate, AdapterRAM

# Via PowerShell avec WMI complet
Get-CimInstance -Namespace root\\WMI -ClassName WmiMonitorBasicDisplayParams |
  Select InstanceName, MaxHorizontalImageSize, MaxVerticalImageSize

# Nom du moniteur (ID EDID)
Get-ItemProperty "HKLM:\\SYSTEM\\CurrentControlSet\\Enum\\DISPLAY\*\*\Device Parameters" -ErrorAction SilentlyContinue |
  Select PSChildName, @{N="Name";E={[System.Text.Encoding]::ASCII.GetString($_.EDID[8..127] | Where-Object {$_ -gt 0})}} 2>$null

# Changer la résolution et le taux de rafraîchissement via PowerShell
# Nécessite le module ChangeScreenResolution ou l'outil NirSoft
# ChangeScreenResolution.exe disponible sur nirsoft.net/utils/change_screen_resolution.html
ChangeScreenResolution.exe /width 2560 /height 1440 /frequency 165

# MultiMonitorTool (Sysinternals-like, gratuit)
# Basculer tous les moniteurs
MultiMonitorTool.exe /SetPrimary 2      # Définir moniteur 2 comme principal
MultiMonitorTool.exe /enable 2          # Activer moniteur 2
MultiMonitorTool.exe /disable 2         # Désactiver moniteur 2

# FancyZones (PowerToys) — layouts de fenêtres
winget install Microsoft.PowerToys
# Après installation : PowerToys > FancyZones > Lancer l'éditeur de disposition`,
      },
      {
        title: "Calibration des couleurs et profils ICC",
        solution: [
          "Calibration : ajuster les couleurs du moniteur pour qu'elles correspondent à la réalité",
          "Outil Windows : colorcpl.exe (Gestion des couleurs) > Calibrer l'affichage",
          "Calibrateur matériel (colorimètre) : Datacolor Spyder, X-Rite i1Display — résultats professionnels",
          "Profil sRGB : pour usage général et web",
          "Profil DCI-P3 : pour le cinéma et la photo professionnelle",
          "DisplayCAL : logiciel gratuit de calibration avancée (nécessite un colorimètre)",
          "ICC Profile : fichier .icm ou .icc qui corrée les couleurs — Gestion des couleurs > Ajouter",
          "Paramètres moniteur : contraste 80%, luminosité selon l'éclairage ambiant, température 6500K (sRGB)",
        ],
        code: `# Gestion des profils de couleurs
# Voir les profils ICC installés
Get-ItemProperty "HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ICM\\Calibration"

# Appliquer un profil ICC
# colorcpl.exe > Gestion des couleurs > Ajouter > sélectionner .icm

# Via PowerShell (API Windows Color System)
Add-Type @"
using System;
using System.Runtime.InteropServices;

public class WCS {
  [DllImport("mscms.dll", SetLastError = true)]
  public static extern bool SetColorProfileElement(IntPtr profile, uint tag, uint offset, uint size, byte[] buffer);
}
"@

# Installer un profil ICC manuellement
$profilePath = "C:\\Windows\\System32\\spool\\drivers\\color\\"
Copy-Item "C:\\Downloads\\moniteur-calibration.icm" $profilePath

# Lancer l'assistant de calibration Windows
Start-Process "dccw.exe"   # Display Color Calibration

# Gamma, luminosité, contraste Windows
# Paramètres > Système > Affichage > Paramètres d'affichage avancés > Calibrer les couleurs`,
      },
    ],
  },
  {
    id: "usb-peripheriques",
    label: "USB & Périphériques",
    icon: "Usb",
    items: [
      {
        title: "USB — dépannage et gestion",
        symptoms: "Périphérique USB non reconnu, code 43, déconnexion aléatoire, USB lent",
        solution: [
          "Code 43 : driver USB a signalé une erreur — réinstaller le driver ou vérifier le hardware",
          "Gestionnaire de périphériques > Vue > Afficher les périphériques cachés (anciens USB fantômes)",
          "USB Selective Suspend : Windows suspend les ports USB inutilisés — peut causer des déconnexions",
          "USB 3.x retrocompatible avec USB 2 : brancher un périph USB 2 sur un port USB 3 fonctionne, mais à vitesse USB 2",
          "USBDeview (NirSoft) : historique de tous les périphériques USB branchés, désinstallation facile",
          "Device Cleanup Tool : supprimer tous les fantômes de périphériques USB débranchés",
          "Hub USB alimenté : si trop de périphériques, le port USB ne fournit pas assez de courant (max 900mA USB 3.0)",
          "USBSTOR : service Windows qui gère les stockages USB — peut être désactivé par GPO en entreprise",
        ],
        code: `# Diagnostiquer les problèmes USB
# Voir tous les périphériques USB (y compris cachés)
Get-PnpDevice -Class USB | Select FriendlyName, Status, Present | Sort-Object FriendlyName

# Périphériques USB problématiques
Get-PnpDevice -Class USB | Where-Object { $_.Status -ne "OK" } | Select FriendlyName, Status, Problem, ProblemDescription

# Identifier le code d'erreur
Get-PnpDevice -PresentOnly | Where-Object { $_.Status -eq "Error" } | ForEach-Object {
  $props = Get-PnpDeviceProperty -InputObject $_
  [PSCustomObject]@{
    Device = $_.FriendlyName
    Code = ($props | Where-Object { $_.KeyName -eq "DEVPKEY_Device_ProblemCode" }).Data
  }
}

# Désactiver USB Selective Suspend (résout les déconnexions)
powercfg /setacvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
powercfg /setdcvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0
powercfg /setactive SCHEME_CURRENT

# Réinitialiser les hubs USB (sans redémarrer)
Get-PnpDevice -Class USB | Where-Object { $_.FriendlyName -like "*Hub*" } |
  ForEach-Object { Disable-PnpDevice -InstanceId $_.InstanceId -Confirm:$false; Enable-PnpDevice -InstanceId $_.InstanceId -Confirm:$false }

# Autoriser/bloquer le stockage USB (GPO/Registre)
# Bloquer :
reg add "HKLM\\SYSTEM\\CurrentControlSet\\Services\\USBSTOR" /v Start /t REG_DWORD /d 4 /f
# Autoriser :
reg add "HKLM\\SYSTEM\\CurrentControlSet\\Services\\USBSTOR" /v Start /t REG_DWORD /d 3 /f`,
      },
      {
        title: "Bluetooth — configuration et dépannage",
        symptoms: "Périphérique Bluetooth ne s'apparie pas, se déconnecte, son de mauvaise qualité",
        solution: [
          "Vérifier que le service Bluetooth est actif : services.msc > Bluetooth Support Service",
          "Réinitialiser le Bluetooth : désactiver/activer dans Paramètres > Bluetooth et appareils",
          "Supprimer et ré-appairer : Paramètres > Bluetooth > Appareils > ... > Supprimer l'appareil",
          "Driver Bluetooth : réinstaller depuis le Gestionnaire de périphériques ou le site fabricant",
          "Qualité audio Bluetooth médiocre : le profil A2DP (streaming audio) est remplacé par HFP (main-libre) quand le micro est actif",
          "aptX / aptX HD / AAC : codecs haute qualité Bluetooth (pas toujours supportés par Windows)",
          "Interférences Wi-Fi : Wi-Fi 2.4GHz et Bluetooth partagent la même bande — utiliser Wi-Fi 5GHz",
          "Bluetooth range : typiquement 10m (Classe 2), peut être réduit par les obstacles",
        ],
        code: `# Gestion Bluetooth via PowerShell
# Voir les adaptateurs Bluetooth
Get-PnpDevice -Class Bluetooth | Select FriendlyName, Status

# Voir les appareils Bluetooth appairés
Get-PnpDevice | Where-Object { $_.Class -eq "Bluetooth" -and $_.FriendlyName -notlike "*Adapter*" } |
  Select FriendlyName, Status, Present

# Désactiver/activer le Bluetooth (via pilote)
$btAdapter = Get-PnpDevice -Class Bluetooth | Where-Object { $_.FriendlyName -like "*Radio*" -or $_.FriendlyName -like "*Adapter*" } | Select-Object -First 1
Disable-PnpDevice -InstanceId $btAdapter.InstanceId -Confirm:$false
Start-Sleep 2
Enable-PnpDevice -InstanceId $btAdapter.InstanceId -Confirm:$false

# Voir les services Bluetooth
Get-Service | Where-Object { $_.Name -like "*Bluetooth*" }
Restart-Service bthserv -Force        # Redémarrer le service Bluetooth

# Forcer la reconnexion d'un appareil Bluetooth
# Via BluetoothView (NirSoft) ou les paramètres Windows
# Ou supprimer le cache Bluetooth :
# C:\\ProgramData\\Microsoft\\Windows\\SystemData\\S-1-5-18\\ReadOnly\\Bluetooth
# ATTENTION : vide tous les appairages

# Changer le profil audio Bluetooth (A2DP vs HFP)
# Sons > Enregistrement > désactiver le microphone Bluetooth
# → Force Windows à utiliser le profil A2DP (meilleure qualité audio)`,
      },
    ],
  },
];
