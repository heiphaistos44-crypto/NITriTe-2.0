import { ref } from "vue";

export function useClipboard() {
  const copied = ref(false);
  let timeout: ReturnType<typeof setTimeout> | null = null;

  async function copy(text: string): Promise<boolean> {
    try {
      await navigator.clipboard.writeText(text);

      if (timeout) clearTimeout(timeout);
      copied.value = true;
      timeout = setTimeout(() => {
        copied.value = false;
      }, 2000);

      return true;
    } catch (e) {
      console.error("[useClipboard] copy failed:", e);
      copied.value = false;
      return false;
    }
  }

  return { copy, copied };
}
