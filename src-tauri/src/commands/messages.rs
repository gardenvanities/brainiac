use crate::database::{queries, DbState};
use crate::error::AppError;
use crate::llm::{client::GroqClient, prompt_builder};
use crate::models::message::{Message, MessageRole};
use chrono::Utc;
use serde::Deserialize;
use tauri::Emitter;
use tauri::{AppHandle, Manager, State};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessagePayload {
    pub conversation_id: String,
    pub content: String,
    pub document_context: Option<String>,
}

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    state: State<'_, DbState>,
    payload: SendMessagePayload,
) -> Result<Message, AppError> {
    // Resolve modelo padrão e credenciais do banco
    let default_model = {
        let conn = state.conn.lock().await;
        queries::providers::get_default_model(&conn).await?
            .ok_or_else(|| AppError::Internal(
                "Nenhum modelo padrão configurado. Adicione um provider em Configurações.".to_string()
            ))?
    };

    let (model_info, credentials) = default_model;
    let parts: Vec<&str> = credentials.splitn(2, '|').collect();
    let api_key = parts[0].to_string();
    let base_url = parts[1].to_string();
    let model_id = model_info.model_id.clone();

    let conn = state.conn.lock().await;

    // Busca histórico ANTES de salvar a nova mensagem
    let history =
        queries::messages::get_by_conversation(&conn, &payload.conversation_id, 50).await?;

    // Salva mensagem do usuário
    let user_message = queries::messages::insert(
        &conn,
        &payload.conversation_id,
        &MessageRole::User,
        &payload.content,
        None,
    )
    .await?;

    // Atualiza timestamp da conversa
    let now = Utc::now().to_rfc3339();
    queries::conversations::update_timestamp(&conn, &payload.conversation_id, &now).await?;

    drop(conn);

    // Monta prompt
    let system_prompt =
        prompt_builder::build_system_prompt(&[], payload.document_context.as_deref());
    let llm_messages = prompt_builder::build_messages(system_prompt, &history, &payload.content);

    // Streaming em background
    let app_clone = app.clone();
    let conversation_id = payload.conversation_id.clone();

    tokio::spawn(async move {
        let client = GroqClient::new(api_key, base_url);

        match client
            .chat_stream(
                &model_id,
                llm_messages,
                &app_clone,
                &conversation_id,
            )
            .await
        {
            Ok(full_content) => {
                let state = app_clone.state::<DbState>();
                let conn = state.conn.lock().await;

                match queries::messages::insert(
                    &conn,
                    &conversation_id,
                    &MessageRole::Assistant,
                    &full_content,
                    Some(model_id.as_str()),
                )
                .await
                {
                    Ok(assistant_message) => {
                        let _ = app_clone.emit(
                            "message_done",
                            serde_json::json!({
                                "conversation_id": conversation_id,
                                "message": assistant_message,
                            }),
                        );
                    }
                    Err(e) => {
                        let _ = app_clone.emit("app_error", e.to_string());
                    }
                }
            }
            Err(e) => {
                let _ = app_clone.emit("app_error", e.to_string());
            }
        }
    });

    Ok(user_message)
}
