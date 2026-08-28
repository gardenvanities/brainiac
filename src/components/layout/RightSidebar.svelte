<script lang="ts">
import { onMount } from "svelte";
import { resolveInlineTitle } from "../../lib/utils/documents";
import { documentsStore } from "../../stores/documents.store.svelte";
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
    <span class="app-name">BRAINIAC</span>
  </div>

  {#if documentsStore.active}
    <div class="context-banner">
      <span class="context-label">󰈙 Contexto: {resolveInlineTitle(documentsStore.active)}</span>
    </div>
  {/if}

  <ChatMessages />

  <ChatInput />
</aside>

<style>
  .right-sidebar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--color-bg-surface);
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    padding: 0 var(--space-3);
    height: 48px;
    border-bottom: 1px solid var(--color-border-subtle);
    flex-shrink: 0;
  }

  .app-name {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--color-text-muted);
    letter-spacing: 0.08em;
    text-transform: uppercase;
    /* Cliques no texto caem no header (drag region) em vez de virarem alvo próprio */
    pointer-events: none;
  }

  .context-banner {
    padding: var(--space-1) var(--space-3);
    flex-shrink: 0;
  }

  .context-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
