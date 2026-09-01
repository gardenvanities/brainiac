---
name: project-architecture
description: Arquitetura geral do projeto BRAINIAC — stack, estrutura de pastas e princípios. Referencia AGENTS.md (fonte da verdade) e as skills especializadas por camada (tauri-rust-patterns, svelte5-runes, libsql-schema).
disable-model-invocation: false
---

# BRAINIAC — Arquitetura

Stack: Tauri 2 (Rust) + Svelte 5 + TypeScript + LibSQL

Estrutura:
- `src-tauri/src/commands/` → Command Layer (invoke handlers)
- `src-tauri/src/database/` → LibSQL + migrations + queries
- `src-tauri/src/models/` → Structs Rust (Serde)
- `src-tauri/src/llm/` → Cliente HTTP para provedores de IA
- `src-tauri/src/memory/` → Memória adaptativa (extração/injeção)
- `src-tauri/src/filesystem/` → Leitura/escrita de arquivos `.md` e frontmatter
- `src/components/` → Componentes Svelte por painel
- `src/stores/` → Svelte Stores com Runes (`.store.svelte.ts`)
- `src/lib/tauri/` → Wrappers tipados do `invoke()`
- `src/lib/utils/` → Helpers puros de frontend (testáveis)
- `src/types/` → TypeScript types (espelham models Rust)

Princípio: "O preguiçoso trabalha em dobro" —
decisões arquiteturais corretas desde o início.
PKs são UUID v4 (TEXT). Sem auto-increment.

## Separação de responsabilidades

- **Commands** (invoke) → orquestração e acesso a `DbState`/`AppHandle`; **nunca** SQL inline.
- **Queries** (`database/queries/`) → todo SQL e acesso ao banco.
- **Models** (Rust) ↔ **Types** (TS) → contrato do IPC, sempre espelhados.
- **Stores** (Runes) → estado global reativo consumido pelos componentes.

Regras detalhadas por camada estão nas skills `tauri-rust-patterns` (backend), `svelte5-runes` (frontend) e `libsql-schema` (banco). Consulte-as antes de atuar em cada área.