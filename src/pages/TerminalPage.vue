<script setup lang="ts">
import { ref, nextTick, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import NButton from "@/components/ui/NButton.vue";
import { Terminal, Trash2, Send, ChevronDown } from "lucide-vue-next";

interface OutputLine {
  timestamp: string;
  type: "cmd" | "stdout" | "stderr" | "info";
  text: string;
}

interface ShellInfo {
  id: string;
  name: string;
  path: string;
  available: boolean;
}

const input = ref("");
const output = ref<OutputLine[]>([]);
const outputEl = ref<HTMLDivElement | null>(null);
const history = ref<string[]>([]);
const historyIndex = ref(-1);
const running = ref(false);
const shells = ref<ShellInfo[]>([]);
const activeShell = ref("cmd");

async function loadShells() {
  try {
    shells.value = await invoke<ShellInfo[]>("detect_shells");
    // Selectionner le premier shell disponible
    const available = shells.value.filter(s => s.available);
    if (available.length > 0) {
      activeShell.value = available[0].id;
    }
  } catch {
    shells.value = [
      { id: "cmd", name: "CMD", path: "cmd.exe", available: true },
      { id: "powershell", name: "PowerShell", path: "powershell.exe", available: true },
    ];
  }
}

function getShellLabel(id: string): string {
  return shells.value.find(s => s.id === id)?.name ?? id;
}

function now(): string {
  return new Date().toLocaleTimeString("fr-FR", { hour: "2-digit", minute: "2-digit", second: "2-digit" });
}

function scrollToBottom() {
  nextTick(() => {
    if (outputEl.value) {
      outputEl.value.scrollTop = outputEl.value.scrollHeight;
    }
  });
}

function pushLine(type: OutputLine["type"], text: string) {
  output.value.push({ timestamp: now(), type, text });
  scrollToBottom();
}

async function executeCommand() {
  const cmd = input.value.trim();
  if (!cmd || running.value) return;

  pushLine("cmd", cmd);
  history.value.push(cmd);
  historyIndex.value = history.value.length;
  input.value = "";
  running.value = true;

  try {
    const result = await invoke<any>("run_in_shell", {
      shellId: activeShell.value,
      command: cmd,
    });

    const stdout = result?.stdout ?? result?.output ?? "";
    const stderr = result?.stderr ?? result?.error ?? "";

    if (stdout) {
      for (const line of stdout.split("\n")) {
        if (line.trim()) pushLine("stdout", line);
      }
    }
    if (stderr) {
      for (const line of stderr.split("\n")) {
        if (line.trim()) pushLine("stderr", line);
      }
    }
    if (!stdout && !stderr) {
      pushLine("info", "Commande executee (aucune sortie)");
    }
  } catch (e: any) {
    pushLine("stderr", `Erreur: ${e?.toString() ?? "Commande echouee"}`);
  }

  running.value = false;
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === "Enter") {
    executeCommand();
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    if (historyIndex.value > 0) {
      historyIndex.value--;
      input.value = history.value[historyIndex.value];
    }
  } else if (e.key === "ArrowDown") {
    e.preventDefault();
    if (historyIndex.value < history.value.length - 1) {
      historyIndex.value++;
      input.value = history.value[historyIndex.value];
    } else {
      historyIndex.value = history.value.length;
      input.value = "";
    }
  }
}

function clearOutput() {
  output.value = [];
  pushLine("info", "Terminal efface");
}

onMounted(() => {
  loadShells();
  pushLine("info", "NiTriTe Terminal — Tapez une commande et appuyez sur Entree");
});
</script>

<template>
  <div class="terminal-page">
    <div class="page-header">
      <div>
        <h1>Terminal</h1>
        <p class="page-subtitle">Executeur de commandes systeme</p>
      </div>
      <div class="header-actions">
        <div class="shell-selector">
          <label class="shell-label">Shell :</label>
          <select v-model="activeShell" class="shell-select">
            <option
              v-for="shell in shells.filter(s => s.available)"
              :key="shell.id"
              :value="shell.id"
            >
              {{ shell.name }}
            </option>
          </select>
        </div>
        <NButton variant="secondary" size="sm" @click="clearOutput">
          <Trash2 :size="14" />
          Effacer
        </NButton>
      </div>
    </div>

    <div class="terminal-container">
      <div class="terminal-output" ref="outputEl">
        <div
          v-for="(line, i) in output"
          :key="i"
          class="output-line"
          :class="`output-${line.type}`"
        >
          <span class="line-time">{{ line.timestamp }}</span>
          <span v-if="line.type === 'cmd'" class="line-prompt">&gt;</span>
          <span v-else-if="line.type === 'stderr'" class="line-prefix">ERR</span>
          <span v-else-if="line.type === 'info'" class="line-prefix info">INF</span>
          <span class="line-text">{{ line.text }}</span>
        </div>
        <div v-if="output.length === 0" class="terminal-empty">
          <Terminal :size="32" />
          <span>Pret. Tapez une commande ci-dessous.</span>
        </div>
      </div>

      <div class="terminal-input">
        <span class="shell-badge">{{ getShellLabel(activeShell) }}</span>
        <span class="input-prompt">&gt;</span>
        <input
          v-model="input"
          type="text"
          class="cmd-input"
          placeholder="Entrez une commande..."
          :disabled="running"
          @keydown="handleKeyDown"
          autofocus
        />
        <NButton variant="primary" size="sm" :loading="running" @click="executeCommand">
          <Send :size="14" />
        </NButton>
      </div>
    </div>
  </div>
</template>

<style scoped>
.terminal-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.page-header h1 { font-size: 24px; font-weight: 700; }
.page-subtitle { color: var(--text-muted); font-size: 13px; margin-top: 2px; }
.header-actions { display: flex; gap: 8px; }

.terminal-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #0d1117;
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  min-height: 400px;
}

.terminal-output {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  font-family: "JetBrains Mono", "Cascadia Code", "Consolas", monospace;
  font-size: 13px;
  line-height: 1.6;
}

.terminal-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  height: 100%;
  color: #484f58;
  min-height: 200px;
}

.output-line {
  display: flex;
  gap: 8px;
  white-space: pre-wrap;
  word-break: break-all;
}

.line-time {
  color: #484f58;
  flex-shrink: 0;
  font-size: 11px;
  min-width: 60px;
}

.line-prompt {
  color: #58a6ff;
  font-weight: 700;
  flex-shrink: 0;
}

.line-prefix {
  color: #f85149;
  font-weight: 600;
  font-size: 11px;
  flex-shrink: 0;
  min-width: 28px;
}

.line-prefix.info {
  color: #58a6ff;
}

.output-cmd .line-text { color: #c9d1d9; font-weight: 500; }
.output-stdout .line-text { color: #8b949e; }
.output-stderr .line-text { color: #f85149; }
.output-info .line-text { color: #58a6ff; font-style: italic; }

.terminal-input {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-top: 1px solid #21262d;
  background: #010409;
}

.input-prompt {
  color: #58a6ff;
  font-family: "JetBrains Mono", monospace;
  font-weight: 700;
  font-size: 14px;
  flex-shrink: 0;
}

.cmd-input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  color: #c9d1d9;
  font-family: "JetBrains Mono", "Cascadia Code", "Consolas", monospace;
  font-size: 13px;
}

.cmd-input::placeholder { color: #484f58; }
.cmd-input:disabled { opacity: 0.5; }

.shell-selector {
  display: flex;
  align-items: center;
  gap: 6px;
}

.shell-label {
  font-size: 12px;
  color: var(--text-muted);
}

.shell-select {
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 12px;
  padding: 4px 8px;
  cursor: pointer;
}

.shell-badge {
  font-size: 10px;
  font-weight: 600;
  color: #7ee787;
  background: #1b4332;
  padding: 2px 8px;
  border-radius: 4px;
  flex-shrink: 0;
  font-family: "JetBrains Mono", monospace;
  text-transform: uppercase;
}
</style>
