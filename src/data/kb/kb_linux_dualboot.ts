import type { KBCategory } from "../knowledgeBase";

export const kbLinuxDualboot: KBCategory[] = [
  {
    id: "linux-installation",
    label: "Linux — Installation & Dual Boot",
    icon: "Terminal",
    items: [
      {
        title: "Dual boot Windows / Linux — guide complet",
        solution: [
          "Installer Windows EN PREMIER, puis Linux — Windows écrase le bootloader Linux si inversé",
          "Créer une partition libre depuis Windows : Gestion des disques > Réduire un volume",
          "Télécharger Ubuntu/Fedora/Mint et créer une clé USB bootable avec Rufus (rufus.ie)",
          "Dans le BIOS : désactiver Secure Boot (ou Ubuntu/Fedora le supportent, Arch non)",
          "Fast Startup Windows DOIT être désactivé : Paramètres > Alimentation > changer le comportement du bouton > décocher Démarrage rapide",
          "L'installeur Ubuntu/Fedora détecte Windows et installe GRUB automatiquement",
          "GRUB 2 : bootloader qui propose Windows ou Linux au démarrage",
          "Réparer GRUB si disparu : booter sur une live USB Linux et exécuter les commandes",
        ],
        code: `# Désactiver Fast Startup Windows (OBLIGATOIRE avant dual boot)
powercfg /hibernate off
reg add "HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power" /v HiberbootEnabled /t REG_DWORD /d 0 /f

# Créer une partition pour Linux depuis PowerShell
# 50 Go de libre recommandé pour Linux
# Option 1 : Gestion des disques Windows (GUI)
diskmgmt.msc

# Option 2 : diskpart
diskpart
list disk
select disk 0
list partition
select partition 3  # La partition Windows principale
shrink desired=51200  # Libérer 50 Go (en Mo)
exit

# Vérifier la table de partition (doit être GPT pour dual boot moderne)
Get-Disk | Select Number, PartitionStyle, Size

# Réparer GRUB depuis live USB Ubuntu
sudo mount /dev/sda5 /mnt          # Monter la partition Linux root
sudo mount /dev/sda1 /mnt/boot/efi # Monter la partition EFI
sudo mount --bind /dev /mnt/dev
sudo mount --bind /proc /mnt/proc
sudo mount --bind /sys /mnt/sys
sudo chroot /mnt
grub-install /dev/sda
update-grub
exit

# Réparer le bootloader Windows si GRUB écrasé
# Booter sur Windows PE/ISO
bootrec /fixmbr
bootrec /fixboot
bootrec /scanos
bootrec /rebuildbcd`,
        note: "Pour éviter que Windows Update écrase GRUB : dans BIOS, mettre GRUB (Linux) premier dans l'ordre de boot. Windows Update peut remettre Windows en premier.",
      },
      {
        title: "Ubuntu — commandes essentielles",
        solution: [
          "apt : gestionnaire de paquets Ubuntu/Debian (sudo apt install/remove/update/upgrade)",
          "snap : paquets universels conteneurisés (snap install vlc)",
          "Mise à jour complète : sudo apt update && sudo apt upgrade -y",
          "Chercher un paquet : apt search nom ou apt-cache search nom",
          "Infos sur un paquet : apt show nom",
          "Lister les installés : apt list --installed",
          "sudo (SuperUser DO) : exécuter une commande en root",
          "chmod/chown : modifier les permissions/propriétaires de fichiers",
        ],
        code: `# Gestion des paquets
sudo apt update                    # Mettre à jour la liste des paquets
sudo apt upgrade -y                # Mettre à jour tous les paquets
sudo apt install vlc git curl      # Installer des paquets
sudo apt remove vlc                # Désinstaller
sudo apt purge vlc                 # Désinstaller + supprimer la config
sudo apt autoremove                # Supprimer les dépendances inutilisées
sudo apt autoclean                 # Nettoyer le cache

# Navigation système de fichiers
ls -la                             # Lister avec détails et fichiers cachés
cd /chemin/vers/dossier
pwd                                # Répertoire courant
find / -name "fichier.txt" 2>/dev/null  # Rechercher un fichier
locate fichier.txt                 # Recherche rapide (via index)

# Permissions
chmod 755 script.sh                # rwxr-xr-x
chmod +x script.sh                 # Rendre exécutable
chown user:group fichier.txt       # Changer propriétaire
sudo chown -R user:user /dossier   # Récursivement

# Réseau
ip addr                            # Interfaces réseau (remplace ifconfig)
ip route                           # Table de routage
ping google.com
curl ifconfig.me                   # Voir l'IP publique
ss -tulnp                          # Ports en écoute (remplace netstat)
sudo ufw enable                    # Activer le pare-feu
sudo ufw allow 22/tcp              # Autoriser SSH
sudo ufw status

# Services (systemd)
systemctl status nginx             # État du service
sudo systemctl start nginx
sudo systemctl stop nginx
sudo systemctl enable nginx        # Démarrer au boot
sudo systemctl disable nginx
journalctl -u nginx -f             # Logs du service en temps réel
journalctl -xe                     # Derniers logs système avec erreurs`,
      },
      {
        title: "GRUB — personnalisation et dépannage",
        code: `# Modifier le timeout GRUB (fichier /etc/default/grub)
sudo nano /etc/default/grub

# Modifier :
GRUB_TIMEOUT=10                    # Secondes avant le boot auto
GRUB_DEFAULT=0                     # 0=premier OS (Linux), ou "Windows Boot Manager"
GRUB_TIMEOUT_STYLE=menu            # Toujours afficher le menu

# Appliquer les changements
sudo update-grub

# Changer l'OS par défaut sur Windows
# Lister les entrées GRUB
grep -E "menuentry|submenu" /boot/grub/grub.cfg | head -20
# Trouver l'index de Windows (ex: "Windows Boot Manager" est l'entrée 2)
sudo grub-set-default 2
sudo update-grub

# GRUB Customizer (GUI)
sudo add-apt-repository ppa:danielrichter2007/grub-customizer
sudo apt install grub-customizer

# Accéder aux options de démarrage Linux en urgence
# Au menu GRUB, appuyer E pour éditer
# Trouver la ligne "linux" et ajouter à la fin : single
# Ou : init=/bin/bash
# Ctrl+X pour booter — root shell sans mot de passe`,
        solution: [
          "GRUB_DEFAULT=saved + grub-set-default : mémoriser le dernier OS booté",
          "GRUB_HIDDEN_TIMEOUT=0 : masquer GRUB (appuyer Esc pour l'afficher)",
          "/etc/grub.d/ : scripts qui génèrent grub.cfg (ne pas modifier grub.cfg directement)",
          "os-prober : outil qui détecte les autres OS (Windows) pour les ajouter à GRUB",
          "EFI/Windows Boot Manager : dans le BIOS, on peut choisir quel bootloader est prioritaire",
        ],
      },
      {
        title: "Linux — SSH et administration à distance",
        code: `# Installer SSH server sur Ubuntu
sudo apt install openssh-server
sudo systemctl enable ssh
sudo systemctl start ssh

# Configuration SSH sécurisée (/etc/ssh/sshd_config)
# Modifier ces paramètres :
Port 2222                          # Changer le port par défaut
PermitRootLogin no                 # Désactiver login root
PasswordAuthentication no          # Forcer les clés SSH
PubkeyAuthentication yes
MaxAuthTries 3

sudo systemctl restart ssh

# Copier sa clé publique vers le serveur
ssh-copy-id -p 2222 user@serveur.com
# Ou manuellement :
cat ~/.ssh/id_ed25519.pub | ssh user@serveur "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"

# Connexion
ssh -p 2222 user@serveur.com

# Tunnels SSH
ssh -L 8080:localhost:80 user@serveur.com    # Rediriger port local → serveur
ssh -R 2222:localhost:22 user@serveur.com    # Exposer un port local via serveur
ssh -D 1080 user@serveur.com                 # Proxy SOCKS5 via serveur

# scp — copier des fichiers
scp -P 2222 fichier.txt user@serveur:/home/user/
scp -P 2222 -r dossier/ user@serveur:/home/user/
scp -P 2222 user@serveur:/home/user/fichier.txt ./

# rsync — synchronisation efficace
rsync -avz --progress -e "ssh -p 2222" ./local/ user@serveur:/remote/
rsync -avz --delete -e "ssh -p 2222" ./local/ user@serveur:/remote/ # Sync miroir`,
      },
    ],
  },
  {
    id: "wsl2-avance",
    label: "WSL2 — Usage Avancé",
    icon: "Terminal",
    items: [
      {
        title: "WSL2 — configuration et optimisation",
        solution: [
          "WSL2 : Windows Subsystem for Linux version 2, utilise un vrai kernel Linux dans une VM légère",
          "Installer : wsl --install (Windows 11) ou via PowerShell sur Win 10",
          "Distros disponibles : Ubuntu (défaut), Debian, Fedora (via winget), Kali Linux, Alpine",
          "Accéder aux fichiers Windows depuis WSL : /mnt/c/ correspond à C:\\",
          "Accéder aux fichiers WSL depuis Windows : \\\\wsl$\\Ubuntu\\ dans l'Explorateur",
          "Performances : les fichiers dans le filesystem WSL (ext4) sont 10x plus rapides que /mnt/c/",
          "GPU via WSL2 : nvidia-smi fonctionne, peut utiliser CUDA dans WSL2",
          "WSLg : interface graphique Linux dans Windows (apps Linux avec fenêtres natives)",
        ],
        code: `# Installation complète WSL2
wsl --install                      # Installe Ubuntu par défaut
wsl --install -d Debian            # Distro spécifique
wsl --list --online                # Distros disponibles
wsl --list --verbose               # Distros installées avec version WSL

# Gestion des distros
wsl --set-default Ubuntu           # Distro par défaut
wsl --set-version Ubuntu 2         # Passer en WSL2
wsl --export Ubuntu ubuntu-backup.tar    # Sauvegarder
wsl --import UbuntuRestore D:\\WSL ubuntu-backup.tar  # Restaurer
wsl --unregister Ubuntu            # Supprimer (DÉTRUIT les données !)
wsl --terminate Ubuntu             # Arrêter sans supprimer
wsl --shutdown                     # Arrêter toutes les distros

# Fichier .wslconfig (dans C:\\Users\\Nom\\.wslconfig)
# Limiter les ressources
[wsl2]
memory=4GB
processors=4
swap=2GB
localhostForwarding=true

# Dans la distro WSL2 : fichier /etc/wsl.conf
[automount]
enabled=true
options="metadata,umask=22,fmask=11"
[network]
generateResolvConf=true
[interop]
enabled=true
appendWindowsPath=true

# Redémarrer WSL après modification de la config
wsl --shutdown

# Lancer des commandes Windows depuis WSL
explorer.exe .                     # Ouvrir l'Explorateur Windows ici
notepad.exe fichier.txt           # Ouvrir dans Notepad
powershell.exe -c "Get-Process"   # PowerShell depuis WSL

# Lancer des commandes WSL depuis Windows
wsl ls -la /home/user
wsl -- bash -c "cd /home/user && npm install"`,
      },
      {
        title: "WSL2 — développement web avec Docker",
        code: `# Docker Desktop avec intégration WSL2
# Dans Docker Desktop > Settings > Resources > WSL Integration
# Activer pour la distro Ubuntu

# Vérifier depuis WSL2
docker --version
docker compose version

# Workflow de développement dans WSL2
# 1. Cloner le projet DANS WSL (pas sur /mnt/c !)
cd ~
git clone https://github.com/user/projet.git
cd projet

# 2. Ouvrir dans VS Code depuis WSL
code .                             # Lance VS Code avec l'extension WSL

# 3. Node.js dans WSL (via nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.bashrc
nvm install 20
nvm use 20
npm install
npm run dev

# 4. Accéder au serveur WSL depuis Windows
# Le port est automatiquement forwardé
# http://localhost:3000 fonctionne dans le navigateur Windows

# Performance — utiliser le filesystem WSL
# ✅ BON : ~/projects/mon-app (filesystem ext4 WSL2)
# ❌ MAUVAIS : /mnt/c/Users/Momo/projects (cross-OS, lent)

# Alias utiles dans ~/.bashrc
echo 'alias ll="ls -la --color=auto"' >> ~/.bashrc
echo 'alias desk="cd /mnt/c/Users/\$USER/Desktop"' >> ~/.bashrc
echo 'alias docs="cd /mnt/c/Users/\$USER/Documents"' >> ~/.bashrc
source ~/.bashrc`,
      },
    ],
  },
];
