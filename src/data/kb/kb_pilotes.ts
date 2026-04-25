import type { KBCategory } from "../knowledgeBase";

export const kbPilotes: KBCategory[] = [
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
  }
];
