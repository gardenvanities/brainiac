mod commands;
mod database;
mod error;
mod filesystem;
mod llm;
mod memory;
mod models;

use database::DbState;
use tauri::Manager;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("Iniciando BRAINIAC...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Falha ao resolver app data dir")
                .join("brainiac.db");

            let db_state = tauri::async_runtime::block_on(async {
                DbState::init(db_path)
                    .await
                    .expect("Falha ao inicializar banco de dados")
            });

            app.manage(db_state);
            info!("BRAINIAC inicializado.");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Documents
            commands::documents::create_document,
            commands::documents::get_documents,
            commands::documents::get_document,
            commands::documents::save_document,
            commands::documents::delete_document,
            // Config
            commands::config::get_config,
            commands::config::save_config,
            commands::config::save_api_key,
            commands::config::get_api_key,
            // Conversations
            commands::conversations::get_or_create_conversation,
            commands::conversations::get_conversation_messages,
            // Messages
            commands::messages::send_message,
            // Providers
            commands::providers::add_provider,
            commands::providers::get_providers,
            commands::providers::fetch_available_models,
            commands::providers::set_default_model,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao executar BRAINIAC");
}
