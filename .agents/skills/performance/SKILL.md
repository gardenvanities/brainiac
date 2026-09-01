---
name: performance
description: Revisão de performance proporcional ao impacto no BRAINIAC — Svelte (reatividade, $effect, reconstrução), Tauri (IPC, payloads, serialização, bloqueio), LibSQL (N+1, índices, paginação) e IA (contexto, streaming, caching). Otimize só quando houver impacto mensurável ou risco evidente.
disable-model-invocation: false
---

# Performance Review — BRAINIAC

> **Nota:** Esta skill complementa `code-review` e `AGENTS.md`. Regra central: **não otimizar prematuramente**.

## Princípio

> Otimize quando houver impacto mensurável, risco evidente ou quando a arquitetura da operação exigir.

Não reescreva código legível em busca de micro-otimização sem evidência. A revisão de performance existe para **identificar riscos estruturais**, não para micro-tunar tudo.

---

## Áreas de revisão

### Svelte
- [ ] reatividade desnecessária (campos `$state` que disparam mais do que deveriam);
- [ ] `$effect` excessivo (side effects que rodam mais vezes que o necessário);
- [ ] reconstrução desnecessária de componentes (ex.: `$effect` recriando editor a cada digitação — usar `untrack`);
- [ ] estado excessivamente global (store global onde estado local bastaria).

### Tauri
- [ ] excesso de chamadas IPC (serializar em menos chamadas quando fizer sentido);
- [ ] payloads excessivos (não trafegar dados que o frontend não usa);
- [ ] serialização desnecessária;
- [ ] operações bloqueantes no caminho do command (I/O síncrono no executor async).

### LibSQL
- [ ] N+1 queries (loop com query por item);
- [ ] queries repetitivas (mesma consulta várias vezes onde bastaria uma);
- [ ] ausência de índices quando necessários (`idx_*` para colunas filtradas/ordenadas);
- [ ] paginação (não carregar tudo quando dá para paginar);
- [ ] estratégia de busca adequada (filtros no SQL, não em memória no Rust/TS).

### IA
- [ ] contextos excessivamente grandes (menos contexto, mais precisão);
- [ ] chamadas redundantes ao provedor;
- [ ] streaming usado quando disponível (evita espera total);
- [ ] cancellation suportado (interromper geração abandonada);
- [ ] caching quando justificável (evitar recomputar contexto idêntico).

---

## Resultado

Reporte:

1. **Riscos de performance** identificados (objetivos, não especulativos).
2. **Problemas críticos** — quando a arquitetura da operação claramente degrada (ex.: N+1 em caminho quente).
3. **Problemas não-bloqueantes** — sugestões com a ressalva "só se mensurável".
4. **Conclusão:** `PASS` ou `FAIL`.

Se não houver impacto mensurável ou risco evidente, registrar explicitamente que a revisão foi feita e **nada a otimizar** — isto também é um resultado válido (evita otimização prematura).