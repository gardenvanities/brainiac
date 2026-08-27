import { invoke } from "@tauri-apps/api/core";
import type { Message } from "../../types";

export interface SendMessagePayload {
  conversationId: string;
  content: string;
  documentContext?: string;
}

export async function sendMessage(payload: SendMessagePayload): Promise<Message> {
  return invoke("send_message", { payload });
}

export async function saveApiKey(apiKey: string): Promise<void> {
  return invoke("save_api_key", { apiKey });
}

export async function getApiKey(): Promise<string | null> {
  return invoke("get_api_key");
}
