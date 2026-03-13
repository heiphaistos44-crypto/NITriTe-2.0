<script setup lang="ts">
import { ref, computed, nextTick, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NDropdown from "@/components/ui/NDropdown.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Bot, Send, Terminal, Play, CheckCircle, XCircle, RefreshCw,
  Cpu, MessageSquare, Settings, FolderOpen, Square, Download, Zap, Server,
} from "lucide-vue-next";

const notify = useNotificationStore();

// ─── Backend selector ──────────────────────────────────────────────────────────
const backend = ref<"llamacpp" | "ollama">("llamacpp");

// ─── llama.cpp portable ────────────────────────────────────────────────────────
interface GgufModel { name: string; path: string; size_gb: number }
const ggufModels     = ref<GgufModel[]>([]);
const selectedGguf   = ref("");
const serverBin      = ref("");
const serverRunning  = ref(false);
const serverChecking = ref(false);
const startingServer = ref(false);

async function detectServer() {
  try { const bin = await invoke<string | null>("ai_find_llamacpp_server"); if (bin) serverBin.value = bin; } catch {}
}

async function scanGgufModels() {
  try {
    const list = await invoke<GgufModel[]>("ai_list_gguf_models", { modelsDir: null });
    ggufModels.value = list;
    if (list.length && !selectedGguf.value) selectedGguf.value = list[0].path;
  } catch {}
}

async function browseModelFile() {
  try {
    const path = await open({ filters: [{ name: "GGUF Model", extensions: ["gguf"] }] });
    if (path && typeof path === "string") {
      selectedGguf.value = path;
      const name = path.split(/[\\/]/).pop() ?? path;
      if (!ggufModels.value.find(m => m.path === path)) {
        ggufModels.value.unshift({ name, path, size_gb: 0 });
      }
    }
  } catch { notify.error("Sélection fichier", "Erreur lors de la sélection du modèle."); }
}

async function startServer() {
  if (!selectedGguf.value) { notify.warning("Modèle requis", "Sélectionnez un fichier .gguf"); return; }
  startingServer.value = true;
  try {
    await invoke("ai_start_llamacpp", { modelPath: selectedGguf.value });
    notify.info("llama.cpp", "Démarrage du serveur...");
    for (let i = 0; i < 30; i++) {
      await new Promise(r => setTimeout(r, 1000));
      const ok = await invoke<boolean>("ai_llamacpp_status");
      if (ok) { serverRunning.value = true; notify.success("llama.cpp", "Serveur prêt !"); break; }
    }
    if (!serverRunning.value) notify.warning("llama.cpp", "Serveur démarré mais pas encore prêt.");
  } catch (e: any) { notify.error("llama.cpp", String(e)); }
  startingServer.value = false;
}

async function stopServer() {
  try { await invoke("ai_stop_llamacpp"); serverRunning.value = false; notify.info("llama.cpp", "Serveur arrêté."); }
  catch (e: any) { notify.error("llama.cpp", String(e)); }
}

async function checkLlamacppStatus() {
  serverChecking.value = true;
  serverRunning.value = await invoke<boolean>("ai_llamacpp_status").catch(() => false);
  serverChecking.value = false;
}

// ─── Ollama ────────────────────────────────────────────────────────────────────
const ollamaConnected      = ref(false);
const ollamaChecking       = ref(false);
const ollamaModels         = ref<{ value: string; label: string }[]>([]);
const selectedOllama       = ref("");
const ollamaModelsLoading  = ref(false);

async function checkOllamaStatus() {
  ollamaChecking.value = true;
  ollamaConnected.value = await invoke<boolean>("ai_check").catch(() => false);
  ollamaChecking.value = false;
}

async function loadOllamaModels() {
  ollamaModelsLoading.value = true;
  try {
    const list = await invoke<{ name: string; size_gb: number }[]>("ai_list_models");
    ollamaModels.value = list.map(m => ({ value: m.name, label: `${m.name} (${m.size_gb.toFixed(1)} GB)` }));
    if (list.length && !selectedOllama.value) selectedOllama.value = list[0].name;
  } catch {
    ollamaModels.value = [{ value: "llama3.2", label: "llama3.2" }, { value: "mistral", label: "mistral" }];
    if (!selectedOllama.value) selectedOllama.value = "llama3.2";
  }
  ollamaModelsLoading.value = false;
}

// ─── Chat ──────────────────────────────────────────────────────────────────────
const systemPromptOptions = [
  { value: "assistant_it", label: "Assistant IT" },
  { value: "diagnostic",   label: "Diagnostic système" },
  { value: "custom",       label: "Personnalisé" },
];
const selectedPromptType = ref("assistant_it");
const customSystemPrompt = ref("");
const systemPromptPresets: Record<string, string> = {
  assistant_it: "Tu es un assistant IT expert en maintenance Windows. Tu réponds en français de manière concise et technique. Quand tu proposes une commande, encadre-la avec ```cmd ou ```powershell.",
  diagnostic:   "Tu es un spécialiste du diagnostic système Windows. Analyse les problèmes décrits, identifie les causes probables et propose des solutions concrètes avec les commandes nécessaires.",
};
const activeSystemPrompt = computed(() =>
  selectedPromptType.value === "custom" ? customSystemPrompt.value : (systemPromptPresets[selectedPromptType.value] ?? "")
);

interface ChatMessage { role: "user" | "assistant"; content: string; command?: string; commandResult?: string; commandRunning?: boolean }
const messages  = ref<ChatMessage[]>([]);
const userInput = ref("");
const sending   = ref(false);
const chatRef   = ref<HTMLElement | null>(null);

const ggufOptions = computed(() => ggufModels.value.map(m => ({
  value: m.path,
  label: `${m.name} (${m.size_gb > 0 ? m.size_gb.toFixed(1) + " GB" : "?"})`,
})));

const activeModel = computed(() => {
  if (backend.value === "llamacpp") {
    return ggufModels.value.find(m => m.path === selectedGguf.value)?.name
      ?? selectedGguf.value.split(/[\\/]/).pop() ?? "";
  }
  return selectedOllama.value;
});

function scrollChat() {
  nextTick(() => { if (chatRef.value) chatRef.value.scrollTop = chatRef.value.scrollHeight; });
}
function extractCommand(text: string): string | null {
  const m = text.match(/```(?:cmd|powershell|bash|shell)\n([\s\S]*?)```/);
  return m ? m[1].trim() : null;
}

async function sendMessage() {
  const text = userInput.value.trim();
  if (!text || sending.value) return;
  if (backend.value === "llamacpp" && !serverRunning.value) {
    notify.warning("Serveur arrêté", "Démarrez le serveur llama.cpp d'abord."); return;
  }
  messages.value.push({ role: "user", content: text });
  userInput.value = "";
  sending.value = true;
  scrollChat();
  const history = messages.value.slice(0, -1).slice(-20).map(m => ({ role: m.role, content: m.content }));
  try {
    const response = await invoke<string>("ai_query", {
      model: activeModel.value || undefined,
      prompt: text,
      systemPrompt: activeSystemPrompt.value || null,
      history: history.length ? history : null,
    });
    messages.value.push({ role: "assistant", content: response, command: extractCommand(response) ?? undefined });
  } catch (err: any) {
    const hint = backend.value === "llamacpp"
      ? "Assurez-vous que le serveur llama.cpp est démarré."
      : "Assurez-vous qu'Ollama est lancé (`ollama serve`).";
    messages.value.push({ role: "assistant", content: `⚠ Erreur IA.\n\n${hint}\n\nDétail : ${String(err)}` });
  } finally { sending.value = false; scrollChat(); }
}

async function executeCommand(i: number) {
  const msg = messages.value[i];
  if (!msg.command) return;
  msg.commandRunning = true;
  try {
    const r = await invoke<{ command: string; success: boolean; stdout: string; stderr: string }>("ai_execute_command", { command: msg.command });
    msg.commandResult = r.stdout || r.stderr || "Aucune sortie";
    notify.success("Commande exécutée", r.success ? "Succès" : "Échec");
  } catch { msg.commandResult = "Simulation dev — OK"; }
  finally { msg.commandRunning = false; scrollChat(); }
}

function clearChat() { messages.value = []; }
function handleKeydown(e: KeyboardEvent) { if (e.key === "Enter" && !e.shiftKey) { e.preventDefault(); sendMessage(); } }
function formatMessage(text: string): string {
  return text
    .replace(/```(\w*)\n([\s\S]*?)```/g, '<pre class="code-block"><code>$2</code></pre>')
    .replace(/`([^`]+)`/g, '<code class="inline-code">$1</code>')
    .replace(/\n/g, "<br>");
}

onMounted(async () => {
  await detectServer();
  await Promise.all([scanGgufModels(), checkLlamacppStatus(), checkOllamaStatus(), loadOllamaModels()]);
});
</script>

<template>
  <div class="ai-agents">
    <div class="page-header">
      <div>
        <h1>Assistant IA</h1>
        <p class="page-subtitle">IA locale portable — aucune installation requise</p>
      </div>
    </div>

    <!-- Backend selector -->
    <div class="backend-selector">
      <button :class="['backend-btn', { active: backend === 'llamacpp' }]" @click="backend = 'llamacpp'">
        <Zap :size="18" style="flex-shrink:0" />
        <div>
          <div class="bk-title">llama.cpp Portable</div>
          <div class="bk-sub">Un binaire + fichier .gguf — sans installation</div>
        </div>
        <NBadge variant="success" style="margin-left:auto;font-size:10px;flex-shrink:0">Recommandé</NBadge>
      </button>
      <button :class="['backend-btn', { active: backend === 'ollama' }]" @click="backend = 'ollama'">
        <Server :size="18" style="flex-shrink:0" />
        <div>
          <div class="bk-title">Ollama</div>
          <div class="bk-sub">Requiert Ollama installé sur le système</div>
        </div>
        <div class="ollama-dot" :class="{ ok: ollamaConnected }" />
      </button>
    </div>

    <div class="ai-layout">
      <!-- Sidebar -->
      <div class="ai-sidebar">
        <!-- llama.cpp config -->
        <NCard v-if="backend === 'llamacpp'">
          <template #header>
            <div class="section-header"><Cpu :size="16" /><span>Modèle GGUF</span></div>
          </template>
          <div class="config-section">
            <NDropdown v-if="ggufOptions.length" :options="ggufOptions" v-model="selectedGguf" placeholder="Sélectionner un modèle..." />
            <div v-else class="hint-box">
              Aucun fichier .gguf dans <code>models/</code>.<br>
              <span style="font-size:11px">Téléchargez un modèle GGUF sur HuggingFace et placez-le dans <code>models/</code>.</span>
            </div>
            <div style="display:flex;gap:8px">
              <NButton variant="secondary" size="sm" style="flex:1" @click="browseModelFile"><FolderOpen :size="13" /> Parcourir</NButton>
              <NButton variant="secondary" size="sm" @click="scanGgufModels"><RefreshCw :size="13" /></NButton>
            </div>
            <div class="server-status" :class="{ running: serverRunning }">
              <template v-if="serverChecking"><NSpinner :size="13" /> Vérification...</template>
              <template v-else-if="serverRunning"><CheckCircle :size="13" /> Serveur actif</template>
              <template v-else><XCircle :size="13" /> Serveur arrêté</template>
            </div>
            <div style="display:flex;gap:8px">
              <NButton v-if="!serverRunning" variant="primary" size="sm" style="flex:1"
                :disabled="startingServer || !selectedGguf" @click="startServer">
                <NSpinner v-if="startingServer" :size="13" /><Play v-else :size="13" />
                {{ startingServer ? "Démarrage..." : "Démarrer" }}
              </NButton>
              <NButton v-else variant="danger" size="sm" style="flex:1" @click="stopServer">
                <Square :size="13" /> Arrêter
              </NButton>
              <NButton variant="ghost" size="sm" @click="checkLlamacppStatus"><RefreshCw :size="13" /></NButton>
            </div>
            <div class="divider-line" />
            <div class="download-section">
              <div class="dl-label"><Download :size="11" /> Modèles recommandés (Q4_K_M)</div>
              <a class="model-link" href="https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf" target="_blank">Phi-3 Mini — 2.2 GB</a>
              <a class="model-link" href="https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF" target="_blank">TinyLlama — 0.6 GB</a>
              <a class="model-link" href="https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF" target="_blank">Mistral 7B — 4.1 GB</a>
            </div>
            <div class="server-info">
              <span>Binaire :</span>
              <span v-if="serverBin" class="ok-text" :title="serverBin">{{ serverBin.split(/[\\/]/).pop() }}</span>
              <a v-else class="model-link" href="https://github.com/ggml-org/llama.cpp/releases" target="_blank">llama-server.exe</a>
              <span v-if="!serverBin" style="font-size:10px;color:var(--text-muted)">→ placer dans <code>logiciel/AI/</code></span>
            </div>
          </div>
        </NCard>

        <!-- Ollama config -->
        <NCard v-else>
          <template #header>
            <div class="section-header">
              <Cpu :size="16" /><span>Modèle Ollama</span>
              <div style="margin-left:auto" :style="{ color: ollamaConnected ? 'var(--success)' : 'var(--danger)' }">
                <NSpinner v-if="ollamaChecking" :size="13" />
                <CheckCircle v-else-if="ollamaConnected" :size="13" />
                <XCircle v-else :size="13" />
              </div>
            </div>
          </template>
          <div class="config-section">
            <NDropdown :options="ollamaModels" v-model="selectedOllama" placeholder="Sélectionner..." />
            <NButton variant="secondary" size="sm" :loading="ollamaModelsLoading" @click="loadOllamaModels">
              <RefreshCw :size="13" /> Rafraîchir
            </NButton>
            <div v-if="!ollamaConnected" class="hint-box">
              Ollama non détecté. Lancez <code>ollama serve</code> puis installez un modèle avec <code>ollama pull llama3.2</code>.
            </div>
          </div>
        </NCard>

        <!-- Prompt système -->
        <NCard>
          <template #header><div class="section-header"><Settings :size="16" /><span>Prompt système</span></div></template>
          <div class="config-section">
            <NDropdown :options="systemPromptOptions" v-model="selectedPromptType" placeholder="Type de prompt" />
            <textarea v-if="selectedPromptType === 'custom'" v-model="customSystemPrompt"
              class="prompt-textarea" placeholder="Entrez votre prompt système..." rows="4" />
            <div v-else class="prompt-preview">{{ activeSystemPrompt }}</div>
          </div>
        </NCard>
      </div>

      <!-- Chat -->
      <NCard class="chat-card">
        <template #header>
          <div class="section-header">
            <MessageSquare :size="16" /><span>Conversation</span>
            <NButton v-if="messages.length" variant="secondary" size="sm" @click="clearChat" style="margin-left:auto">Effacer</NButton>
          </div>
        </template>
        <div ref="chatRef" class="chat-messages">
          <div v-if="!messages.length" class="chat-empty">
            <Bot :size="40" style="opacity:.2" />
            <p>Posez une question sur la maintenance système</p>
            <div class="suggestion-chips">
              <button class="chip" @click="userInput='Mon PC est lent, que faire ?'; sendMessage()">Mon PC est lent</button>
              <button class="chip" @click="userInput='Comment vérifier les fichiers système ?'; sendMessage()">Vérifier fichiers système</button>
              <button class="chip" @click="userInput='Comment optimiser le démarrage ?'; sendMessage()">Optimiser démarrage</button>
            </div>
          </div>
          <template v-for="(msg, i) in messages" :key="i">
            <div class="message" :class="msg.role">
              <div class="message-avatar">
                <Bot v-if="msg.role === 'assistant'" :size="16" />
                <MessageSquare v-else :size="16" />
              </div>
              <div class="message-content">
                <div class="message-role">{{ msg.role === 'assistant' ? 'Assistant' : 'Vous' }}</div>
                <div class="message-text" v-html="formatMessage(msg.content)" />
                <div v-if="msg.command" class="command-block">
                  <div class="command-header"><Terminal :size="13" /><span>Commande suggérée</span></div>
                  <pre class="command-code">{{ msg.command }}</pre>
                  <div class="command-actions">
                    <NButton variant="primary" size="sm" :loading="msg.commandRunning"
                      :disabled="msg.commandRunning || !!msg.commandResult" @click="executeCommand(i)">
                      <Play :size="13" /> Exécuter
                    </NButton>
                  </div>
                  <div v-if="msg.commandResult" class="command-result"><pre>{{ msg.commandResult }}</pre></div>
                </div>
              </div>
            </div>
          </template>
          <div v-if="sending" class="message assistant">
            <div class="message-avatar"><Bot :size="16" /></div>
            <div class="message-content" style="display:flex;align-items:center;gap:8px">
              <NSpinner :size="15" /><span class="typing-text">Réflexion en cours...</span>
            </div>
          </div>
        </div>
        <div class="chat-input-area">
          <textarea v-model="userInput" class="chat-input" placeholder="Posez votre question..." rows="2" @keydown="handleKeydown" />
          <NButton variant="primary" :disabled="!userInput.trim() || sending" :loading="sending" @click="sendMessage">
            <Send :size="16" />
          </NButton>
        </div>
      </NCard>
    </div>
  </div>
</template>

<style scoped>
.ai-agents { display:flex; flex-direction:column; gap:16px; }
.page-header h1 { font-size:22px; font-weight:700; }
.page-subtitle { color:var(--text-muted); font-size:13px; margin-top:2px; }

.backend-selector { display:grid; grid-template-columns:1fr 1fr; gap:10px; }
.backend-btn { display:flex; align-items:center; gap:12px; padding:14px 16px; border:2px solid var(--border); border-radius:10px; background:var(--bg-secondary); cursor:pointer; text-align:left; transition:all .15s; color:var(--text-primary); }
.backend-btn:hover { border-color:var(--accent-muted); }
.backend-btn.active { border-color:var(--accent-primary); background:color-mix(in srgb,var(--accent-primary) 8%,var(--bg-secondary)); }
.bk-title { font-weight:600; font-size:13px; }
.bk-sub { font-size:11px; color:var(--text-muted); margin-top:2px; }
.ollama-dot { width:10px; height:10px; border-radius:50%; background:var(--danger); margin-left:auto; flex-shrink:0; }
.ollama-dot.ok { background:var(--success); }

.ai-layout { display:grid; grid-template-columns:290px 1fr; gap:16px; align-items:start; }
@media (max-width:1000px) { .ai-layout { grid-template-columns:1fr; } }
.ai-sidebar { display:flex; flex-direction:column; gap:14px; }
.section-header { display:flex; align-items:center; gap:8px; width:100%; }
.config-section { display:flex; flex-direction:column; gap:10px; }

.hint-box { font-size:12px; color:var(--text-muted); line-height:1.6; padding:10px; background:var(--bg-tertiary); border-radius:6px; }
.hint-box code { color:var(--accent-primary); background:var(--bg-primary); padding:1px 5px; border-radius:3px; font-size:11px; }
.server-status { display:flex; align-items:center; gap:6px; font-size:12px; font-weight:500; padding:7px 10px; border-radius:6px; background:var(--danger-muted); color:var(--danger); }
.server-status.running { background:var(--success-muted); color:var(--success); }
.divider-line { border-top:1px solid var(--border); margin:2px 0; }
.download-section { display:flex; flex-direction:column; gap:5px; }
.dl-label { display:flex; align-items:center; gap:5px; font-size:11px; color:var(--text-muted); font-weight:500; }
.model-link { font-size:11px; color:var(--accent-primary); text-decoration:none; }
.model-link:hover { text-decoration:underline; }
.server-info { font-size:11px; color:var(--text-muted); display:flex; align-items:center; gap:5px; flex-wrap:wrap; }
.ok-text { color:var(--success); font-family:monospace; font-size:11px; }
.server-info code { color:var(--accent-primary); font-size:10px; }
.prompt-textarea { width:100%; padding:10px; background:var(--bg-primary); border:1px solid var(--border); border-radius:var(--radius-md); color:var(--text-primary); font-family:inherit; font-size:12px; resize:vertical; outline:none; }
.prompt-textarea:focus { border-color:var(--accent-primary); }
.prompt-preview { font-size:12px; color:var(--text-muted); line-height:1.5; padding:10px; background:var(--bg-tertiary); border-radius:var(--radius-md); }

.chat-card { display:flex; flex-direction:column; }
.chat-messages { min-height:400px; max-height:540px; overflow-y:auto; padding:8px 0; display:flex; flex-direction:column; gap:12px; }
.chat-empty { display:flex; flex-direction:column; align-items:center; justify-content:center; gap:12px; height:300px; color:var(--text-muted); font-size:13px; }
.suggestion-chips { display:flex; gap:8px; flex-wrap:wrap; justify-content:center; }
.chip { padding:6px 14px; border-radius:99px; border:1px solid var(--border); background:var(--bg-tertiary); color:var(--text-secondary); font-family:inherit; font-size:12px; cursor:pointer; transition:all .15s; }
.chip:hover { border-color:var(--accent-primary); color:var(--accent-primary); }
.message { display:flex; gap:10px; padding:8px 12px; border-radius:var(--radius-md); }
.message.user { background:var(--bg-tertiary); }
.message-avatar { width:28px; height:28px; border-radius:var(--radius-md); display:flex; align-items:center; justify-content:center; flex-shrink:0; background:var(--bg-secondary); color:var(--text-muted); }
.message.assistant .message-avatar { background:var(--accent-muted); color:var(--accent-primary); }
.message-content { flex:1; min-width:0; }
.message-role { font-size:11px; font-weight:600; color:var(--text-muted); margin-bottom:4px; text-transform:uppercase; letter-spacing:.3px; }
.message-text { font-size:13px; color:var(--text-primary); line-height:1.6; }
.message-text :deep(.code-block) { background:var(--bg-primary); border:1px solid var(--border); border-radius:var(--radius-md); padding:10px 12px; margin:8px 0; font-family:"JetBrains Mono",monospace; font-size:12px; overflow-x:auto; }
.message-text :deep(.inline-code) { background:var(--bg-tertiary); padding:1px 6px; border-radius:4px; font-family:"JetBrains Mono",monospace; font-size:12px; }
.typing-text { font-size:12px; color:var(--text-muted); }
.command-block { margin-top:10px; border:1px solid var(--border); border-radius:var(--radius-md); overflow:hidden; }
.command-header { display:flex; align-items:center; gap:6px; padding:7px 12px; background:var(--bg-tertiary); font-size:11px; font-weight:500; color:var(--text-muted); text-transform:uppercase; }
.command-code { padding:10px 12px; background:var(--bg-primary); font-family:"JetBrains Mono",monospace; font-size:12px; color:var(--accent-primary); margin:0; white-space:pre-wrap; }
.command-actions { padding:8px 12px; background:var(--bg-tertiary); border-top:1px solid var(--border); }
.command-result { padding:10px 12px; border-top:1px solid var(--border); background:var(--bg-primary); }
.command-result pre { font-family:"JetBrains Mono",monospace; font-size:12px; color:var(--success); margin:0; white-space:pre-wrap; }
.chat-input-area { display:flex; gap:8px; align-items:flex-end; padding-top:12px; margin-top:12px; border-top:1px solid var(--border); }
.chat-input { flex:1; padding:10px 12px; background:var(--bg-primary); border:1px solid var(--border); border-radius:var(--radius-md); color:var(--text-primary); font-family:inherit; font-size:13px; resize:none; outline:none; }
.chat-input:focus { border-color:var(--accent-primary); }
</style>
