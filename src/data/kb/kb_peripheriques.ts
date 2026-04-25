import type { KBCategory } from "../knowledgeBase";

export const kbPeripheriques: KBCategory[] = [
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
  }
];
