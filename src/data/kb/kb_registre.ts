import type { KBCategory } from "../knowledgeBase";

export const kbRegistre: KBCategory[] = [
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
  }
];
