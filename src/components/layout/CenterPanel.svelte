<script lang="ts">
  import { documentsStore } from "../../stores/documents.store.svelte";
  import Editor from "../editor/Editor.svelte";
</script>

<main class="center-panel">
  <div class="panel-toolbar">
    <div class="doc-breadcrumb">
      {#if documentsStore.active}
        <span class="breadcrumb-item">{documentsStore.active.title}</span>
        <span class="word-count">{documentsStore.active.word_count} palavras</span>
      {:else}
        <span class="breadcrumb-item muted">Selecione um documento</span>
      {/if}
    </div>
  </div>

  <div class="editor-area">
    <Editor />
  </div>
</main>

<style>
  .center-panel {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-base);
    overflow: hidden;
  }

  .panel-toolbar {
    display: flex;
    align-items: center;
    padding: 0 var(--space-4);
    height: 48px;
    border-bottom: 1px solid var(--border);
    background: var(--bg-surface);
    flex-shrink: 0;
  }

  .doc-breadcrumb {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    font-size: var(--font-size-sm);
  }

  .breadcrumb-item {
    color: var(--text-primary);
    font-weight: 500;
  }

  .muted {
    color: var(--text-muted);
    font-style: italic;
    font-weight: normal;
  }

  .word-count {
    color: var(--text-muted);
    font-size: var(--font-size-xs);
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
  }
</style>
