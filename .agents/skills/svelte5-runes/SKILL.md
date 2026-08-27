---
name: svelte5-runes
description: Padrões obrigatórios do frontend Svelte 5 do BRAINIAC com exemplos reais — stores como classes com $state interno, untrack() para evitar recriação (caso do Editor.svelte), $effect com cleanup correto e a proibição da sintaxe reativa legada do Svelte 4 ($:).
disable-model-invocation: false
---

# Padrões Svelte 5 (Runes) — BRAINIAC

Esta skill documenta os padrões obrigatórios para o frontend Svelte 5, com exemplos reais extraídos do próprio BRAINIAC.

> **Nota:** Esta skill referencia `AGENTS.md` (fonte da verdade). Regras de tipagem e comunicação Tauri ↔ Svelte também estão lá.

---

## 1. Padrão de store como classe (`$state` interno)

Stores globais são **classes** cujos campos usam a Rune `$state`. O arquivo usa a extensão `.store.svelte.ts` e exporta uma **instância única**.

**Padrão usado em `stores/documents.store.svelte.ts`:**

```ts
import { getDocuments, getDocument, createDocument, saveDocument } from "../lib/tauri/documents";

class DocumentsStore {
  list = $state<Document[]>([]);
  active = $state<DocumentWithContent | null>(null);
  listLoading = $state(false);
  error = $state<string | null>(null);

  async loadList() {
    this.listLoading = true;
    this.error = null;
    try {
      this.list = await getDocuments();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.listLoading = false;
    }
  }
  // ... outros métodos
}

export const documentsStore = new DocumentsStore();
```

**Regras:**
- Campos reativos usam `$state(...)` declarados **dentro da classe**.
- O export é uma instância (`new DocumentsStore()`), não a classe.
- Métodos são async e tratam erro guardando `this.error`, nunca lançando para a UI de forma não controlada.
- Consumo nos componentes: `documentsStore.active` (sem prefixo `$` por ser classe, diferente do store antigo do Svelte 4).

---

## 2. Uso de `untrack()` — caso do `Editor.svelte`

Usar `untrack()` para **ler** estado sem criar dependência reativa. No editor, o `$effect` só deve reconstruir o editor quando o **ID** muda — não a cada conteúdo digitado.

**Padrão usado em `components/editor/Editor.svelte`:**

```svelte
<script lang="ts">
  import { untrack } from "svelte";

  $effect(() => {
    if (!editorRef) return;

    // Rastreia APENAS o ID — mudança de conteúdo não recria o editor
    const docId = documentsStore.active?.id;

    // Lê o conteúdo sem rastrear (untrack = não cria dependência reativa)
    const content = untrack(() => documentsStore.active?.content ?? "");

    // ... cria o Crepe (Milkdown) com `content`
    const crepe = new Crepe({ root: editorRef, defaultValue: content });

    return () => {
      // cleanup...
    };
  });
</script>
```

**Regras:**
- Se o `$effect` dependesse do `content`, cada digitação (que atualiza o conteúdo) recriaria o editor inteiro — bug de performance/UX.
- Rastrear **apenas o campo que deve disparar** a reação (o `id`) e usar `untrack()` para os demais.
- Use `untrack()` sempre que ler estado dentro de `$effect`/derivação sem intenção de criar dependência.

---

## 3. Como criar `$effect` com cleanup correto

O retorno do `$effect` é a **função de limpeza**, executada antes da próxima execução e ao desmontar o componente.

**Padrão usado em `components/editor/Editor.svelte`:**

```svelte
<script lang="ts">
  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    // ... setup do editor
    let observer: MutationObserver | null = null;

    crepe.create().then(() => {
      observer = new MutationObserver(() => { /* ... */ });
      observer.observe(prosemirror, { childList: true, subtree: true, characterData: true });
    });

    // Cleanup: sempre devolver a função que desfaz o que foi criado
    return () => {
      if (saveTimer) clearTimeout(saveTimer);
      observer?.disconnect();
      crepe.destroy();
    };
  });
</script>
```

**Regras:**
- Todo listener, timer, observer ou recurso externo criado no `$effect` **deve** ser desfeito no return.
- `observer.disconnect()`, `clearTimeout()`, `crepe.destroy()` — cada recurso tem sua limpeza.
- Regras de verificação: nenhum recurso "vaza" entre execuções do effect.

---

## 4. Regra: nunca usar sintaxe reativa do Svelte 4 (`$:`)

> **PROIBIDO** usar declarações reativas legadas (`$:`) do Svelte 4.

```svelte
<!-- ❌ NÃO usar (Svelte 4) -->
<!-- $: filtered = items.filter(i => i.active) -->

<script lang="ts">
  // ✅ Svelte 5: use Runes
  import { $derived } from "svelte";

  let items = $state<Item[]>([]);
  let filtered = $derived(items.filter((i) => i.active));
</script>
```

**Regras:**
- Reatividade declarada com valores dependentes usa `$derived`.
- Estado local usa `$state`.
- Efeitos colaterais usam `$effect`.
- Leitura sem dependência usa `untrack()`.
- Nenhuma ocorrência de `$:` deve existir no código do BRAINIAC.