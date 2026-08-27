use crate::error::AppError;
use crate::models::conversation::Conversation;
use chrono::Utc;
use libsql::Connection;
use uuid::Uuid;

pub async fn get_or_create(
    conn: &Connection,
    agent_id: &str,
    document_id: Option<&str>,
    model_used: &str,
) -> Result<Conversation, AppError> {
    let mut rows = conn
        .query(
            "SELECT id, agent_id, document_id, title, model_used, is_archived, created_at, updated_at
             FROM conversations
             WHERE agent_id = ?1 AND document_id IS ?2 AND is_archived = 0
             ORDER BY updated_at DESC LIMIT 1",
            libsql::params![
                agent_id.to_string(),
                document_id.map(|s| s.to_string())
            ],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        return Ok(row_to_conversation(&row)?);
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO conversations (id, agent_id, document_id, title, model_used, is_archived, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, ?4, 0, ?5, ?6)",
        libsql::params![
            id.clone(),
            agent_id.to_string(),
            document_id.map(|s| s.to_string()),
            model_used.to_string(),
            now.clone(),
            now.clone(),
        ],
    )
    .await?;

    Ok(Conversation {
        id,
        agent_id: agent_id.to_string(),
        document_id: document_id.map(|s| s.to_string()),
        title: None,
        model_used: model_used.to_string(),
        is_archived: false,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn update_timestamp(
    conn: &Connection,
    id: &str,
    updated_at: &str,
) -> Result<(), AppError> {
    conn.execute(
        "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
        [updated_at, id],
    )
    .await?;
    Ok(())
}

fn row_to_conversation(row: &libsql::Row) -> Result<Conversation, AppError> {
    let is_archived_int: i64 = row.get(5)?;
    Ok(Conversation {
        id: row.get(0)?,
        agent_id: row.get(1)?,
        document_id: row.get(2)?,
        title: row.get(3)?,
        model_used: row.get(4)?,
        is_archived: is_archived_int != 0,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}
