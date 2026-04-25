import type { Component } from "vue";
import {
  Monitor, Cpu, MemoryStick, HardDrive, Globe, Headphones,
  Usb, Battery, Package, Play, Zap, Printer, Key,
  RefreshCw, ScanLine, FileText, FolderOpen,
  CircuitBoard, Wifi, Server, Shield, Activity, FolderTree,
  Users, History, Lock, Wrench, Gauge, Trash2, AlertTriangle,
  Settings, Terminal, Bluetooth,
} from "lucide-vue-next";

export interface TabDef   { id: string; label: string; icon: Component; groupId: string }
export interface GroupDef { id: string; label: string; icon: Component }

export const TABS: TabDef[] = [
  // ── Matériel ──────────────────────────────────────
  { id: "os",            label: "Système OS",         icon: Monitor,       groupId: "hardware" },
  { id: "bios",          label: "BIOS / UEFI",         icon: CircuitBoard,  groupId: "hardware" },
  { id: "mobo",          label: "Carte Mère",           icon: CircuitBoard,  groupId: "hardware" },
  { id: "cpu",           label: "Processeur",           icon: Cpu,           groupId: "hardware" },
  { id: "gpu",           label: "GPU",                  icon: Monitor,       groupId: "hardware" },
  { id: "ram",           label: "RAM",                  icon: MemoryStick,   groupId: "hardware" },
  { id: "disks",         label: "Disques & Volumes",    icon: HardDrive,     groupId: "hardware" },
  // ── Périphériques ─────────────────────────────────
  { id: "monitors",      label: "Écrans",               icon: Monitor,       groupId: "devices"  },
  { id: "audio",         label: "Audio",                icon: Headphones,    groupId: "devices"  },
  { id: "usb",           label: "USB",                  icon: Usb,           groupId: "devices"  },
  { id: "battery",       label: "Batterie",             icon: Battery,       groupId: "devices"  },
  { id: "power",         label: "Énergie",              icon: Zap,           groupId: "devices"  },
  { id: "printers",      label: "Imprimantes",          icon: Printer,       groupId: "devices"  },
  { id: "bluetooth",     label: "Bluetooth",            icon: Bluetooth,     groupId: "devices"  },
  // ── Réseau ────────────────────────────────────────
  { id: "network",       label: "Adaptateurs",          icon: Wifi,          groupId: "network"  },
  { id: "connections",   label: "Connexions TCP",        icon: Activity,      groupId: "network"  },
  { id: "outils-reseau", label: "Outils Réseau",        icon: Wifi,          groupId: "network"  },
  { id: "hosts",         label: "Fichier Hosts",         icon: FileText,      groupId: "network"  },
  { id: "wsl",           label: "WSL / Linux",           icon: Terminal,      groupId: "network"  },
  // ── Système & Logiciels ───────────────────────────
  { id: "software",      label: "Applications",         icon: Package,       groupId: "system"   },
  { id: "env",           label: "Variables Env.",        icon: Server,        groupId: "system"   },
  { id: "startup",       label: "Démarrage",            icon: Play,          groupId: "system"   },
  { id: "processes",     label: "Processus",            icon: Activity,      groupId: "system"   },
  { id: "services",      label: "Services",             icon: Server,        groupId: "system"   },
  { id: "tasks",         label: "Tâches planif.",        icon: RefreshCw,     groupId: "system"   },
  { id: "folders",       label: "Dossiers",             icon: FolderTree,    groupId: "system"   },
  // ── Windows ───────────────────────────────────────
  { id: "updates",       label: "Mises à jour",         icon: RefreshCw,     groupId: "windows"  },
  { id: "pilotes",       label: "Pilotes",              icon: HardDrive,     groupId: "windows"  },
  { id: "activation",    label: "Activation",           icon: Key,           groupId: "windows"  },
  { id: "license",       label: "Licences & Clés",      icon: Key,           groupId: "windows"  },
  { id: "boot",          label: "Boot Manager",          icon: Settings,      groupId: "windows"  },
  { id: "nettoyeur",     label: "Nettoyeur",            icon: Trash2,        groupId: "windows"  },
  // ── Sécurité ──────────────────────────────────────
  { id: "security",      label: "Sécurité",             icon: Shield,        groupId: "security" },
  { id: "comptes",       label: "Comptes",              icon: Users,         groupId: "security" },
  { id: "parefeu",       label: "Pare-feu",             icon: Shield,        groupId: "security" },
  { id: "partages",      label: "Partages",             icon: FolderOpen,    groupId: "security" },
  { id: "registre",      label: "Registre",             icon: Key,           groupId: "security" },
  { id: "historique",    label: "Historique",           icon: History,       groupId: "security" },
  { id: "certificats",   label: "Certificats",          icon: Lock,          groupId: "security" },
  // ── Outils & Diagnostic ──────────────────────────
  { id: "performances",  label: "Performances",         icon: Activity,      groupId: "tools"    },
  { id: "perf-history",  label: "Historique Perf",      icon: Activity,      groupId: "tools"    },
  { id: "benchmark",     label: "Benchmark",            icon: Gauge,         groupId: "tools"    },
  { id: "bsod",          label: "BSOD",                 icon: AlertTriangle, groupId: "tools"    },
  { id: "reparation",    label: "Réparation",           icon: Wrench,        groupId: "tools"    },
  { id: "tools",         label: "Boîte à Outils",       icon: Globe,         groupId: "tools"    },
  { id: "scan",          label: "Scan Avancée",         icon: ScanLine,      groupId: "tools"    },
];

export const GROUPS: GroupDef[] = [
  { id: "hardware",  label: "Matériel",            icon: Cpu      },
  { id: "devices",   label: "Périphériques",       icon: Usb      },
  { id: "network",   label: "Réseau",              icon: Wifi     },
  { id: "system",    label: "Système",             icon: Package  },
  { id: "windows",   label: "Windows",             icon: Settings },
  { id: "security",  label: "Sécurité",            icon: Shield   },
  { id: "tools",     label: "Outils & Diagnostic", icon: Gauge    },
];
