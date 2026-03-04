import { ref, computed, type Ref } from "vue";

export function useSearch<T>(items: Ref<T[]>, searchFields: (keyof T)[]) {
  const query = ref("");

  const filtered = computed(() => {
    const q = query.value.trim().toLowerCase();
    if (!q) return items.value;

    return items.value.filter((item) =>
      searchFields.some((field) => {
        const val = item[field];
        if (val == null) return false;
        return String(val).toLowerCase().includes(q);
      })
    );
  });

  const hasResults = computed(() => filtered.value.length > 0);

  function clear() {
    query.value = "";
  }

  return { query, filtered, hasResults, clear };
}
