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
  // `!editing`: o unmount do input (Escape/cancelamento) dispara blur —
  // sem esta guarda, o blur re-confirma o valor cancelado e ressuscita
  // o erro que cancel() acabou de limpar.
  if (!editing || confirming || !doc) return;
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
      <!-- Mesma tipografia do h1 exibido: a troca não causa salto visual -->
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
      <h1>
        <!-- Botão semântico: clicar no título inicia a edição (o nome é a
             affordance — sem botão de lápis extra). Teclado: Tab + Enter. -->
        <button class="title-button" onclick={startEditing} title="Clique para editar o título">
          {displayTitle}
        </button>
      </h1>
      {#if documentsStore.error}
        <p class="error" role="alert">{documentsStore.error}</p>
      {/if}
    {/if}
  </div>
{/if}

<style>
  .inline-title {
    /* Ocupa a mesma largura do corpo do documento (o pai .doc-surface
       define a largura útil) — o título é o primeiro bloco do documento,
       não uma barra acima dele. */
    min-width: 0;
  }

  /*
   * Hierarquia visual de H1 — espelha `.milkdown .ProseMirror h1`
   * (crepe frame, theme/common/reset.css): fonte serifada de título,
   * 2.625em, peso 400, line-height 1.1905, padding-block 2px.
   * O pai `.doc-surface` usa --font-size-md, a mesma base do .milkdown.
   */
  h1 {
    margin: 0;
    padding-block: 2px;
    font-family: var(--font-title);
    font-size: 2.625em;
    font-weight: 400;
    line-height: 1.1905;
    color: var(--color-text-primary);
    overflow-wrap: break-word;
  }

  .title-button {
    display: block;
    width: 100%;
    padding: 0;
    background: none;
    border: none;
    font: inherit;
    color: inherit;
    text-align: start;
    cursor: text;
    border-radius: 3px;
    transition: color 0.15s ease;
  }

  .title-button:hover {
    color: var(--color-text-secondary);
  }

  .title-button:focus-visible {
    outline: 1px solid var(--color-accent-primary);
    outline-offset: 2px;
  }

  .title-input {
    width: 100%;
    padding: 2px 0;
    background: none;
    border: none;
    border-block-end: 1px solid var(--color-accent-primary);
    border-radius: 0;
    font-family: var(--font-title);
    font-size: 2.625em;
    font-weight: 400;
    line-height: 1.1905;
    color: var(--color-text-primary);
    caret-color: var(--color-accent-primary);
    outline: none;
  }

  .error {
    margin: 0;
    color: var(--color-danger);
    font-size: var(--font-size-xs);
  }
</style>
