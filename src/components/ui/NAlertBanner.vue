<script setup lang="ts">
import { computed } from 'vue';
import { activeAlerts, dismissAlert, dismissAll } from '@/composables/useProactiveAlerts';

const visible = computed(() => activeAlerts.value.filter(a => !a.dismissed));
const criticals = computed(() => visible.value.filter(a => a.severity === 'critical'));
const warnings = computed(() => visible.value.filter(a => a.severity === 'warning'));
</script>

<template>
  <transition name="alert-slide">
    <div v-if="visible.length > 0" class="alert-banner">
      <div class="alert-inner">
        <div class="alert-items">
          <div v-for="alert in visible.slice(0, 5)" :key="alert.id" class="alert-item"
            :class="alert.severity === 'critical' ? 'alert-critical' : 'alert-warning'">
            <span class="alert-icon">{{ alert.severity === 'critical' ? '🔴' : '🟡' }}</span>
            <span class="alert-msg">{{ alert.message }}</span>
            <button class="alert-dismiss" @click="dismissAlert(alert.id)">✕</button>
          </div>
          <div v-if="visible.length > 5" class="alert-more">
            +{{ visible.length - 5 }} alertes supplémentaires
          </div>
        </div>
        <button v-if="visible.length > 1" class="alert-dismiss-all" @click="dismissAll">
          Tout fermer
        </button>
      </div>
    </div>
  </transition>
</template>

<style scoped>
.alert-banner {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 9999;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
  box-shadow: 0 2px 12px rgba(0,0,0,.3);
}
.alert-inner {
  max-width: 1400px;
  margin: 0 auto;
  padding: 8px 16px;
  display: flex;
  align-items: center;
  gap: 12px;
}
.alert-items { display: flex; flex-wrap: wrap; gap: 6px; flex: 1; }
.alert-item {
  display: flex; align-items: center; gap: 6px;
  padding: 4px 10px; border-radius: 6px; font-size: 12px;
  border: 1px solid transparent;
}
.alert-critical { background: rgba(239,68,68,.1); border-color: rgba(239,68,68,.3); color: #fca5a5; }
.alert-warning  { background: rgba(245,158,11,.1); border-color: rgba(245,158,11,.3); color: #fcd34d; }
.alert-msg { font-weight: 500; }
.alert-dismiss { background: none; border: none; cursor: pointer; opacity: .5; color: inherit; padding: 0 2px; font-size: 12px; }
.alert-dismiss:hover { opacity: 1; }
.alert-dismiss-all { font-size: 11px; padding: 4px 10px; border: 1px solid var(--border); border-radius: 6px; background: var(--bg-tertiary); color: var(--text-secondary); cursor: pointer; white-space: nowrap; flex-shrink: 0; }
.alert-dismiss-all:hover { border-color: var(--border-hover); }
.alert-more { font-size: 11px; color: var(--text-muted); align-self: center; }
.alert-slide-enter-active, .alert-slide-leave-active { transition: transform .3s, opacity .3s; }
.alert-slide-enter-from, .alert-slide-leave-to { transform: translateY(-100%); opacity: 0; }
</style>
