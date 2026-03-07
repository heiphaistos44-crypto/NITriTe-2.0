<script setup lang="ts">
import { ref, onMounted } from "vue";
import NBadge from "@/components/ui/NBadge.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import DiagBanner from "@/components/ui/DiagBanner.vue";
import { Users, Shield, Lock, UserX, Key } from "lucide-vue-next";

interface UserAccount {
  name: string; full_name: string; description: string;
  enabled: boolean; last_logon: string;
  password_required: boolean; is_admin: boolean; account_type: string;
}
interface LocalGroup {
  name: string; description: string; member_count: number; members: string[];
}
interface PasswordPolicy {
  min_length: number; max_age_days: number; min_age_days: number;
  complexity: boolean; lockout_threshold: number; lockout_duration: number;
  history_count: number;
}
interface AccountsInfo {
  users: UserAccount[]; groups: LocalGroup[];
  policy: PasswordPolicy; total_enabled: number; total_admin: number;
}

const data = ref<AccountsInfo | null>(null);
const loading = ref(true);
const error = ref("");

onMounted(async () => {
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    data.value = await invoke<AccountsInfo>("get_user_accounts");
  } catch (e: any) { error.value = e?.toString() ?? "Erreur"; }
  finally { loading.value = false; }
});
</script>

<template>
  <div class="diag-tab-content">
    <DiagBanner :icon="Users" title="Comptes Utilisateurs" desc="Comptes locaux, groupes et politique de mots de passe" color="purple" />

    <div v-if="loading" class="diag-loading">
      <NSpinner :size="16" class="diag-spinner" /><span>Chargement comptes...</span>
    </div>
    <div v-else-if="error" style="color:var(--error)">⚠ {{ error }}</div>
    <div v-else-if="data" style="display:flex;flex-direction:column;gap:14px">

      <!-- Résumé -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Résumé</p>
        <div class="info-grid">
          <div class="info-row"><span>Comptes actifs</span>
            <NBadge variant="info">{{ data.total_enabled }} / {{ data.users.length }}</NBadge>
          </div>
          <div class="info-row"><span>Administrateurs</span>
            <NBadge :variant="data.total_admin > 2 ? 'warning' : 'success'">{{ data.total_admin }}</NBadge>
          </div>
        </div>
      </div>

      <!-- Comptes utilisateurs -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">
          <Users :size="13" style="display:inline;margin-right:4px" />Comptes locaux ({{ data.users.length }})
        </p>
        <div style="overflow-x:auto">
          <table style="width:100%;border-collapse:collapse;font-size:12px">
            <thead>
              <tr style="background:var(--bg-secondary)">
                <th style="padding:6px 10px;text-align:left;color:var(--text-secondary);font-weight:500">Compte</th>
                <th style="padding:6px 10px;text-align:left;color:var(--text-secondary);font-weight:500">Nom complet</th>
                <th style="padding:6px 10px;text-align:left;color:var(--text-secondary);font-weight:500">Type</th>
                <th style="padding:6px 10px;text-align:left;color:var(--text-secondary);font-weight:500">Statut</th>
                <th style="padding:6px 10px;text-align:left;color:var(--text-secondary);font-weight:500">Admin</th>
                <th style="padding:6px 10px;text-align:left;color:var(--text-secondary);font-weight:500">Dernière connexion</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="u in data.users" :key="u.name"
                style="border-bottom:1px solid var(--border)"
                :style="{ opacity: u.enabled ? 1 : 0.5 }">
                <td style="padding:6px 10px">
                  <div style="display:flex;align-items:center;gap:6px">
                    <component :is="u.is_admin ? Shield : Users" :size="12"
                      :style="{ color: u.is_admin ? 'var(--accent)' : 'var(--text-muted)' }" />
                    <code>{{ u.name }}</code>
                  </div>
                </td>
                <td style="padding:6px 10px;color:var(--text-secondary)">{{ u.full_name || "—" }}</td>
                <td style="padding:6px 10px">
                  <NBadge :variant="u.account_type === 'Microsoft' ? 'info' : u.account_type === 'Azure AD' ? 'success' : 'neutral'">
                    {{ u.account_type }}
                  </NBadge>
                </td>
                <td style="padding:6px 10px">
                  <NBadge :variant="u.enabled ? 'success' : 'neutral'">{{ u.enabled ? 'Actif' : 'Désactivé' }}</NBadge>
                </td>
                <td style="padding:6px 10px">
                  <NBadge v-if="u.is_admin" variant="warning"><Shield :size="10" /> Admin</NBadge>
                  <span v-else class="muted">—</span>
                </td>
                <td style="padding:6px 10px;color:var(--text-secondary);font-size:11px">{{ u.last_logon }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Politique de mots de passe -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">
          <Key :size="13" style="display:inline;margin-right:4px" />Politique de mots de passe
        </p>
        <div class="info-grid">
          <div class="info-row"><span>Longueur minimale</span>
            <NBadge :variant="data.policy.min_length >= 8 ? 'success' : data.policy.min_length > 0 ? 'warning' : 'danger'">
              {{ data.policy.min_length > 0 ? data.policy.min_length + ' caractères' : 'Non définie ⚠' }}
            </NBadge>
          </div>
          <div class="info-row"><span>Complexité requise</span>
            <NBadge :variant="data.policy.complexity ? 'success' : 'warning'">
              {{ data.policy.complexity ? 'Oui' : 'Non' }}
            </NBadge>
          </div>
          <div class="info-row"><span>Expiration max</span>
            <span>{{ data.policy.max_age_days > 0 ? data.policy.max_age_days + ' jours' : 'Jamais' }}</span>
          </div>
          <div class="info-row"><span>Âge minimum</span>
            <span>{{ data.policy.min_age_days > 0 ? data.policy.min_age_days + ' jours' : 'Aucun' }}</span>
          </div>
          <div class="info-row"><span>Verrouillage après</span>
            <NBadge :variant="data.policy.lockout_threshold > 0 ? 'success' : 'warning'">
              {{ data.policy.lockout_threshold > 0 ? data.policy.lockout_threshold + ' tentatives' : 'Désactivé ⚠' }}
            </NBadge>
          </div>
          <div class="info-row" v-if="data.policy.lockout_threshold > 0"><span>Durée verrouillage</span>
            <span>{{ data.policy.lockout_duration > 0 ? data.policy.lockout_duration + ' min' : 'Admin requis' }}</span>
          </div>
          <div class="info-row"><span>Historique mots de passe</span>
            <span>{{ data.policy.history_count > 0 ? data.policy.history_count + ' mémorisés' : 'Aucun' }}</span>
          </div>
        </div>
      </div>

      <!-- Groupes locaux -->
      <div class="diag-section">
        <p class="diag-section-label" style="margin:0 0 8px 0">Groupes locaux ({{ data.groups.length }})</p>
        <div v-for="g in data.groups" :key="g.name"
          style="padding:8px 0;border-bottom:1px solid var(--border)">
          <div style="display:flex;align-items:center;gap:8px;margin-bottom:4px">
            <Shield :size="12" style="color:var(--accent);flex-shrink:0" />
            <strong style="font-size:12px">{{ g.name }}</strong>
            <NBadge variant="neutral" style="font-size:10px">{{ g.member_count }} membre(s)</NBadge>
            <span class="muted" style="font-size:11px">{{ g.description }}</span>
          </div>
          <div v-if="g.members.length" style="display:flex;gap:4px;flex-wrap:wrap;padding-left:20px">
            <code v-for="(m, i) in g.members" :key="i"
              style="font-size:10px;background:var(--bg-secondary);padding:1px 5px;border-radius:3px">{{ m }}</code>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
