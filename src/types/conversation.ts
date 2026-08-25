export interface Conversation {
  id: string;
  agentId: string;
  documentId: string | null;
  title: string | null;
  modelUsed: string;
  isArchived: boolean;
  createdAt: string;
  updatedAt: string;
}
