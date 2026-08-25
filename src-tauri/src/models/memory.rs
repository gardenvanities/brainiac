use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub category: MemoryCategory,
    pub fact: String,
    pub relevance: f64,
    pub source_conversation_id: Option<String>,
    pub is_confirmed: bool,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MemoryCategory {
    Preferencia,
    Contexto,
    Habito,
    Projeto,
}

impl MemoryCategory {
    pub fn as_str(&self) -> &str {
        match self {
            MemoryCategory::Preferencia => "preferencia",
            MemoryCategory::Contexto => "contexto",
            MemoryCategory::Habito => "habito",
            MemoryCategory::Projeto => "projeto",
        }
    }
}

impl TryFrom<String> for MemoryCategory {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "preferencia" => Ok(MemoryCategory::Preferencia),
            "contexto" => Ok(MemoryCategory::Contexto),
            "habito" => Ok(MemoryCategory::Habito),
            "projeto" => Ok(MemoryCategory::Projeto),
            _ => Err(format!("Categoria inválida: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMemoryPayload {
    pub category: MemoryCategory,
    pub fact: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemoryPayload {
    pub fact: Option<String>,
    pub category: Option<MemoryCategory>,
    pub relevance: Option<f64>,
    pub is_active: Option<bool>,
}
