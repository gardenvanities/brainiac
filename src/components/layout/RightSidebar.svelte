<script lang="ts">
import { onMount } from "svelte";
import { messagesStore } from "../../stores/messages.store.svelte";
import ChatInput from "../chat/ChatInput.svelte";
import ChatMessages from "../chat/ChatMessages.svelte";

onMount(() => {
  let unlisteners: (() => void)[] = [];
  messagesStore.setupListeners().then((uns) => {
    unlisteners = uns;
  });
  messagesStore.loadConversation();
  return () => {
    for (const un of unlisteners) un();
  };
});
</script>

<aside class="right-sidebar">
  <div class="sidebar-header" data-tauri-drag-region>
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

  <ChatMessages />

  <ChatInput />
</aside>

<style>
  .right-sidebar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--color-bg-surface);
    border-left: 1px solid var(--color-border-default);
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-3);
    height: 48px;
    border-bottom: 1px solid var(--color-border-default);
    flex-shrink: 0;
  }

  .agent-info {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    /* Cliques no texto/avatar caem no header (drag region); o .icon-btn continua clicável */
    pointer-events: none;
  }

  .agent-avatar {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: var(--color-accent-primary);
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
    color: var(--color-text-primary);
  }

  .agent-model {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: var(--space-1) var(--space-2);
    border-radius: 4px;
    font-size: var(--font-size-md);
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .context-banner {
    padding: var(--space-2) var(--space-3);
    background: var(--color-bg-elevated);
    border-bottom: 1px solid var(--color-border-default);
    flex-shrink: 0;
  }

  .context-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
