import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getConversationMessages, getOrCreateConversation } from "../lib/tauri/conversations";
import { getApiKey, sendMessage } from "../lib/tauri/messages";
import type { Conversation, Message } from "../types";

class MessagesStore {
  conversation = $state<Conversation | null>(null);
  messages = $state<Message[]>([]);
  streamingContent = $state<string>("");
  isStreaming = $state(false);
  loading = $state(false);
  error = $state<string | null>(null);
  hasApiKey = $state<boolean>(false);

  async checkApiKey() {
    const key = await getApiKey();
    console.log("checkApiKey chamado, key:", key ? "encontrada" : "não encontrada");
    this.hasApiKey = !!key;
  }

  async loadConversation(documentId?: string) {
    await this.checkApiKey();
    this.loading = true;
    this.error = null;
    try {
      const conv = await getOrCreateConversation(documentId);
      this.conversation = conv;
      this.messages = await getConversationMessages(conv.id);
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  async send(content: string, documentContext?: string) {
    if (!this.conversation || !content.trim()) return;
    if (!this.hasApiKey) return;

    this.error = null;
    try {
      const userMessage = await sendMessage({
        conversationId: this.conversation.id,
        content,
        documentContext,
      });
      this.messages = [...this.messages, userMessage];
      this.isStreaming = true;
      this.streamingContent = "";
    } catch (e) {
      this.error = String(e);
      this.isStreaming = false;
    }
  }

  async setupListeners(): Promise<UnlistenFn[]> {
    const unlisteners: UnlistenFn[] = [];

    unlisteners.push(
      await listen<{ conversation_id: string; content: string }>("message_chunk", (event) => {
        if (event.payload.conversation_id === this.conversation?.id) {
          this.streamingContent += event.payload.content;
        }
      }),
    );

    unlisteners.push(
      await listen<{ conversation_id: string; message: Message }>("message_done", (event) => {
        if (event.payload.conversation_id === this.conversation?.id) {
          this.messages = [...this.messages, event.payload.message];
          this.streamingContent = "";
          this.isStreaming = false;
        }
      }),
    );

    unlisteners.push(
      await listen<string>("app_error", (event) => {
        this.error = event.payload;
        this.isStreaming = false;
        this.streamingContent = "";
      }),
    );

    return unlisteners;
  }
}

export const messagesStore = new MessagesStore();
