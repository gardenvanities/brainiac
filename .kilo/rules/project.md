# BRAINIAC — Contexto do Projeto

## Stack
- Desktop: Tauri 2 (Rust)
- Frontend: Svelte 5 + TypeScript (Runes: $state, $derived, $effect, untrack)
- Banco: LibSQL (UUIDs como PK, sem IDs sequenciais)
- Linting: Biome 2.x
- Package manager: Bun

## Padrões obrigatórios

### Rust
- Erros sempre via `AppError` em `src/error.rs`
- Commands em `src/commands/`, queries em `src/database/queries/`
- Structs com `#[serde(rename_all = "camelCase")]` quando recebem dados do frontend
- PRAGMAs que retornam rows usam `execute_batch()`, não `execute()`
- PKs são sempre UUID v4 como TEXT

### Svelte 5
- Reatividade via Runes ($state, $derived, $effect)
- Nunca usar $: reactive declarations (Svelte 4)
- Stores são classes com $state interno em arquivos `.store.svelte.ts`
- Imports de componentes Svelte sem extensão `.ts` explícita
- `untrack()` para ler estado sem criar dependência reativa no $effect

### TypeScript
- Types espelham exatamente os models Rust
- Campos snake_case do Rust viram camelCase no TypeScript
- Wrappers de invoke em `src/lib/tauri/` — nunca chamar invoke diretamente nos componentes

### Comunicação Tauri ↔ Frontend
- Frontend chama `invoke()` via wrappers tipados em `src/lib/tauri/`
- Eventos de streaming: `message_chunk`, `message_done`, `app_error`
- Estado global via stores em `src/stores/`

## Estrutura de pastas relevante
src-tauri/src/
├── commands/     ← handlers dos invokes
├── database/
│   ├── queries/  ← SQL separado dos commands
│   └── migrations/
├── llm/          ← cliente HTTP para APIs de IA
├── models/       ← structs serde
└── error.rs      ← AppError global

src/
├── components/
├── lib/tauri/    ← wrappers de invoke
├── stores/       ← estado global (.store.svelte.ts)
└── types/        ← interfaces TypeScript
