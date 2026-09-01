<script lang="ts">
import { FileText } from "@jis3r/icons";
import { fileDisplayName } from "../../lib/utils/documents";
import { documentsStore } from "../../stores/documents.store.svelte";
import type { Document } from "../../types";

let { doc }: { doc: Document } = $props();

const isActive = $derived(documentsStore.active?.id === doc.id);

// Estado por instância: só o item sob o cursor anima (nunca a lista inteira)
let isHovered = $state(false);

function open() {
  documentsStore.open(doc.id);
}
</script>

<button
  class="file-item"
  class:active={isActive}
  onclick={open}
  onmouseenter={() => (isHovered = true)}
  onmouseleave={() => (isHovered = false)}
>
  <span class="file-icon">
    <FileText
      size={14}
      animate={isHovered}
      color={isActive
        ? "var(--color-interactive)"
        : isHovered
          ? "var(--color-text-primary)"
          : "var(--color-text-secondary)"}
    />
  </span>
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
    background: var(--color-interactive-subtle);
    color: var(--color-interactive);
  }

  .file-icon {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .file-title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
