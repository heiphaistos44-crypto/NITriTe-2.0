<script setup lang="ts">
import NButton from "@/components/ui/NButton.vue";
import { AlertTriangle, Info, Trash2 } from "lucide-vue-next";

const props = withDefaults(defineProps<{
  modelValue: boolean;
  title?: string;
  message?: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: "danger" | "warning" | "info";
}>(), {
  title: "Confirmation",
  message: "Etes-vous sur ?",
  confirmLabel: "Confirmer",
  cancelLabel: "Annuler",
  variant: "warning",
});

const emit = defineEmits<{
  "update:modelValue": [v: boolean];
  confirm: [];
  cancel: [];
}>();

const iconMap = { danger: Trash2, warning: AlertTriangle, info: Info };

function confirm() {
  emit("confirm");
  emit("update:modelValue", false);
}

function cancel() {
  emit("cancel");
  emit("update:modelValue", false);
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog">
      <div v-if="modelValue" class="dialog-overlay" @click.self="cancel">
        <div class="dialog-box">
          <div class="dialog-icon" :class="`icon-${variant}`">
            <component :is="iconMap[variant]" :size="24" />
          </div>
          <h3 class="dialog-title">{{ title }}</h3>
          <p class="dialog-message">{{ message }}</p>
          <div class="dialog-actions">
            <NButton variant="ghost" size="sm" @click="cancel">{{ cancelLabel }}</NButton>
            <NButton :variant="variant === 'danger' ? 'danger' : 'primary'" size="sm" @click="confirm">
              {{ confirmLabel }}
            </NButton>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 9500;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
}

.dialog-box {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 28px;
  max-width: 400px;
  width: 90%;
  text-align: center;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.dialog-icon {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 16px;
}

.icon-danger { background: var(--danger-muted); color: var(--danger); }
.icon-warning { background: var(--warning-muted); color: var(--warning); }
.icon-info { background: var(--info-muted); color: var(--info); }

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.dialog-message {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 20px;
  line-height: 1.5;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: center;
}

.dialog-enter-active { transition: opacity 150ms ease; }
.dialog-leave-active { transition: opacity 100ms ease; }
.dialog-enter-from, .dialog-leave-to { opacity: 0; }
</style>
