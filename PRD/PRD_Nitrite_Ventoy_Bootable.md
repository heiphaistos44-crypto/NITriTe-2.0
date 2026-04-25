# PRD — NiTriTe Bootable USB avec Ventoy (Dual-Boot)
**Version produit** : 27.0.0  
**Basé sur** : NiTriTe v26.44.0 + Windows 11 PE (NitritePE)  
**Statut** : Draft — 2026-03-24  
**Auteur** : Momo  
**Inspiré de** : Hiren's Boot CD PE  

---

## 1. Vision & Objectif

Transformer NiTriTe en une **solution de démarrage complète sur clé USB**, inspirée de **Hiren's Boot CD PE**, offrant au technicien un choix au démarrage :

| Option de boot | Description |
|---------------|-------------|
| 🖥️ **Mini Windows 11 PE** | Environnement Windows 11 complet avec bureau, explorateur, réseau — NiTriTe se lance automatiquement dedans |
| ⚡ **NiTriTe Direct** | Boot minimaliste ultra-rapide directement dans NiTriTe (WinPE allégé, sans bureau Windows) |

Le tout géré par **Ventoy**, le bootloader multi-image qui permet de maintenir plusieurs images bootables sur une seule clé USB et de les mettre à jour simplement en copiant de nouvelles versions.

---

## 2. Pourquoi Ventoy ?

### 2.1 Avantages vs Rufus/ISO unique

| Critère | ISO unique (Rufus) | Ventoy |
|---------|-------------------|--------|
| Mise à jour | Reformater la clé | Copier le nouvel ISO |
| Multi-boot | ❌ 1 seule image | ✅ Illimité |
| Ajout d'outils | Refaire l'ISO | Copier un nouvel ISO |
| Compatibilité | UEFI ou BIOS | UEFI + BIOS automatique |
| Espace libre | Inutilisable | Partition données accessible |
| Maintenance | Complexe | Drag & drop |
| Persistence | Non | Possible (plugin) |

### 2.2 Comparaison avec Hiren's Boot CD PE

| Aspect | Hiren's Boot CD PE | NiTriTe Bootable |
|--------|-------------------|-----------------|
| Base | Win 10/11 PE ISO unique | 2 ISOs Ventoy : Win11 PE + NiTriTe Direct |
| Outils | ~100 outils portables intégrés | NiTriTe (diagnostic, réparation, récupération) + outils portables |
| Personnalisation | Figée (ISO signé) | Modulaire (ajouter/retirer des ISOs) |
| Mise à jour | Attendre nouvelle version | Recompiler + copier l'ISO |
| Boot manager | Bootmgr standard | Ventoy (GRUB2 customisé) |
| Taille | ~3 GB | ~2-3 GB (2 ISOs) |

---

## 3. Architecture globale

### 3.1 Structure de la clé USB Ventoy

```
📁 Clé USB (ex: 16 GB minimum)
│
├── 📁 ventoy/                          ← Configuration Ventoy (partition 1)
│   ├── ventoy.json                     ← Config principale (menu, thème, alias)
│   ├── ventoy_grub.cfg                 ← Menu GRUB2 personnalisé (optionnel)
│   ├── ventoy_wimboot.img              ← Plugin WIMBOOT pour WinPE
│   └── 📁 theme/                       ← Thème GRUB2 NiTriTe
│       ├── theme.txt
│       ├── background.png              ← Fond d'écran menu boot NiTriTe
│       ├── font.pf2
│       ├── icons/
│       │   ├── nitrite.png
│       │   ├── windows.png
│       │   └── tools.png
│       └── select_c.png               ← Curseur de sélection
│
├── 📁 ISO/                             ← Images bootables
│   ├── NitritePE_Win11.iso             ← Mini Windows 11 PE avec NiTriTe intégré
│   └── NiTriTe_Direct_WinPE.iso        ← Boot WinPE minimaliste → NiTriTe direct
│
├── 📁 Applications/                    ← Outils portables accessibles depuis Win11PE
│   ├── 📁 Nitrite_v26.44.0/
│   │   ├── Nitrite.exe
│   │   ├── Drivers/
│   │   ├── Script Windows/
│   │   ├── config/
│   │   └── logiciel/
│   ├── 📁 UniversalConverter Portable/
│   └── 📁 Extras/                      ← Outils additionnels
│       ├── Autoruns.exe
│       ├── ProcessExplorer.exe
│       ├── CrystalDiskInfo.exe
│       └── HWiNFO64.exe
│
├── 📁 Drivers/                         ← Pack drivers réseau/stockage
│   ├── 📁 Network/
│   ├── 📁 Storage/
│   ├── 📁 Chipset/
│   └── 📁 USB/
│
└── 📁 Scripts/                         ← Scripts utilitaires
    ├── repair_boot.bat
    ├── scan_offline.bat
    └── network_setup.bat
```

### 3.2 Schéma de boot

```
┌─────────────────────────────────────────────────────────────┐
│                        BIOS / UEFI                          │
│                     Boot sur clé USB                        │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    VENTOY BOOT MENU                         │
│              (Thème personnalisé NiTriTe)                   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  🖥️  Mini Windows 11 — NiTriTe Desktop              │    │
│  │      Environnement complet avec bureau Windows      │    │
│  ├─────────────────────────────────────────────────────┤    │
│  │  ⚡  NiTriTe Direct — Boot rapide                    │    │
│  │      Lancement direct de NiTriTe sans bureau        │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                             │
│  Ventoy v1.x.x  |  UEFI Secure Boot  |  F5: Thèmes        │
└────────────────────────┬──────────┬─────────────────────────┘
                         │          │
              Choix 1    │          │    Choix 2
                         ▼          ▼
┌──────────────────────────┐ ┌──────────────────────────────┐
│   NitritePE_Win11.iso    │ │  NiTriTe_Direct_WinPE.iso    │
│                          │ │                              │
│  Windows 11 PE complet   │ │  WinPE minimaliste           │
│  ├── Bureau Explorer     │ │  ├── Pas de bureau           │
│  ├── Barre des tâches    │ │  ├── NiTriTe plein écran     │
│  ├── Réseau WiFi/Eth     │ │  ├── Boot < 30 secondes     │
│  ├── NiTriTe auto-start  │ │  ├── RAM < 512 MB           │
│  └── Outils portables    │ │  └── Mode récupération pur  │
│      accessibles         │ │                              │
└──────────────────────────┘ └──────────────────────────────┘
```

---

## 4. Description détaillée des deux ISOs

### 4.1 ISO 1 : NitritePE_Win11.iso — Mini Windows 11 PE

**Base** : Le Windows 11 PE existant dans `Nitrite bootable/NitritePE/`

C'est l'équivalent de ce que fait **Hiren's Boot CD PE** : un Windows 11 PE complet avec un vrai bureau, l'explorateur Windows, le réseau, et tous les outils intégrés.

#### Contenu de l'ISO

```
NitritePE_Win11.iso
├── boot/                           ← Bootloader Windows PE
│   ├── BCD
│   ├── boot.sdi
│   └── bootfix.bin
├── EFI/                            ← Boot UEFI
│   └── Boot/
│       └── bootx64.efi
├── sources/
│   └── boot.wim                    ← Image WinPE customisée (~340 MB → ~500 MB après customisation)
│       │
│       │  ── Contenu du WIM (monté) ──
│       │
│       ├── Windows/
│       │   ├── explorer.exe         ← Bureau Windows 11
│       │   └── System32/
│       │       ├── startnet.cmd     ← Init réseau + environnement
│       │       ├── winpeshl.ini     ← Lancement explorer.exe
│       │       ├── config/
│       │       │   ├── SOFTWARE     ← Registre (shell=explorer.exe)
│       │       │   └── SYSTEM       ← Registre (env vars WebView2)
│       │       ├── dwmapi.dll       ← DLLs pour bureau complet
│       │       ├── ExplorerFrame.dll
│       │       └── ... (DLLs shell Win11)
│       │
│       ├── Nitrite/
│       │   ├── nitrite.exe          ← Application NiTriTe
│       │   ├── WebView2/           ← Runtime WebView2 embarqué (Fixed Version)
│       │   ├── logiciel/           ← Apps portables
│       │   ├── Drivers/            ← Drivers embarqués
│       │   └── Script Windows/     ← Scripts réparation
│       │
│       ├── Programs/               ← Outils portables (comme Hiren's)
│       │   ├── Autoruns.exe
│       │   ├── ProcessExplorer.exe
│       │   ├── CrystalDiskInfo.exe
│       │   ├── HWiNFO64.exe
│       │   ├── Rufus.exe
│       │   ├── 7zFM.exe
│       │   └── Notepad++.exe
│       │
│       └── Users/Default/          ← Profil utilisateur par défaut
│           ├── Desktop/
│           │   ├── NiTriTe.lnk     ← Raccourci bureau
│           │   └── Outils/         ← Dossier raccourcis outils
│           └── AppData/
│               ├── Local/
│               └── Roaming/
│
└── bootmgr / bootmgr.efi          ← Boot managers
```

#### Comportement au boot

1. WinPE démarre et initialise le matériel (`wpeinit`)
2. `startnet.cmd` configure l'environnement (chemins, env vars, réseau)
3. `explorer.exe` se lance (bureau complet avec barre des tâches)
4. **NiTriTe se lance automatiquement** via clé registre `Run`
5. L'utilisateur a un bureau Windows complet :
   - Peut ouvrir l'explorateur de fichiers pour naviguer les disques
   - Peut lancer des outils portables depuis le bureau/menu démarrer
   - NiTriTe fonctionne en fenêtre (peut être réduit/agrandi)
   - Accès au réseau (WiFi/Ethernet si drivers injectés)
   - Peut ouvrir cmd/PowerShell indépendamment

#### Fonctionnalités disponibles

| Fonctionnalité | Disponible | Notes |
|---------------|-----------|-------|
| Bureau Windows complet | ✅ | Explorer.exe + DLLs shell |
| Explorateur de fichiers | ✅ | Navigation disques, copie fichiers |
| NiTriTe (toutes fonctions WinPE) | ✅ | Auto-lancé au démarrage |
| Réseau Ethernet | ✅ | Via drivers injectés |
| Réseau WiFi | ⚠️ | Nécessite drivers WiFi spécifiques |
| Outils portables (bureau) | ✅ | Accessibles via raccourcis bureau |
| PowerShell / CMD | ✅ | Disponible nativement |
| Résolution écran | ✅ | Jusqu'à 1920x1080 |
| Multi-fenêtres | ✅ | Prise en charge Alt+Tab |
| Son | ❌ | Non disponible en WinPE |
| Impression | ❌ | Non disponible en WinPE |

---

### 4.2 ISO 2 : NiTriTe_Direct_WinPE.iso — Boot Direct

**Base** : WinPE 11 minimaliste, sans bureau, NiTriTe en plein écran.

Cet ISO est conçu pour un **démarrage ultra-rapide** quand l'utilisateur veut juste NiTriTe sans le bureau Windows complet. C'est le mode "outil de récupération d'urgence".

#### Contenu de l'ISO

```
NiTriTe_Direct_WinPE.iso
├── boot/
│   ├── BCD
│   ├── boot.sdi
│   └── bootfix.bin
├── EFI/
│   └── Boot/
│       └── bootx64.efi
├── sources/
│   └── boot.wim                    ← Image WinPE allégée (~280 MB)
│       │
│       │  ── Contenu du WIM (monté) ──
│       │
│       ├── Windows/
│       │   └── System32/
│       │       ├── startnet.cmd     ← wpeinit + lance NiTriTe directement
│       │       ├── winpeshl.ini     ← AppPath = X:\Nitrite\nitrite.exe
│       │       └── config/          ← Registre minimal
│       │
│       ├── Nitrite/
│       │   ├── nitrite.exe
│       │   ├── WebView2/           ← Runtime WebView2
│       │   └── ... (assets essentiels uniquement)
│       │
│       └── Users/Default/AppData/  ← Minimum pour WebView2
│
└── bootmgr / bootmgr.efi
```

#### Comportement au boot

1. WinPE démarre (pas de bureau, pas d'explorer.exe)
2. `startnet.cmd` configure env vars + lance NiTriTe
3. NiTriTe apparaît en **plein écran** (seule interface visible)
4. L'utilisateur interagit **exclusivement via NiTriTe**
5. Fermer NiTriTe → retour à un prompt cmd (avec menu : relancer NiTriTe / reboot / shutdown)

#### Différences avec l'ISO Win11 PE

| Aspect | Win11 PE (ISO 1) | NiTriTe Direct (ISO 2) |
|--------|-----------------|----------------------|
| Taille ISO | ~800 MB - 1.2 GB | ~400 - 600 MB |
| RAM utilisée | ~1 - 1.5 GB | ~400 - 600 MB |
| Temps de boot | ~45-60 sec | ~20-30 sec |
| Bureau Windows | ✅ Explorer.exe | ❌ Pas de bureau |
| Multi-fenêtres | ✅ | ❌ NiTriTe uniquement |
| Outils portables | ✅ Accessibles | ❌ Non intégrés |
| Cas d'usage | Intervention complète | Urgence / diagnostic rapide |

---

## 5. Configuration Ventoy détaillée

### 5.1 Installation Ventoy sur la clé USB

**Prérequis** : Clé USB ≥ 16 GB (recommandé 32 GB pour espace supplémentaire)

```
Étapes :
1. Télécharger Ventoy depuis https://ventoy.net
2. Lancer Ventoy2Disk.exe
3. Sélectionner la clé USB
4. Cocher "Secure Boot Support" (optionnel mais recommandé)
5. Schéma de partition : GPT (pour UEFI moderne)
6. Cliquer "Install"
7. La clé est partitionnée en 2 :
   - Partition 1 (VTOYEFI) : ~32 MB — Bootloader Ventoy (ne pas toucher)
   - Partition 2 (Ventoy)  : Reste — exFAT, c'est ici qu'on copie tout
```

### 5.2 Fichier `ventoy/ventoy.json`

```json
{
    "control": [
        { "VTOY_DEFAULT_SEARCH_ROOT": "/ISO" },
        { "VTOY_MENU_TIMEOUT": "0" },
        { "VTOY_DEFAULT_IMAGE": "/ISO/NitritePE_Win11.iso" },
        { "VTOY_FILT_DOT_UNDERSCORE_FILE": "1" },
        { "VTOY_SORT_CASE_SENSITIVE": "0" }
    ],

    "theme": {
        "file": "/ventoy/theme/theme.txt",
        "gfxmode": "1920x1080",
        "display_mode": "GUI",
        "serial_param": "--unit=0 --speed=9600",
        "ventoy_left": "5%",
        "ventoy_top": "50%",
        "ventoy_color": "#FFFFFF"
    },

    "menu_alias": [
        {
            "image": "/ISO/NitritePE_Win11.iso",
            "alias": "🖥️  Mini Windows 11 — NiTriTe Desktop (Environnement complet)"
        },
        {
            "image": "/ISO/NiTriTe_Direct_WinPE.iso",
            "alias": "⚡  NiTriTe Direct — Boot Rapide (Diagnostic & Récupération)"
        }
    ],

    "menu_class": [
        {
            "key": "/ISO/NitritePE_Win11.iso",
            "class": "windows"
        },
        {
            "key": "/ISO/NiTriTe_Direct_WinPE.iso",
            "class": "nitrite"
        }
    ],

    "image_list": [
        "/ISO/NitritePE_Win11.iso",
        "/ISO/NiTriTe_Direct_WinPE.iso"
    ],

    "image_list_mode": 1,

    "auto_memdisk": [
        {
            "image": "/ISO/NiTriTe_Direct_WinPE.iso",
            "mode": "0"
        }
    ],

    "persistence": []
}
```

### 5.3 Thème GRUB2 personnalisé — `ventoy/theme/theme.txt`

```
# NiTriTe Boot Menu Theme
# Inspiré du style NiTriTe (sombre, accent violet/cyan)

title-text: ""
desktop-image: "background.png"
desktop-color: "#0a0a1a"

terminal-font: "Terminus Regular 16"
terminal-box: "terminal_box_*.png"

message-font: "DejaVu Sans Regular 14"
message-color: "#FFFFFF"
message-bg-color: "#1a1a2e"

# Titre du menu
+ label {
    left = 50%-200
    top = 15%
    width = 400
    height = 60
    text = "NiTriTe Boot Manager"
    font = "DejaVu Sans Bold 24"
    color = "#00d4ff"
    align = "center"
}

# Sous-titre
+ label {
    left = 50%-200
    top = 22%
    width = 400
    height = 30
    text = "Sélectionnez une option de démarrage"
    font = "DejaVu Sans Regular 14"
    color = "#808090"
    align = "center"
}

# Zone du menu
+ boot_menu {
    left = 15%
    top = 35%
    width = 70%
    height = 40%
    item_font = "DejaVu Sans Regular 18"
    item_color = "#ccccdd"
    selected_item_font = "DejaVu Sans Bold 18"
    selected_item_color = "#00d4ff"
    item_height = 50
    item_padding = 15
    item_spacing = 10
    item_icon_space = 20
    icon_width = 32
    icon_height = 32
    selected_item_pixmap_style = "select_*.png"
    menu_pixmap_style = "menu_*.png"
    scrollbar = true
    scrollbar_width = 10
    scrollbar_thumb = "scrollbar_thumb.png"
}

# Barre d'aide en bas
+ label {
    left = 50%-300
    top = 85%
    width = 600
    height = 30
    text = "Entrée = Démarrer  |  F5 = Thèmes  |  Ctrl+W = WIMBOOT"
    font = "DejaVu Sans Regular 12"
    color = "#505060"
    align = "center"
}

# Pied de page
+ label {
    left = 50%-200
    top = 92%
    width = 400
    height = 25
    text = "NiTriTe v27.0 — Boîte à outils de récupération"
    font = "DejaVu Sans Regular 11"
    color = "#303040"
    align = "center"
}
```

### 5.4 Fichier `ventoy/ventoy_grub.cfg` (menu personnalisé avancé F6)

```grub
# Menu personnalisé accessible via F6
# Contient des options avancées

menuentry "🔧 Reboot" --class reboot {
    reboot
}

menuentry "⏻ Éteindre" --class shutdown {
    halt
}

menuentry "🔄 Boot depuis disque dur local" --class hd {
    set root=(hd1)
    chainloader +1
}
```

---

## 6. Processus de construction des ISOs

### 6.1 Script principal : `build-ventoy-usb.bat`

Ce script remplace/complète le `build-bootable.bat` existant. Il génère les deux ISOs et prépare la structure Ventoy.

```
WORKFLOW DU SCRIPT :

╔════════════════════════════════════════════════════════════╗
║                  build-ventoy-usb.bat                      ║
╠════════════════════════════════════════════════════════════╣
║                                                            ║
║  1. VÉRIFICATION PRÉREQUIS                                ║
║     ├── Windows ADK + WinPE Add-on installés ?            ║
║     ├── NiTriTe.exe compilé ?                             ║
║     ├── WebView2 Fixed Runtime disponible ?               ║
║     ├── NitritePE (Win11 PE) disponible ?                 ║
║     └── oscdimg.exe accessible ?                          ║
║                                                            ║
║  2. CONSTRUCTION ISO 1 : NitritePE_Win11.iso              ║
║     ├── Copier le contenu NitritePE/media/                ║
║     ├── Monter boot.wim avec DISM                         ║
║     ├── Injecter drivers réseau/stockage via DISM         ║
║     ├── Ajouter packages WinPE (WMI, NetFX, PowerShell)  ║
║     ├── Copier explorer.exe + DLLs shell Win11           ║
║     ├── Copier NiTriTe + WebView2 + assets               ║
║     ├── Copier outils portables (Autoruns, etc.)          ║
║     ├── Créer raccourcis bureau (.lnk)                    ║
║     ├── Configurer registre (shell, env vars, autorun)    ║
║     ├── Installer startnet.cmd (mode bureau)              ║
║     ├── Démonter WIM (commit)                             ║
║     └── Générer ISO avec oscdimg (UEFI + BIOS)           ║
║                                                            ║
║  3. CONSTRUCTION ISO 2 : NiTriTe_Direct_WinPE.iso         ║
║     ├── Créer WinPE depuis ADK (copype amd64)            ║
║     ├── Monter boot.wim                                   ║
║     ├── Injecter drivers essentiels                       ║
║     ├── Copier NiTriTe + WebView2 (sans extras)          ║
║     ├── Configurer winpeshl.ini → NiTriTe direct         ║
║     ├── Installer startnet.cmd (mode direct)              ║
║     ├── Démonter WIM (commit)                             ║
║     └── Générer ISO avec oscdimg                          ║
║                                                            ║
║  4. PRÉPARATION STRUCTURE VENTOY                          ║
║     ├── Créer arborescence /ventoy/, /ISO/, etc.          ║
║     ├── Copier les 2 ISOs dans /ISO/                      ║
║     ├── Générer ventoy.json                               ║
║     ├── Installer le thème GRUB2                          ║
║     ├── Copier ventoy_grub.cfg                            ║
║     └── Copier ventoy_wimboot.img                         ║
║                                                            ║
║  5. (OPTIONNEL) COPIE DIRECTE SUR CLÉ USB                ║
║     ├── Détecter la clé USB Ventoy                        ║
║     ├── Copier la structure complète                      ║
║     └── Vérifier l'intégrité                              ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
```

### 6.2 Scripts de configuration WinPE

#### `startnet_desktop.cmd` — Pour ISO 1 (Mini Windows 11 PE)

```batch
@echo off
:: startnet.cmd — Mode Bureau Windows 11 PE + NiTriTe
:: Ce script est exécuté automatiquement par WinPE au démarrage

:: Initialisation réseau et matériel
wpeinit

:: Environnement utilisateur
set USERPROFILE=X:\Users\Default
set APPDATA=X:\Users\Default\AppData\Roaming
set LOCALAPPDATA=X:\Users\Default\AppData\Local
set TEMP=X:\Temp
set TMP=X:\Temp

:: Création des répertoires nécessaires
mkdir X:\Users\Default\AppData\Local 2>nul
mkdir X:\Users\Default\AppData\Roaming 2>nul
mkdir X:\Users\Default\Desktop 2>nul
mkdir X:\Temp 2>nul

:: WebView2 Fixed Runtime
if exist X:\Nitrite\WebView2 (
    set WEBVIEW2_BROWSER_EXECUTABLE_FOLDER=X:\Nitrite\WebView2
)

:: Résolution d'écran
wpeutil SetBootScreenResolution 1920 1080

:: Désactiver le timeout 72h WinPE
reg add "HKLM\SYSTEM\CurrentControlSet\Control\WMI\Autologger\WdiContextLog" ^
    /v Start /t REG_DWORD /d 0 /f >nul 2>&1

:: Monter les partitions USB pour accéder aux outils portables de la clé
:: Ventoy expose la partition données comme un volume séparé
for %%d in (C D E F G H I J K L M N O P Q R S T U V W Y Z) do (
    if exist "%%d:\ventoy\ventoy.json" (
        set USB_DRIVE=%%d:
        echo [NiTriTe] Clé USB Ventoy détectée sur !USB_DRIVE!
    )
)

:: Attendre l'init matérielle
ping -n 5 127.0.0.1 >nul

:: NiTriTe sera lancé au démarrage via la clé registre Run
:: Le bureau (explorer.exe) est configuré en shell via winpeshl.ini
```

#### `startnet_direct.cmd` — Pour ISO 2 (NiTriTe Direct)

```batch
@echo off
:: startnet.cmd — Mode NiTriTe Direct (sans bureau)

wpeinit

set USERPROFILE=X:\Users\Default
set APPDATA=X:\Users\Default\AppData\Roaming
set LOCALAPPDATA=X:\Users\Default\AppData\Local
set TEMP=X:\Temp
set TMP=X:\Temp

mkdir X:\Users\Default\AppData\Local 2>nul
mkdir X:\Users\Default\AppData\Roaming 2>nul
mkdir X:\Temp 2>nul

if exist X:\Nitrite\WebView2 (
    set WEBVIEW2_BROWSER_EXECUTABLE_FOLDER=X:\Nitrite\WebView2
)

:: Désactiver timeout WinPE
reg add "HKLM\SYSTEM\CurrentControlSet\Control\WMI\Autologger\WdiContextLog" ^
    /v Start /t REG_DWORD /d 0 /f >nul 2>&1

:: Attendre l'init
ping -n 8 127.0.0.1 >nul

:: Lancer NiTriTe directement
:launch
if exist X:\Nitrite\nitrite.exe (
    echo.
    echo ============================================
    echo   NiTriTe — Démarrage en mode Direct...
    echo ============================================
    echo.
    start /wait "" X:\Nitrite\nitrite.exe
)

:: Si NiTriTe se ferme, proposer un menu
echo.
echo ============================================
echo   NiTriTe s'est fermé.
echo ============================================
echo.
echo   1. Relancer NiTriTe
echo   2. Ouvrir un invite de commandes
echo   3. Redémarrer le PC
echo   4. Éteindre le PC
echo.
set /p CHOICE="Votre choix : "
if "%CHOICE%"=="1" goto launch
if "%CHOICE%"=="2" cmd /k
if "%CHOICE%"=="3" wpeutil reboot
if "%CHOICE%"=="4" wpeutil shutdown
goto launch
```

#### `winpeshl_desktop.ini` — Pour ISO 1

```ini
[LaunchApps]
%WINDIR%\System32\wpeinit.exe
%WINDIR%\explorer.exe
```

#### `winpeshl_direct.ini` — Pour ISO 2

```ini
[LaunchApp]
AppPath = %WINDIR%\System32\startnet.cmd
```

---

## 7. Drivers à injecter

### 7.1 Pack drivers commun (injecté dans les deux ISOs)

| Catégorie | Modèles / Chipsets | Priorité |
|-----------|-------------------|----------|
| **Réseau Ethernet** | Intel I219-V/LM, I225-V, I226-V | P0 |
| | Realtek RTL8111/8168, RTL8125 (2.5GbE) | P0 |
| | Broadcom NetXtreme BCM57xx | P1 |
| | Killer E2xxx, E3xxx | P1 |
| **WiFi** | Intel AX200, AX201, AX210, AX211 | P0 |
| | Intel Wi-Fi 6E AX411 | P1 |
| | Realtek RTL8821CE, RTL8852BE | P1 |
| | MediaTek MT7921/MT7922 | P2 |
| **Stockage NVMe** | Intel 600p/660p/670p RST | P0 |
| | Samsung 970/980/990 NVMe | P0 |
| | WD Black SN770/850 | P1 |
| | SK Hynix P31/P41 | P2 |
| **Stockage SATA/RAID** | Intel RST (Rapid Storage) | P0 |
| | AMD SATA/AHCI | P0 |
| **USB 3.x** | ASMedia ASM1042/1142/2142 | P0 |
| | Renesas uPD720201/202 | P1 |
| **Chipset** | Intel Alder Lake / Raptor Lake | P0 |
| | AMD B550/B650/X570/X670 | P0 |

### 7.2 Source des drivers

1. **Dossier existant** : `Nitrite 2.0/Drivers/` (déjà dans le projet)
2. **Windows ADK drivers** : Inclus dans WinPE par défaut (basiques)
3. **Pack additionnel** : Télécharger depuis les fabricants pour les modèles courants

### 7.3 Méthode d'injection

```batch
:: Injection via DISM dans le boot.wim monté
dism /Image:"C:\mount" /Add-Driver /Driver:".\Drivers\Network" /Recurse /ForceUnsigned
dism /Image:"C:\mount" /Add-Driver /Driver:".\Drivers\Storage" /Recurse /ForceUnsigned
dism /Image:"C:\mount" /Add-Driver /Driver:".\Drivers\Chipset" /Recurse /ForceUnsigned
dism /Image:"C:\mount" /Add-Driver /Driver:".\Drivers\USB" /Recurse /ForceUnsigned
```

---

## 8. Outils portables intégrés (Mini Windows 11 PE)

Comme Hiren's Boot CD PE, le mini Windows 11 inclut des outils portables accessibles depuis le bureau :

### 8.1 Outils à intégrer dans l'ISO Win11 PE

| Catégorie | Outil | Usage | Taille |
|-----------|-------|-------|--------|
| **Diagnostic** | HWiNFO64 | Info matérielle complète | ~8 MB |
| | CrystalDiskInfo | Santé disques SMART | ~5 MB |
| | CPU-Z | Info processeur détaillée | ~3 MB |
| | GPU-Z | Info carte graphique | ~8 MB |
| **Sécurité** | Autoruns | Programmes démarrage | ~2 MB |
| | Process Explorer | Gestionnaire processus avancé | ~3 MB |
| **Fichiers** | 7-Zip File Manager | Compression/extraction | ~2 MB |
| | Notepad++ | Éditeur de texte avancé | ~5 MB |
| | Everything | Recherche fichiers ultra-rapide | ~2 MB |
| **Disques** | MiniTool Partition Wizard | Gestion partitions | ~30 MB |
| | Rufus | Création clé USB bootable | ~1 MB |
| | HDDScan | Test surface disque | ~4 MB |
| **Réseau** | WinSCP | Transfert fichiers SFTP/SCP | ~10 MB |
| | PuTTY | Client SSH/Telnet | ~1 MB |
| **Récupération** | TestDisk/PhotoRec | Récupération données | ~5 MB |
| | Recuva | Récupération fichiers supprimés | ~3 MB |
| **Registry** | Registry Backup | Sauvegarde registre | ~1 MB |
| | Registry Viewer | Lecture registre offline | ~2 MB |
| **Total estimé** | | | **~95 MB** |

### 8.2 Organisation sur le bureau Win11 PE

```
Bureau (X:\Users\Default\Desktop\)
├── NiTriTe.lnk                    ← Lance NiTriTe
├── 📁 Diagnostic/
│   ├── HWiNFO64.lnk
│   ├── CrystalDiskInfo.lnk
│   ├── CPU-Z.lnk
│   └── GPU-Z.lnk
├── 📁 Sécurité/
│   ├── Autoruns.lnk
│   └── ProcessExplorer.lnk
├── 📁 Disques/
│   ├── MiniTool Partition.lnk
│   ├── Rufus.lnk
│   └── TestDisk.lnk
├── 📁 Réseau/
│   ├── WinSCP.lnk
│   └── PuTTY.lnk
└── 📁 Utilitaires/
    ├── 7-Zip.lnk
    ├── Notepad++.lnk
    └── Everything.lnk
```

---

## 9. Module Rust : Détection du mode et adaptation

### 9.1 Détection WinPE + Ventoy dans NiTriTe

```rust
// src-tauri/src/system/winpe.rs

/// Détecte si NiTriTe tourne en mode WinPE
pub fn is_winpe() -> bool {
    std::env::var("WINPE").is_ok()
    || std::path::Path::new("X:\\Windows\\System32\\winpe.ini").exists()
    || std::path::Path::new("X:\\Windows\\System32\\startnet.cmd").exists()
}

/// Détecte si la clé USB Ventoy est accessible
pub fn find_ventoy_drive() -> Option<String> {
    for letter in b'C'..=b'Z' {
        let drive = format!("{}:", letter as char);
        let ventoy_json = format!("{}\\ventoy\\ventoy.json", drive);
        if std::path::Path::new(&ventoy_json).exists() {
            return Some(drive);
        }
    }
    None
}

/// Détecte le mode de boot (Desktop vs Direct)
pub enum BootMode {
    Desktop,    // Win11 PE avec bureau
    Direct,     // NiTriTe seul
    Normal,     // Installation Windows standard
}

pub fn detect_boot_mode() -> BootMode {
    if !is_winpe() {
        return BootMode::Normal;
    }
    // En mode Desktop, explorer.exe tourne
    if is_process_running("explorer.exe") {
        BootMode::Desktop
    } else {
        BootMode::Direct
    }
}
```

### 9.2 Adaptations UI selon le mode

| Élément UI | Mode Normal | Mode Desktop (Win11 PE) | Mode Direct (WinPE) |
|-----------|-------------|------------------------|---------------------|
| Barre de titre | Standard | Badge "WinPE Desktop" | Badge "Mode Récupération" |
| Onglets Services | ✅ Visible | ❌ Masqué | ❌ Masqué |
| Onglets Mises à jour | ✅ Visible | ❌ Masqué | ❌ Masqué |
| Onglet WinPE Tools | ❌ Masqué | ✅ Visible | ✅ Visible |
| Bouton Reboot/Shutdown | ❌ Masqué | ✅ Visible | ✅ Visible |
| Sélecteur disque cible | ❌ Masqué | ✅ Visible | ✅ Visible |
| Thème | Défaut utilisateur | Thème sombre WinPE | Thème sombre compact |
| Fullscreen | Non | Non (fenêtre) | Oui (par défaut) |

---

## 10. Phases de développement

### Phase 1 — Infrastructure Ventoy & ISOs (v27.0)
**Durée estimée** : 2-3 semaines

- [ ] Créer le script `build-ventoy-usb.bat` (génération des 2 ISOs)
- [ ] Construire l'ISO 1 : NitritePE_Win11.iso à partir du NitritePE existant
  - [ ] Monter et customiser le boot.wim existant
  - [ ] Injecter NiTriTe + WebView2 Fixed Runtime
  - [ ] Copier les DLLs manquantes (VC++, CRT, shell)
  - [ ] Configurer explorer.exe comme shell
  - [ ] Configurer l'autorun de NiTriTe via registre
- [ ] Construire l'ISO 2 : NiTriTe_Direct_WinPE.iso
  - [ ] Créer WinPE minimal via ADK
  - [ ] Intégrer NiTriTe uniquement (sans bureau)
  - [ ] Configurer winpeshl.ini → NiTriTe direct
  - [ ] Menu de secours en cas de fermeture NiTriTe
- [ ] Configurer la structure Ventoy
  - [ ] Créer ventoy.json (menu, alias, ordre)
  - [ ] Tester le boot via Ventoy sur VM

### Phase 2 — Thème & UX Boot (v27.1)
**Durée estimée** : 1-2 semaines

- [ ] Créer le thème GRUB2 NiTriTe pour Ventoy
  - [ ] Design du fond d'écran (style NiTriTe : sombre, accents cyan)
  - [ ] Icônes personnalisées pour chaque entrée de menu
  - [ ] Curseur de sélection animé
  - [ ] Police personnalisée
- [ ] Configurer le menu GRUB2 avancé (F6)
- [ ] Ajouter le splash screen WinPE personnalisé

### Phase 3 — Outils portables & Drivers (v27.2)
**Durée estimée** : 1-2 semaines

- [ ] Collecter et intégrer les outils portables dans l'ISO Win11 PE
  - [ ] HWiNFO64, CrystalDiskInfo, CPU-Z, GPU-Z
  - [ ] Autoruns, Process Explorer
  - [ ] 7-Zip, Notepad++, Everything
  - [ ] MiniTool Partition, Rufus, TestDisk
  - [ ] WinSCP, PuTTY
- [ ] Créer les raccourcis bureau organisés par catégorie
- [ ] Constituer le pack de drivers complet
  - [ ] Réseau Ethernet (Intel, Realtek)
  - [ ] WiFi (Intel AX, Realtek)
  - [ ] NVMe/SATA
  - [ ] USB 3.x
- [ ] Injecter drivers dans les deux ISOs

### Phase 4 — Module Rust WinPE + Adaptations UI (v27.3)
**Durée estimée** : 2-3 semaines

- [ ] Implémenter `system/winpe.rs` dans le backend Tauri
  - [ ] Détection WinPE (`is_winpe()`)
  - [ ] Détection Ventoy (`find_ventoy_drive()`)
  - [ ] Détection mode boot (Desktop vs Direct)
- [ ] Adapter l'UI NiTriTe (Vue 3)
  - [ ] Badge mode WinPE dans la status bar
  - [ ] Masquer les onglets incompatibles
  - [ ] Page WinPE Tools (boot repair, registry offline, etc.)
  - [ ] Mode plein écran automatique en mode Direct
- [ ] Fonctionnalités WinPE exclusives
  - [ ] Sélecteur de disque cible
  - [ ] Réparation MBR/GPT (bootrec)
  - [ ] Registre offline (montage ruche SAM/SYSTEM)
  - [ ] System Repair Wizard guidé

### Phase 5 — Tests & Validation (v27.4)
**Durée estimée** : 1-2 semaines

- [ ] Tests sur machines virtuelles
  - [ ] VirtualBox (UEFI + BIOS legacy)
  - [ ] VMware Workstation
  - [ ] Hyper-V
- [ ] Tests sur matériel réel
  - [ ] PC de bureau récent (Intel 12e/13e gen)
  - [ ] PC de bureau AMD (Ryzen 5000/7000)
  - [ ] Laptop récent (UEFI Secure Boot)
  - [ ] Laptop ancien (BIOS legacy)
  - [ ] PC avec NVMe
  - [ ] PC avec SATA uniquement
- [ ] Validation fonctionnelle
  - [ ] Boot Ventoy → menu OK
  - [ ] ISO 1 → bureau Win11 PE + NiTriTe OK
  - [ ] ISO 2 → NiTriTe direct OK
  - [ ] Réseau Ethernet fonctionnel
  - [ ] Outils portables accessibles
  - [ ] NiTriTe diagnostic matériel OK
  - [ ] Boot repair fonctionne
- [ ] Tests de performance
  - [ ] Temps de boot < 60s (ISO 1) / < 30s (ISO 2)
  - [ ] RAM < 1.5 GB (ISO 1) / < 600 MB (ISO 2)
  - [ ] Taille totale clé USB < 3 GB

---

## 11. Prérequis système de build

| Outil | Version | Usage | Installation |
|-------|---------|-------|-------------|
| Windows 10/11 64-bit | N/A | OS de build | — |
| Windows ADK | 11 (22H2+) | Outils WinPE | `winget install Microsoft.WindowsADK` |
| WinPE add-on for ADK | 11 | Contenu WinPE | `winget install Microsoft.ADKPEAddOn` |
| Ventoy | 1.0.99+ | Préparation clé USB | https://ventoy.net |
| Droits Administrateur | N/A | DISM, montage image | — |
| Espace disque libre | > 15 GB | Montage + 2 ISOs + workspace | — |
| NiTriTe compilé | v26.44.0+ | Exécutable final | `build.bat` |
| WebView2 Fixed Runtime | v120+ | Runtime pour WinPE | https://developer.microsoft.com/microsoft-edge/webview2/ |
| NitritePE (Win11 PE) | — | Base de l'ISO 1 | Déjà dans le projet (`Nitrite bootable/NitritePE/`) |

---

## 12. Structure des nouveaux fichiers dans le projet

```
Nitrite 2.0/
├── Nitrite bootable/
│   ├── NitritePE/                          ← EXISTANT — Windows 11 PE
│   ├── Applications/                       ← EXISTANT — Apps portables
│   ├── build-ventoy-usb.bat                ← NOUVEAU — Script principal de construction
│   ├── config/                             ← NOUVEAU — Fichiers de configuration
│   │   ├── ventoy.json                     ← Config Ventoy à copier sur la clé
│   │   ├── ventoy_grub.cfg                 ← Menu GRUB2 personnalisé
│   │   ├── startnet_desktop.cmd            ← startnet pour ISO Win11 PE
│   │   ├── startnet_direct.cmd             ← startnet pour ISO Direct
│   │   ├── winpeshl_desktop.ini            ← Config shell bureau
│   │   ├── winpeshl_direct.ini             ← Config shell direct
│   │   └── boot_menu.bat                   ← Menu de secours post-NiTriTe
│   ├── theme/                              ← NOUVEAU — Thème GRUB2 Ventoy
│   │   ├── theme.txt
│   │   ├── background.png
│   │   ├── font.pf2
│   │   └── icons/
│   │       ├── nitrite.png
│   │       └── windows.png
│   ├── tools/                              ← NOUVEAU — Outils portables à intégrer
│   │   ├── HWiNFO64/
│   │   ├── CrystalDiskInfo/
│   │   ├── Autoruns/
│   │   ├── ProcessExplorer/
│   │   ├── 7zip/
│   │   ├── NotepadPP/
│   │   └── ...
│   └── output/                             ← NOUVEAU — ISOs générés
│       ├── NitritePE_Win11.iso
│       └── NiTriTe_Direct_WinPE.iso
│
├── src-tauri/src/
│   └── system/
│       └── winpe.rs                        ← NOUVEAU — Module détection WinPE
│
└── src/
    └── pages/
        └── WinPEModePage.vue               ← NOUVEAU — Page outils WinPE
```

---

## 13. Guide utilisateur : Préparation de la clé USB

### Étape 1 : Préparer la clé USB avec Ventoy
```
1. Télécharger Ventoy depuis https://ventoy.net
2. Brancher une clé USB ≥ 16 GB
3. Lancer Ventoy2Disk.exe en Administrateur
4. Sélectionner la clé USB
5. Option → Schéma de partition : GPT
6. Option → Secure Boot Support : coché
7. Cliquer "Install"
8. Accepter le formatage
```

### Étape 2 : Construire les ISOs
```
1. Ouvrir un terminal Administrateur
2. cd "C:\Users\Momo\Desktop\Nitrite 2.0\Nitrite bootable"
3. build-ventoy-usb.bat
4. Attendre la fin (~10-20 minutes)
5. Les ISOs sont dans output/
```

### Étape 3 : Copier sur la clé USB
```
Structure à copier sur la partition Ventoy de la clé :

[Clé USB]/
├── ventoy/
│   ├── ventoy.json
│   ├── ventoy_grub.cfg
│   ├── ventoy_wimboot.img    ← Télécharger depuis Ventoy GitHub
│   └── theme/                ← Dossier thème complet
├── ISO/
│   ├── NitritePE_Win11.iso
│   └── NiTriTe_Direct_WinPE.iso
├── Applications/             ← Optionnel, outils accessibles depuis Win11 PE
└── Drivers/                  ← Optionnel, drivers supplémentaires
```

### Étape 4 : Booter
```
1. Insérer la clé USB dans le PC cible
2. Accéder au menu de boot (F2/F12/DEL/ESC selon la marque)
3. Sélectionner la clé USB
4. Le menu Ventoy NiTriTe apparaît
5. Choisir :
   - "Mini Windows 11" pour un environnement complet
   - "NiTriTe Direct" pour un diagnostic rapide
```

---

## 14. Contraintes & Risques

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| WebView2 incompatible WinPE | Moyen | Critique | Tester Fixed Runtime v120+ ; le build-bootable existant fonctionne déjà |
| Ventoy ne boot pas sur certains PC | Faible | Élevé | Ventoy supporte 99% des machines ; mode Secure Boot activé |
| Taille totale > 4 GB (limite FAT32) | Moyen | Élevé | Utiliser exFAT pour la partition Ventoy (par défaut) ; compression LZMA des WIM |
| Drivers réseau manquants | Élevé | Moyen | Pack de 50+ drivers communs ; documentation pour ajout custom |
| Explorer.exe crash dans Win11 PE | Moyen | Élevé | DLLs shell testées ; fallback sur cmd si crash |
| Performances lentes sur vieux PC | Moyen | Moyen | ISO Direct (léger) disponible pour les machines faibles |
| Secure Boot bloque Ventoy | Faible | Élevé | Ventoy supporte Secure Boot nativement depuis v1.0.74 |
| NitritePE boot.wim corrompu | Faible | Critique | Vérification intégrité SHA256 dans le script de build |
| Mise à jour Ventoy casse le boot | Faible | Moyen | Épingler la version Ventoy ; tester avant mise à jour |

---

## 15. Métriques de succès

| Métrique | Cible ISO Win11 PE | Cible ISO Direct |
|----------|-------------------|-----------------|
| Temps boot → UI utilisable | < 60 secondes | < 30 secondes |
| Taille ISO | < 1.5 GB | < 800 MB |
| Taille totale clé USB | < 3 GB | — |
| RAM utilisée | < 1.5 GB | < 512 MB |
| Compatibilité UEFI | ≥ 95% PC depuis 2015 | ≥ 95% |
| Compatibilité réseau Ethernet | ≥ 85% cartes communes | ≥ 85% |
| Fonctionnalités NiTriTe dispo | ≥ 70% | ≥ 60% |
| Outils portables accessibles | ≥ 15 outils | N/A |

---

## 16. Hors scope (v27.x)

- Boot sur macOS ou Linux
- Support ARM (Surface Pro X, Qualcomm)
- Chiffrement du contenu de la clé USB
- Persistance des données entre reboots (sauf sur la partition Ventoy)
- Agent IA (llama.cpp) en mode WinPE
- Interface multilingue (FR uniquement dans cette version)
- Création automatique de la clé USB depuis NiTriTe (l'utilisateur utilise Ventoy2Disk)
- Support des ISOs tierces dans le menu Ventoy (l'utilisateur peut les ajouter manuellement)

---

## 17. Décisions d'architecture à valider

| # | Question | Recommandation | Statut |
|---|----------|----------------|--------|
| D1 | WinPE 10 vs WinPE 11 pour l'ISO Direct ? | WinPE 11 (cohérence avec l'ISO Win11 PE) | ⏳ |
| D2 | Ventoy partition exFAT ou NTFS ? | exFAT (compatibilité maximale, pas de limite 4 GB) | ⏳ |
| D3 | Taille minimum clé USB ? | 16 GB minimum, 32 GB recommandé | ⏳ |
| D4 | Inclure Ventoy2Disk dans le projet ? | Non, lien de téléchargement dans la doc | ⏳ |
| D5 | Ventoy Secure Boot activé par défaut ? | Oui, avec MOK enrollment documenté | ⏳ |
| D6 | Pack drivers : embarqué dans les ISOs ou sur la partition Ventoy ? | Embarqué dans les ISOs (pas de dépendance au montage USB) | ⏳ |
| D7 | Outils portables : dans l'ISO ou sur la partition Ventoy ? | Dans l'ISO Win11 PE (accès garanti) + copies sur partition Ventoy (bonus) | ⏳ |
| D8 | Script de build : interactif ou automatique ? | Semi-automatique avec options en ligne de commande | ⏳ |

---

## 18. Glossaire

| Terme | Définition |
|-------|-----------|
| **WinPE** | Windows Preinstallation Environment — version allégée de Windows bootable depuis USB |
| **Ventoy** | Bootloader open-source permettant de démarrer des ISOs/WIM/VHD directement depuis USB |
| **GRUB2** | Grand Unified Bootloader 2 — bootloader utilisé par Ventoy pour afficher le menu |
| **WIM** | Windows Imaging Format — format d'image utilisé par WinPE (boot.wim) |
| **DISM** | Deployment Image Servicing and Management — outil Microsoft pour modifier les images WIM |
| **ADK** | Assessment and Deployment Kit — outils Microsoft pour créer des images WinPE |
| **NitritePE** | Le Windows 11 PE pré-construit existant dans le projet |
| **ISO Direct** | L'ISO minimaliste qui lance NiTriTe sans bureau Windows |
| **Fixed Runtime** | Version de WebView2 embarquée localement (ne nécessite pas d'installation) |
| **oscdimg** | Outil Microsoft pour créer des images ISO bootables (UEFI + BIOS) |
| **Hiren's Boot CD PE** | Outil de récupération populaire basé sur WinPE, référence d'inspiration |
