import { invoke } from "@tauri-apps/api/core";
import type {
  CreateProviderPayload,
  LlmModel,
  LlmProvider,
  SetDefaultModelPayload,
} from "../../types";

export async function addProvider(payload: CreateProviderPayload): Promise<LlmProvider> {
  return invoke("add_provider", { payload });
}

export async function getProviders(): Promise<LlmProvider[]> {
  return invoke("get_providers");
}

export async function fetchAvailableModels(providerId: string): Promise<LlmModel[]> {
  return invoke("fetch_available_models", { providerId });
}

export async function setDefaultModel(payload: SetDefaultModelPayload): Promise<LlmModel> {
  return invoke("set_default_model", { payload });
}
