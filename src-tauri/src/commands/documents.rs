use crate::database::queries;
use crate::database::DbState;
use crate::error::AppError;
use crate::filesystem;
use crate::models::document::{
    CreateDocumentPayload, Document, DocumentWithContent, SaveDocumentPayload,
};
use chrono::Utc;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_document(
    state: State<'_, DbState>,
    payload: CreateDocumentPayload,
) -> Result<Document, AppError> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    filesystem::documents::write_file(&payload.path, "")?;

    let frontmatter = payload
        .frontmatter
        .map(|f| serde_json::to_string(&f))
        .transpose()?;

    let doc = Document {
        id,
        path: payload.path,
        title: payload.title,
        frontmatter,
        word_count: 0,
        is_deleted: false,
        created_at: now.clone(),
        updated_at: now,
    };

    let conn = state.conn.lock().await;
    queries::documents::insert(&conn, &doc).await?;

    Ok(doc)
}

#[tauri::command]
pub async fn get_documents(state: State<'_, DbState>) -> Result<Vec<Document>, AppError> {
    let conn = state.conn.lock().await;
    queries::documents::get_all(&conn).await
}

#[tauri::command]
pub async fn get_document(
    state: State<'_, DbState>,
    id: String,
) -> Result<DocumentWithContent, AppError> {
    let conn = state.conn.lock().await;

    let doc = queries::documents::get_by_id(&conn, &id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Documento {} não encontrado", id)))?;

    let content = filesystem::documents::read_file(&doc.path)?;

    Ok(DocumentWithContent {
        document: doc,
        content,
    })
}

#[tauri::command]
pub async fn save_document(
    state: State<'_, DbState>,
    payload: SaveDocumentPayload,
) -> Result<Document, AppError> {
    let conn = state.conn.lock().await;

    let doc = queries::documents::get_by_id(&conn, &payload.id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Documento {} não encontrado", payload.id)))?;

    filesystem::documents::write_file(&doc.path, &payload.content)?;

    let word_count = filesystem::documents::count_words(&payload.content);
    let now = Utc::now().to_rfc3339();

    queries::documents::update_meta(
        &conn,
        &payload.id,
        &doc.title,
        doc.frontmatter.as_deref(),
        word_count,
        &now,
    )
    .await?;

    queries::documents::get_by_id(&conn, &payload.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Documento não encontrado após save".to_string()))
}

#[tauri::command]
pub async fn delete_document(state: State<'_, DbState>, id: String) -> Result<bool, AppError> {
    let conn = state.conn.lock().await;
    let now = Utc::now().to_rfc3339();
    queries::documents::soft_delete(&conn, &id, &now).await?;
    Ok(true)
}
