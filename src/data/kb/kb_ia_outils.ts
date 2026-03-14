import type { KBCategory } from "../knowledgeBase";

export const kbIaOutils: KBCategory[] = [
  {
    id: "ia-locale",
    label: "IA Locale — LLM sur votre PC",
    icon: "Bot",
    items: [
      {
        title: "Ollama — faire tourner des LLMs localement",
        solution: [
          "Ollama : outil pour faire tourner des LLMs (LLama, Mistral, Gemma...) localement sur Windows",
          "Télécharger depuis ollama.com — installation simple (Windows Installer)",
          "Prérequis : GPU NVIDIA avec CUDA ou CPU (plus lent), 8 Go RAM minimum",
          "Les modèles sont téléchargés depuis la bibliothèque Ollama (bibliothèque en ligne)",
          "Modèles recommandés : llama3.2 (3B), mistral (7B), qwen2.5-coder (code)",
          "API REST locale : http://localhost:11434 — compatible OpenAI API",
          "Open WebUI : interface web pour Ollama (comme ChatGPT en local) — via Docker",
          "Quantization : Q4_K_M (bon équilibre qualité/taille), Q8_0 (meilleure qualité, plus lourd)",
        ],
        code: `# Installation Ollama
# Télécharger OllamaSetup.exe depuis ollama.com

# Ou via winget
winget install Ollama.Ollama

# Commandes Ollama
ollama pull llama3.2           # Télécharger un modèle
ollama pull mistral:7b
ollama pull qwen2.5-coder:7b   # Modèle spécialisé code
ollama pull nomic-embed-text   # Modèle d'embeddings

ollama list                    # Modèles téléchargés
ollama run llama3.2            # Lancer un chat interactif
ollama run llama3.2 "Explique Docker en 3 phrases"  # Requête directe
ollama stop llama3.2           # Arrêter le modèle
ollama rm llama3.2             # Supprimer le modèle

# API REST (compatible avec les clients OpenAI)
# Depuis PowerShell
$body = @{
  model = "llama3.2"
  messages = @(
    @{ role = "user"; content = "Qu'est-ce que le RAID 5 ?" }
  )
  stream = $false
} | ConvertTo-Json

$response = Invoke-RestMethod -Uri "http://localhost:11434/api/chat" -Method POST -Body $body -ContentType "application/json"
$response.message.content

# Streaming (comme ChatGPT)
$body = @{ model = "llama3.2"; prompt = "Explique TCP/IP"; stream = $true } | ConvertTo-Json
Invoke-WebRequest "http://localhost:11434/api/generate" -Method POST -Body $body -ContentType "application/json"

# Open WebUI (interface graphique via Docker)
docker run -d -p 3000:8080 --add-host=host.docker.internal:host-gateway ^
  -v open-webui:/app/backend/data ^
  --name open-webui --restart always ^
  ghcr.io/open-webui/open-webui:main`,
        note: "Avec un GPU NVIDIA (RTX 3060 12Go+), llama3.2:3b répond en 50ms/token. Sans GPU : 5-10s/token selon le CPU.",
      },
      {
        title: "Stable Diffusion — génération d'images locale",
        solution: [
          "Stable Diffusion : modèle de génération d'images IA open source, fait tourner localement",
          "Interfaces : AUTOMATIC1111 (classic), ComfyUI (workflow avancé), Forge (AUTOMATIC1111 optimisé)",
          "GPU requis : NVIDIA 4 Go VRAM minimum, 8 Go recommandé pour la plupart des modèles",
          "Modèles SDXL (Stable Diffusion XL) : meilleure qualité, 1024x1024 natif, 6-8 Go VRAM",
          "LoRA : fine-tuning léger pour un style ou sujet spécifique (fichiers de quelques Mo)",
          "VAE : Variational Autoencoder — améliore les couleurs et les détails fins",
          "Sampler : DPM++ 2M Karras (recommandé), DDIM, Euler a",
          "CFG Scale : force du prompt (7-9 typiquement), Sampling Steps : 20-30",
        ],
        code: `# Installation AUTOMATIC1111 (Windows)
# 1. Installer Python 3.10.x depuis python.org (cocher "Add to PATH")
# 2. Installer Git depuis git-scm.com
# 3. Cloner le repo
git clone https://github.com/AUTOMATIC1111/stable-diffusion-webui.git
cd stable-diffusion-webui

# 4. Télécharger un modèle de base depuis Hugging Face ou Civitai
# Placer le fichier .safetensors dans models/Stable-diffusion/

# 5. Lancer (installe automatiquement les dépendances au premier lancement)
webui-user.bat

# Options de lancement (dans webui-user.bat, ligne set COMMANDLINE_ARGS=)
--xformers                 # Accélération mémoire (Nvidia)
--medvram                  # Pour 4-6 Go VRAM
--lowvram                  # Pour <4 Go VRAM (lent)
--api                      # Activer l'API REST
--listen                   # Accessible depuis le réseau local
--port 7860

# API REST (après --api)
$body = @{
  prompt = "a beautiful landscape, detailed, 4k, masterpiece"
  negative_prompt = "blurry, ugly, bad anatomy"
  steps = 25
  cfg_scale = 7
  width = 512
  height = 512
  sampler_name = "DPM++ 2M Karras"
} | ConvertTo-Json

$response = Invoke-RestMethod "http://localhost:7860/sdapi/v1/txt2img" -Method POST -Body $body -ContentType "application/json"
# L'image est en base64 dans $response.images[0]
[System.IO.File]::WriteAllBytes("C:\\output.png", [Convert]::FromBase64String($response.images[0]))`,
        note: "ComfyUI est plus complexe mais beaucoup plus puissant et flexible grâce aux workflows visuels. Forge est AUTOMATIC1111 avec 30-50% de performance en plus.",
      },
      {
        title: "Whisper — transcription audio IA en local",
        solution: [
          "Whisper : modèle de reconnaissance vocale OpenAI, open source, très précis",
          "Supporte 99 langues incluant le français",
          "Modèles : tiny (75Mo, rapide), base (145Mo), small (465Mo), medium (1.5Go), large-v3 (3Go)",
          "GPU CUDA accélère 5-10x — fonctionne aussi sur CPU",
          "Whisper.cpp : port C++ ultra-rapide, fonctionne sans GPU",
          "Whisper Desktop : interface graphique Windows pour Whisper.cpp",
          "FasterWhisper : version optimisée Python, 4x plus rapide que Whisper original",
          "Cas d'usage : sous-titrer des vidéos, transcrire des réunions, convertir audio en texte",
        ],
        code: `# Installation via pip
pip install openai-whisper
# ou FasterWhisper (recommandé, plus rapide)
pip install faster-whisper

# Transcription basique
import whisper
model = whisper.load_model("medium")
result = model.transcribe("audio.mp3", language="fr")
print(result["text"])

# Avec FasterWhisper
from faster_whisper import WhisperModel
model = WhisperModel("medium", device="cuda", compute_type="float16")
segments, info = model.transcribe("audio.mp3", language="fr")
for segment in segments:
  print(f"[{segment.start:.2f}s -> {segment.end:.2f}s] {segment.text}")

# Générer des sous-titres SRT
from faster_whisper import WhisperModel

def transcribe_to_srt(audio_file, output_file):
  model = WhisperModel("medium", device="cuda")
  segments, _ = model.transcribe(audio_file, language="fr")

  with open(output_file, "w", encoding="utf-8") as f:
    for i, segment in enumerate(segments, 1):
      start = format_time(segment.start)
      end = format_time(segment.end)
      f.write(f"{i}\\n{start} --> {end}\\n{segment.text.strip()}\\n\\n")

def format_time(seconds):
  h = int(seconds // 3600)
  m = int((seconds % 3600) // 60)
  s = int(seconds % 60)
  ms = int((seconds % 1) * 1000)
  return f"{h:02d}:{m:02d}:{s:02d},{ms:03d}"

transcribe_to_srt("reunion.mp3", "sous-titres.srt")

# Via CLI whisper (après pip install openai-whisper)
whisper audio.mp3 --language fr --model medium --output_format srt
whisper video.mp4 --language fr --model large-v3 --output_dir ./subtitles`,
      },
      {
        title: "LM Studio — interface desktop pour LLMs",
        solution: [
          "LM Studio : application Windows/Mac avec interface graphique pour les LLMs locaux (lmstudio.ai)",
          "Télécharger et installer — inclut un gestionnaire de modèles intégré",
          "Rechercher et télécharger des modèles GGUF depuis Hugging Face directement dans l'app",
          "Chat interface : conversations similaires à ChatGPT",
          "Serveur local OpenAI-compatible : activer dans Local Server > Start Server (port 1234)",
          "Compatible avec les clients OpenAI : utiliser l'URL http://localhost:1234/v1",
          "Comparaison de modèles : tester plusieurs modèles côte à côte",
          "Format GGUF : format de quantization pour llama.cpp — efficace pour les CPU et GPU",
        ],
        code: `# Utiliser LM Studio comme serveur OpenAI local
# 1. Ouvrir LM Studio, charger un modèle
# 2. Local Server > Start Server
# URL : http://localhost:1234/v1

# Depuis Python (client openai)
pip install openai
from openai import OpenAI

client = OpenAI(base_url="http://localhost:1234/v1", api_key="not-needed")

response = client.chat.completions.create(
  model="local-model",  # Ignoré par LM Studio, utilise le modèle chargé
  messages=[
    {"role": "system", "content": "Tu es un expert en informatique Windows."},
    {"role": "user", "content": "Comment diagnostiquer un BSOD ?"}
  ],
  temperature=0.7,
  max_tokens=500
)
print(response.choices[0].message.content)

# Depuis PowerShell
$body = @{
  model = "local-model"
  messages = @(
    @{ role = "user"; content = "Explique-moi le RAID 5" }
  )
  temperature = 0.7
} | ConvertTo-Json

$response = Invoke-RestMethod "http://localhost:1234/v1/chat/completions" -Method POST -Body $body -ContentType "application/json"
$response.choices[0].message.content`,
      },
    ],
  },
  {
    id: "copilot-microsoft",
    label: "Copilot & IA Microsoft",
    icon: "Star",
    items: [
      {
        title: "GitHub Copilot — IA dans VS Code",
        solution: [
          "GitHub Copilot : assistant IA pour le code (abonnement payant : 10$/mois, gratuit pour les étudiants)",
          "Extension VS Code : ms-vscode.github-copilot et ms-vscode.github-copilot-chat",
          "Complétion inline : Copilot suggère du code en temps réel (Tab pour accepter, Esc pour refuser)",
          "Copilot Chat : conversation avec l'IA dans le panneau chat (Ctrl+Shift+I)",
          "@workspace : analyser l'ensemble du projet",
          "@terminal : aide avec les commandes terminal",
          "/explain : expliquer le code sélectionné",
          "/fix : corriger les erreurs dans le code sélectionné",
          "/tests : générer des tests unitaires pour le code sélectionné",
        ],
        code: `// GitHub Copilot — exemples d'utilisation

// 1. Demander une fonction via commentaire (puis Tab pour accepter)
// Fonction qui convertit des bytes en taille lisible (Ko, Mo, Go)
function formatBytes(bytes: number, decimals = 2): string {
  // Copilot complète automatiquement ici...
}

// 2. Copilot Chat — questions dans le panneau
// @workspace Où sont définis les composants Vue de la page diagnostic ?
// @workspace Quel fichier gère les routes de l'application ?
// /explain Explique ce que fait ce code
// /fix Il y a une fuite mémoire ici
// /tests Génère des tests unitaires pour cette fonction

// 3. Inline Chat (Ctrl+I sur une sélection)
// Refactore ce code pour utiliser async/await
// Traduis les commentaires en français
// Ajoute la gestion d'erreurs

// 4. Générer du boilerplate
// Crée un composant Vue 3 avec Options API pour afficher une liste de processus
// Génère un hook React qui fetch des données avec loading et error state

// 5. Agent mode (Copilot Workspace)
// "Ajoute une fonctionnalité de tri à cette table"
// "Convertis ce composant de JavaScript en TypeScript"`,
        note: "Copilot Free (depuis déc. 2024) : limité à 50 complétion/mois + 10 messages chat. Copilot Pro : illimité pour 10$/mois.",
      },
      {
        title: "Microsoft Copilot — intégration Windows 11",
        solution: [
          "Copilot Windows 11 : IA assistant intégré à Windows (Win+C ou bouton dans la barre des tâches)",
          "Copilot dans Microsoft 365 : Word, Excel, PowerPoint, Outlook, Teams (licence M365 Copilot requise)",
          "Copilot Pro : abonnement 22€/mois pour accès GPT-4o dans toutes les apps Microsoft",
          "Bing Chat → Microsoft Copilot : renommé en novembre 2023, basé sur GPT-4",
          "Edge Copilot : accessible depuis Edge avec le bouton Copilot (résumé de pages, explications)",
          "Copilot dans PowerShell (preview) : aide contextuelle dans Windows Terminal",
          "Excel Copilot : générer des formules, analyser des données, créer des graphiques en langage naturel",
          "Word Copilot : rédiger, résumer, reformuler, traduire des documents",
        ],
        code: `# PowerShell — interagir avec l'API Azure OpenAI / Copilot
# Nécessite une clé API Azure OpenAI

$endpoint = "https://votre-resource.openai.azure.com"
$apiKey = "votre-api-key"
$deployment = "gpt-4"

$body = @{
  messages = @(
    @{ role = "system"; content = "Tu es un assistant expert en administration Windows." }
    @{ role = "user"; content = "Génère un script PowerShell pour lister tous les services en échec" }
  )
  max_tokens = 500
  temperature = 0.7
} | ConvertTo-Json

$headers = @{
  "api-key" = $apiKey
  "Content-Type" = "application/json"
}

$response = Invoke-RestMethod "$endpoint/openai/deployments/$deployment/chat/completions?api-version=2024-02-01" \`
  -Method POST -Headers $headers -Body $body

Write-Host $response.choices[0].message.content

# Copilot Studio — créer un chatbot personnalisé
# Accès : copilotstudio.microsoft.com
# Crée des agents IA sur mesure avec des sources de données internes
# Intégration Teams, SharePoint, sites web`,
      },
      {
        title: "Automatisation avec l'IA — scripts intelligents",
        code: `# Script PowerShell avec IA intégrée (Ollama)
# Diagnostic automatique avec explication IA

function Get-AIExplanation {
  param([string]$Context, [string]$Question)

  $body = @{
    model = "llama3.2"
    messages = @(
      @{ role = "system"; content = "Tu es un expert en administration système Windows. Réponds en français de façon concise et pratique." }
      @{ role = "user"; content = "$Context\`n\`nQuestion: $Question" }
    )
    stream = $false
  } | ConvertTo-Json -Depth 5

  try {
    $response = Invoke-RestMethod "http://localhost:11434/api/chat" -Method POST -Body $body -ContentType "application/json"
    return $response.message.content
  } catch {
    return "Ollama non disponible. Démarrer avec: ollama serve"
  }
}

# Analyser les processus qui consomment le plus de CPU
$topProcesses = Get-Process | Sort-Object CPU -Descending | Select-Object -First 5
$processInfo = $topProcesses | ForEach-Object {
  "- $($_.Name) (PID: $($_.Id)) : CPU $([math]::Round($_.CPU, 1))s, RAM $([math]::Round($_.WorkingSet64/1MB, 1))Mo"
} | Out-String

$explanation = Get-AIExplanation \`
  -Context "Voici les 5 processus qui consomment le plus de CPU:\`n$processInfo" \`
  -Question "Ces processus semblent-ils normaux ? Y a-t-il des problèmes potentiels ?"

Write-Host "=== ANALYSE IA ===" -ForegroundColor Cyan
Write-Host $explanation

# Analyser les erreurs du journal des événements
$errors = Get-WinEvent -FilterHashtable @{LogName='System'; Level=2; StartTime=(Get-Date).AddHours(-24)} -MaxEvents 5 |
  Select-Object TimeCreated, ProviderName, Message

$errorInfo = $errors | ForEach-Object {
  "[$($_.TimeCreated)] $($_.ProviderName): $($_.Message.Substring(0, [Math]::Min(200, $_.Message.Length)))"
} | Out-String

if ($errors.Count -gt 0) {
  $analysis = Get-AIExplanation \`
    -Context "Erreurs système des dernières 24h:\`n$errorInfo" \`
    -Question "Quelles sont les causes probables et les solutions recommandées ?"
  Write-Host "\`n=== ANALYSE ERREURS ===" -ForegroundColor Red
  Write-Host $analysis
}`,
      },
    ],
  },
  {
    id: "outils-modernes",
    label: "Outils Modernes & Productivité",
    icon: "Wrench",
    items: [
      {
        title: "Windows Terminal — configuration avancée",
        solution: [
          "Windows Terminal : terminal moderne avec onglets, profils multiples, GPU rendering (Microsoft Store)",
          "Profiles : PowerShell, CMD, WSL Ubuntu, Git Bash, Azure Cloud Shell — configurer dans settings.json",
          "Oh My Posh : thème de terminal avec informations Git, Python env, heure (ohmyposh.dev)",
          "Nerd Fonts : polices avec icônes spéciales requises pour Oh My Posh (nerdfonts.com)",
          "JetBrains Mono Nerd Font ou Cascadia Code NF : polices recommandées",
          "Copilot dans Terminal : Windows Terminal preview a une intégration IA expérimentale",
          "Split panes : Alt+Shift+D (horizontal) ou Alt+Shift+Plus (vertical)",
          "Acrylic/Mica : fond transparent ou material Windows 11",
        ],
        code: `# Installation complète Windows Terminal
winget install Microsoft.WindowsTerminal
winget install JanDeDobbeleer.OhMyPosh

# Installer une Nerd Font (exemple : JetBrains Mono)
oh-my-posh font install

# Configurer Oh My Posh dans le profil PowerShell
Add-Content $PROFILE 'oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\\jandedobbeleer.omp.json" | Invoke-Expression'

# Terminal settings.json (~\AppData\Local\Packages\Microsoft.WindowsTerminal_*/LocalState/settings.json)
{
  "$help": "https://aka.ms/terminal-documentation",
  "defaultProfile": "{pwsh-guid}",
  "theme": "dark",
  "profiles": {
    "defaults": {
      "font": { "face": "JetBrainsMono Nerd Font", "size": 12 },
      "opacity": 90,
      "useAcrylic": true,
      "acrylicOpacity": 0.8,
      "cursorShape": "bar"
    },
    "list": [
      {
        "guid": "{pwsh-guid}",
        "name": "PowerShell",
        "source": "Windows.Terminal.PowershellCore",
        "colorScheme": "One Dark"
      },
      {
        "guid": "{wsl-guid}",
        "name": "Ubuntu",
        "source": "Windows.Terminal.Wsl",
        "startingDirectory": "~"
      }
    ]
  },
  "keybindings": [
    { "command": "closeTab", "keys": "ctrl+w" },
    { "command": "splitPane", "keys": "alt+shift+d" },
    { "command": "nextTab", "keys": "ctrl+tab" }
  ]
}`,
      },
      {
        title: "PowerToys — outils de productivité Windows",
        solution: [
          "PowerToys : suite d'outils de productivité Microsoft pour Windows (gratuit, open source)",
          "Installer : winget install Microsoft.PowerToys ou Microsoft Store",
          "FancyZones : layouts de fenêtres personnalisés (grilles, colonnes)",
          "PowerToys Run (Alt+Espace) : lanceur d'applications + calcul + recherche (alternative à Spotlight)",
          "Color Picker (Win+Shift+C) : sélectionner une couleur n'importe où sur l'écran",
          "Image Resizer : clic droit sur des images > Resize Pictures",
          "Keyboard Manager : remapper les touches du clavier",
          "Mouse Without Borders : contrôler plusieurs PC avec une seule souris/clavier",
          "Peek (Ctrl+Espace) : aperçu rapide de fichiers sans les ouvrir",
          "Text Extractor (Win+Shift+T) : OCR pour extraire du texte depuis n'importe quelle image/zone d'écran",
        ],
        code: `# Installation PowerToys
winget install Microsoft.PowerToys

# FancyZones — créer un layout personnalisé
# Maintenir Shift en déplaçant une fenêtre → zones apparaissent
# Win+~ (tilde) → activer/désactiver FancyZones

# PowerToys Run — plugins utiles
# = 2+2           → Calculatrice
# > commande      → Lancer dans le terminal
# // recherche    → Recherche web
# ~ fichier       → Chercher un fichier
# ? question      → Aide contextuelle

# Keyboard Manager — remapper CapsLock en Ctrl (très populaire)
# PowerToys > Keyboard Manager > Remap a key
# CapsLock → Ctrl Left

# Command Not Found (PowerShell 7.4+)
# Suggère winget install quand une commande est introuvable
Enable-ExperimentalFeature -Name PSFeedbackProvider
Enable-ExperimentalFeature -Name PSCommandNotFoundSuggestion

# Winget — gestionnaire de paquets Windows
winget search vlc
winget install VideoLAN.VLC
winget upgrade --all                     # Tout mettre à jour
winget list                              # Apps installées
winget export -o apps.json               # Exporter la liste
winget import -i apps.json               # Réinstaller depuis la liste (nouveau PC)`,
      },
    ],
  },
];
