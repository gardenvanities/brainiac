<script lang="ts">
import { fileDisplayName } from "../../lib/utils/documents";
import { documentsStore } from "../../stores/documents.store.svelte";
import type { Document } from "../../types";

let { doc }: { doc: Document } = $props();

const isActive = $derived(documentsStore.active?.id === doc.id);

function open() {
  documentsStore.open(doc.id);
}
</script>

<button class="file-item" class:active={isActive} onclick={open}>
  <span class="file-icon">✦</span>
  <span class="file-title">{fileDisplayName(doc.path)}</span>
</button>

<style>
  .file-item {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    width: 100%;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
    transition: all 0.15s ease;
  }

  .file-item:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .file-item.active {
    background: var(--color-accent-primary-subtle);
    color: var(--color-accent-primary);
  }

  .file-icon {
    font-size: 10px;
    opacity: 0.5;
    flex-shrink: 0;
  }

  .file-title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
