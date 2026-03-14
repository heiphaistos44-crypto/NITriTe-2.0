/**
 * Wrapper autour de invoke() qui sert les données depuis le cache de démarrage.
 * Si la commande a été préchargée, retour immédiat (0ms).
 * Sinon, invoke Tauri normal + mise en cache pour les prochains appels.
 */
import { invoke } from "@tauri-apps/api/core";
import { useDataCache } from "@/stores/dataCache";

export async function cachedInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const cacheStore = useDataCache();
  // Clé unique par commande + arguments
  const key = args ? `${cmd}::${JSON.stringify(args)}` : cmd;

  if (cacheStore.has(key)) {
    return cacheStore.get<T>(key) as T;
  }

  const result = await invoke<T>(cmd, args);
  cacheStore.set(key, result);
  return result;
}

/**
 * Invalide le cache d'une commande et re-fetche les données fraîches.
 * À utiliser après une action qui modifie l'état (nettoyage, installation, etc.)
 */
export async function refreshCached<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const cacheStore = useDataCache();
  const key = args ? `${cmd}::${JSON.stringify(args)}` : cmd;
  cacheStore.invalidate(key);
  return cachedInvoke<T>(cmd, args);
}
