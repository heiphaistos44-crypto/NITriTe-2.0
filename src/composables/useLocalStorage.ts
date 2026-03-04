import { ref, watch, type Ref } from "vue";

function read<T>(key: string, defaultValue: T): T {
  try {
    const raw = localStorage.getItem(key);
    if (raw === null) return defaultValue;
    return JSON.parse(raw) as T;
  } catch {
    return defaultValue;
  }
}

export function useLocalStorage<T>(key: string, defaultValue: T) {
  const value = ref<T>(read(key, defaultValue)) as Ref<T>;

  watch(
    value,
    (val) => {
      try {
        localStorage.setItem(key, JSON.stringify(val));
      } catch (e) {
        console.error(`[useLocalStorage] write failed for key "${key}":`, e);
      }
    },
    { deep: true }
  );

  function set(val: T) {
    value.value = val;
  }

  function remove() {
    localStorage.removeItem(key);
    value.value = defaultValue;
  }

  return { value, set, remove };
}
