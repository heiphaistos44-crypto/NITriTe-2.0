import type { KBCategory } from "../knowledgeBase";

export const kbReseauEntreprise: KBCategory[] = [
  {
    id: "switching-vlan",
    label: "Switching & VLANs",
    icon: "Network",
    items: [
      {
        title: "VLANs — concept et configuration Windows",
        solution: [
          "VLAN (Virtual LAN) : segmenter un réseau physique en plusieurs réseaux logiques isolés",
          "Trunk port : port de switch qui transporte plusieurs VLANs (tagged 802.1Q)",
          "Access port : port qui n'appartient qu'à un seul VLAN (untagged)",
          "Native VLAN : VLAN par défaut sur un trunk (trafic non taggé)",
          "Hyperviseur avec VLANs : configurer le VLAN ID sur l'adaptateur réseau virtuel de la VM",
          "Windows avec VLAN : possible via les propriétés avancées de la carte réseau (si driver supporte 802.1Q)",
          "pfSense / OPNsense : firewalls open source qui gèrent les VLANs nativamente",
          "Sondes VLAN : outils comme Wireshark montrent le tag VLAN 802.1Q dans les captures",
        ],
        code: `# Configurer un VLAN sur Windows (si le driver réseau supporte 802.1Q)
# Propriétés de la carte réseau > Avancé > VLAN ID

# Via PowerShell (Intel NICs, Hyper-V)
# Voir si la carte supporte les VLANs
Get-NetAdapterAdvancedProperty -Name "Ethernet" | Where-Object { $_.RegistryKeyword -like "*VLAN*" }

# Configurer le VLAN ID
Set-NetAdapterAdvancedProperty -Name "Ethernet" -RegistryKeyword "VlanID" -RegistryValue 10

# Hyper-V — configurer un VLAN sur une VM
Set-VMNetworkAdapterVlan -VMName "Server-Web" -Access -VlanId 100
Set-VMNetworkAdapterVlan -VMName "Server-Web" -Untagged  # Supprimer le VLAN

# Voir la config VLAN d'une VM
Get-VMNetworkAdapterVlan -VMName "Server-Web"

# Configurer le VLAN sur un switch Hyper-V virtuel (Trunk)
Set-VMNetworkAdapterVlan -ManagementOS -VMNetworkAdapterName "External Switch" -Trunk -AllowedVlanIdList "10,20,30,100" -NativeVlanId 1

# Cisco IOS — config VLAN de base (pour référence)
# vlan 10
#   name Serveurs
# interface GigabitEthernet0/1
#   switchport mode access
#   switchport access vlan 10
# interface GigabitEthernet0/24
#   switchport mode trunk
#   switchport trunk allowed vlan 10,20,30,100`,
      },
      {
        title: "pfSense / OPNsense — firewall open source",
        solution: [
          "pfSense et OPNsense : firewalls basés sur FreeBSD, gratuits, très complets",
          "Télécharger : pfsense.org ou opnsense.org (ISO pour VM ou hardware dédié)",
          "Hardware minimal : 2 cœurs, 4 Go RAM, 16 Go SSD, 2 interfaces réseau (WAN + LAN)",
          "Appliances dédiées : Netgate (pfSense), PC Engines APU, Protectli — faible consommation",
          "Fonctionnalités : NAT, pare-feu stateful, DHCP, DNS, OpenVPN, WireGuard, IDS/IPS (Suricata/Snort), VLANs",
          "Dashboard : interface web sur https://IP_LAN (admin/pfsense par défaut)",
          "Packages : Suricata (IDS/IPS), pfBlockerNG (bloquer publicités/malwares), HAProxy (load balancer)",
          "HA (High Availability) : deux pfSense en CARP pour la redondance",
        ],
        code: `# pfSense — commandes shell (via console ou SSH)
# Activer SSH : System > Advanced > Admin Access > Enable Secure Shell

# Depuis la console pfSense
# 8) Shell

# Voir les connexions actives
pfctl -s state | head -50

# Voir les règles de pare-feu
pfctl -s rules

# Afficher la table NAT
pfctl -s nat

# Voir les interfaces
ifconfig

# Diagnostics réseau
ping 8.8.8.8
traceroute google.com
nslookup google.com

# Suricata — voir les alertes IDS
cat /var/log/suricata/suricata_igb0.log | tail -20

# pfBlockerNG — forcer la mise à jour des listes
/usr/local/pkg/pfblockerng/pfblockerng.php

# Sauvegarde de la configuration
# GUI : Diagnostics > Backup & Restore > Download Configuration

# Via script (API pfSense)
$baseUrl = "https://192.168.1.1"
$creds = [Convert]::ToBase64String([Text.Encoding]::ASCII.GetBytes("admin:password"))
$backup = Invoke-WebRequest "$baseUrl/diag_backup.php" -Headers @{Authorization="Basic $creds"} -Method POST
$backup.Content | Set-Content "pfsense-backup.xml"`,
      },
      {
        title: "Monitoring réseau — SNMP et outils",
        solution: [
          "SNMP (Simple Network Management Protocol) : protocole standard pour monitorer les équipements réseau",
          "OID (Object Identifier) : identifiant unique pour chaque métrique (ex: interface speed, CPU, RAM)",
          "SNMPv3 : version sécurisée avec chiffrement et authentification (recommandée en production)",
          "MIB (Management Information Base) : bibliothèque des OIDs disponibles pour un équipement",
          "Outils gratuits : PRTG Network Monitor (100 sondes gratuites), Zabbix, Nagios, LibreNMS",
          "Wireshark : capturer et analyser le trafic SNMP",
          "Grafana + InfluxDB + Telegraf : stack moderne pour visualiser les métriques réseau",
          "Windows SNMP : activer dans Fonctionnalités Windows > SNMP Protocol",
        ],
        code: `# Activer SNMP sur Windows
Enable-WindowsOptionalFeature -Online -FeatureName "SNMP" -All

# Configurer SNMP via PowerShell
Set-ItemProperty -Path "HKLM:\\SYSTEM\\CurrentControlSet\\Services\\SNMP\\Parameters\\ValidCommunities" -Name "public" -Value 4 -Type DWord
Set-ItemProperty -Path "HKLM:\\SYSTEM\\CurrentControlSet\\Services\\SNMP\\Parameters" -Name "EnableAuthenticationTraps" -Value 1

# Requêtes SNMP depuis Windows (snmpwalk)
# Installer snmpwalk : winget install EzSoftware.SNMPWalk ou via Python avec pysnmp

# Python — requêtes SNMP
pip install pysnmp
python -c "
from pysnmp.hlapi import *

# Lire l'uptime via SNMP
for (errorIndication, errorStatus, errorIndex, varBinds) in getCmd(
    SnmpEngine(),
    CommunityData('public', mpModel=0),
    UdpTransportTarget(('192.168.1.1', 161)),
    ContextData(),
    ObjectType(ObjectIdentity('SNMPv2-MIB', 'sysUpTime', 0)),
    ObjectType(ObjectIdentity('SNMPv2-MIB', 'sysDescr', 0))
):
    for varBind in varBinds:
        print(varBind)
"

# Script de monitoring SNMP via PowerShell (avec module)
Install-Module -Name Indented.Net.IP -Scope CurrentUser

# Alerter si une interface réseau est Down
# Zabbix Agent (Windows)
winget install Zabbix.ZabbixAgent
# Configurer : C:\\Program Files\\Zabbix Agent\\zabbix_agentd.conf
# Server=192.168.1.200
# Hostname=NomDuPC`,
      },
    ],
  },
  {
    id: "dns-serveur",
    label: "DNS & Serveur Web",
    icon: "Globe",
    items: [
      {
        title: "DNS Windows Server — configuration et dépannage",
        solution: [
          "DNS Manager (dnsmgmt.msc) : console de gestion DNS Windows Server",
          "Zone de recherche directe : résout les noms en adresses IP (A records)",
          "Zone de recherche inversée : résout les IPs en noms (PTR records)",
          "Types d'enregistrements : A (IPv4), AAAA (IPv6), CNAME (alias), MX (messagerie), TXT (vérification), SRV (services)",
          "TTL (Time To Live) : durée de mise en cache du résultat DNS",
          "Root hints vs Forwarders : les forwarders (ex: 8.8.8.8) sont plus rapides pour les noms Internet",
          "Conditional Forwarder : rediriger les requêtes d'un domaine spécifique vers un DNS spécifique",
          "DNSSEC : signer les zones DNS pour prévenir l'empoisonnement du cache",
        ],
        code: `# Gestion DNS via PowerShell (module DnsServer)
Import-Module DnsServer

# Voir les zones DNS
Get-DnsServerZone -ComputerName "DC01" | Select ZoneName, ZoneType, IsAutoCreated

# Créer une zone DNS
Add-DnsServerPrimaryZone -Name "entreprise.local" -ZoneFile "entreprise.local.dns" -ComputerName "DC01"

# Créer des enregistrements DNS
Add-DnsServerResourceRecordA -Name "serveur-web" -ZoneName "entreprise.local" -IPv4Address "192.168.1.50" -TimeToLive 01:00:00
Add-DnsServerResourceRecordAAAA -Name "serveur-web" -ZoneName "entreprise.local" -IPv6Address "2001:db8::50"
Add-DnsServerResourceRecordCName -Name "www" -ZoneName "entreprise.local" -HostNameAlias "serveur-web.entreprise.local."
Add-DnsServerResourceRecordMx -Name "@" -ZoneName "entreprise.local" -MailExchange "mail.entreprise.local." -Preference 10

# Lister les enregistrements d'une zone
Get-DnsServerResourceRecord -ZoneName "entreprise.local" -ComputerName "DC01" | Select HostName, RecordType, RecordData

# Modifier un enregistrement
$oldRecord = Get-DnsServerResourceRecord -ZoneName "entreprise.local" -Name "serveur-web" -RRType A
$newRecord = $oldRecord.Clone()
$newRecord.RecordData.IPv4Address = [System.Net.IPAddress]::Parse("192.168.1.51")
Set-DnsServerResourceRecord -ZoneName "entreprise.local" -OldInputObject $oldRecord -NewInputObject $newRecord

# Diagnostiquer DNS
Resolve-DnsName "serveur-web.entreprise.local" -Server "DC01"
Clear-DnsClientCache                    # Vider le cache DNS client
ipconfig /flushdns                     # Vider le cache DNS Windows
nslookup serveur-web.entreprise.local DC01  # Tester contre un DNS spécifique

# Voir le cache DNS Windows
Get-DnsClientCache | Where-Object { $_.Name -like "*google*" }
Get-DnsServerCache -ComputerName "DC01"    # Cache côté serveur DNS`,
      },
      {
        title: "IIS — serveur web Windows",
        solution: [
          "IIS (Internet Information Services) : serveur web Microsoft intégré à Windows",
          "Installer : Fonctionnalités Windows > Internet Information Services",
          "Gestionnaire IIS (inetmgr) : interface graphique pour configurer les sites",
          "Site par défaut : écoute sur le port 80, contenu dans C:\\inetpub\\wwwroot",
          "Application Pool : isolation des applications (chaque app dans son propre processus)",
          "HTTPS : configurer un certificat SSL dans IIS > Liaisons > Ajouter > HTTPS",
          "URL Rewrite : module IIS pour la réécriture d'URLs (API REST, redirections)",
          "ARR (Application Request Routing) : load balancer et reverse proxy pour IIS",
        ],
        code: `# IIS via PowerShell (module WebAdministration)
Import-Module WebAdministration

# Voir les sites
Get-Website | Select Name, State, PhysicalPath, @{N="URL";E={"$($_.Bindings.Collection[0].Protocol)://$($_.Bindings.Collection[0].bindingInformation)"}}

# Créer un nouveau site
New-WebSite -Name "MonSite" -PhysicalPath "C:\\Sites\\MonSite" -Port 8080

# Créer un Application Pool
New-WebAppPool -Name "MonSitePool"
Set-ItemProperty "IIS:\\AppPools\\MonSitePool" processModel.identityType "LocalSystem"
Set-WebConfigurationProperty "/system.applicationHost/applicationPools/add[@name='MonSitePool']" -Name "managedRuntimeVersion" -Value "v4.0"

# Assigner un App Pool à un site
Set-ItemProperty "IIS:\\Sites\\MonSite" applicationPool "MonSitePool"

# Démarrer / Arrêter un site
Start-WebSite "MonSite"
Stop-WebSite "MonSite"
Restart-WebSite "MonSite"

# Configurer HTTPS avec un certificat auto-signé
$cert = New-SelfSignedCertificate -DnsName "monsite.local" -CertStoreLocation cert:\\LocalMachine\\My
New-WebBinding -Name "MonSite" -Protocol https -Port 443 -HostHeader "monsite.local"
$binding = Get-WebBinding -Name "MonSite" -Protocol https
$binding.AddSslCertificate($cert.Thumbprint, "My")

# Journaux IIS
# Par défaut : C:\\inetpub\\logs\\LogFiles\\W3SVC1\\
Get-ChildItem "C:\\inetpub\\logs\\LogFiles" -Recurse -Filter "*.log" |
  Sort-Object LastWriteTime -Descending | Select-Object -First 5 | ForEach-Object {
    Get-Content $_.FullName | Select-String "500|404|401" | Select-Object -Last 20
  }

# iisreset — redémarrer IIS
iisreset
iisreset /stop
iisreset /start
iisreset /restart /noforce`,
      },
    ],
  },
  {
    id: "analyse-securite-reseau",
    label: "Sécurité Réseau",
    icon: "Shield",
    items: [
      {
        title: "Nmap — scan réseau et découverte",
        solution: [
          "Nmap : outil de scan réseau et de découverte d'hôtes (nmap.org)",
          "Usage autorisé : uniquement sur son propre réseau ou avec autorisation explicite",
          "Découverte d'hôtes : -sn (ping scan, pas de scan de ports)",
          "Scan de ports : -sV (détection des services et versions)",
          "OS fingerprinting : -O (détection du système d'exploitation)",
          "Timing : -T0 (lent/furtif) à -T5 (très rapide/bruyant) — T3 est le défaut",
          "Zenmap : interface graphique de Nmap (inclus dans l'installeur Windows)",
          "Output : -oN (texte), -oX (XML), -oG (grepable) pour sauvegarder les résultats",
        ],
        code: `# Nmap — commandes essentielles (réseau AUTORISÉ uniquement)
# Installer : nmap.org ou winget install Insecure.Nmap

# Découverte des hôtes sur le réseau local
nmap -sn 192.168.1.0/24              # Ping scan (pas de ports)
nmap -sn 192.168.1.0/24 -oN hosts.txt  # Sauvegarder la liste

# Scan de ports courants
nmap 192.168.1.100                   # 1000 ports les plus courants
nmap -p 22,80,443,3389,445 192.168.1.100  # Ports spécifiques
nmap -p 1-65535 192.168.1.100        # Tous les ports (long)
nmap -sV 192.168.1.100               # Détection des services et versions
nmap -sV --version-intensity 9 192.168.1.100  # Détection aggressive

# Scan complet (réseau entier, ports courants + versions + OS)
nmap -sV -O -A 192.168.1.0/24 -oX scan-complet.xml

# Scan UDP (plus lent)
nmap -sU -p 53,67,68,161,162 192.168.1.0/24  # DNS, DHCP, SNMP

# Détecter les vulnérabilités (NSE scripts)
nmap --script vuln 192.168.1.100    # Scripts de vulnérabilités
nmap --script smb-vuln-ms17-010 192.168.1.0/24  # EternalBlue (WannaCry)
nmap --script ssl-cert,ssl-enum-ciphers -p 443 192.168.1.100  # Analyse SSL

# Depuis PowerShell — alternative légère
Test-NetConnection -ComputerName 192.168.1.100 -Port 445  # Test un port
1..1024 | ForEach-Object { Test-NetConnection -ComputerName 192.168.1.100 -Port $_ -WarningAction SilentlyContinue } | Where-Object TcpTestSucceeded`,
      },
      {
        title: "Wireshark — analyse de trafic réseau",
        solution: [
          "Wireshark : analyseur de protocoles réseau, capture et analyse les paquets (wireshark.org)",
          "Npcap (Windows) : driver de capture — installé automatiquement avec Wireshark",
          "Interface : sélectionner l'interface réseau à capturer, filtrer, analyser",
          "Filtres de capture (BPF) : capturent seulement ce qui correspond (plus efficace)",
          "Filtres d'affichage : filtrent l'affichage sans modifier la capture",
          "Follow TCP/UDP Stream : reconstituer une conversation complète",
          "Expert Info : analyse automatique des anomalies (retransmissions, délais, erreurs)",
          "TShark : version CLI de Wireshark pour les scripts et l'automatisation",
        ],
        code: `# Filtres Wireshark utiles (filtres d'affichage)
ip.addr == 192.168.1.100           # Trafic d'une IP
tcp.port == 443                    # HTTPS
http                               # Tout le trafic HTTP
dns                                # Requêtes/réponses DNS
tcp.flags.syn == 1                 # Paquets SYN (nouvelles connexions)
tcp.analysis.retransmission        # Retransmissions TCP (problèmes réseau)
tcp.analysis.zero_window           # Fenêtre TCP à zéro (congestion)
smb || smb2                        # Trafic SMB (partages réseau)
icmp                               # Ping
arp                                # Requêtes ARP (résolution MAC)
!(arp or icmp or dns)              # Exclure les protocoles de bruit
ip.addr == 192.168.1.100 and tcp.port == 80  # Combinaison

# Filtres de capture (BPF) — plus efficaces
host 192.168.1.100
port 443
tcp and host 192.168.1.100
not port 22 and not arp

# TShark — capture CLI
tshark -i Ethernet -w capture.pcap  # Capturer dans un fichier
tshark -r capture.pcap -Y "http" -T fields -e ip.src -e ip.dst -e http.host  # Analyser
tshark -i Ethernet -Y "dns" -T fields -e dns.qry.name -e dns.a  # DNS en temps réel
tshark -r capture.pcap -qz io,stat,1  # Statistiques par seconde

# Analyser une capture existante avec PowerShell + editcap
editcap -c 1000 grosse-capture.pcap partie.pcap  # Découper en fichiers de 1000 paquets`,
        note: "Sur un réseau chiffré (HTTPS), Wireshark voit les métadonnées (IPs, ports, SNI) mais pas le contenu. Pour déchiffrer HTTPS : configurer le SSLKEYLOGFILE dans le navigateur.",
      },
    ],
  },
];
