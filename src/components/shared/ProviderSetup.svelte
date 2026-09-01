<script lang="ts">
import { onMount } from "svelte";
import {
  addProvider,
  fetchAvailableModels,
  getProviders,
  setDefaultModel,
} from "../../lib/tauri/providers";
import type { LlmModel, LlmProvider } from "../../types";

const PRESET_PROVIDERS = [
  { name: "Groq", baseUrl: "https://api.groq.com/openai/v1" },
  { name: "OpenRouter", baseUrl: "https://openrouter.ai/api/v1" },
  { name: "Mistral", baseUrl: "https://api.mistral.ai/v1" },
  { name: "OpenAI", baseUrl: "https://api.openai.com/v1" },
  {
    name: "Gemini (OpenAI compat)",
    baseUrl: "https://generativelanguage.googleapis.com/v1beta/openai",
  },
];

let providers = $state<LlmProvider[]>([]);
let modelsByProvider = $state<Record<string, LlmModel[]>>({});
let loadingModels = $state<Record<string, boolean>>({});
let defaultModel = $state<{ providerId: string; modelId: string } | null>(null);

let name = $state("");
let baseUrl = $state("");
let apiKey = $state("");
let error = $state<string | null>(null);
let adding = $state(false);

const canAdd = $derived(
  name.trim().length > 0 && baseUrl.trim().length > 0 && apiKey.trim().length > 0 && !adding,
);

onMount(loadProviders);

async function loadProviders() {
  try {
    providers = await getProviders();
  } catch (e) {
    error = String(e);
  }
}

function applyPreset(preset: { name: string; baseUrl: string }) {
  name = preset.name;
  baseUrl = preset.baseUrl;
}

async function handleAdd() {
  if (!canAdd) return;
  adding = true;
  error = null;
  try {
    await addProvider({
      name: name.trim(),
      baseUrl: baseUrl.trim().replace(/\/+$/, ""),
      apiKey: apiKey.trim(),
    });
    apiKey = "";
    await loadProviders();
  } catch (e) {
    error = String(e);
  } finally {
    adding = false;
  }
}

async function handleFetchModels(provider: LlmProvider) {
  loadingModels[provider.id] = true;
  error = null;
  try {
    modelsByProvider[provider.id] = await fetchAvailableModels(provider.id);
  } catch (e) {
    error = String(e);
  } finally {
    loadingModels[provider.id] = false;
  }
}

async function handleSelectModel(provider: LlmProvider, model: LlmModel) {
  error = null;
  try {
    const saved = await setDefaultModel({
      providerId: provider.id,
      modelId: model.modelId,
      modelName: model.name,
    });
    defaultModel = { providerId: saved.providerId, modelId: saved.modelId };
  } catch (e) {
    error = String(e);
  }
}
</script>

<div class="provider-setup">
  <section class="form-section">
    <h2>Adicionar provider</h2>

    <div class="presets">
      {#each PRESET_PROVIDERS as preset}
        <button class="preset-btn" onclick={() => applyPreset(preset)}>
          {preset.name}
        </button>
      {/each}
    </div>

    <div class="form">
      <input type="text" placeholder="Nome (ex: Groq)" bind:value={name} />
      <input type="text" placeholder="Base URL (ex: https://api.groq.com/openai/v1)" bind:value={baseUrl} />
      <input type="password" placeholder="API Key" bind:value={apiKey} />
      <button class="add-btn" onclick={handleAdd} disabled={!canAdd}>
        {adding ? "Adicionando..." : "Adicionar"}
      </button>
    </div>

    {#if error}
      <div class="error-banner">⚠ {error}</div>
    {/if}
  </section>

  <section class="providers-section">
    <h2>Providers cadastrados</h2>

    {#if providers.length === 0}
      <p class="empty">Nenhum provider cadastrado ainda.</p>
    {/if}

    {#each providers as provider (provider.id)}
      <div class="provider-card">
        <div class="provider-header">
          <div class="provider-info">
            <span class="provider-name">{provider.name}</span>
            <span class="provider-url">{provider.baseUrl}</span>
          </div>
          <button
            class="fetch-btn"
            onclick={() => handleFetchModels(provider)}
            disabled={loadingModels[provider.id]}
          >
            {loadingModels[provider.id] ? "Carregando..." : "Carregar modelos"}
          </button>
        </div>

        {#if modelsByProvider[provider.id]}
          <div class="models-list">
            {#if modelsByProvider[provider.id].length === 0}
              <p class="empty">Nenhum modelo retornado.</p>
            {/if}
            {#each modelsByProvider[provider.id] as model (model.modelId)}
              {@const isDefault =
                defaultModel?.providerId === provider.id &&
                defaultModel?.modelId === model.modelId}
              <button
                class="model-btn"
                class:default={isDefault}
                onclick={() => handleSelectModel(provider, model)}
              >
                {model.name}
                {#if isDefault}
                  <span class="default-badge">padrão</span>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </section>
</div>

<style>
  .provider-setup {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-3);
  }

  h2 {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-primary);
    margin: 0 0 var(--space-2) 0;
  }

  .presets {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin-bottom: var(--space-2);
  }

  .preset-btn {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-2);
    transition: all 0.15s ease;
  }

  .preset-btn:hover {
    border-color: var(--color-interactive);
    color: var(--color-interactive);
  }

  .form {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .form input {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    padding: var(--space-2) var(--space-3);
    outline: none;
    transition: border-color 0.15s ease;
  }

  .form input:focus {
    border-color: var(--color-interactive);
  }

  .form input::placeholder {
    color: var(--color-text-muted);
  }

  .add-btn {
    align-self: flex-start;
    background: var(--color-interactive);
    border: none;
    border-radius: 4px;
    color: white;
    cursor: pointer;
    font-size: var(--font-size-sm);
    padding: var(--space-2) var(--space-4);
    transition: opacity 0.15s ease;
  }

  .add-btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .error-banner {
    background: rgba(255, 100, 100, 0.1);
    border: 1px solid rgba(255, 100, 100, 0.2);
    border-radius: 4px;
    color: #ff8080;
    font-size: var(--font-size-xs);
    margin-top: var(--space-2);
    padding: var(--space-2) var(--space-3);
  }

  .empty {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    font-style: italic;
    margin: 0;
  }

  .provider-card {
    background: var(--color-bg-surface);
    border: 1px solid var(--color-border-default);
    border-radius: 6px;
    margin-bottom: var(--space-2);
    padding: var(--space-3);
  }

  .provider-header {
    align-items: center;
    display: flex;
    gap: var(--space-2);
    justify-content: space-between;
  }

  .provider-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }

  .provider-name {
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    font-weight: 600;
  }

  .provider-url {
    color: var(--color-text-muted);
    font-family: var(--font-body);
    font-size: var(--font-size-xs);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .fetch-btn {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    flex-shrink: 0;
    font-size: var(--font-size-xs);
    padding: var(--space-1) var(--space-3);
    transition: all 0.15s ease;
  }

  .fetch-btn:hover:not(:disabled) {
    border-color: var(--color-interactive);
    color: var(--color-interactive);
  }

  .fetch-btn:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .models-list {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin-top: var(--space-2);
  }

  .model-btn {
    align-items: center;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-default);
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
    display: inline-flex;
    font-size: var(--font-size-xs);
    gap: var(--space-1);
    padding: var(--space-1) var(--space-2);
    transition: all 0.15s ease;
  }

  .model-btn:hover {
    border-color: var(--color-interactive);
    color: var(--color-interactive);
  }

  .model-btn.default {
    background: var(--color-interactive-subtle);
    border-color: var(--color-interactive);
    color: var(--color-interactive);
  }

  .default-badge {
    background: var(--color-interactive);
    border-radius: 3px;
    color: white;
    font-size: var(--font-size-xs);
    padding: 1px 4px;
    text-transform: uppercase;
  }
</style>
