<script lang="ts">
import { messagesStore } from "../../stores/messages.store.svelte";
import ChatMessage from "./ChatMessage.svelte";

let containerRef = $state<HTMLDivElement | null>(null);

// Auto-scroll para o final quando nova mensagem chegar
$effect(() => {
  const _ = messagesStore.messages.length;
  const __ = messagesStore.streamingContent;
  if (containerRef) {
    containerRef.scrollTop = containerRef.scrollHeight;
  }
});
</script>

<div class="messages-container" bind:this={containerRef}>
  {#if messagesStore.messages.length === 0 && !messagesStore.isStreaming}
    <div class="empty-state">
      <p>Olá! Como posso te ajudar hoje?</p>
    </div>
  {:else}
    {#each messagesStore.messages as message (message.id)}
      <ChatMessage {message} />
    {/each}

    {#if messagesStore.isStreaming}
      <ChatMessage
        message={{
          id: "streaming",
          conversationId: messagesStore.conversation?.id ?? "",
          role: "assistant",
          content: messagesStore.streamingContent,
          modelUsed: null,
          tokensInput: null,
          tokensOutput: null,
          createdAt: "",
        }}
        streaming={true}
      />
    {/if}
  {/if}
</div>

<style>
  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4) var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    scroll-behavior: smooth;
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
</style>
