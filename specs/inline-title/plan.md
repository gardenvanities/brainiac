# Plano técnico: Inline Title — título inline e sincronização com o painel lateral

> Pré-requisito: `specs/inline-title/spec.md` (aprovado).

## Decisão de arquitetura central

O arquivo `.md` em disco passa a ser **self-describing** (frontmatter + corpo), e o **backend é o único escritor** de arquivo:

- `get_document` passa a retornar o **conteúdo sem o bloco de frontmatter** (só o corpo) — o editor nunca vê o frontmatter.
- `save_document` **recompõe** o arquivo ao gravar: bloco frontmatter (do banco, ou do `title` recebido no payload) + corpo.
- Assim o título inline (Cenário B) atualiza o frontmatter **sem** o conteúdo em memória do Milkdown dessincronizar do disco (o autosave existente continua gravando só o corpo — o backend insere o frontmatter).

## Arquivos afetados

| Arquivo | Ação | Descrição |
|---|---|---|
| `src-tauri/src/filesystem/frontmatter.rs` | Modificar (está vazio) | Helpers puros: `strip_frontmatter(content) -> (Option<Frontmatter>, body)`, `compose_frontmatter(fm, body) -> content` — TDD puro |
| `src-tauri/src/filesystem/documents.rs` | Modificar | `rename_file(old, new)`; gravação via compose_frontmatter |
| `src-tauri/src/error.rs` | Modificar | Variante `Conflict(String)` (`#[error("Conflict: {0}")]`) |
| `src-tauri/src/models/document.rs` | Modificar | `RenameDocumentPayload { id, new_name }`; `SaveDocumentPayload` ganha `title: Option<String>` |
| `src-tauri/src/database/queries/documents.rs` | Modificar | `update_path_and_title(conn, id, path, title, updated_at)`; `path_exists(conn, path) -> bool` |
| `src-tauri/src/commands/documents.rs` | Modificar | Novo command `rename_document`; `save_document` recomponde frontmatter; `get_document` retorna corpo sem frontmatter |
| `src-tauri/src/lib.rs` | Modificar | Registrar `rename_document` |
| `src-tauri/tests/rust/` (ou `#[cfg(test)]`) | Criar | Testes: normalizeção de nome, detecção de conflito (arquivo em disco e row de outro id), rename happy path (tempdir via `std::env::temp_dir`, sem deps novas) |
| `src/types/document.ts` | Modificar | `RenameDocumentPayload`; `SaveDocumentPayload.title?` |
| `src/lib/tauri/documents.ts` | Modificar | Wrapper `renameDocument`; assinatura de `saveDocument` |
| `src/lib/utils/document-title.ts` | Criar | **Lógica pura** (alvo dos testes Vitest): `resolveInlineTitle(doc)`, `fileDisplayName(path)`, `parseFrontmatterTitle(doc)` |
| `src/stores/documents.store.svelte.ts` | Modificar | `renameDocument(id, name)` e `updateTitle(id, title)` — update cirúrgico do nó em `list` (map por id), `active` atualizado sem tocar `currentContent` (editor não recria — `$effect` rastreia só o id) |
| `src/components/editor/InlineTitle.svelte` | Criar | Input do título inline: mostra `resolveInlineTitle(active)`; Enter/blur confirma; decide Cenário A (sem fm title → rename) ou B (com fm title → updateTitle); em conflito exibe "Já existe um arquivo com este nome" e reverte |
| `src/components/layout/CenterPanel.svelte` | Modificar | Hospedar `<InlineTitle />` acima do editor |
| `src/components/sidebar-left/FileTreeItem.svelte` | Modificar | Exibir `fileDisplayName(doc.path)` (nome físico), **não** `doc.title` |
| `tests/frontend/document-title.test.ts` | Criar | Vitest: resolução título/filename, parse de frontmatter, basename |
| `package.json` | Modificar | Dev-dep `vitest` + script `"test": "vitest run"` |

## Schema (se aplicável)

**Nenhuma migration nova.** Tabelas e índices atuais já suportam a feature: `documents.path` é `TEXT UNIQUE` (guarda de conflito no nível do banco) e as colunas `title`/`frontmatter` já existem. No rename, `update_path_and_title` mantém `title` coerente com o novo nome; `updated_at` via `chrono::Utc::now().to_rfc3339()` (RFC 3339, padrão do projeto).

## Commands Tauri (se aplicável)

| Command | Tipo | Contrato |
|---|---|---|
| `rename_document` | **Novo** | `(state, payload: RenameDocumentPayload) -> Result<Document, AppError>` — 404 se id não existe; `AppError::Conflict("Já existe um arquivo com este nome")` se o caminho alvo existe em disco **ou** pertence a outra row; `std::fs::rename` no disco + `update_path_and_title` no banco; retorna o `Document` atualizado |
| `save_document` | Alterado | Payload ganha `title: Option<String>`; gravação do arquivo passa a ser `compose_frontmatter` + corpo; atualiza `frontmatter`/`title` no banco quando title presente |
| `get_document` | Alterado | Retorna `DocumentWithContent` com `content` **sem** o bloco de frontmatter |

Eventos: os nomes `file:renamed` / `file:metadata-updated` do adendo se materializam como **métodos do `documentsStore`** (`renameDocument` / `updateTitle`) — a reatividade de runes substitui o barramento de eventos (não há evento Tauri novo; `document-watcher` segue como está).

## Riscos técnicos e decisões

1. **Dessincronização editor ↔ disco** (maior risco): resolvido com a decisão central — backend único escritor, editor só vê/recebe corpo. O autosave existente (`Editor.svelte`) não pode clobber o frontmatter porque `save_document` sempre recomponde.
2. **Update cirúrgico do sidebar**: `{#each ... (doc.id)}` com replace do item via `map` preserva nós não afetados; `active` é atualizado criando novo objeto com o **mesmo `id`**, então o `$effect` do Editor (que rastreia só o id via `untrack`) não recria o Milkdown.
3. **Conflito de rename**: checagem em disco (`Path::exists`) **e** no banco (`path_exists`), pois a pasta de arquivos pode conter `.md` órfãos não rastreados. Reversão do input é responsabilidade do componente (store não altera nada em caso de erro).
4. **Watcher de arquivos** (`filesystem/watcher.rs` + `document-watcher.ts`): o rename via command pode disparar evento de watcher e um `loadList` extra — idempotente (mesma fonte de verdade), aceitável; monitorar duplicação de refresh.
5. **Vitest + runes**: stores `.store.svelte.ts` exigiriam plugin Svelte no Vitest; decisão — **lógica pura mora em helpers** (`document-title.ts` e backend) e é isso que os testes cobrem; a store fica como orquestração fina. Sem dependência nova além de `vitest` (dev).
6. **Normalização de nome**: mesma regra do `create_document` (espaço → `_`), strip de `.md` informado, rejeição de nome vazio — função pura testável.
