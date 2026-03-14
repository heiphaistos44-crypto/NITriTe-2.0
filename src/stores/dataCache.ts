/**
 * Cache global de données préchargées au démarrage.
 * Les pages lisent depuis ce store au lieu de re-invoquer Tauri.
 * Résultat : navigation instantanée après le splash screen.
 */
import { defineStore } from "pinia";
import { ref } from "vue";

export const useDataCache = defineStore("dataCache", () => {
  // Stockage clé → donnée (clé = nom de la commande Tauri)
  const cache = ref<Record<string, unknown>>({});
  // Commandes actuellement en cours de chargement (évite les doublons)
  const loading = ref<Set<string>>(new Set());

  function set(key: string, data: unknown) {
    cache.value[key] = data;
  }

  function get<T>(key: string): T | undefined {
    return cache.value[key] as T | undefined;
  }

  function has(key: string): boolean {
    return key in cache.value;
  }

  function invalidate(key: string) {
    delete cache.value[key];
  }

  return { cache, loading, set, get, has, invalidate };
});
