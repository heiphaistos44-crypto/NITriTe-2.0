/**
 * Cache global de données préchargées au démarrage.
 * Les pages lisent depuis ce store au lieu de re-invoquer Tauri.
 * Résultat : navigation instantanée après le splash screen.
 *
 * TTL par commande : les données courtes (processus, réseau) expirent vite,
 * les données statiques (hardware, software) restent longtemps.
 */
import { defineStore } from "pinia";
import { ref } from "vue";

// ── TTL par commande Tauri (millisecondes) ───────────────────────────────────

const TTL_MAP: Record<string, number> = {
  // Données volatiles — se rafraîchissent fréquemment
  get_running_processes:       15_000,   // 15s
  get_processes_extended:      15_000,
  get_windows_services:        30_000,   // 30s
  get_active_connections:      20_000,
  get_network_overview:        20_000,
  get_connections:             20_000,
  get_perf_history:            10_000,   // 10s
  get_top_processes_by_cpu:    10_000,

  // Données semi-stables — changent rarement en session
  get_system_info:           5 * 60_000, // 5 min
  get_platform_info:         5 * 60_000,
  get_battery_detailed:      2 * 60_000, // 2 min (charge varie)
  get_battery_extended:      2 * 60_000,
  get_network_adapters_detailed: 60_000, // 1 min
  get_storage_physical_info: 5 * 60_000,
  get_disk_usage:            2 * 60_000,

  // Données quasi-statiques — changent seulement après install/désinstall
  get_installed_software:   10 * 60_000, // 10 min
  get_system_history:       10 * 60_000,
  get_apps:                 10 * 60_000,
  get_tools:                10 * 60_000,
  get_startup_programs:      5 * 60_000,
  get_scheduled_tasks:       5 * 60_000,
  get_firewall_rules:        5 * 60_000,
  get_user_accounts:         5 * 60_000,

  // Données hardware — quasi-immuables en session
  get_gpu_detailed:         30 * 60_000, // 30 min
  get_ram_detailed:         30 * 60_000,
  get_motherboard_detailed: 30 * 60_000,
  get_cpu_cache_info:       30 * 60_000,
  get_bios_info:            30 * 60_000,
  get_bios_extended:        30 * 60_000,
};

/** TTL par défaut si la commande n'est pas dans TTL_MAP */
const DEFAULT_TTL = 60_000; // 1 min

// ── Types ────────────────────────────────────────────────────────────────────

interface CacheEntry {
  data: unknown;
  timestamp: number; // Date.now() à l'écriture
  ttl: number;       // Durée de vie en ms
}

// ── Store ────────────────────────────────────────────────────────────────────

export const useDataCache = defineStore("dataCache", () => {
  const cache = ref<Record<string, CacheEntry>>({});

  /** Durée de vie pour une clé donnée (utilise le préfixe de commande avant "::" si args) */
  function resolveTtl(key: string, ttlOverride?: number): number {
    if (ttlOverride !== undefined) return ttlOverride;
    const cmd = key.split("::")[0];
    return TTL_MAP[cmd] ?? DEFAULT_TTL;
  }

  /** Vérifie si une entrée existe et est encore fraîche */
  function isAlive(entry: CacheEntry): boolean {
    return Date.now() - entry.timestamp < entry.ttl;
  }

  function set(key: string, data: unknown, ttl?: number) {
    cache.value[key] = {
      data,
      timestamp: Date.now(),
      ttl: resolveTtl(key, ttl),
    };
  }

  function get<T>(key: string): T | undefined {
    const entry = cache.value[key];
    if (!entry) return undefined;
    if (!isAlive(entry)) {
      delete cache.value[key]; // Expiration automatique à la lecture
      return undefined;
    }
    return entry.data as T;
  }

  /** Retourne true seulement si la donnée existe ET est encore fraîche */
  function has(key: string): boolean {
    return get(key) !== undefined;
  }

  function invalidate(key: string) {
    delete cache.value[key];
  }

  /** Supprime toutes les entrées expirées (nettoyage proactif optionnel) */
  function purgeExpired() {
    for (const key of Object.keys(cache.value)) {
      if (!isAlive(cache.value[key])) {
        delete cache.value[key];
      }
    }
  }

  /** Vide tout le cache (ex: après un changement de session ou une action majeure) */
  function clear() {
    cache.value = {};
  }

  return { cache, set, get, has, invalidate, purgeExpired, clear };
});
