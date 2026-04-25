import type { KBCategory } from "../knowledgeBase";

export const kbAffichage: KBCategory[] = [
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
  }
];
