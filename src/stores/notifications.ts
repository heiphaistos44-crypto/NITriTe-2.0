import { defineStore } from "pinia";
import { ref } from "vue";

export interface Toast {
  id: string;
  type: "success" | "error" | "warning" | "info";
  title: string;
  message?: string;
  duration?: number;
}

export const useNotificationStore = defineStore("notifications", () => {
  const toasts = ref<Toast[]>([]);

  function addToast(toast: Omit<Toast, "id">) {
    const id = crypto.randomUUID();
    const entry: Toast = { ...toast, id };
    toasts.value.push(entry);

    const duration = toast.duration ?? 5000;
    setTimeout(() => removeToast(id), duration);
    return id;
  }

  function removeToast(id: string) {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  }

  function success(title: string, message?: string) {
    return addToast({ type: "success", title, message });
  }

  function error(title: string, message?: string) {
    return addToast({ type: "error", title, message });
  }

  function warning(title: string, message?: string) {
    return addToast({ type: "warning", title, message });
  }

  function info(title: string, message?: string) {
    return addToast({ type: "info", title, message });
  }

  return { toasts, addToast, removeToast, success, error, warning, info };
});
