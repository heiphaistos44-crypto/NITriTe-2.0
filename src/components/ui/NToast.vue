<script setup lang="ts">
import { useNotificationStore, type Toast } from "@/stores/notifications";
import { CheckCircle, XCircle, AlertTriangle, Info, X } from "lucide-vue-next";
import { computed } from "vue";

const store = useNotificationStore();

const iconMap = {
  success: CheckCircle,
  error: XCircle,
  warning: AlertTriangle,
  info: Info,
};
</script>

<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast">
        <div v-for="toast in store.toasts" :key="toast.id" class="toast" :class="`toast--${toast.type}`">
          <component :is="iconMap[toast.type]" :size="18" class="toast-icon" />
          <div class="toast-content">
            <div class="toast-title">{{ toast.title }}</div>
            <div v-if="toast.message" class="toast-message">{{ toast.message }}</div>
          </div>
          <button class="toast-close" @click="store.removeToast(toast.id)">
            <X :size="14" />
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-container {
  position: fixed;
  top: 12px;
  right: 12px;
  z-index: 10000;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 380px;
}

.toast {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px 14px;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  box-shadow: var(--shadow-xl);
  animation: toast-in 300ms ease forwards;
}

.toast--success { border-left: 3px solid var(--success); }
.toast--error { border-left: 3px solid var(--danger); }
.toast--warning { border-left: 3px solid var(--warning); }
.toast--info { border-left: 3px solid var(--info); }

.toast--success .toast-icon { color: var(--success); }
.toast--error .toast-icon { color: var(--danger); }
.toast--warning .toast-icon { color: var(--warning); }
.toast--info .toast-icon { color: var(--info); }

.toast-icon { flex-shrink: 0; margin-top: 1px; }

.toast-content { flex: 1; min-width: 0; }

.toast-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.toast-message {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 2px;
}

.toast-close {
  flex-shrink: 0;
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px;
  border-radius: 4px;
  transition: all var(--transition-fast);
}

.toast-close:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.toast-enter-active { animation: toast-in 300ms ease; }
.toast-leave-active { animation: toast-out 300ms ease forwards; }
</style>
