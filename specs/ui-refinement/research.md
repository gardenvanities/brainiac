# Research: Refinamento minimalista da UI principal

> Research factual da arquitetura de UI existente, dos padrões reutilizáveis e dos pontos de chrome visual permanente que competem com o conteúdo. Baseado no código real (não em suposição).

---

## 1. Estrutura das três regiões (estado atual)

`src/routes/+layout.svelte` monta o `AppShell`; `+page.svelte` é só um placeholder.

### `AppShell.svelte` — o grid mestre

- Grid de 5 colunas: `left | handle | center | handle | right`.
- Larguras vêm de variáveis CSS `--left-width` / `--right-width` alimentadas por `uiStore.sidebarLeftWidth` (260px) e `sidebarRightWidth` (340px).
- Dois `.resize-handle` (4px, `--resize-handle-width`) renderizados como **linhas visíveis permanentes** com `background: var(--color-border-default)` — além disso, LeftSidebar tem `border-right` e RightSidebar tem `border-left`, ou seja, existem **duas linhas de separação por lado**.
- Não existe nenhum mecanismo para esconder/colapsar uma região.

### `LeftSidebar.svelte` (workspace sidebar)

Estrutura de chrome visível o tempo todo:

1. **Header 48px** — logo `⬡ BRAINIAC` (sempre visível, border-bottom).
2. **Nav de seções** — 3 botões (✦ Arquivos, ⌕ Buscar, # Tags), sempre visíveis; apenas "files" tem conteúdo real; "search" é funcional só como input vazio; "tags" é placeholder fixo.
3. **Conteúdo** — header de seção "Documentos" + botão `+`, input de criação condicional, lista via `FileTreeItem`.
4. **Footer** — 3 botões (`⊕ Memórias`, `⊕ Agentes`, `⊕ Configurações`, sempre visíveis, border-top); apenas Configurações tem ação (abre modal com `ProviderSetup`). Memórias/Agentes são inertes.

Artefatos: modal de configurações vive aqui (backdrop + dialog inline).

### `CenterPanel.svelte` (main workspace)

1. **Toolbar 48px permanente** — breadcrumb com nome físico do arquivo (`fileDisplayName`) + contagem de palavras; é `data-tauri-drag-region`.
2. **`InlineTitle`** — h1 editável com botão ✎ que só aparece no hover (já segue o princípio de "desaparecer").
3. **`Editor`** — max-width 760px centrado, `padding-top: var(--space-6)`; placeholder quando não há doc.

### `RightSidebar.svelte` (AI sidebar)

1. **Header 48px permanente** — avatar B, nome, modelo (hardcoded "claude-sonnet-4-6"; o provider real é Groq), botão "+" inerte; border-bottom.
2. **Banner de contexto permanente** — "Sem documento aberto" estático (não reage ao doc ativo); border-bottom + bg própria.
3. `ChatMessages` (flex:1) e `ChatInput` (textarea + footer com hint + botão Enviar; banner de erro/API key condicional).

---

## 2. Estado de UI existente (reutilizável, Reuse Before Create)

### `uiStore` (`src/stores/ui.store.svelte.ts`)

Já contém **flags de visibilidade não utilizadas**:

```ts
sidebarLeftWidth = $state(260);
sidebarRightWidth = $state(340);
sidebarLeftOpen = $state(true);   // ← declarado, NUNCA lido/escrito
sidebarRightOpen = $state(true);  // ← declarado, NUNCA lido/escrito
activeDocumentId = $state<string | null>(null); // shadow de documentsStore.active?.id — duplicado
activeAgentId = $state("00000000-…");
```

**A infraestrutura de "esconder sidebars" já existe no estado — falta só ser ligada à UI.** Não criar store nova: estender o `uiStore` com actions (`toggleLeft()`, `toggleRight()`, `toggleFocus()`).

### Padrões já aplicados que o refinamento deve seguir

- Reatividade por runes em stores-classe (`*.store.svelte.ts`); componentes leem a instância diretamente.
- Update cirúrgico de listas via `upsertDocument` (`src/lib/utils/documents.ts`).
- `InlineTitle` já é o modelo local de "UI que some": botão de ação só no hover, `error` limpo ao cancelar.
- Editor já quebra em max-width legível; tokens semânticos (`--color-*`) em todos os componentes migrados (design-system).
- `resize-handle` usa `color-mix`/tokens e já tem estado `:hover`/`.active` com `--color-accent-primary` (canal de feedback sem mudar cor de fundo de nada).

## 3. Design system — limites e recursos disponíveis

- Camada de tokens semânticos tem tudo que o refinamento precisa: `--color-bg-base/surface/elevated/hover/active`, `--color-border-subtle/default/strong/focus`, `--color-text-*`, `--color-accent-primary(-subtle/-hover/-active)`, `--color-accent-secondary*` (rosa, reservado a IA/agentes), `--shadow-sm/md/glow-accent`, spacing `--space-1..6`, tipografia `--font-size-*`/`--font-sans/mono` (estes dois ainda no `legacy.css`, em transição).
- Regra: **só tokens semânticos nos componentes**; hover/active via `color-mix(in oklch, …)`. Nenhum token novo de cor é necessário para o refinamento — o trabalho é de **layout/visibilidade/densidade**, não de cor.
- Falta (lacuna real de tokens): nenhum token de **dimensão de cromo** (altura de barra 48px é literal), nenhum token de **transição** (há `transition: all 0.15s ease` repetido em ~10 locais) e nenhuma largura mínima/compacta de sidebar. São tokens de *dimensão/motion*, não de cor — adição mínima no legacy/semantic conforme o plano, sem violar a regra de cor.
- Milkdown traz o CSS próprio (`crepe/style.css`, `frame.css`) e o app o sobrepõe via seletores `:global(.milkdown …)` em `Editor.svelte` — refinamento do editor fica concentrado nesse arquivo.

## 4. Problemas concretos de UI (chrome permanente vs. conteúdo)

Mapeamento do "ruído visual" atual que o refinamento deve eliminar ou tornar evanescente:

| # | Região | Problema | Tipo |
|---|---|---|---|
| 1 | Left | Header + nav + footer sempre visíveis; 2 delas sem função real | chrome morto |
| 2 | Left/Right | Borda do aside + resize-handle ao lado = linhas duplas | redundância visual |
| 3 | Left | Botões "Memórias/Agentes" inertes no footer | chrome morto |
| 4 | Right | Header com modelo hardcoded errado + botão `+` inerte | chrome morto/misleading |
| 5 | Right | Banner "Sem documento aberto" permanente e estático | chrome que devia ser contextual |
| 6 | Center | Toolbar 48px sempre visível mesmo em leitura | densidade |
| 7 | Global | Sidebars sem recolher: impossível dar 100% ao texto | ausente (estado pronto) |
| 8 | Global | Sem atalhos de teclado para alternar painéis/foco | ausente |

## 5. Achados laterais relevantes (registrar, decidir no plano se entram)

- **`console.log` em `messages.store.svelte.ts:17`** — viola AGENTS.md ("Sem Console Logs"). Correção trivial, cabe na mesma entrega como chore.
- **`uiStore.activeDocumentId` / `activeAgentId`** duplicam/parasitam outros stores; o refinamento pode removê-los (não há consumidores no código visual).
- **Arquivos legados vazios**: `src/routes/{Home,Settings,Agents,Memories}.svelte` (0 bytes), `src/stores/index.ts` (vazio), `src/lib/events/document-watcher.ts` (vazio). Rotas não são usadas (o app é single-view via AppShell). Não é escopo visual direto, mas "interface minimalista" se beneficia de não manter fachadas mortas — decisão registrada no plano.
- **atual modelo hardcoded** no `RightSidebar` ("claude-sonnet-4-6") não reflete o provider default real; já existe `config`/`providers` em `src/lib/tauri/providers.ts` e `config.store` — buscar o default real é viável sem nova infra.

## 6. O que já é bom e deve ser preservado

- Grid único com resize por drag (não recriar).
- Inline title com hover-reveal (modelo de referência para outros cromos).
- Editor com max-width/leading apropriados e placeholder.
- Chat com bolhas discretas e banner de erro reutilizando tokens de status.
- Todos os componentes já dependem de tokens semânticos — o refinamento não precisa de nenhuma cor nova.

## 7. Skills relevantes consultadas

- `spec-driven-flow` (este fluxo) · `design-system` (regras de token/cor/camadas — obrigatórias para qualquer CSS) · `svelte5-runes` (stores/runes, `untrack` no Editor — não violar) · `tdd-workflow` (testes antes das partes não puramente visuais) · `code-review`/`verification` (etapas finais) · `dependency-policy` (nenhuma dependência nova é esperada).
