<script lang="ts">
import { saveApiKey } from "../../lib/tauri/messages";
import { documentsStore } from "../../stores/documents.store.svelte";
import { messagesStore } from "../../stores/messages.store.svelte";

let message = $state("");
let apiKeyInput = $state("");
let showApiKeyForm = $state(false);

const canSend = $derived(
  message.trim().length > 0 && !messagesStore.isStreaming && messagesStore.hasApiKey,
);

async function handleSend() {
  if (!canSend) return;
  const content = message.trim();
  message = "";
  const context = documentsStore.active?.content;
  await messagesStore.send(content, context);
}

async function handleSaveApiKey() {
  if (!apiKeyInput.trim()) return;
  await saveApiKey(apiKeyInput.trim());
  apiKeyInput = "";
  showApiKeyForm = false;
  await messagesStore.checkApiKey();
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSend();
  }
}
</script>

<div class="input-area">
  {#if !messagesStore.hasApiKey}
    <div class="setup-banner">
      {#if !showApiKeyForm}
        <span>Configure sua Groq API key para começar</span>
        <button onclick={() => (showApiKeyForm = true)}>Configurar</button>
      {:else}
        <div class="api-key-form">
          <input
            type="password"
            placeholder="gsk_..."
            bind:value={apiKeyInput}
            onkeydown={(e) => e.key === "Enter" && handleSaveApiKey()}
          />
          <button onclick={handleSaveApiKey}>Salvar</button>
          <button class="cancel" onclick={() => (showApiKeyForm = false)}>✕</button>
        </div>
      {/if}
    </div>
  {:else if messagesStore.error}
    <div class="error-banner">⚠ {messagesStore.error}</div>
  {/if}

  <textarea
    class="message-input"
    placeholder={messagesStore.hasApiKey ? "Mensagem para o BRAINIAC..." : "Configure a API key primeiro..."}
    rows="3"
    bind:value={message}
    onkeydown={handleKeydown}
    disabled={messagesStore.isStreaming || !messagesStore.hasApiKey}
  ></textarea>

  <div class="input-footer">
    <span class="hint">Enter · Shift+Enter para quebrar linha</span>
    <button
      class="send-btn"
      class:active={canSend}
      onclick={handleSend}
      disabled={!canSend}
    >
      {messagesStore.isStreaming ? "..." : "Enviar"}
    </button>
  </div>
</div>

<style>
  .input-area {
    padding: var(--space-3);
    border-top: 1px solid var(--color-border-default);
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .setup-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--color-interactive-subtle);
    border: 1px solid color-mix(in oklch, var(--color-interactive) 20%, transparent);
    border-radius: 6px;
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-xs);
    color: var(--color-interactive);
    gap: var(--space-2);
  }

  .setup-banner button {
    background: var(--color-interactive);
    border: none;
    border-radius: 3px;
    color: white;
    cursor: pointer;
    font-size: var(--font-size-xs);
    padding: 3px var(--space-3);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .api-key-form {
    display: flex;
    gap: var(--space-2);
    width: 100%;
  }

  .api-key-form input {
    flex: 1;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-interactive);
    border-radius: 4px;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    padding: var(--space-1) var(--space-2);
    outline: none;
    min-width: 0;
  }

  .api-key-form button {
    background: var(--color-interactive);
    border: none;
    border-radius: 4px;
    color: white;
    cursor: pointer;
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    flex-shrink: 0;
  }

  .api-key-form button.cancel {
    background: transparent;
    border: 1px solid var(--color-border-default);
    color: var(--color-text-secondary);
  }

  .error-banner {
    background: var(--color-danger-subtle);
    border: 1px solid color-mix(in oklch, var(--color-danger) 20%, transparent);
    border-radius: 4px;
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-xs);
    color: var(--color-danger);
  }

  .message-input {
    width: 100%;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 6px;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    font-family: var(--font-body);
    padding: var(--space-3);
    resize: none;
    outline: none;
    line-height: 1.5;
    transition: border-color 0.15s ease;
  }

  .message-input:focus {
    border-color: var(--color-interactive);
  }

  .message-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .message-input::placeholder {
    color: var(--color-text-muted);
  }

  .input-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .send-btn {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: var(--space-1) var(--space-4);
    transition: all 0.15s ease;
  }

  .send-btn.active {
    background: var(--color-interactive);
    border-color: var(--color-interactive);
    color: white;
  }

  .send-btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>
