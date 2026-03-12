<script setup lang="ts">
import { computed } from "vue";
import {
  CheckCircle, AlertTriangle, RefreshCw, ScanLine, Lock, LockOpen,
  Key, HardDrive, Cpu, MemoryStick, Monitor, Battery,
  FileDown, FileText, FileCode, Wrench,
} from "lucide-vue-next";
import NProgress from "@/components/ui/NProgress.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NButton from "@/components/ui/NButton.vue";
import NBadge from "@/components/ui/NBadge.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import { invoke } from "@tauri-apps/api/core";
import { useNotificationStore } from "@/stores/notifications";

const props = defineProps<{
  scanning: boolean;
  scanProgress: number;
  scanStep: string;
  scanResult: any;
  scanProblems: string[];
  batteries: any[];
  onRunScan: () => void;
}>();

function kbStr(v: number) { return v >= 1024 ? `${(v / 1024).toFixed(0)} MB` : `${v} KB`; }

// ── Solutions recommandées ────────────────────────────────────────────────────
interface Solution { problem: string; action: string; repairKey?: string; severity: "critical" | "warning" | "info" }

const scanSolutions = computed<Solution[]>(() => {
  if (!props.scanResult) return [];
  const sr = props.scanResult;
  const sol: Solution[] = [];
  if (!sr.firewall_enabled) sol.push({ problem: "Pare-feu désactivé", action: "Activer le pare-feu (tous profils)", repairKey: "enable_firewall", severity: "critical" });
  if (!sr.defender_enabled) sol.push({ problem: "Defender (temps réel) inactif", action: "Réactiver via Paramètres → Sécurité Windows", severity: "critical" });
  if (sr.smbv1_enabled) sol.push({ problem: "SMBv1 activé (vulnérable)", action: "Désactiver SMBv1 via Fonctionnalités Windows", severity: "critical" });
  if (sr.wmi_subscriptions > 0) sol.push({ problem: `${sr.wmi_subscriptions} abonnement(s) WMI suspect(s)`, action: "Inspecter les abonnements WMI (indicateur malware)", severity: "critical" });
  if (sr.pending_reboot) sol.push({ problem: "Redémarrage requis", action: "Redémarrer le PC pour appliquer les mises à jour", severity: "warning" });
  if (sr.last_update_days > 60) sol.push({ problem: `Dernière MAJ il y a ${sr.last_update_days} jours`, action: "Lancer Windows Update", repairKey: "wu_usoclient", severity: "warning" });
  if (sr.defender_definition_age_days > 7) sol.push({ problem: `Définitions Defender datant de ${sr.defender_definition_age_days} jours`, action: "Mettre à jour les signatures Defender", repairKey: "defender_update", severity: "warning" });
  if (sr.dism_status && !sr.dism_status.toLowerCase().includes("sain") && !sr.dism_status.toLowerCase().includes("healthy")) sol.push({ problem: "Composant Windows corrompu (DISM)", action: "Lancer DISM /RestoreHealth", repairKey: "dism_restore", severity: "critical" });
  if (sr.sfc_status && sr.sfc_status.toLowerCase().includes("corrupt")) sol.push({ problem: "Fichiers système corrompus (SFC)", action: "Exécuter SFC /scannow", repairKey: "sfc", severity: "critical" });
  if (sr.temp_folder_size_mb > 2048) sol.push({ problem: `Fichiers temp volumineux (${(sr.temp_folder_size_mb/1024).toFixed(1)} GB)`, action: "Nettoyer %TEMP%", repairKey: "temp_cleanup", severity: "warning" });
  if (sr.disk_usage?.some((d: any) => d.used_percent > 90)) sol.push({ problem: "Disque(s) à plus de 90% de capacité", action: "Nettoyer les fichiers temporaires et le cache", repairKey: "diskcleanup", severity: "critical" });
  if (sr.disk_usage?.some((d: any) => d.used_percent > 80)) sol.push({ problem: "Disque(s) à plus de 80% de capacité", action: "Activer Storage Sense", repairKey: "storage_sense", severity: "warning" });
  if (!sr.tpm_present) sol.push({ problem: "TPM absent ou désactivé", action: "Activer le TPM dans le BIOS (requis pour Windows 11)", severity: "warning" });
  if (!sr.secure_boot) sol.push({ problem: "Secure Boot désactivé", action: "Activer Secure Boot dans le BIOS/UEFI", severity: "warning" });
  if (sr.rdp_enabled) sol.push({ problem: "Bureau à distance (RDP) activé", action: "Désactiver si non nécessaire (Paramètres → Système → Bureau à distance)", severity: "info" });
  if (sr.guest_enabled) sol.push({ problem: "Compte Invité activé", action: "Désactiver le compte Invité (lusrmgr.msc)", severity: "warning" });
  if (sr.suspicious_processes?.length > 0) sol.push({ problem: `${sr.suspicious_processes.length} processus hors chemins sécurisés`, action: "Vérifier manuellement ces processus avec Process Explorer", severity: "warning" });
  if (!sr.network_ok) sol.push({ problem: "Pas de connectivité Internet", action: "Réinitialiser la pile réseau (Winsock + IP)", repairKey: "net_reset_all", severity: "critical" });
  if (sr.winget_upgradable?.length > 5) sol.push({ problem: `${sr.winget_upgradable.length} logiciels obsolètes`, action: "Mettre à jour via WinGet (onglet Mises à jour)", severity: "info" });
  return sol;
});

async function exportScanTxt() {
  if (!props.scanResult) return;
  const sr = props.scanResult;
  const W = 78;
  const SEP  = "=".repeat(W);
  const THIN = "-".repeat(W);

  function sec(title: string): string {
    const t = `  ${title}  `;
    const r = W - t.length;
    const l = Math.floor(r / 2);
    return "=".repeat(l) + t + "=".repeat(r - l);
  }
  function kv(label: string, value: string): string {
    return `  ${(label + " :").padEnd(26)} ${value}`;
  }
  function bar(pct: number, w = 20): string {
    const filled = Math.round(pct / 100 * w);
    return "[" + "#".repeat(filled) + ".".repeat(w - filled) + "]";
  }

  const lines: string[] = [
    SEP,
    sec("RAPPORT SCAN TOTAL — NiTriTe"),
    `${"  Generated :".padEnd(28)} ${new Date().toLocaleString()}`,
    SEP, "",
  ];

  // =========================================================
  // PARTIE 1 : COMPOSANTS DU PC
  // =========================================================

  lines.push(sec("=== COMPOSANTS DU PC ==="), "");

  // --- IDENTITE SYSTEME & BIOS ---
  lines.push(sec("IDENTITE SYSTEME & BIOS"), THIN,
    kv("Fabricant",             sr.system_manufacturer || "N/A"),
    kv("Modele",                sr.system_model        || "N/A"),
    kv("N. Serie",              sr.system_serial       || "N/A"),
    kv("BIOS Fabricant",        sr.bios_manufacturer   || "N/A"),
    kv("BIOS Version",          sr.bios_version        || "N/A"),
    kv("BIOS Date",             sr.bios_date           || "N/A"),
    "");

  // --- COMPOSANTS MATERIELS ---
  lines.push(sec("COMPOSANTS MATERIELS"), THIN,
    kv("CPU",                   `${sr.cpu_name}  --  ${sr.cpu_cores} coeurs / ${sr.cpu_threads || "?"} threads @ ${sr.cpu_frequency_ghz || "?"} GHz`),
    kv("CPU Utilisation",       `${sr.cpu_usage_percent?.toFixed(1) || "?"}%`),
    kv("CPU Temperature",       sr.cpu_temperature || "N/A"),
    kv("RAM",                   `${sr.ram_used_gb?.toFixed(1) || "?"} / ${sr.ram_total_gb?.toFixed(0) || "?"} GB (${sr.ram_usage_percent?.toFixed(0) || "?"}%)`),
    kv("RAM Detail",            sr.ram_detail || "N/A"),
    kv("Memoire virtuelle",     `${sr.virtual_memory_total_mb || "?"} MB total  /  ${sr.virtual_memory_available_mb || "?"} MB libre`),
    kv("GPU",                   `${sr.gpu_name || "N/A"}  --  VRAM: ${sr.gpu_vram_mb >= 1024 ? (sr.gpu_vram_mb / 1024).toFixed(0) + " GB" : (sr.gpu_vram_mb || 0) + " MB"}`),
    kv("Carte mere",            sr.motherboard || "N/A"),
    kv("Ecrans",                sr.monitors_detail || sr.screen_resolution || "N/A"),
    kv("Plan alimentation",     sr.power_plan || "N/A"),
    "");

  // --- STOCKAGE ---
  if (sr.storage_items?.length) {
    lines.push(sec("STOCKAGE PHYSIQUE"), THIN);
    for (const s of sr.storage_items) {
      lines.push(kv(s.model || "N/A", `${s.size_gb} GB  --  ${s.media_type}  --  ${s.interface_type}  --  ${s.health}`));
    }
    lines.push("");
  }
  lines.push(sec("ESPACE DISQUE (VOLUMES)"), THIN);
  for (const d of sr.disk_usage || []) {
    lines.push(kv(d.drive, `${d.used_percent.toFixed(0).padStart(3)}%  ${bar(d.used_percent)}  ${d.free_gb.toFixed(1)} GB libres / ${d.total_gb.toFixed(0)} GB`));
  }
  lines.push("");

  // --- RESEAU ---
  lines.push(sec("RESEAU"), THIN,
    kv("Connectivite",          sr.network_ok ? "OK (8.8.8.8 joignable)" : "!!! HORS LIGNE !!!"),
    kv("Adaptateurs actifs",    sr.network_adapters_summary || "N/A"),
  );
  if (sr.open_ports?.length) {
    lines.push(kv("Ports ecoute", sr.open_ports.slice(0, 25).join(", ") + (sr.open_ports.length > 25 ? `  (+${sr.open_ports.length - 25})` : "")));
  }
  lines.push("");

  // =========================================================
  // PARTIE 2 : INFORMATIONS WINDOWS
  // =========================================================

  lines.push(sec("=== INFORMATIONS WINDOWS ==="), "");

  // --- IDENTITE WINDOWS ---
  lines.push(sec("SYSTEME WINDOWS"), THIN,
    kv("OS",                    sr.windows_version || "N/A"),
    kv("Uptime",                sr.uptime_hours >= 24 ? `${(sr.uptime_hours / 24).toFixed(1)} jours` : `${sr.uptime_hours?.toFixed(1) || "?"} heures`),
    kv("Activation",            sr.windows_activation || "Inconnu"),
    kv("Type licence",          sr.license_type        || "N/A"),
    kv("Dernier KB (MAJ)",      sr.last_update_days  >= 0 ? `il y a ${sr.last_update_days} jours`  : "Inconnu"),
    kv("Reboot requis",         sr.pending_reboot ? "!!! OUI !!!" : "Non"),
    kv("MAJ en attente",        sr.pending_updates_cached >= 0 ? String(sr.pending_updates_cached) : "N/A"),
    kv("Logiciels installes",   String(sr.installed_software_count || 0)),
    kv("Services",              `${sr.services_running || 0} actifs  /  ${sr.services_stopped || 0} arretes`),
    kv("Prog. demarrage",       String(sr.startup_count || 0)),
    kv("Dossiers Temp",         `${sr.temp_folder_size_mb?.toFixed(0) || "?"} MB`),
    "");

  // --- LICENCES & CHIFFREMENT ---
  lines.push(sec("LICENCES & CHIFFREMENT"), THIN,
    kv("Cle Windows",           sr.windows_product_key || "Non disponible"),
    kv("Office",                `${sr.office_name || "N/A"}  --  ${sr.office_product_key || "N/A"}`),
  );
  if (sr.bitlocker_volumes?.length) {
    for (const bv of sr.bitlocker_volumes) {
      const prot = bv.protection_status === "On" || bv.protection_status === "1" ? "Protege" : "Non protege";
      lines.push(kv(`BitLocker ${bv.drive}`, prot));
      if (bv.recovery_password) lines.push(kv("  Cle recuperation", bv.recovery_password));
    }
  } else { lines.push(kv("BitLocker", "Aucun volume chiffre")); }
  lines.push("");

  // --- INTEGRITE WINDOWS (DISM + SFC) ---
  lines.push(sec("INTEGRITE SYSTEME"), THIN,
    kv("DISM",                  sr.dism_status || "N/A"),
    kv("SFC",                   sr.sfc_status  || "N/A"),
  );
  if (sr.sfc_details && sr.sfc_details.trim()) {
    lines.push("  Sortie SFC :");
    for (const l of sr.sfc_details.split("\n").slice(0, 15)) lines.push(`    ${l}`);
  }
  if (sr.dism_details && !sr.dism_status?.toLowerCase().includes("sain")) {
    lines.push("  Sortie DISM :");
    for (const l of sr.dism_details.split("\n").slice(0, 15)) lines.push(`    ${l}`);
  }
  lines.push("");

  // --- MISES A JOUR ---
  if (sr.winget_upgradable?.length || sr.choco_upgradable?.length) {
    lines.push(sec(`MISES A JOUR DISPONIBLES`), THIN);
    if (sr.winget_upgradable?.length) {
      lines.push(`  WinGet (${sr.winget_upgradable.length} logiciel(s)) :`);
      for (const u of sr.winget_upgradable.slice(0, 20)) {
        const name = (u.name || "").padEnd(32).substring(0, 32);
        const ver = `${u.current_version || "?"} -> ${u.available_version || "?"}`;
        lines.push(`    - ${name} ${ver}  [${u.id || ""}]`);
      }
      if (sr.winget_upgradable.length > 20) lines.push(`    ... (${sr.winget_upgradable.length - 20} autres)`);
    }
    if (sr.choco_upgradable?.length) {
      lines.push(`  Chocolatey (${sr.choco_upgradable.length}) :`);
      for (const u of sr.choco_upgradable.slice(0, 8)) lines.push(`    - ${u}`);
    }
    lines.push("");
  }

  // --- TOP PROCESSUS ---
  if (sr.top_cpu?.length || sr.top_ram?.length) {
    lines.push(sec("TOP PROCESSUS (snapshot)"), THIN);
    if (sr.top_cpu?.length) {
      lines.push("  CPU :");
      for (const p of sr.top_cpu) lines.push(`    [${String(p.pid).padEnd(6)}] ${p.name.padEnd(32)} ${p.value}s CPU`);
    }
    if (sr.top_ram?.length) {
      lines.push("  RAM :");
      for (const p of sr.top_ram) lines.push(`    [${String(p.pid).padEnd(6)}] ${p.name.padEnd(32)} ${p.value} MB`);
    }
    lines.push("");
  }

  // =========================================================
  // PARTIE 3 : PROBLEMES & SECURITE
  // =========================================================

  lines.push(sec("=== PROBLEMES & SECURITE ==="), "");

  // --- PROBLEMES DETECTES ---
  if (props.scanProblems?.length) {
    lines.push(sec(`!!! PROBLEMES DETECTES : ${props.scanProblems.length} !!!`), THIN);
    for (const p of props.scanProblems) lines.push(`  !! ${p}`);
    lines.push("");
  } else {
    lines.push(sec("BILAN"), THIN, "  Aucun probleme critique detecte.", "");
  }

  // --- SECURITE ---
  lines.push(sec("SECURITE"), THIN,
    kv("Pare-feu Windows",      sr.firewall_enabled ? "Actif" : "!!! DESACTIVE !!!"),
    kv("Windows Defender",      sr.defender_enabled  ? "Actif" : "!!! INACTIF !!!"),
    kv("Antivirus tiers",       sr.antivirus_installed || "Aucun (Defender)"),
    kv("Defs Defender",         sr.defender_definition_age_days >= 0 ? `${sr.defender_definition_age_days} jours` : "N/A"),
    kv("Dernier BSOD",          sr.last_bsod || "Aucun BSOD recent"),
    kv("Connexion Internet",    sr.network_ok ? "OK" : "!!! HORS LIGNE !!!"),
    "");

  // --- SECURITE AVANCEE ---
  lines.push(sec("SECURITE AVANCEE"), THIN,
    kv("TPM",                   sr.tpm_present ? (sr.tpm_enabled ? `Present & Actif (v${sr.tpm_version || "?"})` : "Present (desactive)") : "Absent"),
    kv("Secure Boot",           sr.secure_boot ? "Actif" : "Desactive"),
    kv("UAC",                   sr.uac_level || "Inconnu"),
    kv("RDP (Bureau a dist.)",  sr.rdp_enabled  ? "!!! Actif (risque) !!!" : "Desactive"),
    kv("SMBv1",                 sr.smbv1_enabled ? "!!! Actif (risque) !!!" : "Desactive"),
    kv("Abonnements WMI",       String(sr.wmi_subscriptions ?? 0)),
    kv("Compte Invite",         sr.guest_enabled ? "!!! Actif !!!" : "Desactive"),
    kv("Admins locaux",         sr.local_admins?.join(", ") || "N/A"),
    kv("Point restauration",    sr.last_restore_point || "N/A"),
  );
  if (sr.wmi_subscription_details?.length) {
    lines.push("  Details abonnements WMI :");
    for (const sub of sr.wmi_subscription_details) {
      lines.push(`    [${sub.consumer_type}] ${sub.name || "(sans nom)"}`);
      lines.push(`      Chemin : ${sub.path}`);
    }
  }
  lines.push("");

  // --- PROCESSUS SUSPECTS ---
  if (sr.suspicious_processes?.length) {
    lines.push(sec(`PROCESSUS HORS CHEMINS SECURISES (${sr.suspicious_processes.length})`), THIN);
    for (const p of sr.suspicious_processes) {
      lines.push(`  [PID ${p.pid}] ${p.name}`);
      lines.push(`           Raison : ${p.reason}`);
      lines.push(`           Chemin : ${p.path}`);
    }
    lines.push("");
  }

  // --- SERVICES SUSPECTS ---
  if (sr.suspicious_services?.length) {
    lines.push(sec(`SERVICES TIERS ACTIFS (${sr.suspicious_services.length})`), THIN);
    for (const s of sr.suspicious_services.slice(0, 15)) {
      lines.push(`  ${s.name}  (${s.state})`);
      lines.push(`    ${s.display_name}`);
      lines.push(`    ${s.path}`);
    }
    lines.push("");
  }

  // --- AUTORUNS ---
  if (sr.autorun_entries?.length) {
    lines.push(sec(`ENTREES AUTORUN TIERS (${sr.autorun_entries.length})`), THIN);
    for (const a of sr.autorun_entries.slice(0, 25)) {
      lines.push(`  ${a.name.padEnd(36)} [${a.location}]`);
      lines.push(`    Exec   : ${a.path}`);
      lines.push(`    RegKey : ${fullRegPath(a.location, a.name)}`);
    }
    lines.push("");
  }

  // --- TACHES SUSPECTES ---
  if (sr.susp_tasks?.length) {
    lines.push(sec(`TACHES PLANIFIEES SUSPECTES (${sr.susp_tasks_count})`), THIN);
    for (const t of sr.susp_tasks) {
      lines.push(`  ${t.name}  (${t.path})`);
      lines.push(`    Executable : ${t.exec}`);
    }
    lines.push("");
  }

  // --- ERREURS RECENTES ---
  if (sr.recent_errors?.length) {
    lines.push(sec(`ERREURS RECENTES (${sr.recent_errors.length} dans 48h)`), THIN);
    for (const e of sr.recent_errors.slice(0, 10)) {
      lines.push(`  [${(e.level || "?").padEnd(8)}] ${e.time}  --  ${e.source}`);
      const msg = (e.message || "").replace(/\r?\n/g, " ").trim();
      lines.push(`    ${msg.substring(0, 100)}${msg.length > 100 ? "..." : ""}`);
    }
    lines.push("");
  }

  // --- PIED DE PAGE ---
  lines.push(SEP, sec("FIN DU RAPPORT"), SEP);

  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const filePath = await save({ defaultPath: "scan_total.txt", filters: [{ name: "TXT", extensions: ["txt"] }] });
    if (!filePath) return;
    await invoke("save_content_to_path", { path: filePath, content: lines.join("\n") });
    useNotificationStore().success("Scan exporte (.txt)", filePath);
  } catch (e: any) { useNotificationStore().error("Erreur export", String(e)); }
}

async function exportScanHtml() {
  if (!props.scanResult) return;
  const sr = props.scanResult;
  const now = new Date().toLocaleString();
  const h = (s: any) => String(s ?? "").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;");
  const ok  = (v: boolean, t="Activé", f="DÉSACTIVÉ") => `<span class="${v?'ok':'bad'}">${v?t:f}</span>`;
  const badge = (txt: string, cls: string) => `<span class="badge badge-${cls}">${h(txt)}</span>`;
  const kv  = (label: string, val: string) => `<tr><td>${h(label)}</td><td>${val}</td></tr>`;
  const sec = (title: string, icon: string) =>
    `<h2>${icon} ${h(title)}</h2>`;
  const tbl = (head: string[], rows: string[][]) =>
    `<table><thead><tr>${head.map(c=>`<th>${h(c)}</th>`).join("")}</tr></thead><tbody>`
    + rows.map(r=>`<tr>${r.map(c=>`<td>${c}</td>`).join("")}</tr>`).join("")
    + `</tbody></table>`;

  const css = `
*{box-sizing:border-box;margin:0;padding:0}
body{font-family:'Segoe UI',sans-serif;background:#0d0d0f;color:#e0e0e8;padding:32px;max-width:1100px;margin:0 auto}
h1{font-size:24px;color:#f97316;margin-bottom:4px}
.subtitle{color:#888;font-size:13px;margin-bottom:32px}
h2{font-size:13px;text-transform:uppercase;letter-spacing:.1em;color:#f97316;border-bottom:1px solid #2e2e33;padding-bottom:5px;margin:28px 0 10px}
.part-title{font-size:16px;font-weight:700;color:#fff;margin:36px 0 4px;padding:10px 16px;background:linear-gradient(90deg,rgba(249,115,22,.2),transparent);border-left:4px solid #f97316;border-radius:0 6px 6px 0}
table{width:100%;border-collapse:collapse;font-size:12.5px;margin-bottom:14px}
th{background:#1a1a20;color:#aaa;text-align:left;padding:7px 12px;font-size:11px;text-transform:uppercase;letter-spacing:.06em}
td{padding:6px 12px;border-bottom:1px solid #1e1e24;vertical-align:top}
tr:hover td{background:#14141a}
code{font-family:monospace;background:#1a1a24;padding:1px 5px;border-radius:3px;font-size:11px;color:#e0e0e8;word-break:break-all}
pre{font-family:monospace;background:#111116;padding:8px 12px;border-radius:4px;font-size:10.5px;color:#9090a0;white-space:pre-wrap;word-break:break-all;max-height:200px;overflow-y:auto;border:1px solid #2a2a32;margin-top:4px}
.ok{color:#22c55e}.warn{color:#f59e0b}.bad{color:#ef4444}.muted{color:#666}
.badge{display:inline-block;padding:2px 7px;border-radius:10px;font-size:10.5px;font-weight:600}
.badge-success{background:rgba(34,197,94,.15);color:#22c55e}
.badge-warning{background:rgba(245,158,11,.15);color:#f59e0b}
.badge-danger{background:rgba(239,68,68,.15);color:#ef4444}
.badge-info{background:rgba(59,130,246,.15);color:#60a5fa}
.badge-neutral{background:rgba(156,163,175,.12);color:#9ca3af}
ul.problems{list-style:none;margin-bottom:14px}
ul.problems li{padding:6px 10px;border-left:3px solid #f59e0b;background:#1a1510;margin-bottom:4px;border-radius:0 4px 4px 0;font-size:13px}
.sev-critical td:first-child{border-left:3px solid #ef4444}
.sev-warning td:first-child{border-left:3px solid #f59e0b}
.sev-info td:first-child{border-left:3px solid #60a5fa}
.wmi-block{background:rgba(239,68,68,.07);border:1px solid rgba(239,68,68,.3);border-radius:6px;padding:10px 12px;margin-top:8px}
footer{margin-top:48px;color:#444;font-size:11px;text-align:center;border-top:1px solid #1e1e24;padding-top:16px}`;

  // ---- helpers ----
  const diskBar = (pct: number) => {
    const f = Math.round(pct/100*20);
    const color = pct>90?'#ef4444':pct>80?'#f59e0b':'#22c55e';
    return `<div style="display:flex;align-items:center;gap:6px"><div style="flex:1;height:6px;background:#1e1e24;border-radius:3px"><div style="width:${pct.toFixed(0)}%;height:6px;background:${color};border-radius:3px"></div></div><span style="font-size:11px;min-width:34px;text-align:right">${pct.toFixed(0)}%</span></div>`;
  };

  // ===== PARTIE 1 : COMPOSANTS DU PC =====
  const p1 = `
<div class="part-title">🖥️ Partie 1 — Composants du PC</div>

${sec("Identité Système & BIOS","🔧")}
${tbl(["Propriété","Valeur"],[
  [h("Fabricant"),          h(sr.system_manufacturer||"N/A")],
  [h("Modèle"),             h(sr.system_model||"N/A")],
  [h("N° Série"),           `<code>${h(sr.system_serial||"N/A")}</code>`],
  [h("BIOS Fabricant"),     h(sr.bios_manufacturer||"N/A")],
  [h("BIOS Version"),       `<code>${h(sr.bios_version||"N/A")}</code>`],
  [h("BIOS Date"),          h(sr.bios_date||"N/A")],
])}

${sec("Composants Matériels","🔩")}
${tbl(["Composant","Détail"],[
  [h("CPU"),    h(`${sr.cpu_name} — ${sr.cpu_cores} cœurs / ${sr.cpu_threads||"?"}T @ ${sr.cpu_frequency_ghz||"?"}GHz`)],
  [h("CPU Utilisation"), badge(sr.cpu_usage_percent?.toFixed(1)+"%", sr.cpu_usage_percent>80?"danger":sr.cpu_usage_percent>50?"warning":"success")],
  [h("CPU Température"),  sr.cpu_temperature && sr.cpu_temperature!=="N/A" ? badge(sr.cpu_temperature, parseInt(sr.cpu_temperature)>80?"danger":parseInt(sr.cpu_temperature)>65?"warning":"success") : "<span class='muted'>N/A</span>"],
  [h("RAM"),    badge(`${sr.ram_used_gb?.toFixed(1)} / ${sr.ram_total_gb?.toFixed(0)} GB (${sr.ram_usage_percent?.toFixed(0)}%)`, sr.ram_usage_percent>85?"danger":sr.ram_usage_percent>65?"warning":"success")],
  [h("Config RAM"), h(sr.ram_detail||"N/A")],
  [h("Mémoire virtuelle"), h(`${sr.virtual_memory_total_mb||"?"} MB total / ${sr.virtual_memory_available_mb||"?"} MB libre`)],
  [h("GPU"),    h(`${sr.gpu_name||"N/A"} — VRAM: ${sr.gpu_vram_mb>=1024?(sr.gpu_vram_mb/1024).toFixed(0)+"GB":sr.gpu_vram_mb+"MB"}`)],
  [h("Carte mère"), h(sr.motherboard||"N/A")],
  [h("Écrans"),  h(sr.monitors_detail||sr.screen_resolution||"N/A")],
  [h("Plan d'alim."), h(sr.power_plan||"N/A")],
])}

${sr.storage_items?.length ? sec("Stockage Physique","💾") + tbl(["Modèle","Type","Interface","Taille","Santé"],
  (sr.storage_items||[]).map((s:any)=>[h(s.model||"—"),h(s.media_type),h(s.interface_type),h(s.size_gb+" GB"),badge(s.health,s.health?.toLowerCase().includes("health")||s.health==="Sain"?"success":"warning")])
) : ""}

${sec("Espace Disque (Volumes)","💽")}
${tbl(["Lecteur","Utilisation","Libre","Total"],
  (sr.disk_usage||[]).map((d:any)=>[`<code>${h(d.drive)}</code>`, diskBar(d.used_percent), h(d.free_gb.toFixed(1)+" GB"), h(d.total_gb.toFixed(0)+" GB")])
)}

${sec("Réseau","🌐")}
${tbl(["Indicateur","Valeur"],[
  [h("Connectivité"),         ok(sr.network_ok,"OK (8.8.8.8)","Hors ligne ⚠")],
  [h("Adaptateurs actifs"),   h(sr.network_adapters_summary||"N/A")],
  [h("Ports en écoute"),      sr.open_ports?.length ? `<code>${h(sr.open_ports.slice(0,30).join(", "))}${sr.open_ports.length>30?" …":""}</code>` : "<span class='ok'>Aucun</span>"],
])}`;

  // ===== PARTIE 2 : INFORMATIONS WINDOWS =====
  const bitlockerRows = (sr.bitlocker_volumes||[]).map((bv:any)=>{
    const prot = bv.protection_status==="On"||bv.protection_status==="1";
    return [h(bv.drive), badge(prot?"Protégé":"Non protégé",prot?"success":"warning"),
      bv.recovery_password?`<code style="font-size:10px">${h(bv.recovery_password)}</code>`:"<span class='muted'>—</span>"];
  });

  const wingetRows = (sr.winget_upgradable||[]).map((u:any)=>[
    h(u.name), `<code>${h(u.id)}</code>`,
    badge(u.current_version||"?","warning"),
    `<span class="ok">${h(u.available_version||"?")}</span>`,
  ]);

  const p2 = `
<div class="part-title">🪟 Partie 2 — Informations Windows</div>

${sec("Système Windows","🪟")}
${tbl(["Indicateur","Valeur"],[
  [h("Version OS"),           h(sr.windows_version||"N/A")],
  [h("Uptime"),               h(sr.uptime_hours>=24?(sr.uptime_hours/24).toFixed(1)+" jours":sr.uptime_hours?.toFixed(1)+" heures")],
  [h("Activation"),           badge(sr.windows_activation||"Inconnu", sr.windows_activation==="Activé"||sr.windows_activation==="Licencié"?"success":"danger")],
  [h("Type licence"),         badge(sr.license_type||"N/A","info")],
  [h("Dernier KB"),           badge(sr.last_update_days>=0?"il y a "+sr.last_update_days+" jours":"N/A", sr.last_update_days<0?"neutral":sr.last_update_days<=30?"success":sr.last_update_days<=60?"warning":"danger")],
  [h("Redémarrage requis"),   badge(sr.pending_reboot?"Oui ⚠":"Non", sr.pending_reboot?"warning":"success")],
  [h("MAJ en attente (cache)"), badge(String(sr.pending_updates_cached>=0?sr.pending_updates_cached:"N/A"), sr.pending_updates_cached>10?"danger":sr.pending_updates_cached>0?"warning":"success")],
  [h("Logiciels installés"),  h(String(sr.installed_software_count||0))],
  [h("Services"),             h(`${sr.services_running||0} actifs / ${sr.services_stopped||0} arrêtés`)],
  [h("Prog. démarrage"),      h(String(sr.startup_count||0))],
  [h("Fichiers TEMP"),        badge(sr.temp_folder_size_mb>=1024?(sr.temp_folder_size_mb/1024).toFixed(1)+"GB":sr.temp_folder_size_mb?.toFixed(0)+"MB", sr.temp_folder_size_mb>2048?"danger":sr.temp_folder_size_mb>512?"warning":"success")],
  [h("Dernier BSOD"),         h(sr.last_bsod||"Aucun BSOD récent")],
])}

${sec("Licences & Chiffrement","🔑")}
${tbl(["Propriété","Valeur"],[
  [h("Clé Windows"),          sr.windows_product_key?`<code>${h(sr.windows_product_key)}</code>`:"<span class='muted'>Non disponible</span>"],
  [h(sr.office_name||"Office"), sr.office_product_key?`<code>${h(sr.office_product_key)}</code>`:"<span class='muted'>N/A</span>"],
])}
${bitlockerRows.length ? `<h3 style="font-size:11px;color:#aaa;margin:8px 0 4px;text-transform:uppercase">BitLocker</h3>`+tbl(["Volume","Protection","Clé de récupération"],bitlockerRows) : "<p class='muted' style='font-size:12px;margin:4px 0 12px'>BitLocker non configuré</p>"}

${sec("Intégrité Système (DISM / SFC)","🛡️")}
${tbl(["Outil","Statut"],[
  [h("DISM"), badge(sr.dism_status||"N/A", sr.dism_status?.toLowerCase().includes("sain")?"success":"danger")],
  [h("SFC"),  badge(sr.sfc_status||"N/A",  sr.sfc_status?.toLowerCase().includes("intèg")||sr.sfc_status?.toLowerCase().includes("integ")?"success":"danger")],
])}
${sr.sfc_details ? `<pre>${h(sr.sfc_details)}</pre>` : ""}
${sr.dism_details && !sr.dism_status?.toLowerCase().includes("sain") ? `<pre>${h(sr.dism_details)}</pre>` : ""}

${wingetRows.length ? sec(`Mises à Jour Disponibles (${wingetRows.length} logiciel(s))`, "📦") + tbl(["Nom","ID","Version actuelle","Disponible"], wingetRows) : ""}

${(sr.top_cpu?.length||sr.top_ram?.length) ? sec("Top 5 Processus (snapshot)","📊") + `<div style="display:grid;grid-template-columns:1fr 1fr;gap:16px">`
  + (sr.top_cpu?.length ? tbl(["PID","Processus","CPU (s)"], sr.top_cpu.map((p:any)=>[h(String(p.pid)),h(p.name),`<code>${h(String(p.value))}</code>`])) : "")
  + (sr.top_ram?.length ? tbl(["PID","Processus","RAM (MB)"], sr.top_ram.map((p:any)=>[h(String(p.pid)),h(p.name),`<code>${h(String(p.value))}</code>`])) : "")
  + `</div>` : ""}`;

  // ===== PARTIE 3 : PROBLEMES & SECURITE =====
  const wmiBlock = sr.wmi_subscriptions>0 ? `
<div class="wmi-block">
  <p style="color:#ef4444;font-weight:600;margin-bottom:8px">⚠ ${sr.wmi_subscriptions} abonnement(s) WMI suspects détectés</p>
  ${tbl(["Nom","Type WMI","Chemin"],[...(sr.wmi_subscription_details||[]).map((s:any)=>[h(s.name||"(sans nom)"),badge(s.consumer_type,"danger"),`<code style="font-size:10px">${h(s.path)}</code>`])])}
  <p style="font-size:11px;color:#aaa;margin-top:6px">Commande de suppression : <code>Get-WmiObject -Namespace root\\subscription -Class __EventConsumer | Remove-WmiObject</code></p>
</div>` : "";

  const processRows = (sr.suspicious_processes||[]).map((p:any)=>
    [h(p.name), h(String(p.pid)), badge(p.reason,"warning"), `<code style="font-size:10px">${h(p.path)}</code>`]
  );
  const serviceRows = (sr.suspicious_services||[]).slice(0,20).map((s:any)=>
    [h(s.name), h(s.display_name), badge(s.state,"info"), `<code style="font-size:10px">${h(s.path)}</code>`]
  );
  const autorunRows = (sr.autorun_entries||[]).slice(0,30).map((a:any)=>[
    h(a.name),
    `<code style="font-size:10px">${h(fullRegPath(a.location, a.name))}</code>`,
    `<code style="font-size:10px">${h(a.path)}</code>`,
  ]);
  const taskRows = (sr.susp_tasks||[]).map((t:any)=>
    [h(t.name), badge(t.path,"neutral"), `<code style="font-size:10px">${h(t.exec)}</code>`]
  );
  const errorRows = (sr.recent_errors||[]).slice(0,20).map((e:any)=>[
    `<code>${h(e.time)}</code>`,
    badge(e.level, e.level?.toLowerCase().includes("crit")?"danger":"warning"),
    h(e.source),
    `<span style="font-size:11px">${h((e.message||"").substring(0,120))}</span>`,
  ]);

  const p3 = `
<div class="part-title">🚨 Partie 3 — Problèmes &amp; Sécurité</div>

${props.scanProblems.length
  ? `${sec(`Problèmes Détectés (${props.scanProblems.length})`, "⚠")}<ul class="problems">${props.scanProblems.map(p=>`<li>⚠ ${h(p)}</li>`).join("")}</ul>`
  : `<p style="color:#22c55e;padding:10px 0;font-size:13px">✅ Aucun problème critique détecté</p>`}

${scanSolutions.value.length ? sec("Solutions Recommandées","💡") + tbl(["Problème","Action","Sévérité"],
  scanSolutions.value.map(s=>[h(s.problem), h(s.action),
    badge(s.severity==="critical"?"🔴 Critique":s.severity==="warning"?"🟡 Attention":"🔵 Info",
          s.severity==="critical"?"danger":s.severity==="warning"?"warning":"info")])
) : ""}

${sec("Sécurité","🔒")}
${tbl(["Indicateur","Valeur"],[
  [h("Pare-feu Windows"),     ok(sr.firewall_enabled)],
  [h("Windows Defender"),     ok(sr.defender_enabled,"Actif","INACTIF ⚠")],
  [h("Antivirus tiers"),      h(sr.antivirus_installed||"Aucun (Defender)")],
  [h("Defs Defender"),        badge(sr.defender_definition_age_days>=0?sr.defender_definition_age_days+" j":"N/A", sr.defender_definition_age_days<0?"neutral":sr.defender_definition_age_days<=3?"success":sr.defender_definition_age_days<=7?"warning":"danger")],
  [h("Connectivité Internet"), ok(sr.network_ok,"OK","Hors ligne ⚠")],
  [h("Dernier BSOD"),         h(sr.last_bsod||"Aucun")],
])}

${sec("Sécurité Avancée","🛡️")}
${tbl(["Indicateur","Valeur"],[
  [h("TPM"),          badge(sr.tpm_present?(sr.tpm_enabled?`Présent & Actif (v${sr.tpm_version||"?"})`:  "Présent (désactivé)"):"Absent", sr.tpm_present&&sr.tpm_enabled?"success":"warning")],
  [h("Secure Boot"),  badge(sr.secure_boot?"Activé":"Désactivé", sr.secure_boot?"success":"warning")],
  [h("Niveau UAC"),   h(sr.uac_level||"Inconnu")],
  [h("RDP"),          badge(sr.rdp_enabled?"Activé ⚠":"Désactivé", sr.rdp_enabled?"warning":"success")],
  [h("SMBv1"),        badge(sr.smbv1_enabled?"Activé ⚠":"Désactivé", sr.smbv1_enabled?"danger":"success")],
  [h("Abonnements WMI"), badge(String(sr.wmi_subscriptions??0), sr.wmi_subscriptions>0?"danger":"success")],
  [h("Compte Invité"), badge(sr.guest_enabled?"Activé ⚠":"Désactivé", sr.guest_enabled?"warning":"success")],
  [h("Administrateurs locaux"), h(sr.local_admins?.join(", ")||"N/A")],
  [h("Dernier point de restauration"), badge(sr.last_restore_point||"N/A", sr.last_restore_point?.includes("Aucun")?"warning":"success")],
])}
${wmiBlock}

${processRows.length ? sec(`Processus Hors Chemins Sécurisés (${processRows.length})`,"🔍") + tbl(["Nom","PID","Raison","Chemin"],processRows) : ""}
${serviceRows.length ? sec(`Services Tiers Actifs (${serviceRows.length})`,"⚙️") + tbl(["Nom","Affichage","État","Chemin"],serviceRows) : ""}
${autorunRows.length ? sec(`Entrées Autorun Tiers (${autorunRows.length})`,"🚀") + tbl(["Nom","Clé Registre","Exécutable"],autorunRows) : ""}
${taskRows.length ? sec(`Tâches Planifiées Suspectes (${taskRows.length})`,"📅") + tbl(["Tâche","Chemin","Exécutable"],taskRows) : ""}
${errorRows.length ? sec(`Erreurs Récentes (${errorRows.length} — 48h)`,"🔴") + tbl(["Heure","Niveau","Source","Message"],errorRows) : ""}`;

  const html = `<!DOCTYPE html>
<html lang="fr">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Rapport Scan Total — NiTriTe</title>
<style>${css}</style>
</head>
<body>
<h1>🔍 Rapport Scan Total — NiTriTe</h1>
<p class="subtitle">Généré le ${now}</p>
${p1}
${p2}
${p3}
<footer>Rapport complet généré par <strong>NiTriTe v26.30.0</strong> — ${now}</footer>
</body>
</html>`;

  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const filePath = await save({ defaultPath: "scan_total.html", filters: [{ name: "HTML", extensions: ["html"] }] });
    if (!filePath) return;
    await invoke("save_content_to_path", { path: filePath, content: html });
    useNotificationStore().success("Scan exporté (HTML)", filePath);
  } catch (e: any) { useNotificationStore().error("Erreur export", String(e)); }
}

async function exportScanMd() {
  if (!props.scanResult) return;
  const sr = props.scanResult;
  const now = new Date().toLocaleString();
  const e = (s: any) => String(s ?? "").replace(/\|/g, "\\|");
  const row = (...cells: string[]) => `| ${cells.map(e).join(" | ")} |`;
  const head = (...cols: string[]) => [row(...cols), `|${cols.map(()=>"---").join("|")}|`];

  const L: string[] = [
    `# 🔍 Rapport Scan Total — NiTriTe`,
    ``,
    `> Généré le **${now}**`,
    ``,
    `---`,
    ``,
  ];

  // ============================================================
  // PARTIE 1 — COMPOSANTS DU PC
  // ============================================================
  L.push(`## 🖥️ Partie 1 — Composants du PC`, ``);

  L.push(`### 🔧 Identité Système & BIOS`, ``);
  L.push(...head("Propriété","Valeur"),
    row("Fabricant",         sr.system_manufacturer||"N/A"),
    row("Modèle",            sr.system_model||"N/A"),
    row("N° Série",          sr.system_serial||"N/A"),
    row("BIOS Fabricant",    sr.bios_manufacturer||"N/A"),
    row("BIOS Version",      sr.bios_version||"N/A"),
    row("BIOS Date",         sr.bios_date||"N/A"),
    ``);

  L.push(`### 🔩 Composants Matériels`, ``);
  L.push(...head("Composant","Détail"),
    row("CPU",               `${sr.cpu_name} — ${sr.cpu_cores} cœurs / ${sr.cpu_threads||"?"}T @ ${sr.cpu_frequency_ghz||"?"}GHz`),
    row("CPU Utilisation",   `${sr.cpu_usage_percent?.toFixed(1)||"?"}%`),
    row("CPU Température",   sr.cpu_temperature||"N/A"),
    row("RAM",               `${sr.ram_used_gb?.toFixed(1)} / ${sr.ram_total_gb?.toFixed(0)} GB (${sr.ram_usage_percent?.toFixed(0)}%)`),
    row("Config RAM",        sr.ram_detail||"N/A"),
    row("Mémoire virtuelle", `${sr.virtual_memory_total_mb||"?"} MB total / ${sr.virtual_memory_available_mb||"?"} MB libre`),
    row("GPU",               `${sr.gpu_name||"N/A"} — VRAM: ${sr.gpu_vram_mb>=1024?(sr.gpu_vram_mb/1024).toFixed(0)+"GB":sr.gpu_vram_mb+"MB"}`),
    row("Carte mère",        sr.motherboard||"N/A"),
    row("Écrans",            sr.monitors_detail||sr.screen_resolution||"N/A"),
    row("Plan d'alim.",      sr.power_plan||"N/A"),
    ``);

  if (sr.storage_items?.length) {
    L.push(`### 💾 Stockage Physique`, ``);
    L.push(...head("Modèle","Type","Interface","Taille","Santé"));
    for (const s of sr.storage_items) L.push(row(s.model||"—", s.media_type, s.interface_type, s.size_gb+" GB", s.health));
    L.push(``);
  }

  L.push(`### 💽 Espace Disque (Volumes)`, ``);
  L.push(...head("Lecteur","Utilisé","Libre","Total"));
  for (const d of sr.disk_usage||[]) L.push(row(d.drive, d.used_percent.toFixed(0)+"%", d.free_gb.toFixed(1)+" GB", d.total_gb.toFixed(0)+" GB"));
  L.push(``);

  L.push(`### 🌐 Réseau`, ``);
  L.push(...head("Indicateur","Valeur"),
    row("Connectivité",       sr.network_ok?"✅ OK (8.8.8.8)":"❌ Hors ligne"),
    row("Adaptateurs actifs", sr.network_adapters_summary||"N/A"),
    row("Ports en écoute",    sr.open_ports?.length ? sr.open_ports.slice(0,30).join(", ")+(sr.open_ports.length>30?" …":"") : "Aucun"),
    ``);

  // ============================================================
  // PARTIE 2 — INFORMATIONS WINDOWS
  // ============================================================
  L.push(`---`, ``, `## 🪟 Partie 2 — Informations Windows`, ``);

  L.push(`### 🪟 Système Windows`, ``);
  L.push(...head("Indicateur","Valeur"),
    row("Version OS",               sr.windows_version||"N/A"),
    row("Uptime",                   sr.uptime_hours>=24?(sr.uptime_hours/24).toFixed(1)+" jours":sr.uptime_hours?.toFixed(1)+" heures"),
    row("Activation",               sr.windows_activation||"Inconnu"),
    row("Type licence",             sr.license_type||"N/A"),
    row("Dernier KB",               sr.last_update_days>=0?"il y a "+sr.last_update_days+" jours":"N/A"),
    row("Redémarrage requis",       sr.pending_reboot?"⚠ Oui":"Non"),
    row("MAJ en attente (cache)",   sr.pending_updates_cached>=0?String(sr.pending_updates_cached):"N/A"),
    row("Logiciels installés",      String(sr.installed_software_count||0)),
    row("Services",                 `${sr.services_running||0} actifs / ${sr.services_stopped||0} arrêtés`),
    row("Prog. démarrage",          String(sr.startup_count||0)),
    row("Fichiers TEMP",            sr.temp_folder_size_mb>=1024?(sr.temp_folder_size_mb/1024).toFixed(1)+"GB":sr.temp_folder_size_mb?.toFixed(0)+" MB"),
    row("Dernier BSOD",             sr.last_bsod||"Aucun BSOD récent"),
    ``);

  L.push(`### 🔑 Licences & Chiffrement`, ``);
  L.push(...head("Propriété","Valeur"),
    row("Clé Windows",              sr.windows_product_key ? `\`${sr.windows_product_key}\`` : "Non disponible"),
    row(sr.office_name||"Office",   sr.office_product_key ? `\`${sr.office_product_key}\`` : "N/A"),
  );
  if (sr.bitlocker_volumes?.length) {
    for (const bv of sr.bitlocker_volumes) {
      const prot = bv.protection_status==="On"||bv.protection_status==="1";
      L.push(row(`BitLocker ${bv.drive}`, prot?"🔒 Protégé":"🔓 Non protégé"));
      if (bv.recovery_password) L.push(row("  Clé récupération", `\`${bv.recovery_password}\``));
    }
  } else { L.push(row("BitLocker","Non configuré")); }
  L.push(``);

  L.push(`### 🛡️ Intégrité Système (DISM / SFC)`, ``);
  L.push(...head("Outil","Statut"),
    row("DISM", sr.dism_status||"N/A"),
    row("SFC",  sr.sfc_status||"N/A"),
    ``);
  if (sr.sfc_details?.trim()) {
    L.push(`**Sortie SFC :**`, ``, "```", sr.sfc_details.split("\n").slice(0,20).join("\n"), "```", ``);
  }
  if (sr.dism_details && !sr.dism_status?.toLowerCase().includes("sain")) {
    L.push(`**Sortie DISM :**`, ``, "```", sr.dism_details.split("\n").slice(0,20).join("\n"), "```", ``);
  }

  if (sr.winget_upgradable?.length) {
    L.push(`### 📦 Mises à Jour Disponibles (${sr.winget_upgradable.length} logiciel(s))`, ``);
    L.push(...head("Nom","ID","Version actuelle","Disponible"));
    for (const u of sr.winget_upgradable) L.push(row(u.name, u.id, u.current_version||"?", u.available_version||"?"));
    L.push(``);
  }
  if (sr.choco_upgradable?.length) {
    L.push(`### 🍫 Mises à Jour Chocolatey (${sr.choco_upgradable.length})`, ``);
    for (const u of sr.choco_upgradable) L.push(`- ${u}`);
    L.push(``);
  }

  if (sr.top_cpu?.length || sr.top_ram?.length) {
    L.push(`### 📊 Top 5 Processus (snapshot)`, ``);
    if (sr.top_cpu?.length) {
      L.push(`**CPU :**`, ``,...head("PID","Processus","CPU (s)"));
      for (const p of sr.top_cpu) L.push(row(String(p.pid), p.name, String(p.value)));
      L.push(``);
    }
    if (sr.top_ram?.length) {
      L.push(`**RAM :**`, ``,...head("PID","Processus","RAM (MB)"));
      for (const p of sr.top_ram) L.push(row(String(p.pid), p.name, String(p.value)));
      L.push(``);
    }
  }

  // ============================================================
  // PARTIE 3 — PROBLEMES & SECURITE
  // ============================================================
  L.push(`---`, ``, `## 🚨 Partie 3 — Problèmes & Sécurité`, ``);

  if (props.scanProblems.length) {
    L.push(`### ⚠ Problèmes Détectés (${props.scanProblems.length})`, ``);
    for (const p of props.scanProblems) L.push(`- ⚠ ${p}`);
    L.push(``);
  } else {
    L.push(`> ✅ **Aucun problème critique détecté**`, ``);
  }

  if (scanSolutions.value.length) {
    L.push(`### 💡 Solutions Recommandées`, ``);
    L.push(...head("Problème","Action","Sévérité"));
    for (const s of scanSolutions.value) {
      const sev = s.severity==="critical"?"🔴 Critique":s.severity==="warning"?"🟡 Attention":"🔵 Info";
      L.push(row(s.problem, s.action, sev));
    }
    L.push(``);
  }

  L.push(`### 🔒 Sécurité`, ``);
  L.push(...head("Indicateur","Valeur"),
    row("Pare-feu Windows",   sr.firewall_enabled?"✅ Activé":"❌ DÉSACTIVÉ"),
    row("Windows Defender",   sr.defender_enabled?"✅ Actif":"❌ INACTIF"),
    row("Antivirus tiers",    sr.antivirus_installed||"Aucun (Defender)"),
    row("Defs Defender",      sr.defender_definition_age_days>=0?sr.defender_definition_age_days+" jours":"N/A"),
    row("Connectivité",       sr.network_ok?"✅ OK":"❌ Hors ligne"),
    row("Dernier BSOD",       sr.last_bsod||"Aucun"),
    ``);

  L.push(`### 🛡️ Sécurité Avancée`, ``);
  L.push(...head("Indicateur","Valeur"),
    row("TPM",                sr.tpm_present?(sr.tpm_enabled?`✅ Présent & Actif (v${sr.tpm_version||"?"})`:"⚠ Présent (désactivé)"):"❌ Absent"),
    row("Secure Boot",        sr.secure_boot?"✅ Activé":"⚠ Désactivé"),
    row("Niveau UAC",         sr.uac_level||"Inconnu"),
    row("RDP",                sr.rdp_enabled?"⚠ Activé":"✅ Désactivé"),
    row("SMBv1",              sr.smbv1_enabled?"❌ Activé (risque)":"✅ Désactivé"),
    row("Abonnements WMI",    String(sr.wmi_subscriptions??0)),
    row("Compte Invité",      sr.guest_enabled?"⚠ Activé":"✅ Désactivé"),
    row("Admins locaux",      sr.local_admins?.join(", ")||"N/A"),
    row("Dernier pt. restauration", sr.last_restore_point||"N/A"),
    ``);

  if (sr.wmi_subscriptions > 0) {
    L.push(`> ⚠ **${sr.wmi_subscriptions} abonnement(s) WMI suspects** — indicateur de malware potentiel`, ``);
    if (sr.wmi_subscription_details?.length) {
      L.push(...head("Nom","Type","Chemin"));
      for (const s of sr.wmi_subscription_details) L.push(row(s.name||"(sans nom)", s.consumer_type, s.path));
      L.push(``);
    }
    L.push(`**Commande suppression :** \`Get-WmiObject -Namespace root\\subscription -Class __EventConsumer | Remove-WmiObject\``, ``);
  }

  if (sr.suspicious_processes?.length) {
    L.push(`### 🔍 Processus Hors Chemins Sécurisés (${sr.suspicious_processes.length})`, ``);
    L.push(...head("PID","Nom","Raison","Chemin"));
    for (const p of sr.suspicious_processes) L.push(row(String(p.pid), p.name, p.reason, p.path));
    L.push(``);
  }

  if (sr.suspicious_services?.length) {
    L.push(`### ⚙️ Services Tiers Actifs (${sr.suspicious_services.length})`, ``);
    L.push(...head("Nom","Affichage","État","Chemin"));
    for (const s of (sr.suspicious_services||[]).slice(0,20)) L.push(row(s.name, s.display_name, s.state, s.path));
    L.push(``);
  }

  if (sr.autorun_entries?.length) {
    L.push(`### 🚀 Entrées Autorun Tiers (${sr.autorun_entries.length})`, ``);
    L.push(...head("Nom","Clé Registre","Exécutable"));
    for (const a of (sr.autorun_entries||[]).slice(0,30)) L.push(row(a.name, fullRegPath(a.location, a.name), a.path));
    L.push(``);
  }

  if (sr.susp_tasks?.length) {
    L.push(`### 📅 Tâches Planifiées Suspectes (${sr.susp_tasks_count})`, ``);
    L.push(...head("Tâche","Chemin","Exécutable"));
    for (const t of sr.susp_tasks) L.push(row(t.name, t.path, t.exec));
    L.push(``);
  }

  if (sr.recent_errors?.length) {
    L.push(`### 🔴 Erreurs Récentes (${sr.recent_errors.length} — 48h)`, ``);
    L.push(...head("Heure","Niveau","Source","Message"));
    for (const ev of (sr.recent_errors||[]).slice(0,20))
      L.push(row(ev.time, ev.level||"?", ev.source, (ev.message||"").substring(0,120)));
    L.push(``);
  }

  L.push(`---`, `*Rapport complet généré par **NiTriTe v26.30.0***`);

  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const filePath = await save({ defaultPath: "scan_total.md", filters: [{ name: "Markdown", extensions: ["md"] }] });
    if (!filePath) return;
    await invoke("save_content_to_path", { path: filePath, content: L.join("\n") });
    useNotificationStore().success("Scan exporté (Markdown)", filePath);
  } catch (e: any) { useNotificationStore().error("Erreur export", String(e)); }
}

function fullRegPath(location: string, name?: string): string {
  let p = location
    .replace(/^HKCU(\\|$)/, "HKEY_CURRENT_USER$1")
    .replace(/^HKLM(\\|$)/, "HKEY_LOCAL_MACHINE$1")
    .replace(/^HKCR(\\|$)/, "HKEY_CLASSES_ROOT$1")
    .replace(/^HKU(\\|$)/, "HKEY_USERS$1");
  if (name) p = p + (p.endsWith("\\") ? "" : "\\") + name;
  return p;
}

async function copyRegPath(path: string) {
  try {
    await navigator.clipboard.writeText(path);
    useNotificationStore().success("Chemin copié", path.slice(0, 60) + (path.length > 60 ? "…" : ""));
  } catch { useNotificationStore().error("Copie échouée"); }
}

async function openRegedit(location: string) {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("open_in_regedit", { keyPath: location });
  } catch (e: any) { useNotificationStore().error("Impossible d'ouvrir Regedit", String(e)); }
}

async function runRepairCommand(type: "sfc" | "dism") {
  const notif = useNotificationStore();
  try {
    if (type === "sfc") {
      await invoke("run_system_command", { cmd: "cmd", args: ["/c", "start", "cmd", "/k", "sfc /scannow"] });
      notif.success("SFC lancé", "Une fenêtre cmd s'est ouverte avec sfc /scannow");
    } else {
      await invoke("run_system_command", { cmd: "cmd", args: ["/c", "start", "cmd", "/k", "DISM /Online /Cleanup-Image /RestoreHealth"] });
      notif.success("DISM RestoreHealth lancé", "Réparation des composants Windows en cours");
    }
  } catch (e: any) {
    notif.error("Erreur lancement commande", String(e));
  }
}

async function exportScanJson() {
  if (!props.scanResult) return;
  try {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const filePath = await save({ defaultPath: "scan_total.json", filters: [{ name: "JSON", extensions: ["json"] }] });
    if (!filePath) return;
    await invoke("save_content_to_path", {
      path: filePath,
      content: JSON.stringify({ generated: new Date().toISOString(), problems: props.scanProblems, solutions: scanSolutions.value, scan: props.scanResult }, null, 2)
    });
    useNotificationStore().success("Scan exporté (.json)", filePath);
  } catch (e: any) { useNotificationStore().error("Erreur export", String(e)); }
}
</script>

<template>
  <div class="diag-tab-content">
    <DiagBanner :icon="ScanLine" title="Scan Complet du Système" desc="Analyse approfondie : sécurité, performances, licences et intégrité" color="emerald" />

    <!-- En cours -->
    <div v-if="scanning" class="scan-progress" style="display:flex;flex-direction:column;gap:10px">
      <div style="display:flex;align-items:center;gap:8px;font-size:13px;color:var(--text-secondary)">
        <NSpinner :size="16" /><span>{{ scanStep }}</span>
      </div>
      <NProgress :value="scanProgress" showLabel size="lg" />
      <p class="muted" style="font-size:12px">Analyse complète en cours — cela peut prendre 1 à 3 minutes...</p>
    </div>

    <!-- Aucun scan -->
    <div v-else-if="!scanResult">
      <p class="muted" style="margin-bottom:16px;font-size:13px;line-height:1.6">
        Le scan complet analyse : système, CPU/RAM/GPU/disques/carte mère, sécurité,
        licences Windows & Office en clair, clés BitLocker, processus suspects,
        réseau, DISM/SFC, mises à jour, batterie complète, journaux d'erreurs et antivirus.
      </p>
      <NButton variant="primary" @click="onRunScan"><ScanLine :size="14" /> Lancer le Scan Complet</NButton>
    </div>

    <!-- Résultats -->
    <div v-else style="display:flex;flex-direction:column;gap:14px">

      <!-- ===== BILAN ===== -->
      <div class="diag-section" :style="{borderLeft: `3px solid ${scanProblems.length ? 'var(--warning)' : 'var(--success)'}`}">
        <p class="diag-section-label" style="margin:0 0 8px 0">
          {{ scanProblems.length ? `⚠ ${scanProblems.length} problème(s) détecté(s)` : '✓ Aucun problème critique' }}
        </p>
        <div v-if="!scanProblems.length" style="color:var(--success);font-size:13px">Système en bonne santé</div>
        <div v-for="(p, i) in scanProblems" :key="i"
          style="display:flex;align-items:center;gap:8px;padding:5px 0;border-bottom:1px solid var(--border);font-size:13px">
          <AlertTriangle :size="13" class="ic-warn" />{{ p }}
        </div>
      </div>

      <!-- ===== SOLUTIONS RECOMMANDÉES ===== -->
      <div v-if="scanSolutions.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 10px 0">
          <Wrench :size="13" style="display:inline;margin-right:6px;color:var(--accent-primary)" />
          Solutions Recommandées ({{ scanSolutions.length }})
        </p>
        <div v-for="(s, i) in scanSolutions" :key="i" class="solution-row"
          :class="`solution-${s.severity}`">
          <div class="solution-sev">
            <span v-if="s.severity === 'critical'" style="color:var(--danger)">🔴</span>
            <span v-else-if="s.severity === 'warning'" style="color:var(--warning)">🟡</span>
            <span v-else style="color:var(--info)">🔵</span>
          </div>
          <div style="flex:1;min-width:0">
            <div style="font-size:13px;font-weight:500;color:var(--text-primary)">{{ s.problem }}</div>
            <div style="font-size:12px;color:var(--text-secondary);margin-top:2px">→ {{ s.action }}</div>
          </div>
          <NBadge v-if="s.severity === 'critical'" variant="danger" style="flex-shrink:0">Critique</NBadge>
          <NBadge v-else-if="s.severity === 'warning'" variant="warning" style="flex-shrink:0">Attention</NBadge>
          <NBadge v-else variant="info" style="flex-shrink:0">Info</NBadge>
        </div>
      </div>

      <!-- ===== SYSTÈME GÉNÉRAL ===== -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Résumé Système</p>
        <div class="info-grid">
          <div class="info-row"><span>Windows</span><span>{{ scanResult.windows_version }}</span></div>
          <div class="info-row"><span>Activation</span>
            <NBadge :variant="scanResult.windows_activation === 'Activé' || scanResult.windows_activation === 'Licencié' ? 'success' : 'danger'">
              {{ scanResult.windows_activation || "Inconnu" }}
            </NBadge>
          </div>
          <div class="info-row"><span>Uptime</span>
            <span>{{ scanResult.uptime_hours >= 24 ? `${(scanResult.uptime_hours/24).toFixed(1)} j` : `${scanResult.uptime_hours.toFixed(1)} h` }}</span>
          </div>
          <div class="info-row"><span>Redémarrage requis</span>
            <NBadge :variant="scanResult.pending_reboot ? 'warning' : 'success'">{{ scanResult.pending_reboot ? "Oui" : "Non" }}</NBadge>
          </div>
          <div class="info-row"><span>Démarrage auto</span><span>{{ scanResult.startup_count }} programmes</span></div>
          <div class="info-row"><span>Fichiers %TEMP%</span>
            <NBadge :variant="scanResult.temp_folder_size_mb > 2048 ? 'danger' : scanResult.temp_folder_size_mb > 512 ? 'warning' : 'success'">
              {{ scanResult.temp_folder_size_mb >= 1024 ? (scanResult.temp_folder_size_mb/1024).toFixed(1)+' GB' : scanResult.temp_folder_size_mb.toFixed(0)+' MB' }}
            </NBadge>
          </div>
          <div class="info-row"><span>Logiciels installés</span><span>{{ scanResult.installed_software_count }}</span></div>
          <div class="info-row"><span>Services actifs / arrêtés</span>
            <span><span style="color:var(--success)">{{ scanResult.services_running }}</span> / <span class="muted">{{ scanResult.services_stopped }}</span></span>
          </div>
          <div class="info-row"><span>Mém. virtuelle</span>
            <span>{{ scanResult.virtual_memory_available_mb > 0 ? (scanResult.virtual_memory_available_mb/1024).toFixed(1)+'GB libres / '+(scanResult.virtual_memory_total_mb/1024).toFixed(1)+'GB' : 'N/A' }}</span>
          </div>
          <div class="info-row" v-if="scanResult.plan_alimentation || scanResult.power_plan">
            <span>Plan d'alimentation</span><span>{{ scanResult.power_plan }}</span>
          </div>
        </div>
      </div>

      <!-- ===== COMPOSANTS MATÉRIELS ===== -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Composants matériels</p>
        <div class="info-grid">
          <!-- CPU -->
          <div class="info-row">
            <span style="display:flex;align-items:center;gap:4px"><Cpu :size="12" /> Processeur</span>
            <span>{{ scanResult.cpu_name }}</span>
          </div>
          <div class="info-row"><span>Cœurs / Threads</span>
            <span>{{ scanResult.cpu_cores }} cœurs / {{ scanResult.cpu_threads > 0 ? scanResult.cpu_threads : '—' }} threads</span>
          </div>
          <div class="info-row"><span>Fréquence max</span>
            <span>{{ scanResult.cpu_frequency_ghz > 0 ? scanResult.cpu_frequency_ghz + ' GHz' : '—' }}</span>
          </div>
          <div class="info-row"><span>Utilisation CPU</span>
            <NBadge :variant="scanResult.cpu_usage_percent > 80 ? 'danger' : scanResult.cpu_usage_percent > 50 ? 'warning' : 'success'">
              {{ scanResult.cpu_usage_percent.toFixed(1) }}%
            </NBadge>
          </div>
          <div v-if="scanResult.cpu_temperature && scanResult.cpu_temperature !== 'N/A'" class="info-row">
            <span>Température CPU</span>
            <NBadge :variant="parseInt(scanResult.cpu_temperature) > 80 ? 'danger' : parseInt(scanResult.cpu_temperature) > 65 ? 'warning' : 'success'">
              {{ scanResult.cpu_temperature }}
            </NBadge>
          </div>
          <!-- RAM -->
          <div class="info-row">
            <span style="display:flex;align-items:center;gap:4px"><MemoryStick :size="12" /> RAM utilisée</span>
            <NBadge :variant="scanResult.ram_usage_percent > 85 ? 'danger' : scanResult.ram_usage_percent > 65 ? 'warning' : 'success'">
              {{ scanResult.ram_used_gb.toFixed(1) }} / {{ scanResult.ram_total_gb.toFixed(0) }} GB ({{ scanResult.ram_usage_percent.toFixed(0) }}%)
            </NBadge>
          </div>
          <div v-if="scanResult.ram_detail" class="info-row"><span>Configuration RAM</span><span>{{ scanResult.ram_detail }}</span></div>
          <!-- GPU -->
          <div v-if="scanResult.gpu_name" class="info-row">
            <span style="display:flex;align-items:center;gap:4px"><Monitor :size="12" /> GPU</span>
            <span>{{ scanResult.gpu_name }}</span>
          </div>
          <div v-if="scanResult.gpu_vram_mb > 0" class="info-row">
            <span>VRAM GPU</span>
            <span>{{ scanResult.gpu_vram_mb >= 1024 ? (scanResult.gpu_vram_mb/1024).toFixed(1)+' GB' : scanResult.gpu_vram_mb+' MB' }}</span>
          </div>
          <!-- Carte mère -->
          <div v-if="scanResult.motherboard" class="info-row"><span>Carte mère</span><span>{{ scanResult.motherboard }}</span></div>
          <!-- Écrans -->
          <div v-if="scanResult.monitors_detail" class="info-row info-full">
            <span style="display:flex;align-items:center;gap:4px"><Monitor :size="12" /> Écran(s)</span>
            <span style="font-size:11px">{{ scanResult.monitors_detail }}</span>
          </div>
          <div v-else-if="scanResult.screen_resolution" class="info-row"><span>Résolution</span><span>{{ scanResult.screen_resolution }}</span></div>
          <!-- Réseau -->
          <div v-if="scanResult.network_adapters_summary" class="info-row info-full">
            <span>Interfaces réseau actives</span><span style="font-size:11px">{{ scanResult.network_adapters_summary }}</span>
          </div>
        </div>
      </div>

      <!-- ===== STOCKAGE PHYSIQUE ===== -->
      <div v-if="scanResult.storage_items?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Stockage physique ({{ scanResult.storage_items.length }} disque(s))</p>
        <div v-for="(s, i) in scanResult.storage_items" :key="i"
          style="display:flex;align-items:center;gap:10px;padding:6px 0;border-bottom:1px solid var(--border);font-size:12px;flex-wrap:wrap">
          <HardDrive :size="13" style="color:var(--accent);flex-shrink:0" />
          <span style="font-weight:500;flex:1;min-width:160px">{{ s.model || "Disque inconnu" }}</span>
          <NBadge :variant="s.media_type === 'SSD' || s.media_type === 'NVMe' ? 'info' : 'default'">{{ s.media_type }}</NBadge>
          <NBadge variant="neutral">{{ s.interface_type }}</NBadge>
          <span class="muted">{{ s.size_gb > 0 ? s.size_gb + ' GB' : '—' }}</span>
          <NBadge :variant="s.health === 'Healthy' || s.health === 'Sain' ? 'success' : s.health ? 'warning' : 'neutral'">
            {{ s.health || "—" }}
          </NBadge>
        </div>
      </div>

      <!-- ===== ESPACE DISQUE ===== -->
      <div v-if="scanResult.disk_usage?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Espace disque (volumes)</p>
        <div v-for="d in scanResult.disk_usage" :key="d.drive"
          style="display:flex;align-items:center;gap:12px;margin-bottom:8px;font-size:13px">
          <code style="min-width:40px">{{ d.drive }}</code>
          <NProgress :value="d.used_percent"
            :variant="d.used_percent > 90 ? 'danger' : d.used_percent > 80 ? 'warning' : 'default'"
            size="sm" showLabel style="flex:1" />
          <span class="muted" style="min-width:130px;text-align:right;font-size:11px;font-family:monospace">
            {{ d.free_gb.toFixed(0) }} GB libres / {{ d.total_gb.toFixed(0) }} GB
          </span>
        </div>
      </div>

      <!-- ===== LICENCES & CHIFFREMENT ===== -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Licences & Chiffrement</p>
        <div class="info-grid">
          <!-- Clé Windows -->
          <div class="info-row info-full">
            <span style="display:flex;align-items:center;gap:4px"><Key :size="12" /> Clé Windows</span>
            <code v-if="scanResult.windows_product_key" style="color:var(--accent);font-size:12px">
              {{ scanResult.windows_product_key }}
            </code>
            <span v-else class="muted" style="font-size:11px">Non disponible (BIOS/UEFI sans clé embarquée)</span>
          </div>
          <!-- Clé Office -->
          <div v-if="scanResult.office_name || scanResult.office_product_key" class="info-row info-full">
            <span style="display:flex;align-items:center;gap:4px"><Key :size="12" /> {{ scanResult.office_name || "Office" }}</span>
            <code v-if="scanResult.office_product_key" style="color:var(--success);font-size:12px">
              {{ scanResult.office_product_key }}
            </code>
            <span v-else class="muted" style="font-size:11px">Clé non disponible dans le registre</span>
          </div>
        </div>

        <!-- BitLocker -->
        <div v-if="scanResult.bitlocker_volumes?.length" style="margin-top:10px">
          <p style="font-size:12px;font-weight:600;margin-bottom:6px;color:var(--text-secondary)">BitLocker</p>
          <div v-for="(bv, i) in scanResult.bitlocker_volumes" :key="i"
            style="border:1px solid var(--border);border-radius:6px;padding:10px;margin-bottom:8px">
            <div style="display:flex;align-items:center;gap:8px;margin-bottom:6px">
              <component :is="bv.protection_status === 'On' || bv.protection_status === '1' ? Lock : LockOpen"
                :size="14" :style="{ color: bv.protection_status === 'On' || bv.protection_status === '1' ? 'var(--success)' : 'var(--warning)' }" />
              <strong>{{ bv.drive }}</strong>
              <NBadge :variant="bv.protection_status === 'On' || bv.protection_status === '1' ? 'success' : 'warning'">
                {{ bv.protection_status === 'On' || bv.protection_status === '1' ? 'Protégé' : 'Non protégé' }}
              </NBadge>
              <span v-if="bv.encryption_percent < 100 && bv.encryption_percent > 0" class="muted" style="font-size:11px">
                {{ bv.encryption_percent }}% chiffré
              </span>
              <div style="display:flex;gap:4px;flex-wrap:wrap">
                <NBadge v-for="(p, pi) in bv.protectors" :key="pi" variant="neutral" style="font-size:10px">{{ p }}</NBadge>
              </div>
            </div>
            <div v-if="bv.recovery_password" class="info-row" style="background:var(--bg-secondary);padding:6px 8px;border-radius:4px">
              <span style="font-size:12px;color:var(--text-secondary)">Clé de récupération</span>
              <code style="font-size:11px;color:var(--warning);word-break:break-all">{{ bv.recovery_password }}</code>
            </div>
            <div v-else class="muted" style="font-size:11px">Aucune clé de récupération disponible</div>
          </div>
        </div>
        <div v-else class="muted" style="font-size:12px;margin-top:8px">
          BitLocker non configuré (ou non détectable sans droits admin)
        </div>
      </div>

      <!-- ===== BATTERIE ===== -->
      <div v-if="batteries?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Batterie</p>
        <div v-for="(b, i) in batteries" :key="i">
          <div style="display:flex;align-items:center;gap:8px;margin-bottom:8px">
            <Battery :size="14" style="color:var(--accent)" />
            <strong>{{ b.name }}</strong>
            <NBadge :variant="b.battery_health_percent > 80 ? 'success' : b.battery_health_percent > 50 ? 'warning' : 'danger'">
              {{ b.battery_health_percent.toFixed(0) }}% santé
            </NBadge>
            <NBadge :variant="b.status === 'Charging' || b.status === 'En charge' ? 'success' : 'info'">
              {{ b.status || "—" }}
            </NBadge>
          </div>
          <div class="info-grid">
            <div class="info-row"><span>Charge actuelle</span>
              <NBadge :variant="b.estimated_charge_remaining < 20 ? 'danger' : b.estimated_charge_remaining < 40 ? 'warning' : 'success'">
                {{ b.estimated_charge_remaining }}%
              </NBadge>
            </div>
            <div class="info-row"><span>Autonomie estimée</span><span>{{ b.estimated_run_time || "—" }}</span></div>
            <div class="info-row"><span>Cycles de charge</span>
              <NBadge :variant="b.cycle_count > 500 ? 'danger' : b.cycle_count > 300 ? 'warning' : 'success'">
                {{ b.cycle_count > 0 ? b.cycle_count : 'N/A' }}
              </NBadge>
            </div>
            <div class="info-row"><span>Capacité de conception</span>
              <span>{{ b.design_capacity > 0 ? b.design_capacity + ' mWh' : 'N/A' }}</span>
            </div>
            <div class="info-row"><span>Capacité actuelle</span>
              <span>{{ b.full_charge_capacity > 0 ? b.full_charge_capacity + ' mWh' : 'N/A' }}</span>
            </div>
            <div v-if="b.design_capacity > 0 && b.full_charge_capacity > 0" class="info-row">
              <span>Usure capacité</span>
              <code style="color:var(--warning)">
                -{{ ((1 - b.full_charge_capacity / b.design_capacity) * 100).toFixed(1) }}%
              </code>
            </div>
            <div class="info-row"><span>Chimie</span><span>{{ b.chemistry || "N/A" }}</span></div>
          </div>
        </div>
      </div>

      <!-- ===== SÉCURITÉ ===== -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Sécurité</p>
        <div v-for="[ok, label, val] in [
          [scanResult.firewall_enabled, 'Pare-feu Windows', scanResult.firewall_enabled ? 'Activé' : 'DÉSACTIVÉ ⚠'],
          [scanResult.defender_enabled, 'Defender (temps réel)', scanResult.defender_enabled ? 'Actif' : 'INACTIF ⚠'],
          [scanResult.network_ok, 'Connectivité Internet (8.8.8.8)', scanResult.network_ok ? 'OK' : 'Hors ligne'],
          [scanResult.suspicious_processes?.length === 0, 'Processus suspects', scanResult.suspicious_processes?.length === 0 ? 'Aucun' : `${scanResult.suspicious_processes?.length} détecté(s)`],
        ]" :key="label" style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
          <component :is="ok ? CheckCircle : AlertTriangle" :size="14" :class="ok ? 'ic-ok' : 'ic-warn'" />
          <span style="flex:1">{{ label }}</span>
          <span class="mono">{{ val }}</span>
        </div>
        <div v-if="scanResult.open_ports?.length" style="display:flex;align-items:center;gap:8px;padding:6px 0;font-size:13px">
          <AlertTriangle :size="14" class="ic-warn" />
          <span style="flex:1">Ports en écoute globale</span>
          <code class="mono" style="font-size:11px">{{ scanResult.open_ports.join(", ") }}</code>
        </div>
        <div style="display:flex;align-items:center;gap:8px;padding:6px 0;border-bottom:1px solid var(--border);font-size:13px">
          <component :is="!scanResult.last_bsod || scanResult.last_bsod.includes('Aucun') ? CheckCircle : AlertTriangle"
            :size="14" :class="!scanResult.last_bsod || scanResult.last_bsod.includes('Aucun') ? 'ic-ok' : 'ic-warn'" />
          <span style="flex:1">Dernier BSOD</span>
          <span class="mono">{{ scanResult.last_bsod || "Aucun" }}</span>
        </div>
      </div>

      <!-- ===== ANTIVIRUS ===== -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Antivirus & Protection</p>
        <div class="info-grid">
          <div class="info-row"><span>Antivirus tiers</span><span>{{ scanResult.antivirus_installed || "Aucun (Defender)" }}</span></div>
          <div class="info-row"><span>Définitions Defender</span>
            <NBadge :variant="scanResult.defender_definition_age_days < 0 ? 'neutral' : scanResult.defender_definition_age_days <= 3 ? 'success' : scanResult.defender_definition_age_days <= 7 ? 'warning' : 'danger'">
              {{ scanResult.defender_definition_age_days >= 0 ? scanResult.defender_definition_age_days + ' j' : 'N/A' }}
            </NBadge>
          </div>
          <div class="info-row"><span>Dernier KB Windows</span>
            <NBadge :variant="scanResult.last_update_days < 0 ? 'neutral' : scanResult.last_update_days <= 30 ? 'success' : scanResult.last_update_days <= 60 ? 'warning' : 'danger'">
              {{ scanResult.last_update_days >= 0 ? 'il y a ' + scanResult.last_update_days + ' j' : 'N/A' }}
            </NBadge>
          </div>
        </div>
      </div>

      <!-- ===== INTÉGRITÉ WINDOWS ===== -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Intégrité Windows</p>

        <!-- DISM row -->
        <div style="padding:6px 0;border-bottom:1px solid var(--border)">
          <div style="display:flex;align-items:center;gap:8px;font-size:13px">
            <component :is="scanResult.dism_status?.toLowerCase().includes('sain') ? CheckCircle : AlertTriangle"
              :size="14" :class="scanResult.dism_status?.toLowerCase().includes('sain') ? 'ic-ok' : 'ic-warn'" />
            <span style="flex:1">DISM (Health Store)</span>
            <span class="mono">{{ scanResult.dism_status }}</span>
            <NButton v-if="!scanResult.dism_status?.toLowerCase().includes('sain')" size="sm" variant="danger"
              style="font-size:10px;padding:2px 8px" @click="runRepairCommand('dism')">
              Réparer (DISM)
            </NButton>
          </div>
          <pre v-if="scanResult.dism_details && !scanResult.dism_status?.toLowerCase().includes('sain')"
            class="scan-details-pre">{{ scanResult.dism_details }}</pre>
        </div>

        <!-- SFC row -->
        <div style="padding:6px 0;border-bottom:1px solid var(--border)">
          <div style="display:flex;align-items:center;gap:8px;font-size:13px">
            <component :is="scanResult.sfc_status?.toLowerCase().includes('intèg') || scanResult.sfc_status?.toLowerCase().includes('integ') ? CheckCircle : AlertTriangle"
              :size="14" :class="scanResult.sfc_status?.toLowerCase().includes('intèg') || scanResult.sfc_status?.toLowerCase().includes('integ') ? 'ic-ok' : 'ic-warn'" />
            <span style="flex:1">SFC (System File Checker)</span>
            <span class="mono">{{ scanResult.sfc_status }}</span>
            <NButton size="sm" variant="ghost" style="font-size:10px;padding:2px 8px" @click="runRepairCommand('sfc')">
              Lancer SFC
            </NButton>
          </div>
          <pre v-if="scanResult.sfc_details" class="scan-details-pre">{{ scanResult.sfc_details }}</pre>
        </div>

        <!-- WinGet row -->
        <div style="padding:6px 0">
          <div style="display:flex;align-items:center;gap:8px;font-size:13px">
            <component :is="scanResult.winget_upgradable?.length === 0 ? CheckCircle : AlertTriangle"
              :size="14" :class="scanResult.winget_upgradable?.length === 0 ? 'ic-ok' : 'ic-warn'" />
            <span style="flex:1">WinGet — mises à jour disponibles</span>
            <span class="mono">{{ scanResult.winget_upgradable?.length || 0 }} logiciel(s)</span>
          </div>
          <div v-if="scanResult.winget_upgradable?.length" style="margin-top:8px">
            <div v-for="(u, i) in scanResult.winget_upgradable" :key="i"
              style="display:flex;align-items:center;gap:8px;padding:4px 0;border-bottom:1px solid var(--border);font-size:12px">
              <span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-weight:500">{{ u.name }}</span>
              <code class="muted" style="font-size:10px;min-width:160px">{{ u.id }}</code>
              <NBadge variant="warning" style="font-size:10px;flex-shrink:0">{{ u.current_version }}</NBadge>
              <span style="color:var(--success);font-size:10px;flex-shrink:0">→ {{ u.available_version }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- ===== ABONNEMENTS WMI SUSPECTS ===== -->
      <div v-if="scanResult.wmi_subscriptions > 0" class="diag-section" style="border-left:3px solid var(--danger)">
        <p class="diag-section-label" style="margin:0 0 8px 0;color:var(--danger)">
          <AlertTriangle :size="13" style="display:inline;margin-right:6px" />
          Abonnements WMI suspects ({{ scanResult.wmi_subscriptions }}) — Indicateur malware potentiel
        </p>
        <div v-for="(sub, i) in scanResult.wmi_subscription_details" :key="i"
          style="padding:6px 0;border-bottom:1px solid var(--border)">
          <div style="display:flex;align-items:center;gap:8px;font-size:12px;flex-wrap:wrap">
            <code style="font-weight:600;color:var(--danger)">{{ sub.name || "(sans nom)" }}</code>
            <NBadge variant="danger" style="font-size:10px">{{ sub.consumer_type }}</NBadge>
            <button class="reg-action-btn" title="Copier le chemin WMI" @click="copyRegPath(sub.path)">📋</button>
          </div>
          <div class="muted" style="font-size:10px;margin-top:2px;font-family:monospace;word-break:break-all">{{ sub.path }}</div>
        </div>
        <div style="margin-top:8px;padding:8px;background:rgba(239,68,68,0.08);border-radius:6px;font-size:11px;color:var(--text-secondary)">
          Pour supprimer : <code style="color:var(--danger)">Get-WmiObject -Namespace root\subscription -Class __EventConsumer | Remove-WmiObject</code>
          <button class="reg-action-btn" style="margin-left:6px" @click="copyRegPath('Get-WmiObject -Namespace root\\subscription -Class __EventConsumer | Remove-WmiObject')">📋</button>
        </div>
      </div>

      <!-- ===== PROCESSUS SUSPECTS ===== -->
      <div v-if="scanResult.suspicious_processes?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Processus hors chemins sécurisés ({{ scanResult.suspicious_processes.length }})</p>
        <div v-for="p in scanResult.suspicious_processes" :key="p.pid" class="list-row">
          <code class="list-name">{{ p.name }}</code>
          <NBadge variant="warning" style="flex-shrink:0">{{ p.reason }}</NBadge>
          <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ p.path }}</div>
        </div>
      </div>

      <!-- ===== SERVICES TIERS ===== -->
      <div v-if="scanResult.suspicious_services?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Services tiers actifs ({{ scanResult.suspicious_services.length }})</p>
        <div v-for="(s, i) in scanResult.suspicious_services.slice(0, 15)" :key="i" class="list-row">
          <code class="list-name" style="min-width:130px">{{ s.name }}</code>
          <div class="muted" style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ s.display_name }}</div>
          <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ s.path }}</div>
        </div>
      </div>

      <!-- ===== AUTORUNS ===== -->
      <div v-if="scanResult.autorun_entries?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Entrées Autorun tiers ({{ scanResult.autorun_entries.length }})</p>
        <div v-for="(a, i) in scanResult.autorun_entries.slice(0, 20)" :key="i" class="list-row autorun-row">
          <code class="list-name" style="min-width:120px;flex-shrink:0">{{ a.name }}</code>
          <div class="autorun-reg-path" :title="fullRegPath(a.location, a.name)">
            <span class="reg-path-text">{{ fullRegPath(a.location, a.name) }}</span>
          </div>
          <div class="muted" style="flex:1;min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px" :title="a.path">{{ a.path }}</div>
          <div class="autorun-actions">
            <button class="reg-action-btn" title="Copier le chemin de registre" @click="copyRegPath(fullRegPath(a.location, a.name))">📋</button>
            <button class="reg-action-btn" title="Ouvrir dans Regedit" @click="openRegedit(a.location)">🔑</button>
          </div>
        </div>
      </div>

      <!-- ===== ÉVÉNEMENTS D'ERREUR ===== -->
      <div v-if="scanResult.recent_errors?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">
          Événements d'erreur récents (48h) — {{ scanResult.recent_errors.length }}
        </p>
        <div v-for="(e, i) in scanResult.recent_errors.slice(0, 25)" :key="i" class="list-row">
          <code class="muted" style="min-width:110px;font-size:10px">{{ e.time }}</code>
          <NBadge :variant="e.level?.toLowerCase().includes('critical') || e.level?.toLowerCase().includes('critique') ? 'danger' : 'warning'"
            style="flex-shrink:0;font-size:10px">{{ e.level }}</NBadge>
          <div style="flex:1;min-width:0">
            <span style="font-size:12px;font-weight:500">{{ e.source }}</span>
            <div class="muted" style="font-size:11px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis">{{ e.message }}</div>
          </div>
        </div>
      </div>

      <!-- ===== SÉCURITÉ AVANCÉE ===== -->
      <div v-if="scanResult.tpm_present !== undefined" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Sécurité Avancée</p>
        <div class="info-grid">
          <div class="info-row"><span>TPM</span>
            <NBadge :variant="scanResult.tpm_present ? 'success' : 'warning'">
              {{ scanResult.tpm_present ? (scanResult.tpm_enabled ? 'Présent & Activé' : 'Présent (désactivé)') : 'Absent' }}
            </NBadge>
          </div>
          <div v-if="scanResult.tpm_present && scanResult.tpm_version" class="info-row">
            <span>Version TPM</span><span>{{ scanResult.tpm_version }}</span>
          </div>
          <div class="info-row"><span>Secure Boot</span>
            <NBadge :variant="scanResult.secure_boot ? 'success' : 'warning'">{{ scanResult.secure_boot ? 'Activé' : 'Désactivé' }}</NBadge>
          </div>
          <div class="info-row"><span>Niveau UAC</span><span>{{ scanResult.uac_level || 'Inconnu' }}</span></div>
          <div class="info-row"><span>Bureau à distance (RDP)</span>
            <NBadge :variant="scanResult.rdp_enabled ? 'warning' : 'success'">{{ scanResult.rdp_enabled ? 'Activé' : 'Désactivé' }}</NBadge>
          </div>
          <div class="info-row"><span>SMBv1 (obsolète)</span>
            <NBadge :variant="scanResult.smbv1_enabled ? 'danger' : 'success'">{{ scanResult.smbv1_enabled ? 'Activé ⚠' : 'Désactivé' }}</NBadge>
          </div>
          <div class="info-row"><span>Abonnements WMI (indicateur malware)</span>
            <NBadge :variant="scanResult.wmi_subscriptions > 0 ? 'danger' : 'success'">{{ scanResult.wmi_subscriptions ?? 0 }}</NBadge>
          </div>
          <div class="info-row"><span>Compte Invité</span>
            <NBadge :variant="scanResult.guest_enabled ? 'warning' : 'success'">{{ scanResult.guest_enabled ? 'Activé ⚠' : 'Désactivé' }}</NBadge>
          </div>
          <div v-if="scanResult.pending_updates_cached >= 0" class="info-row">
            <span>MAJ Windows en attente (cache)</span>
            <NBadge :variant="scanResult.pending_updates_cached > 10 ? 'danger' : scanResult.pending_updates_cached > 0 ? 'warning' : 'success'">
              {{ scanResult.pending_updates_cached }}
            </NBadge>
          </div>
          <div v-if="scanResult.last_restore_point" class="info-row">
            <span>Dernier point de restauration</span>
            <NBadge :variant="scanResult.last_restore_point.includes('Aucun') ? 'warning' : 'success'">
              {{ scanResult.last_restore_point }}
            </NBadge>
          </div>
        </div>
        <div v-if="scanResult.local_admins?.length" style="margin-top:8px">
          <p style="font-size:12px;color:var(--text-secondary);margin:0 0 4px 0">Administrateurs locaux ({{ scanResult.local_admins.length }})</p>
          <div style="display:flex;gap:6px;flex-wrap:wrap">
            <code v-for="(a, i) in scanResult.local_admins" :key="i"
              style="font-size:11px;background:var(--bg-secondary);padding:2px 6px;border-radius:4px">{{ a }}</code>
          </div>
        </div>
      </div>

      <!-- ===== IDENTITÉ SYSTÈME & BIOS ===== -->
      <div v-if="scanResult.system_manufacturer || scanResult.bios_manufacturer" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Identité Système & BIOS</p>
        <div class="info-grid">
          <div v-if="scanResult.system_manufacturer" class="info-row"><span>Fabricant</span><span>{{ scanResult.system_manufacturer }}</span></div>
          <div v-if="scanResult.system_model" class="info-row"><span>Modèle</span><span>{{ scanResult.system_model }}</span></div>
          <div v-if="scanResult.system_serial && scanResult.system_serial !== 'N/A'" class="info-row">
            <span>N° Série carte mère</span><code style="font-size:11px">{{ scanResult.system_serial }}</code>
          </div>
          <div v-if="scanResult.bios_manufacturer" class="info-row"><span>BIOS Fabricant</span><span>{{ scanResult.bios_manufacturer }}</span></div>
          <div v-if="scanResult.bios_version" class="info-row"><span>BIOS Version</span><code style="font-size:11px">{{ scanResult.bios_version }}</code></div>
          <div v-if="scanResult.bios_date" class="info-row"><span>BIOS Date</span><span>{{ scanResult.bios_date }}</span></div>
          <div v-if="scanResult.license_type" class="info-row"><span>Type de licence Windows</span>
            <NBadge :variant="scanResult.license_type === 'OEM' ? 'info' : scanResult.license_type === 'Retail' ? 'success' : 'neutral'">
              {{ scanResult.license_type }}
            </NBadge>
          </div>
        </div>
      </div>

      <!-- ===== TOP PROCESSUS ===== -->
      <div v-if="scanResult.top_cpu?.length || scanResult.top_ram?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Top 5 Processus (snapshot)</p>
        <div style="display:flex;gap:16px;flex-wrap:wrap">
          <div v-if="scanResult.top_cpu?.length" style="flex:1;min-width:200px">
            <p style="font-size:11px;font-weight:600;color:var(--text-secondary);margin:0 0 6px 0"><Cpu :size="11" style="display:inline;margin-right:4px" />CPU — temps cumulé (sec.)</p>
            <div v-for="(p, i) in scanResult.top_cpu" :key="i"
              style="display:flex;align-items:center;gap:8px;padding:4px 0;border-bottom:1px solid var(--border);font-size:12px">
              <code class="muted" style="min-width:34px;font-size:10px">{{ p.pid }}</code>
              <span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ p.name }}</span>
              <code style="color:var(--accent);font-size:11px;flex-shrink:0">{{ p.value }}s</code>
            </div>
          </div>
          <div v-if="scanResult.top_ram?.length" style="flex:1;min-width:200px">
            <p style="font-size:11px;font-weight:600;color:var(--text-secondary);margin:0 0 6px 0"><MemoryStick :size="11" style="display:inline;margin-right:4px" />RAM (MB)</p>
            <div v-for="(p, i) in scanResult.top_ram" :key="i"
              style="display:flex;align-items:center;gap:8px;padding:4px 0;border-bottom:1px solid var(--border);font-size:12px">
              <code class="muted" style="min-width:34px;font-size:10px">{{ p.pid }}</code>
              <span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{{ p.name }}</span>
              <code style="color:var(--success);font-size:11px;flex-shrink:0">{{ p.value }} MB</code>
            </div>
          </div>
        </div>
      </div>

      <!-- ===== TÂCHES PLANIFIÉES SUSPECTES ===== -->
      <div v-if="scanResult.susp_tasks?.length" class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Tâches planifiées suspectes ({{ scanResult.susp_tasks_count }})</p>
        <div v-for="(t, i) in scanResult.susp_tasks" :key="i" class="list-row">
          <code class="list-name" style="min-width:140px">{{ t.name }}</code>
          <NBadge variant="warning" style="flex-shrink:0;font-size:10px">{{ t.path }}</NBadge>
          <div class="muted" style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:11px">{{ t.exec }}</div>
        </div>
      </div>

      <!-- Export scan -->
      <div style="display:flex;gap:8px;flex-wrap:wrap;align-items:center;padding-top:4px">
        <NButton variant="ghost" size="sm" @click="onRunScan"><RefreshCw :size="12" /> Relancer</NButton>
        <NButton variant="ghost" size="sm" @click="exportScanTxt"><FileText :size="12" /> Export .txt</NButton>
        <NButton variant="ghost" size="sm" @click="exportScanHtml"><FileCode :size="12" /> Export .html</NButton>
        <NButton variant="ghost" size="sm" @click="exportScanMd"><FileDown :size="12" /> Export .md</NButton>
        <NButton variant="ghost" size="sm" @click="exportScanJson"><FileDown :size="12" /> Export .json</NButton>
      </div>
    </div>
  </div>
</template>
