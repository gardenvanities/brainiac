use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub path: String,
    pub title: String,
    pub frontmatter: Option<String>, // JSON serializado
    pub word_count: i64,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

// Document + conteúdo do arquivo — usado quando abre um doc para edição
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentWithContent {
    #[serde(flatten)]
    pub document: Document,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocumentPayload {
    pub title: String,
    pub path: String,
    pub frontmatter: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct SaveDocumentPayload {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GetDocumentsFilter {
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
    pub search: Option<String>,
}
