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

// Contexto real da IA na pipeline atual — sem nova arquitetura de contexto:
// 1) `conversation.documentId` — contexto persistido da conversa (quando existir);
// 2) senão, o documento aberto — que o ChatInput envia como `documentContext`
//    a cada mensagem (o que o modelo efetivamente recebe);
// 3) nenhum → "Sem contexto".
const contextDoc = $derived.by(() => {
  const persistedId = messagesStore.conversation?.documentId ?? null;
  if (persistedId) {
    return documentsStore.list.find((d) => d.id === persistedId) ?? null;
  }
  return documentsStore.active;
});
const contextLabel = $derived(contextDoc ? resolveInlineTitle(contextDoc) : "Sem contexto");

// Identificação do modelo/agente atual, quando a conversa já tem um.
const modelLabel = $derived(messagesStore.conversation?.modelUsed ?? null);
</script>

<aside class="right-sidebar">
  <div class="sidebar-header" data-tauri-drag-region>
    {#if modelLabel}
      <span class="agent-model" title="Modelo atual">{modelLabel}</span>
    {/if}
  </div>

  <div class="context-banner">
    <span class="context-label">
      {contextDoc ? `󰈙 Contexto: ${contextLabel}` : "Sem contexto"}
    </span>
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
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    padding: 0 var(--space-3);
    height: 40px;
    flex-shrink: 0;
  }

  .agent-model {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
    /* Cliques caem no header (drag region) */
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
