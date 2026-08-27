<script lang="ts">
  import { messagesStore } from "../../stores/messages.store.svelte";
  import { documentsStore } from "../../stores/documents.store.svelte";
  import { saveApiKey } from "../../lib/tauri/messages";

  $effect(() => {
    console.log("hasApiKey:", messagesStore.hasApiKey);
  });

  let message = $state("");
  let apiKeyInput = $state("");
  let showApiKeyForm = $state(false);

  const canSend = $derived(
    message.trim().length > 0 &&
    !messagesStore.isStreaming &&
    messagesStore.hasApiKey
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
    border-top: 1px solid var(--border);
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .setup-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--accent-subtle);
    border: 1px solid rgba(124, 106, 247, 0.2);
    border-radius: 6px;
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-xs);
    color: var(--accent);
    gap: var(--space-2);
  }

  .setup-banner button {
    background: var(--accent);
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
    background: var(--bg-elevated);
    border: 1px solid var(--accent);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    padding: var(--space-1) var(--space-2);
    outline: none;
    min-width: 0;
  }

  .api-key-form button {
    background: var(--accent);
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
    border: 1px solid var(--border);
    color: var(--text-secondary);
  }

  .error-banner {
    background: rgba(255, 100, 100, 0.1);
    border: 1px solid rgba(255, 100, 100, 0.2);
    border-radius: 4px;
    padding: var(--space-2) var(--space-3);
    font-size: var(--font-size-xs);
    color: #ff8080;
  }

  .message-input {
    width: 100%;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    font-family: var(--font-sans);
    padding: var(--space-3);
    resize: none;
    outline: none;
    line-height: 1.5;
    transition: border-color 0.15s ease;
  }

  .message-input:focus {
    border-color: var(--accent);
  }

  .message-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .message-input::placeholder {
    color: var(--text-muted);
  }

  .input-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .hint {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }

  .send-btn {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: var(--space-1) var(--space-4);
    transition: all 0.15s ease;
  }

  .send-btn.active {
    background: var(--accent);
    border-color: var(--accent);
    color: white;
  }

  .send-btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>
