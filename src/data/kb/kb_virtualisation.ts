import type { KBCategory } from "../knowledgeBase";

export const kbVirtualisation: KBCategory[] = [
  {
    id: "virtualbox-vmware",
    label: "VirtualBox & VMware",
    icon: "Server",
    items: [
      {
        title: "VirtualBox — créer et configurer une VM",
        solution: [
          "Télécharger VirtualBox depuis virtualbox.org (gratuit, open source)",
          "RAM : allouer 25-50% de la RAM physique max à la VM",
          "Processeurs : 2-4 cœurs pour un usage normal, 50% des cœurs max",
          "Disque : VDI dynamique (grandit selon les besoins) ou fixe (plus performant)",
          "Réseau : NAT (accès internet via host), Bridge (IP propre sur le réseau local), Host-only (réseau isolé host↔VM)",
          "Extensions Guest Additions : améliore performances, active le copier-coller, le dossier partagé",
          "Snapshot : capture l'état de la VM — revenir en arrière en cas de problème",
          "USB 3.0, Clipboard bidirectionnel : activer dans les Settings de la VM",
        ],
        code: `# VBoxManage — CLI pour VirtualBox
# Lister les VMs
VBoxManage list vms
VBoxManage list runningvms

# Créer une VM Ubuntu
VBoxManage createvm --name "Ubuntu 24.04" --ostype Ubuntu_64 --register
VBoxManage modifyvm "Ubuntu 24.04" --memory 4096 --cpus 2 --vram 128
VBoxManage modifyvm "Ubuntu 24.04" --nic1 nat --clipboard bidirectional

# Créer et attacher un disque
VBoxManage createhd --filename "Ubuntu-disk.vdi" --size 50000 --variant Standard
VBoxManage storagectl "Ubuntu 24.04" --name "SATA" --add sata
VBoxManage storageattach "Ubuntu 24.04" --storagectl "SATA" --port 0 --device 0 --type hdd --medium "Ubuntu-disk.vdi"

# Démarrer/arrêter
VBoxManage startvm "Ubuntu 24.04"
VBoxManage startvm "Ubuntu 24.04" --type headless  # Sans interface graphique
VBoxManage controlvm "Ubuntu 24.04" poweroff
VBoxManage controlvm "Ubuntu 24.04" savestate      # Pause/suspend

# Snapshots
VBoxManage snapshot "Ubuntu 24.04" take "Avant mise à jour"
VBoxManage snapshot "Ubuntu 24.04" list
VBoxManage snapshot "Ubuntu 24.04" restore "Avant mise à jour"
VBoxManage snapshot "Ubuntu 24.04" delete "Avant mise à jour"

# Dossier partagé
VBoxManage sharedfolder add "Ubuntu 24.04" --name "Partage" --hostpath "C:\\Partage" --automount`,
        note: "Installer les Guest Additions dans la VM : Périphériques > Insérer l'image CD des additions invité, puis exécuter l'installeur dans l'OS invité.",
      },
      {
        title: "VMware Workstation Player — configuration",
        solution: [
          "VMware Workstation Player : gratuit pour usage personnel (vmware.com)",
          "VMware Pro : payant, ajoute les snapshots, les réseaux virtuels, le mode clone",
          "Easy Install : VMware détecte l'ISO et automatise l'installation de l'OS",
          "VMware Tools : équivalent aux Guest Additions — améliore les performances et l'intégration",
          "Mode Unity : les fenêtres de la VM s'intègrent dans le bureau Windows host",
          "vCenter Converter : convertir un PC physique en VM (P2V)",
          "Préférer VMware pour les VMs Windows (meilleure compatibilité), VirtualBox pour Linux",
        ],
        code: `# Convertir un VDI VirtualBox en VMDK VMware
VBoxManage clonehd "vm.vdi" "vm.vmdk" --format VMDK

# VMware VMs en ligne de commande (vmrun)
vmrun start "C:\\VMs\\Ubuntu\\Ubuntu.vmx"
vmrun stop "C:\\VMs\\Ubuntu\\Ubuntu.vmx"
vmrun suspend "C:\\VMs\\Ubuntu\\Ubuntu.vmx"
vmrun snapshot "C:\\VMs\\Ubuntu\\Ubuntu.vmx" "snapshot_name"
vmrun revertToSnapshot "C:\\VMs\\Ubuntu\\Ubuntu.vmx" "snapshot_name"
vmrun listSnapshots "C:\\VMs\\Ubuntu\\Ubuntu.vmx"

# Transfert de fichiers host → VM
vmrun copyFileFromHostToGuest "C:\\VMs\\Ubuntu\\Ubuntu.vmx" "C:\\fichier.txt" "/home/user/fichier.txt"

# Exécuter une commande dans la VM
vmrun runProgramInGuest "C:\\VMs\\Ubuntu\\Ubuntu.vmx" "/bin/bash" "-c" "apt update && apt upgrade -y"`,
      },
      {
        title: "Réseau virtuel — modes et configuration avancée",
        solution: [
          "NAT : la VM accède à Internet via l'adresse IP du host — simple, aucune config réseau",
          "Bridge (Bridged) : la VM obtient une IP propre du DHCP réseau — visible sur le réseau local",
          "Host-Only : réseau entre host et VM uniquement — aucun accès Internet pour la VM",
          "Internal : réseau entre VMs uniquement — isolé du host et d'Internet",
          "NAT Network : comme NAT mais les VMs peuvent communiquer entre elles",
          "Port Forwarding (NAT) : exposer un port de la VM vers le host (ex: VM:22 → Host:2222)",
          "Use case : web dev → Bridge ; environnement de test isolé → Host-Only ; lab multi-VM → Internal",
        ],
        code: `# VirtualBox — configurer le port forwarding NAT
VBoxManage modifyvm "MaVM" --natpf1 "SSH,tcp,,2222,,22"
VBoxManage modifyvm "MaVM" --natpf1 "Web,tcp,,8080,,80"
VBoxManage modifyvm "MaVM" --natpf1 "HTTPS,tcp,,8443,,443"

# Ajouter un adaptateur Host-Only en plus du NAT
VBoxManage modifyvm "MaVM" --nic2 hostonly --hostonlyadapter2 "VirtualBox Host-Only Ethernet Adapter"

# Créer un réseau Host-Only personnalisé
VBoxManage hostonlyif create
VBoxManage hostonlyif ipconfig "VirtualBox Host-Only Ethernet Adapter #2" --ip 192.168.99.1 --netmask 255.255.255.0

# Se connecter en SSH à la VM (via port forwarding)
ssh -p 2222 user@localhost

# Vérifier la connectivité réseau dans la VM
ip addr show          # Linux
ipconfig /all         # Windows invité`,
      },
      {
        title: "Vagrant — VMs reproductibles avec code",
        solution: [
          "Vagrant : outil pour créer et gérer des VMs via un fichier de configuration (Vagrantfile)",
          "Permet de définir une VM entière en code — partageable via Git",
          "Compatible VirtualBox (par défaut), VMware, Hyper-V, libvirt",
          "Vagrant Cloud : bibliothèque de boxes préconfigurées (Ubuntu, CentOS, Windows...)",
          "Idéal pour les environnements de développement reproductibles",
          "vagrant up : télécharge la box si absente et démarre la VM",
          "vagrant ssh : se connecter directement en SSH sans config",
          "vagrant destroy : supprimer complètement la VM",
        ],
        code: `# Installation
# 1. Installer VirtualBox
# 2. Télécharger Vagrant depuis vagrantup.com

# Vagrantfile basique (Ubuntu 24.04)
Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/jammy64"
  config.vm.network "private_network", ip: "192.168.33.10"
  config.vm.network "forwarded_port", guest: 80, host: 8080
  config.vm.synced_folder ".", "/vagrant"

  config.vm.provider "virtualbox" do |vb|
    vb.memory = "2048"
    vb.cpus = 2
    vb.name = "dev-ubuntu"
  end

  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y nginx git curl
    systemctl enable nginx
  SHELL
end

# Commandes Vagrant
vagrant up                  # Créer et démarrer
vagrant halt                # Arrêter (garde le disque)
vagrant destroy -f          # Supprimer complètement
vagrant ssh                 # Se connecter
vagrant status              # État
vagrant reload --provision  # Redémarrer + re-provisionner
vagrant snapshot save "avant-modification"
vagrant snapshot restore "avant-modification"
vagrant box list            # Boxes téléchargées
vagrant box update          # Mettre à jour les boxes`,
      },
    ],
  },
  {
    id: "docker-containers",
    label: "Docker & Containers",
    icon: "Box",
    items: [
      {
        title: "Docker — concepts avancés",
        code: `# Volumes — persistance des données
docker volume create mon-volume
docker run -v mon-volume:/data nginx
docker run -v /host/path:/container/path nginx  # Bind mount
docker volume ls
docker volume inspect mon-volume
docker volume rm mon-volume

# Réseaux Docker
docker network create mon-reseau
docker run --network mon-reseau --name db postgres
docker run --network mon-reseau --name app --link db myapp
docker network ls
docker network inspect mon-reseau

# Docker images — build et optimisation
docker build -t mon-app:1.0 .
docker build -t mon-app:1.0 --no-cache .      # Sans cache
docker images                                   # Lister les images
docker image prune                              # Supprimer images inutilisées
docker system prune -a                          # Nettoyer TOUT (images, containers, volumes)

# Inspecter un container
docker inspect <container>
docker stats                                    # Utilisation CPU/RAM en temps réel
docker top <container>                          # Processus dans le container

# Copier des fichiers
docker cp container:/fichier ./local
docker cp ./local container:/fichier

# Dockerfile multi-stage (réduit la taille finale)
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20-alpine AS production
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
EXPOSE 3000
CMD ["node", "dist/index.js"]`,
        solution: [
          "Multi-stage build : sépare l'environnement de build de l'image finale — image 10x plus petite",
          ".dockerignore : exclure node_modules, .git, .env de l'image",
          "Layer caching : copier package.json AVANT le reste du code pour utiliser le cache",
          "alpine : images Linux minimales, 5-10x plus petites que debian/ubuntu",
          "COPY vs ADD : toujours préférer COPY sauf pour les archives TAR",
          "ENTRYPOINT vs CMD : ENTRYPOINT = commande fixe, CMD = arguments par défaut",
        ],
      },
      {
        title: "Docker Compose — orchestration multi-services",
        code: `# docker-compose.yml complet (stack web)
version: '3.9'

services:
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - app
    restart: unless-stopped

  app:
    build:
      context: .
      target: production
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgresql://user:pass@db:5432/mydb
      - REDIS_URL=redis://redis:6379
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
    restart: unless-stopped

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: mydb
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:

# Commandes
docker compose up -d           # Démarrer en arrière-plan
docker compose logs -f app     # Suivre les logs d'un service
docker compose ps              # État des services
docker compose exec app bash   # Shell dans un service
docker compose restart app     # Redémarrer un service
docker compose down -v         # Arrêter + supprimer les volumes`,
      },
      {
        title: "Docker — débogage et optimisation",
        code: `# Diagnostiquer les problèmes
docker logs <container> --tail 50 -f
docker events                                   # Événements Docker en temps réel
docker inspect <container> | jq '.[0].State'   # État du container

# Container qui ne démarre pas
docker run --rm -it <image> sh                  # Lancer un shell interactif
docker run --rm -it --entrypoint sh <image>     # Forcer le shell comme entrypoint

# Limiter les ressources
docker run --memory="512m" --cpus="1.5" nginx

# Sauvegarder/restaurer un container
docker commit <container> mon-backup:latest
docker save mon-backup:latest | gzip > backup.tar.gz
docker load < backup.tar.gz

# Registry privé
docker tag mon-app:1.0 registry.exemple.com/mon-app:1.0
docker push registry.exemple.com/mon-app:1.0
docker pull registry.exemple.com/mon-app:1.0

# Analyser la taille d'une image layer par layer
docker history mon-app:1.0
dive mon-app:1.0  # Outil tiers: github.com/wagoodman/dive

# Scan de sécurité
docker scout cves mon-app:1.0     # Docker Scout (intégré)
trivy image mon-app:1.0           # Trivy (outil tiers)`,
        note: "docker system df : voir l'espace utilisé par Docker. docker system prune -a --volumes : libérer tout l'espace (ATTENTION : supprime tout).",
      },
    ],
  },
  {
    id: "hyperv-windows",
    label: "Hyper-V Windows",
    icon: "Server",
    items: [
      {
        title: "Hyper-V — activation et configuration",
        solution: [
          "Hyper-V : hyperviseur natif Windows (Pro, Enterprise, Education seulement)",
          "Activer : Activer ou désactiver des fonctionnalités Windows > Hyper-V (toutes les cases)",
          "Ou via PowerShell : Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All",
          "Redémarrage requis après activation",
          "Hyper-V et VirtualBox/VMware peuvent coexister sur Windows 11 (depuis v21H1 avec WHP)",
          "Hyper-V Manager ou Windows Admin Center pour gérer les VMs",
          "Commutateurs virtuels : External (bridge), Internal (host+VMs), Private (VMs uniquement)",
          "Generation 1 (BIOS/MBR) vs Generation 2 (UEFI/GPT) : toujours Gen 2 pour les OS modernes",
        ],
        code: `# Activer Hyper-V
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All

# Créer une VM via PowerShell
New-VM -Name "Ubuntu-Dev" -Generation 2 -MemoryStartupBytes 4GB -Path "D:\\VMs"
Set-VMProcessor "Ubuntu-Dev" -Count 4
Set-VMMemory "Ubuntu-Dev" -DynamicMemoryEnabled $true -MinimumBytes 2GB -MaximumBytes 8GB

# Créer et attacher un disque virtuel
New-VHD -Path "D:\\VMs\\Ubuntu-Dev\\disk.vhdx" -SizeBytes 60GB -Dynamic
Add-VMHardDiskDrive -VMName "Ubuntu-Dev" -Path "D:\\VMs\\Ubuntu-Dev\\disk.vhdx"

# Créer un commutateur virtuel
New-VMSwitch -Name "LAN" -NetAdapterName "Ethernet" -SwitchType External
Connect-VMNetworkAdapter -VMName "Ubuntu-Dev" -SwitchName "LAN"

# Ajouter un lecteur DVD (ISO)
Add-VMDvdDrive -VMName "Ubuntu-Dev" -Path "D:\\ISO\\ubuntu-24.04.iso"
Set-VMFirmware "Ubuntu-Dev" -FirstBootDevice (Get-VMDvdDrive "Ubuntu-Dev")

# Démarrer
Start-VM "Ubuntu-Dev"

# Checkpoints (snapshots)
Checkpoint-VM "Ubuntu-Dev" -SnapshotName "Avant mise à jour"
Get-VMCheckpoint "Ubuntu-Dev"
Restore-VMCheckpoint "Ubuntu-Dev" -Name "Avant mise à jour" -Confirm:$false
Remove-VMCheckpoint "Ubuntu-Dev" -Name "Avant mise à jour"`,
      },
      {
        title: "Windows Sandbox — environnement jetable",
        solution: [
          "Windows Sandbox : VM légère Windows 10/11 jetable, réinitialisée à chaque fermeture",
          "Nécessite Windows 10/11 Pro, Enterprise ou Education",
          "Activer : Fonctionnalités Windows > Windows Sandbox",
          "Démarrer : chercher 'Windows Sandbox' dans le menu Démarrer",
          "Usage : tester des logiciels suspects, parcourir des sites risqués, tester des configs",
          "Les fichiers créés dans le Sandbox sont perdus à la fermeture",
          "Configurer via fichier .wsb : partage de dossiers, réseau désactivé, scripts de démarrage",
          "Copier-coller bidirectionnel avec le host activé par défaut",
        ],
        code: `# Fichier .wsb — configuration avancée du Sandbox
<!-- Sauvegarder en .wsb et double-cliquer pour lancer -->
<Configuration>
  <!-- Désactiver le réseau (sécurité maximale) -->
  <Networking>Disable</Networking>

  <!-- Partager un dossier host en lecture seule -->
  <MappedFolders>
    <MappedFolder>
      <HostFolder>C:\\Tools</HostFolder>
      <SandboxFolder>C:\\Tools</SandboxFolder>
      <ReadOnly>true</ReadOnly>
    </MappedFolder>
  </MappedFolders>

  <!-- Lancer un script au démarrage -->
  <LogonCommand>
    <Command>C:\\Tools\\setup.bat</Command>
  </LogonCommand>

  <!-- Désactiver le GPU virtuel (plus léger) -->
  <VGpu>Disable</VGpu>

  <!-- Désactiver le copier-coller -->
  <ClipboardRedirection>Disable</ClipboardRedirection>
</Configuration>

# Activer/désactiver Windows Sandbox via PowerShell
Enable-WindowsOptionalFeature -FeatureName "Containers-DisposableClientVM" -All -Online
Disable-WindowsOptionalFeature -FeatureName "Containers-DisposableClientVM" -Online`,
      },
    ],
  },
];
