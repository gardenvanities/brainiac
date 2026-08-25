use crate::error::AppError;
use libsql::Connection;
use tracing::info;

// Embute os arquivos SQL em tempo de compilação — sem dependência de paths em runtime
const MIGRATIONS: &[(&str, &str)] = &[("0001_initial", include_str!("0001_initial.sql"))];

pub async fn run(conn: &Connection) -> Result<(), AppError> {
    // Garante que a tabela de controle existe
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version     TEXT PRIMARY KEY,
            executed_at TEXT NOT NULL
        )",
        (),
    )
    .await?;

    for (version, sql) in MIGRATIONS {
        // Verifica se esta migration já foi executada
        let mut rows = conn
            .query(
                "SELECT version FROM schema_migrations WHERE version = ?1",
                [*version],
            )
            .await?;

        if rows.next().await?.is_some() {
            info!("Migration {} já executada, pulando.", version);
            continue;
        }

        info!("Executando migration {}...", version);

        // Executa o SQL da migration
        conn.execute_batch(sql).await?;

        // Registra como executada
        conn.execute(
            "INSERT INTO schema_migrations (version, executed_at) VALUES (?1, datetime('now'))",
            [*version],
        )
        .await?;

        info!("Migration {} concluída.", version);
    }

    Ok(())
}
