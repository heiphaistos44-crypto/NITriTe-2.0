import type { KBCategory } from "../knowledgeBase";

export const kbWindows11: KBCategory[] = [
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
  }
];
