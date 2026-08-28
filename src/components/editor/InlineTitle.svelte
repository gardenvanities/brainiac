<script lang="ts">
import { parseFrontmatterTitle, resolveInlineTitle } from "../../lib/utils/documents";
import { documentsStore } from "../../stores/documents.store.svelte";

let editing = $state(false);
let value = $state("");
let confirming = $state(false);
let inputEl: HTMLInputElement | null = $state(null);

const doc = $derived(documentsStore.active);
const displayTitle = $derived(doc ? resolveInlineTitle(doc) : "");
// Cenário A (sem frontmatter) renomeia o arquivo; Cenário B (com
// frontmatter) atualiza só o `title:` — a sidebar nunca muda no B.
const hasFrontmatterTitle = $derived(doc ? parseFrontmatterTitle(doc) !== null : false);

function startEditing() {
  if (!doc) return;
  value = displayTitle;
  editing = true;
}

$effect(() => {
  if (editing) inputEl?.select();
});

function cancel() {
  editing = false;
  documentsStore.error = null;
}

async function confirm() {
  if (confirming || !doc) return;
  const next = value.trim();
  if (!next || next === displayTitle) {
    cancel();
    return;
  }
  confirming = true;
  documentsStore.error = null;
  try {
    const ok = hasFrontmatterTitle
      ? await documentsStore.updateTitle(doc.id, next)
      : await documentsStore.rename(doc.id, next);
    if (ok) editing = false;
    // Em conflito: mantém em edição com o valor digitado para correção
  } finally {
    confirming = false;
  }
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Enter") void confirm();
  if (e.key === "Escape") cancel();
}
</script>

{#if doc}
  <div class="inline-title">
    {#if editing}
      <input
        bind:this={inputEl}
        bind:value
        class="title-input"
        type="text"
        aria-label="Título do documento"
        onkeydown={onKeydown}
        onblur={() => void confirm()}
      />
      {#if documentsStore.error}
        <p class="error" role="alert">{documentsStore.error}</p>
      {/if}
    {:else}
      <h1 ondblclick={startEditing} title="Duplo clique para renomear">{displayTitle}</h1>
      <button class="edit-btn" onclick={startEditing} aria-label="Renomear documento">✎</button>
    {/if}
  </div>
{/if}

<style>
  .inline-title {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-5) 0;
    min-height: 32px;
  }

  h1 {
    margin: 0;
    font-size: var(--font-size-md);
    font-weight: 600;
    color: var(--color-text-primary);
    cursor: default;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .edit-btn {
    border: none;
    background: transparent;
    color: var(--color-text-muted);
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: var(--space-1);
    line-height: 1;
    opacity: 0;
    transition: opacity 0.15s ease, color 0.15s ease;
  }

  .inline-title:hover .edit-btn {
    opacity: 1;
  }

  .edit-btn:hover {
    color: var(--color-accent-primary);
  }

  .title-input {
    width: 100%;
    max-width: 420px;
    padding: var(--space-1) var(--space-2);
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-focus);
    border-radius: 4px;
    color: var(--color-text-primary);
    font-size: var(--font-size-md);
    font-family: var(--font-sans);
    outline: none;
  }

  .error {
    margin: 0;
    color: var(--color-danger);
    font-size: var(--font-size-xs);
  }
</style>
