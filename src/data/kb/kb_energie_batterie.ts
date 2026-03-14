import type { KBCategory } from "../knowledgeBase";

export const kbEnergieBatterie: KBCategory[] = [
  {
    id: "gestion-batterie-laptop",
    label: "Batterie & Laptops",
    icon: "Battery",
    items: [
      {
        title: "Rapport de santé de la batterie Windows",
        solution: [
          "Windows génère un rapport détaillé de l'état de la batterie via powercfg",
          "Capacity Design : capacité originale en sortie d'usine",
          "Full Charge Capacity : capacité actuelle (diminue avec l'âge)",
          "Santé batterie = (Full Charge / Design Capacity) × 100",
          "En-dessous de 80% : envisager le remplacement",
          "En-dessous de 60% : remplacement urgent",
          "Cycles de charge : compteur du nombre de charges complètes effectuées",
          "BatteryInfoView (NirSoft) : alternative gratuite avec graphiques d'historique",
        ],
        code: `# Générer le rapport de batterie complet
powercfg /batteryreport /output "C:\\battery-report.html"
# Ouvrir le rapport
Start-Process "C:\\battery-report.html"

# Rapport énergie (analyse les 7 derniers jours)
powercfg /energy /output "C:\\energy-report.html" /duration 7

# Voir l'état de la batterie en temps réel
powercfg /batteryreport /duration 14  # 14 jours d'historique

# Informations batterie via PowerShell (WMI)
Get-WmiObject Win32_Battery | Select-Object \`
  @{N="Nom";E={$_.Name}},
  @{N="Status";E={$_.Status}},
  @{N="Charge estimée %";E={$_.EstimatedChargeRemaining}},
  @{N="Durée restante (min)";E={$_.EstimatedRunTime}},
  @{N="Voltage (mV)";E={$_.DesignVoltage}}

# Capacité via CIM (plus précis)
Get-CimInstance -ClassName BatteryFullChargedCapacity -Namespace root/WMI
Get-CimInstance -ClassName BatteryStaticData -Namespace root/WMI
Get-CimInstance -ClassName BatteryStatus -Namespace root/WMI

# PowerShell — calcul de santé de batterie
$design = (Get-CimInstance -Namespace ROOT/WMI -Class BatteryStaticData).DesignedCapacity
$full = (Get-CimInstance -Namespace ROOT/WMI -Class BatteryFullChargedCapacity).FullChargedCapacity
$sante = [math]::Round(($full / $design) * 100, 1)
Write-Host "Santé de la batterie : $sante%" -ForegroundColor $(if ($sante -gt 80) {"Green"} elseif ($sante -gt 60) {"Yellow"} else {"Red"})`,
        note: "Le rapport HTML powercfg contient aussi l'historique d'utilisation et les applications les plus énergivores.",
      },
      {
        title: "Préserver la durée de vie de la batterie",
        solution: [
          "Ne pas laisser la batterie à 0% ou 100% constamment — optimal : 20-80%",
          "Limiter la charge à 80% : la plupart des laptops modernes ont cette option dans les paramètres",
          "Mode Conservation (Lenovo Vantage, ASUS Armory Crate, HP Command Center, Dell Power Manager)",
          "Activer le mode économie d'énergie (icône batterie > Meilleure autonomie)",
          "Désactiver Bluetooth et Wi-Fi quand inutilisés",
          "Réduire la luminosité — c'est le premier consommateur d'énergie",
          "Hiberner plutôt que veille prolongée — la veille consomme légèrement",
          "Température : éviter les hautes températures (>35°C) qui dégradent la chimie Li-Ion",
        ],
        code: `# Paramètres économie d'énergie via PowerShell
# Activer le mode économie d'énergie
powercfg /setactive SCHEME_MAX               # Performance max
powercfg /setactive SCHEME_BALANCED          # Équilibré
powercfg /setactive SCHEME_MIN               # Économie d'énergie

# Créer un plan personnalisé
powercfg /duplicatescheme SCHEME_BALANCED "11111111-1111-1111-1111-111111111111"
powercfg /setactive "11111111-1111-1111-1111-111111111111"
powercfg /changename "11111111-1111-1111-1111-111111111111" "Laptop Travel Mode"

# Modifier les seuils veille/hibernation (en minutes)
powercfg /change standby-timeout-ac 30      # Veille sur secteur : 30 min
powercfg /change standby-timeout-dc 10      # Veille sur batterie : 10 min
powercfg /change hibernate-timeout-dc 60   # Hibernation sur batterie : 60 min
powercfg /change monitor-timeout-dc 5      # Écran off sur batterie : 5 min

# Désactiver la veille (pour les serveurs)
powercfg /change standby-timeout-ac 0
powercfg /change standby-timeout-dc 0

# Identifier les processus qui empêchent la veille
powercfg /requests                          # Voir ce qui bloque la veille
powercfg /requestsoverride process "Teams.exe" SYSTEM  # Forcer autoriser veille malgré Teams

# Voir quel programme a réveillé le PC
powercfg /lastwake
powercfg /waketimers                        # Timers qui peuvent réveiller le PC`,
      },
      {
        title: "Modern Standby (S0 Low Power Idle) — problèmes et solutions",
        symptoms: "Batterie qui se vide en veille, PC chaud en veille, durée de veille courte",
        solution: [
          "Modern Standby (S0ix) : remplace le mode S3 classique sur les laptops modernes UEFI",
          "Problème : certaines applications (Teams, antivirus) empêchent le vrai endormissement",
          "Diagnostic : powercfg /sleepstudy génère un rapport HTML détaillé",
          "Vérifier : powercfg /a — liste les états de veille supportés",
          "Forcer S3 (mode veille classique) : possible sur certains BIOS (Sleep State = S3)",
          "Désactiver le Wake on LAN : peut réveiller le PC de façon inattendue",
          "Hyper-V désactiver Modern Standby : connu pour causer des problèmes de veille",
          "Solution rapide : hiberner plutôt que veiller pour économiser la batterie",
        ],
        code: `# Diagnostic de veille
powercfg /sleepstudy /output "C:\\sleepstudy.html"
powercfg /a                                    # États supportés
powercfg /systempowerreport /output "C:\\powerreport.html"

# Identifier ce qui consomme en veille
powercfg /sleepstudy /duration 7 /output "C:\\sleepstudy7j.html"

# Désactiver les réveils réseau
# Désactiver Wake on LAN dans le gestionnaire de périphériques
$adapters = Get-NetAdapter | Where-Object { $_.Status -eq "Up" }
foreach ($adapter in $adapters) {
  Disable-NetAdapterPowerManagement -Name $adapter.Name -WakeOnMagicPacket -WakeOnPattern -ErrorAction SilentlyContinue
}

# Vérifier et désactiver les timers de réveil
powercfg /waketimers

# Forcer l'hibernation immédiate (économise plus que la veille)
shutdown /h

# Configurer le bouton fermeture couvercle
powercfg /setdcvalueindex SCHEME_CURRENT 4f971e89-eebd-4455-a8de-9e59040e7347 5ca83367-6e45-459f-a27b-476b1d01c936 3
# 3 = Hiberner quand on ferme le couvercle sur batterie`,
      },
      {
        title: "ThrottleStop — gestion avancée du CPU laptop",
        symptoms: "CPU bridé en mode batterie, laptop qui chauffe, performances réduites",
        solution: [
          "ThrottleStop : outil pour surveiller et contrôler le throttling du CPU Intel (gratuit)",
          "Power Limit Throttling (PL1/PL2) : le laptop bride le CPU pour économiser l'énergie ou limiter la chaleur",
          "PROCHOT : signal de protection thermique — processeur ralentit si trop chaud",
          "BD PROCHOT : Intel Management Engine bride le CPU pour protéger la batterie/GPU",
          "Undervolt (via ThrottleStop) : réduire la tension du CPU pour moins de chaleur à même fréquence",
          "ATTENTION : Intel a bloqué l'undervolt sur les CPU Tiger Lake+ via une mise à jour BIOS (CVE-2019-11157)",
          "HWiNFO64 : surveiller les températures et raisons de throttling en détail",
          "Pâte thermique : souvent plus efficace que les logiciels pour réduire la chaleur",
        ],
        code: `# Via PowerShell — surveiller la fréquence et le throttling
Get-Counter '\\Processor Information(_Total)\\% Processor Performance' -Continuous
Get-Counter '\\Processor Information(*)\\Processor Frequency'

# Vérifier le throttling via WMI
Get-WmiObject -Namespace root\\WMI -Class MSAcpi_ThermalZoneTemperature | ForEach-Object {
  Write-Host "Zone: $($_.InstanceName) = $(($_.CurrentTemperature - 2732) / 10)°C"
}

# Optimiser les limites de puissance via registre (attention, risqué)
# Certains laptops ont des limites dans ACPI/registre

# Désactiver le throttling adaptatif (si disponible dans le BIOS)
# BIOS > Advanced > Power & Performance > CPU Power Management Control
# Disable CPU C-States (teste seulement, mauvais pour batterie)

# Plan alimentation "Ultimate Performance" (désactive le throttling CPU)
powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61
powercfg /setactive e9a42b02-d5df-448d-aa00-03f14749eb61

# Vérifier si Ultimate Performance est disponible (Windows 10/11 Pro)
powercfg /list`,
      },
    ],
  },
  {
    id: "plans-alimentation",
    label: "Plans d'alimentation",
    icon: "Zap",
    items: [
      {
        title: "Plans d'alimentation — guide complet",
        solution: [
          "Accéder : Panneau de configuration > Options d'alimentation (ou powercfg.cpl)",
          "Équilibré : recommandé pour la plupart des usages — balance perf et consommation",
          "Performances élevées : CPU à fréquence max constante — idéal pour les postes fixes",
          "Économie d'énergie : réduit la fréquence CPU, écran off rapidement — laptop batterie",
          "Ultimate Performance : élimine les micro-délais de throttling — serveurs et workstations",
          "Plan personnalisé : dupliquer un plan existant et modifier les paramètres",
          "Paramètres avancés : délai disque dur, PCI Express link state, USB selective suspend",
          "Paramètre crucial : 'Gestion de l'alimentation du processeur' > État min/max du processeur",
        ],
        code: `# Gestion des plans via powercfg
powercfg /list                                 # Lister tous les plans
powercfg /getactivescheme                      # Plan actif
powercfg /query SCHEME_CURRENT                 # Tous les paramètres du plan actif

# Activer Ultimate Performance (Windows 10/11 Pro+)
powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61
powercfg /setactive e9a42b02-d5df-448d-aa00-03f14749eb61

# Modifier l'état max du processeur (100% = pas de throttling)
powercfg /setacvalueindex SCHEME_CURRENT SUB_PROCESSOR PROCTHROTTLEMAX 100
powercfg /setdcvalueindex SCHEME_CURRENT SUB_PROCESSOR PROCTHROTTLEMAX 80  # 80% sur batterie

# Modifier l'état min du processeur
powercfg /setacvalueindex SCHEME_CURRENT SUB_PROCESSOR PROCTHROTTLEMIN 5   # 5% min secteur
powercfg /setdcvalueindex SCHEME_CURRENT SUB_PROCESSOR PROCTHROTTLEMIN 5   # 5% min batterie

# Délai extinction disque dur (0 = jamais)
powercfg /setacvalueindex SCHEME_CURRENT 0012ee47-9041-4b5d-9b77-535fba8b1442 6738e2c4-e8a5-4a42-b16a-e040e769756e 0

# Désactiver USB Selective Suspend (résout certains problèmes de périphériques)
powercfg /setacvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0

# Appliquer les changements
powercfg /setactive SCHEME_CURRENT

# Sauvegarder un plan
powercfg /export "mon-plan.pow" {GUID-DU-PLAN}
# Importer sur un autre PC
powercfg /import "mon-plan.pow"`,
      },
      {
        title: "Hibernation — activer, configurer, dépanner",
        solution: [
          "Hibernation : sauvegarde l'état de la RAM sur le disque (hiberfil.sys) et éteint le PC",
          "Avantage : consomme 0W en hibernation vs quelques watts en veille",
          "Inconvénient : démarrage plus long qu'une veille (lecture du fichier hiberfil.sys)",
          "hiberfil.sys = environ 75% de la RAM (avec la compression) stocké sur C:\\",
          "Fast Startup Windows utilise l'hibernation du kernel (démarrage Windows rapide)",
          "Désactiver hiberfil.sys libère de l'espace disque mais désactive le démarrage rapide",
          "Hybrid Sleep : combine veille + hibernation (si coupure secteur → reprise depuis disque)",
          "Le bouton d'hibernation n'apparaît pas → activer l'hibernation d'abord",
        ],
        code: `# Activer l'hibernation
powercfg /hibernate on

# Désactiver l'hibernation (libère de l'espace disque ≈ RAM)
powercfg /hibernate off

# Taille de hiberfil.sys (réduit à 40% = Fast Startup uniquement, pas d'hibernation complète)
powercfg /h /size 40                         # 40% de la RAM (Fast Startup seulement)
powercfg /h /size 100                        # 100% (hibernation complète + Fast Startup)

# Afficher le bouton Hibernation dans le menu Arrêter
# Paramètres > Alimentation > changer le comportement du bouton > Activer des paramètres non disponibles
# Cocher "Mettre en veille prolongée"

# Via registre
reg add "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Explorer" /v ShowHibernateOption /t REG_DWORD /d 1 /f

# Configurer le bouton marche/arrêt pour hiberner
powercfg /setacvalueindex SCHEME_CURRENT SUB_BUTTONS PBUTTONACTION 2   # 2 = Hibernate
powercfg /setdcvalueindex SCHEME_CURRENT SUB_BUTTONS PBUTTONACTION 2

# Forcer l'hibernation immédiate
shutdown /h
Start-Process shutdown -ArgumentList "/h"

# Vérifier si hiberfil.sys est actif
powercfg /a    # Affiche les états disponibles (S4 = Hibernate)`,
      },
    ],
  },
];
