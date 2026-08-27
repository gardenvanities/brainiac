use crate::database::{queries, DbState};
use crate::error::AppError;
use crate::models::{conversation::Conversation, message::Message};
use tauri::State;

const DEFAULT_MODEL: &str = "llama-3.3-70b-versatile";
const DEFAULT_AGENT: &str = "00000000-0000-0000-0000-000000000001";

#[tauri::command]
pub async fn get_or_create_conversation(
    state: State<'_, DbState>,
    document_id: Option<String>,
) -> Result<Conversation, AppError> {
    let conn = state.conn.lock().await;
    queries::conversations::get_or_create(
        &conn,
        DEFAULT_AGENT,
        document_id.as_deref(),
        DEFAULT_MODEL,
    )
    .await
}

#[tauri::command]
pub async fn get_conversation_messages(
    state: State<'_, DbState>,
    conversation_id: String,
) -> Result<Vec<Message>, AppError> {
    let conn = state.conn.lock().await;
    queries::messages::get_by_conversation(&conn, &conversation_id, 100).await
}
