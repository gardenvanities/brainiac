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

  /* Base da prosa — background transparente neutraliza o fundo do tema
     frame do Milkdown (dark mode only); o resto segue skills/editor-typography.md */
  :global(.milkdown) {
    background: transparent !important;
    max-width: var(--prose-measure);
    font-family: var(--font-body);
    font-size: var(--font-size-lg);
    line-height: var(--line-height-relaxed);
    color: var(--color-text-primary);
  }

  :global(.milkdown .ProseMirror) {
    outline: none;
    caret-color: var(--color-interactive);
  }

  /* Hierarquia de headings: peso E tamanho decrescentes
     (h1-h2 bold, h3-h4 semibold, h5-h6 medium) */
  :global(.milkdown h1) {
    font-family: var(--font-heading);
    font-size: var(--prose-h1-size);
    font-weight: var(--font-weight-bold);
    line-height: var(--line-height-tight);
    margin-top: var(--prose-heading-margin-top);
    margin-bottom: var(--prose-heading-margin-bottom);
  }

  :global(.milkdown h1:first-child) {
    margin-top: 0;
  }

  :global(.milkdown h2) {
    font-family: var(--font-heading);
    font-size: var(--prose-h2-size);
    font-weight: var(--font-weight-bold);
    line-height: var(--line-height-tight);
    margin-top: var(--prose-heading-margin-top);
    margin-bottom: var(--prose-heading-margin-bottom);
  }

  :global(.milkdown h3) {
    font-family: var(--font-heading);
    font-size: var(--prose-h3-size);
    font-weight: var(--font-weight-semibold);
    line-height: var(--line-height-tight);
    margin-top: var(--prose-heading-margin-top);
    margin-bottom: var(--prose-heading-margin-bottom);
  }

  :global(.milkdown h4) {
    font-family: var(--font-heading);
    font-size: var(--prose-h4-size);
    font-weight: var(--font-weight-semibold);
    line-height: var(--line-height-tight);
    margin-top: var(--prose-heading-margin-top);
    margin-bottom: var(--prose-heading-margin-bottom);
  }

  :global(.milkdown h5) {
    font-family: var(--font-heading);
    font-size: var(--prose-h5-size);
    font-weight: var(--font-weight-medium);
    line-height: var(--line-height-tight);
    margin-top: var(--prose-heading-margin-top);
    margin-bottom: var(--prose-heading-margin-bottom);
  }

  :global(.milkdown h6) {
    font-family: var(--font-heading);
    font-size: var(--prose-h6-size);
    font-weight: var(--font-weight-medium);
    line-height: var(--line-height-tight);
    margin-top: var(--prose-heading-margin-top);
    margin-bottom: var(--prose-heading-margin-bottom);
  }

  :global(.milkdown p) {
    margin-block: var(--prose-space-base);
  }

  :global(.milkdown ul, .milkdown ol) {
    padding-inline-start: var(--prose-indent);
    margin-block: var(--prose-space-base);
  }

  :global(.milkdown li) {
    margin-block: var(--prose-space-tight);
  }

  :global(.milkdown li::marker) {
    color: var(--color-text-muted);
  }

  :global(.milkdown blockquote) {
    border-inline-start: var(--prose-blockquote-border-width) solid var(--color-border-strong);
    padding-inline-start: var(--space-4);
    margin-block: var(--prose-space-base);
    color: var(--color-text-secondary);
  }

  :global(.milkdown a) {
    color: var(--color-link);
    text-decoration: underline;
    text-underline-offset: 2px;
    text-decoration-thickness: 1px;
  }

  :global(.milkdown a:visited) {
    color: var(--color-link-visited);
  }

  :global(.milkdown hr) {
    border: none;
    border-top: 1px solid var(--color-border-default);
    margin-block: var(--prose-space-loose);
  }

  :global(.milkdown code) {
    font-family: var(--font-code);
    font-feature-settings: var(--font-feature-code);
    font-size: 0.9em;
    background: var(--color-bg-elevated);
    border-radius: var(--radius-sm);
    padding: 0.15em 0.4em;
  }

  :global(.milkdown pre) {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: var(--radius-md);
    padding: var(--space-4);
    margin-block: var(--prose-space-base);
    overflow-x: auto;
  }

  :global(.milkdown pre code) {
    background: transparent;
    padding: 0;
    font-size: var(--font-size-md);
    line-height: var(--line-height-relaxed);
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
