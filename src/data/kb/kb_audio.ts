import type { KBCategory } from "../knowledgeBase";

export const kbAudio: KBCategory[] = [
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
  }
];
