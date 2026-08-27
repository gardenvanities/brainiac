---
name: tauri-rust-patterns
description: Padrões obrigatórios do backend Tauri/Rust do BRAINIAC com exemplos reais do código — criação de commands, queries separadas, propagação de erros com AppError e a regra de usar execute_batch (nunca execute) para PRAGMAs que retornam linhas.
disable-model-invocation: false
---

# Padrões Tauri + Rust — BRAINIAC

Esta skill documenta os padrões obrigatórios para código Rust no backend Tauri, com exemplos reais extraídos do próprio BRAINIAC.

> **Nota:** Esta skill referencia `AGENTS.md` (fonte da verdade) e complementa a skill `tdd-workflow` (todo command/query exige teste antes da implementação).

---

## 1. Como criar um novo command

Os handlers de invoke ficam em `src-tauri/src/commands/`. Cada função é anotada com `#[tauri::command]` e retorna `Result<T, AppError>`.

**Padrão usado em `commands/documents.rs`:**

```rust
use crate::database::queries;
use crate::database::DbState;
use crate::error::AppError;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn create_document(
    app: AppHandle,
    state: State<'_, DbState>,
    payload: CreateDocumentPayload,
) -> Result<Document, AppError> {
    // 1. Resolve diretório / recursos do app
    let files_dir = resolve_files_dir(&app)?;

    // 2. Gera UUID v4 e timestamp RFC 3339
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 3. Trabalho em filesystem (se necessário)
    filesystem::documents::write_file(&path_str, "")?;

    // 4. Persistência SEMPRE via query (command NÃO fala direto com SQL)
    let conn = state.conn.lock().await;
    queries::documents::insert(&conn, &doc).await?;

    Ok(doc)
}
```

**Regras:**
- O command obtém a conexão via `State<'_, DbState>` e `state.conn.lock().await`.
- O command **nunca escreve SQL inline** — ele delega para `crate::database::queries`.
- Quando precisa acessar o filesystem, usa o `AppHandle` e `app.path().app_data_dir()`.
- Logs de erro usam a variante certa de `AppError` (ex.: `AppError::NotFound(...)`).

---

## 2. Como criar uma query

As operações de banco ficam em `src-tauri/src/database/queries/`. Cada função recebe `&Connection` (protegido pelo Mutex do `DbState`) e retorna `Result<T, AppError>`.

**Padrão usado em `database/queries/documents.rs`:**

```rust
use crate::error::AppError;
use crate::models::document::Document;
use libsql::Connection;

pub async fn insert(conn: &Connection, doc: &Document) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO documents (id, path, title, frontmatter, word_count, is_deleted, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        libsql::params![
            doc.id.clone(),
            doc.path.clone(),
            doc.title.clone(),
            doc.frontmatter.clone(),
            doc.word_count,
            doc.is_deleted as i64,
            doc.created_at.clone(),
            doc.updated_at.clone()
        ],
    )
    .await?;
    Ok(())
}

pub async fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Document>, AppError> {
    let mut rows = conn
        .query(
            "SELECT id, path, title, frontmatter, word_count, is_deleted, created_at, updated_at
             FROM documents WHERE id = ?1",
            [id],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        Ok(Some(row_to_document(&row)?))
    } else {
        Ok(None)
    }
}

fn row_to_document(row: &libsql::Row) -> Result<Document, AppError> {
    let is_deleted_int: i64 = row.get(5)?;
    Ok(Document {
        id: row.get(0)?,
        path: row.get(1)?,
        // ...
        is_deleted: is_deleted_int != 0,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}
```

**Regras:**
- Parâmetros SQL são **sempre posicionais** (`?1`, `?2`, ...) — nunca interpolação de string.
- `Option<T>` é retornado para "achou nenhum" (o command converte para `AppError::NotFound` se precisar).
- A conversão `Row` → struct é extraída em helper próprio (ex.: `row_to_document`).
- Bools e flags são persistidos como `INTEGER` (`0`/`1`) e reconvertidos na leitura.

---

## 3. Como propagar erros com `AppError`

**Definição em `src-tauri/src/error.rs`:**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] libsql::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Necessário para Tauri serializar erros nas respostas dos commands
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
```

**Regras:**
- Usar **sempre `?`** para propagar erros — nunca `.unwrap()` em produção.
- As variantes com `#[from]` permitem conversão automática via `?` (ex.: `libsql::Error`, `std::io::Error`, `serde_json::Error`).
- Para "recurso não encontrado", retornar `AppError::NotFound(...)` explicitamente (ver `get_document` abaixo).
- Todo erro de command precisa ser serializável — por isso o `impl Serialize`.

```rust
#[tauri::command]
pub async fn get_document(
    state: State<'_, DbState>,
    id: String,
) -> Result<DocumentWithContent, AppError> {
    let conn = state.conn.lock().await;

    let doc = queries::documents::get_by_id(&conn, &id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Documento {} não encontrado", id)))?;
    // ...
}
```

---

## 4. Regra: PRAGMA que retorna linha usa `execute_batch`, nunca `execute`

**Padrão usado em `database/connection.rs`:**

```rust
// Habilita WAL mode e foreign_keys — PRAGMAs configuram o banco e NÃO
// retornam um único conjunto de linhas "normal", então use execute_batch.
conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
    .await?;
```

**Motivo:** Em LibSQL/SQLite, certos PRAGMAs (como `PRAGMA journal_mode=WAL`) retornam uma linha com o resultado da configuração. Chamar `execute()` para isso falha ou causa comportamento inesperado, enquanto `execute_batch()` executa múltiplos statements e ignora corretamente os retornos de configuração. Use `execute_batch` para blocos de configuração/PRAGMA e `execute` apenas para DML simples (INSERT/UPDATE/DELETE).