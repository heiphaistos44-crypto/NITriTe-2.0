import type { KBCategory } from "../knowledgeBase";

export const kbMaintenance: KBCategory[] = [
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
  }
];
