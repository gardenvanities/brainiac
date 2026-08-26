---
name: svelte5-patterns
description: Padrões Svelte 5 do projeto BRAINIAC — Runes, stores como classes e convenções de import.
disable-model-invocation: false
---

# BRAINIAC — Padrões Svelte 5

Usar Runes, nunca API legada:
- `$state()` para estado reativo
- `$derived()` para valores computados
- `$effect()` para side effects e lifecycle
- `$props()` para props de componentes

## Stores são classes com propriedades $state

```ts
class NomeStore {
  valor = $state<Tipo>(inicial)
}
export const nomeStore = new NomeStore()
```

Imports de componentes Svelte: sem extensão `.ts`
Sempre: `import { x } from "../stores/x.store.svelte"`
