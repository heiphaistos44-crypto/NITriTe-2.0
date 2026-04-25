import type { KBCategory } from "../knowledgeBase";

export const kbLogicielsCore: KBCategory[] = [
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
  }
];
