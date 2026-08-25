use crate::error::AppError;
use libsql::{Builder, Connection};
use std::path::PathBuf;
use tracing::info;

pub struct DbState {
    pub conn: tokio::sync::Mutex<Connection>,
}

impl DbState {
    pub async fn init(db_path: PathBuf) -> Result<Self, AppError> {
        // Garante que o diretório pai existe
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        info!("Inicializando banco de dados em: {:?}", db_path);

        let db = Builder::new_local(db_path).build().await?;
        let conn = db.connect()?;

        // Habilita WAL mode para melhor performance em leitura/escrita concorrente
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .await?;

        // Roda migrations
        super::migrations::run(&conn).await?;

        info!("Banco de dados inicializado com sucesso.");

        Ok(Self {
            conn: tokio::sync::Mutex::new(conn),
        })
    }
}
