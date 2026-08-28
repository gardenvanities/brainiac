<script lang="ts">
import { documentsStore } from "../../stores/documents.store.svelte";
import Editor from "../editor/Editor.svelte";
import InlineTitle from "../editor/InlineTitle.svelte";
</script>

<main class="center-panel">
  <div class="panel-toolbar" data-tauri-drag-region>
    {#if documentsStore.active}
      <span class="word-count-info" aria-label="{documentsStore.active.word_count} palavras">
        <span class="info-glyph" aria-hidden="true">ℹ</span>
        <span class="word-count-tooltip" aria-hidden="true"
          >{documentsStore.active.word_count} palavras</span
        >
      </span>
    {/if}
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
    justify-content: flex-end;
    padding: 0 var(--space-4);
    height: 40px;
    background: var(--color-bg-base);
    flex-shrink: 0;
  }

  /* Progressive disclosure: word count vive num affordance discreto e
     só aparece (tooltip) sob hover/foco. Não há header de filename duplicado. */
  .word-count-info {
    position: relative;
    display: inline-flex;
    align-items: center;
    color: var(--color-text-muted);
    cursor: default;
  }

  .info-glyph {
    font-size: var(--font-size-xs);
    opacity: 0.6;
    line-height: 1;
  }

  .word-count-tooltip {
    position: absolute;
    top: calc(100% + var(--space-1));
    right: 0;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    padding: var(--space-1) var(--space-2);
    font-size: var(--font-size-xs);
    color: var(--color-text-secondary);
    white-space: nowrap;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.12s ease;
    pointer-events: none;
  }

  .word-count-info:hover .word-count-tooltip {
    opacity: 1;
    visibility: visible;
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
</style>
