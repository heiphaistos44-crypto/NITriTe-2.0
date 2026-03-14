import type { KBCategory } from "../knowledgeBase";

export const kbDeveloppement: KBCategory[] = [
  {
    id: "git-versionning",
    label: "Git & Versionnement",
    icon: "GitBranch",
    items: [
      {
        title: "Git — installation et configuration initiale",
        solution: [
          "Télécharger Git depuis git-scm.com (Windows) — inclut Git Bash",
          "Configuration minimale obligatoire après installation :",
          "git config --global user.name 'Votre Nom'",
          "git config --global user.email 'votre@email.com'",
          "git config --global core.autocrlf true (Windows) ou input (Linux/Mac)",
          "git config --global init.defaultBranch main",
          "Éditeur par défaut : git config --global core.editor 'code --wait' (VS Code)",
          "Vérifier la config : git config --list --global",
        ],
        code: `# Configuration complète Git initiale
git config --global user.name "Prénom Nom"
git config --global user.email "email@exemple.com"
git config --global core.autocrlf true
git config --global init.defaultBranch main
git config --global core.editor "code --wait"
git config --global pull.rebase false
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.lg "log --oneline --graph --decorate --all"

# Générer une clé SSH pour GitHub/GitLab
ssh-keygen -t ed25519 -C "votre@email.com"
# Ajouter la clé à l'agent SSH
Start-Service ssh-agent
ssh-add ~/.ssh/id_ed25519
# Afficher la clé publique à copier sur GitHub
Get-Content ~/.ssh/id_ed25519.pub`,
        note: "Utiliser SSH plutôt que HTTPS pour GitHub — plus sécurisé et ne demande pas de mot de passe à chaque push.",
      },
      {
        title: "Git — commandes quotidiennes essentielles",
        code: `# Initialiser un repo
git init
git clone https://github.com/user/repo.git

# Workflow de base
git status                          # État du working directory
git add fichier.txt                 # Stager un fichier
git add .                           # Stager tous les changements
git commit -m "feat: ajouter login" # Committer

# Branches
git branch feature/login            # Créer une branche
git checkout -b feature/login       # Créer + basculer
git switch -c feature/login         # Même chose (moderne)
git merge feature/login             # Merger dans la branche courante
git branch -d feature/login         # Supprimer branche locale

# Remote
git remote add origin https://...   # Ajouter remote
git push -u origin main             # Premier push
git pull origin main                # Récupérer les changements
git fetch --all                     # Récupérer sans merger

# Historique
git log --oneline --graph --all     # Voir l'historique
git diff HEAD~1 HEAD                # Diff avec le commit précédent
git blame fichier.ts                # Voir qui a modifié chaque ligne`,
        solution: [
          "git add -p : stager interactivement des hunks (portions de fichier)",
          "git stash : mettre de côté des changements temporairement",
          "git stash pop : récupérer les changements mis de côté",
          "git cherry-pick <hash> : appliquer un commit spécifique sur la branche courante",
          "git rebase main : rebaser la branche courante sur main",
        ],
      },
      {
        title: "Git — annuler des changements",
        code: `# Annuler des changements non committés
git restore fichier.ts              # Annuler modifications d'un fichier
git restore .                       # Annuler TOUT (dangereux)
git restore --staged fichier.ts     # Dé-stager un fichier

# Modifier le dernier commit
git commit --amend -m "nouveau message"  # Changer le message
git commit --amend --no-edit             # Ajouter des fichiers au dernier commit

# Revenir à un commit précédent
git revert <hash>                   # Créer un commit qui annule (safe)
git reset --soft HEAD~1             # Annuler le commit, garder les changements stagés
git reset --mixed HEAD~1            # Annuler le commit, garder les fichiers modifiés
git reset --hard HEAD~1             # DANGER: efface tout

# Sauvetage en urgence
git reflog                          # Voir TOUS les mouvements HEAD (incluant reset --hard)
git checkout <hash>                 # Retourner à n'importe quel état

# Nettoyer les fichiers non trackés
git clean -n                        # Prévisualiser ce qui sera supprimé
git clean -fd                       # Supprimer fichiers + dossiers non trackés`,
        note: "git reflog sauve souvent la mise — même après un reset --hard, les commits sont récupérables pendant 90 jours.",
      },
      {
        title: "Git — résoudre les conflits de merge",
        solution: [
          "Un conflit survient quand deux branches modifient la même ligne",
          "Git marque les conflits dans les fichiers avec <<<<<<, =======, >>>>>>>",
          "Ouvrir les fichiers en conflit dans VS Code (affiche un éditeur de merge visuel)",
          "Choisir : Accept Current Change, Accept Incoming Change, Accept Both",
          "Après résolution : git add fichier.ts puis git commit",
          "Aborter un merge problématique : git merge --abort",
          "Pour les rebases : git rebase --abort ou --continue après résolution",
        ],
        code: `# Workflow résolution de conflit
git merge feature/login
# CONFLICT (content): Merge conflict in src/auth.ts
# Ouvrir le fichier et résoudre manuellement

# Dans VS Code : ouvrir la palette de commande
# > Git: Open Merge Editor

# Après résolution manuelle
git add src/auth.ts
git merge --continue
# ou
git commit -m "merge: résolution conflit auth.ts"

# Utiliser un outil de merge graphique
git config --global merge.tool vscode
git config --global mergetool.vscode.cmd 'code --wait $MERGED'
git mergetool`,
      },
      {
        title: ".gitignore — patterns essentiels",
        code: `# Fichier .gitignore — exemples complets

# Node.js
node_modules/
dist/
build/
.env
.env.local
.env.*.local
npm-debug.log*
yarn-debug.log*

# Python
__pycache__/
*.py[cod]
*.pyo
venv/
.venv/
env/
*.egg-info/
dist/
.pytest_cache/

# Rust
target/
Cargo.lock  # (pour les bibliothèques; garder pour les apps)

# Java
*.class
*.jar
*.war
target/
.gradle/

# IDE
.idea/
.vscode/settings.json
*.suo
*.user
.vs/

# OS
.DS_Store
Thumbs.db
desktop.ini

# Logs & temp
*.log
*.tmp
*.bak
logs/
tmp/

# Secrets
.env*
*.key
*.pem
secrets/
credentials.json`,
        note: "gitignore.io génère automatiquement des .gitignore selon la stack (ex: gitignore.io/api/node,react,vscode).",
      },
    ],
  },
  {
    id: "nodejs-javascript",
    label: "Node.js & JavaScript",
    icon: "Code",
    items: [
      {
        title: "Node.js — installation et gestion des versions",
        solution: [
          "Recommandé : installer NVM (Node Version Manager) pour gérer plusieurs versions",
          "Windows : nvm-windows depuis github.com/coreybutler/nvm-windows",
          "nvm install 20 : installer Node.js LTS",
          "nvm use 20 : utiliser une version",
          "nvm list : voir les versions installées",
          "Alternative : installer directement depuis nodejs.org (LTS recommandée)",
          "Vérifier : node --version et npm --version",
          "npm update -g npm : mettre npm à jour",
        ],
        code: `# Via NVM Windows (recommandé)
nvm install lts
nvm install 20.11.0
nvm use 20
nvm list
nvm alias default 20

# Créer un projet Node.js
mkdir mon-projet && cd mon-projet
npm init -y                    # package.json par défaut
npm install express            # Installer une dépendance
npm install -D typescript      # Dépendance de dev
npm install -g nodemon         # Outil global

# Commandes npm utiles
npm list                       # Dépendances du projet
npm list -g --depth=0          # Packages globaux
npm outdated                   # Packages à mettre à jour
npm update                     # Mettre à jour les packages
npm audit                      # Vérifier les vulnérabilités
npm audit fix                  # Corriger automatiquement`,
      },
      {
        title: "npm / yarn / pnpm — gestion des paquets",
        code: `# npm (inclus avec Node.js)
npm install package-name          # Installer
npm install -D package-name       # Dev dependency
npm install -g package-name       # Global
npm uninstall package-name
npm run script-name               # Lancer un script de package.json
npm start                         # Alias pour npm run start
npm test                          # Alias pour npm run test

# yarn (yarn.dev)
npm install -g yarn
yarn add package-name
yarn add -D package-name
yarn remove package-name
yarn upgrade
yarn dlx create-next-app          # Equivalent à npx

# pnpm (pnpm.io — plus rapide, économe en disque)
npm install -g pnpm
pnpm add package-name
pnpm add -D package-name
pnpm install                      # Installe les deps du package.json
pnpm update

# npx — exécuter sans installer
npx create-react-app mon-app
npx create-vue@latest
npx prettier --write .`,
        note: "pnpm est recommandé pour les monorepos — utilise des liens symboliques pour économiser l'espace disque.",
      },
      {
        title: "TypeScript — configuration et compilation",
        code: `# Installation
npm install -D typescript @types/node
npx tsc --init                    # Créer tsconfig.json

# tsconfig.json recommandé pour projet Node.js
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "CommonJS",
    "lib": ["ES2022"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}

# Compilation
npx tsc                           # Compiler une fois
npx tsc --watch                   # Mode watch
npx tsc --noEmit                  # Vérifier sans compiler (CI)

# ts-node — exécuter TS directement
npm install -D ts-node
npx ts-node src/index.ts

# tsx — alternative moderne (plus rapide)
npm install -D tsx
npx tsx src/index.ts
npx tsx watch src/index.ts`,
      },
      {
        title: "Debug Node.js — techniques et outils",
        code: `# Debug via VS Code — launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "node",
      "request": "launch",
      "name": "Debug Node.js",
      "program": "\${workspaceFolder}/src/index.ts",
      "preLaunchTask": "tsc: build - tsconfig.json",
      "outFiles": ["\${workspaceFolder}/dist/**/*.js"],
      "sourceMaps": true
    },
    {
      "type": "node",
      "request": "attach",
      "name": "Attach to Process",
      "processId": "\${command:PickProcess}"
    }
  ]
}

# Lancer en mode debug
node --inspect src/index.js        # Port 9229
node --inspect-brk src/index.js   # Pause au démarrage
nodemon --inspect src/index.js    # Avec redémarrage auto

# Dans Chrome : chrome://inspect

# Profiling performances
node --prof src/index.js
node --prof-process isolate-*.log > processed.txt

# Memory leaks
node --expose-gc --inspect src/index.js
# Dans Chrome DevTools > Memory > Take Heap Snapshot`,
      },
    ],
  },
  {
    id: "python-dev",
    label: "Python — Développement",
    icon: "Code",
    items: [
      {
        title: "Python — environnements virtuels",
        solution: [
          "Toujours travailler dans un environnement virtuel — évite les conflits de versions",
          "venv (standard, inclus dans Python 3.3+)",
          "conda/miniconda : mieux pour la data science et les packages C",
          "poetry : gestion moderne des dépendances",
          "pyenv-win (Windows) : gérer plusieurs versions Python",
          "Activer l'env avant d'installer des packages",
        ],
        code: `# venv — standard
python -m venv .venv              # Créer l'environnement
.venv\\Scripts\\activate           # Windows
source .venv/bin/activate         # Linux/Mac
deactivate                        # Désactiver

# pip — gestion des paquets
pip install requests              # Installer
pip install -r requirements.txt   # Depuis un fichier
pip freeze > requirements.txt     # Exporter les dépendances
pip list --outdated               # Paquets obsolètes
pip show requests                 # Infos sur un paquet

# conda (Miniconda)
conda create -n monenv python=3.11
conda activate monenv
conda install numpy pandas matplotlib
conda deactivate
conda env list

# poetry (moderne)
pip install poetry
poetry new mon-projet
poetry add requests
poetry add --dev pytest
poetry install
poetry run python main.py
poetry shell`,
      },
      {
        title: "Python — débogage et profiling",
        code: `# pdb — débogueur intégré
import pdb; pdb.set_trace()        # Point d'arrêt
# ou Python 3.7+
breakpoint()

# Commandes pdb
# n (next) : ligne suivante
# s (step) : entrer dans la fonction
# c (continue) : continuer
# l (list) : afficher le code
# p variable : afficher une valeur
# q (quit) : quitter

# ipdb — version améliorée
pip install ipdb
import ipdb; ipdb.set_trace()

# VS Code debug — launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Python: Current File",
      "type": "python",
      "request": "launch",
      "program": "\${file}",
      "console": "integratedTerminal",
      "justMyCode": false
    }
  ]
}

# Profiling
python -m cProfile -o output.prof script.py
python -m pstats output.prof

# Memory profiling
pip install memory-profiler
python -m memory_profiler script.py`,
      },
      {
        title: "Python — packages essentiels par domaine",
        solution: [
          "Web : FastAPI (API moderne), Flask (léger), Django (full-stack)",
          "Data Science : pandas, numpy, matplotlib, seaborn, scikit-learn",
          "ML/IA : torch (PyTorch), tensorflow, transformers (HuggingFace)",
          "Tests : pytest, unittest, coverage",
          "HTTP : requests, httpx, aiohttp",
          "CLI : click, typer, argparse",
          "DB : SQLAlchemy, sqlite3 (stdlib), psycopg2 (PostgreSQL)",
          "Utilitaires : pydantic, rich, loguru, python-dotenv",
        ],
        code: `# Installer un stack complet Data Science
pip install pandas numpy matplotlib seaborn scikit-learn jupyter

# Stack Web FastAPI
pip install fastapi uvicorn sqlalchemy pydantic python-dotenv

# Tests
pip install pytest pytest-cov pytest-asyncio
pytest tests/                     # Lancer les tests
pytest --cov=src tests/           # Avec couverture
pytest -v -k "test_login"         # Filtrer les tests

# Formatage et linting
pip install ruff black isort mypy
ruff check .                      # Linter rapide
black .                           # Formater le code
mypy src/                         # Vérification des types

# Exemple .python-version (pyenv)
3.11.7`,
      },
    ],
  },
  {
    id: "vscode-ide",
    label: "VS Code — Configuration",
    icon: "Code",
    items: [
      {
        title: "VS Code — extensions indispensables",
        solution: [
          "Prettier : formatage automatique du code (tous langages)",
          "ESLint : analyse statique JavaScript/TypeScript",
          "GitLens : supercharge les fonctionnalités Git",
          "GitHub Copilot : complétion IA (abonnement nécessaire)",
          "Thunder Client : tester les API REST (alternative à Postman)",
          "Docker : gérer les containers depuis VS Code",
          "Vim / Neovim : édition vi dans VS Code",
          "Material Icon Theme / One Dark Pro : thème et icônes",
          "Remote - SSH : développer sur un serveur distant",
          "Live Share : collaboration en temps réel",
        ],
        code: `# Installer des extensions via CLI
code --install-extension esbenp.prettier-vscode
code --install-extension dbaeumer.vscode-eslint
code --install-extension eamodio.gitlens
code --install-extension pkief.material-icon-theme
code --install-extension ms-azuretools.vscode-docker
code --install-extension ms-vscode-remote.remote-ssh

# Exporter/importer ses extensions
code --list-extensions > extensions.txt
# Réinstaller depuis la liste
Get-Content extensions.txt | ForEach-Object { code --install-extension $_ }

# settings.json recommandé
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "editor.tabSize": 2,
  "editor.fontSize": 14,
  "editor.fontFamily": "JetBrains Mono, Fira Code, monospace",
  "editor.fontLigatures": true,
  "editor.minimap.enabled": false,
  "editor.wordWrap": "on",
  "terminal.integrated.defaultProfile.windows": "Git Bash",
  "files.autoSave": "afterDelay",
  "files.autoSaveDelay": 1000,
  "workbench.colorTheme": "One Dark Pro"
}`,
      },
      {
        title: "VS Code — raccourcis clavier indispensables",
        solution: [
          "Ctrl+P : ouvrir un fichier rapidement",
          "Ctrl+Shift+P : palette de commandes",
          "Ctrl+` : ouvrir/fermer le terminal",
          "Ctrl+D : sélectionner la prochaine occurrence",
          "Alt+Clic : multi-curseur",
          "Ctrl+Shift+L : sélectionner toutes les occurrences",
          "F12 : aller à la définition",
          "Alt+F12 : aperçu de la définition (peek)",
          "Shift+Alt+F : formater le document",
          "Ctrl+G : aller à la ligne",
          "Ctrl+Shift+E/G/X/D/F : Explorateur/Git/Extensions/Debug/Recherche",
          "Ctrl+K Ctrl+S : ouvrir les raccourcis clavier",
        ],
        code: `# Personnaliser les raccourcis — keybindings.json
[
  {
    "key": "ctrl+alt+t",
    "command": "workbench.action.terminal.new"
  },
  {
    "key": "ctrl+shift+d",
    "command": "editor.action.copyLinesDownAction",
    "when": "editorTextFocus"
  },
  {
    "key": "alt+up",
    "command": "editor.action.moveLinesUpAction",
    "when": "editorTextFocus"
  },
  {
    "key": "ctrl+shift+k",
    "command": "editor.action.deleteLines",
    "when": "editorTextFocus"
  }
]`,
      },
      {
        title: "Docker — démarrage rapide",
        solution: [
          "Docker Desktop (Windows/Mac) : interface graphique + CLI",
          "Concepts clés : image (template), container (instance qui tourne), volume (données persistantes)",
          "Dockerfile : instructions pour construire une image",
          "docker-compose.yml : orchestrer plusieurs containers",
          "Docker Hub : registre d'images officielles",
          "Les containers partagent le kernel du host (plus léger que les VMs)",
        ],
        code: `# Commandes Docker essentielles
docker pull nginx:latest           # Télécharger une image
docker run -d -p 8080:80 nginx    # Lancer un container
docker ps                          # Containers actifs
docker ps -a                       # Tous les containers
docker stop <id>                   # Arrêter
docker rm <id>                     # Supprimer container
docker rmi nginx                   # Supprimer image
docker logs <id>                   # Voir les logs
docker exec -it <id> bash          # Shell dans container

# Dockerfile simple (Node.js)
FROM node:20-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
EXPOSE 3000
CMD ["node", "src/index.js"]

# docker-compose.yml
version: '3.8'
services:
  app:
    build: .
    ports: ["3000:3000"]
    environment:
      - NODE_ENV=production
  db:
    image: postgres:16
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      POSTGRES_PASSWORD: secret

volumes:
  postgres_data:

# Lancer
docker compose up -d
docker compose logs -f app
docker compose down`,
      },
    ],
  },
];
