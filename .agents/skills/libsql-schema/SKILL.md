---
name: libsql-schema
description: Regras obrigatórias do schema LibSQL do BRAINIAC com exemplos reais — CREATE TABLE/INDEX com IF NOT EXISTS, PKs UUID v4 como TEXT, soft delete via is_deleted, timestamps RFC 3339 e migrations append-only (nunca editar uma já aplicada).
disable-model-invocation: false
---

# Schema LibSQL — BRAINIAC

Esta skill documenta as regras obrigatórias para o schema do banco LibSQL do BRAINIAC (migrations, PKs, soft delete e timestamps).

> **Nota:** Esta skill referencia `AGENTS.md` (fonte da verdade). As migrations ficam em `src-tauri/src/database/migrations/` e são executadas automaticamente em `database/connection.rs`.

## Visão geral do schema

Tabelas atuais: `agents`, `conversations`, `messages`, `memories`, `documents`, `llm_usage_log`, `llm_providers`, `llm_models` (definidas em `database/migrations/0001_initial.sql`).

- Todas as PKs são UUID v4 gravadas como `TEXT`.
- Timestamps em ISO 8601 (`TEXT`) via `chrono::Utc::now().to_rfc3339()`.
- Soft delete em entidades principais via `is_deleted`.
- `documents.frontmatter` é armazenado como JSON string.
- Agente padrão (seed): `id = '00000000-0000-0000-0000-000000000001'`.
- Arquivos `.md` dos documentos vivem no filesystem, em `~/.local/share/project.brainiac/files/` (via `AppHandle::path().app_data_dir()`).

---

## 1. Toda tabela/índice novo usa `IF NOT EXISTS`

**Padrão real (extraído de `database/migrations/0001_initial.sql`):**

```sql
CREATE TABLE IF NOT EXISTS documents (
    id          TEXT PRIMARY KEY,
    path        TEXT NOT NULL UNIQUE,
    title       TEXT NOT NULL,
    frontmatter TEXT,
    word_count  INTEGER NOT NULL DEFAULT 0,
    is_deleted  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_documents_path ON documents (path);
```

**Regra:** toda `CREATE TABLE` e todo `CREATE INDEX` **obrigatoriamente** incluem `IF NOT EXISTS`. Isso garante idempotência nas migrations e segurança em execuções repetidas.

---

## 2. PKs sempre UUID v4 como `TEXT`

```sql
id TEXT PRIMARY KEY
```

- Nunca usar auto-incremento (`INTEGER PRIMARY KEY AUTOINCREMENT`) ou IDs sequenciais.
- O UUID é gerado no backend Rust via `Uuid::new_v4().to_string()` (ver `commands/documents.rs`):

```rust
let id = Uuid::new_v4().to_string();
```

- A PK é armazenada como `TEXT`, nunca como BLOB ou INTEGER.

---

## 3. Soft delete via `is_deleted` — nunca DELETE físico

Toda entidade principal tem a coluna `is_deleted INTEGER NOT NULL DEFAULT 0` e a remoção é **lógica**.

**Padrão real (query de soft delete em `database/queries/documents.rs`):**

```rust
pub async fn soft_delete(conn: &Connection, id: &str, updated_at: &str) -> Result<(), AppError> {
    conn.execute(
        "UPDATE documents SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
        [updated_at, id],
    )
    .await?;
    Ok(())
}
```

**Regras:**
- `DELETE FROM` físico é **proibido** para documentos e entidades principais.
- Reads filtram por `is_deleted = 0`:

```sql
SELECT ... FROM documents WHERE is_deleted = 0 ORDER BY updated_at DESC;
```

---

## 4. Timestamps em RFC 3339 (ISO 8601)

Timestamps (`created_at`, `updated_at`) são armazenados como `TEXT` no formato RFC 3339.

```sql
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL
```

No Rust, o timestamp é gerado com `chrono`:

```rust
use chrono::Utc;

let now = Utc::now().to_rfc3339(); // ex.: "2024-01-01T00:00:00Z"
```

**Regra:** nunca armazenar timestamps em epoch/unix ou formatos locais sem offset — sempre ISO 8601 (RFC 3339), normalmente em UTC.

---

## 5. Migrations são append-only — nunca editar migration já aplicada

- Cada migration é um arquivo numerado sequencialmente: `0001_initial.sql`, `0002_*.sql`, `0003_*.sql`, etc.
- **Nunca edite** uma migration que já foi aplicada — para qualquer mudança de schema, **crie uma nova migration** (`0002_*`, `0003_*`, ...).
- As migrations são registradas em `src-tauri/src/database/migrations/mod.rs`:

```rust
// Embute os arquivos SQL em tempo de compilação
const MIGRATIONS: &[(&str, &str)] = &[("0001_initial", include_str!("0001_initial.sql"))];
```

- A tabela de controle `schema_migrations` garante que cada migration é executada **uma única vez**:

```rust
pub async fn run(conn: &Connection) -> Result<(), AppError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version     TEXT PRIMARY KEY,
            executed_at TEXT NOT NULL
        )", ()).await?;

    for (version, sql) in MIGRATIONS {
        // pula se já executada
        let mut rows = conn
            .query("SELECT version FROM schema_migrations WHERE version = ?1", [*version])
            .await?;
        if rows.next().await?.is_some() { continue; }

        conn.execute_batch(sql).await?;
        conn.execute(
            "INSERT INTO schema_migrations (version, executed_at) VALUES (?1, datetime('now'))",
            [*version],
        ).await?;
    }
    Ok(())
}
```

**Regras:**
- Adicionar nova migration = adicionar ao array `MIGRATIONS` **somente no final**.
- Nunca reordenar, renomear ou sobrescrever o conteúdo de migration existente.
- Cada migration nova usa `execute_batch(sql)` para o bloco completo (padrão alinhado à skill `tauri-rust-patterns`).