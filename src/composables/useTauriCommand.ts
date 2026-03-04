import { ref, type Ref } from "vue";

interface TauriCommandReturn<T> {
  data: Ref<T | null>;
  loading: Ref<boolean>;
  error: Ref<string | null>;
  execute: (args?: Record<string, unknown>) => Promise<T | null>;
}

const isTauri = (): boolean => {
  return "__TAURI_INTERNALS__" in window;
};

export function useTauriCommand<T>(
  commandName: string,
  fallbackData?: T
): TauriCommandReturn<T> {
  const data = ref<T | null>(null) as Ref<T | null>;
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function execute(args?: Record<string, unknown>): Promise<T | null> {
    loading.value = true;
    error.value = null;

    try {
      if (!isTauri()) {
        if (fallbackData !== undefined) {
          data.value = fallbackData;
          return fallbackData;
        }
        throw new Error(`Tauri non disponible pour la commande "${commandName}"`);
      }

      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke<T>(commandName, args);
      data.value = result;
      return result;
    } catch (e: any) {
      error.value = e?.toString() ?? "Erreur inconnue";
      if (!isTauri() && fallbackData !== undefined) {
        data.value = fallbackData;
        return fallbackData;
      }
      return null;
    } finally {
      loading.value = false;
    }
  }

  return { data, loading, error, execute };
}
