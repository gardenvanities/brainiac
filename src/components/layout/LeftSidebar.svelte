<script lang="ts">
let activeSection: "files" | "search" | "tags" = $state("files");
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
      <span class="icon">󰈚</span>
    </button>
    <button
      class="nav-item"
      class:active={activeSection === "search"}
      onclick={() => (activeSection = "search")}
      title="Buscar"
    >
      <span class="icon">󰍉</span>
    </button>
    <button
      class="nav-item"
      class:active={activeSection === "tags"}
      onclick={() => (activeSection = "tags")}
      title="Tags"
    >
      <span class="icon">󰓹</span>
    </button>
  </nav>

  <div class="sidebar-content">
    {#if activeSection === "files"}
      <div class="section-header">
        <span class="section-title">Documentos</span>
        <button class="icon-btn" title="Novo documento">+</button>
      </div>
      <div class="file-list">
        <!-- FileTree será implementado no Passo 7 -->
        <div class="placeholder-item">Nenhum documento ainda</div>
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
    <button class="footer-btn" title="Memórias">󰧑 Memórias</button>
    <button class="footer-btn" title="Agentes">󰋦 Agentes</button>
    <button class="footer-btn" title="Configurações">󰒓 Configurações</button>
  </div>
</aside>

<style>
  .left-sidebar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-surface);
    overflow: hidden;
    border-right: 1px solid var(--border);
  }

  .sidebar-header {
    padding: var(--space-4);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    height: 48px;
  }

  .logo {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--accent);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: row;
    padding: var(--space-2) var(--space-2);
    gap: var(--space-1);
    border-bottom: 1px solid var(--border);
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
    color: var(--text-secondary);
    cursor: pointer;
    font-size: var(--font-size-md);
    transition: all 0.15s ease;
  }

  .nav-item:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--accent-subtle);
    color: var(--accent);
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
    padding: var(--space-2) var(--space-2);
    margin-bottom: var(--space-1);
  }

  .section-title {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 3px;
    font-size: var(--font-size-md);
    transition: all 0.15s ease;
  }

  .icon-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .search-input {
    width: 100%;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    padding: var(--space-2) var(--space-3);
    outline: none;
    transition: border-color 0.15s ease;
  }

  .search-input:focus {
    border-color: var(--accent);
  }

  .placeholder-item {
    padding: var(--space-2) var(--space-2);
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
  }

  .sidebar-footer {
    display: flex;
    flex-direction: column;
    padding: var(--space-2);
    border-top: 1px solid var(--border);
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
    color: var(--text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    text-align: left;
    transition: all 0.15s ease;
  }

  .footer-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
</style>
