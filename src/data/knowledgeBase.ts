import type { Component } from "vue";
import { kbHardware } from "./kb/kb_hardware";
import { kbGaming } from "./kb/kb_gaming";
import { kbWindowsAvance } from "./kb/kb_windows_avance";
import { kbSecuriteAvancee } from "./kb/kb_securite_avancee";
import { kbReparation } from "./kb/kb_reparation";
import { kbLogiciels } from "./kb/kb_logiciels";
import { kbReseauAvance } from "./kb/kb_reseau_avance";
import { kbScriptsAvances } from "./kb/kb_scripts_avances";
import { kbDeveloppement } from "./kb/kb_developpement";
import { kbMultimedia } from "./kb/kb_multimedia";
import { kbVirtualisation } from "./kb/kb_virtualisation";
import { kbCloudSauvegarde } from "./kb/kb_cloud_sauvegarde";
import { kbEntrepriseAD } from "./kb/kb_entreprise_ad";
import { kbLinuxDualboot } from "./kb/kb_linux_dualboot";
import { kbEnergieBatterie } from "./kb/kb_energie_batterie";
import { kbStockageRaid } from "./kb/kb_stockage_raid";
import { kbAutomatisation } from "./kb/kb_automatisation";
import { kbErreursSysteme } from "./kb/kb_erreurs_systeme";
import { kbAndroidMobile } from "./kb/kb_android_mobile";
import { kbPeripheriquesAvances } from "./kb/kb_peripheriques_avances";
import { kbReseauEntreprise } from "./kb/kb_reseau_entreprise";
import { kbIaOutils } from "./kb/kb_ia_outils";
// === Catégories de base ===
import { kbReseau } from "./kb/kb_reseau";
import { kbPerformance } from "./kb/kb_performance";
import { kbSecurite } from "./kb/kb_securite";
import { kbStockage } from "./kb/kb_stockage";
import { kbAffichage } from "./kb/kb_affichage";
import { kbAudio } from "./kb/kb_audio";
import { kbPeripheriques } from "./kb/kb_peripheriques";
import { kbBsod } from "./kb/kb_bsod";
import { kbRegistre } from "./kb/kb_registre";
import { kbDemarrage } from "./kb/kb_demarrage";
import { kbWindows11 } from "./kb/kb_windows11";
import { kbPowershell } from "./kb/kb_powershell";
import { kbCmd } from "./kb/kb_cmd";
import { kbHardwareCore } from "./kb/kb_hardware_core";
import { kbMaintenance } from "./kb/kb_maintenance";
import { kbPilotes } from "./kb/kb_pilotes";
import { kbLogicielsCore } from "./kb/kb_logiciels_core";

export interface KBItem {
  title: string;
  symptoms?: string;
  solution?: string[];
  command?: string;
  code?: string;
  note?: string;
}

export interface KBCategory {
  id: string;
  label: string;
  icon: string;
  items: KBItem[];
}

export const knowledgeBase: KBCategory[] = [
  // === Catégories de base ===
  ...kbReseau,
  ...kbPerformance,
  ...kbSecurite,
  ...kbStockage,
  ...kbAffichage,
  ...kbAudio,
  ...kbPeripheriques,
  ...kbBsod,
  ...kbRegistre,
  ...kbDemarrage,
  ...kbWindows11,
  ...kbPowershell,
  ...kbCmd,
  ...kbHardwareCore,
  ...kbMaintenance,
  ...kbPilotes,
  ...kbLogicielsCore,
  // === Catégories étendues ===
  ...kbHardware,
  ...kbGaming,
  ...kbWindowsAvance,
  ...kbSecuriteAvancee,
  ...kbReparation,
  ...kbLogiciels,
  ...kbReseauAvance,
  ...kbScriptsAvances,
  // === Catégories v26.36.0 ===
  ...kbDeveloppement,
  ...kbMultimedia,
  ...kbVirtualisation,
  ...kbCloudSauvegarde,
  ...kbEntrepriseAD,
  ...kbLinuxDualboot,
  ...kbEnergieBatterie,
  ...kbStockageRaid,
  ...kbAutomatisation,
  ...kbErreursSysteme,
  ...kbAndroidMobile,
  ...kbPeripheriquesAvances,
  ...kbReseauEntreprise,
  ...kbIaOutils,
];
