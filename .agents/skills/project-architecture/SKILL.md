---
name: project-architecture
description: Arquitetura geral do projeto BRAINIAC — stack, estrutura de pastas e princípios.
disable-model-invocation: false
---

# BRAINIAC — Arquitetura

Stack: Tauri (Rust) + Svelte 5 + TypeScript + LibSQL

Estrutura:
- src-tauri/src/commands/ → Command Layer (invoke handlers)
- src-tauri/src/database/ → LibSQL + migrations
- src-tauri/src/models/ → Structs Rust
- src-tauri/src/llm/ → Integração LiteLLM
- src-tauri/src/memory/ → Memória adaptativa
- src/components/ → Componentes Svelte por painel
- src/stores/ → Svelte Stores com Runes
- src/lib/tauri/ → Wrappers tipados do invoke()
- src/types/ → TypeScript types (espelham models Rust)

Princípio: "O preguiçoso trabalha em dobro" —
decisões arquiteturais corretas desde o início.
PKs são UUID v4. Sem auto-increment.
