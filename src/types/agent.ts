export interface Agent {
  id: string;
  name: string;
  description: string | null;
  systemPrompt: string;
  modelDefault: string;
  avatarPath: string | null;
  isDefault: boolean;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateAgentPayload {
  name: string;
  description?: string;
  systemPrompt: string;
  modelDefault: string;
  avatarPath?: string;
}
