# Tarefas: Inline Title — título inline e sincronização com o painel lateral

> Execução em ordem, uma tarefa por vez, TDD obrigatório (`skills/tdd-workflow`).

## Tarefa 1: Helpers de frontmatter no backend
- Teste: `#[cfg(test)]` em `filesystem/frontmatter.rs` — `split_frontmatter` (com/sem bloco), `set_title_in_fm` (inserir e substituir), `compose_document` (roundtrip)
- Pronto quando: `cargo test` passa para os 3 helpers puros

## Tarefa 2: Normalização de nome de arquivo (backend)
- Teste: `#[cfg(test)]` — strip de `.md`, espaço→`_`, rejeita vazio, mantém nome simples
- Pronto quando: `cargo test` passa para `normalize_document_name`

## Tarefa 3: Queries `update_path_and_title` e `path_exists`
- Teste: `#[cfg(test)]` em `queries/documents.rs` com LibSQL `:memory:` + migrations — rename persiste, `path_exists` acha/não acha
- Pronto quando: `cargo test` passa

## Tarefa 4: `AppError::Conflict` + `rename_document` (Cenário A)
- Teste: impl testável `rename_document_impl` — happy path (disco renomeado + row atualizada), 404, conflito em disco, conflito no banco, no-op quando nome igual
- Pronto quando: `cargo test` passa com `AppError::Conflict("Já existe um arquivo com este nome")` nos casos de conflito; command registrado no `lib.rs`

## Tarefa 5: Pipeline frontmatter em `save_document`/`get_document` (Cenário B)
- Teste: `save_document_impl` com `title` → arquivo em disco começa com bloco `--- title: ... ---` e corpo preservado; `get_document` retorna corpo **sem** bloco; `merge_title_into_fm_json` atualiza o JSON do banco
- Pronto quando: `cargo test` passa; autosave sem `title` **preserva** frontmatter existente

## Tarefa 6: Helpers puros do frontend + Vitest
- Teste: `tests/frontend/documents.test.ts` — `resolveInlineTitle` (fm title → usa; senão filename sem `.md`), `fileDisplayName` (basename **com** `.md`), `upsertDocument` (replace por id, preserva demais)
- Pronto quando: `bun add -d vitest` feito; `bun test` (vitest run) passa

## Tarefa 7: Types + wrapper `renameDocument`
- Teste: tipagem compila (`bun run check`); contrato espelha payload Rust em camelCase
- Pronto quando: `bun run check` sem erros

## Tarefa 8: Store — `renameDocument` / `updateTitle` com update cirúrgico
- Teste: cobertura via helpers (T6) + verificação manual dos invariantes: nó substituído por id, `active` mantém `id` e `content`, conflito retorna `false` e popula `error`
- Pronto quando: store compila, conflito NÃO altera `list`/`active`, sucesso atualiza só o nó afetado

## Tarefa 9: `InlineTitle.svelte` + hospedagem no `CenterPanel`
- Teste: manual guiado — exibe `resolveInlineTitle`; Enter/blur confirma; Cenário A renomeia; Cenário B não muda sidebar; conflito mostra "Já existe um arquivo com este nome" e reverte o input
- Pronto quando: checklist manual do `bun tauri dev` passa ponta a ponta

## Tarefa 10: Sidebar mostra nome físico
- Teste: `FileTreeItem` renderiza `fileDisplayName(doc.path)`; verificação manual de seleção preservada após rename
- Pronto quando: sidebar nunca exibe frontmatter title; nenhum colapso/perda de seleção

## Tarefa 11: Verificação final (DoD)
- Teste: `cargo test` + `vitest run` + `bun run check` + `cargo check` todos verdes; checklist completo dos Cenários A/B e conflito no app rodando
- Pronto quando: outputs de sucesso exibidos e commit feito
