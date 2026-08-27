use crate::database::{queries, DbState};
use crate::error::AppError;
use crate::models::provider::{CreateProviderPayload, LlmModel, LlmProvider, SetDefaultModelPayload};
use reqwest::Client;
use serde::Deserialize;
use tauri::State;

#[derive(Deserialize)]
struct OpenAiModelsResponse {
    data: Vec<OpenAiModel>,
}

#[derive(Deserialize)]
struct OpenAiModel {
    id: String,
}

#[tauri::command]
pub async fn add_provider(
    state: State<'_, DbState>,
    payload: CreateProviderPayload,
) -> Result<LlmProvider, AppError> {
    let conn = state.conn.lock().await;
    queries::providers::insert_provider(&conn, &payload.name, &payload.base_url, &payload.api_key).await
}

#[tauri::command]
pub async fn get_providers(state: State<'_, DbState>) -> Result<Vec<LlmProvider>, AppError> {
    let conn = state.conn.lock().await;
    queries::providers::get_providers(&conn).await
}

#[tauri::command]
pub async fn fetch_available_models(
    state: State<'_, DbState>,
    provider_id: String,
) -> Result<Vec<LlmModel>, AppError> {
    let conn = state.conn.lock().await;
    let api_key = queries::providers::get_api_key(&conn, &provider_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Provider não encontrado".to_string()))?;

    let providers = queries::providers::get_providers(&conn).await?;
    let provider = providers.iter().find(|p| p.id == provider_id)
        .ok_or_else(|| AppError::NotFound("Provider não encontrado".to_string()))?;

    let base_url = format!("{}/models", provider.base_url.trim_end_matches('/'));
    drop(conn);

    let client = Client::new();
    let response = client
        .get(&base_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !response.status().is_success() {
        return Err(AppError::Internal(format!("Erro ao buscar modelos: {}", response.status())));
    }

    let models_response: OpenAiModelsResponse = response.json().await
        .map_err(|e| AppError::Internal(format!("Erro ao parsear modelos: {}", e)))?;

    Ok(models_response.data.iter().map(|m| LlmModel {
        id: String::new(),
        provider_id: provider_id.clone(),
        model_id: m.id.clone(),
        name: m.id.clone(),
        is_default: false,
        created_at: String::new(),
    }).collect())
}

#[tauri::command]
pub async fn set_default_model(
    state: State<'_, DbState>,
    payload: SetDefaultModelPayload,
) -> Result<LlmModel, AppError> {
    let conn = state.conn.lock().await;
    queries::providers::upsert_default_model(&conn, &payload.provider_id, &payload.model_id, &payload.model_name).await
}
