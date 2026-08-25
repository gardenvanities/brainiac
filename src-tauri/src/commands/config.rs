use crate::error::AppError;
use crate::models::config::AppConfig;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const CONFIG_KEY: &str = "app_config";
const STORE_FILE: &str = "config.json";

#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<AppConfig, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let config = match store.get(CONFIG_KEY) {
        Some(val) => serde_json::from_value(val).unwrap_or_default(),
        None => AppConfig::default(),
    };

    Ok(config)
}

#[tauri::command]
pub async fn save_config(app: AppHandle, config: AppConfig) -> Result<AppConfig, AppError> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    store.set(CONFIG_KEY, serde_json::to_value(&config)?);

    store
        .save()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(config)
}
