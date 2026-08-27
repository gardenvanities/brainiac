use crate::error::AppError;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<LlmMessage>,
    stream: bool,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

pub struct GroqClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GroqClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub async fn chat_stream(
        &self,
        model_id: &str,
        messages: Vec<LlmMessage>,
        app: &AppHandle,
        conversation_id: &str,
    ) -> Result<String, AppError> {
        let url = format!(
            "{}/chat/completions",
            self.base_url.trim_end_matches('/')
        );

        let request = ChatRequest {
            model: model_id.to_string(),
            messages,
            stream: true,
            max_tokens: 4096,
            temperature: 0.7,
        };

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "Groq API error: {}",
                error_text
            )));
        }

        let mut stream = response.bytes_stream();
        let mut full_content = String::new();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AppError::Internal(e.to_string()))?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            loop {
                if let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if let Some(data) = line.strip_prefix("data: ") {
                        if data == "[DONE]" {
                            break;
                        }
                        if let Ok(parsed) = serde_json::from_str::<StreamChunk>(data) {
                            if let Some(content) = parsed
                                .choices
                                .first()
                                .and_then(|c| c.delta.content.as_ref())
                            {
                                full_content.push_str(content);
                                let _ = app.emit(
                                    "message_chunk",
                                    serde_json::json!({
                                        "conversation_id": conversation_id,
                                        "content": content,
                                    }),
                                );
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }

        Ok(full_content)
    }
}
