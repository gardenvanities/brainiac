<main class="center-panel">
  <div class="panel-toolbar">
    <div class="doc-breadcrumb">
      <span class="breadcrumb-item muted">Selecione um documento</span>
    </div>
    <div class="toolbar-actions">
      <button class="toolbar-btn" title="Visualizar">Prévia</button>
    </div>
  </div>

  <div class="editor-area">
    <!-- Milkdown será integrado aqui no Passo 7 -->
    <div class="editor-placeholder">
      <span class="placeholder-icon">󰈙</span>
      <p>Selecione ou crie um documento para começar</p>
    </div>
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
    justify-content: space-between;
    padding: 0 var(--space-4);
    height: 48px;
    border-bottom: 1px solid var(--border);
    background: var(--bg-surface);
    flex-shrink: 0;
  }

  .doc-breadcrumb {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--font-size-sm);
  }

  .breadcrumb-item {
    color: var(--text-primary);
  }

  .muted {
    color: var(--text-muted);
    font-style: italic;
  }

  .toolbar-actions {
    display: flex;
    gap: var(--space-2);
  }

  .toolbar-btn {
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: var(--space-1) var(--space-3);
    transition: all 0.15s ease;
  }

  .toolbar-btn:hover {
    border-color: var(--border-focus);
    color: var(--text-primary);
  }

  .editor-area {
    flex: 1;
    overflow-y: auto;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .editor-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
    color: var(--text-muted);
    text-align: center;
  }

  .placeholder-icon {
    font-size: 48px;
    opacity: 0.3;
  }

  .editor-placeholder p {
    font-size: var(--font-size-md);
  }
</style>
