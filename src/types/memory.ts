export type MemoryCategory = "preferencia" | "contexto" | "habito" | "projeto";

export interface Memory {
  id: string;
  category: MemoryCategory;
  fact: string;
  relevance: number;
  sourceConversationId: string | null;
  isConfirmed: boolean;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}
