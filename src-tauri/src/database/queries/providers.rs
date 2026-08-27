use crate::error::AppError;
use crate::models::provider::{LlmModel, LlmProvider};
use chrono::Utc;
use libsql::Connection;
use uuid::Uuid;

pub async fn insert_provider(
    conn: &Connection,
    name: &str,
    base_url: &str,
    api_key: &str,
) -> Result<LlmProvider, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO llm_providers (id, name, base_url, api_key, is_active, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6)",
        libsql::params![id.clone(), name.to_string(), base_url.to_string(), api_key.to_string(), now.clone(), now.clone()],
    ).await?;
    Ok(LlmProvider { id, name: name.to_string(), base_url: base_url.to_string(), is_active: true, created_at: now.clone(), updated_at: now })
}

pub async fn get_providers(conn: &Connection) -> Result<Vec<LlmProvider>, AppError> {
    let mut rows = conn.query(
        "SELECT id, name, base_url, is_active, created_at, updated_at FROM llm_providers WHERE is_active = 1",
        (),
    ).await?;
    let mut providers = Vec::new();
    while let Some(row) = rows.next().await? {
        let is_active: i64 = row.get(3)?;
        providers.push(LlmProvider {
            id: row.get(0)?, name: row.get(1)?, base_url: row.get(2)?,
            is_active: is_active != 0, created_at: row.get(4)?, updated_at: row.get(5)?,
        });
    }
    Ok(providers)
}

pub async fn get_api_key(conn: &Connection, provider_id: &str) -> Result<Option<String>, AppError> {
    let mut rows = conn.query(
        "SELECT api_key FROM llm_providers WHERE id = ?1",
        [provider_id.to_string()],
    ).await?;
    if let Some(row) = rows.next().await? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub async fn upsert_default_model(
    conn: &Connection,
    provider_id: &str,
    model_id: &str,
    model_name: &str,
) -> Result<LlmModel, AppError> {
    conn.execute(
        "UPDATE llm_models SET is_default = 0 WHERE provider_id = ?1",
        [provider_id.to_string()],
    ).await?;

    let mut rows = conn.query(
        "SELECT id FROM llm_models WHERE provider_id = ?1 AND model_id = ?2",
        libsql::params![provider_id.to_string(), model_id.to_string()],
    ).await?;

    let id = if let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        conn.execute(
            "UPDATE llm_models SET is_default = 1 WHERE id = ?1",
            [id.clone()],
        ).await?;
        id
    } else {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO llm_models (id, provider_id, model_id, name, is_default, created_at)
             VALUES (?1, ?2, ?3, ?4, 1, ?5)",
            libsql::params![id.clone(), provider_id.to_string(), model_id.to_string(), model_name.to_string(), now],
        ).await?;
        id
    };

    Ok(LlmModel { id, provider_id: provider_id.to_string(), model_id: model_id.to_string(), name: model_name.to_string(), is_default: true, created_at: String::new() })
}

pub async fn get_default_model(conn: &Connection) -> Result<Option<(LlmModel, String)>, AppError> {
    let mut rows = conn.query(
        "SELECT m.id, m.provider_id, m.model_id, m.name, m.is_default, m.created_at, p.api_key, p.base_url
         FROM llm_models m
         JOIN llm_providers p ON p.id = m.provider_id
         WHERE m.is_default = 1 AND p.is_active = 1
         LIMIT 1",
        (),
    ).await?;
    if let Some(row) = rows.next().await? {
        let is_default: i64 = row.get(4)?;
        let model = LlmModel {
            id: row.get(0)?, provider_id: row.get(1)?, model_id: row.get(2)?,
            name: row.get(3)?, is_default: is_default != 0, created_at: row.get(5)?,
        };
        let api_key: String = row.get(6)?;
        let base_url: String = row.get(7)?;
        Ok(Some((model, format!("{}|{}", api_key, base_url))))
    } else {
        Ok(None)
    }
}
