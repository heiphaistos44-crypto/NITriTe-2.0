<script setup lang="ts">
import { ref } from "vue";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import { useNotificationStore } from "@/stores/notifications";
import { HardDrive, ExternalLink, Monitor, Info, ChevronDown, ChevronRight } from "lucide-vue-next";

const notifications = useNotificationStore();
// Toutes les categories sont rétractées par défaut — remplissage après définition de osSections
const collapsed = ref<Set<string>>(new Set());

function toggleSection(title: string) {
  if (collapsed.value.has(title)) collapsed.value.delete(title);
  else collapsed.value.add(title);
  collapsed.value = new Set(collapsed.value);
}

interface OsOption {
  name: string; version: string; description: string;
  url: string; badge: string; badgeVariant: "accent" | "success" | "warning" | "info" | "neutral";
}
interface OsSection { title: string; items: OsOption[]; }

const osSections: OsSection[] = [
  {
    title: "Microsoft Windows — Actuels",
    items: [
      { name: "Windows 11", version: "24H2", description: "Dernière version — interface modernisée. Nécessite TPM 2.0 + Secure Boot.", url: "https://www.microsoft.com/software-download/windows11", badge: "Recommandé", badgeVariant: "success" },
      { name: "Windows 10", version: "22H2", description: "Version stable, support jusqu'en oct. 2025. Compatible la majorité du matériel.", url: "https://www.microsoft.com/software-download/windows10", badge: "Support étendu", badgeVariant: "warning" },
      { name: "Windows 11 LTSC", version: "2024", description: "Long Term Servicing Channel — mises à jour minimales, idéal entreprise.", url: "https://www.microsoft.com/fr-fr/evalcenter/evaluate-windows-11-enterprise", badge: "LTSC", badgeVariant: "info" },
      { name: "Windows 10 LTSC", version: "2021", description: "Version entreprise sans apps préinstallées, support 2027.", url: "https://www.microsoft.com/fr-fr/evalcenter/evaluate-windows-10-enterprise", badge: "LTSC", badgeVariant: "info" },
    ],
  },
  {
    title: "Microsoft Windows — Serveurs",
    items: [
      { name: "Windows Server 2025", version: "2025", description: "Dernière version serveur Microsoft avec Active Directory et cloud.", url: "https://www.microsoft.com/en-us/evalcenter/evaluate-windows-server-2025", badge: "Serveur", badgeVariant: "accent" },
      { name: "Windows Server 2022", version: "21H2", description: "Serveur professionnel — Azure hybrid + Secured-core.", url: "https://www.microsoft.com/en-us/evalcenter/evaluate-windows-server-2022", badge: "Serveur", badgeVariant: "info" },
      { name: "Windows Server 2019", version: "1809", description: "Serveur stable largement déployé en entreprise.", url: "https://www.microsoft.com/en-us/evalcenter/evaluate-windows-server-2019", badge: "Serveur", badgeVariant: "neutral" },
      { name: "Windows Server 2016", version: "2016", description: "Serveur legacy encore supporté.", url: "https://www.microsoft.com/en-us/evalcenter/evaluate-windows-server-2016", badge: "Serveur", badgeVariant: "neutral" },
    ],
  },
  {
    title: "Microsoft Windows — Legacy",
    items: [
      { name: "Windows 8.1", version: "8.1", description: "Support terminé — usage legacy uniquement.", url: "https://www.microsoft.com/fr-fr/software-download/windows8ISO", badge: "End of Life", badgeVariant: "warning" },
      { name: "Windows 7 SP1", version: "7", description: "Support terminé — usage legacy / rétro-gaming uniquement.", url: "https://www.microsoft.com/fr-fr/software-download/windows7", badge: "End of Life", badgeVariant: "warning" },
    ],
  },
  {
    title: "macOS — Versions officielles Apple",
    items: [
      { name: "macOS Sequoia", version: "15", description: "Dernier macOS — Apple Silicon + Intel récents. App Store ou createinstallmedia.", url: "https://apps.apple.com/fr/app/macos-sequoia/id6596773750", badge: "Officiel", badgeVariant: "success" },
      { name: "macOS Sonoma", version: "14", description: "macOS 14 — compatibilité étendue Intel + Apple Silicon.", url: "https://apps.apple.com/fr/app/macos-sonoma/id6450717509", badge: "Officiel", badgeVariant: "success" },
      { name: "macOS Ventura", version: "13", description: "macOS 13 — Stage Manager, Continuity Camera.", url: "https://apps.apple.com/fr/app/macos-ventura/id1638787999", badge: "Officiel", badgeVariant: "info" },
      { name: "macOS Monterey", version: "12", description: "macOS 12 — dernière version compatible certains Mac 2015.", url: "https://apps.apple.com/fr/app/macos-monterey/id1576738294", badge: "Officiel", badgeVariant: "info" },
      { name: "macOS Big Sur", version: "11", description: "macOS 11 — premier macOS Apple Silicon natif.", url: "https://apps.apple.com/fr/app/macos-big-sur/id1526878132", badge: "Officiel", badgeVariant: "neutral" },
      { name: "macOS Catalina", version: "10.15", description: "Dernier macOS 32 bits — abandon applis 32 bits.", url: "https://apps.apple.com/fr/app/macos-catalina/id1466841314", badge: "Ancien", badgeVariant: "neutral" },
      { name: "macOS Mojave", version: "10.14", description: "Dark Mode, stacks — compatible GPU Metal.", url: "https://apps.apple.com/fr/app/macos-mojave/id1398502828", badge: "Ancien", badgeVariant: "neutral" },
      { name: "macOS High Sierra", version: "10.13", description: "APFS, HEVC, Metal 2 — compatible 2010+.", url: "https://apps.apple.com/fr/app/macos-high-sierra/id1246284741", badge: "Ancien", badgeVariant: "neutral" },
    ],
  },
  {
    title: "macOS — Hackintosh & Outils Communauté",
    items: [
      { name: "OpenCore Bootloader", version: "1.0.x", description: "Chargeur Hackintosh — remplaçant de Clover. Config via config.plist.", url: "https://github.com/acidanthera/OpenCorePkg/releases", badge: "Hackintosh", badgeVariant: "warning" },
      { name: "OCLP — OpenCore Legacy Patcher", version: "2.x", description: "macOS récent sur vieux Mac non supportés (2009-2019).", url: "https://github.com/dortania/OpenCore-Legacy-Patcher/releases", badge: "Mac anciens", badgeVariant: "warning" },
      { name: "gibMacOS", version: "latest", description: "Télécharge l'installateur macOS officiel depuis les serveurs Apple.", url: "https://github.com/corpnewt/gibMacOS", badge: "Communauté", badgeVariant: "accent" },
      { name: "OSX-KVM", version: "latest", description: "macOS dans une VM QEMU/KVM Linux — accélération matérielle KVM.", url: "https://github.com/kholia/OSX-KVM", badge: "VM Linux", badgeVariant: "accent" },
      { name: "Asahi Linux", version: "latest", description: "Linux natif sur Mac Apple Silicon (M1/M2/M3) — GPU accéléré.", url: "https://asahilinux.org/", badge: "Apple Silicon", badgeVariant: "info" },
      { name: "Guide createinstallmedia", version: "Apple", description: "Créer une clé USB d'installation macOS avec la commande officielle Apple.", url: "https://support.apple.com/fr-fr/101578", badge: "Guide Officiel", badgeVariant: "success" },
      { name: "Dortania Build Repo", version: "latest", description: "Kexts et outils Hackintosh compilés automatiquement.", url: "https://dortania.github.io/builds/", badge: "Kexts", badgeVariant: "neutral" },
      { name: "Clover Bootloader", version: "5xxx", description: "Ancien chargeur Hackintosh — supplanté par OpenCore.", url: "https://github.com/CloverHackyColor/CloverBootloader/releases", badge: "Legacy HK", badgeVariant: "neutral" },
    ],
  },
  {
    title: "Distributions Linux — Populaires",
    items: [
      { name: "Ubuntu", version: "24.04 LTS", description: "Distribution la plus populaire. Parfait pour débuter.", url: "https://ubuntu.com/download/desktop", badge: "Populaire", badgeVariant: "success" },
      { name: "Linux Mint", version: "22", description: "Basée sur Ubuntu, interface familière type Windows.", url: "https://linuxmint.com/download.php", badge: "Débutant", badgeVariant: "success" },
      { name: "Fedora", version: "41", description: "Technologies récentes, sponsorisé par Red Hat.", url: "https://fedoraproject.org/workstation/download", badge: "Moderne", badgeVariant: "accent" },
      { name: "Debian", version: "12", description: "Distribution stable et fiable, base de nombreuses distros.", url: "https://www.debian.org/download", badge: "Stable", badgeVariant: "info" },
      { name: "Arch Linux", version: "Rolling", description: "Distribution avancée, installation manuelle, toujours à jour.", url: "https://archlinux.org/download/", badge: "Avancé", badgeVariant: "warning" },
      { name: "Manjaro", version: "24", description: "Basée sur Arch mais plus accessible. Rolling release.", url: "https://manjaro.org/download/", badge: "Rolling", badgeVariant: "accent" },
      { name: "openSUSE", version: "Tumbleweed", description: "Distribution professionnelle stable et complète.", url: "https://get.opensuse.org/", badge: "Pro", badgeVariant: "info" },
      { name: "Pop!_OS", version: "24.04", description: "Par System76, optimisée gaming et dev.", url: "https://pop.system76.com/", badge: "Gaming", badgeVariant: "accent" },
      { name: "Zorin OS", version: "17", description: "Interface Windows-like, idéal migration vers Linux.", url: "https://zorin.com/os/download/", badge: "Migration", badgeVariant: "success" },
      { name: "Kali Linux", version: "2025", description: "Distribution spécialisée sécurité et pentest.", url: "https://www.kali.org/get-kali/", badge: "Sécurité", badgeVariant: "warning" },
      { name: "Rocky Linux", version: "9", description: "Alternative à CentOS, compatibilité RHEL.", url: "https://rockylinux.org/download", badge: "Serveur", badgeVariant: "info" },
      { name: "MX Linux", version: "23", description: "Légère et performante, basée sur Debian.", url: "https://mxlinux.org/download-links/", badge: "Léger", badgeVariant: "success" },
      { name: "Elementary OS", version: "8", description: "Design élégant inspiré de macOS.", url: "https://elementary.io/", badge: "Design", badgeVariant: "accent" },
    ],
  },
  {
    title: "ISOs Utilitaires & Sauvetage",
    items: [
      { name: "Hiren's BootCD PE", version: "1.0.2", description: "Environnement de dépannage Windows basé PE.", url: "https://www.hirensbootcd.org/download/", badge: "Dépannage", badgeVariant: "warning" },
      { name: "SystemRescue", version: "11", description: "Distribution de sauvetage Linux avec outils de récupération.", url: "https://www.system-rescue.org/Download/", badge: "Sauvetage", badgeVariant: "warning" },
      { name: "GParted Live", version: "1.6", description: "Outil de partitionnement bootable.", url: "https://gparted.org/download.php", badge: "Partition", badgeVariant: "accent" },
      { name: "Clonezilla", version: "3.1", description: "Clonage et imagerie de disques.", url: "https://clonezilla.org/downloads.php", badge: "Clone", badgeVariant: "accent" },
      { name: "Tails", version: "6", description: "OS portable axé vie privée et anonymat.", url: "https://tails.net/install/", badge: "Vie privée", badgeVariant: "info" },
      { name: "Ventoy", version: "1.0.99", description: "Clé USB multi-boot sans formatage à chaque ISO.", url: "https://www.ventoy.net/en/download.html", badge: "Multi-boot", badgeVariant: "success" },
      { name: "FreeBSD", version: "14", description: "OS open-source performant, base de nombreux serveurs.", url: "https://www.freebsd.org/where/", badge: "BSD", badgeVariant: "info" },
      { name: "ShredOS", version: "latest", description: "Effacement sécurisé de disques — nwipe, NIST 800-88.", url: "https://github.com/PartialVolume/shredos.x86_64/releases", badge: "Effacement", badgeVariant: "warning" },
    ],
  },
];

// Toutes les catégories rétractées par défaut
collapsed.value = new Set(osSections.map(s => s.title));

async function openDownload(url: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("open_url", { url });
  } catch {
    try {
      const { open } = await import("@tauri-apps/plugin-shell");
      await open(url);
    } catch {
      window.open(url, "_blank");
      notifications.info("Ouverture dans le navigateur");
    }
  }
}
</script>

<template>
  <div class="os-downloads-page">
    <div class="page-header">
      <div>
        <h1>Téléchargement OS</h1>
        <p class="page-subtitle">ISO officielles — Windows, macOS, Linux, utilitaires</p>
      </div>
    </div>

    <NCard>
      <div class="info-banner">
        <Info :size="20" style="color: var(--accent-primary); flex-shrink: 0;" />
        <p class="info-text">
          Les liens redirigent vers les pages officielles de téléchargement. Cliquez sur l'en-tête de chaque catégorie pour la replier.
        </p>
      </div>
    </NCard>

    <div v-for="section in osSections" :key="section.title" class="os-section">
      <button class="section-header-btn" @click="toggleSection(section.title)">
        <Monitor :size="16" />
        <span class="section-title">{{ section.title }}</span>
        <span class="section-count">{{ section.items.length }}</span>
        <component :is="collapsed.has(section.title) ? ChevronRight : ChevronDown" :size="16" class="chevron" />
      </button>

      <div v-if="!collapsed.has(section.title)" class="os-grid">
        <NCard v-for="os in section.items" :key="os.name" hoverable>
          <div class="os-card">
            <div class="os-info">
              <div class="os-top">
                <h3>{{ os.name }}</h3>
                <NBadge :variant="os.badgeVariant">{{ os.badge }}</NBadge>
                <span class="os-version">v{{ os.version }}</span>
              </div>
              <p class="os-desc">{{ os.description }}</p>
              <NButton variant="primary" size="sm" @click="openDownload(os.url)">
                <ExternalLink :size="14" />
                Télécharger
              </NButton>
            </div>
          </div>
        </NCard>
      </div>
    </div>

    <NCard>
      <template #header>
        <div class="section-header-info">
          <HardDrive :size="16" />
          <span>Informations utiles</span>
        </div>
      </template>
      <div class="tips-list">
        <div class="tip-item">Utilisez une clé USB d'au moins 8 Go pour créer un média d'installation.</div>
        <div class="tip-item">L'outil Media Creation Tool de Microsoft permet de créer une clé bootable facilement.</div>
        <div class="tip-item">Sauvegardez toujours vos données avant une réinstallation du système.</div>
        <div class="tip-item">Windows 11 nécessite TPM 2.0, Secure Boot, et 4 Go RAM minimum.</div>
        <div class="tip-item">Pour macOS Hackintosh, utilisez OpenCore — Clover est obsolète.</div>
      </div>
    </NCard>
  </div>
</template>

<style scoped>
.os-downloads-page { display: flex; flex-direction: column; gap: 16px; }
.page-header h1 { font-size: 24px; font-weight: 700; }
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }

.section-header-btn {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 10px 14px;
  cursor: pointer;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 15px;
  font-weight: 600;
  transition: all var(--transition-fast);
  text-align: left;
}

.section-header-btn:hover { background: var(--bg-tertiary); border-color: var(--accent-primary); }

.section-title { flex: 1; }

.section-count {
  font-size: 12px;
  background: var(--accent-muted);
  color: var(--accent-primary);
  padding: 2px 8px;
  border-radius: 99px;
  font-weight: 600;
}

.chevron { color: var(--text-muted); }

.os-section { display: flex; flex-direction: column; gap: 10px; }

.section-header-info { display: flex; align-items: center; gap: 8px; }

.info-banner { display: flex; gap: 12px; align-items: flex-start; }
.info-text { font-size: 13px; color: var(--text-secondary); line-height: 1.6; }

.os-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 12px;
}

.os-card { display: flex; gap: 16px; }

.os-info { flex: 1; display: flex; flex-direction: column; gap: 6px; }

.os-top { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }

.os-top h3 { font-size: 16px; font-weight: 700; color: var(--text-primary); }

.os-version {
  font-size: 11px;
  color: var(--text-muted);
  font-family: "JetBrains Mono", monospace;
  margin-left: auto;
}

.os-desc { font-size: 12px; color: var(--text-secondary); line-height: 1.5; }

.tips-list { display: flex; flex-direction: column; gap: 8px; }
.tip-item {
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  font-size: 13px;
  color: var(--text-secondary);
  border-left: 3px solid var(--accent-primary);
}
</style>
