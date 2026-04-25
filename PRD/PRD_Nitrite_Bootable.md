# PRD — NiTriTe Bootable (WinPE Edition)
**Version produit** : 27.0.0
**Basé sur** : NiTriTe v26.56.0
**Statut** : Draft — 2026-03-23
**Auteur** : Momo

---

## 1. Vision & Objectif

NiTriTe doit pouvoir **démarrer directement depuis une clé USB** sur n'importe quel PC Windows 10 / 11, **sans système d'exploitation installé** ou sur un système inaccessible (BSOD, corruption, ransomware, boot impossible).

L'outil devient ainsi la **boîte à outils de récupération ultime** : diagnostic matériel, récupération de données, réinstallation, nettoyage — le tout depuis un environnement sûr et contrôlé, indépendant du disque système.

---

## 2. Cas d'usage cibles

| # | Scénario | Priorité |
|---|----------|----------|
| U1 | PC qui ne démarre plus (BSOD, boot loop) — diagnostic + réparation | P0 |
| U2 | Récupération de données sur disque endommagé | P0 |
| U3 | PC potentiellement infecté (ransomware, rootkit) — analyse hors-ligne | P0 |
| U4 | Réinstallation propre Windows depuis NiTriTe (ISO + Rufus) | P1 |
| U5 | Diagnostic matériel complet (RAM, disques SMART, températures) | P1 |
| U6 | Clonage disque avant remplacement | P1 |
| U7 | Technicien itinérant — outil unique pour toutes interventions | P2 |

---

## 3. Environnement cible : Windows PE

### 3.1 Qu'est-ce que WinPE ?

**Windows Preinstallation Environment (WinPE)** est une version allégée de Windows, bootstrappable depuis USB, utilisée par Microsoft pour l'installation et la récupération du système. C'est l'environnement standard pour ce type d'outil.

- Noyau Windows complet (NT kernel)
- API Win32 disponibles → NiTriTe.exe est compatible nativement
- Support réseau (drivers injectables)
- Durée de session illimitée (avec patch registre)
- Taille ISO finale cible : **~1.5 GB**

### 3.2 Alternatives écartées

| Alternative | Raison du rejet |
|-------------|-----------------|
| Linux Live (Ubuntu, SystemRescue) | WebView2 / WinAPI non disponibles → NiTriTe non fonctionnel |
| WinRE (Recovery Environment natif) | Trop limité, non personnalisable librement |
| DOS / UEFI shell | Pas d'interface graphique possible |
| VM embarquée | Trop lourd, complexité inutile |

---

## 4. Architecture technique

```
┌─────────────────────────────────────────────────────────┐
│                     Clé USB bootable                     │
│                                                          │
│  /boot/                  ← Bootloader EFI + MBR         │
│  /sources/boot.wim       ← Image WinPE customisée       │
│  /NiTriTe/               ← Application + assets         │
│    ├── NiTriTe.exe                                       │
│    ├── WebView2/         ← Runtime WebView2 embarqué    │
│    ├── logiciel/         ← Apps portables               │
│    ├── Drivers/          ← Drivers injectables           │
│    └── Script Windows/   ← Scripts de réparation        │
│  /WinPE/                                                 │
│    ├── startnet.cmd      ← Lance NiTriTe au démarrage   │
│    └── winpeshl.ini      ← Config shell WinPE           │
└─────────────────────────────────────────────────────────┘
```

### 4.1 Couches logicielles

```
┌─────────────────────────────────┐
│   NiTriTe GUI (Vue 3 + Tauri)   │  ← inchangé
├─────────────────────────────────┤
│   WebView2 Runtime (embedded)   │  ← packagé localement
├─────────────────────────────────┤
│   Windows PE 64-bit             │  ← environnement host
├─────────────────────────────────┤
│   WinPE Drivers (réseau, USB,   │  ← injectés via DISM
│   stockage NVMe/SATA)           │
├─────────────────────────────────┤
│   UEFI / BIOS                   │  ← firmware machine
└─────────────────────────────────┘
```

---

## 5. Fonctionnalités par catégorie en mode WinPE

### 5.1 Disponibles sans modification (✅)

| Fonctionnalité | Notes |
|----------------|-------|
| Diagnostic complet (CPU, RAM, GPU, disques) | WMI disponible en WinPE |
| SMART disques | Accès direct via Win32_DiskDrive |
| Analyse BSOD | Lecture dump `.dmp` du disque cible |
| Hash Checker | Pur filesystem |
| Récupération données | VSS non dispo mais scan direct fichiers |
| Terminal (cmd/powershell) | Disponible en WinPE |
| Scripts Windows | Execution PowerShell disponible |
| Visualiseur disque | Lecture filesystem |
| Gros fichiers / Doublons | Lecture filesystem |
| Portables (Autoruns, CrystalDisk, etc.) | Exécutables standalone |
| Logs système | Lecture fichiers `.evtx` du disque cible |

### 5.2 Partiellement disponibles (⚠️)

| Fonctionnalité | Limitation | Contournement |
|----------------|-----------|---------------|
| Réseau | Drivers à injecter | Package drivers réseau communs (Intel, Realtek) |
| Clonage (wbadmin) | Non dispo en WinPE | Robocopy + xcopy disponibles |
| Températures | ACPI limité | WMI de base dispo |
| WiFi Analyzer | Drivers WiFi à injecter | Injection drivers Realtek/Intel |
| DNS Switcher | Réseau requis | Conditionnel si réseau actif |

### 5.3 Non disponibles en WinPE (❌)

| Fonctionnalité | Raison |
|----------------|--------|
| Services Windows | Services du PC cible, pas WinPE |
| Processus actifs | Processus WinPE seulement |
| Mises à jour Windows | Windows Update non dispo en WinPE |
| Pilotes (gestionnaire) | Gestionnaire de périphériques limité |
| Comptes utilisateurs locaux | SAM du système cible accessible en lecture |
| Restauration système | VSS non disponible en WinPE |
| Agent IA (llama.cpp) | Trop lourd pour WinPE |

### 5.4 Nouvelles fonctionnalités exclusives WinPE (🆕)

| Fonctionnalité | Description |
|----------------|-------------|
| **Boot Drive Selector** | Sélection du disque système cible au démarrage |
| **MBR/GPT Repair** | Réparation du secteur de démarrage via `bootrec` |
| **Registry Offline** | Lecture/écriture registre depuis disque déconnecté |
| **Password Reset** | Réinitialisation mot de passe compte local (SAM offline) |
| **Malware Scanner Offline** | Scan antivirus sans que le malware soit actif |
| **Disk Wipe** | Effacement sécurisé avant vente/recyclage |
| **System Repair Wizard** | Wizard guidé : `sfc /scannow`, `bootrec`, `chkdsk` |
| **PE Info Panel** | Affiche l'état WinPE (IP, drivers chargés, mémoire libre) |

---

## 6. Composants à développer

### 6.1 Outil de création USB (`build-bootable.bat`)

Script Windows qui automatise la création de la clé USB :

```
ÉTAPES DU SCRIPT :
1. Vérifie prérequis (ADK Windows installé, Rufus, droits Admin)
2. Télécharge ADK + WinPE add-on si absent
3. Copymount boot.wim (WinPE 64-bit)
4. Injecte drivers réseau/stockage via DISM
5. Configure startnet.cmd → lance NiTriTe
6. Copie NiTriTe.exe + WebView2 embedded + assets
7. Crée image ISO avec oscdimg
8. Flash USB via Rufus CLI (optionnel)
9. Vérifie intégrité image finale
```

**Dépendances** :
- Windows ADK + WinPE add-on (gratuit, Microsoft)
- `oscdimg.exe` (inclus dans ADK)
- `dism.exe` (inclus dans Windows)
- Rufus CLI (optionnel, pour flash direct USB)

### 6.2 Module Rust : `system/winpe.rs`

Détection du mode WinPE et adaptations runtime :

```rust
// Détection WinPE
pub fn is_winpe() -> bool {
    // WinPE définit toujours WINPE=1 dans l'environnement
    std::env::var("WINPE").is_ok()
    || std::path::Path::new("X:\\Windows\\System32\\winpe.ini").exists()
}

// Sélection du disque système cible
pub struct BootDriveSelector {
    pub available_drives: Vec<DriveInfo>,
    pub selected_drive: Option<String>,  // ex: "C:"
}

// Réparation MBR/GPT
pub fn repair_mbr(drive: &str) -> Result<String, Error>;
pub fn repair_boot_sector() -> Result<String, Error>;

// Registre offline
pub fn mount_offline_registry(drive: &str) -> Result<(), Error>;
pub fn unmount_offline_registry() -> Result<(), Error>;
```

### 6.3 Vue : `WinPEModePage.vue`

Page d'accueil exclusive au mode WinPE :
- Sélecteur de disque cible (dropdown des volumes détectés)
- Statut réseau + IP
- Actions rapides : Repair Boot, Scan Malware, Recover Data
- Wizard de réparation guidé step-by-step

### 6.4 Adaptation `App.vue` — Mode WinPE

- Détection au boot : si `is_winpe()` → afficher badge "WinPE" dans la status bar
- Masquer les onglets non disponibles (Services, Mises à jour, etc.)
- Afficher les onglets exclusifs WinPE
- Fond/thème légèrement différent pour identifier visuellement le mode

### 6.5 WebView2 Embedded

**Problème** : WinPE n'a pas Edge/WebView2 installé.
**Solution** : Packager le **WebView2 Fixed Version Runtime** directement dans NiTriTe.

```
/NiTriTe/
  ├── NiTriTe.exe
  └── WebView2Runtime/          ← ~200MB, version fixe
        ├── msedgewebview2.exe
        └── ...
```

Dans `tauri.conf.json` :
```json
{
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "fixedRuntime",
        "location": "./WebView2Runtime"
      }
    }
  }
}
```

---

## 7. Drivers à injecter dans WinPE

Catégories prioritaires pour la compatibilité maximale :

| Catégorie | Drivers |
|-----------|---------|
| **Réseau filaire** | Intel I219/I225, Realtek RTL8111/8125, Broadcom |
| **Réseau WiFi** | Intel AX200/AX210, Realtek RTL8821CE |
| **Stockage NVMe** | Intel RST, Samsung NVMe, WD Black |
| **USB 3.x** | ASMedia ASM1042/1142, Renesas |
| **Chipsets** | Intel Alder Lake, AMD 500/600 series |
| **GPU (display)** | Résolution 1080p basique sans accélération 3D |

**Source** : Utiliser le pack de drivers déjà présent dans `/Drivers/` du projet.
**Méthode** : `dism /image:C:\mount /Add-Driver /driver:".\Drivers\" /recurse`

---

## 8. Configuration WinPE

### `startnet.cmd` (auto-lancé par WinPE au boot)
```batch
@echo off
wpeinit                          :: Init réseau + matériel
X:\NiTriTe\NiTriTe.exe           :: Lancer NiTriTe
```

### `winpeshl.ini`
```ini
[LaunchApp]
AppPath = X:\NiTriTe\NiTriTe.exe
```

### Patch registre WinPE (timeout session)
```batch
:: Désactiver le timeout 72h WinPE
reg add "HKLM\SYSTEM\CurrentControlSet\Control\WMI\Autologger\WdiContextLog" /v Start /t REG_DWORD /d 0 /f
```

---

## 9. Phases de développement

### Phase 1 — Fondations (v27.0)
**Durée estimée** : 2-3 semaines

- [ ] `build-bootable.bat` — script création USB
- [ ] Intégration WebView2 Fixed Runtime dans le build
- [ ] Module Rust `system/winpe.rs` — détection WinPE + `is_winpe()`
- [ ] Adaptation App.vue — masquage onglets incompatibles en mode WinPE
- [ ] Badge "WinPE Mode" dans AppStatusBar
- [ ] Tests : boot sur VM VirtualBox + vrai hardware

### Phase 2 — Fonctionnalités de récupération (v27.1)
**Durée estimée** : 2-3 semaines

- [ ] `WinPEModePage.vue` — page d'accueil WinPE avec sélecteur disque
- [ ] Boot repair : `bootrec /fixmbr`, `bootrec /fixboot`, `bootrec /rebuildbcd`
- [ ] `chkdsk` intégré avec barre de progression
- [ ] Lecture des logs `.evtx` du système cible (offline)
- [ ] Scan antivirus offline (Windows Defender via CLI)

### Phase 3 — Fonctionnalités avancées (v27.2)
**Durée estimée** : 3-4 semaines

- [ ] Registre offline (montage ruche SAM/SYSTEM/SOFTWARE)
- [ ] Password reset compte local
- [ ] Clonage via Robocopy (remplacement wbadmin)
- [ ] Disk Wipe (effacement sécurisé DoD 5220.22-M)
- [ ] System Repair Wizard step-by-step
- [ ] Injection drivers réseau auto-détectés

### Phase 4 — Polish & Distribution (v27.3)
**Durée estimée** : 1-2 semaines

- [ ] ISO signée (optionnel)
- [ ] Splash screen WinPE personnalisé (fond NiTriTe)
- [ ] Script `flash-usb.bat` (Rufus CLI intégré)
- [ ] Documentation utilisateur bootable
- [ ] Tests de compatibilité hardware (10 machines différentes)

---

## 10. Prérequis système de build

Pour générer l'image bootable, la machine de build doit avoir :

| Outil | Version | Usage |
|-------|---------|-------|
| Windows 10/11 64-bit | N/A | OS de build |
| Windows ADK | 11 (22H2+) | outils WinPE |
| Windows PE add-on for ADK | 11 | contenu WinPE |
| Droits Administrateur | N/A | DISM, montage image |
| Espace disque libre | > 10 GB | montage + build |
| NiTriTe v26.56.0+ | build Tauri | executable final |

**Installation ADK** (automatisable dans `build-bootable.bat`) :
```
winget install Microsoft.WindowsADK
winget install Microsoft.ADKPEAddOn
```

---

## 11. Structure du projet (nouveaux fichiers)

```
Nitrite 2.0/
├── boot/                          ← NOUVEAU
│   ├── build-bootable.bat         ← Script création USB/ISO
│   ├── flash-usb.bat              ← Script flash USB (Rufus)
│   ├── startnet.cmd               ← Script boot WinPE
│   ├── winpeshl.ini               ← Config shell WinPE
│   └── README_BOOT.md             ← Instructions build bootable
│
├── src/
│   └── pages/
│       └── WinPEModePage.vue      ← NOUVEAU — page accueil WinPE
│
└── src-tauri/src/
    └── system/
        └── winpe.rs               ← NOUVEAU — commandes WinPE
```

---

## 12. Contraintes & risques

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|-----------|
| WebView2 incompatible WinPE | Moyen | Critique | Tester Fixed Runtime v120+ ; fallback sur version statique |
| Drivers réseau manquants | Élevé | Moyen | Pack de 50+ drivers les plus communs injectés par défaut |
| Taille ISO > 2 GB | Faible | Mineur | Compression LZMA WIM, split assets facultatifs |
| Secure Boot bloque le boot | Moyen | Élevé | Guide désactivation Secure Boot + option signature |
| Certains PC refusent boot USB | Faible | Moyen | Doc BIOS/UEFI settings par marque |
| Performance lente sur vieux PC | Moyen | Moyen | Limiter animations, réduire mémoire WebView2 |
| Legality SAM password reset | Élevé | Élevé | Feature réservée admin local prouvé ; disclaimer affiché |

---

## 13. Métriques de succès

| Métrique | Cible |
|----------|-------|
| Boot time (WinPE → NiTriTe UI) | < 60 secondes |
| Taille ISO finale | < 1.8 GB |
| Compatibilité hardware | ≥ 90% des PC depuis 2015 |
| Fonctionnalités disponibles en WinPE | ≥ 70% de NiTriTe standard |
| Temps de création clé USB | < 15 minutes (build complet) |
| Drivers réseau auto-détectés | ≥ 80% des cartes réseaux communes |

---

## 14. Hors scope (v27.x)

- Boot sur macOS ou Linux
- Support ARM (Surface Pro X, etc.)
- Chiffrement du contenu de la clé USB
- Mode persistance (sauvegarde sur la clé)
- Interface multilingue (FR uniquement)
- NiTriTe Cloud (synchro logs distante)

---

## 15. Décisions d'architecture à valider

| # | Question | Recommandation |
|---|----------|----------------|
| D1 | WinPE 10 vs WinPE 11 ? | WinPE 11 (W10/W11 compatibles, drivers modernes) |
| D2 | WebView2 Fixed vs Download ? | Fixed Runtime — indispensable hors réseau |
| D3 | ISO uniquement ou flash direct ? | Les deux — `build-bootable.bat` génère ISO + option flash |
| D4 | Taille WebView2 (~200MB) acceptable ? | Oui — USB 8GB ciblée |
| D5 | Drivers réseau embarqués ou détectés ? | Embarqués (pas de réseau au boot pour télécharger) |
| D6 | Mode WinPE masque ou grise les features incompatibles ? | Masque (sidebar épurée) |
