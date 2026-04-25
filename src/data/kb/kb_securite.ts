import type { KBCategory } from "../knowledgeBase";

export const kbSecurite: KBCategory[] = [
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
  }
];
