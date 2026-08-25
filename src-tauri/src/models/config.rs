use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub litellm_base_url: String,
    pub default_model: String,
    pub documents_path: String,
    pub theme: AppTheme,
    pub sidebar_left_width: u32,
    pub sidebar_right_width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AppTheme {
    Light,
    Dark,
    System,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            litellm_base_url: "http://localhost:4000".to_string(),
            default_model: "claude-sonnet-4-6".to_string(),
            documents_path: String::new(), // resolvido em runtime
            theme: AppTheme::Dark,
            sidebar_left_width: 260,
            sidebar_right_width: 340,
        }
    }
}
