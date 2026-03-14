import type { KBCategory } from "../knowledgeBase";

export const kbAndroidMobile: KBCategory[] = [
  {
    id: "android-windows",
    label: "Android & Windows",
    icon: "Smartphone",
    items: [
      {
        title: "ADB — Android Debug Bridge",
        solution: [
          "ADB : outil de communication PC↔Android via USB ou Wi-Fi (debugging, transfer, commandes shell)",
          "Télécharger : Android Platform Tools depuis developer.android.com",
          "Activer sur l'Android : Paramètres > À propos du téléphone > Numéro de build (tapoter 7x) > Options développeur > Débogage USB",
          "Premier usage : accepter la popup 'Autoriser le débogage USB' sur le téléphone",
          "adb devices : lister les appareils connectés",
          "adb shell : ouvrir un terminal sur l'Android",
          "adb over Wi-Fi (Android 11+) : Paramètres > Options développeur > Débogage sans fil",
          "Scrcpy : contrôler l'écran Android depuis Windows (github.com/Genymobile/scrcpy)",
        ],
        code: `# Configuration ADB
# Télécharger platform-tools et ajouter au PATH
# Ou via Android Studio / winget install Google.PlatformTools

# Commandes de base
adb devices                              # Lister les appareils
adb -s SERIAL_NUMBER shell               # Connecter à un appareil spécifique
adb shell                                # Shell interactif sur Android
adb shell pm list packages               # Lister toutes les applications
adb shell pm list packages -3            # Apps tierces seulement
adb shell pm list packages | findstr netflix  # Chercher une app

# Transfert de fichiers
adb push "C:\\fichier.txt" "/sdcard/Download/"        # Envoyer vers Android
adb pull "/sdcard/Download/photo.jpg" "C:\\Photos\\"  # Récupérer depuis Android
adb sync                                               # Sync dossiers entiers

# Captures d'écran et enregistrement
adb shell screencap /sdcard/screenshot.png
adb pull /sdcard/screenshot.png C:\\
adb shell screenrecord /sdcard/video.mp4              # Enregistrer l'écran
adb shell screenrecord --time-limit 30 /sdcard/video.mp4  # 30 secondes

# Installer/désinstaller une APK
adb install "C:\\app.apk"
adb install -r "C:\\app.apk"                          # Réinstaller (garder les données)
adb uninstall com.package.name                         # Désinstaller

# Désinstaller les bloatwares (apps constructeur) SANS ROOT
adb shell pm uninstall -k --user 0 com.package.bloatware
# -k = garder les données, --user 0 = désinstaller pour l'utilisateur courant
# ATTENTION : la liste varie selon le constructeur

# Supprimer des bloatwares courants Samsung
adb shell pm uninstall -k --user 0 com.samsung.android.app.tips
adb shell pm uninstall -k --user 0 com.microsoft.skydrive   # OneDrive Samsung
adb shell pm uninstall -k --user 0 com.facebook.appmanager

# ADB via Wi-Fi (Android 11+)
adb pair 192.168.1.X:PORT                              # Pairer (code demandé)
adb connect 192.168.1.X:PORT                           # Connecter

# ADB Wi-Fi (ancienne méthode — Android 10 et moins)
adb tcpip 5555                                         # USB → Wi-Fi mode
adb connect 192.168.1.X:5555`,
        note: "Universal Android Debloater (UAD) : interface graphique pour supprimer les bloatwares avec des recommandations de sécurité.",
      },
      {
        title: "Scrcpy — miroir et contrôle Android depuis PC",
        solution: [
          "Scrcpy : afficher et contrôler l'écran Android sur PC sans root (gratuit, open source)",
          "Installation : winget install Genymobile.scrcpy ou télécharger le ZIP depuis GitHub",
          "Prérequis : ADB activé et appareil autorisé",
          "Faible latence : ~35-70ms sur USB, 100-150ms sur Wi-Fi",
          "Fonctionnalités : copier-coller bidirectionnel, transfert de fichiers par glisser-déposer, capture d'écran",
          "Mode miroir uniquement : scrcpy --no-control (lecture seule)",
          "Enregistrer en même temps : scrcpy --record sortie.mp4",
          "Plusieurs appareils : scrcpy -s SERIAL (adb devices pour voir le serial)",
        ],
        code: `# Lancer scrcpy
scrcpy                                   # Lancer avec les paramètres par défaut
scrcpy --max-size 1080                   # Limiter la résolution (performances)
scrcpy --bit-rate 4M                     # Bitrate streaming
scrcpy --max-fps 60                      # Max 60 FPS
scrcpy --stay-awake                      # Empêcher le téléphone de se verrouiller
scrcpy --turn-screen-off                 # Éteindre l'écran du téléphone pendant le miroir
scrcpy --no-audio                        # Sans audio (réduit la latence)

# Enregistrement
scrcpy --record="C:\\captures\\session.mp4"
scrcpy --record="C:\\captures\\session.mp4" --no-display  # Enregistrer sans afficher

# Mode Wi-Fi (après adb connect)
scrcpy --tcpip=192.168.1.X:5555

# Raccourcis clavier Scrcpy
# Ctrl+H → Bouton Home
# Ctrl+B → Bouton Back
# Ctrl+A → Bouton App switcher
# Ctrl+N → Notifications
# Ctrl+P → Power (veille)
# Alt+F → Plein écran
# Ctrl+V → Coller (du presse-papier PC vers Android)
# Ctrl+Shift+V → Copier vers Android presse-papier
# Drag & Drop → Installer APK ou transférer fichier

# Lancer scrcpy minimaliste (juste le streaming, pas de contrôle)
scrcpy --no-control --max-size 720 --bit-rate 2M`,
      },
      {
        title: "Sauvegarde Android vers PC",
        solution: [
          "Sauvegarde complète ADB (sans root) : adb backup — limité sur Android 12+",
          "Photos/Vidéos : câble USB + Explorateur Windows (appareil photo) ou Google Photos",
          "MTP (Media Transfer Protocol) : protocole standard pour transfert de fichiers Android↔Windows",
          "Problème MTP : installer les drivers MediaTransferProtocol si non reconnu",
          "Android File Transfer (Mac) ou Explorateur Windows (Windows) pour MTP",
          "Applications de sauvegarde : Titanium Backup (root), Swift Backup, My Backup",
          "SMS/Contacts : SMS Backup & Restore, Google One (sauvegarde cloud complète)",
          "WhatsApp : sauvegarde locale dans /sdcard/WhatsApp/ ou cloud Google Drive",
        ],
        code: `# Sauvegarde ADB complète
adb backup -apk -shared -all -f "C:\\Backup\\android-backup.ab"
# -apk : inclure les APK
# -shared : inclure la carte SD
# -all : toutes les applications

# Restaurer
adb restore "C:\\Backup\\android-backup.ab"

# Sauvegarde des contacts (format vCard)
adb shell content query --uri content://contacts/people > contacts.txt
# Ou via app : Contacts > Exporter en VCF > /sdcard/contacts.vcf
adb pull /sdcard/contacts.vcf "C:\\Backup\\contacts.vcf"

# Sauvegarde WhatsApp (local vers PC)
adb pull "/sdcard/Android/media/com.whatsapp/WhatsApp/" "C:\\Backup\\WhatsApp\\"

# Sauvegarde photos avec date préservée
adb pull /sdcard/DCIM/ "C:\\Backup\\Photos\\"

# Synchronisation automatique via robocopy (MTP nécessite un lecteur mappé)
# Solution : utiliser Android Transfer Tool ou WinDroid pour monter le téléphone comme lecteur

# Récupérer les SMS (format XML via SMS Backup & Restore app)
# 1. Installer SMS Backup & Restore sur Android
# 2. Effectuer la sauvegarde → génère un XML sur /sdcard/
# 3. Récupérer le fichier
adb pull "/sdcard/SMSBackupRestore/" "C:\\Backup\\SMS\\"`,
      },
      {
        title: "Windows Subsystem for Android (WSA) — apps Android sur PC",
        solution: [
          "WSA permet de faire tourner des applications Android nativement sur Windows 11",
          "Nécessite : Windows 11, virtualisation activée dans le BIOS, au moins 8 Go RAM",
          "Installation officielle : via le Microsoft Store (Amazon Appstore — limité aux USA au départ)",
          "Installation manuelle : télécharger WSA depuis les archives GitHub + PowerShell",
          "Après installation : le sous-système Android aparaît dans les applications installées",
          "Les apps Android apparaissent dans le menu Démarrer comme des apps Windows normales",
          "ADB fonctionne avec WSA : adb connect 127.0.0.1:58526",
          "Performance : excellent pour les apps légères, moins bon pour les jeux 3D exigeants",
        ],
        code: `# Installer des apps Android dans WSA via ADB
# WSA doit être en cours d'exécution
adb connect 127.0.0.1:58526
adb devices                              # Vérifier la connexion
adb install "C:\\chemin\\vers\\app.apk"

# Installer Aurora Store (alternative open-source au Play Store)
# Télécharger Aurora Store APK
adb install "C:\\Aurora_Store.apk"

# Contrôler WSA
# Démarrer WSA : chercher "Android" dans le menu Démarrer
# WSA Settings (pour activer les options développeur)
adb shell settings put global development_settings_enabled 1

# Déboguer une app dans WSA
adb shell
# Puis les commandes Android habituelles...

# Paramètres WSA via registre
# Les paramètres sont dans : %LocalAppData%\\Packages\\MicrosoftCorporationII.WindowsSubsystemForAndroid_8wekyb3d8bbwe

# Désinstaller WSA complètement
Get-AppxPackage -Name "*WindowsSubsystemForAndroid*" | Remove-AppxPackage`,
      },
    ],
  },
  {
    id: "ios-windows",
    label: "iOS & Windows",
    icon: "Smartphone",
    items: [
      {
        title: "Synchronisation iPhone / iPad avec Windows",
        solution: [
          "iTunes : logiciel principal Apple pour synchroniser iPhone/iPad (Microsoft Store ou apple.com)",
          "Apple Devices (Windows 11) : nouvelle app Microsoft Store, remplace iTunes pour la gestion des appareils",
          "Photos iPhone : Windows les reconnaît comme un appareil photo (MTP/PTP)",
          "Accéder aux photos : Explorateur > Ce PC > iPhone > Stockage interne > DCIM",
          "iCloud pour Windows : synchronise Photos, Documents, Contacts, Calendrier avec Windows",
          "Partager depuis iPhone (Photos) : bouton partager > AirDrop (pas disponible sur Windows) > WhatsApp/Email",
          "iTunes Backup : sauvegarde chiffrée locale de l'iPhone complet",
          "Importer photos automatiquement : Paramètres Windows > Bluetooth et appareils > Appareil photo",
        ],
        code: `# Gérer iPhone avec iTunes CLI (limité)
# iTunes ne propose pas vraiment de CLI, mais on peut :

# Importer les photos depuis iPhone via PowerShell (MTP)
# 1. Connecter l'iPhone, déverrouiller, accepter "Faire confiance à cet ordinateur"
# 2. L'iPhone apparaît dans l'Explorateur comme appareil photo
# Accès direct :
$iPhone = Get-PnpDevice -Class "WPD" | Where-Object { $_.FriendlyName -like "*iPhone*" }

# Copier les photos via Explorateur
# \\\\iPhone\\Internal Storage\\DCIM\\100APPLE\\

# Importer automatiquement avec AutoPlay
# Paramètres > Bluetooth et appareils > Configurer AutoPlay > iPhone > Importer des photos et vidéos

# Transfert de fichiers via iTunes File Sharing
# iTunes > Appareil > Partage de fichiers > Sélectionner app > Ajouter/Sauvegarder

# Sauvegarde locale iPhone (chiffrée)
# iTunes > Récapitulatif > Sauvegardes > Ce PC > Chiffrer la sauvegarde locale

# Localisation des sauvegardes iTunes
# C:\\Users\\Username\\AppData\\Roaming\\Apple Computer\\MobileSync\\Backup\\

# Extraire des fichiers depuis une sauvegarde iTunes
# Utiliser iMazing (payant) ou iBackup Extractor (gratuit limité)
# Ou explorer manuellement (fichiers sans extension dans le dossier backup)`,
      },
      {
        title: "iCloud sur Windows — configuration",
        solution: [
          "Télécharger iCloud depuis le Microsoft Store (recommandé, toujours à jour)",
          "Ou depuis apple.com/icloud/icloud-for-windows",
          "Connexion : compte Apple ID (le même que sur l'iPhone/Mac)",
          "Fonctionnalités : Photos (bibliothèque partagée), iCloud Drive (comme OneDrive), Contacts, Calendrier, Signets",
          "iCloud Photos : toutes les photos/vidéos synchronisées entre tous les appareils Apple + Windows",
          "iCloud Drive : disque dur virtuel partagé — accessible dans l'Explorateur Windows",
          "Mot de passe iCloud : si l'authentification à deux facteurs est activée, utiliser un mot de passe d'app spécifique",
          "Problème de sync : fermer/rouvrir iCloud, vérifier la connexion Internet, l'espace disponible (5 Go gratuits)",
        ],
        code: `# Accéder aux fichiers iCloud via PowerShell
$iCloudDrive = "$env:USERPROFILE\\iCloudDrive"
if (Test-Path $iCloudDrive) {
  Get-ChildItem $iCloudDrive | Select Name, Length, LastWriteTime
} else {
  Write-Warning "iCloud Drive n'est pas configuré sur ce PC"
}

# Photos iCloud
$iCloudPhotos = "$env:USERPROFILE\\Pictures\\iCloud Photos"
if (Test-Path $iCloudPhotos) {
  $photos = Get-ChildItem -Recurse $iCloudPhotos -Include "*.jpg","*.heic","*.png","*.mov","*.mp4"
  Write-Host "Photos iCloud : $($photos.Count) fichiers"
  $totalSize = ($photos | Measure-Object Length -Sum).Sum / 1GB
  Write-Host "Taille totale : $([math]::Round($totalSize, 2)) Go"
}

# Convertir HEIC (format Apple) en JPEG
# HEIC nécessite l'installation du codec Microsoft HEVC
# Depuis le Microsoft Store : HEIF Image Extensions (gratuit)
# Ou convertir avec ImageMagick :
Get-ChildItem -Recurse "C:\\iCloud Photos" -Filter "*.heic" | ForEach-Object {
  $out = $_.FullName -replace "\.heic$", ".jpg"
  magick $_.FullName -quality 90 $out
}`,
      },
    ],
  },
];
