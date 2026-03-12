# PRD — Fonctionnalités Manquantes NiTriTe Tauri v2.0

> **Objectif** : Document de référence exhaustif listant toutes les fonctionnalités de la version Python (v26.0) qui sont **absentes, partiellement implémentées ou non fonctionnelles** dans la version Tauri/Rust actuelle. Ce PRD est conçu pour permettre à un développeur IA (Claude Sonnet) d'implémenter chaque fonctionnalité manquante de manière autonome.

> **Date** : 2026-03-11
> **Base de comparaison** : [NiTriTe Python v26.0](file:///c:/Users/Momo/Desktop/Nitrite-26.0/) → [NiTriTe Tauri v2.0](file:///c:/Users/Momo/Desktop/Nitrite%202.0/)

---

## 📊 Résumé Comparatif Global

| Aspect | Python v26.0 | Tauri v2.0 | État |
|--------|-------------|-----------|------|
| Pages/Routes | 19 pages | 36 routes | ✅ Tauri a plus de pages |
| Commandes backend | ~150 fonctions | 190+ commandes | ✅ Backend riche |
| Base programmes | 716 apps (`programs.json`) | Via `searchIndex.ts` (30K) | ⚠️ À vérifier |
| Thèmes | 10 thèmes + Premium | Éditeur de thèmes avancé | ✅ Mieux dans Tauri |
| Internationalisation | FR/EN (130 clés) | ❌ Non implémenté | 🔴 MANQUANT |
| Profils Master Install | 7 profils prédéfinis | Page MasterInstall basique | ⚠️ PARTIEL |
| Agent IA Multi-API | Ollama + 14 APIs cloud | Ollama uniquement | 🔴 MANQUANT (Multi-API) |
| Export rapports | JSON/HTML/MD/TXT/PDF | JSON + TXT basique | 🔴 MANQUANT (HTML/PDF) |
| Monitoring graphiques | Canvas avec historique 60pts | Données temps réel | ⚠️ PARTIEL |
| Speed Test réseau | Download/Upload/Ping intégré | Ping uniquement | 🔴 MANQUANT |
| Scanner réseau local | ARP scan + hostname + vendor | Vue réseau basique | 🔴 MANQUANT |
| Scanner de ports | Scan ports configurable | `scan_ports` existe | ⚠️ À vérifier UI |

---

## Table des Matières

1. [Internationalisation (i18n)](#1-internationalisation-i18n)
2. [Agent IA Multi-API](#2-agent-ia-multi-api)
3. [Système de Profils Master Install](#3-système-de-profils-master-install)
4. [Export Rapports Multi-Format](#4-export-rapports-multi-format)
5. [Monitoring — Graphiques Historiques](#5-monitoring--graphiques-historiques)
6. [Réseau — Scanner Réseau Local](#6-réseau--scanner-réseau-local)
7. [Réseau — Test de Vitesse Intégré](#7-réseau--test-de-vitesse-intégré)
8. [Réseau — Scanner de Ports (UI)](#8-réseau--scanner-de-ports-ui)
9. [Score Santé PC](#9-score-santé-pc)
10. [Système de Favoris & Historique d'Installation](#10-système-de-favoris--historique-dinstallation)
11. [Génération de Scripts de Déploiement](#11-génération-de-scripts-de-déploiement)
12. [Vérification Multi-Méthode « Déjà Installé »](#12-vérification-multi-méthode-déjà-installé)
13. [Scripts Windows — Planificateur de Tâches](#13-scripts-windows--planificateur-de-tâches)
14. [Scripts Windows — Validation Sécurité](#14-scripts-windows--validation-sécurité)
15. [Base de Connaissances Enrichie](#15-base-de-connaissances-enrichie)
16. [Statistiques & Compteurs d'Utilisation](#16-statistiques--compteurs-dutilisation)
17. [Sauvegarde — Éléments Manquants](#17-sauvegarde--éléments-manquants)
18. [Système d'Élévation Admin Structuré](#18-système-délévation-admin-structuré)
19. [Nettoyage Application (Auto-Cleanup)](#19-nettoyage-application-auto-cleanup)
20. [Animations & Effets UI](#20-animations--effets-ui)

---

## 1. Internationalisation (i18n)

### Statut : 🔴 NON IMPLÉMENTÉ

### Description
La version Python dispose d'un système complet FR/EN avec ~130 clés de traduction. La version Tauri n'a **aucun** système d'internationalisation — tout est en français codé en dur dans les templates Vue.

### Spécifications d'Implémentation

#### Fichiers à créer

##### [NEW] `src/i18n/index.ts`
Point d'entrée du module i18n. Expose une fonction `t(key)` et un composable `useI18n()`.

```typescript
// Structure attendue
export interface I18nConfig {
  currentLanguage: 'fr' | 'en';
  translations: Record<string, Record<string, string>>;
}

export function useI18n() {
  // Retourne { t, locale, setLocale, availableLocales }
}
export function t(key: string): string {
  // Lookup dans translations[currentLanguage][key]
  // Fallback vers la clé si non trouvée
}
```

##### [NEW] `src/i18n/fr.json`
Traductions françaises — toutes les chaînes UI actuellement en dur dans les templates.

##### [NEW] `src/i18n/en.json`
Traductions anglaises équivalentes.

#### Catégories de clés à traduire (~200+ clés)

| Catégorie | Exemples de clés |
|-----------|-----------------|
| **Navigation** | `nav.dashboard`, `nav.diagnostic`, `nav.applications`, `nav.tools`, ... |
| **Diagnostic** | `diag.health_score`, `diag.benchmark_cpu`, `diag.tab_system`, `diag.tab_devices`, ... |
| **Applications** | `apps.install`, `apps.search`, `apps.category_browsers`, `apps.installing`, ... |
| **Optimisations** | `optim.cleanup`, `optim.services`, `optim.startup`, `optim.telemetry`, ... |
| **Paramètres** | `settings.theme`, `settings.language`, `settings.font_size`, `settings.animations`, ... |
| **Réseau** | `net.info`, `net.connections`, `net.scanner`, `net.ports`, `net.speed_test`, ... |
| **Backup** | `backup.create`, `backup.restore`, `backup.items`, `backup.archive`, ... |
| **AI** | `ai.send`, `ai.model`, `ai.thinking`, `ai.tool_calling`, ... |
| **Commun** | `common.loading`, `common.error`, `common.success`, `common.cancel`, `common.save`, ... |

#### Fichiers à modifier
Tous les 36 fichiers `.vue` dans `src/pages/` doivent remplacer les chaînes en dur par `{{ t('clé') }}`.

#### Persistence
Sauvegarder la langue choisie dans la config Tauri (`AppConfig`) et la charger au démarrage.

#### Fichier de référence Python
[translations.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/translations.py) — 287 lignes, ~130 clés par langue.

---

## 2. Agent IA Multi-API

### Statut : 🔴 PARTIELLEMENT IMPLÉMENTÉ (Ollama seul)

### Description
La version Python supporte **15 providers IA** avec fallback automatique, cache local et streaming. La version Tauri ne supporte que **Ollama** (local uniquement).

### Ce qui existe dans Tauri
- [ai/ollama.rs](file:///c:/Users/Momo/Desktop/Nitrite%202.0/src-tauri/src/ai/ollama.rs) — Connexion Ollama basique
- [ai/tool_calling.rs](file:///c:/Users/Momo/Desktop/Nitrite%202.0/src-tauri/src/ai/tool_calling.rs) — Tool calling sécurisé
- [AiAgentsPage.vue](file:///c:/Users/Momo/Desktop/Nitrite%202.0/src/pages/AiAgentsPage.vue) — Interface chat basique

### Fonctionnalités manquantes

#### 2.1 Support Multi-API Cloud

##### [NEW] `src-tauri/src/ai/providers/mod.rs`
Module trait générique pour tous les providers.

```rust
pub trait AiProvider: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn query(&self, prompt: &str, system_prompt: Option<&str>, model: Option<&str>) -> Result<String>;
    fn stream_query(&self, prompt: &str, callback: Box<dyn Fn(&str)>) -> Result<()>;
}
```

##### [NEW] Fichiers providers individuels
Un fichier par provider, chacun implémentant `AiProvider` :

| Provider | Fichier | API Base URL | Modèles |
|----------|---------|-------------|---------|
| Ollama | `ollama.rs` (existe) | `localhost:11434` | llama3, mistral, deepseek-r1 |
| DeepSeek | `deepseek.rs` | `api.deepseek.com` | deepseek-chat |
| Groq | `groq.rs` | `api.groq.com` | llama-3.1-70b |
| HuggingFace | `huggingface.rs` | `api-inference.huggingface.co` | Variable |
| OpenRouter | `openrouter.rs` | `openrouter.ai/api` | Multiples gratuits |
| Google Gemini | `gemini.rs` | `generativelanguage.googleapis.com` | gemini-pro |
| OpenAI | `openai.rs` | `api.openai.com` | gpt-4, gpt-3.5-turbo |
| Anthropic | `anthropic.rs` | `api.anthropic.com` | claude-3 |
| Mistral | `mistral.rs` | `api.mistral.ai` | mistral-large |
| Together | `together.rs` | `api.together.xyz` | Variable |
| Cohere | `cohere.rs` | `api.cohere.ai` | command |
| Fireworks | `fireworks.rs` | `api.fireworks.ai` | Variable |
| DeepInfra | `deepinfra.rs` | `api.deepinfra.com` | Variable |
| Novita | `novita.rs` | `api.novita.ai` | Variable |
| Perplexity | `perplexity.rs` | `api.perplexity.ai` | pplx-7b-online |

#### 2.2 Système de Fallback Automatique

```rust
// ai/router.rs
pub struct AiRouter {
    providers: Vec<Box<dyn AiProvider>>,
    api_keys: HashMap<String, String>,
    cache: LruCache<String, String>,
}

impl AiRouter {
    /// Essaie chaque provider dans l'ordre de priorité
    /// 1. Ollama (local) → 2. APIs gratuites → 3. APIs payantes
    pub fn query(&self, prompt: &str) -> Result<String> {
        for provider in &self.providers {
            if provider.is_available() {
                match provider.query(prompt, None, None) {
                    Ok(response) => {
                        self.cache.put(prompt.to_string(), response.clone());
                        return Ok(response);
                    }
                    Err(_) => continue,
                }
            }
        }
        Err("Aucun provider IA disponible".into())
    }
}
```

#### 2.3 Gestion des Clés API (UI)

##### [MODIFY] `src/pages/SettingsPage.vue`
Ajouter une section « Clés API IA » avec :
- Champ texte masqué pour chaque provider
- Bouton « Tester la connexion »
- Indicateur vert/rouge de disponibilité
- Sauvegarde dans `config/api_keys.json` (portable)

#### 2.4 Streaming Token par Token

##### [MODIFY] `src/pages/AiAgentsPage.vue`
Le chat doit afficher les réponses token par token (streaming) au lieu d'attendre la réponse complète. Utiliser les events Tauri (`window.emit`) pour le streaming côté Rust.

#### 2.5 Cache Local des Réponses

Stocker les réponses dans un cache LRU (100 entrées max) dans `config/ai_cache.json` pour éviter les requêtes redondantes.

#### 2.6 Historique des Conversations

Sauvegarder les conversations dans `config/ai_conversations.json` avec horodatage, modèle utilisé, et provider ayant répondu.

---

## 3. Système de Profils Master Install

### Statut : ⚠️ PARTIEL — Page existe, profils prédéfinis absents

### Description
La version Python a un système de profils d'installation complet avec 7 profils prédéfinis, gestion de favoris, historique d'installation, et suggestion d'apps manquantes. La version Tauri a une page [MasterInstallPage.vue](file:///c:/Users/Momo/Desktop/Nitrite%202.0/src/pages/MasterInstallPage.vue) mais sans gestion de profils structurée.

### Fonctionnalités à implémenter

#### 3.1 Profils Prédéfinis

```typescript
interface InstallProfile {
  id: string;
  name: string;
  icon: string;      // Emoji
  color: string;      // Couleur accent
  description: string;
  apps: string[];     // Liste des winget_id ou app_id
}

const PREDEFINED_PROFILES: InstallProfile[] = [
  {
    id: 'gaming',
    name: 'Gaming Station',
    icon: '🎮',
    color: '#e74c3c',
    description: 'Steam, Discord, MSI Afterburner, HWMonitor...',
    apps: ['Valve.Steam', 'Discord.Discord', 'MSI.Afterburner', ...],
  },
  {
    id: 'office',
    name: 'Bureau Pro',
    icon: '💼',
    color: '#3498db',
    description: 'LibreOffice, Chrome, Adobe Reader, 7-Zip, VLC...',
    apps: ['TheDocumentFoundation.LibreOffice', 'Google.Chrome', ...],
  },
  {
    id: 'dev',
    name: 'Développeur',
    icon: '👨‍💻',
    color: '#2ecc71',
    description: 'VS Code, Git, Node.js, Python, Docker...',
    apps: ['Microsoft.VisualStudioCode', 'Git.Git', ...],
  },
  {
    id: 'creative',
    name: 'Créatif',
    icon: '🎨',
    color: '#9b59b6',
    description: 'GIMP, Blender, Audacity, OBS, Inkscape...',
    apps: ['GIMP.GIMP', 'BlenderFoundation.Blender', ...],
  },
  {
    id: 'security',
    name: 'Sécurité',
    icon: '🔒',
    color: '#e67e22',
    description: 'Malwarebytes, VeraCrypt, KeePass, ProtonVPN...',
    apps: ['Malwarebytes.Malwarebytes', 'IDRIX.VeraCrypt', ...],
  },
  {
    id: 'essential',
    name: 'Essentiel',
    icon: '⚡',
    color: '#f39c12',
    description: 'Chrome, 7-Zip, VLC, LibreOffice, Adobe Reader',
    apps: ['Google.Chrome', '7zip.7zip', 'VideoLAN.VLC', ...],
  },
  {
    id: 'custom',
    name: 'Personnalisé',
    icon: '🛠️',
    color: '#95a5a6',
    description: 'Sélection manuelle par l\'utilisateur',
    apps: [],
  },
];
```

#### 3.2 Profils Personnalisés
- Création/suppression de profils custom
- Import/export de profils en JSON
- Stockage dans `config/install_profiles.json`

#### 3.3 Détection Apps Déjà Installées
Scanner le système pour identifier quelles apps d'un profil sont déjà installées (icône verte) vs manquantes (icône rouge). Utiliser :
1. Registre Windows (`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`)
2. `winget list` existant
3. Scan Program Files

#### Fichier de référence Python
[profiles_manager.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/profiles_manager.py) — 482 lignes.

---

## 4. Export Rapports Multi-Format

### Statut : 🔴 PARTIELLEMENT IMPLÉMENTÉ

### Description
La version Python exporte les diagnostics en JSON + copie presse-papiers. Le PRD original prévoit 5 formats d'export. La version Tauri a `save_export_file` et `save_content_to_path` mais ne génère pas de rapports formatés.

### Formats à implémenter

| Format | Priorité | Description |
|--------|----------|-------------|
| **JSON** | ✅ Existe | Export données brutes structurées |
| **HTML** | 🔴 MANQUANT | Rapport visuel avec graphiques, CSS intégré, imprimable |
| **Markdown** | 🔴 MANQUANT | Documentation technique, partage |
| **TXT** | ⚠️ Basique | Texte formaté lisible |
| **PDF** | 🔴 MANQUANT | Rapport client professionnel (via génération HTML → print) |

### Spécifications

#### [NEW] `src-tauri/src/utils/report_generator.rs`

```rust
pub enum ReportFormat { Html, Markdown, PlainText, Json, Pdf }

pub struct ReportGenerator;

impl ReportGenerator {
    /// Génère un rapport diagnostic complet au format spécifié
    pub fn generate(data: &serde_json::Value, format: ReportFormat) -> Result<String> {
        match format {
            ReportFormat::Html => Self::generate_html(data),
            ReportFormat::Markdown => Self::generate_markdown(data),
            ReportFormat::PlainText => Self::generate_text(data),
            ReportFormat::Json => Ok(serde_json::to_string_pretty(data)?),
            ReportFormat::Pdf => Self::generate_pdf_via_html(data),
        }
    }
    
    fn generate_html(data: &serde_json::Value) -> Result<String> {
        // Rapport HTML autonome avec :
        // - CSS intégré (dark theme cohérent avec NiTriTe)
        // - Logo NiTriTe
        // - Sections : Résumé, CPU, RAM, Disques, GPU, Réseau, OS
        // - Barres de progression colorées
        // - Horodatage + nom machine
        // - Meta viewport pour impression
    }
    
    fn generate_markdown(data: &serde_json::Value) -> Result<String> {
        // Rapport Markdown avec tableaux formatés
        // Compatible GitHub/GitLab
    }
}
```

#### [NEW] Commande Tauri `export_diagnostic_report`

```rust
#[tauri::command]
async fn export_diagnostic_report(
    format: String, // "html", "md", "txt", "json", "pdf"
    data: serde_json::Value,
) -> Result<String, NiTriTeError> {
    // Génère le rapport et le sauvegarde dans le dossier exports/
    // Retourne le chemin du fichier généré
}
```

#### [MODIFY] Pages Vue concernées
- `DiagnosticPage.vue` — Bouton « Exporter » avec choix de format
- `MonitoringPage.vue` — Export snapshot monitoring
- `NetworkPage.vue` — Export résultats réseau
- `StatsReportsPage.vue` — Génération rapport global

---

## 5. Monitoring — Graphiques Historiques

### Statut : ⚠️ PARTIEL — Données temps réel sans graphiques historiques

### Description
La version Python affiche des graphiques Canvas avec 60 derniers points de données (CPU, RAM, Réseau). La version Tauri collecte les données via `start_monitoring` mais l'historique graphique n'est pas implémenté.

### Fonctionnalités manquantes

#### 5.1 Graphiques temps réel (60 points)
- **CPU** : Courbe % usage global + % par cœur
- **RAM** : Courbe % usage + swap
- **Réseau** : Courbes upload/download en temps réel (débit Mbps)
- **Température** : Courbes CPU/GPU température

#### 5.2 Implémentation recommandée

##### [MODIFY] `src/pages/MonitoringPage.vue`
Intégrer une librairie de graphiques (Chart.js ou uPlot recommandé pour les performances) :

```typescript
// Composable pour les graphiques monitoring
interface MonitoringChart {
  cpuHistory: number[];      // 60 derniers % CPU
  ramHistory: number[];      // 60 derniers % RAM
  netUpHistory: number[];    // 60 derniers débit upload (bytes/s)
  netDownHistory: number[];  // 60 derniers débit download (bytes/s)
  tempHistory: number[];     // 60 dernières températures CPU
  maxPoints: 60;
  updateInterval: 1000;      // 1 seconde
}
```

##### Dépendance frontend à ajouter
```bash
npm install chart.js  # ou npm install uplot
```

#### Référence Python
[monitoring_dashboard.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/monitoring_dashboard.py) — lignes 180-188, `_create_chart_widget()` et `_draw_chart_line()`.

---

## 6. Réseau — Scanner Réseau Local

### Statut : 🔴 NON IMPLÉMENTÉ (UI + Backend partiel)

### Description
La version Python scanne le réseau local via ARP ping avec détection hostname et vendor MAC. La version Tauri a `get_network_extended` et `get_arp_table` mais pas de scanner réseau interactif.

### Spécifications

#### [NEW] `src-tauri/src/system/network_scanner.rs`

```rust
#[derive(Serialize)]
pub struct NetworkDevice {
    pub ip: String,
    pub mac: Option<String>,
    pub hostname: Option<String>,
    pub vendor: Option<String>,    // Via OUI MAC lookup
    pub response_time_ms: f32,
    pub is_gateway: bool,
}

#[tauri::command]
pub async fn scan_local_network(
    window: tauri::Window,
) -> Result<Vec<NetworkDevice>, NiTriTeError> {
    // 1. Détecter le subnet local (via ipconfig)
    // 2. Pour chaque IP du subnet :
    //    a. Ping (timeout 500ms)
    //    b. Si réponse: récupérer MAC via ARP
    //    c. Résoudre hostname via nbtstat ou DNS reverse
    //    d. Lookup vendor via base OUI (3 premiers octets MAC)
    // 3. Émettre progression via window.emit("scan-progress", ...)
}
```

#### [MODIFY] `src/pages/NetworkPage.vue`
Ajouter un onglet « Scanner Réseau » avec :
- Bouton « Scanner le réseau local »
- Barre de progression
- Tableau résultats : IP, Hostname, MAC, Vendor, Temps réponse
- Filtre / recherche dans les résultats
- Export CSV/JSON des résultats

#### Référence Python
[network_manager.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/network_manager.py) — fonctions `scan_network()`, `_scan_host()`, `_get_mac_address()`.

---

## 7. Réseau — Test de Vitesse Intégré

### Statut : 🔴 NON IMPLÉMENTÉ

### Description
La version Python intègre un test de vitesse complet (download, upload, ping) sans dépendance externe. La version Tauri n'a que le ping.

### Spécifications

#### [NEW] `src-tauri/src/system/speed_test.rs`

```rust
#[derive(Serialize)]
pub struct SpeedTestResult {
    pub download_mbps: f64,
    pub upload_mbps: f64,
    pub ping_ms: f64,
    pub jitter_ms: f64,
    pub server: String,
    pub timestamp: String,
}

#[tauri::command]
pub async fn run_speed_test(
    window: tauri::Window,
) -> Result<SpeedTestResult, NiTriTeError> {
    // 1. Test Ping (8.8.8.8, 1.1.1.1) — moyenne 4 essais
    // 2. Test Download — télécharger fichier test (~10 MB)
    //    Sources possibles : speed.cloudflare.com, speed.hetzner.de
    //    Émettre progression via events Tauri
    // 3. Test Upload — envoyer données vers endpoint test
    // 4. Calculer jitter (variation du ping)
}
```

#### [MODIFY] `src/pages/NetworkPage.vue`
Ajouter onglet « Test de Vitesse » avec :
- Bouton « Lancer le test »
- Jauge animée pendant le test (type speedometer)
- Résultats : Download ↓ Mbps, Upload ↑ Mbps, Ping ms
- Historique des tests précédents
- Comparaison avec la moyenne

#### Référence Python
[network_manager.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/network_manager.py) — fonctions `test_internet_speed()`, `_test_ping()`, `_test_download()`, `_test_upload()`.

---

## 8. Réseau — Scanner de Ports (UI)

### Statut : ⚠️ Backend existe, UI incomplète

### Description
Le backend Tauri a `system::net_tools::scan_ports` mais l'interface utilisateur dans `NetworkPage.vue` ne présente pas un scanner de ports interactif.

### Fonctionnalités UI manquantes

#### [MODIFY] `src/pages/NetworkPage.vue`
Ajouter un onglet « Scanner de Ports » avec :
- Champ IP/hostname cible
- Sélection plage de ports : « Ports communs (1-1024) », « Tous (1-65535) », « Personnalisé »
- Barre de progression pendant le scan
- Tableau résultats : Port, État (Ouvert/Fermé), Service (HTTP, HTTPS, SSH...)
- Mapping port → service intégré (top 100 services)

---

## 9. Score Santé PC

### Statut : 🔴 NON IMPLÉMENTÉ

### Description
La version Python calcule un score de santé PC sur 100 avec barre colorée (vert/orange/rouge). Ce score est basé sur CPU usage, RAM libre, espace disque, température, et mises à jour en attente.

### Spécifications

#### [NEW] `src-tauri/src/system/health_score.rs`

```rust
#[derive(Serialize)]
pub struct HealthScore {
    pub total: u32,              // Score /100
    pub rating: String,          // "Excellent", "Bon", "Moyen", "Critique"
    pub color: String,           // Couleur hex selon le score
    pub breakdown: Vec<HealthMetric>,
}

#[derive(Serialize)]
pub struct HealthMetric {
    pub name: String,            // "CPU", "RAM", "Disque", "Température"
    pub score: u32,              // Score partiel /100
    pub weight: f32,             // Poids dans le score total
    pub details: String,         // "Usage CPU : 45%"
}

#[tauri::command]
pub async fn get_health_score() -> Result<HealthScore, NiTriTeError> {
    // Calcul pondéré :
    // - CPU usage < 80% → 25 pts max
    // - RAM libre > 20% → 25 pts max
    // - Espace disque libre > 15% → 25 pts max
    // - Température CPU < 80°C → 15 pts max
    // - Pas de mise à jour critique → 10 pts max
}
```

#### [MODIFY] `src/pages/DiagnosticPage.vue`
Afficher le score santé en haut de la page diagnostic avec :
- Cercle progressif animé (score /100)
- Couleur dynamique (vert > 70, orange 40-70, rouge < 40)
- Breakdown par catégorie avec mini-barres

#### Référence Python
[advanced_pages.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/advanced_pages.py) — `_create_pc_health_section()`.

---

## 10. Système de Favoris & Historique d'Installation

### Statut : 🔴 NON IMPLÉMENTÉ

### Description
La version Python gère des favoris utilisateur et un historique complet des installations (app, date, succès/échec, durée).

### Spécifications

#### [NEW] `src-tauri/src/installer/favorites.rs`

```rust
#[derive(Serialize, Deserialize)]
pub struct InstallHistory {
    pub entries: Vec<InstallHistoryEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct InstallHistoryEntry {
    pub app_name: String,
    pub app_id: String,
    pub timestamp: String,
    pub success: bool,
    pub duration_seconds: u32,
    pub method: String, // "winget", "chocolatey", "scoop", "direct"
}

#[tauri::command]
pub async fn add_favorite(app_id: String) -> Result<(), NiTriTeError>;
#[tauri::command]
pub async fn remove_favorite(app_id: String) -> Result<(), NiTriTeError>;
#[tauri::command]
pub async fn get_favorites() -> Result<Vec<String>, NiTriTeError>;
#[tauri::command]
pub async fn get_install_history(limit: u32) -> Result<Vec<InstallHistoryEntry>, NiTriTeError>;
#[tauri::command]
pub async fn get_most_installed(limit: u32) -> Result<Vec<(String, u32)>, NiTriTeError>;
```

#### Stockage
- Favoris : `config/favorites.json`
- Historique : `config/install_history.json`

#### [MODIFY] `src/pages/ApplicationsPage.vue`
- Icône ⭐ sur chaque app pour toggle favoris
- Onglet « Favoris » dans la sidebar catégories
- Section « Historique récent » en bas de page
- Badge « Top installés » sur les apps fréquemment installées

---

## 11. Génération de Scripts de Déploiement

### Statut : 🔴 NON IMPLÉMENTÉ

### Description
La version Python permet d'exporter un script `.bat` ou `.ps1` pour déployer les programmes sélectionnés sur d'autres machines sans NiTriTe.

### Spécifications

#### [NEW] `src-tauri/src/installer/script_generator.rs`

```rust
pub struct ScriptGenerator;

impl ScriptGenerator {
    /// Génère un script batch (.bat)
    pub fn generate_batch(programs: &[AppEntry]) -> String {
        // @echo off + boucle winget install --id ... pour chaque app
    }
    
    /// Génère un script PowerShell (.ps1)
    pub fn generate_powershell(programs: &[AppEntry]) -> String {
        // #Requires -RunAsAdministrator + foreach avec try/catch
    }
}

#[tauri::command]
async fn generate_deploy_script(
    app_ids: Vec<String>,
    format: String, // "bat" ou "ps1"
) -> Result<String, NiTriTeError>;
```

#### [MODIFY] `src/pages/ApplicationsPage.vue` & `src/pages/MasterInstallPage.vue`
Bouton « Exporter Script de Déploiement » qui ouvre un dialogue de sauvegarde fichier.

---

## 12. Vérification Multi-Méthode « Déjà Installé »

### Statut : ⚠️ PARTIEL — WinGet uniquement

### Description
La version Python vérifie si une app est installée via 3 méthodes : registre Windows, `winget list`, et scan des dossiers Program Files. La version Tauri utilise principalement WinGet.

### Méthodes manquantes à ajouter dans `installer/`

```rust
/// Vérifie si un programme est installé via le registre Windows
fn check_registry_installation(app_name: &str) -> bool {
    // Chercher dans :
    // HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall
    // HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall
    // HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall
}

/// Vérifie dans les chemins d'installation communs
fn check_common_paths(app_name: &str) -> bool {
    // Chercher dans :
    // C:\Program Files\
    // C:\Program Files (x86)\
    // %APPDATA%\
    // %LOCALAPPDATA%\
}
```

---

## 13. Scripts Windows — Planificateur de Tâches

### Statut : ⚠️ Backend existe (`create_scheduled_task`), logique métier scripts manquante

### Description
La version Python a un planificateur de scripts intégré permettant de programmer l'exécution de scripts custom à intervalles réguliers (daily, weekly, monthly).

### Fonctionnalités manquantes

#### [MODIFY] `src/pages/ScriptsPage.vue`
Ajouter un onglet « Planifier » avec :
- Liste des scripts planifiés (nom, fréquence, prochaine exécution)
- Dialogue de planification :
  - Sélection du script
  - Type : Quotidien / Hebdomadaire / Mensuel
  - Heure d'exécution
  - Bouton Activer/Désactiver
- Historique des exécutions planifiées

#### Référence Python
[script_automation.py](file:///c:/Users/Momo/Desktop/Nitrite-26.0/src/script_automation.py) — classe `TaskScheduler`, 50+ lignes.

---

## 14. Scripts Windows — Validation Sécurité

### Statut : 🔴 NON IMPLÉMENTÉ

### Description
La version Python valide la sécurité des scripts avant exécution en détectant des patterns dangereux.

### Spécifications

#### [NEW] `src-tauri/src/scripts/validator.rs`

```rust
pub struct ScriptValidator;

#[derive(Serialize)]
pub struct ValidationResult {
    pub is_safe: bool,
    pub risk_level: String,  // "low", "medium", "high", "critical"
    pub warnings: Vec<String>,
}

impl ScriptValidator {
    pub fn validate(code: &str, language: &str) -> ValidationResult {
        // Patterns dangereux à détecter :
        let dangerous_patterns = vec![
            "Remove-Item -Recurse -Force",
            "rm -rf",
            "Format-Volume",
            "Clear-Disk",
            "Stop-Computer",
            "Restart-Computer",
            "Set-ExecutionPolicy Unrestricted",
            "Invoke-Expression",  // Sur URL externe
            "reg delete",
            "del /f /s /q C:\\",
            "diskpart",
            "format",
        ];
        // Retourner warnings + risk_level
    }
}
```

#### [MODIFY] `src/pages/ScriptsPage.vue`
- Validation automatique avant exécution
- Affichage avertissements colorés (jaune pour medium, rouge pour high/critical)
- Confirmation obligatoire pour scripts à risque élevé

---

## 15. Base de Connaissances Enrichie

### Statut : ⚠️ Page existe, contenu insuffisant

### Description
La version Python a 19 articles techniques dans `data/knowledge/` avec recherche full-text et scoring TF-IDF. Le PRD prévoit 1500+ conseils catégorisés.

### Fonctionnalités manquantes

#### 15.1 Articles embarqués
Créer des fichiers Markdown dans `src/data/knowledge/` couvrant :

| Catégorie | Nombre articles | Exemples |
|-----------|----------------|----------|
| Windows | 30+ | Réparer Windows Update, BSOD communs, Activation |
| Réseau | 20+ | DNS, Wi-Fi lent, VPN, Firewall |
| Sécurité | 15+ | Malware removal, Ransomware, Password managers |
| Hardware | 20+ | SSD vs HDD, RAM upgrade, Thermal paste |
| Software | 15+ | Navigateurs, LibreOffice, VLC |

#### 15.2 Recherche Full-Text
- Indexation de tous les articles au démarrage
- Recherche fuzzy dans titres + contenu
- Scoring de pertinence

#### 15.3 Liens contextuels vers NiTriTe
Chaque article doit inclure des liens vers les pages NiTriTe pertinentes (ex: « Voir la page Optimisations pour nettoyer le système »).

---

## 16. Statistiques & Compteurs d'Utilisation

### Statut : 🔴 NON IMPLÉMENTÉ (page `StatsReportsPage.vue` existe mais données absentes)

### Description
La version Python conserve des compteurs d'utilisation (installations, nettoyages, diagnostics) et un historique des actions.

### Spécifications

#### [NEW] `src-tauri/src/utils/stats.rs`

```rust
#[derive(Serialize, Deserialize, Default)]
pub struct AppStats {
    pub total_installs: u32,
    pub total_cleanups: u32,
    pub total_diagnostics: u32,
    pub total_scripts_run: u32,
    pub total_backups: u32,
    pub total_ai_queries: u32,
    pub first_use: Option<String>,
    pub last_use: String,
    pub action_history: Vec<ActionEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionEntry {
    pub timestamp: String,
    pub action_type: String,  // "install", "cleanup", "diagnostic", "script", "backup"
    pub details: String,
    pub success: bool,
}
```

#### Commandes Tauri
```rust
#[tauri::command]
async fn get_stats() -> Result<AppStats, NiTriTeError>;
#[tauri::command]
async fn log_action(action_type: String, details: String, success: bool) -> Result<(), NiTriTeError>;
```

#### [MODIFY] `src/pages/StatsReportsPage.vue`
- Dashboard avec compteurs visuels (grandes tuiles)
- Graphique historique des actions (barres par semaine)
- Tableaux détaillés par catégorie
- Export rapport d'activité

---

## 17. Sauvegarde — Éléments Manquants

### Statut : ⚠️ PARTIEL — 7 éléments sur 20+ implémentés

### Description
La version Python sauvegarde 20+ éléments. La version Tauri ([backup/collector.rs](file:///c:/Users/Momo/Desktop/Nitrite%202.0/src-tauri/src/backup/collector.rs)) en supporte environ 7.

### Éléments manquants à ajouter

| Élément | Commande système | État |
|---------|-----------------|------|
| Favoris navigateurs (Chrome/Edge/Firefox/Brave/Opera/Vivaldi) | Lecture fichiers `Bookmarks` / `places.sqlite` | 🔴 |
| Clés BitLocker | `manage-bde -protectors -get C:` | 🔴 |
| Licence Windows | `slmgr /dli` (existe `run_slmgr`) | ⚠️ |
| Licence Office | Registre `HKLM\SOFTWARE\Microsoft\Office` | 🔴 |
| Règles pare-feu | `netsh advfirewall export` | 🔴 |
| Tâches planifiées | `schtasks /query /fo CSV` | ⚠️ (`get_scheduled_tasks` existe) |
| Variables d'environnement | Registre `HKLM\SYSTEM\...\Environment` | ⚠️ (`get_environment_variables` existe) |
| Fichiers Bureau | Archive ZIP du Desktop | 🔴 |
| Polices installées | `Get-ItemProperty "HKLM:\SOFTWARE\...\Fonts"` | 🔴 |
| Fonctionnalités Windows | `dism /online /get-features` | 🔴 |
| Tailles dossiers utilisateur | Scan Desktop/Documents/Downloads | ⚠️ (`get_folder_sizes` existe) |
| Point de restauration | `Checkpoint-Computer` | ✅ Existe |
| Archive ZIP complète | Compression de toute la sauvegarde | 🔴 |

#### [MODIFY] `src-tauri/src/backup/collector.rs`
Ajouter les collecteurs manquants comme fonctions séparées avec le même pattern que les existants.

#### [MODIFY] `src/pages/BackupPage.vue`
Afficher tous les éléments sauvegardables avec checkboxes et taille estimée.

---

## 18. Système d'Élévation Admin Structuré

### Statut : ⚠️ PARTIEL — Élévation ad-hoc via PowerShell mais pas structurée

### Description
La version Python a un module dédié `elevation_helper.py` avec des fonctions structurées pour l'élévation UAC. La version Tauri fait de l'élévation inline dans chaque commande.

### Recommandation
Centraliser l'élévation dans un module unique :

#### [NEW] `src-tauri/src/utils/elevation.rs`

```rust
pub struct ElevationHelper;

impl ElevationHelper {
    /// Vérifie si le processus est administrateur
    pub fn is_admin() -> bool;
    
    /// Exécute une commande avec élévation UAC (1 seul popup)
    pub fn run_elevated(commands: &[&str], timeout_s: u64) -> Result<CommandOutput>;
    
    /// Exécute un lot de commandes avec 1 seul popup UAC
    pub fn run_elevated_batch(commands: &[&str]) -> Result<Vec<CommandOutput>>;
}
```

---

## 19. Nettoyage Application (Auto-Cleanup)

### Statut : ⚠️ `cleanup_on_exit` existe mais pas le nettoyage complet

### Description
La version Python génère un script batch qui nettoie toutes les traces de NiTriTe après fermeture (dossier temp, logs, Python embeddé).

### Ce qui manque
Le `cleanup_on_exit` Tauri nettoie les fichiers temporaires mais ne propose pas :
- Interface utilisateur pour choisir quoi nettoyer
- Calcul de la taille récupérable avant nettoyage
- Nettoyage des logs uniquement (option légère)
- Génération de script batch de nettoyage post-fermeture

#### [MODIFY] `src/pages/SettingsPage.vue`
Ajouter section « Nettoyage NiTriTe » :
- Liste des éléments nettoyables (logs, cache, config) avec taille
- Boutons « Nettoyer logs uniquement » et « Nettoyage complet »
- Estimation espace récupérable

---

## 20. Animations & Effets UI

### Statut : ⚠️ CSS transitions basiques, animations avancées manquantes

### Description
La version Python a un `AnimationEngine` avec fade-in, slide-in, et ripple effect Material Design. La version Tauri utilise des transitions CSS basiques.

### Améliorations recommandées

#### [MODIFY] `src/assets/` (CSS global)
Ajouter :
- **Ripple effect** au clic sur les boutons et cartes (Material Design)
- **Fade-in** au chargement des pages (transition route Vue)
- **Slide-in** pour les éléments de sidebar
- **Skeleton loading** pendant le chargement des données
- **Micro-animations** sur les métriques (compteurs qui s'incrémentent)
- **Hover effects** enrichis sur les cartes et boutons

```css
/* Exemple ripple effect */
.ripple {
  position: relative;
  overflow: hidden;
}
.ripple::after {
  content: '';
  position: absolute;
  border-radius: 50%;
  background: rgba(255,255,255,0.3);
  transform: scale(0);
  animation: ripple-anim 0.6s ease-out;
}
@keyframes ripple-anim {
  to { transform: scale(4); opacity: 0; }
}
```

---

## 📋 Récapitulatif par Priorité

### 🔴 Priorité Haute (Impact utilisateur majeur)

| # | Fonctionnalité | Effort estimé |
|---|---------------|---------------|
| 1 | Internationalisation FR/EN | 3-4 jours |
| 2 | Agent IA Multi-API | 4-5 jours |
| 4 | Export Rapports HTML/PDF | 2-3 jours |
| 9 | Score Santé PC | 1 jour |
| 17 | Sauvegarde éléments manquants | 2-3 jours |

### 🟡 Priorité Moyenne (Parité fonctionnelle)

| # | Fonctionnalité | Effort estimé |
|---|---------------|---------------|
| 3 | Profils Master Install | 2 jours |
| 5 | Graphiques historiques monitoring | 2 jours |
| 6 | Scanner réseau local | 2 jours |
| 7 | Test de vitesse intégré | 1-2 jours |
| 10 | Favoris & Historique install | 1 jour |
| 11 | Génération scripts déploiement | 1 jour |
| 16 | Statistiques & Compteurs | 1-2 jours |

### 🟢 Priorité Basse (Nice-to-have)

| # | Fonctionnalité | Effort estimé |
|---|---------------|---------------|
| 8 | Scanner ports (UI) | 0.5 jour |
| 12 | Vérification multi-méthode installé | 1 jour |
| 13 | Planificateur scripts | 1 jour |
| 14 | Validation sécurité scripts | 0.5 jour |
| 15 | Base connaissances enrichie | 2 jours |
| 18 | Élévation admin structurée | 0.5 jour |
| 19 | Auto-cleanup UI | 0.5 jour |
| 20 | Animations UI avancées | 1 jour |

---

## 🏗️ Nouvelles Fonctionnalités Tauri (Non présentes en Python)

> La version Tauri a **aussi ajouté des fonctionnalités absentes de Python** :

| Fonctionnalité | Page Tauri |
|---------------|-----------|
| Désinstallateur propre avec résidus | `UninstallerPage.vue` |
| Clonage système (Image + Robocopy) | `ClonePage.vue` |
| Récupération de données (Shadow Copy, USN, Corbeille) | `DataRecoveryPage.vue` |
| Analyseur BSOD | `BsodAnalyzerPage.vue` |
| Éditeur de fichier Hosts | `HostsEditorPage.vue` |
| Gestionnaire de démarrage (Boot Manager) | `BootManagerPage.vue` |
| Support WSL | `WslPage.vue` |
| Gestion Bluetooth | `BluetoothPage.vue` |
| Nettoyeur avancé (gros fichiers) | `CleanerPage.vue` |
| Historique performances | `PerfHistoryPage.vue` |
| Points de restauration | `RestorePointsPage.vue` |
| Éditeur de thème visuel | `ThemeEditorPage.vue` |
| Téléchargement OS & USB Tools | `OsDownloadsPage.vue` |
| Debloat Windows (12 actions) | Via `OptimizationsPage.vue` |
| Dashboard centralisé | `DashboardPage.vue` |
| Benchmark dédié (CPU/RAM/Disk/GPU) | `BenchmarkPage.vue` |

Ces fonctionnalités sont **à conserver et améliorer**, pas à retirer.
