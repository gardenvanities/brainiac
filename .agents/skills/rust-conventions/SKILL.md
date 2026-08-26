---
name: rust-conventions
description: Convenções Rust do projeto BRAINIAC — tratamento de erros, padrão de commands Tauri e acesso ao banco.
disable-model-invocation: false
---

# BRAINIAC — Convenções Rust

Erros: sempre `AppError` via `thiserror`
Comandos Tauri: `async`, `State<'_, DbState>`, `AppHandle` quando precisar de path
Banco: `conn` via `state.conn.lock().await`
Queries ficam em `database/queries/`, não nos commands

## Padrão de command

```rust
#[tauri::command]
pub async fn nome(state: State<'_, DbState>) -> Result<T, AppError>
```

Nunca usar `unwrap()` em produção — sempre propagar com `?`
