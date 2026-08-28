<script lang="ts">
import { onMount } from "svelte";
import { documentsStore } from "../../stores/documents.store.svelte";
import ProviderSetup from "../shared/ProviderSetup.svelte";
import FileTreeItem from "../sidebar-left/FileTreeItem.svelte";

let activeSection: "files" | "search" | "tags" = $state("files");
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
</script>

<aside class="left-sidebar">
  <div class="sidebar-header">
    <span class="logo">⬡ BRAINIAC</span>
  </div>

  <nav class="sidebar-nav">
    <button
      class="nav-item"
      class:active={activeSection === "files"}
      onclick={() => (activeSection = "files")}
      title="Arquivos"
    >
      ✦
    </button>
    <button
      class="nav-item"
      class:active={activeSection === "search"}
      onclick={() => (activeSection = "search")}
      title="Buscar"
    >
      ⌕
    </button>
    <button
      class="nav-item"
      class:active={activeSection === "tags"}
      onclick={() => (activeSection = "tags")}
      title="Tags"
    >
      #
    </button>
  </nav>

  <div class="sidebar-content">
    {#if activeSection === "files"}
      <div class="section-header">
        <span class="section-title">Documentos</span>
        <button
          class="icon-btn"
          title="Novo documento"
          onclick={() => (creatingDoc = true)}
        >+</button>
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
    {/if}

    {#if activeSection === "search"}
      <div class="section-header">
        <span class="section-title">Buscar</span>
      </div>
      <input class="search-input" type="text" placeholder="Buscar documentos..." />
    {/if}

    {#if activeSection === "tags"}
      <div class="section-header">
        <span class="section-title">Tags</span>
      </div>
      <div class="placeholder-item">Nenhuma tag ainda</div>
    {/if}
  </div>

  <div class="sidebar-footer">
    <button class="footer-btn">⊕ Memórias</button>
    <button class="footer-btn">⊕ Agentes</button>
    <button class="footer-btn" onclick={() => (showSettings = !showSettings)}>
      ⊕ Configurações
    </button>
  </div>
</aside>

{#if showSettings}
  <div
    class="modal-backdrop"
    role="presentation"
    onclick={() => (showSettings = false)}
    onkeydown={(e) => e.key === "Escape" && (showSettings = false)}
  >
    <div class="modal" role="dialog" aria-label="Configurações" onclick={(e) => e.stopPropagation()}>
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
    border-right: 1px solid var(--color-border-default);
  }

  .sidebar-header {
    padding: var(--space-4);
    border-bottom: 1px solid var(--color-border-default);
    display: flex;
    align-items: center;
    height: 48px;
  }

  .logo {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-accent-primary);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: row;
    padding: var(--space-2);
    gap: var(--space-1);
    border-bottom: 1px solid var(--color-border-default);
  }

  .nav-item {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-2);
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-md);
    transition: all 0.15s ease;
  }

  .nav-item:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .nav-item.active {
    background: var(--color-accent-primary-subtle);
    color: var(--color-accent-primary);
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

  .search-input {
    width: 100%;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    padding: var(--space-2) var(--space-3);
    outline: none;
    transition: border-color 0.15s ease;
  }

  .search-input:focus {
    border-color: var(--color-accent-primary);
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
    border-top: 1px solid var(--color-border-default);
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
