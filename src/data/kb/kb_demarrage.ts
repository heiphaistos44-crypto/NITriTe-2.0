import type { KBCategory } from "../knowledgeBase";

export const kbDemarrage: KBCategory[] = [
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
  }
];
