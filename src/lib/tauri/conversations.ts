import { invoke } from "@tauri-apps/api/core";
import type { Conversation, Message } from "../../types";

export async function getOrCreateConversation(documentId?: string): Promise<Conversation> {
  return invoke("get_or_create_conversation", { documentId });
}

export async function getConversationMessages(conversationId: string): Promise<Message[]> {
  return invoke("get_conversation_messages", { conversationId });
}
