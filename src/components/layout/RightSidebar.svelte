<script lang="ts">
let message = $state("");

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
}

function sendMessage() {
  if (!message.trim()) return;
  // Lógica de envio será implementada no Passo 8
  message = "";
}
</script>

<aside class="right-sidebar">
  <div class="sidebar-header">
    <div class="agent-info">
      <div class="agent-avatar">B</div>
      <div class="agent-meta">
        <span class="agent-name">BRAINIAC</span>
        <span class="agent-model">claude-sonnet-4-6</span>
      </div>
    </div>
    <button class="icon-btn" title="Nova conversa">+</button>
  </div>

  <div class="context-banner">
    <span class="context-label">󰈙 Sem documento aberto</span>
  </div>

  <div class="messages-area">
    <!-- ChatMessages será implementado no Passo 8 -->
    <div class="empty-state">
      <p>Olá! Como posso te ajudar hoje?</p>
    </div>
  </div>

  <div class="input-area">
    <textarea
      class="message-input"
      placeholder="Mensagem para o BRAINIAC..."
      rows="3"
      bind:value={message}
      onkeydown={handleKeydown}
    ></textarea>
    <div class="input-footer">
      <span class="hint">Enter para enviar · Shift+Enter para quebrar linha</span>
      <button
        class="send-btn"
        class:active={message.trim().length > 0}
        onclick={sendMessage}
        disabled={!message.trim()}
      >
        Enviar
      </button>
    </div>
  </div>
</aside>

<style>
  .right-sidebar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-surface);
    border-left: 1px solid var(--border);
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-3);
    height: 48px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .agent-info {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .agent-avatar {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: var(--accent);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-sm);
    font-weight: 700;
    flex-shrink: 0;
  }

  .agent-meta {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .agent-name {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-primary);
  }

  .agent-model {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: var(--space-1) var(--space-2);
    border-radius: 4px;
    font-size: var(--font-size-md);
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .context-banner {
    padding: var(--space-2) var(--space-3);
    background: var(--bg-elevated);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .context-label {
    font-size: var(--font-size-xs);
    color: var(--text-muted);
  }

  .messages-area {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4) var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
    text-align: center;
  }

  .input-area {
    padding: var(--space-3);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
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

  .message-input::placeholder {
    color: var(--text-muted);
  }

  .input-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: var(--space-2);
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
