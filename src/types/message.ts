export type MessageRole = "user" | "assistant" | "system";

export interface Message {
  id: string;
  conversationId: string;
  role: MessageRole;
  content: string;
  modelUsed: string | null;
  tokensInput: number | null;
  tokensOutput: number | null;
  createdAt: string;
}
