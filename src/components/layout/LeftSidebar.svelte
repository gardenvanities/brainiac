<script lang="ts">
import { onMount } from "svelte";
import { documentsStore } from "../../stores/documents.store.svelte";
import ProviderSetup from "../shared/ProviderSetup.svelte";
import FileTreeItem from "../sidebar-left/FileTreeItem.svelte";

let creatingDoc = $state(false);
let newDocTitle = $state("");
let showSettings = $state(false);

onMount(() => {
  documentsStore.loadList();
});

async function handleCreate() {
  if (!newDocTitle.trim()) return;
  await documentsStore.create(newDocTitle.trim());
  newDocTitle = "";
  creatingDoc = false;
}

function handleCreateKeydown(e: KeyboardEvent) {
  if (e.key === "Enter") handleCreate();
  if (e.key === "Escape") {
    creatingDoc = false;
    newDocTitle = "";
  }
}

let inputRef = $state<HTMLInputElement | null>(null);

$effect(() => {
  if (creatingDoc && inputRef) {
    inputRef.focus();
  }
});

// Move o foco para dentro do diálogo ao abrir — o teclado começa nele
// (o backdrop sozinho só captura keydown quando o foco está dentro dele)
let modalRef = $state<HTMLDivElement | null>(null);

$effect(() => {
  if (showSettings && modalRef) {
    modalRef.focus();
  }
});
</script>

<!-- Escape fecha o modal a partir de qualquer foco (o backdrop sozinho
     só captura o evento se o foco estiver dentro dele) -->
<svelte:window
  onkeydown={(e) => {
    if (showSettings && e.key === "Escape") showSettings = false;
  }}
/>

<aside class="left-sidebar">
  <div class="sidebar-header" data-tauri-drag-region>
    <span class="logo" aria-label="BRAINIAC" title="BRAINIAC">⬡</span>
    <button
      class="icon-btn"
      title="Novo documento"
      onclick={() => (creatingDoc = true)}
    >+</button>
  </div>

  <div class="sidebar-content">
    <div class="section-header">
      <span class="section-title">Documentos</span>
    </div>

    {#if creatingDoc}
      <div class="new-doc-input">
        <input
          type="text"
          placeholder="Nome do documento..."
          bind:this={inputRef}
          bind:value={newDocTitle}
          onkeydown={handleCreateKeydown}
        />
      </div>
    {/if}

    <div class="file-list">
        {#if documentsStore.listLoading}
          <div class="placeholder-item">Carregando...</div>
        {:else if documentsStore.list.length === 0}
          <div class="placeholder-item">Nenhum documento ainda</div>
        {:else}
          {#each documentsStore.list as doc (doc.id)}
            <FileTreeItem {doc} />
          {/each}
        {/if}
    </div>
  </div>

  <div class="sidebar-footer">
    <button class="footer-btn" onclick={() => (showSettings = !showSettings)}>
      ⚙ Configurações
    </button>
  </div>
</aside>

{#if showSettings}
  <!-- Só fecha quando o clique é no próprio backdrop (target === currentTarget):
       cliques dentro do diálogo não borram para o fechamento -->
  <div
    class="modal-backdrop"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) showSettings = false;
    }}
    onkeydown={(e) => e.key === "Escape" && (showSettings = false)}
  >
    <div
      class="modal"
      role="dialog"
      aria-modal="true"
      aria-label="Configurações"
      tabindex="-1"
      bind:this={modalRef}
    >
      <div class="modal-header">
        <span class="modal-title">Configurações</span>
        <button class="modal-close" title="Fechar" onclick={() => (showSettings = false)}>✕</button>
      </div>
      <div class="modal-body">
        <ProviderSetup />
      </div>
    </div>
  </div>
{/if}

<style>
  .left-sidebar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--color-bg-surface);
    overflow: hidden;
  }

  .sidebar-header {
    padding: 0 var(--space-4);
    border-bottom: 1px solid var(--color-border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 48px;
    flex-shrink: 0;
  }

  .logo {
    font-size: var(--font-size-md);
    color: var(--color-accent-primary);
    /* Cliques no texto caem no header (drag region) em vez de virarem alvo próprio */
    pointer-events: none;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--color-text-secondary);
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: var(--font-size-md);
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .sidebar-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-2);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2);
    margin-bottom: var(--space-1);
  }

  .section-title {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .new-doc-input {
    padding: var(--space-1) var(--space-2);
    margin-bottom: var(--space-1);
  }

  .new-doc-input input {
    width: 100%;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-accent-primary);
    border-radius: 4px;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    padding: var(--space-2) var(--space-3);
    outline: none;
  }

  .placeholder-item {
    padding: var(--space-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
  }

  .sidebar-footer {
    display: flex;
    flex-direction: column;
    padding: var(--space-2);
    border-top: 1px solid var(--color-border-subtle);
    gap: 2px;
  }

  .footer-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
    transition: all 0.15s ease;
  }

  .footer-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 100;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-4);
  }

  .modal {
    background: var(--color-bg-surface);
    border: 1px solid var(--color-border-default);
    border-radius: 8px;
    max-width: 560px;
    width: 100%;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-3) var(--space-4);
    border-bottom: 1px solid var(--color-border-default);
    flex-shrink: 0;
  }

  .modal-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .modal-close {
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: var(--space-1) var(--space-2);
    transition: all 0.15s ease;
  }

  .modal-close:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .modal-body {
    overflow-y: auto;
  }
</style>
