<script lang="ts">
import { Crepe } from "@milkdown/crepe";
import { untrack } from "svelte";
import "@milkdown/crepe/theme/common/style.css";
import "@milkdown/crepe/theme/frame.css";
import { documentsStore } from "../../stores/documents.store.svelte";
import InlineTitle from "./InlineTitle.svelte";

let editorRef = $state<HTMLDivElement | null>(null);
let saveTimer: ReturnType<typeof setTimeout> | null = null;
let isInitializing = false;

function scheduleAutoSave(crepe: Crepe) {
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(() => {
    if (!isInitializing && documentsStore.active) {
      const content = crepe.getMarkdown();
      documentsStore.updateContent(content);
      documentsStore.save(content);
    }
  }, 1000);
}

$effect(() => {
  if (!editorRef) return;

  // Rastreia APENAS o ID — mudança de conteúdo não recria o editor
  const docId = documentsStore.active?.id;

  // Lê o conteúdo sem rastrear (untrack = não cria dependência reativa)
  const content = untrack(() => documentsStore.active?.content ?? "");

  if (saveTimer) clearTimeout(saveTimer);
  isInitializing = true;
  editorRef.innerHTML = "";

  if (!docId) {
    isInitializing = false;
    return;
  }

  const crepe = new Crepe({
    root: editorRef,
    defaultValue: content,
  });

  let observer: MutationObserver | null = null;

  crepe.create().then(() => {
    isInitializing = false;

    const prosemirror = editorRef?.querySelector(".ProseMirror");
    if (prosemirror) {
      observer = new MutationObserver(() => {
        if (!isInitializing) {
          const markdown = crepe.getMarkdown();
          documentsStore.updateContent(markdown);
          scheduleAutoSave(crepe);
        }
      });

      observer.observe(prosemirror, {
        childList: true,
        subtree: true,
        characterData: true,
      });
    }
  });

  return () => {
    if (saveTimer) clearTimeout(saveTimer);
    observer?.disconnect();
    crepe.destroy();
  };
});
</script>

<div class="editor-wrapper">
  {#if documentsStore.active}
    <div class="editor-container">
      <!-- Superfície do documento: título inline + conteúdo compartilham
           a mesma largura, padding e fluxo vertical — o título rola junto
           com o texto, como primeiro elemento do documento. -->
      <div class="doc-surface">
        <InlineTitle />
        <div class="editor-mount" bind:this={editorRef}></div>
      </div>
    </div>
  {:else}
    <div class="editor-placeholder">
      <p>Selecione ou crie um documento para começar</p>
    </div>
  {/if}
</div>

<style>
  .editor-wrapper {
    position: relative;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .editor-container {
    flex: 1;
    overflow-y: auto;
    height: 100%;
  }

  /* Superfície do documento: mesma largura/alinhamento que o corpo do
     Milkdown usava (max-width 760px centrado, padding do tema) — título e
     conteúdo dividem o espaço e rolam juntos. */
  .doc-surface {
    max-width: 760px;
    margin-inline: auto;
    padding-block: var(--space-6);
    padding-inline: var(--space-4);
    min-height: 100%;
    font-size: var(--font-size-md);
  }

  :global(.milkdown) {
    background: transparent !important;
    color: var(--color-text-primary) !important;
    font-family: var(--font-ui) !important;
    font-size: var(--font-size-md) !important;
  }

  :global(.milkdown .ProseMirror) {
    outline: none;
    caret-color: var(--color-accent-primary);
  }

  :global(.milkdown p) {
    color: var(--color-text-primary);
    line-height: 1.7;
  }

  :global(.milkdown h1),
  :global(.milkdown h2),
  :global(.milkdown h3),
  :global(.milkdown h4) {
    color: var(--color-text-primary);
    line-height: 1.3;
  }

  :global(.milkdown code) {
    background: var(--color-bg-elevated);
    border-radius: 3px;
    font-family: var(--font-body);
    font-size: var(--font-size-sm);
    padding: 1px 4px;
  }

  :global(.milkdown pre) {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 6px;
    padding: var(--space-4);
  }

  :global(.milkdown blockquote) {
    border-left: 3px solid var(--color-accent-primary);
    padding-left: var(--space-4);
    color: var(--color-text-secondary);
  }

  .editor-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--color-text-muted);
  }

  .editor-placeholder p {
    font-size: var(--font-size-md);
  }
</style>
