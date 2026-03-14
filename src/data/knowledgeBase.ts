import type { Component } from "vue";
import { kbHardware } from "./kb/kb_hardware";
import { kbGaming } from "./kb/kb_gaming";
import { kbWindowsAvance } from "./kb/kb_windows_avance";
import { kbSecuriteAvancee } from "./kb/kb_securite_avancee";
import { kbReparation } from "./kb/kb_reparation";
import { kbLogiciels } from "./kb/kb_logiciels";
import { kbReseauAvance } from "./kb/kb_reseau_avance";
import { kbScriptsAvances } from "./kb/kb_scripts_avances";
import { kbDeveloppement } from "./kb/kb_developpement";
import { kbMultimedia } from "./kb/kb_multimedia";
import { kbVirtualisation } from "./kb/kb_virtualisation";
import { kbCloudSauvegarde } from "./kb/kb_cloud_sauvegarde";
import { kbEntrepriseAD } from "./kb/kb_entreprise_ad";
import { kbLinuxDualboot } from "./kb/kb_linux_dualboot";
import { kbEnergieBatterie } from "./kb/kb_energie_batterie";
import { kbStockageRaid } from "./kb/kb_stockage_raid";
import { kbAutomatisation } from "./kb/kb_automatisation";
import { kbErreursSysteme } from "./kb/kb_erreurs_systeme";
import { kbAndroidMobile } from "./kb/kb_android_mobile";
import { kbPeripheriquesAvances } from "./kb/kb_peripheriques_avances";
import { kbReseauEntreprise } from "./kb/kb_reseau_entreprise";
import { kbIaOutils } from "./kb/kb_ia_outils";

export interface KBItem {
  title: string;
  symptoms?: string;
  solution?: string[];
  command?: string;
  code?: string;
  note?: string;
}

export interface KBCategory {
  id: string;
  label: string;
  icon: string;
  items: KBItem[];
}

export const knowledgeBase: KBCategory[] = [
  {
    id: "reseau",
    label: "Réseau",
    icon: "Wifi",
    items: [
      {
        title: "Pas de connexion Internet",
        symptoms: "Pages web inaccessibles, icône réseau avec triangle jaune",
        solution: [
          "Vérifier le câble Ethernet ou la connexion Wi-Fi",
          "Redémarrer le routeur/modem (débrancher 30s)",
          "Vider le cache DNS : ipconfig /flushdns",
          "Réinitialiser la pile TCP/IP : netsh int ip reset",
          "Réinitialiser Winsock : netsh winsock reset",
          "Relancer le service DHCP : net stop dhcp && net start dhcp",
          "Si connexion cellulaire active : désactiver partage de connexion potentiellement conflictuel",
        ],
        command: "ipconfig /flushdns && netsh int ip reset && netsh winsock reset",
      },
      {
        title: "Wi-Fi se déconnecte fréquemment",
        symptoms: "Connexion instable, déconnexions répétées, signal fluctuant",
        solution: [
          "Mettre à jour le pilote Wi-Fi via Gestionnaire de périphériques",
          "Désactiver la gestion d'alimentation de l'adaptateur (Propriétés > Gestion de l'alimentation > décocher 'Autoriser arrêt pour économie d'énergie')",
          "Changer le canal Wi-Fi sur le routeur (éviter canaux 1/6/11 si encombrés — utiliser un scanner Wi-Fi)",
          "Vérifier les interférences (micro-ondes, Bluetooth, téléphones sans fil)",
          "Désactiver le mode économie d'énergie Wi-Fi dans Panneau de config > Options d'alimentation",
          "Forcer le protocole 5GHz si routeur dual-band et PC compatible",
          "Réinitialiser les profils réseau corrompus : netsh wlan delete profile name=*",
        ],
      },
      {
        title: "DNS lent ou ne résout pas",
        symptoms: "Sites longs à charger, erreur DNS_PROBE_FINISHED_NXDOMAIN, erreur 'Serveur introuvable'",
        solution: [
          "Changer les DNS : 8.8.8.8 / 8.8.4.4 (Google) ou 1.1.1.1 / 1.0.0.1 (Cloudflare)",
          "Via PowerShell admin : Set-DnsClientServerAddress -InterfaceAlias Wi-Fi -ServerAddresses 8.8.8.8,8.8.4.4",
          "Vider le cache DNS : ipconfig /flushdns",
          "Vérifier le fichier hosts pour entrées parasites : C:\\Windows\\System32\\drivers\\etc\\hosts",
          "Tester résolution DNS : nslookup google.com 8.8.8.8",
          "Désactiver le DNS-over-HTTPS du navigateur si problème spécifique navigateur",
        ],
        command: "ipconfig /flushdns && nslookup google.com 8.8.8.8",
      },
      {
        title: "IP en conflit ou en 169.254.x.x",
        symptoms: "IP affichée 169.254.x.x (APIPA), icône réseau avec point d'exclamation, pas d'accès réseau local",
        solution: [
          "Libérer et renouveler l'adresse IP : ipconfig /release puis ipconfig /renew",
          "Vérifier que le service DHCP est actif sur le routeur",
          "Vérifier qu'il n'y a pas deux PCs avec la même IP statique sur le réseau",
          "Passer temporairement en IP statique pour tester : définir une IP dans la plage 192.168.1.x",
          "Relancer le client DHCP Windows : net stop dhcp && net start dhcp",
          "Vérifier qu'aucun antivirus/pare-feu ne bloque le DHCP",
        ],
        command: "ipconfig /release && ipconfig /renew",
      },
      {
        title: "Pare-feu bloque une application",
        symptoms: "Application ne peut pas accéder à Internet, connection refused, timeout",
        solution: [
          "Ouvrir Sécurité Windows > Pare-feu et protection réseau > Autoriser une application",
          "Ou via Panneau de config > Système et sécurité > Pare-feu Windows > Autoriser une appli",
          "Créer une règle entrante/sortante via Règles avancées (wf.msc)",
          "Vérifier si c'est le pare-feu Windows ou un antivirus tiers",
          "Tester en désactivant temporairement le pare-feu (RISQUE : remettre après test)",
          "Vérifier les logs du pare-feu : wf.msc > Propriétés > Journal de sécurité",
        ],
        command: "wf.msc",
      },
      {
        title: "Ports réseau à connaître",
        symptoms: "Besoin de débloquer un port, diagnostic de connectivité",
        solution: [
          "80/443 : HTTP/HTTPS (navigation web)",
          "22 : SSH (connexion distante sécurisée)",
          "3389 : RDP (Bureau à distance Windows)",
          "25/587 : SMTP (envoi mail)",
          "110/993 : POP3/IMAP (réception mail)",
          "21 : FTP (transfert fichiers)",
          "53 : DNS (résolution noms)",
          "3306 : MySQL, 5432 : PostgreSQL",
          "8080 : proxy/HTTP alternatif",
          "Lister ports ouverts : netstat -an | find 'LISTENING'",
        ],
        command: 'netstat -an | find "LISTENING"',
      },
      {
        title: "Connexion lente malgré bon débit",
        symptoms: "Ping élevé, lag en gaming/vidéoconférence, téléchargements lents malgré bonne box",
        solution: [
          "Tester le débit réel : speedtest.net ou fast.com depuis le PC concerné",
          "Vérifier MTU : ping -f -l 1472 8.8.8.8 (si timeout, réduire MTU)",
          "Ajuster MTU : netsh interface ipv4 set subinterface 'Wi-Fi' mtu=1400 store=persistent",
          "Désactiver QoS Paquets si non utilisé (peut limiter à 20% la bande passante)",
          "Vérifier que la carte réseau n'est pas en mode duplex half : Propriétés adaptateur > Vitesse de la liaison",
          "Désactiver l'auto-tuning TCP si lenteur sur réseau local : netsh interface tcp set global autotuninglevel=disabled",
          "Vérifier les applications en fond qui consomment de la bande passante (Gestionnaire des tâches > Réseau)",
        ],
      },
    ],
  },
  {
    id: "performance",
    label: "Performance",
    icon: "Zap",
    items: [
      {
        title: "PC lent au démarrage",
        symptoms: "Démarrage > 2 minutes, bureau long à apparaître, HDD/SSD voyant actif longtemps",
        solution: [
          "Désactiver les programmes au démarrage : Gestionnaire des tâches > Démarrage > Désactiver les inutiles",
          "Vérifier l'espace disque (> 15% libre minimum, idéalement > 20%)",
          "Lancer un nettoyage de disque : cleanmgr",
          "Vérifier les malwares avec Windows Defender (scan complet)",
          "Passer le plan d'alimentation en 'Hautes performances'",
          "Activer le démarrage rapide : Panneau config > Options d'alimentation > Comportement bouton d'alimentation",
          "Si HDD : défragmenter (inutile sur SSD). Si SSD : vérifier santé avec CrystalDiskInfo",
          "Augmenter RAM à 16 GB si < 8 GB avec Windows 11",
        ],
        command: "cleanmgr",
      },
      {
        title: "Utilisation CPU à 100%",
        symptoms: "Ventilateurs bruyants, PC très lent, tâches qui ne répondent plus, températures élevées",
        solution: [
          "Ouvrir le Gestionnaire des tâches (Ctrl+Shift+Échap) > onglet Processus",
          "Identifier le processus consommateur (trier par CPU)",
          "Si svchost.exe : chercher dans Détails quel service est derrière (clic droit > Services associés)",
          "Si WMI (WmiPrvSE.exe) : redémarrer service WMI : net stop winmgmt && net start winmgmt",
          "Si SearchIndexer.exe : reconstruire l'index (Paramètres > Recherche Windows > Indexation avancée > Reconstruire)",
          "Si antivirus : planifier le scan en dehors des heures d'utilisation",
          "Scanner les malwares : ils sont souvent derrière une utilisation CPU anormale",
          "Vérifier les températures CPU : HWiNFO64 (si > 95°C → problème refroidissement)",
        ],
        command: 'tasklist /FI "STATUS eq running" /SOR MEMUSAGE /FO LIST',
      },
      {
        title: "Mémoire RAM saturée",
        symptoms: "Message 'mémoire insuffisante', ralentissements, swap disque intensif, PC qui rame avec peu d'onglets",
        solution: [
          "Fermer les onglets de navigateur inutiles (Chrome/Edge prend ~100MB par onglet)",
          "Identifier les processus gourmands en RAM dans le Gestionnaire des tâches",
          "Vider le cache RAM : RAMMap de Sysinternals > Empty > Empty Standby List",
          "Augmenter la mémoire virtuelle (fichier d'échange) : Paramètres système avancés > Performances > Avancé > Mémoire virtuelle",
          "Vérifier la RAM : mdsched.exe (Diagnostic mémoire Windows) pour détecter les erreurs",
          "Désactiver SysMain/Superfetch si SSD : services.msc > SysMain > Désactivé",
          "Passer à 16 GB ou 32 GB de RAM si utilisation régulière > 80%",
          "Vérifier les fuites mémoire : RAMMap ou Process Explorer de Sysinternals",
        ],
        command: "mdsched.exe",
      },
      {
        title: "Disque à 100% en permanence",
        symptoms: "Voyant disque constamment allumé, PC figé plusieurs secondes, Gestionnaire des tâches montre disque à 100%",
        solution: [
          "Désactiver SysMain (Superfetch) : services.msc > SysMain > Désactivé (surtout sur HDD)",
          "Désactiver Windows Search si non essentiel : services.msc > Windows Search > Désactivé",
          "Désactiver les notifications de Windows Defender ou planifier les scans",
          "Vérifier les mises à jour en cours : Windows Update peut utiliser 100% du disque en fond",
          "Changer le fichier d'échange vers un SSD secondaire si disponible",
          "Tester la santé du disque : chkdsk C: /scan ou CrystalDiskInfo",
          "Sur HDD : défragmenter (mais inutile si le HDD est proche de la retraite)",
          "Envisager fortement de remplacer HDD par SSD — gain de performance massif (x5 à x10)",
        ],
        command: "services.msc",
      },
      {
        title: "Jeux lagguent / FPS instables",
        symptoms: "Chutes de FPS soudaines, stuttering, lag même sur jeux anciens, FPS drops lors de nouveaux objets",
        solution: [
          "Vérifier GPU dans Gestionnaire des tâches > GPU (doit approcher 95-100% en charge normale)",
          "Activer Mode Jeu Windows : Paramètres > Gaming > Mode Jeu",
          "Désactiver Xbox Game Bar si non utilisé : Paramètres > Gaming > Xbox Game Bar",
          "Passer le plan d'alimentation en Hautes performances (désactive les économies d'énergie GPU)",
          "Activer HAGS (Hardware Accelerated GPU Scheduling) : Paramètres > Système > Affichage > Graphiques",
          "Vérifier la température GPU : ne doit pas dépasser 83°C en gaming — nettoyer les ventilateurs",
          "Mettre à jour les pilotes GPU (NVIDIA GeForce Experience ou AMD Radeon Software)",
          "Désactiver l'enregistrement en arrière-plan Xbox DVR : Paramètres > Gaming > Captures > Désactiver",
        ],
        command: "powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c",
      },
      {
        title: "Optimisation pour travail en bureau",
        symptoms: "Lenteurs sur applications Office, Teams lent, VSCode lent, beaucoup d'applications ouvertes",
        solution: [
          "Installer les mises à jour Windows + Office pour les derniers correctifs de performance",
          "Désactiver les animations : Paramètres > Accessibilité > Effets visuels > Toujours désactiver",
          "Fermer les applications inutilisées dans la barre des tâches",
          "Activer efficacité de la batterie pour Teams dans le Gestionnaire des tâches",
          "Passer à 32 GB de RAM pour le développement multi-projets",
          "Utiliser un SSD NVMe pour le disque système",
          "Placer le fichier d'échange sur le disque le plus rapide",
          "Nettoyer régulièrement avec cleanmgr et supprimer les fichiers temporaires",
        ],
      },
    ],
  },
  {
    id: "securite",
    label: "Sécurité",
    icon: "Shield",
    items: [
      {
        title: "Windows Defender désactivé",
        symptoms: "Icône bouclier avec croix rouge, notifications de sécurité manquantes, protection temps réel inactive",
        solution: [
          "Ouvrir Sécurité Windows (ms-settings:windowsdefender)",
          "Cliquer Protection contre les virus et menaces > Activer la protection en temps réel",
          "Vérifier qu'aucun antivirus tiers ne bloque Defender (conflit courant avec Avast/AVG)",
          "Si grisé, vérifier les stratégies de groupe : gpedit.msc > Modèles d'administration > Composants Windows > Antivirus Microsoft Defender",
          "Via registre admin : HKEY_LOCAL_MACHINE\\SOFTWARE\\Policies\\Microsoft\\Windows Defender > DisableAntiSpyware = 0",
          "Redémarrer le service : sc start WinDefend (admin)",
          "Si Defender reste désactivé : exécuter MalwareBytes pour chercher un malware qui aurait désactivé la protection",
        ],
        command: "sc query WinDefend",
      },
      {
        title: "Suspicion de malware / virus",
        symptoms: "Pop-ups intempestifs, redirections web, programmes inconnus, ralentissement soudain, connexions réseau anormales",
        solution: [
          "Lancer un scan complet Windows Defender (Sécurité Windows > Scan rapide/complet)",
          "Démarrer en mode sans échec (F8 ou Shift+Redémarrer) et scanner depuis là",
          "Télécharger et lancer MalwareBytes (version gratuite) pour scan complémentaire",
          "Vérifier les programmes installés récemment (Ajout/Suppression de programmes, trier par date)",
          "Vérifier les extensions de navigateur : n'en garder que les connues/légitimes",
          "Vérifier les entrées Autorun : utiliser Autoruns64 de Sysinternals",
          "Réinitialiser les navigateurs vers leurs paramètres par défaut",
          "En dernier recours : réinitialisation Windows (Paramètres > Récupération)",
        ],
        command: "start ms-settings:windowsdefender",
      },
      {
        title: "Ransomware - que faire",
        symptoms: "Fichiers chiffrés avec extension inconnue (.locked, .encrypted, etc.), demande de rançon à l'écran",
        solution: [
          "DÉCONNECTER immédiatement d'Internet (débrancher câble réseau, désactiver Wi-Fi)",
          "ÉTEINDRE le PC pour stopper le chiffrement en cours",
          "Ne PAS payer la rançon : aucune garantie de récupération",
          "Identifier le ransomware sur nomoreransom.org (site officiel avec outils de déchiffrement gratuits)",
          "Vérifier les copies fantôme Windows : vssadmin list shadows",
          "Restaurer depuis une sauvegarde externe non connectée au moment de l'infection",
          "Réinstaller Windows si système compromis",
          "Porter plainte à la cyberpolice : cybermalveillance.gouv.fr (France)",
        ],
        command: "vssadmin list shadows",
      },
      {
        title: "Activation Windows / Office sans clé",
        symptoms: "Filigrane 'Windows non activé', fonctionnalités limitées, notifications d'activation",
        solution: [
          "Méthode officielle HWID : uniquement si vous avez acheté une licence numérique liée à votre compte Microsoft",
          "Via compte Microsoft : Paramètres > Système > Activation > Connexion compte Microsoft",
          "Vérifier si votre PC OEM avait une licence intégrée : powershell (Get-WmiObject SoftwareLicensingService).OA3xOriginalProductKey",
          "Pour Office : vérifier si licence Microsoft 365 incluse avec votre école/entreprise",
          "Récupérer clé existante avec ShowKeyPlus (logiciel gratuit)",
          "Utiliser NiTriTe > Activation pour les méthodes de réactivation légitimes",
          "Acheter une licence numérique Microsoft Store si nécessaire",
        ],
        command: "slmgr /dli",
      },
      {
        title: "BitLocker - récupération / gestion",
        symptoms: "Écran de récupération BitLocker au démarrage, demande de clé de récupération à 48 chiffres",
        solution: [
          "Récupérer la clé : compte Microsoft (account.microsoft.com/devices/recoverykey)",
          "Si clé sauvegardée en AD : domaine entreprise > utilisateur > attribut msFVE-RecoveryPassword",
          "Depuis la clé USB de sauvegarde créée lors de l'activation BitLocker",
          "Suspendre BitLocker si accès encore possible : Panneau config > BitLocker > Suspendre la protection",
          "Après BIOS update ou remplacement matériel, BitLocker peut demander la clé normalement",
          "Désactiver BitLocker si non nécessaire : Panneau config > BitLocker > Désactiver",
          "Sauvegarder toujours la clé dans DEUX emplacements (compte Microsoft + clé USB)",
        ],
        command: "manage-bde -status",
      },
      {
        title: "Connexions réseau suspectes",
        symptoms: "PC lent sur réseau, activité réseau inexpliquée, suspicion de surveillance",
        solution: [
          "Lister toutes connexions actives avec PID : netstat -ano",
          "Identifier les PID suspects : Gestionnaire des tâches > colonne PID",
          "Utiliser TCPView de Sysinternals pour vue graphique temps réel",
          "Vérifier les connexions sortantes dans le pare-feu Windows : wf.msc",
          "Analyser avec Wireshark si expertise réseau (filtre par IP suspecte)",
          "Bloquer connexion suspecte : pare-feu > Règle sortante > Bloquer programme",
          "Scanner avec MalwareBytes pour confirmer si c'est un malware",
        ],
        command: "netstat -ano",
      },
    ],
  },
  {
    id: "stockage",
    label: "Stockage",
    icon: "HardDrive",
    items: [
      {
        title: "Disque plein",
        symptoms: "Barre d'espace disque rouge, impossible de sauvegarder, applications qui crashent",
        solution: [
          "Lancer le nettoyage de disque avec fichiers système : cleanmgr /d C: /sageset:1 puis cleanmgr /d C: /sagerun:1",
          "Analyser l'espace utilisé avec WinDirStat ou TreeSize Free",
          "Vider la Corbeille",
          "Supprimer les fichiers temporaires : del /q /f /s %TEMP%\\*",
          "Supprimer les anciennes installations Windows : Nettoyage disque > Nettoyer fichiers système > Installations Windows précédentes",
          "Déplacer les gros fichiers (vidéos, ISOs) sur un disque externe",
          "Désinstaller les programmes inutilisés (Ajout/Suppression)",
          "Compresser les anciens dossiers en ZIP ou utiliser Windows Compact",
        ],
        command: "cleanmgr /d C:",
      },
      {
        title: "Disque dur bruyant ou lent",
        symptoms: "Clics, grattements, temps d'accès élevés, Gestionnaire des tâches montre disque à 100%",
        solution: [
          "SAUVEGARDER IMMÉDIATEMENT toutes les données importantes vers un disque externe",
          "Lancer un diagnostic SMART : wmic diskdrive get status",
          "Utiliser CrystalDiskInfo pour voir l'état SMART détaillé (chercher 'Caution' ou 'Bad')",
          "Lancer chkdsk /f /r (programme un scan au prochain redémarrage)",
          "Si SMART indique 'Reallocated Sectors' élevé : remplacement urgent",
          "Si secteurs défaillants détectés : récupérer les données avec Recuva ou GetDataBack",
          "Vérifier câble SATA (desserré = ralentissement ou erreurs)",
          "Remplacer par un SSD SATA ou NVMe — amélioration de performance x5 minimum",
        ],
        command: "wmic diskdrive get status,model,size",
      },
      {
        title: "SSD santé et optimisation",
        symptoms: "SSD qui ralentit avec le temps, écriture lente, durée de vie inquiétante",
        solution: [
          "Vérifier la santé avec CrystalDiskInfo : regarder 'Total Host Writes' et '% de vie restante'",
          "S'assurer que TRIM est activé : fsutil behavior query DisableDeleteNotify (0 = TRIM actif)",
          "Activer TRIM si désactivé : fsutil behavior set DisableDeleteNotify 0",
          "NE PAS défragmenter un SSD (inutile et réduit la durée de vie)",
          "Garder 15-20% d'espace libre pour permettre l'over-provisioning",
          "Mettre à jour le firmware du SSD depuis le site fabricant (Samsung Magician, etc.)",
          "Désactiver l'hibernation si espace critique : powercfg /h off",
          "Vérifier que le SSD est en mode AHCI dans le BIOS (pas IDE)",
        ],
        command: "fsutil behavior query DisableDeleteNotify",
      },
      {
        title: "Partitions et gestion de l'espace",
        symptoms: "Partition trop petite, espace non alloué, besoin de redimensionner",
        solution: [
          "Ouvrir Gestion des disques : diskmgmt.msc",
          "Rétrécir une partition : clic droit > Réduire le volume",
          "Étendre une partition : clic droit > Étendre le volume (espace non alloué doit être adjacent à droite)",
          "Si espace non adjacent : utiliser MiniTool Partition Wizard (gratuit) ou AOMEI Partition Assistant",
          "Convertir MBR en GPT sans perte de données : via MBR2GPT.exe (Windows 10/11)",
          "Créer partition de récupération séparée pour images système",
          "Attention : toujours sauvegarder avant toute opération sur les partitions",
        ],
        command: "diskmgmt.msc",
      },
      {
        title: "RAID - configuration et récupération",
        symptoms: "Disque RAID défaillant, rebuild RAID, performance RAID dégradée",
        solution: [
          "RAID 0 (striping) : performance maximale, zéro tolérance aux pannes — 1 disque mort = tout perdu",
          "RAID 1 (mirroring) : redondance totale, 50% de l'espace disponible — résiste à 1 panne",
          "RAID 5 : équilibre performance/redondance, minimum 3 disques, résiste à 1 panne",
          "RAID 10 : RAID 1+0, résiste à plusieurs pannes (1 par paire), coûteux",
          "Pour RAID logiciel Windows : Gestionnaire des disques ou Espaces de stockage",
          "Vérifier l'état RAID : Panneau Intel RST ou AMD RAIDXpert selon contrôleur",
          "Si disque RAID défaillant : remplacer physiquement, REBUILD automatique démarre",
          "Toujours sauvegarder externement même avec RAID — ce n'est PAS un backup",
        ],
      },
    ],
  },
  {
    id: "affichage",
    label: "Affichage",
    icon: "Monitor",
    items: [
      {
        title: "Écran noir au démarrage",
        symptoms: "PC démarre mais écran reste noir, curseur visible ou non, PC répond mais pas d'image",
        solution: [
          "Essayer Ctrl+Alt+Suppr puis Gestionnaire des tâches (parfois Bureau chargé mais invisible)",
          "Win+P pour cycler entre modes d'affichage (HDMI/DP peut avoir basculé sur autre sortie)",
          "Tester avec un autre câble HDMI/DisplayPort",
          "Tester sur un autre moniteur pour isoler le problème",
          "Démarrer en mode sans échec (F8 ou Shift+Redémarrer > Dépanner > Mode sans échec)",
          "Mettre à jour le pilote graphique depuis le mode sans échec",
          "Réinitialiser l'affichage : Win+Ctrl+Shift+B (resettle le driver graphique)",
          "Vérifier si le PC a un iGPU et essayer de brancher sur la sortie de la carte mère",
        ],
        command: "dxdiag",
      },
      {
        title: "Résolution incorrecte ou moniteur non détecté",
        symptoms: "Image floue, éléments trop grands, second moniteur absent, résolution limitée",
        solution: [
          "Clic droit bureau > Paramètres d'affichage > Détecter",
          "Sélectionner la résolution recommandée pour chaque moniteur",
          "Mettre à jour le pilote graphique (NVIDIA/AMD/Intel)",
          "Vérifier le câble (HDMI 2.0 requis pour 4K@60Hz, DisplayPort 1.4 pour 4K@144Hz)",
          "Vérifier le port sur la carte graphique (certains ports DP peuvent être désactivés si iGPU actif)",
          "Installer les pilotes du moniteur si disponibles sur le site fabricant",
          "Pour multi-écrans : vérifier l'orientation et la disposition dans Paramètres d'affichage",
          "Désactiver puis réactiver l'adaptateur graphique dans Gestionnaire des périphériques",
        ],
        command: "start ms-settings:display",
      },
      {
        title: "Écran qui scintille / artefacts graphiques",
        symptoms: "Lignes horizontales, tearing, glitches visuels, écran qui clignote",
        solution: [
          "Si scintillement dans Gestionnaire des tâches : c'est le pilote ou une application — sinon c'est le matériel",
          "Mettre à jour le pilote graphique (ou RÉTROGRADER si problème après mise à jour)",
          "Désinstaller proprement l'ancien pilote avec DDU (Display Driver Uninstaller) en mode sans échec puis réinstaller",
          "Tester en Mode sans échec : si ça scintille encore → problème matériel (GPU ou câble)",
          "Vérifier le câble DisplayPort/HDMI — les câbles DP sont particulièrement sensibles",
          "Tester avec un autre câble ou un autre port",
          "Vérifier la température GPU (HWiNFO64) — artefacts à haute température = GPU en fin de vie",
          "Réappliquer la pâte thermique GPU si artefacts sous charge uniquement",
        ],
      },
      {
        title: "HDR et taux de rafraîchissement",
        symptoms: "HDR qui lave les couleurs, taux de rafraîchissement bloqué à 60Hz au lieu de 144Hz",
        solution: [
          "Vérifier le câble : HDMI 2.1 ou DisplayPort 1.4+ requis pour 4K@144Hz HDR",
          "Paramètres > Système > Affichage > Paramètres d'affichage avancés > Fréquence de rafraîchissement",
          "Pour HDR : Paramètres > Système > Affichage > HDR > activer",
          "Configurer le calibrage HDR dans les paramètres Windows",
          "Si HDR lave les couleurs : ajuster luminosité max de l'écran dans les paramètres HDR Windows",
          "Certains moniteurs nécessitent activation HDR côté moniteur ET côté Windows",
          "NVIDIA : forcer HDR dans Panneau NVIDIA > Résolution > Utiliser paramètres NVIDIA",
          "Vérifier bandwidth du port (DP 1.4 = 32.4 Gbps, HDMI 2.1 = 48 Gbps)",
        ],
      },
    ],
  },
  {
    id: "audio",
    label: "Audio",
    icon: "Volume2",
    items: [
      {
        title: "Pas de son",
        symptoms: "Icône son avec croix rouge, aucun son des haut-parleurs ou du casque",
        solution: [
          "Vérifier le volume et le périphérique de sortie (clic droit icône son > Ouvrir les paramètres de son)",
          "Vérifier que le bon périphérique de sortie est sélectionné (ex: haut-parleurs vs HDMI)",
          "Lancer l'utilitaire de résolution des problèmes audio (clic droit icône son > Résoudre les problèmes audio)",
          "Réinstaller le pilote audio : Gestionnaire de périphériques > Contrôleurs audio > Désinstaller > Réinstaller",
          "Redémarrer le service audio : net stop audiosrv && net start audiosrv",
          "Vérifier les sons via le panneau de configuration du BIOS (parfois audio désactivé en BIOS)",
          "Si USB/HDMI audio : vérifier que le périphérique est bien reconnu dans Gestionnaire des périphériques",
        ],
        command: "net stop audiosrv && net start audiosrv",
      },
      {
        title: "Son crachotant / distordu",
        symptoms: "Grésillements, craquements, son métallique, qualité audio dégradée",
        solution: [
          "Mettre à jour les pilotes audio (Realtek, IDT, etc.) depuis le site fabricant de la carte mère",
          "Désactiver les effets audio Windows : clic droit périphérique de sortie > Propriétés > Améliorations > Désactiver tous",
          "Changer le format audio : Propriétés > Avancé > Format par défaut (essayer 16 bits, 44100 Hz)",
          "Vérifier le câble audio (jack 3.5mm endommagé = craquements)",
          "Tester un autre port audio (façade vs arrière du PC — les ports façade sont souvent de moins bonne qualité)",
          "Désactiver l'accélération matérielle dans les applications (Chrome, etc.)",
          "Si USB : changer de port USB ou utiliser un hub USB alimenté",
        ],
        command: "mmsys.cpl",
      },
      {
        title: "Microphone non reconnu / qualité faible",
        symptoms: "Microphone absent dans les paramètres, voix trop faible, écho ou larsen",
        solution: [
          "Vérifier Paramètres > Système > Son > Entrée : le micro doit apparaître",
          "Vérifier les autorisations : Paramètres > Confidentialité > Microphone > Autoriser",
          "Vérifier le niveau d'entrée dans le panneau son > Enregistrement > Propriétés du micro > Niveaux",
          "Désactiver 'Suppression du bruit' si elle crée des artefacts",
          "Pour l'écho : activer l'annulation d'écho dans les paramètres de la visioconférence",
          "Tester avec un autre port jack ou USB",
          "Sur casque avec micro séparé : vérifier la prise combo (4 segments) vs prise séparée (3 segments)",
          "Mettre à jour les pilotes audio pour améliorer le traitement du signal",
        ],
        command: "mmsys.cpl",
      },
    ],
  },
  {
    id: "peripheriques",
    label: "Périphériques",
    icon: "Usb",
    items: [
      {
        title: "Périphérique USB non reconnu",
        symptoms: "Message 'Périphérique USB non reconnu', pas de détection, son de déconnexion/reconnexion en boucle",
        solution: [
          "Essayer un autre port USB (préférer ports USB 3.0 directement sur la carte mère, pas les hubs)",
          "Redémarrer le PC avec le périphérique débranché",
          "Mettre à jour les pilotes USB : Gestionnaire de périphériques > Contrôleurs USB > Mettre à jour",
          "Désinstaller le périphérique dans le Gestionnaire, redémarrer, rebrancher",
          "Désactiver la gestion d'alimentation USB : Gestionnaire > Contrôleurs USB > Propriétés > Gestion alimentation > Décocher",
          "Vérifier l'alimentation USB (hub alimenté nécessaire pour disques externes)",
          "Tester sur un autre PC pour confirmer si c'est le périphérique ou le PC",
          "Désactiver puis réactiver le port USB dans le Gestionnaire des périphériques",
        ],
      },
      {
        title: "Imprimante ne fonctionne pas",
        symptoms: "Impression bloquée, imprimante hors ligne, file d'impression bloquée",
        solution: [
          "Vérifier la connexion physique (USB) ou réseau (Wi-Fi/Ethernet)",
          "Redémarrer le spooler d'impression : net stop spooler && net start spooler",
          "Vider la file d'impression : arrêter spooler > supprimer C:\\Windows\\System32\\spool\\PRINTERS\\* > redémarrer spooler",
          "Supprimer et rajouter l'imprimante : Paramètres > Bluetooth et appareils > Imprimantes",
          "Mettre à jour les pilotes depuis le site du fabricant (HP, Canon, Epson, etc.)",
          "Si réseau : vérifier l'adresse IP de l'imprimante (parfois elle change si DHCP)",
          "Lancer l'utilitaire de résolution des problèmes d'imprimante",
          "Vérifier que le service 'Spouleur d'impression' est bien démarré (services.msc)",
        ],
        command: 'net stop spooler && del /Q /F /S "%systemroot%\\System32\\spool\\PRINTERS\\*.*" && net start spooler',
      },
      {
        title: "Clavier / souris qui ne fonctionne plus",
        symptoms: "Clavier ou souris sans réponse, certaines touches ne marchent pas, pointeur qui saute",
        solution: [
          "Débrancher et rebrancher (attendre 10s pour USB)",
          "Tester sur un autre port USB ou PS/2",
          "Vérifier les piles (souris/clavier sans fil)",
          "Vérifier la fréquence de sondage USB si souris gaming saute (réduire à 500Hz si 1000Hz pose problème)",
          "Désinstaller et réinstaller le périphérique dans Gestionnaire des périphériques",
          "Mettre à jour les pilotes HID (Human Interface Device)",
          "Pour clavier : vérifier si certaines touches bloquées physiquement (nettoyage)",
          "Si clavier PS/2 : tester en mode BIOS (si fonctionne en BIOS mais pas Windows → problème pilote)",
        ],
      },
      {
        title: "Webcam non détectée",
        symptoms: "Webcam absente dans les applications de visio, image noire, erreur d'accès",
        solution: [
          "Vérifier Paramètres > Confidentialité > Caméra > Autoriser les applications à accéder à votre caméra",
          "Vérifier Gestionnaire des périphériques > Appareils d'images",
          "Mettre à jour les pilotes de la webcam",
          "Fermer toutes les autres applications qui pourraient utiliser la webcam (une seule à la fois en général)",
          "Tester dans la caméra Windows intégrée pour isoler le problème",
          "Désinstaller et réinstaller les pilotes depuis le site fabricant",
          "Si webcam intégrée laptop : vérifier le switch physique ou touche Fn de désactivation caméra",
          "Vérifier que la caméra n'est pas désactivée dans le BIOS",
        ],
      },
    ],
  },
  {
    id: "bsod",
    label: "Écrans Bleus (BSOD)",
    icon: "AlertTriangle",
    items: [
      {
        title: "CRITICAL_PROCESS_DIED (0x000000EF)",
        symptoms: "Arrêt brutal avec code 0x000000EF, redémarrage automatique",
        solution: [
          "Lancer SFC pour réparer les fichiers système corrompus : sfc /scannow",
          "Lancer DISM si SFC échoue : DISM /Online /Cleanup-Image /RestoreHealth",
          "Vérifier la RAM avec Windows Memory Diagnostic : mdsched.exe",
          "Mettre à jour tous les pilotes (chipset, stockage, réseau en priorité)",
          "Vérifier les logs événements : eventvwr > Journaux Windows > Système (chercher Critical/Error)",
          "Désinstaller les programmes récemment installés au moment des BSOD",
          "Restaurer un point de restauration antérieur aux BSOD",
          "Vérifier la santé du disque système avec CrystalDiskInfo",
        ],
        command: "sfc /scannow",
      },
      {
        title: "MEMORY_MANAGEMENT (0x0000001A)",
        symptoms: "Code 0x0000001A, souvent après ajout de RAM ou mise à jour",
        solution: [
          "Lancer Windows Memory Diagnostic : mdsched.exe (redémarrage requis)",
          "Tester avec MemTest86 (USB bootable) pour test RAM approfondi — laisser tourner 4h minimum",
          "Tester les barrettes RAM une par une pour identifier la fautive",
          "Vérifier les slots RAM (nettoyage des contacts à l'alcool isopropylique)",
          "Essayer de passer en mono-canal (une seule barrette) pour tester",
          "Vérifier que la RAM est dans les bons slots (souvent A2+B2 pour dual-channel)",
          "Mettre à jour les pilotes graphiques et chipset",
          "Vérifier que le profil XMP/EXPO est bien activé dans le BIOS (si RAM OC)",
        ],
        command: "mdsched.exe",
      },
      {
        title: "DRIVER_IRQL_NOT_LESS_OR_EQUAL (0x000000D1)",
        symptoms: "Code 0x000000D1, souvent lié à un pilote défaillant, peut survenir au démarrage ou à l'arrêt",
        solution: [
          "Analyser le fichier minidump : %SystemRoot%\\Minidump (utiliser WhoCrashed ou WinDbg)",
          "Identifier le pilote fautif mentionné dans le dump (souvent .sys file)",
          "Mettre à jour ou réinstaller le pilote incriminé",
          "Désinstaller les logiciels récemment installés (antivirus, VPN, outils système)",
          "Utiliser l'outil Driver Verifier pour diagnostiquer : verifier.exe > Paramètres standard > Sélectionner tous les pilotes",
          "Désinstaller proprement les pilotes GPU avec DDU puis réinstaller",
          "Mettre à jour le BIOS de la carte mère",
          "Vérifier la compatibilité des drivers avec la version Windows",
        ],
        command: "verifier.exe",
      },
      {
        title: "PAGE_FAULT_IN_NONPAGED_AREA (0x00000050)",
        symptoms: "Code 0x50, souvent lié à RAM défaillante ou pilote accédant à une zone mémoire invalide",
        solution: [
          "Tester la RAM avec MemTest86",
          "Vérifier les pilotes récemment installés",
          "Lancer SFC /scannow et DISM",
          "Analyser le dump avec WhoCrashed pour identifier le module fautif",
          "Vérifier la santé du disque : chkdsk C: /f",
          "Désactiver l'overclocking RAM (XMP) temporairement pour tester",
          "Vérifier la température RAM avec HWiNFO64 (RAM trop chaude = erreurs)",
          "Si sur SSD NVMe récent : mettre à jour le firmware du SSD",
        ],
        command: "sfc /scannow && DISM /Online /Cleanup-Image /RestoreHealth",
      },
      {
        title: "SYSTEM_SERVICE_EXCEPTION (0x0000003B)",
        symptoms: "Code 0x3B, peut survenir aléatoirement, souvent lié à des pilotes corrompus",
        solution: [
          "Analyser le dump : WhoCrashed ou WinDbg (chercher le .sys fautif)",
          "Mettre à jour en priorité : pilotes chipset, réseau, son, Bluetooth",
          "Désinstaller les logiciels de virtualisation (VMware, VirtualBox) si récemment installés",
          "Mettre à jour ou désinstaller l'antivirus (souvent impliqué dans les SYSTEM_SERVICE_EXCEPTION)",
          "Lancer SFC /scannow puis DISM /Online /Cleanup-Image /RestoreHealth",
          "Vérifier les mises à jour Windows (un KB peut parfois causer ce BSOD)",
          "Rétrograder un pilote récemment mis à jour si le problème a commencé après",
          "Créer un rapport de diagnostic : perfmon /report",
        ],
      },
      {
        title: "Analyse complète d'un BSOD",
        symptoms: "BSOD récurrents, besoin d'identifier la cause racine",
        solution: [
          "Activer la création de dump complet : Paramètres système avancés > Démarrage et récupération > Vidage mémoire complet",
          "Analyser avec WhoCrashed (gratuit, analyse automatique les dumps)",
          "Analyser avec WinDbg (Microsoft) pour analyse experte : 'analyze -v' dans WinDbg",
          "Consulter l'Observateur d'événements : eventvwr > Journaux Windows > Application + Système",
          "Site web : passepourdépannage.org ou Nirsoft BlueScreenView pour historique BSODs",
          "Chercher le code d'erreur sur Google avec le .sys incriminé pour identifier le pilote",
          "Utiliser NiTriTe > Analyse BSOD pour voir l'historique automatiquement",
          "Paramètre clé dans WinDbg : !analyze -v puis lm (list modules) pour voir le driver fautif",
        ],
        command: "eventvwr.msc",
      },
    ],
  },
  {
    id: "registre",
    label: "Registre Windows",
    icon: "Settings",
    items: [
      {
        title: "Nettoyer les entrées Autorun suspectes",
        symptoms: "Programmes inconnus au démarrage, lenteur, comportement anormal, icônes inconnues",
        solution: [
          "Utiliser Autoruns64 de Sysinternals (outil le plus complet pour les autoruns)",
          "Ouvrir regedit.exe en administrateur",
          "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run (programmes user)",
          "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run (programmes système)",
          "HKLM\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run (32bits sur 64bits)",
          "Vérifier HKCU\\..\\RunOnce (programmes lancés une fois puis supprimés)",
          "Désactiver (ne pas supprimer d'abord) les entrées inconnues pour tester",
          "Supprimer les entrées dont le programme source n'existe plus sur le disque",
        ],
        command: "regedit.exe",
      },
      {
        title: "Réparer les associations de fichiers",
        symptoms: "Double-clic ne fonctionne plus, mauvais programme par défaut, 'Ouvrir avec' ne mémorise pas",
        solution: [
          "Paramètres > Applications > Applications par défaut",
          "Clic droit sur un fichier > Ouvrir avec > Choisir une autre application > Toujours utiliser cette application",
          "PowerShell : cmd /c assoc .ext=AppName (ex: assoc .pdf=Acrobat.Document.DC)",
          "Réinitialiser toutes les associations : DISM /Online /Enable-Feature /FeatureName:MediaPlayback",
          "Via registre : HKCR\\.ext (clé par extension) et HKCR\\AppID associé",
          "Si associations cassées après installation logiciel : désinstaller le logiciel et réparer",
          "En dernier recours : Réparer Windows avec DISM /Online /Cleanup-Image /RestoreHealth",
        ],
        command: "dism /online /cleanup-image /restorehealth",
      },
      {
        title: "Sauvegarder et restaurer le registre",
        symptoms: "Avant modification du registre, protection avant installation risquée",
        solution: [
          "Sauvegarder tout le registre : regedit > Fichier > Exporter > Tous (Toutes les branches)",
          "Sauvegarder une clé spécifique : naviguer vers la clé > Fichier > Exporter > Branche sélectionnée",
          "Restaurer : double-cliquer le fichier .reg exporté ou regedit > Fichier > Importer",
          "Créer un point de restauration AVANT toute modification registre importante",
          "Utiliser reg save pour sauvegardes scriptées : reg save HKLM\\Software backup.hiv",
          "Restaurer depuis backup.hiv : reg restore HKLM\\Software backup.hiv",
          "Ne jamais modifier HKLM\\SYSTEM\\CurrentControlSet\\Services sans backup",
        ],
        command: "regedit.exe",
      },
    ],
  },
  {
    id: "demarrage",
    label: "Démarrage Windows",
    icon: "Cpu",
    items: [
      {
        title: "Windows ne démarre plus",
        symptoms: "Écran noir, boucle de réparation, impossible d'accéder au bureau, PC qui redémarre en boucle",
        solution: [
          "Démarrer sur un support USB Windows 10/11 (créer avec Rufus si nécessaire)",
          "Aller dans Dépanner > Options avancées > Réparer le démarrage",
          "Utiliser bootrec depuis l'invite de commandes WinRE : bootrec /fixmbr && bootrec /fixboot && bootrec /rebuildbcd",
          "Si partition corrompue : chkdsk C: /f /r depuis l'invite WinRE",
          "SFC depuis WinRE : sfc /scannow /offbootdir=C:\\ /offwindir=C:\\Windows",
          "Restaurer un point de restauration depuis WinRE > Options avancées > Restauration du système",
          "DISM depuis WinRE pour réparer l'image Windows",
          "En dernier recours : réinstallation de Windows (options : garder les fichiers ou format complet)",
        ],
        command: "bootrec /rebuildbcd",
      },
      {
        title: "Boucle de réparation automatique",
        symptoms: "Windows tente de se réparer en boucle, 'Préparation de la réparation automatique' indéfiniment",
        solution: [
          "Appuyer sur F4 depuis l'écran de démarrage ou accéder via Shift+Redémarrer > Dépanner > Mode sans échec",
          "Depuis WinRE (Options avancées) : bcdedit /set {default} recoveryenabled No",
          "Lancer SFC depuis WinRE : sfc /scannow /offbootdir=C:\\ /offwindir=C:\\Windows",
          "Lancer CHKDSK depuis WinRE : chkdsk C: /f /r",
          "Désactiver le démarrage automatique des réparations : bcdedit /set {current} bootstatuspolicy ignoreallfailures",
          "Reconstruire les fichiers de démarrage : bootrec /rebuildbcd",
          "Si pilote causait le problème : désinstaller depuis mode sans échec",
        ],
        command: "bcdedit /set {default} recoveryenabled No",
      },
      {
        title: "BIOS/UEFI - paramètres importants",
        symptoms: "Besoin de changer l'ordre de boot, activer/désactiver Secure Boot, TPM, XMP",
        solution: [
          "Accès BIOS/UEFI : appuyer sur Del, F2, F10, ou F12 selon le fabricant au démarrage",
          "Ordre de boot : Boot > Boot Priority > placer USB/DVD en premier pour installation",
          "Secure Boot : Security > Secure Boot > Enable/Disable (désactiver pour Linux ou ancien OS)",
          "TPM 2.0 : Security > TPM > Enable (requis pour Windows 11 officiellement)",
          "XMP/EXPO : Extreme Memory Profile > Enable (active OC RAM officiel)",
          "AHCI vs IDE : Storage > SATA Mode (AHCI obligatoire pour SSD, jamais IDE)",
          "Wake on LAN : Power > Wake on LAN > Disable (économise énergie si non nécessaire)",
          "Sauvegarder les paramètres BIOS : certaines cartes mères permettent profils BIOS",
        ],
      },
      {
        title: "Dual-boot Windows/Linux",
        symptoms: "Seul Windows apparaît, Linux inaccessible, GRUB manquant après mise à jour Windows",
        solution: [
          "Windows Update peut écraser le bootloader GRUB : normal, se corrige facilement",
          "Démarrer depuis un live USB Linux, ouvrir terminal : sudo grub-install /dev/sda && sudo update-grub",
          "Ou utiliser Boot-Repair (Linux) depuis un live USB pour réparer automatiquement",
          "Vérifier que la partition Linux existe toujours : Gestionnaire de disques Windows",
          "Utiliser EasyBCD (Windows) pour ajouter une entrée Linux dans le BCD Windows",
          "Pour UEFI : vérifier entrées dans le firmware : bcdedit /enum firmware",
          "Désactiver le démarrage rapide Windows si Linux ne voit pas le disque Windows (hibernation partielle)",
          "Toujours installer Windows AVANT Linux pour éviter les conflits de bootloader",
        ],
      },
    ],
  },
  {
    id: "windows11",
    label: "Windows 11",
    icon: "Monitor",
    items: [
      {
        title: "Pré-requis et installation Windows 11",
        symptoms: "Message 'Ce PC ne peut pas exécuter Windows 11', erreur TPM, CPU non supporté",
        solution: [
          "Vérifier compatibilité avec PC Health Check de Microsoft",
          "Prérequis officiels : CPU 64-bit 1GHz+ sur liste Microsoft, 4GB RAM, 64GB, UEFI+Secure Boot, TPM 2.0",
          "Activer TPM 2.0 dans le BIOS : Security > TPM Device Selection > Firmware TPM",
          "Activer Secure Boot : Security > Secure Boot > Enabled (nécessite mode UEFI pas Legacy/CSM)",
          "Méthode Rufus pour contourner : télécharger ISO W11 + Rufus 3.19+ > cocher 'Remove requirements'",
          "Méthode registre pendant installation : Shift+F10 > regedit > HKLM\\SYSTEM\\Setup > LabConfig > BypassTPMCheck=1 + BypassSecureBootCheck=1",
          "Installation propre : sauvegarder données, créer USB bootable avec Media Creation Tool",
          "Mise à jour depuis W10 : Paramètres > Mise à jour Windows (si PC éligible)",
        ],
        command: "start ms-settings:windowsupdate",
      },
      {
        title: "Optimiser Windows 11",
        symptoms: "Windows 11 lent, animations fluides mais ressources consommées, télémétrie excessive",
        solution: [
          "Désactiver télémétrie : Paramètres > Confidentialité > Diagnostics > Données nécessaires uniquement",
          "Désactiver Widgets : clic droit barre des tâches > Désactiver",
          "Désactiver Teams Chat si non utilisé : Paramètres > Applications > Démarrage",
          "Désactiver animations : Paramètres > Accessibilité > Effets visuels > Désactiver",
          "Nettoyer démarrage : Gestionnaire des tâches > Démarrage > Désactiver inutiles",
          "Activer Mode Hautes Performances : Paramètres > Alimentation > Meilleures performances",
          "Désactiver services inutiles : Xbox Live Auth Manager, DiagTrack, dmwappushservice",
          "Désactiver indexation si SSD : services.msc > Windows Search > Manuel (désactiver si inutile)",
        ],
        command: "services.msc",
      },
      {
        title: "Confidentialité Windows 11",
        symptoms: "Envoi de données à Microsoft, publicités personnalisées, tracking",
        solution: [
          "Paramètres > Confidentialité et sécurité > Général : tout désactiver",
          "Désactiver ID publicitaire, suivi linguistique, suggestions personnalisées",
          "Confidentialité > Voix : désactiver reconnaissance vocale en ligne",
          "Confidentialité > Activité : effacer et désactiver historique des activités",
          "Autorisations par app : désactiver Emplacement, Caméra, Micro pour apps non essentielles",
          "Désactiver OneDrive si non utilisé : winget uninstall Microsoft.OneDrive",
          "Bloquer serveurs télémétrie via fichier hosts ou via NiTriTe > Scripts Windows",
          "Utiliser O&O ShutUp10++ pour désactivation en masse des traqueurs",
        ],
        command: "start ms-settings:privacy",
      },
      {
        title: "Restauration système et réinitialisation",
        symptoms: "Windows instable, besoin de revenir en arrière après une mauvaise mise à jour ou installation",
        solution: [
          "Points de restauration : Paramètres > Système > À propos > Protection du système > Restaurer",
          "Créer un point manuellement : Protection du système > Créer (5 min, 500MB - 2GB)",
          "Réinitialiser le PC (garder fichiers) : Paramètres > Système > Récupération > Réinitialiser",
          "Réinitialiser le PC (tout effacer) : même chemin, choisir 'Tout supprimer'",
          "Reinstallation depuis WinRE sans USB : Dépanner > Réinitialiser ce PC",
          "Restauration depuis image système (backup complet) : Panneau config > Récupération > Options avancées de récupération",
          "Utiliser les snapshots Hyper-V ou VMware si PC virtualisé",
          "Après réinstallation : utiliser NiTriTe Master Install pour réinstaller tous les programmes rapidement",
        ],
        command: "systemrst.exe",
      },
    ],
  },
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
  },
  {
    id: "cmd",
    label: "Invite de Commandes (CMD)",
    icon: "Terminal",
    items: [
      {
        title: "CMD - Commandes essentielles réseau",
        solution: [
          "ipconfig /all : configuration réseau complète (IP, DNS, MAC, passerelle)",
          "ipconfig /flushdns : vider le cache DNS",
          "ipconfig /release et /renew : renouveler l'adresse IP",
          "ping -t 8.8.8.8 : ping continu (Ctrl+C pour arrêter)",
          "tracert google.com : tracer la route jusqu'à un serveur",
          "netstat -ano : toutes les connexions actives avec PID",
          "nslookup google.com : résolution DNS manuelle",
          "netsh wlan show profiles : voir les profils Wi-Fi enregistrés",
          "netsh wlan show profile name='MonWifi' key=clear : voir mot de passe Wi-Fi",
        ],
        command: "ipconfig /all",
      },
      {
        title: "CMD - Commandes de réparation système",
        solution: [
          "sfc /scannow : scanner et réparer les fichiers système corrompus (ADMIN)",
          "DISM /Online /Cleanup-Image /RestoreHealth : réparer l'image Windows depuis Windows Update",
          "chkdsk C: /f /r : vérifier et réparer le disque (redémarrage requis)",
          "bootrec /fixmbr : réparer le MBR (depuis WinRE)",
          "bootrec /rebuildbcd : reconstruire le BCD (boot configuration)",
          "bcdedit : gérer les entrées de démarrage",
          "wmic diskdrive get status : état SMART des disques",
          "systeminfo : informations complètes système",
        ],
        command: "sfc /scannow",
      },
      {
        title: "CMD - Scripts Batch utiles",
        solution: [
          "@echo off en première ligne : masquer les commandes affichées",
          "REM ou :: pour les commentaires",
          "pause : attendre l'appui sur une touche",
          "set VARIABLE=valeur : définir une variable",
          "if '%ERRORLEVEL%'=='0' (...) else (...) : condition",
          "for %%F in (*.txt) do echo %%F : boucle sur fichiers",
          "Créer raccourci de lancement en admin avec runas /user:Administrator",
          "call script.bat : appeler un sous-script depuis un script",
        ],
        code: `@echo off
REM Script de nettoyage simple
title Nettoyage Systeme
echo Nettoyage en cours...

echo Suppression fichiers temporaires...
del /f /s /q %temp%\\* 2>nul

echo Vidage corbeille...
powershell -Command "Clear-RecycleBin -Force"

echo Vider cache DNS...
ipconfig /flushdns

echo Nettoyage termine!
pause`,
      },
    ],
  },
  {
    id: "hardware",
    label: "Matériel & Hardware",
    icon: "Cpu",
    items: [
      {
        title: "CPU - Températures et refroidissement",
        symptoms: "PC qui chauffe, throttling, performances réduites sous charge, ventilateurs à fond",
        solution: [
          "Températures normales : < 70°C en gaming, < 50°C au repos (AMD/Intel récent)",
          "Températures critiques : > 95°C → throttling automatique, > 100°C → BSOD possible",
          "Surveiller avec HWiNFO64 (le plus précis) ou HWMonitor",
          "Nettoyer les ventilateurs et radiateurs à l'air comprimé tous les 6-12 mois",
          "Remplacer la pâte thermique si CPU > 5 ans ou température anormale (Thermal Grizzly Kryonaut)",
          "Vérifier que les ventilateurs du boîtier tournent dans le bon sens (entrée avant/bas, sortie arrière/haut)",
          "Améliorer le flux d'air : câbles bien rangés, filtres à poussière propres",
          "Undervoltage CPU : réduire la tension avec Intel XTU ou AMD Ryzen Master (-50mV à -100mV stable)",
        ],
        command: "wmic cpu get name,CurrentClockSpeed,MaxClockSpeed,LoadPercentage",
      },
      {
        title: "GPU - Diagnostic et optimisation",
        symptoms: "FPS bas, artefacts graphiques, crash jeux, GPU non détecté ou utilisation < 50% anormale",
        solution: [
          "Vérifier utilisation GPU en gaming : doit être proche de 100% (si < 50% → CPU bottleneck)",
          "Températures GPU normales : < 83°C en gaming (certaines cartes jusqu'à 90°C par design)",
          "Mettre à jour pilotes NVIDIA via GeForce Experience ou AMD via Radeon Software",
          "Désinstaller proprement avec DDU (Display Driver Uninstaller) en mode sans échec avant réinstallation",
          "Activer Resizable BAR/SAM dans le BIOS pour +5% FPS gratuits",
          "NVIDIA : activer DLSS dans les jeux supportés (qualité/performance)",
          "AMD : activer FSR dans les jeux supportés",
          "Vérifier alimentation GPU : câble PCIe 8-pin/16-pin bien connecté, PSU suffisant (RTX 4090 = 450W)",
        ],
        command: "dxdiag",
      },
      {
        title: "RAM - Configuration et problèmes",
        symptoms: "PC instable, crashs aléatoires, BSOD, performances inférieures à ce qui est attendu",
        solution: [
          "Vérifier dual-channel : RAM dans les slots A2+B2 (pas A1+B1) selon manuel carte mère",
          "Activer XMP/EXPO dans le BIOS pour atteindre la fréquence annoncée (sinon fonctionne à 2133 MHz par défaut)",
          "Fréquences actuelles avec CPU-Z : onglet Memory, vérifier Frequency et Type (DDR4/DDR5)",
          "Test RAM complet : MemTest86 depuis USB bootable (minimum 2 passages = 4-8h)",
          "Si barrette défaillante : tester une par une pour identifier",
          "Compatibilité : vérifier QVL (Qualified Vendor List) de la carte mère pour les kits validés",
          "DDR5 vs DDR4 : DDR5 obligatoire sur Intel LGA1700 12th-14th gen et AMD AM5",
          "Ordre installation : A2 puis B2 pour dual-channel chez la plupart des fabricants",
        ],
        command: "wmic memorychip get BankLabel,Capacity,Speed,MemoryType",
      },
      {
        title: "Alimentation (PSU) - problèmes et diagnostic",
        symptoms: "PC qui s'éteint sous charge, instabilité, composants non alimentés, redémarrages aléatoires",
        solution: [
          "Puissances recommandées 2024 : RTX 4070=650W, RTX 4080=850W, RTX 4090=1000W minimum",
          "Vérifier le PSU avec OCCT (test PSU pendant 30 min sous charge maximale)",
          "Extinction sous charge = PSU sous-dimensionné ou en fin de vie",
          "Vérifier les câbles d'alimentation PCIe du GPU (bien connectés, bons connecteurs)",
          "Nettoyer ventilateur du PSU (poussière = surchauffe et protection thermique)",
          "Mesurer la qualité du courant avec un multimètre sur les rails +12V/+5V/+3.3V",
          "PSU à remplacer si > 7-8 ans ou s'il ne garantit plus 80% de sa puissance",
          "Bon budget PSU : 15-20% du budget total PC pour une alimentation de qualité",
        ],
      },
      {
        title: "Overclocking - bases et stabilité",
        symptoms: "Vouloir améliorer les performances sans achat de matériel, PC instable après OC",
        solution: [
          "CPU OC : Intel (séries K sur carte mère Z) avec Intel XTU ou via BIOS Ratio Multiplier",
          "AMD OC : Ryzen Master (logiciel) ou BIOS (Precision Boost Overdrive 2 = OC automatique)",
          "RAM OC : activer XMP/EXPO dans le BIOS en premier (OC officiel du fabricant)",
          "GPU OC : MSI Afterburner, augmenter Core Clock +100-200MHz et Memory Clock +200-400MHz progressivement",
          "Tester la stabilité : AIDA64 (CPU/RAM), FurMark (GPU), 30 min minimum sans crash ni artefact",
          "Surveiller les températures pendant les tests : HWiNFO64",
          "Si instable : augmenter la tension (Core Voltage) légèrement ou réduire la fréquence",
          "Règle d'or : stabilité > performance. Un crash OC peut corrompre des données",
        ],
      },
    ],
  },
  {
    id: "maintenance",
    label: "Maintenance Système",
    icon: "Settings",
    items: [
      {
        title: "Outils de réparation Windows (SFC, DISM)",
        symptoms: "Fichiers système corrompus, fonctionnalités Windows défaillantes, erreurs d'application",
        solution: [
          "SFC (System File Checker) : répare les fichiers système corrompus depuis le cache local",
          "Lancer SFC : sfc /scannow (en administrateur, 5-15 min)",
          "DISM : répare l'image Windows depuis les serveurs Microsoft ou un ISO local",
          "Lancer DISM : DISM /Online /Cleanup-Image /RestoreHealth (nécessite Internet)",
          "Ordre recommandé : DISM en premier, puis SFC, puis redémarrer, puis SFC à nouveau",
          "SFC hors ligne depuis WinRE : sfc /scannow /offbootdir=C:\\ /offwindir=C:\\Windows",
          "Vérifier le log SFC : %windir%\\Logs\\CBS\\CBS.log (chercher 'Cannot repair')",
          "Si SFC et DISM échouent : réinitialiser Windows ou installation de réparation depuis ISO",
        ],
        command: "DISM /Online /Cleanup-Image /RestoreHealth && sfc /scannow",
      },
      {
        title: "Nettoyage et maintenance régulière",
        symptoms: "PC qui accumule de la poussière, espace disque qui disparaît, applications obsolètes",
        solution: [
          "Nettoyage mensuel : cleanmgr, vider Corbeille, supprimer %TEMP%",
          "Mise à jour Windows : vérifier et installer toutes les mises à jour",
          "Mise à jour des logiciels : winget upgrade --all (met à jour tous les programmes winget)",
          "Scan antivirus complet mensuel",
          "Vérifier l'état SMART des disques avec CrystalDiskInfo",
          "Nettoyer physiquement le PC (poussière) tous les 6-12 mois",
          "Vérifier les logs d'événements pour les erreurs répétées",
          "Créer un point de restauration avant toute modification majeure",
        ],
        command: "winget upgrade --all",
      },
      {
        title: "Windows Update - gestion et dépannage",
        symptoms: "Mises à jour bloquées, erreurs 0x80070002 / 0x80070057, disque plein à cause de Windows Update",
        solution: [
          "Vider le cache Windows Update : net stop wuauserv && rd /s /q C:\\Windows\\SoftwareDistribution && net start wuauserv",
          "Lancer l'utilitaire de résolution des problèmes Windows Update",
          "Réinitialiser les composants Windows Update (script Microsoft Reset_Windows_Update)",
          "Vérifier l'espace disque : Windows Update nécessite minimum 20 GB libres pour les mises à jour majeures",
          "Installer manuellement une KB depuis catalog.update.microsoft.com",
          "Mettre en pause les mises à jour si moment inopportun : Paramètres > Windows Update > Autres options",
          "Différer les mises à jour de fonctionnalités de 1 an pour éviter les bugs de déploiement initial",
          "Vérifier que le service Windows Update est en démarrage automatique : services.msc",
        ],
        command: "net stop wuauserv && net stop bits && net start bits && net start wuauserv",
      },
    ],
  },
  {
    id: "pilotes",
    label: "Pilotes (Drivers)",
    icon: "Settings",
    items: [
      {
        title: "Installer et mettre à jour les pilotes",
        symptoms: "Périphériques non reconnus, performances sous-optimales, fonctionnalités manquantes",
        solution: [
          "Priorité : chipset > GPU > réseau > son > autres (ordre d'installation recommandé)",
          "Pilotes chipset : site de la carte mère (ASUS, MSI, Gigabyte, ASRock) ou Intel/AMD",
          "Pilotes GPU : NVIDIA GeForce Experience ou AMD Radeon Software (auto-update)",
          "Pilotes réseau/Wi-Fi : gestionnaire de périphériques > clic droit > Mettre à jour",
          "Via Windows Update : Paramètres > Windows Update > Options avancées > Mises à jour optionnelles",
          "Identifier matériel sans pilote : Device Manager, les points d'exclamation jaunes",
          "Utiliser l'outil NiTriTe > Scanner Pilotes pour une vue complète",
          "Éviter les sites tiers de pilotes (risque de bundleware/malware)",
        ],
        command: "devmgmt.msc",
      },
      {
        title: "Désinstaller proprement les pilotes GPU (DDU)",
        symptoms: "Pilote GPU corrompu, artefacts, BSOD lié au GPU, mise à jour pilote qui échoue",
        solution: [
          "Télécharger DDU (Display Driver Uninstaller) depuis wagnardsoft.com",
          "Démarrer en mode sans échec : Shift+Redémarrer > Dépanner > Mode sans échec",
          "Lancer DDU > Sélectionner GPU (NVIDIA/AMD/Intel) > Clean and Restart",
          "Windows redémarre sans pilote GPU (résolution basse, aucune accélération)",
          "Télécharger et installer le nouveau pilote depuis NVIDIA/AMD",
          "Ne pas utiliser GeForce Experience/Radeon Software si le pilote précédent était corrompu — installer manuellement",
          "Vérifier dans Gestionnaire des périphériques que le GPU est reconnu après installation",
          "Si problème persiste après DDU + réinstallation → problème matériel GPU",
        ],
      },
      {
        title: "Problèmes pilotes audio Realtek",
        symptoms: "Pas de son, son grésillant, micro non détecté, onglets dans le panneau Realtek manquants",
        solution: [
          "Désinstaller le pilote Realtek depuis Ajout/Suppression de programmes",
          "Redémarrer (Windows installe le pilote audio générique)",
          "Télécharger le dernier pilote Realtek depuis le site de la carte mère (ASUS/MSI/etc.) — pas depuis Realtek directement",
          "Installer et redémarrer",
          "Si problème persiste : outil de dépannage audio Windows",
          "Vérifier BIOS : HD Audio Controller doit être activé",
          "Vérifier que les connecteurs front panel jack sont bien branchés sur la carte mère",
          "Parfois un rollback de pilote résout le problème : Gestionnaire > Propriétés pilote > Revenir en arrière",
        ],
      },
    ],
  },
  {
    id: "logiciels",
    label: "Logiciels & Applications",
    icon: "Terminal",
    items: [
      {
        title: "Winget - gestionnaire de paquets Windows",
        symptoms: "Besoin d'installer/mettre à jour des applications en ligne de commande",
        solution: [
          "Vérifier l'installation : winget --version (disponible Windows 10 1809+ et Windows 11)",
          "Chercher une application : winget search firefox",
          "Installer : winget install --id Mozilla.Firefox --exact",
          "Installer en silencieux : winget install Mozilla.Firefox --silent --accept-package-agreements",
          "Mettre à jour tout : winget upgrade --all",
          "Mettre à jour une app : winget upgrade Mozilla.Firefox",
          "Désinstaller : winget uninstall Mozilla.Firefox",
          "Exporter liste installée : winget export -o packages.json",
          "Importer/Restaurer : winget import -i packages.json",
        ],
        command: "winget upgrade --all --accept-source-agreements",
      },
      {
        title: "Désinstallation propre d'applications",
        symptoms: "Application mal désinstallée, résidus dans le registre, programme qui réapparaît",
        solution: [
          "Désinstaller via Paramètres > Applications ou Panneau de config > Programmes",
          "Pour désinstallation complète : utiliser Revo Uninstaller ou BCUninstaller (gratuit)",
          "BCUninstaller : scanne les résidus registre et fichiers après désinstallation standard",
          "Supprimer manuellement : C:\\Program Files\\AppName, C:\\ProgramData\\AppName, %APPDATA%\\AppName",
          "Nettoyage registre : HKLM\\SOFTWARE\\AppName + HKCU\\SOFTWARE\\AppName",
          "Vérifier les tâches planifiées liées à l'app : taskschd.msc",
          "Vérifier les services liés : services.msc",
          "Redémarrer après désinstallation pour libérer les fichiers verrouillés",
        ],
        command: "bcdedit",
      },
    ],
  },
  // === Nouvelles catégories étendues ===
  ...kbHardware,
  ...kbGaming,
  ...kbWindowsAvance,
  ...kbSecuriteAvancee,
  ...kbReparation,
  ...kbLogiciels,
  ...kbReseauAvance,
  ...kbScriptsAvances,
  // === Nouvelles catégories v26.36.0 ===
  ...kbDeveloppement,
  ...kbMultimedia,
  ...kbVirtualisation,
  ...kbCloudSauvegarde,
  ...kbEntrepriseAD,
  ...kbLinuxDualboot,
  ...kbEnergieBatterie,
  ...kbStockageRaid,
  ...kbAutomatisation,
  ...kbErreursSysteme,
  ...kbAndroidMobile,
  ...kbPeripheriquesAvances,
  ...kbReseauEntreprise,
  ...kbIaOutils,
];
