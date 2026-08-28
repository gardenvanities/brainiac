// Surrogate do editor para testes — espelha EXATAMENTE o padrão de
// dependência do `$effect` de `src/components/editor/Editor.svelte`:
// lê `store.active?.id` (rastreia o CAMPO `active` e a propriedade `id`)
// e o conteúdo via `untrack`. Se o Editor mudar seu padrão de leitura,
// atualize este arquivo para continuar representando-o fielmente.
//
// Por que existe: o bug "renomear apaga o conteúdo" se manifesta na
// fronteira store ↔ componente (substituição do objeto `active` recria
// o Milkdown com `active.content` obsoleto). Componentes exigem DOM para
// montar; este surrogate permite testar essa fronteira sem DOM.

import { untrack } from "svelte";
import { documentsStore } from "../../../src/stores/documents.store.svelte";

export interface EditorSurrogateState {
  /** Quantas vezes o "editor" foi (re)criado — cada criação = 1 rebuild do Milkdown */
  creations: number;
  /** Conteúdo usado em cada criação (o que apareceria no editor) */
  contentsAtCreation: string[];
}

/**
 * Aguarda o scheduler do Svelte 5 concluir: efeitos criados em
 * `$effect.root` rodam no auto-flush em microtask do batch — um macrotask
 * espera TODAS as microtasks pendentes (incluídas as cascateadas), então a
 * espera é determinística.
 */
export function flushReactivity(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

export async function attachEditorSurrogate(
  store: typeof documentsStore = documentsStore,
): Promise<EditorSurrogateState> {
  const state: EditorSurrogateState = { creations: 0, contentsAtCreation: [] };

  $effect.root(() => {
    $effect(() => {
      // Mesmas leituras do $effect de Editor.svelte — na mesma ordem.
      const docId = store.active?.id;
      const content = untrack(() => store.active?.content ?? "");
      if (!docId) return;
      state.creations += 1;
      state.contentsAtCreation.push(content);
    });
  });
  await flushReactivity();

  return state;
}
