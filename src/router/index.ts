import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "dashboard",
      component: () => import("@/pages/DashboardPage.vue"),
      meta: { title: "Tableau de bord" },
    },
    {
      path: "/diagnostic",
      name: "diagnostic",
      component: () => import("@/pages/DiagnosticPage.vue"),
      meta: { title: "Diagnostic" },
    },
    {
      path: "/monitoring",
      name: "monitoring",
      component: () => import("@/pages/MonitoringPage.vue"),
      meta: { title: "Monitoring" },
    },
    {
      path: "/optimizations",
      name: "optimizations",
      component: () => import("@/pages/OptimizationsPage.vue"),
      meta: { title: "Optimisations" },
    },
    {
      path: "/applications",
      name: "applications",
      component: () => import("@/pages/ApplicationsPage.vue"),
      meta: { title: "Applications" },
    },
    {
      path: "/tools",
      name: "tools",
      component: () => import("@/pages/ToolsPage.vue"),
      meta: { title: "Outils Systeme" },
    },
    {
      path: "/master-install",
      name: "master-install",
      component: () => import("@/pages/MasterInstallPage.vue"),
      meta: { title: "Master Install" },
    },
    {
      path: "/portables",
      name: "portables",
      component: () => import("@/pages/PortablesPage.vue"),
      meta: { title: "Apps Portables" },
    },
    {
      path: "/os-downloads",
      name: "os-downloads",
      component: () => import("@/pages/OsDownloadsPage.vue"),
      meta: { title: "OS & USB Tools" },
    },
    {
      path: "/updates",
      name: "updates",
      component: () => import("@/pages/UpdatesPage.vue"),
      meta: { title: "Mises a jour" },
    },
    {
      path: "/drivers",
      name: "drivers",
      component: () => import("@/pages/DriversPage.vue"),
      meta: { title: "Drivers" },
    },
    {
      path: "/driver-scanner",
      name: "driver-scanner",
      component: () => import("@/pages/DriverScannerPage.vue"),
      meta: { title: "Scanner Pilotes" },
    },
    {
      path: "/backup",
      name: "backup",
      component: () => import("@/pages/BackupPage.vue"),
      meta: { title: "Sauvegarde" },
    },
    {
      path: "/uninstaller",
      name: "uninstaller",
      component: () => import("@/pages/UninstallerPage.vue"),
      meta: { title: "Désinstallateur Propre" },
    },
    {
      path: "/clone",
      name: "clone",
      component: () => import("@/pages/ClonePage.vue"),
      meta: { title: "Clonage Système" },
    },
    {
      path: "/data-recovery",
      name: "data-recovery",
      component: () => import("@/pages/DataRecoveryPage.vue"),
      meta: { title: "Récupération de Données" },
    },
    {
      path: "/scanvirus",
      name: "scanvirus",
      component: () => import("@/pages/ScanVirusPage.vue"),
      meta: { title: "Scan Antivirus" },
    },
    {
      path: "/network",
      name: "network",
      component: () => import("@/pages/NetworkPage.vue"),
      meta: { title: "Reseau" },
    },
    {
      path: "/terminal",
      name: "terminal",
      component: () => import("@/pages/TerminalPage.vue"),
      meta: { title: "Terminal" },
    },
    {
      path: "/scripts",
      name: "scripts",
      component: () => import("@/pages/ScriptsPage.vue"),
      meta: { title: "Scripts Windows" },
    },
    {
      path: "/system-utils",
      name: "system-utils",
      component: () => import("@/pages/ToolsPage.vue"),
      meta: { title: "Utilitaires" },
    },
    {
      path: "/ai-agents",
      name: "ai-agents",
      component: () => import("@/pages/AiAgentsPage.vue"),
      meta: { title: "Agent IA" },
    },
    {
      path: "/knowledge-base",
      name: "knowledge-base",
      component: () => import("@/pages/KnowledgeBasePage.vue"),
      meta: { title: "Base de Connaissances" },
    },
    {
      path: "/documentation",
      name: "documentation",
      component: () => import("@/pages/DocumentationPage.vue"),
      meta: { title: "Documentation" },
    },
    {
      path: "/logs",
      name: "logs",
      component: () => import("@/pages/LogsPage.vue"),
      meta: { title: "Logs" },
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/pages/SettingsPage.vue"),
      meta: { title: "Parametres" },
    },
  ],
});

export default router;
