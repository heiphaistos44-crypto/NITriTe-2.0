/**
 * Wrapper autour de invoke() qui sert les données depuis le cache de démarrage.
 * Si la commande a été préchargée, retour immédiat (0ms).
 * Sinon, invoke Tauri normal + mise en cache pour les prochains appels.
 * Déduplication : deux appels simultanés identiques partagent la même Promise.
 */
import { invoke } from "@/utils/invoke";
import { useDataCache } from "@/stores/dataCache";

// Requêtes en vol — évite de lancer deux invoke identiques en parallèle
const pendingRequests = new Map<string, Promise<unknown>>();

export async function cachedInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const cacheStore = useDataCache();
  const key = args ? `${cmd}::${JSON.stringify(args)}` : cmd;

  if (cacheStore.has(key)) {
    return cacheStore.get<T>(key) as T;
  }

  // Déduplique les requêtes en vol
  if (pendingRequests.has(key)) {
    return pendingRequests.get(key) as Promise<T>;
  }

  const request = invoke<T>(cmd, args).then((result) => {
    cacheStore.set(key, result);
    pendingRequests.delete(key);
    return result;
  }).catch((err) => {
    pendingRequests.delete(key);
    throw err;
  });

  pendingRequests.set(key, request);
  return request;
}

/**
 * Invalide le cache d'une commande et re-fetche les données fraîches.
 * À utiliser après une action qui modifie l'état (nettoyage, installation, etc.)
 */
export async function refreshCached<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const cacheStore = useDataCache();
  const key = args ? `${cmd}::${JSON.stringify(args)}` : cmd;
  cacheStore.invalidate(key);
  pendingRequests.delete(key);
  return cachedInvoke<T>(cmd, args);
}
