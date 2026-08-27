use crate::error::AppError;
use crate::models::message::{Message, MessageRole};
use chrono::Utc;
use libsql::Connection;
use uuid::Uuid;

pub async fn insert(
    conn: &Connection,
    conversation_id: &str,
    role: &MessageRole,
    content: &str,
    model_used: Option<&str>,
) -> Result<Message, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO messages (id, conversation_id, role, content, model_used, tokens_input, tokens_output, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, NULL, NULL, ?6)",
        libsql::params![
            id.clone(),
            conversation_id.to_string(),
            role.as_str().to_string(),
            content.to_string(),
            model_used.map(|s| s.to_string()),
            now.clone(),
        ],
    )
    .await?;

    Ok(Message {
        id,
        conversation_id: conversation_id.to_string(),
        role: role.clone(),
        content: content.to_string(),
        model_used: model_used.map(|s| s.to_string()),
        tokens_input: None,
        tokens_output: None,
        created_at: now,
    })
}

pub async fn get_by_conversation(
    conn: &Connection,
    conversation_id: &str,
    limit: i64,
) -> Result<Vec<Message>, AppError> {
    let mut rows = conn
        .query(
            "SELECT id, conversation_id, role, content, model_used, tokens_input, tokens_output, created_at
             FROM messages WHERE conversation_id = ?1
             ORDER BY created_at ASC LIMIT ?2",
            libsql::params![conversation_id.to_string(), limit],
        )
        .await?;

    let mut messages = Vec::new();
    while let Some(row) = rows.next().await? {
        messages.push(row_to_message(&row)?);
    }
    Ok(messages)
}

fn row_to_message(row: &libsql::Row) -> Result<Message, AppError> {
    let role_str: String = row.get(2)?;
    let role = MessageRole::try_from(role_str).map_err(AppError::Internal)?;
    Ok(Message {
        id: row.get(0)?,
        conversation_id: row.get(1)?,
        role,
        content: row.get(3)?,
        model_used: row.get(4)?,
        tokens_input: row.get(5)?,
        tokens_output: row.get(6)?,
        created_at: row.get(7)?,
    })
}
