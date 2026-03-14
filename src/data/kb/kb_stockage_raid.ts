import type { KBCategory } from "../knowledgeBase";

export const kbStockageRaid: KBCategory[] = [
  {
    id: "partitions-disques",
    label: "Partitions & Disques",
    icon: "HardDrive",
    items: [
      {
        title: "Diskpart — gestion avancée des disques",
        solution: [
          "Diskpart : outil CLI Windows pour la gestion complète des disques et partitions",
          "Exécuter en administrateur (cmd admin > diskpart)",
          "list disk : afficher tous les disques physiques",
          "select disk N : sélectionner un disque",
          "list partition : afficher les partitions du disque sélectionné",
          "ATTENTION : clean efface toutes les données du disque sans confirmation",
          "GPT vs MBR : GPT requis pour les disques >2 To et les systèmes UEFI modernes",
          "Convert gpt : convertir MBR→GPT (ATTENTION : efface toutes les partitions !)",
        ],
        code: `# Diskpart — commandes essentielles
diskpart

# Lister les disques
list disk
# Résultat : Disk 0 (système), Disk 1 (HDD externe), etc.

# Initialiser un nouveau disque en GPT
select disk 1
clean                              # EFFACE TOUT !
convert gpt
create partition primary size=51200  # 50 Go
format fs=ntfs quick label="Data"
assign letter=D
active                             # Pour les partitions bootables

# Créer plusieurs partitions
select disk 1
clean
convert gpt
create partition primary size=204800  # 200 Go (Data)
format fs=ntfs quick label="Data"
assign letter=D
create partition primary            # Reste de l'espace
format fs=ntfs quick label="Backup"
assign letter=E

# Supprimer une partition
select disk 1
select partition 2
delete partition override           # override = force même si système

# Étendre une partition
select disk 0
select partition 3
extend size=10240                   # Ajouter 10 Go (espace libre adjacent requis)

# Réduire une partition
select disk 0
select partition 3
shrink desired=10240               # Libérer 10 Go

# Marquer une partition comme cachée
select partition 2
set id=de94bba4-06d1-4d40-a16a-bfd50179d6ac override  # Type partition Recovery

# Afficher les détails d'un volume
list volume
select volume 2
detail volume

exit`,
        note: "PowerShell équivalent : Get-Disk, Get-Partition, New-Partition, Format-Volume — plus moderne et scriptable que diskpart.",
      },
      {
        title: "PowerShell — gestion des disques et volumes",
        code: `# Lister les disques et partitions
Get-Disk | Select Number, FriendlyName, PartitionStyle, Size, OperationalStatus
Get-Partition | Select DiskNumber, PartitionNumber, DriveLetter, Size, Type
Get-Volume | Select DriveLetter, FileSystem, FileSystemLabel, Size, SizeRemaining

# Initialiser et formater un nouveau disque
$disk = Get-Disk | Where-Object { $_.PartitionStyle -eq "RAW" } | Select-Object -First 1
Initialize-Disk -InputObject $disk -PartitionStyle GPT
$partition = New-Partition -InputObject $disk -UseMaximumSize -AssignDriveLetter
Format-Volume -Partition $partition -FileSystem NTFS -NewFileSystemLabel "Données" -Confirm:$false

# Calculer l'espace libre sur tous les volumes
Get-Volume | Where-Object { $_.DriveLetter } | Select-Object \`
  DriveLetter, FileSystemLabel,
  @{N="Taille (Go)";E={[math]::Round($_.Size/1GB, 2)}},
  @{N="Libre (Go)";E={[math]::Round($_.SizeRemaining/1GB, 2)}},
  @{N="Libre %";E={[math]::Round($_.SizeRemaining/$_.Size*100, 1)}}

# Alerter si un volume est plein à plus de 90%
Get-Volume | Where-Object { $_.DriveLetter -and $_.Size -gt 0 } | ForEach-Object {
  $pct = $_.SizeRemaining / $_.Size * 100
  if ($pct -lt 10) {
    Write-Warning "ALERTE: Volume $($_.DriveLetter): à $([math]::Round(100-$pct,1))% de remplissage"
  }
}

# Vérifier et réparer un volume
Repair-Volume -DriveLetter D -Scan       # Scan uniquement
Repair-Volume -DriveLetter D -OfflineScanAndFix  # Scan + correction (volume démontable)

# Optimiser (TRIM pour SSD, défragmentation pour HDD)
Optimize-Volume -DriveLetter C -Analyze
Optimize-Volume -DriveLetter C -Retrim   # Forcer TRIM sur SSD
Optimize-Volume -DriveLetter D -Defrag -Verbose  # Défrag HDD`,
      },
      {
        title: "Conversion MBR ↔ GPT sans perte de données",
        solution: [
          "MBR (Master Boot Record) : ancien standard, max 2 To, max 4 partitions primaires",
          "GPT (GUID Partition Table) : moderne, max 9.4 Zo, 128 partitions, requis pour UEFI",
          "mbr2gpt.exe : outil Microsoft intégré Win10/11 pour convertir MBR→GPT SANS perte de données",
          "Prérequis mbr2gpt : max 3 partitions primaires sur le disque système, pas de recovery complexe",
          "Après conversion : changer le BIOS de Legacy→UEFI pour booter correctement",
          "AOMEI Partition Assistant, MiniTool Partition Wizard : alternatives GUI pour conversions complexes",
          "Pour les disques de données (non-système) : diskpart convert gpt (efface tout) ou outils tiers sans perte",
        ],
        code: `# Convertir le disque système MBR → GPT (sans perte de données !)
# Étape 1 : Valider la faisabilité
mbr2gpt /validate /disk:0 /allowFullOS

# Étape 2 : Convertir
mbr2gpt /convert /disk:0 /allowFullOS

# Étape 3 : Changer dans BIOS
# BIOS > Boot Mode : Legacy/CSM → UEFI only
# Puis redémarrer

# Si erreur : partition count exceeded (plus de 3 partitions primaires)
# Supprimer la partition de récupération temporairement :
diskpart
list partition
# Identifier la partition de récupération
select partition X
delete partition override
exit
# Puis reconvertir avec mbr2gpt
# Recréer la partition de récupération après :
reagentc /enable

# Vérifier le style de partition après conversion
Get-Disk | Select Number, PartitionStyle
# PartitionStyle doit afficher GPT

# Vérifier le mode de boot Windows après conversion
bcdedit | findstr "path"
# Doit montrer \\EFI\\Microsoft\\Boot\\bootmgfw.efi (pas bootmgr)`,
      },
      {
        title: "Analyse S.M.A.R.T. — surveiller la santé des disques",
        solution: [
          "S.M.A.R.T. (Self-Monitoring Analysis and Reporting Technology) : surveillance intégrée aux disques",
          "Attributs critiques : Reallocated Sectors Count (5), Pending Sectors (197), Uncorrectable Sectors (198)",
          "CrystalDiskInfo : interface claire, couleurs : Bon (bleu), Attention (jaune), Mauvais (rouge)",
          "HDD Sentinel : monitoring continu avec alertes",
          "Valeur et Seuil : si Valeur < Seuil → disque en échec imminent",
          "Temperature : HDD max 45°C, SSD max 70°C — au-delà les données sont menacées",
          "Reallocated Sectors > 0 : secteurs défaillants remplacés par des secteurs de réserve — signe de vieillissement",
          "NVMe : smartctl -a /dev/nvme0 (outil smartmontools — aussi pour Windows)",
        ],
        code: `# PowerShell — obtenir les infos S.M.A.R.T. basiques
Get-PhysicalDisk | Select-Object DeviceId, FriendlyName, MediaType, OperationalStatus, HealthStatus,
  @{N="Taille (Go)";E={[math]::Round($_.Size/1GB, 0)}}

# Statut de santé détaillé
Get-StorageReliabilityCounter -PhysicalDisk (Get-PhysicalDisk | Where-Object { $_.DeviceId -eq 0 })

# Smartmontools (outil avancé cross-platform)
# Télécharger : smartmontools.sourceforge.net
smartctl -a /dev/sda               # Linux / WSL
smartctl -a /dev/pd0               # Windows (disque physique 0)
smartctl -H /dev/pd0               # Bilan de santé seulement
smartctl --test=short /dev/pd0     # Test court (~2 min)
smartctl --test=long /dev/pd0      # Test long (~plusieurs heures)
smartctl -l selftest /dev/pd0      # Résultats des tests

# Surveiller via Task Scheduler (alerte si problème)
$disk = Get-PhysicalDisk | Where-Object { $_.DeviceId -eq 0 }
if ($disk.HealthStatus -ne "Healthy") {
  $body = "ALERTE: Le disque '$($disk.FriendlyName)' a un statut : $($disk.HealthStatus)"
  Write-EventLog -LogName Application -Source "DiskMonitor" -EventId 1001 -EntryType Warning -Message $body
  # Send-MailMessage -To "admin@exemple.com" -Subject "Alerte disque" -Body $body ...
}`,
      },
    ],
  },
  {
    id: "raid-storage-spaces",
    label: "RAID & Storage Spaces",
    icon: "Server",
    items: [
      {
        title: "Windows Storage Spaces — RAID logiciel",
        solution: [
          "Storage Spaces : fonctionnalité RAID logiciel intégrée à Windows (Panneau de configuration > Storage Spaces)",
          "Pool de stockage : regrouper plusieurs disques physiques",
          "Espace de stockage (virtual disk) : volume créé depuis le pool",
          "Simple (JBOD) : pas de redondance, espace maximum — perte de données si un disque tombe",
          "Miroir (RAID 1) : données dupliquées, survit à 1 panne de disque (2 disques min)",
          "Parité (RAID 5) : parité XOR, survit à 1 panne (3 disques min), plus lent en écriture",
          "Parité double (RAID 6) : survit à 2 pannes simultanées (7 disques min)",
          "Thin provisioning : créer un volume virtuel plus grand que la capacité physique (grandit à la demande)",
        ],
        code: `# Storage Spaces via PowerShell (Admin)
# Lister les disques disponibles (sans partition)
Get-PhysicalDisk | Where-Object { $_.CanPool -eq $true }

# Créer un pool de stockage
$disks = Get-PhysicalDisk | Where-Object { $_.CanPool -eq $true }
New-StoragePool -FriendlyName "MonPool" -StorageSubSystemFriendlyName "Windows Storage*" \`
  -PhysicalDisks $disks

# Créer un disque virtuel en miroir (RAID 1)
New-VirtualDisk -StoragePoolFriendlyName "MonPool" \`
  -FriendlyName "Mirror-Data" \`
  -ResiliencySettingName "Mirror" \`
  -NumberOfDataCopies 2 \`
  -UseMaximumSize

# Créer un disque virtuel en parité (RAID 5)
New-VirtualDisk -StoragePoolFriendlyName "MonPool" \`
  -FriendlyName "Parity-Archive" \`
  -ResiliencySettingName "Parity" \`
  -UseMaximumSize

# Initialiser et formater le disque virtuel
$vdisk = Get-VirtualDisk -FriendlyName "Mirror-Data"
$vdisk | Get-Disk | Initialize-Disk -PartitionStyle GPT -PassThru |
  New-Partition -AssignDriveLetter -UseMaximumSize |
  Format-Volume -FileSystem NTFS -NewFileSystemLabel "Mirror" -Confirm:$false

# Vérifier l'état du pool et des disques
Get-StoragePool | Select FriendlyName, HealthStatus, OperationalStatus, Size, AllocatedSize
Get-VirtualDisk | Select FriendlyName, ResiliencySettingName, HealthStatus, OperationalStatus
Get-PhysicalDisk | Select FriendlyName, HealthStatus, OperationalStatus, Usage

# En cas de panne — remplacer un disque
# 1. Retirer le disque défaillant
# 2. Insérer le nouveau disque
Get-PhysicalDisk | Where-Object { $_.CanPool -eq $true }  # Nouveau disque visible ?
$newDisk = Get-PhysicalDisk | Where-Object { $_.FriendlyName -eq "Nouveau SSD" }
Add-PhysicalDisk -StoragePoolFriendlyName "MonPool" -PhysicalDisks $newDisk`,
      },
      {
        title: "RAID matériel — configuration et dépannage",
        solution: [
          "RAID matériel : contrôleur RAID dédié (LSI, Broadcom, HP Smart Array) — meilleur que logiciel",
          "Le RAID matériel est transparent pour Windows — le volume RAID apparaît comme un seul disque",
          "RAID 0 (Striping) : 2 disques = 2x les perfs + 2x l'espace — AUCUNE redondance",
          "RAID 1 (Mirroring) : 2 disques identiques — même espace mais 2x la sécurité",
          "RAID 5 : minimum 3 disques, espace utile = (N-1) disques, tolère 1 panne",
          "RAID 6 : minimum 4 disques, tolère 2 pannes simultanées",
          "RAID 10 : RAID 1+0 (miroir + striping) — minimum 4 disques, meilleures perfs + redondance",
          "Hot spare : disque de remplacement automatique en cas de panne",
        ],
        code: `# Vérifier l'état du RAID via Windows
# Les contrôleurs RAID matériels ont leurs propres outils :
# HP : hpssacli / ssacli (HP Smart Storage Administrator)
# Dell : OMSA (OpenManage Server Administrator)
# LSI/Broadcom : StorCLI, MegaCLI

# StorCLI — exemple de commandes (LSI/Broadcom)
storcli /c0 show                          # Infos contrôleur
storcli /c0 /vall show                    # Tous les arrays virtuels
storcli /c0 /eall /sall show              # Tous les disques physiques
storcli /c0 /v0 show                      # Détail d'un array
storcli /c0 /v0 show rebuild              # Progression de la reconstruction

# En cas de panne RAID 5 (un disque mort)
# 1. Identifier le disque en erreur dans l'outil RAID
# 2. Remplacer le disque physique (à chaud si hot-swap supporté)
# 3. Initier la reconstruction :
storcli /c0 /e252 /s4 start rebuild       # Reconstruire depuis slot 4

# Windows — état du volume RAID (si RAID logiciel Windows)
Get-VirtualDisk | Select FriendlyName, HealthStatus, OperationalStatus, ResiliencySettingName
Get-StorageJob                             # Reconstruction en cours ?

# Forcer une vérification du volume RAID (parité)
Get-VirtualDisk -FriendlyName "Parity-Archive" | Repair-VirtualDisk`,
      },
      {
        title: "iSCSI — stockage réseau bloc",
        solution: [
          "iSCSI : protocole pour utiliser du stockage sur le réseau comme un disque SCSI local",
          "Initiateur iSCSI : côté client (Windows) — intégré dans Windows (iscsicpl.msc)",
          "Cible iSCSI : côté serveur (NAS Synology/QNAP, Windows Server, TrueNAS)",
          "LUN (Logical Unit Number) : volume iSCSI présenté au client comme un disque",
          "Avantages vs SMB : meilleure performance pour les VMs et bases de données",
          "CHAP : authentification du client iSCSI (recommandé en entreprise)",
          "Multipath I/O (MPIO) : plusieurs connexions réseau vers la même cible pour redondance et performance",
          "Prérequis réseau : réseau dédié iSCSI recommandé (VLAN ou interface séparée)",
        ],
        code: `# Configuration de l'initiateur iSCSI Windows
# Activer le service iSCSI
Start-Service MSiSCSI
Set-Service MSiSCSI -StartupType Automatic

# Configurer via iscsicpl.msc (GUI)
iscsicpl

# Configuration via PowerShell (iSCSI)
# Ajouter une cible iSCSI
New-IscsiTargetPortal -TargetPortalAddress "192.168.10.20" -TargetPortalPortNumber 3260

# Lister les cibles disponibles
Get-IscsiTarget

# Se connecter à une cible
Connect-IscsiTarget -NodeAddress "iqn.2023-01.com.synology:nas01.target1.123456"

# Connexion avec CHAP
Connect-IscsiTarget -NodeAddress "iqn.2023-01.com.synology:nas01.target1.123456" \`
  -AuthenticationType ONEWAYCHAP \`
  -ChapUsername "iscsi-client" \`
  -ChapSecret (ConvertTo-SecureString "MonSecretCHAP" -AsPlainText -Force)

# Voir les sessions actives
Get-IscsiSession

# Déconnecter
Get-IscsiSession | Disconnect-IscsiTarget -Confirm:$false

# Après connexion — le LUN apparaît comme un nouveau disque (Get-Disk)
# L'initialiser comme n'importe quel autre disque :
Initialize-Disk -Number 2 -PartitionStyle GPT
New-Partition -DiskNumber 2 -UseMaximumSize -AssignDriveLetter |
  Format-Volume -FileSystem NTFS -NewFileSystemLabel "iSCSI-LUN"`,
      },
    ],
  },
];
