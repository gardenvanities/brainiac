use super::client::LlmMessage;
use crate::models::message::Message;

pub fn build_system_prompt(memories: &[String], document_context: Option<&str>) -> String {
    let mut prompt = String::from(
        "Você é o BRAINIAC, um segundo cérebro pessoal e adaptativo. \
         Você conhece profundamente o usuário e personaliza cada resposta com base no que aprendeu sobre ele. \
         Seja direto, preciso e útil. Responda sempre no idioma do usuário.\n\n",
    );

    if !memories.is_empty() {
        prompt.push_str("## O que você sabe sobre o usuário:\n");
        for memory in memories {
            prompt.push_str(&format!("- {}\n", memory));
        }
        prompt.push('\n');
    }

    if let Some(context) = document_context {
        if !context.trim().is_empty() {
            prompt.push_str("## Documento aberto atualmente pelo usuário:\n```markdown\n");
            let truncated = if context.len() > 8000 {
                &context[..8000]
            } else {
                context
            };
            prompt.push_str(truncated);
            prompt.push_str("\n```\n\nVocê tem acesso ao conteúdo deste documento. O usuário pode fazer perguntas sobre ele ou pedir que você o edite.\n");
        }
    }

    prompt
}

pub fn build_messages(
    system_prompt: String,
    history: &[Message],
    user_content: &str,
) -> Vec<LlmMessage> {
    let mut messages = vec![LlmMessage {
        role: "system".to_string(),
        content: system_prompt,
    }];

    for msg in history {
        messages.push(LlmMessage {
            role: msg.role.as_str().to_string(),
            content: msg.content.clone(),
        });
    }

    messages.push(LlmMessage {
        role: "user".to_string(),
        content: user_content.to_string(),
    });

    messages
}
