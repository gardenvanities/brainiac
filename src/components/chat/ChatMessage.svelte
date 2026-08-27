<script lang="ts">
  import type { Message } from "../../types";

  let { message, streaming = false }: { message?: Message; streaming?: boolean } = $props();

  const isUser = $derived(message?.role === "user");
  const content = $derived(message?.content ?? "");
</script>

<div class="message" class:user={isUser} class:assistant={!isUser}>
  {#if !isUser}
    <div class="avatar">B</div>
  {/if}
  <div class="bubble" class:streaming>
    <p>{content}</p>
    {#if streaming}
      <span class="cursor">▊</span>
    {/if}
  </div>
</div>

<style>
  .message {
    display: flex;
    gap: var(--space-2);
    align-items: flex-start;
  }

  .message.user {
    flex-direction: row-reverse;
  }

  .avatar {
    width: 24px;
    height: 24px;
    border-radius: 5px;
    background: var(--accent);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-xs);
    font-weight: 700;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .bubble {
    max-width: 85%;
    padding: var(--space-2) var(--space-3);
    border-radius: 8px;
    font-size: var(--font-size-sm);
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .message.user .bubble {
    background: var(--accent-subtle);
    color: var(--text-primary);
    border: 1px solid rgba(124, 106, 247, 0.2);
  }

  .message.assistant .bubble {
    background: var(--bg-elevated);
    color: var(--text-primary);
    border: 1px solid var(--border);
  }

  .bubble p {
    margin: 0;
  }

  .cursor {
    display: inline-block;
    animation: blink 1s step-end infinite;
    color: var(--accent);
    margin-left: 2px;
  }

  @keyframes blink {
    50% { opacity: 0; }
  }
</style>
