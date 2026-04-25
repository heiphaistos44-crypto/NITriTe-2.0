import type { KBCategory } from "../knowledgeBase";

export const kbReseau: KBCategory[] = [
  {
    id: "reseau",
    label: "Réseau",
    icon: "Wifi",
    items: [
      {
        title: "Pas de connexion Internet",
        symptoms: "Pages web inaccessibles, icône réseau avec triangle jaune",
        solution: [
          "Vérifier le câble Ethernet ou la connexion Wi-Fi",
          "Redémarrer le routeur/modem (débrancher 30s)",
          "Vider le cache DNS : ipconfig /flushdns",
          "Réinitialiser la pile TCP/IP : netsh int ip reset",
          "Réinitialiser Winsock : netsh winsock reset",
          "Relancer le service DHCP : net stop dhcp && net start dhcp",
          "Si connexion cellulaire active : désactiver partage de connexion potentiellement conflictuel",
        ],
        command: "ipconfig /flushdns && netsh int ip reset && netsh winsock reset",
      },
      {
        title: "Wi-Fi se déconnecte fréquemment",
        symptoms: "Connexion instable, déconnexions répétées, signal fluctuant",
        solution: [
          "Mettre à jour le pilote Wi-Fi via Gestionnaire de périphériques",
          "Désactiver la gestion d'alimentation de l'adaptateur (Propriétés > Gestion de l'alimentation > décocher 'Autoriser arrêt pour économie d'énergie')",
          "Changer le canal Wi-Fi sur le routeur (éviter canaux 1/6/11 si encombrés — utiliser un scanner Wi-Fi)",
          "Vérifier les interférences (micro-ondes, Bluetooth, téléphones sans fil)",
          "Désactiver le mode économie d'énergie Wi-Fi dans Panneau de config > Options d'alimentation",
          "Forcer le protocole 5GHz si routeur dual-band et PC compatible",
          "Réinitialiser les profils réseau corrompus : netsh wlan delete profile name=*",
        ],
      },
      {
        title: "DNS lent ou ne résout pas",
        symptoms: "Sites longs à charger, erreur DNS_PROBE_FINISHED_NXDOMAIN, erreur 'Serveur introuvable'",
        solution: [
          "Changer les DNS : 8.8.8.8 / 8.8.4.4 (Google) ou 1.1.1.1 / 1.0.0.1 (Cloudflare)",
          "Via PowerShell admin : Set-DnsClientServerAddress -InterfaceAlias Wi-Fi -ServerAddresses 8.8.8.8,8.8.4.4",
          "Vider le cache DNS : ipconfig /flushdns",
          "Vérifier le fichier hosts pour entrées parasites : C:\\Windows\\System32\\drivers\\etc\\hosts",
          "Tester résolution DNS : nslookup google.com 8.8.8.8",
          "Désactiver le DNS-over-HTTPS du navigateur si problème spécifique navigateur",
        ],
        command: "ipconfig /flushdns && nslookup google.com 8.8.8.8",
      },
      {
        title: "IP en conflit ou en 169.254.x.x",
        symptoms: "IP affichée 169.254.x.x (APIPA), icône réseau avec point d'exclamation, pas d'accès réseau local",
        solution: [
          "Libérer et renouveler l'adresse IP : ipconfig /release puis ipconfig /renew",
          "Vérifier que le service DHCP est actif sur le routeur",
          "Vérifier qu'il n'y a pas deux PCs avec la même IP statique sur le réseau",
          "Passer temporairement en IP statique pour tester : définir une IP dans la plage 192.168.1.x",
          "Relancer le client DHCP Windows : net stop dhcp && net start dhcp",
          "Vérifier qu'aucun antivirus/pare-feu ne bloque le DHCP",
        ],
        command: "ipconfig /release && ipconfig /renew",
      },
      {
        title: "Pare-feu bloque une application",
        symptoms: "Application ne peut pas accéder à Internet, connection refused, timeout",
        solution: [
          "Ouvrir Sécurité Windows > Pare-feu et protection réseau > Autoriser une application",
          "Ou via Panneau de config > Système et sécurité > Pare-feu Windows > Autoriser une appli",
          "Créer une règle entrante/sortante via Règles avancées (wf.msc)",
          "Vérifier si c'est le pare-feu Windows ou un antivirus tiers",
          "Tester en désactivant temporairement le pare-feu (RISQUE : remettre après test)",
          "Vérifier les logs du pare-feu : wf.msc > Propriétés > Journal de sécurité",
        ],
        command: "wf.msc",
      },
      {
        title: "Ports réseau à connaître",
        symptoms: "Besoin de débloquer un port, diagnostic de connectivité",
        solution: [
          "80/443 : HTTP/HTTPS (navigation web)",
          "22 : SSH (connexion distante sécurisée)",
          "3389 : RDP (Bureau à distance Windows)",
          "25/587 : SMTP (envoi mail)",
          "110/993 : POP3/IMAP (réception mail)",
          "21 : FTP (transfert fichiers)",
          "53 : DNS (résolution noms)",
          "3306 : MySQL, 5432 : PostgreSQL",
          "8080 : proxy/HTTP alternatif",
          "Lister ports ouverts : netstat -an | find 'LISTENING'",
        ],
        command: 'netstat -an | find "LISTENING"',
      },
      {
        title: "Connexion lente malgré bon débit",
        symptoms: "Ping élevé, lag en gaming/vidéoconférence, téléchargements lents malgré bonne box",
        solution: [
          "Tester le débit réel : speedtest.net ou fast.com depuis le PC concerné",
          "Vérifier MTU : ping -f -l 1472 8.8.8.8 (si timeout, réduire MTU)",
          "Ajuster MTU : netsh interface ipv4 set subinterface 'Wi-Fi' mtu=1400 store=persistent",
          "Désactiver QoS Paquets si non utilisé (peut limiter à 20% la bande passante)",
          "Vérifier que la carte réseau n'est pas en mode duplex half : Propriétés adaptateur > Vitesse de la liaison",
          "Désactiver l'auto-tuning TCP si lenteur sur réseau local : netsh interface tcp set global autotuninglevel=disabled",
          "Vérifier les applications en fond qui consomment de la bande passante (Gestionnaire des tâches > Réseau)",
        ],
      },
    ],
  }
];
