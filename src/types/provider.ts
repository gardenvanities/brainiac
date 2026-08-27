export interface LlmProvider {
  id: string;
  name: string;
  baseUrl: string;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface LlmModel {
  id: string;
  providerId: string;
  modelId: string;
  name: string;
  isDefault: boolean;
  createdAt: string;
}

export interface CreateProviderPayload {
  name: string;
  baseUrl: string;
  apiKey: string;
}

export interface SetDefaultModelPayload {
  providerId: string;
  modelId: string;
  modelName: string;
}
