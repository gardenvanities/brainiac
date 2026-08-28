<script lang="ts">
import { fileDisplayName } from "../../lib/utils/documents";
import { documentsStore } from "../../stores/documents.store.svelte";
import Editor from "../editor/Editor.svelte";
import InlineTitle from "../editor/InlineTitle.svelte";
</script>

<main class="center-panel">
  <div class="panel-toolbar" data-tauri-drag-region>
    <div class="doc-breadcrumb">
      {#if documentsStore.active}
        <span class="breadcrumb-item">{fileDisplayName(documentsStore.active.path)}</span>
        <span class="word-count">{documentsStore.active.word_count} palavras</span>
      {:else}
        <span class="breadcrumb-item muted">Selecione um documento</span>
      {/if}
    </div>
  </div>

  <div class="editor-area">
    <InlineTitle />
    <Editor />
  </div>
</main>

<style>
  .center-panel {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--color-bg-base);
    overflow: hidden;
  }

  .panel-toolbar {
    display: flex;
    align-items: center;
    padding: 0 var(--space-4);
    height: 48px;
    border-bottom: 1px solid var(--color-border-subtle);
    background: var(--color-bg-surface);
    flex-shrink: 0;
  }

  .doc-breadcrumb {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    font-size: var(--font-size-sm);
    /* Cliques no texto caem na toolbar (drag region) em vez de virarem alvo próprio */
    pointer-events: none;
  }

  .breadcrumb-item {
    color: var(--color-text-primary);
    font-weight: 500;
  }

  .muted {
    color: var(--color-text-muted);
    font-style: italic;
    font-weight: normal;
  }

  .word-count {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>
