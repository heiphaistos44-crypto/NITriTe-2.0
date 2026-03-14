import type { KBCategory } from "../knowledgeBase";

export const kbEntrepriseAD: KBCategory[] = [
  {
    id: "active-directory",
    label: "Active Directory",
    icon: "Users",
    items: [
      {
        title: "Active Directory — concepts fondamentaux",
        solution: [
          "Active Directory (AD) : service d'annuaire Microsoft pour gérer les utilisateurs, ordinateurs et ressources d'un domaine",
          "Domaine : unité administrative (ex: entreprise.local ou entreprise.com)",
          "Contrôleur de domaine (DC) : serveur qui héberge AD DS (Active Directory Domain Services)",
          "OU (Organizational Unit) : conteneur pour organiser les objets (utilisateurs, groupes, PC)",
          "GPO (Group Policy Object) : politiques de configuration appliquées aux OUs",
          "Forêt : ensemble de domaines partageant un schéma commun",
          "Rejoindre un domaine : Paramètres > Système > Informations système > Rejoindre un domaine",
          "Outils RSAT : Remote Server Administration Tools — gérer AD depuis un PC client",
        ],
        code: `# Installer RSAT sur Windows 10/11 (admin)
Add-WindowsCapability -Online -Name Rsat.ActiveDirectory.DS-LDS.Tools~~~~0.0.1.0
Add-WindowsCapability -Online -Name Rsat.GroupPolicy.Management.Tools~~~~0.0.1.0
Add-WindowsCapability -Online -Name Rsat.DNS.Tools~~~~0.0.1.0
Add-WindowsCapability -Online -Name Rsat.DHCP.Tools~~~~0.0.1.0

# Vérifier la connexion au domaine
nltest /sc_query:entreprise.local          # Test connexion au DC
nltest /dsgetdc:entreprise.local           # Trouver le DC principal
gpresult /r                                # Politiques GPO appliquées
gpresult /h rapport.html                  # Rapport HTML complet

# Informations sur le compte courant
whoami /all                                # SID, groupes, privilèges
net user %username% /domain               # Détails compte AD
net group "Domain Admins" /domain         # Membres d'un groupe

# Forcer la synchronisation des GPO
gpupdate /force
gpupdate /force /logoff                   # Forcer + déconnecter

# Diagnostiquer les problèmes d'authentification
klist                                     # Tickets Kerberos actifs
klist purge                               # Vider le cache Kerberos
netdom query fsmo                         # Rôles FSMO du domaine`,
      },
      {
        title: "PowerShell AD — gestion des utilisateurs",
        solution: [
          "Module Active Directory : installé avec RSAT ou via Add-WindowsFeature sur un serveur",
          "Import-Module ActiveDirectory requis avant les commandes AD",
          "Préfixe des commandes : Get-AD*, New-AD*, Set-AD*, Remove-AD*",
          "Recherches LDAP avancées avec -Filter ou -LDAPFilter",
          "SecureString pour les mots de passe : ConvertTo-SecureString",
        ],
        code: `# Importer le module
Import-Module ActiveDirectory

# === UTILISATEURS ===
# Créer un utilisateur
New-ADUser -Name "Jean Dupont" \`
  -SamAccountName "j.dupont" \`
  -UserPrincipalName "j.dupont@entreprise.local" \`
  -GivenName "Jean" -Surname "Dupont" \`
  -Department "Informatique" -Title "Technicien" \`
  -Path "OU=Informatique,DC=entreprise,DC=local" \`
  -AccountPassword (ConvertTo-SecureString "P@ssw0rd!" -AsPlainText -Force) \`
  -Enabled $true \`
  -ChangePasswordAtLogon $true

# Modifier un utilisateur
Set-ADUser "j.dupont" -Department "IT" -Title "Ingénieur" -Mobile "+33612345678"

# Désactiver un compte
Disable-ADAccount "j.dupont"

# Réinitialiser le mot de passe
Set-ADAccountPassword "j.dupont" -NewPassword (ConvertTo-SecureString "NewP@ss!" -AsPlainText -Force) -Reset
Set-ADUser "j.dupont" -ChangePasswordAtLogon $true

# Débloquer un compte verrouillé
Unlock-ADAccount "j.dupont"

# Rechercher des utilisateurs
Get-ADUser -Filter {Department -eq "Informatique"} -Properties * | Select DisplayName, Title, LastLogonDate
Get-ADUser -Filter {Enabled -eq $false} | Select Name, DistinguishedName
Get-ADUser -Filter {PasswordNeverExpires -eq $true} | Select Name

# === GROUPES ===
New-ADGroup -Name "VPN-Acces" -GroupScope Universal -GroupCategory Security \`
  -Path "OU=Groupes,DC=entreprise,DC=local"
Add-ADGroupMember "VPN-Acces" -Members "j.dupont", "m.martin"
Get-ADGroupMember "VPN-Acces" | Select Name, SamAccountName
Remove-ADGroupMember "VPN-Acces" -Members "j.dupont" -Confirm:$false

# === ORDINATEURS ===
Get-ADComputer -Filter {OperatingSystem -like "*Windows 11*"} | Select Name, OperatingSystem
Move-ADObject "CN=PC-IT-001,CN=Computers,DC=entreprise,DC=local" \`
  -TargetPath "OU=Informatique,DC=entreprise,DC=local"`,
      },
      {
        title: "GPO — créer et appliquer des politiques",
        solution: [
          "Group Policy Management Console (gpmc.msc) : outil principal de gestion des GPO",
          "GPO liées à un Site, Domaine ou OU — ordre d'application : Local > Site > Domaine > OU",
          "Computer Configuration : s'applique à l'ordinateur au démarrage",
          "User Configuration : s'applique à l'utilisateur à la connexion",
          "GPO Enforced : la GPO parent prend le dessus, impossible de bloquer l'héritage",
          "WMI Filters : appliquer une GPO seulement aux ordinateurs qui remplissent un critère WMI",
          "Résultats GPO : gpresult /r (texte) ou gpresult /h rapport.html (HTML)",
          "Modélisation GPO : simuler l'effet d'une GPO sans l'appliquer (gpmc > Modélisation de stratégie de groupe)",
        ],
        code: `# GPO via PowerShell (module GroupPolicy)
Import-Module GroupPolicy

# Créer une GPO
New-GPO -Name "IT-SecuritéPostesTravail" -Domain "entreprise.local" \`
  -Comment "Politiques de sécurité pour les postes de travail"

# Lier une GPO à une OU
New-GPLink -Name "IT-SecuritéPostesTravail" -Target "OU=PostesTravail,DC=entreprise,DC=local" -Enforced No

# Configurer des paramètres de registre via GPO
Set-GPRegistryValue -Name "IT-SecuritéPostesTravail" \`
  -Key "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer" \`
  -ValueName "NoDrives" -Type DWord -Value 67108864

# Désactiver l'USB storage via GPO (registre)
Set-GPRegistryValue -Name "IT-SecuritéPostesTravail" \`
  -Key "HKLM\\SYSTEM\\CurrentControlSet\\Services\\USBSTOR" \`
  -ValueName "Start" -Type DWord -Value 4  # 4 = Disabled

# Sauvegarder toutes les GPO
Backup-GPO -All -Path "\\\\DC01\\SYSVOL\\GPO-Backups"

# Restaurer une GPO
Restore-GPO -Name "IT-SecuritéPostesTravail" -Path "\\\\DC01\\SYSVOL\\GPO-Backups"

# Rapport HTML d'une GPO
Get-GPOReport -Name "IT-SecuritéPostesTravail" -ReportType HTML -Path "rapport-gpo.html"

# WMI Filter exemple : appliquer seulement aux Windows 11
# Dans GPMC : WMI Filters > Nouveau
# Namespace : root\\CIMv2
# Query : SELECT * FROM Win32_OperatingSystem WHERE Caption LIKE "%Windows 11%"`,
      },
      {
        title: "DHCP Windows Server — gestion",
        solution: [
          "DHCP Server : attribue automatiquement les adresses IP aux clients",
          "Étendue (Scope) : plage d'adresses IP à distribuer",
          "Exclusions : plages d'IP exclues du DHCP (pour les périphériques avec IP fixe)",
          "Réservations : lier une adresse IP à une adresse MAC spécifique",
          "Options DHCP : transmettre des infos supplémentaires (gateway 003, DNS 006, domaine 015)",
          "Bail (Lease) : durée pendant laquelle l'IP est attribuée (8 jours par défaut)",
          "Failover DHCP : haute disponibilité entre deux serveurs DHCP",
          "DHCP Audit Log : C:\\Windows\\System32\\dhcp\\DhcpSrvLog-*.log",
        ],
        code: `# Gestion DHCP via PowerShell
# Voir les étendues
Get-DhcpServerv4Scope -ComputerName "DC01"

# Créer une étendue
Add-DhcpServerv4Scope -Name "Réseau Principal" \`
  -StartRange 192.168.1.100 -EndRange 192.168.1.254 \`
  -SubnetMask 255.255.255.0 -LeaseDuration (New-TimeSpan -Days 8) \`
  -ComputerName "DC01"

# Options DHCP (gateway, DNS, domaine)
Set-DhcpServerv4OptionValue -ScopeId 192.168.1.0 \`
  -Router 192.168.1.1 \`
  -DnsServer 192.168.1.10, 192.168.1.11 \`
  -DnsDomain "entreprise.local" \`
  -ComputerName "DC01"

# Réservation (IP fixe par adresse MAC)
Add-DhcpServerv4Reservation -ScopeId 192.168.1.0 \`
  -IPAddress 192.168.1.50 \`
  -ClientId "00-11-22-33-44-55" \`
  -Name "Imprimante-RDC" \`
  -ComputerName "DC01"

# Voir les baux actifs
Get-DhcpServerv4Lease -ScopeId 192.168.1.0 -ComputerName "DC01" | \`
  Select IPAddress, ClientId, HostName, LeaseExpiryTime | Sort-Object IPAddress

# Exporter/importer la config DHCP
Export-DhcpServer -File "C:\\dhcp-backup.xml" -Leases -ComputerName "DC01"
Import-DhcpServer -File "C:\\dhcp-backup.xml" -BackupPath "C:\\dhcp-backup" -ComputerName "DC02"`,
      },
    ],
  },
  {
    id: "intune-mdm",
    label: "Intune & Gestion Moderne",
    icon: "Smartphone",
    items: [
      {
        title: "Microsoft Intune — gestion des appareils MDM",
        solution: [
          "Intune : solution MDM/MAM cloud de Microsoft pour gérer les appareils Windows/Mac/iOS/Android",
          "MDM (Mobile Device Management) : gestion complète de l'appareil (config, apps, sécurité)",
          "MAM (Mobile Application Management) : gestion des applications seulement (BYOD)",
          "Enrôler un Windows 11 : Paramètres > Comptes > Accès professionnel ou scolaire > Connecter",
          "Autopilot : déploiement zero-touch des nouveaux PCs (livraison directe à l'utilisateur)",
          "Compliance Policies : règles pour vérifier la conformité (BitLocker, antivirus, OS à jour)",
          "Configuration Profiles : appliquer des paramètres (Wi-Fi, VPN, restrictions, certificats)",
          "Company Portal : app permettant à l'utilisateur d'installer les apps autorisées",
        ],
        code: `# Vérifier l'état d'enrôlement Intune
dsregcmd /status                    # État Azure AD Join et MDM
# Chercher :
# AzureAdJoined: YES
# MDMUrl: https://enrollment.manage.microsoft.com

# Forcer la synchronisation avec Intune
# Via Company Portal : Sync
# Via PowerShell (Admin)
Get-ScheduledTask -TaskName "*Push*" | Start-ScheduledTask
Start-Process -FilePath "C:\\Program Files (x86)\\Microsoft Intune Management Extension\\agentexecutor.exe"

# Diagnostics Intune
# Journaux MDM : Event Viewer > Applications and Services > Microsoft > Windows > DeviceManagement-Enterprise-Diagnostics-Provider
# Journaux IME (Intune Management Extension) : C:\\ProgramData\\Microsoft\\IntuneManagementExtension\\Logs

# Powershell — voir les scripts Intune exécutés
Get-ChildItem "C:\\ProgramData\\Microsoft\\IntuneManagementExtension\\Scripts"

# Azure AD — vérifier les groupes d'un utilisateur (nécessite module AzureAD)
Install-Module AzureAD
Connect-AzureAD
Get-AzureADUserMembership -ObjectId "user@entreprise.com" | Select DisplayName`,
      },
      {
        title: "Windows Autopilot — déploiement zero-touch",
        solution: [
          "Autopilot : permet de configurer automatiquement un nouveau PC Windows depuis le cloud",
          "L'appareil démarre, se connecte au Wi-Fi/Ethernet, télécharge la config depuis Intune",
          "L'utilisateur final n'a qu'à entrer son compte Microsoft 365 — tout le reste est automatique",
          "Prérequis : licences Microsoft 365 Business Premium ou E3/E5, Azure AD P1",
          "Enregistrer le hash matériel : script PowerShell sur le PC neuf, upload dans Intune/Partner Center",
          "Deployment Profiles : configuration OOBE, nom du PC, apps à installer",
          "Enrollment Status Page : affiche la progression de l'installation à l'utilisateur",
          "Hybrid Azure AD Join : pour les appareils dans un domaine on-premise + cloud",
        ],
        code: `# Extraire le hardware hash du PC (à faire sur le PC neuf)
# Méthode 1 : depuis Windows (pendant OOBE, Shift+F10 pour ouvrir CMD)
Install-Script -Name Get-WindowsAutopilotInfo -Force
Get-WindowsAutopilotInfo -OutputFile C:\\autopilot.csv

# Méthode 2 : OA3Tool (OEM)
# Disponible dans Windows ADK

# Méthode 3 : depuis WinPE
# Injecter le script dans l'image WinPE et exporter le CSV

# Importer dans Intune
# Portail Intune > Appareils > Windows > Inscription Windows > Appareils > Importer

# Configurer en masse via CSV
# CSV format : Device Serial Number, Windows Product ID, Hardware Hash
# Fournisseur/revendeur peut uploader directement via Partner Center

# Surveiller le déploiement
# Intune > Appareils > Superviser > Déploiements Autopilot`,
      },
      {
        title: "WSUS — gestion des mises à jour en entreprise",
        solution: [
          "WSUS (Windows Server Update Services) : gérer et approuver les mises à jour Windows en interne",
          "Évite que chaque PC télécharge les mises à jour depuis Internet (économie bande passante)",
          "Installer sur Windows Server : Gestionnaire de serveur > Ajouter des rôles > WSUS",
          "Synchroniser avec Microsoft Update : quota de temps (quotidiennement suffit)",
          "Groupes de mise à jour : créer des groupes (Test, Production) pour déployer progressivement",
          "Approuver manuellement ou automatiquement les mises à jour critiques/sécurité",
          "Configurer les clients via GPO : Computer Config > Admin Templates > Windows Components > Windows Update",
          "Supprimer les anciennes mises à jour : Console WSUS > Server Cleanup Wizard",
        ],
        code: `# Configurer les clients WSUS via GPO (paramètres clés)
# Computer Configuration > Administrative Templates > Windows Components > Windows Update

# Specify intranet Microsoft update service location
# = http://wsus-server.entreprise.local:8530

# Configure Automatic Updates
# = 4 - Auto download and schedule the install
# Scheduled install day : 0 (daily) ou 1-7 (jour semaine)
# Scheduled install time : 03:00

# Client WSUS — diagnostics
wuauclt /detectnow                    # Forcer la détection (ancien)
UsoClient StartScan                   # Windows 10/11 (nouveau)
UsoClient StartDownload               # Forcer le téléchargement
UsoClient StartInstall                # Forcer l'installation

# Vérifier le serveur WSUS configuré
reg query "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate" /v WUServer
reg query "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\WindowsUpdate" /v WUStatusServer

# Rapport WSUS via PowerShell (sur le serveur WSUS)
$wsus = Get-WsusServer -Name "wsus-server" -PortNumber 8530
$computers = $wsus.GetComputerTargets()
$computers | Select-Object FullDomainName, LastReportedStatusTime, LastSyncTime | \`
  Sort-Object FullDomainName | Export-Csv "wsus-report.csv" -NoTypeInformation`,
      },
    ],
  },
  {
    id: "securite-entreprise",
    label: "Sécurité Entreprise",
    icon: "Shield",
    items: [
      {
        title: "Audit de sécurité Active Directory",
        solution: [
          "BloodHound : visualiser les chemins d'attaque dans AD (outil offensif / défensif)",
          "PingCastle : audit automatique de la sécurité AD avec score et recommandations (gratuit)",
          "Microsoft Secure Score : tableau de bord sécurité Microsoft 365",
          "Comptes privilegiés : limiter les comptes Domain Admins, utiliser des comptes dédiés",
          "Tier Model : séparer les comptes admin (Tier 0 = DC, Tier 1 = Servers, Tier 2 = Workstations)",
          "PAW (Privileged Access Workstation) : station dédiée pour les tâches d'administration",
          "LAPS (Local Administrator Password Solution) : mots de passe admin locaux uniques et rotatifs",
          "Fine-Grained Password Policy : politique de mot de passe différente par groupe",
        ],
        code: `# PingCastle — audit AD
# Télécharger depuis pingcastle.com
PingCastle.exe --healthcheck --server entreprise.local

# LAPS — configurer les mots de passe admin locaux
# Installer LAPS sur le DC
Install-Module LAPS
Import-Module LAPS
Update-LapsADSchema
Set-LapsADComputerSelfPermission -Identity "OU=PostesTravail,DC=entreprise,DC=local"

# GPO LAPS (Computer Config > Admin Templates > LAPS)
# Enable Local Admin Password Management : Enabled
# Password Settings : longueur 16, complexe, 30 jours rotation

# Récupérer le mot de passe LAPS d'un PC
Get-LapsADPassword -Identity "PC-FINANCE-01" -AsPlainText

# Audit des connexions — activer dans Audit Policies
# Local Security Policy > Local Policies > Audit Policy
# Audit logon events : Success, Failure
# Audit account logon events : Success, Failure

# Chercher les échecs de connexion dans les journaux
Get-WinEvent -FilterHashtable @{LogName='Security'; Id=4625} |
  Select TimeCreated, @{N='User';E={$_.Properties[5].Value}}, @{N='IP';E={$_.Properties[19].Value}} |
  Group-Object User | Sort-Object Count -Descending | Select -First 20`,
      },
      {
        title: "BitLocker en entreprise — déploiement en masse",
        solution: [
          "BitLocker via GPO : Computer Config > Windows Settings > Security Settings > BitLocker Drive Encryption",
          "Escrow des clés dans AD : sauvegarder automatiquement les clés de récupération dans AD",
          "Microsoft BitLocker Administration and Monitoring (MBAM) : gestion centralisée (fin de vie, remplacé par Intune)",
          "Intune : peut activer BitLocker et sauvegarder les clés dans Azure AD",
          "Prérequis : TPM 1.2 minimum, TPM 2.0 recommandé",
          "Manage-bde : outil CLI pour gérer BitLocker",
          "Déploiement silencieux : possible avec TPM + GPO sans intervention utilisateur",
        ],
        code: `# Activer BitLocker en masse via PowerShell (avec sauvegarde clé dans AD)
# Sur chaque PC (via script ou Intune)
$BitLockerVolume = Get-BitLockerVolume -MountPoint "C:"
if ($BitLockerVolume.ProtectionStatus -eq "Off") {
  Enable-BitLocker -MountPoint "C:" -TpmProtector -UsedSpaceOnly -SkipHardwareTest
  Enable-BitLockerAutoUnlock -MountPoint "D:" -ErrorAction SilentlyContinue

  # Sauvegarder la clé de récupération dans AD
  $KeyProtector = (Get-BitLockerVolume -MountPoint "C:").KeyProtector |
    Where-Object { $_.KeyProtectorType -eq "RecoveryPassword" }
  Backup-BitLockerKeyProtector -MountPoint "C:" -KeyProtectorId $KeyProtector.KeyProtectorId
}

# Vérifier l'état BitLocker sur tous les volumes
Get-BitLockerVolume | Select MountPoint, EncryptionPercentage, ProtectionStatus, VolumeStatus

# Récupérer la clé depuis AD (Admin)
Get-ADObject -Filter {objectClass -eq "msFVE-RecoveryInformation"} -Properties msFVE-RecoveryPassword |
  Where-Object { $_.DistinguishedName -like "*PC-FINANCE-01*" } |
  Select msFVE-RecoveryPassword

# manage-bde (outil CLI)
manage-bde -status C:                          # État
manage-bde -on C: -RecoveryPassword -UsedSpaceOnly  # Activer
manage-bde -protectors -get C:                 # Voir les protecteurs (clé de récupération)
manage-bde -off C:                             # Désactiver`,
      },
    ],
  },
];
