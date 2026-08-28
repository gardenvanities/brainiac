# Plan: Refinamento minimalista da UI principal

> Plano técnico para o spec aprovado em `spec.md`. Cada item aponta arquivos a tocar, decisões de arquitetura e os pontos de risco levantados no research.

---

## 1. Visão geral da mudança

Tudo se concentra no frontend (`src/`). Sem alteração em backend, schema, comandos Tauri ou dependências. A mudança tem três eixos:

1. **Reativar o que já existe** — `uiStore` já tem `sidebarLeftOpen`/`sidebarRightOpen`; faltam ações, handler de atalho e ligação ao grid.
2. **Remover chrome morto/misleading** — botões e seções sem função no sidebar esquerdo e no chat.
3. **Tornar o editor o protagonista** — bordas duplicadas, banner fixo e densidade de barras rebaixados.

---

## 2. Arquivos a modificar

| Arquivo | Natureza |
|---|---|
| `src/stores/ui.store.svelte.ts` | Adicionar `toggleLeft()`, `toggleRight()`, `toggleFocus()`; remover `activeDocumentId`/`activeAgentId` (sem consumidores). |
| `src/lib/utils/keyboard.ts` *(novo)* | Helper puro `isShortcut(e, combo)` para normalizar Cmd/Ctrl+tecla. Testável por Vitest. |
| `src/components/layout/AppShell.svelte` | Reagir ao `uiStore.sidebarLeftOpen/RightOpen` (colunas vão a 0); handlers globais de atalho via `$effect` + cleanup; ocultar handles quando a sidebar correspondente está fechada; remover `border-right/left` dos asides (sai daqui também a migração visual das bordas, vide §6). |
| `src/components/layout/LeftSidebar.svelte` | Reduzir nav (apenas seção "Arquivos"); footer fica só com botão de Configurações; modal de settings mantém-se aqui. |
| `src/components/layout/RightSidebar.svelte` | Header mostra apenas o nome da aplicação; remover botão `+` inerte; banner de contexto vira reativo (`{#if documentsStore.active}`); remover modelo hardcoded. |
| `src/components/chat/ChatMessages.svelte` | (Sem mudança funcional; pode exigir micro-ajuste se o banner saiu do parent e o `flex:1` precisa absorver). |
| `src/stores/messages.store.svelte.ts` | Remover `console.log` de `checkApiKey`. |
| `tests/frontend/keyboard.test.ts` *(novo)* | Cobre `isShortcut` para Mac/Win/Linux e teclas modificadoras. |
| `tests/frontend/ui-store.test.ts` *(novo)* | Cobre toggles isolados e o modo foco (fecha ambas; abrir um lado não reabre o outro). |

Sem mudanças em `src-tauri/`, `package.json`, `biome.json` ou nos tokens de cor.

---

## 3. Estratégia do grid e do recolhimento (decisão central)

**Hoje:** `grid-template-columns: var(--left-width) 4px 1fr 4px var(--right-width);` (larguras em CSS vars).

**Depois:**

```
grid-template-columns:
  calc(var(--left-width) * var(--left-visible, 0))
  var(--resize-handle-width)
  1fr
  var(--resize-handle-width)
  calc(var(--right-width) * var(--right-visible, 0));
```

Variáveis `--left-visible` / `--right-visible` são `0` ou `1` e vêm de um pequeno estilo inline no `AppShell.svelte`:

```
style="
  --left-width: {uiStore.sidebarLeftWidth}px;
  --right-width: {uiStore.sidebarRightWidth}px;
  --left-visible: {uiStore.sidebarLeftOpen ? 1 : 0};
  --right-visible: {uiStore.sidebarRightOpen ? 1 : 0};
"
```

Por que `* var(--visible, 0)` em vez de `grid-template-columns: 0 ...`?
- Permite transicionar (`transition: grid-template-columns 240ms ease`) sem jank — animando entre `calc(260px * 1)` e `calc(260px * 0)`.
- Mantém `1fr` real, sem gambiarra de overflow.

**Handles ocultos quando a sidebar está fechada:**

```html
<div class="resize-handle" hidden={!uiStore.sidebarLeftOpen} ... />
```

`hidden` é a forma semântica e acessível (esconder do a11y tree e do foco). Drag quando oculto não é possível porque o elemento nem existe para o cursor.

**Abrir uma sidebar fechada pelo mouse:** spec pediu "controle discreto". Estratégia mínima sem novo elemento visual permanente: quando a sidebar esquerda está fechada, **um rail de 4px** aparece na borda da janela com `--color-border-subtle` e click → abre. Idem à direita. Isso satisfaz o "controle visível apenas quando relevante" e não introduz toolbar permanente. Adicionar esse par de rails como `<button class="rail" hidden={open} aria-label="Abrir sidebar">` dentro do AppShell (entre as colunas).

---

## 4. Atalhos de teclado

**Implementação:** único handler em `AppShell.svelte` via `$effect`:

```ts
$effect(() => {
  function onKeydown(e: KeyboardEvent) {
    // exige modificador (nunca captura teclas simples)
    if (!e.ctrlKey && !e.metaKey) return;
    if (isShortcut(e, "mod+b"))     { e.preventDefault(); uiStore.toggleLeft(); }
    else if (isShortcut(e, "mod+j")) { e.preventDefault(); uiStore.toggleRight(); }
    else if (isShortcut(e, "mod+shift+f")) { e.preventDefault(); uiStore.toggleFocus(); }
  }
  window.addEventListener("keydown", onKeydown);
  return () => window.removeEventListener("keydown", onKeydown);
});
```

`isShortcut(e, combo)` é puro: aceita `"mod+b"`, `"mod+shift+f"`, etc., normaliza `mod` para `e.ctrlKey || e.metaKey`, e compara case-insensitive para a tecla principal. Não captura nada dentro de `contenteditable` (a Milkdown) **porque** nenhuma das combinações tem apenas a tecla letra sem modificador — o atalho só dispara com Cmd/Ctrl, e o Milkdown usa shortcuts sem Cmd (negrito, listas) — não há colisão.

**Onde fica documentado:** no `tooltip` dos rails/botões de toggle (futuro) e na própria docstrings do handler. Não adicionar uma lista visível de atalhos na UI (adiciona chrome).

---

## 5. Subtração de chrome morto

### `LeftSidebar.svelte`

- Remover a nav de 3 seções (`✦`/`⌕`/`#`) — apenas `Documentos` permanece como seção.
- Remover botão `+` da seção de busca/tags (já não há seção).
- Footer: de 3 botões → 1. Manter Configurações (ícone ⚙ no canto do header da sidebar ou em um slot mínimo no rodapé). Decisão final: **slot de rodapé com apenas o botão de Configurações** (consistência: manter o footer curto e discreto em vez de mover para o header).
- Manter o modal de configurações como está (continua aqui).

### `RightSidebar.svelte`

- Remover bloco `.agent-info` (avatar + nome + modelo). Header fica com o nome do app "BRAINIAC" discreto à esquerda (já é o que está, mas sem o meta redundante); botão `+` removido por completo.
- `.context-banner` vira `{#if documentsStore.active}` com o nome real do documento (`resolveInlineTitle` já existe em `src/lib/utils/documents.ts`).
- O `+` remover é seguro porque não tem consumidor e o spec define "removido nesta iteração".

---

## 6. Separações visuais

- Remover `border-right` de `.left-sidebar` e `border-left` de `.right-sidebar` em `LeftSidebar.svelte` / `RightSidebar.svelte`.
- Em `AppShell.svelte`, o `.resize-handle` muda a cor em repouso para `--color-border-subtle` (mantém `var(--color-accent-primary)` no `:hover`/`.active`).
- Toolbar do editor (`CenterPanel.svelte`) e header do chat (`RightSidebar.svelte`) trocam `var(--color-border-default)` por `var(--color-border-subtle)` em suas bordas internas.

---

## 7. Limpeza acoplada

- **`messages.store.svelte.ts:17`** — remover o `console.log` (chore obrigatório, alinhado a AGENTS.md).
- **`uiStore.activeDocumentId` / `activeAgentId`** — antes de remover, `grep` em `src/` confirma zero consumidores. Se houver, manter e ligar a um consumidor real; se não, remover.

---

## 8. Testes (TDD onde faz sentido)

| Item | Teste | Tipo |
|---|---|---|
| `isShortcut` | Combinações válidas, normalização Cmd/Ctrl, modificadores extras, case-insensitive, sem modificador → false | Vitest |
| `uiStore.toggleLeft/Right` | Flag inverte, `toggleFocus` fecha ambas, `toggleFocus` reabre se já estavam as duas fechadas | Vitest |
| UI (AppShell, sidebars, banner reativo) | Integração via agent-browser: confirmar colapso do grid, banner só com doc ativo, ausência dos botões mortos, drag continua funcionando quando aberta | agent-browser |

**Por que não testar TDD em tudo:** toggles e matchers têm contrato isolado e justificam teste unitário. O resto é layout/JSX — a verificação é UI (agent-browser), conforme skill `verification`.

---

## 9. Skills ativadas

`spec-driven-flow` · `tdd-workflow` (toggles + shortcut matcher) · `design-system` (qualquer novo CSS — aqui, ajustes de bordas) · `svelte5-runes` (handler global com `$effect`+cleanup) · `verification` (UI com agent-browser) · `code-review` (final).

Não se aplica: `tauri-rust-patterns`/`libsql-schema`/`security`/`performance`/`dependency-policy` (zero dependências novas, zero IPC novo, zero alteração de DB).

---

## 10. Riscos e mitigações

| Risco | Mitigação |
|---|---|
| `$effect` global de teclas acumula listeners em hot reload | Cleanup correto via retorno de `$effect`; verificado em revisão. |
| Colapso do grid recria o Editor | Grid via CSS (`grid-template-columns`) altera layout, não DOM; `documentsStore.active?.id` não muda; `$effect` de `Editor.svelte` não dispara. |
| Drag region da janela Tauri perdida com a redução de barras | `data-tauri-drag-region` permanece no toolbar do editor; sidebar esquerda mantém o header como drag region. |
| Atalhos conflitando com Milkdown | Toda combinação exige `mod+`, Milkdown usa combinações sem `mod+`; não há colisão. |
| Larguras armazenadas em store com sidebar fechada | `sidebarLeftWidth` permanece (não zera); ao reabrir, volta ao tamanho anterior. |
| Persistência entre sessões | Fora do escopo (spec §4). |

---

## 11. Critérios de aceite alinhados ao spec

Mapeamento direto dos 8 critérios de aceite do `spec.md` para o que será executado/verificado:

1–3 (alternância por teclado) → `$effect` global + toggle de flags no `uiStore`.
4 (sem chrome morto) → edição de LeftSidebar/RightSidebar.
5 (banner contextual) → `{#if documentsStore.active}` + `resolveInlineTitle`.
6 (separação única) → remover `border-right/left`, ajustar `--resize-handle` em repouso.
7 (drag continua quando aberta) → handle `hidden={!open}`; drag intacto no estado aberto.
8 (sem regressão de conteúdo) → verificação end-to-end por agent-browser (criar doc, editar, autosave, enviar mensagem).

---

## 12. Fora de escopo confirmado (do spec §4 + decisão deste plano)

- Remoção dos arquivos de rotas vazios (`Settings/Home/Agents/Memories.svelte`, `stores/index.ts`, `lib/events/document-watcher.ts`) — **fica fora**. É código morto, não chrome. Pode entrar como chore numa entrega futura focada em dead-code, com decisão de quem chama.
- Persistência do estado de UI.
- Busca/Tags/Memórias/Agentes funcionais.
- Mobile/responsivo.
- Novos tokens de cor.
