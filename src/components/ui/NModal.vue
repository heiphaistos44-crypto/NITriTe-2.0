<script setup lang="ts">
import { X } from "lucide-vue-next";

defineProps<{ open: boolean; title?: string; width?: string }>();
defineEmits<{ close: [] }>();
</script>

<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="open" class="modal-backdrop" @click.self="$emit('close')">
        <div class="modal animate-scale-in" :style="{ maxWidth: width ?? '480px' }">
          <div v-if="title" class="modal-header">
            <h3>{{ title }}</h3>
            <button class="modal-close" @click="$emit('close')">
              <X :size="18" />
            </button>
          </div>
          <div class="modal-body">
            <slot />
          </div>
          <div v-if="$slots.footer" class="modal-footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  z-index: 9000;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px) saturate(0.8);
  -webkit-backdrop-filter: blur(8px) saturate(0.8);
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-hover);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl), 0 0 0 1px rgba(255,255,255,0.04);
  width: 90%;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
  background: linear-gradient(135deg, var(--bg-tertiary) 0%, var(--bg-secondary) 100%);
  border-radius: var(--radius-xl) var(--radius-xl) 0 0;
}

.modal-header h3 {
  font-size: 15px;
  font-weight: 700;
  letter-spacing: -0.1px;
}

.modal-close {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.modal-close:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.modal-body {
  padding: 20px;
  overflow-y: auto;
}

.modal-footer {
  padding: 12px 20px;
  border-top: 1px solid var(--border);
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 200ms ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
