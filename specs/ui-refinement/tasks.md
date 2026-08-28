# Tarefas: Refinamento minimalista da UI principal

> Execução em ordem, uma tarefa por vez, TDD onde há lógica (`skills/tdd-workflow`). Critérios de aceite do spec: §3.

## Tarefa 1: Helper puro `isShortcut` (atalhos de teclado)
- Teste: `tests/frontend/keyboard.test.ts` — `mod+b` via ctrlKey e via metaKey (Mac); `mod+shift+f` exige shift; sem modificador → `false`; tecla errada → `false`; modificador extra não especificado → `false` (combinação exata); case-insensitive
- Pronto quando: `bun run test` passa (ciclo vermelho→verde visível)

## Tarefa 2: Toggles no `uiStore` + Vitest para módulos `.svelte.ts`
- Teste: `tests/frontend/ui-store.test.ts` — defaults (ambas abertas); `toggleLeft`/`toggleRight` invertem só o próprio lado; `toggleFocus` fecha ambas e memoriza o estado; repetir restaura o layout anterior
- Pronto quando: Vitest compila `.store.svelte.ts` (plugin svelte reaproveitado do SvelteKit — decisão documentada na task, sem dependência nova real); grep confirma zero consumidores de `activeDocumentId`/`activeAgentId` e flags parasitas removidas

## Tarefa 3: Handler global de atalhos no `AppShell`
- Teste: verificação UI (agent-browser) — `KeyboardEvent` sintético com `ctrlKey` alterna `--left-visible`/`--right-visible` no estilo do shell; sem modificador não altera nada; handler com cleanup de `$effect`
- Pronto quando: os 3 atalhos do spec funcionam e não interceptam digitação sem `mod+`

## Tarefa 4: Grid colapsável + handles ocultos + rails de reabertura
- Teste: verificação UI — sidebar fechada: coluna calculada `0`, handle some (`hidden`), rail de 4px aparece e reabre ao clique; drag continua funcional com sidebar aberta; largura anterior preservada ao reabrir
- Pronto quando: colapso/expansão suaves, sem handle órfão, `Editor` não recria (doc segue aberto)

## Tarefa 5: Chrome morto removido da sidebar esquerda
- Teste: verificação UI — botões "Memórias", "Agentes" e seções "Buscar"/"Tags" inexistentes no DOM; footer só com Configurações; modal de configurações continua abrindo e fechando
- Pronto quando: DOM limpo + `bun run check` verde

## Tarefa 6: Sidebar de IA — header honesto + banner de contexto reativo
- Teste: verificação UI — modelo hardcoded e botão `+` ausentes; banner aparece com o nome real do documento **em contexto na conversa** (`messagesStore.conversation.documentId` → `resolveInlineTitle`) e some por completo sem contexto (sem faixa vazia)
- Pronto quando: verificado + `bun run check` verde

## Tarefa 7: Editor como palco — remover toolbar duplicada + word count sob demanda
- Teste: verificação UI — `CenterPanel` sem breadcrumb de filename nem word count permanente; título visível apenas como H1 inline (`InlineTitle`); word count aparece só no hover do affordance de info; estado vazio simplificado ("Selecione ou crie um documento para começar", sem decoração)
- Pronto quando: nenhum filename duplicado no topo do editor; `InlineTitle` preservado; `bun run check` verde

## Tarefa 8: Separação única e mais leve
- Teste: verificação UI (computed styles) — `border-right/left` dos asides removidos; handle em repouso `--color-border-subtle`; toolbar do editor e headers com `--color-border-subtle`
- Pronto quando: uma única linha de separação por lado + screenshot comparativo

## Tarefa 9: Limpeza acoplada
- Teste: `grep -rn "console.log" src/` sem resultados; testes e check verdes
- Pronto quando: `console.log` de `messages.store` removido; nenhuma flag parasita restante

## Tarefa 10: Verificação final (DoD + verification)
- Teste: `cargo check` (inalterado, DoD) + `cargo test` + `bun run test` + `bun run check` todos verdes; checklist dos 8 critérios de aceite do spec no app rodando (`bun tauri dev` + agent-browser): criar doc, editar (inline title A/B), autosave, enviar mensagem, alternar sidebars, modo foco, banner contextual, drag
- Pronto quando: outputs de sucesso exibidos e commit feito
