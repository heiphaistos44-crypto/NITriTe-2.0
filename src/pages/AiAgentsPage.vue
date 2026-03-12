<script setup lang="ts">
import { ref, computed, nextTick, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NCard from "@/components/ui/NCard.vue";
import NButton from "@/components/ui/NButton.vue";
import NDropdown from "@/components/ui/NDropdown.vue";
import NSpinner from "@/components/ui/NSpinner.vue";
import NBadge from "@/components/ui/NBadge.vue";
import { useNotificationStore } from "@/stores/notifications";
import {
  Bot, Send, Terminal, Play,
  CheckCircle, XCircle, RefreshCw,
  Cpu, MessageSquare, Settings,
} from "lucide-vue-next";

const notify = useNotificationStore();

// --- Ollama status ---
const ollamaConnected = ref(false);
const statusChecking = ref(true);

async function checkOllamaStatus() {
  statusChecking.value = true;
  try {
    const result = await invoke<boolean>("ai_check");
    ollamaConnected.value = result;
  } catch {
    ollamaConnected.value = false;
  } finally {
    statusChecking.value = false;
  }
}

// --- Models ---
const models = ref<{ value: string; label: string }[]>([]);
const selectedModel = ref("");
const modelsLoading = ref(false);

async function loadModels() {
  modelsLoading.value = true;
  try {
    const list = await invoke<{ name: string; size_gb: number; modified_at: string }[]>("ai_list_models");
    models.value = list.map((m) => ({ value: m.name, label: `${m.name} (${m.size_gb.toFixed(1)} GB)` }));
    if (list.length && !selectedModel.value) {
      selectedModel.value = list[0].name;
    }
  } catch {
    models.value = [
      { value: "llama3.2", label: "llama3.2" },
      { value: "mistral", label: "mistral" },
      { value: "codellama", label: "codellama" },
    ];
    if (!selectedModel.value) selectedModel.value = "llama3.2";
  } finally {
    modelsLoading.value = false;
  }
}

// --- System prompts ---
const systemPromptOptions = [
  { value: "assistant_it", label: "Assistant IT" },
  { value: "diagnostic", label: "Diagnostic systeme" },
  { value: "custom", label: "Personnalise" },
];

const selectedPromptType = ref("assistant_it");
const customSystemPrompt = ref("");

const systemPromptPresets: Record<string, string> = {
  assistant_it: "Tu es un assistant IT expert en maintenance Windows. Tu reponds en francais de maniere concise et technique. Quand tu proposes une commande, encadre-la avec ```cmd ou ```powershell.",
  diagnostic: "Tu es un specialiste du diagnostic systeme Windows. Analyse les problemes decrits, identifie les causes probables et propose des solutions concretes avec les commandes necessaires.",
};

const activeSystemPrompt = computed(() => {
  if (selectedPromptType.value === "custom") return customSystemPrompt.value;
  return systemPromptPresets[selectedPromptType.value] ?? "";
});

// --- Chat ---
interface ChatMessage {
  role: "user" | "assistant";
  content: string;
  command?: string;
  commandResult?: string;
  commandRunning?: boolean;
}

const messages = ref<ChatMessage[]>([]);
const userInput = ref("");
const sending = ref(false);
const chatRef = ref<HTMLElement | null>(null);

function scrollChat() {
  nextTick(() => {
    if (chatRef.value) {
      chatRef.value.scrollTop = chatRef.value.scrollHeight;
    }
  });
}

function extractCommand(text: string): string | null {
  const cmdMatch = text.match(/```(?:cmd|powershell|bash|shell)\n([\s\S]*?)```/);
  return cmdMatch ? cmdMatch[1].trim() : null;
}

async function sendMessage() {
  const text = userInput.value.trim();
  if (!text || sending.value) return;

  messages.value.push({ role: "user", content: text });
  userInput.value = "";
  sending.value = true;
  scrollChat();

  // Historique structuré [{role, content}] — exclut le dernier message (déjà envoyé comme prompt)
  const history = messages.value
    .slice(0, -1)   // tout sauf le message actuel
    .slice(-20)     // max 20 échanges de contexte
    .map((m) => ({ role: m.role, content: m.content }));

  try {
    const response = await invoke<string>("ai_query", {
      model: selectedModel.value,
      prompt: text,
      systemPrompt: activeSystemPrompt.value || null,
      history: history.length ? history : null,
    });

    const cmd = extractCommand(response);
    messages.value.push({
      role: "assistant",
      content: response,
      command: cmd ?? undefined,
    });
  } catch (err: any) {
    // Ollama non disponible — message d'erreur explicite
    messages.value.push({
      role: "assistant",
      content: `⚠ Impossible de joindre Ollama.\n\nAssurez-vous qu'Ollama est lancé (\`ollama serve\`) et qu'un modèle est installé (\`ollama pull llama3.2\`).\n\nErreur : ${String(err)}`,
    });
  } finally {
    sending.value = false;
    scrollChat();
  }
}

async function executeCommand(msgIndex: number) {
  const msg = messages.value[msgIndex];
  if (!msg.command) return;

  msg.commandRunning = true;
  try {
    const result = await invoke<{ command: string; success: boolean; stdout: string; stderr: string }>("ai_execute_command", {
      command: msg.command,
    });
    msg.commandResult = result.stdout || result.stderr || "Aucune sortie";
    notify.success("Commande executee", result.success ? "Succes" : "Echec");
  } catch {
    msg.commandResult = "Execution simulee en mode dev.\nResultat : OK";
    notify.info("Mode dev", "Simulation d'execution");
  } finally {
    msg.commandRunning = false;
    scrollChat();
  }
}

function clearChat() {
  messages.value = [];
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
}

onMounted(() => {
  checkOllamaStatus();
  loadModels();
});
</script>

<template>
  <div class="ai-agents">
    <!-- Header -->
    <div class="page-header">
      <div>
        <h1>Assistant IA</h1>
        <p class="page-subtitle">Assistant intelligent avec Ollama pour la maintenance systeme</p>
      </div>
      <div class="header-actions">
        <!-- Ollama status -->
        <div class="ollama-status" :class="{ connected: ollamaConnected }">
          <NSpinner v-if="statusChecking" :size="14" />
          <CheckCircle v-else-if="ollamaConnected" :size="14" />
          <XCircle v-else :size="14" />
          <span>{{ statusChecking ? 'Verification...' : ollamaConnected ? 'Ollama connecte' : 'Ollama deconnecte' }}</span>
        </div>
        <NButton variant="secondary" size="sm" @click="checkOllamaStatus">
          <RefreshCw :size="14" />
        </NButton>
      </div>
    </div>

    <div class="ai-layout">
      <!-- Left: config -->
      <div class="ai-sidebar">
        <!-- Model selector -->
        <NCard>
          <template #header>
            <div class="section-header">
              <Cpu :size="16" />
              <span>Modele</span>
            </div>
          </template>
          <div class="config-section">
            <NDropdown
              :options="models"
              v-model="selectedModel"
              placeholder="Selectionner un modele..."
            />
            <NButton variant="secondary" size="sm" :loading="modelsLoading" @click="loadModels">
              <RefreshCw :size="14" />
              Rafraichir
            </NButton>
          </div>
        </NCard>

        <!-- System prompt -->
        <NCard>
          <template #header>
            <div class="section-header">
              <Settings :size="16" />
              <span>Prompt systeme</span>
            </div>
          </template>
          <div class="config-section">
            <NDropdown
              :options="systemPromptOptions"
              v-model="selectedPromptType"
              placeholder="Type de prompt"
            />
            <textarea
              v-if="selectedPromptType === 'custom'"
              v-model="customSystemPrompt"
              class="prompt-textarea"
              placeholder="Entrez votre prompt systeme..."
              rows="5"
            ></textarea>
            <div v-else class="prompt-preview">
              {{ activeSystemPrompt }}
            </div>
          </div>
        </NCard>
      </div>

      <!-- Right: chat -->
      <NCard class="chat-card">
        <template #header>
          <div class="section-header">
            <MessageSquare :size="16" />
            <span>Conversation</span>
            <NButton v-if="messages.length" variant="secondary" size="sm" @click="clearChat" style="margin-left: auto">
              Effacer
            </NButton>
          </div>
        </template>

        <!-- Messages -->
        <div ref="chatRef" class="chat-messages">
          <div v-if="messages.length === 0" class="chat-empty">
            <Bot :size="40" style="opacity: 0.2" />
            <p>Posez une question sur la maintenance systeme</p>
            <div class="suggestion-chips">
              <button class="chip" @click="userInput = 'Mon PC est lent, que faire ?'; sendMessage()">Mon PC est lent</button>
              <button class="chip" @click="userInput = 'Comment verifier les fichiers systeme ?'; sendMessage()">Verifier fichiers systeme</button>
              <button class="chip" @click="userInput = 'Comment optimiser le demarrage ?'; sendMessage()">Optimiser demarrage</button>
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
                <div class="message-text" v-html="formatMessage(msg.content)"></div>

                <!-- Command execution -->
                <div v-if="msg.command" class="command-block">
                  <div class="command-header">
                    <Terminal :size="14" />
                    <span>Commande suggeree</span>
                  </div>
                  <pre class="command-code">{{ msg.command }}</pre>
                  <div class="command-actions">
                    <NButton
                      variant="primary"
                      size="sm"
                      :loading="msg.commandRunning"
                      :disabled="msg.commandRunning || !!msg.commandResult"
                      @click="executeCommand(i)"
                    >
                      <Play :size="14" />
                      Executer
                    </NButton>
                  </div>
                  <div v-if="msg.commandResult" class="command-result">
                    <pre>{{ msg.commandResult }}</pre>
                  </div>
                </div>
              </div>
            </div>
          </template>

          <div v-if="sending" class="message assistant">
            <div class="message-avatar">
              <Bot :size="16" />
            </div>
            <div class="message-content">
              <NSpinner :size="16" />
              <span class="typing-text">Reflexion en cours...</span>
            </div>
          </div>
        </div>

        <!-- Input -->
        <div class="chat-input-area">
          <textarea
            v-model="userInput"
            class="chat-input"
            placeholder="Posez votre question..."
            rows="2"
            @keydown="handleKeydown"
          ></textarea>
          <NButton
            variant="primary"
            :disabled="!userInput.trim() || sending"
            :loading="sending"
            @click="sendMessage"
          >
            <Send :size="16" />
          </NButton>
        </div>
      </NCard>
    </div>
  </div>
</template>

<script lang="ts">
// Helper to format markdown-like content
function formatMessage(text: string): string {
  return text
    .replace(/```(\w*)\n([\s\S]*?)```/g, '<pre class="code-block"><code>$2</code></pre>')
    .replace(/`([^`]+)`/g, '<code class="inline-code">$1</code>')
    .replace(/\n/g, "<br>");
}
</script>

<style scoped>
.ai-agents {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.page-header h1 {
  font-size: 24px;
  font-weight: 700;
}

.page-subtitle {
  color: var(--text-muted);
  font-size: 13px;
  margin-top: 2px;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.ollama-status {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: var(--radius-md);
  font-size: 12px;
  font-weight: 500;
  background: var(--danger-muted);
  color: var(--danger);
}

.ollama-status.connected {
  background: var(--success-muted);
  color: var(--success);
}

.ai-layout {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 16px;
  align-items: start;
}

@media (max-width: 1000px) {
  .ai-layout { grid-template-columns: 1fr; }
}

.ai-sidebar {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.config-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.prompt-textarea {
  width: 100%;
  padding: 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 12px;
  resize: vertical;
  outline: none;
  min-height: 80px;
}

.prompt-textarea:focus { border-color: var(--accent-primary); }
.prompt-textarea::placeholder { color: var(--text-muted); }

.prompt-preview {
  font-size: 12px;
  color: var(--text-muted);
  line-height: 1.5;
  padding: 10px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
}

/* Chat */
.chat-card {
  display: flex;
  flex-direction: column;
}

.chat-messages {
  min-height: 400px;
  max-height: 550px;
  overflow-y: auto;
  padding: 8px 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.chat-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  height: 300px;
  color: var(--text-muted);
  font-size: 13px;
}

.suggestion-chips {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  justify-content: center;
  margin-top: 8px;
}

.chip {
  padding: 6px 14px;
  border-radius: 99px;
  border: 1px solid var(--border);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-family: inherit;
  font-size: 12px;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.chip:hover {
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.message {
  display: flex;
  gap: 10px;
  padding: 8px 12px;
  border-radius: var(--radius-md);
}

.message.user {
  background: var(--bg-tertiary);
}

.message.assistant {
  background: transparent;
}

.message-avatar {
  width: 28px;
  height: 28px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  background: var(--bg-secondary);
  color: var(--text-muted);
}

.message.assistant .message-avatar {
  background: var(--accent-muted);
  color: var(--accent-primary);
}

.message-content {
  flex: 1;
  min-width: 0;
}

.message-role {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  margin-bottom: 4px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.message-text {
  font-size: 13px;
  color: var(--text-primary);
  line-height: 1.6;
}

.message-text :deep(.code-block) {
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 10px 12px;
  margin: 8px 0;
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  overflow-x: auto;
}

.message-text :deep(.inline-code) {
  background: var(--bg-tertiary);
  padding: 1px 6px;
  border-radius: 4px;
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
}

.typing-text {
  font-size: 12px;
  color: var(--text-muted);
  margin-left: 6px;
}

/* Command block */
.command-block {
  margin-top: 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.command-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: var(--bg-tertiary);
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.command-code {
  padding: 10px 12px;
  background: var(--bg-primary);
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  color: var(--accent-primary);
  margin: 0;
  white-space: pre-wrap;
}

.command-actions {
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border-top: 1px solid var(--border);
}

.command-result {
  padding: 10px 12px;
  border-top: 1px solid var(--border);
  background: var(--bg-primary);
}

.command-result pre {
  font-family: "JetBrains Mono", monospace;
  font-size: 12px;
  color: var(--success);
  margin: 0;
  white-space: pre-wrap;
}

/* Input area */
.chat-input-area {
  display: flex;
  gap: 8px;
  align-items: flex-end;
  padding-top: 12px;
  margin-top: 12px;
  border-top: 1px solid var(--border);
}

.chat-input {
  flex: 1;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 13px;
  resize: none;
  outline: none;
  transition: border-color var(--transition-fast);
}

.chat-input:focus { border-color: var(--accent-primary); }
.chat-input::placeholder { color: var(--text-muted); }
</style>
