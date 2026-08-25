use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub model_default: String,
    pub avatar_path: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAgentPayload {
    pub name: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub model_default: String,
    pub avatar_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAgentPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub system_prompt: Option<String>,
    pub model_default: Option<String>,
    pub avatar_path: Option<String>,
}
