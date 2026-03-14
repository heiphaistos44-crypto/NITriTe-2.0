import type { KBCategory } from "../knowledgeBase";

export const kbCloudSauvegarde: KBCategory[] = [
  {
    id: "cloud-services",
    label: "Services Cloud",
    icon: "Cloud",
    items: [
      {
        title: "OneDrive — configuration et synchronisation",
        solution: [
          "OneDrive intégré à Windows 11 — compte Microsoft requis (gratuit : 5 Go, Microsoft 365 : 1 To)",
          "Fichiers à la demande : les fichiers n'occupent pas de place locale sauf si ouverts",
          "Sauvegarder le Bureau, Documents, Images : Paramètres OneDrive > Sauvegarde des dossiers",
          "Accès hors ligne : clic droit > Toujours conserver sur l'appareil",
          "Libérer de l'espace : clic droit > Libérer de l'espace (redevient fichier en ligne)",
          "Problème de sync : clic droit icône OneDrive systray > Afficher l'état de sync",
          "Réinitialiser OneDrive (résout 90% des problèmes) : voir commande ci-dessous",
          "Personal Vault : dossier protégé par code PIN ou biométrie dans OneDrive",
        ],
        code: `# Réinitialiser OneDrive (résout les problèmes de sync)
%localappdata%\\Microsoft\\OneDrive\\OneDrive.exe /reset

# Si OneDrive ne redémarre pas automatiquement après 2 min
%localappdata%\\Microsoft\\OneDrive\\OneDrive.exe

# Désactiver OneDrive complètement (sans désinstaller)
# Via Registre (Admin)
reg add "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\OneDrive" /v DisableFileSyncNGSC /t REG_DWORD /d 1 /f

# Via GPO
# Computer Config > Admin Templates > Windows Components > OneDrive
# > Prevent the usage of OneDrive for file storage = Enabled

# Exclure des dossiers de la synchronisation
# Paramètres OneDrive > Compte > Choisir les dossiers

# OneDrive CLI (via OneDrive for Linux ou rclone)
# rclone — outil universel pour synchroniser avec le cloud
rclone config                          # Configurer une connexion cloud
rclone sync /local/path remote:bucket  # Synchroniser
rclone copy remote:bucket /local/path  # Copier depuis le cloud
rclone ls remote:                      # Lister les fichiers`,
        note: "Pour partager facilement des fichiers via OneDrive : clic droit > Partager > Copier le lien (accès lecture seule ou édition).",
      },
      {
        title: "rclone — synchroniser avec tous les services cloud",
        solution: [
          "rclone : outil CLI pour synchroniser avec 70+ services cloud (Google Drive, S3, OneDrive, Dropbox, Backblaze...)",
          "Télécharger depuis rclone.org (Windows : rclone.exe à placer dans PATH)",
          "rclone config : assistant interactif pour configurer une connexion",
          "rclone sync : synchronisation unidirectionnelle (destination = source, supprime les fichiers en trop)",
          "rclone copy : copie sans supprimer à la destination",
          "rclone mount : monter un stockage cloud comme un lecteur Windows",
          "Chiffrement intégré : créer un remote 'crypt' pour chiffrer avant d'uploader",
          "Idéal pour les sauvegardes automatisées vers le cloud",
        ],
        code: `# Installation
# Télécharger rclone.exe depuis rclone.org et ajouter au PATH

# Configurer Google Drive
rclone config
# > n (nouveau remote) > nom: gdrive > type: drive > suivre l'auth OAuth

# Commandes essentielles
rclone ls gdrive:                              # Lister
rclone lsd gdrive:                             # Dossiers seulement
rclone copy /local/path gdrive:backup/         # Copier local → cloud
rclone sync /local/path gdrive:backup/         # Synchroniser
rclone copy gdrive:backup/ /local/restore/     # Restaurer depuis cloud

# Filtres
rclone copy /local gdrive:backup --include "*.pdf" --include "*.docx"
rclone copy /local gdrive:backup --exclude "*.tmp" --exclude "node_modules/**"

# Monter comme lecteur Windows
rclone mount gdrive: G: --vfs-cache-mode full
# Démonter
taskkill /F /IM rclone.exe

# Chiffrement (remote 'crypt' sur un autre remote)
# Dans rclone config : type = crypt, remote = gdrive:encrypted, password = ...
rclone copy /local crypt:               # Upload chiffré
rclone copy crypt: /local/restore      # Télécharge et déchiffre

# Automatiser avec une tâche planifiée
rclone sync "C:\\Users\\%USERNAME%\\Documents" gdrive:backup/Documents --log-file=backup.log --log-level INFO

# Stats et vérification
rclone check /local gdrive:backup      # Vérifier l'intégrité
rclone size gdrive:                    # Taille totale`,
      },
      {
        title: "Backblaze B2 — stockage cloud économique",
        solution: [
          "Backblaze B2 : stockage cloud S3-compatible à 0.006$/Go/mois (10x moins cher qu'AWS S3)",
          "Premier 10 Go gratuits",
          "Compatible avec rclone, Cyberduck, WinSCP, clients S3",
          "Parfait pour les sauvegardes froides (archives, backups long terme)",
          "Personal Backup Backblaze : 7$/mois pour sauvegarder un PC entier (illimité)",
          "Bucket : conteneur de fichiers (comme un dossier racine)",
          "Application Key : clé API avec permissions restreintes (bonne pratique)",
          "Lifecycle Rules : supprimer automatiquement les anciens fichiers",
        ],
        code: `# Configuration rclone avec Backblaze B2
rclone config
# > n > nom: b2 > type: b2 > account: votre_account_id > key: votre_app_key

# Créer un bucket
rclone mkdir b2:mon-bucket-backup

# Synchroniser vers B2
rclone sync "C:\\Users\\User\\Documents" b2:mon-bucket-backup/Documents ^
  --transfers 8 --checkers 16 --fast-list ^
  --log-file="C:\\.logs\\rclone-b2.log" --log-level INFO

# Script PowerShell de backup automatique vers B2
$source = "C:\\Users\\$env:USERNAME"
$dest = "b2:mon-bucket-backup"
$logFile = "C:\\.logs\\backup-$(Get-Date -Format 'yyyy-MM-dd').log"

rclone sync $source $dest \`
  --exclude "AppData/**" \`
  --exclude "*.tmp" \`
  --exclude "node_modules/**" \`
  --log-file=$logFile \`
  --log-level INFO \`
  --transfers 4

if ($LASTEXITCODE -eq 0) {
  Write-Host "[OK] Backup B2 terminé $(Get-Date)" -ForegroundColor Green
} else {
  Write-Host "[ERREUR] Backup B2 échoué. Voir $logFile" -ForegroundColor Red
}`,
      },
    ],
  },
  {
    id: "strategie-sauvegarde",
    label: "Stratégie de Sauvegarde",
    icon: "HardDrive",
    items: [
      {
        title: "Règle 3-2-1 — stratégie de sauvegarde robuste",
        solution: [
          "3 copies des données : originale + 2 sauvegardes",
          "2 supports différents : disque local + NAS, ou SSD + HDD, ou local + cloud",
          "1 copie hors site : cloud ou disque stocké dans un autre lieu physique",
          "Sauvegarde locale : Windows Backup, Macrium Reflect, Veeam Agent Free",
          "Sauvegarde cloud : OneDrive/Google Drive (sync), rclone (push), Backblaze (backup complet)",
          "Tester les restaurations régulièrement — une sauvegarde non testée n'existe pas",
          "Historique des versions : conserver 30 jours minimum pour se protéger des ransomwares",
          "Immutable backups : sauvegardes en lecture seule — même un ransomware ne peut pas les chiffrer",
        ],
        code: `# Script de sauvegarde 3-2-1 complet
# Sauvegarde 1 : robocopy vers NAS local
$source = "C:\\Users\\$env:USERNAME\\Documents"
$nas = "\\\\192.168.1.100\\Backup\\$env:COMPUTERNAME"
robocopy $source $nas /MIR /Z /W:5 /R:3 /XD "*.tmp" /LOG:"C:\\.logs\\backup-local.log"

# Sauvegarde 2 : cloud via rclone (différé la nuit)
rclone sync $source "gdrive:backup/$env:COMPUTERNAME" --log-file="C:\\.logs\\backup-cloud.log"

# Vérification de l'espace disque
$minFreeGB = 10
$drive = Get-PSDrive C
$freeGB = [math]::Round($drive.Free / 1GB, 2)
if ($freeGB -lt $minFreeGB) {
  Write-Warning "ALERTE: Espace disque faible : $freeGB Go restants"
}

# Rapport de sauvegarde par email (avec Outlook)
Send-MailMessage -To "moi@exemple.com" -From "backup@exemple.com" \`
  -Subject "Rapport backup $((Get-Date).ToString('dd/MM/yyyy'))" \`
  -Body "Sauvegarde terminée. Logs: C:\\.logs\\" \`
  -SmtpServer "smtp.gmail.com" -Port 587 -UseSsl`,
      },
      {
        title: "Macrium Reflect Free — clonage et sauvegarde système",
        solution: [
          "Macrium Reflect Free : logiciel de sauvegarde/clonage image disque (macrium.com)",
          "Image disque : copie complète du disque (système + données) restaurable sur hardware différent",
          "Clone : copier un disque entier vers un autre (utile pour remplacer un HDD par un SSD)",
          "Sauvegardes différentielles : ne sauvegarde que les changements depuis la dernière full",
          "Sauvegardes incrémentielles : changements depuis la dernière sauvegarde (full ou incrémentielle)",
          "Rescue Media : créer une clé USB bootable pour restaurer même si Windows ne démarre plus",
          "Planifier : sauvegardes automatiques quotidiennes/hebdomadaires",
          "Veeam Agent Free : alternative gratuite, excellent pour les entreprises",
        ],
        code: `# Macrium Reflect CLI (reflect.exe)
# Sauvegarder le disque système en image
reflect.exe backup --task "C:\\Reflect\\backup_system.xml"

# Restaurer depuis une image
reflect.exe restore --xml "C:\\Reflect\\restore.xml"

# Cloner un disque en ligne de commande
reflect.exe clone --src 1 --dst 2  # Disque 1 → Disque 2

# Windows Backup intégré (wbadmin)
# Sauvegarde système complète
wbadmin start backup -backupTarget:\\\\NAS\\Backup -include:C: -allCritical -quiet

# Sauvegarder des fichiers spécifiques
wbadmin start backup -backupTarget:D: -include:C:\\Users -quiet

# Lister les sauvegardes disponibles
wbadmin get versions -backupTarget:\\\\NAS\\Backup

# Restaurer des fichiers
wbadmin start recovery -version:01/01/2025-00:00 -items:C:\\Users\\User\\Documents -recursive -overwrite:yes`,
      },
      {
        title: "Syncthing — synchronisation P2P sans cloud",
        solution: [
          "Syncthing : synchronisation P2P chiffrée entre appareils, aucune donnée ne passe par un serveur tiers",
          "Gratuit, open source, cross-platform (Windows, Linux, Mac, Android)",
          "Configurer via interface web locale (localhost:8384)",
          "Chaque appareil a un identifiant unique — partager cet ID pour connecter deux appareils",
          "Dossiers partagés : chaque dossier a son propre set d'appareils autorisés",
          "Types de partage : Envoyer & Recevoir (sync bidirectionnelle), Envoyer seulement, Recevoir seulement",
          "Versionnement : conserver les versions précédentes des fichiers (corbeille locale)",
          "Fonctionne sur le réseau local ET via Internet (traversée NAT automatique)",
        ],
        code: `# Installation Windows
# Télécharger syncthing-windows-amd64.zip depuis syncthing.net
# Ou via winget :
winget install SyncthingCommunity.SyncthingTray

# Démarrer Syncthing en service Windows
# Avec SyncthingTray : démarre automatiquement avec Windows

# Syncthing via ligne de commande
syncthing.exe                          # Démarrer (interface web sur localhost:8384)
syncthing.exe -no-browser             # Sans ouvrir le navigateur
syncthing.exe -generate=C:\\syncthing  # Générer la config dans un dossier

# Configuration avancée (config.xml)
# Dossier config : %APPDATA%\\Local\\Syncthing

# Syncthing API (pour automation)
$apiKey = "votre_api_key"             # Visible dans interface web > Actions > Settings
$headers = @{ "X-API-Key" = $apiKey }

# Status
Invoke-RestMethod "http://localhost:8384/rest/system/status" -Headers $headers

# Forcer une synchronisation
Invoke-RestMethod "http://localhost:8384/rest/db/scan?folder=default" -Method POST -Headers $headers

# Lister les dossiers
Invoke-RestMethod "http://localhost:8384/rest/config/folders" -Headers $headers`,
      },
      {
        title: "NAS (Synology/QNAP) — configuration Windows",
        solution: [
          "NAS (Network Attached Storage) : serveur de stockage en réseau local",
          "Synology DSM et QNAP QTS : interfaces web complètes pour gérer le NAS",
          "Accéder depuis Windows : \\\\IP_NAS dans l'Explorateur ou mapper un lecteur réseau",
          "Protocoles : SMB (Windows), NFS (Linux), AFP (Mac), iSCSI (block storage)",
          "Time Machine (Mac) / Windows Backup : configurer le NAS comme destination de sauvegarde",
          "Hyper Backup (Synology) : sauvegarder le NAS lui-même vers le cloud (AWS, Backblaze B2)",
          "RAID sur le NAS : RAID 1 (miroir) pour la redondance, RAID 5 pour espace + redondance",
          "QuickConnect (Synology) / myQNAPcloud : accéder au NAS depuis Internet sans ouvrir de ports",
        ],
        code: `# Mapper un lecteur NAS Windows
# Via Explorateur : Réseau > + Mapper un lecteur réseau
# Via PowerShell
New-PSDrive -Name "N" -PSProvider FileSystem -Root "\\\\192.168.1.100\\Media" -Persist -Credential (Get-Credential)

# Mapper au démarrage via script
$net = New-Object -ComObject WScript.Network
$net.MapNetworkDrive("N:", "\\\\192.168.1.100\\Media", $true, "SYNOLOGY\\user", "password")

# Robocopy vers NAS (sauvegarde automatique)
$source = "C:\\Users\\$env:USERNAME"
$dest = "\\\\192.168.1.100\\Backup\\PC"
robocopy $source $dest /MIR /Z /W:10 /R:3 \`
  /XD "AppData" "node_modules" ".git" \`
  /XF "*.tmp" "*.log" \`
  /LOG:"C:\\.logs\\nas-backup.log"

# Accéder au NAS via SFTP (Synology)
# Activer SSH dans DSM > Panneau de configuration > Terminal & SNMP
sftp user@192.168.1.100
# Dans sftp : get /volume1/backup/file.zip . (télécharger)
#             put localfile.zip /volume1/backup/ (upload)

# Monter un partage NFS (Linux dans VM ou WSL)
sudo mount -t nfs 192.168.1.100:/volume1/media /mnt/nas
# Permanent dans /etc/fstab :
# 192.168.1.100:/volume1/media /mnt/nas nfs defaults,_netdev 0 0`,
      },
    ],
  },
];
