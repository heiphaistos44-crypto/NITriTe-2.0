<script setup lang="ts">
import { ref, computed } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NSearchBar from "@/components/ui/NSearchBar.vue";
import {
  BookOpen, Wifi, Zap, Shield, HardDrive,
  Monitor, Volume2, Usb, ChevronDown, ChevronRight,
  Terminal,
} from "lucide-vue-next";

const search = ref("");
const expandedCategory = ref<string | null>(null);
const expandedItem = ref<string | null>(null);

interface KBItem {
  title: string;
  symptoms: string;
  solution: string[];
  command?: string;
}

interface KBCategory {
  id: string;
  label: string;
  icon: any;
  items: KBItem[];
}

const categories: KBCategory[] = [
  {
    id: "reseau", label: "Reseau", icon: Wifi,
    items: [
      {
        title: "Pas de connexion Internet",
        symptoms: "Pages web inaccessibles, icone reseau avec triangle jaune",
        solution: [
          "Verifier le cable Ethernet ou la connexion Wi-Fi",
          "Redemarrer le routeur/modem (attendre 30s)",
          "Vider le cache DNS : ipconfig /flushdns",
          "Reinitialiser la pile TCP/IP : netsh int ip reset",
          "Reinitialiser Winsock : netsh winsock reset",
        ],
        command: "ipconfig /flushdns && netsh int ip reset && netsh winsock reset",
      },
      {
        title: "Wi-Fi deconnecte frequemment",
        symptoms: "Connexion instable, deconnexions repetees",
        solution: [
          "Mettre a jour le pilote Wi-Fi via le Gestionnaire de peripheriques",
          "Desactiver la gestion d'alimentation de l'adaptateur Wi-Fi",
          "Changer le canal Wi-Fi sur le routeur (eviter les canaux encombres)",
          "Verifier les interferences (micro-ondes, Bluetooth)",
        ],
      },
      {
        title: "DNS lent ou ne resout pas",
        symptoms: "Sites longs a charger, erreur DNS_PROBE_FINISHED_NXDOMAIN",
        solution: [
          "Changer les DNS : utiliser 8.8.8.8 / 8.8.4.4 (Google) ou 1.1.1.1 (Cloudflare)",
          "Vider le cache DNS : ipconfig /flushdns",
          "Verifier le fichier hosts : C:\\Windows\\System32\\drivers\\etc\\hosts",
        ],
        command: "ipconfig /flushdns",
      },
    ],
  },
  {
    id: "performance", label: "Performance", icon: Zap,
    items: [
      {
        title: "PC lent au demarrage",
        symptoms: "Demarrage > 2 minutes, bureau long a apparaitre",
        solution: [
          "Desactiver les programmes au demarrage (Gestionnaire des taches > Demarrage)",
          "Verifier l'espace disque (> 15% libre minimum)",
          "Lancer un nettoyage de disque : cleanmgr",
          "Verifier les malwares avec Windows Defender",
          "Envisager un SSD si disque dur mecanique",
        ],
        command: "cleanmgr",
      },
      {
        title: "Utilisation CPU a 100%",
        symptoms: "Ventilateurs bruyants, PC tres lent, taches ne repondent plus",
        solution: [
          "Ouvrir le Gestionnaire des taches (Ctrl+Shift+Echap)",
          "Identifier le processus consommateur",
          "Si svchost.exe : verifier Windows Update",
          "Si SearchIndexer : reconstruire l'index de recherche",
          "Scanner les malwares",
        ],
        command: "tasklist /FI \"STATUS eq running\" /SOR MEMUSAGE",
      },
      {
        title: "Memoire RAM saturee",
        symptoms: "Message 'memoire insuffisante', ralentissements",
        solution: [
          "Fermer les onglets de navigateur inutiles",
          "Verifier les fuites memoire dans le Gestionnaire des taches",
          "Augmenter la memoire virtuelle (Panneau de config > Systeme > Parametres avances)",
          "Envisager d'ajouter de la RAM physique",
        ],
      },
    ],
  },
  {
    id: "securite", label: "Securite", icon: Shield,
    items: [
      {
        title: "Windows Defender desactive",
        symptoms: "Icone bouclier avec croix rouge, notifications de securite",
        solution: [
          "Ouvrir Securite Windows (ms-settings:windowsdefender)",
          "Activer la protection en temps reel",
          "Verifier qu'aucun antivirus tiers ne bloque Defender",
          "Lancer : sc start WinDefend",
        ],
        command: "sc query WinDefend",
      },
      {
        title: "Suspicion de malware",
        symptoms: "Pop-ups, redirections web, programmes inconnus, lenteur soudaine",
        solution: [
          "Lancer un scan complet Windows Defender",
          "Demarrer en mode sans echec et scanner",
          "Verifier les programmes installes recemment",
          "Verifier les extensions de navigateur",
          "Reinitialiser les navigateurs si necessaire",
        ],
      },
    ],
  },
  {
    id: "stockage", label: "Stockage", icon: HardDrive,
    items: [
      {
        title: "Disque plein",
        symptoms: "Barre d'espace disque rouge, impossible de sauvegarder",
        solution: [
          "Lancer le nettoyage de disque : cleanmgr /d C:",
          "Vider la corbeille",
          "Supprimer les fichiers temporaires : del /q /f /s %TEMP%\\*",
          "Deplacer les gros fichiers sur un autre disque",
          "Desinstaller les programmes inutilises",
        ],
        command: "cleanmgr /d C:",
      },
      {
        title: "Disque dur bruyant ou lent",
        symptoms: "Clics, grattements, temps d'acces eleves",
        solution: [
          "Sauvegarder immediatement les donnees importantes",
          "Lancer un diagnostic : wmic diskdrive get status",
          "Verifier avec chkdsk /f /r (au redemarrage)",
          "Envisager le remplacement par un SSD",
        ],
        command: "wmic diskdrive get status",
      },
    ],
  },
  {
    id: "affichage", label: "Affichage", icon: Monitor,
    items: [
      {
        title: "Ecran noir au demarrage",
        symptoms: "PC demarre mais ecran reste noir, curseur visible ou non",
        solution: [
          "Essayer Ctrl+Alt+Suppr puis Gestionnaire des taches",
          "Tester avec un autre cable/port video",
          "Demarrer en mode sans echec (F8 ou Shift+Redemarrer)",
          "Mettre a jour le pilote graphique",
          "Reinitialiser l'affichage : Win+Ctrl+Shift+B",
        ],
      },
      {
        title: "Resolution incorrecte",
        symptoms: "Image floue, elements trop grands ou trop petits",
        solution: [
          "Clic droit bureau > Parametres d'affichage",
          "Selectionner la resolution recommandee",
          "Mettre a jour le pilote graphique",
          "Verifier le cable (HDMI/DP pour les hautes resolutions)",
        ],
      },
    ],
  },
  {
    id: "audio", label: "Audio", icon: Volume2,
    items: [
      {
        title: "Pas de son",
        symptoms: "Icone son avec croix, aucun son des haut-parleurs",
        solution: [
          "Verifier le volume et le peripherique de sortie",
          "Clic droit icone son > Ouvrir les parametres de son",
          "Lancer l'utilitaire de resolution des problemes audio",
          "Reinstaller le pilote audio : Gestionnaire de peripheriques > Controleurs audio",
          "Redemarrer le service audio : net stop audiosrv && net start audiosrv",
        ],
        command: "net stop audiosrv && net start audiosrv",
      },
    ],
  },
  {
    id: "peripheriques", label: "Peripheriques", icon: Usb,
    items: [
      {
        title: "Peripherique USB non reconnu",
        symptoms: "Message 'peripherique USB non reconnu', pas de detection",
        solution: [
          "Essayer un autre port USB",
          "Redemarrer le PC",
          "Mettre a jour les pilotes USB : Gestionnaire de peripheriques > Controleurs USB",
          "Desinstaller le peripherique dans le Gestionnaire puis rebrancher",
          "Verifier l'alimentation USB (hub alimente si necessaire)",
        ],
      },
      {
        title: "Imprimante ne fonctionne pas",
        symptoms: "Impression bloquee, imprimante hors ligne",
        solution: [
          "Verifier la connexion (USB/Wi-Fi)",
          "Redemarrer le spooler d'impression",
          "Supprimer et rajouter l'imprimante",
          "Mettre a jour les pilotes depuis le site du fabricant",
        ],
        command: "net stop spooler && net start spooler",
      },
    ],
  },
];

function toggleCategory(id: string) {
  expandedCategory.value = expandedCategory.value === id ? null : id;
  expandedItem.value = null;
}

function toggleItem(title: string) {
  expandedItem.value = expandedItem.value === title ? null : title;
}

const filteredCategories = computed(() => {
  if (!search.value) return categories;
  const q = search.value.toLowerCase();
  return categories
    .map((cat) => ({
      ...cat,
      items: cat.items.filter(
        (item) =>
          item.title.toLowerCase().includes(q) ||
          item.symptoms.toLowerCase().includes(q) ||
          item.solution.some((s) => s.toLowerCase().includes(q))
      ),
    }))
    .filter((cat) => cat.items.length > 0);
});

async function runCommand(cmd: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("run_system_command", { cmd: "cmd", args: ["/C", cmd] });
  } catch {
    // dev
  }
}
</script>

<template>
  <div class="kb-page">
    <div class="page-header">
      <h1><BookOpen :size="22" /> Base de Connaissances</h1>
      <p class="page-subtitle">Solutions aux problemes courants</p>
    </div>

    <NSearchBar v-model="search" placeholder="Rechercher un probleme..." />

    <div class="categories">
      <div v-for="cat in filteredCategories" :key="cat.id" class="category">
        <button class="cat-header" @click="toggleCategory(cat.id)">
          <component :is="cat.icon" :size="18" style="color: var(--accent-primary)" />
          <span class="cat-label">{{ cat.label }}</span>
          <span class="cat-count">{{ cat.items.length }} articles</span>
          <ChevronDown v-if="expandedCategory === cat.id" :size="16" />
          <ChevronRight v-else :size="16" />
        </button>

        <div v-if="expandedCategory === cat.id" class="cat-items">
          <div v-for="item in cat.items" :key="item.title" class="kb-item">
            <button class="item-header" @click="toggleItem(item.title)">
              <span class="item-title">{{ item.title }}</span>
              <ChevronDown v-if="expandedItem === item.title" :size="14" />
              <ChevronRight v-else :size="14" />
            </button>

            <div v-if="expandedItem === item.title" class="item-content">
              <div class="symptoms">
                <strong>Symptomes :</strong> {{ item.symptoms }}
              </div>
              <div class="solution">
                <strong>Solution :</strong>
                <ol>
                  <li v-for="(step, i) in item.solution" :key="i">{{ step }}</li>
                </ol>
              </div>
              <div v-if="item.command" class="command-block">
                <code>{{ item.command }}</code>
                <button class="run-btn" @click="runCommand(item.command!)">
                  <Terminal :size="12" /> Executer
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div v-if="filteredCategories.length === 0" class="empty-state">
        Aucun resultat pour "{{ search }}"
      </div>
    </div>
  </div>
</template>

<style scoped>
.kb-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.page-header h1 {
  font-size: 22px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 10px;
}
.page-subtitle {
  color: var(--text-muted);
  font-size: 13px;
  margin-top: 2px;
}
.categories {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.category {
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--bg-secondary);
}
.cat-header {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 14px 16px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  transition: background 0.15s;
}
.cat-header:hover { background: var(--bg-tertiary); }
.cat-count {
  margin-left: auto;
  font-size: 12px;
  color: var(--text-muted);
  font-weight: 400;
}
.cat-items {
  border-top: 1px solid var(--border);
}
.kb-item {
  border-bottom: 1px solid var(--border);
}
.kb-item:last-child { border-bottom: none; }
.item-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 12px 16px 12px 44px;
  border: none;
  background: transparent;
  cursor: pointer;
  font-family: inherit;
  font-size: 13px;
  color: var(--text-secondary);
  transition: background 0.15s;
}
.item-header:hover { background: var(--bg-tertiary); color: var(--text-primary); }
.item-content {
  padding: 0 16px 16px 44px;
  font-size: 13px;
  color: var(--text-secondary);
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.symptoms {
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  border-left: 3px solid var(--warning);
}
.solution ol {
  margin: 6px 0 0 18px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.command-block {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}
.command-block code {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  color: var(--accent-primary);
  flex: 1;
}
.run-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border: 1px solid var(--accent-primary);
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--accent-primary);
  cursor: pointer;
  font-family: inherit;
  font-size: 11px;
  transition: all 0.15s;
}
.run-btn:hover { background: var(--accent-primary); color: #fff; }
.empty-state {
  text-align: center;
  padding: 40px;
  color: var(--text-muted);
  font-size: 14px;
}
</style>
