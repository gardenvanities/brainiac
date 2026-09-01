---
name: code-review
description: Processo obrigatório de revisão de código após a implementação — verifica arquitetura, Svelte/TypeScript, Rust/Tauri, banco, segurança, performance e acessibilidade, e conclui com PASS ou FAIL. Complementa as regras de AGENTS.md e a skill tdd-workflow.
disable-model-invocation: false
---

# Code Review — BRAINIAC

> **Nota:** Esta skill referencia `AGENTS.md` (fonte da verdade) e complementa `tdd-workflow`, `verification`, `security` e `performance`. É executada **após** a implementação e a verification, e **antes** de reportar conclusão (ver Definition of Done no `AGENTS.md`).

## Propósito

Uma revisão objetiva que responde: **o código entregue está correto, consistente com a arquitetura e livre de problemas críticos?**

A revisão produz uma conclusão única:

```text
PASS
```

ou

```text
FAIL
```

`FAIL` em problemas críticos **obriga** a correção antes de considerar a tarefa concluída.

---

## O que revisar (checklist por área)

### Arquitetura
- [ ] separação de responsabilidades respeitada (commands vs queries vs models vs stores);
- [ ] acoplamento adequado (sem dependências indevidas entre camadas);
- [ ] duplicação (respeita a regra **Reuse Before Create** do `AGENTS.md`);
- [ ] abstrações desnecessárias (não criar camada sem justificativa);
- [ ] violações da arquitetura existente (ex.: SQL inline fora de `queries/`);
- [ ] dependências circulares;
- [ ] reutilização inadequada ou inexistente.

### Svelte/TypeScript
- [ ] uso correto de Runes: `$state`, `$derived`, `$effect`, `$props`;
- [ ] `untrack()` onde a leitura não deve criar dependência (caso `Editor.svelte`);
- [ ] lifecycle correto (`$effect` com cleanup quando há listeners/timers/observers);
- [ ] tipagem explícita e consistente com `src/types/` (espelhando models Rust);
- [ ] estado global vs. local (não jogar estado local para store global sem necessidade);
- [ ] reatividade acidental (campos que disparam atualizações indesejadas);
- [ ] performance (sem reconstrução desnecessária de componentes).

### Rust/Tauri
- [ ] nenhum `.unwrap()`/`.expect()` indevido em produção (sempre `?`);
- [ ] tratamento de erros via `AppError`;
- [ ] boundaries entre `commands` e `queries` (command nunca escreve SQL inline);
- [ ] uso correto de `State<'_, DbState>` e `AppHandle`;
- [ ] locks de `Mutex` obtidos e liberados naturalmente (`drop`);
- [ ] async correto (sem bloqueio do executor com trabalho síncrono pesado);
- [ ] clones desnecessários;
- [ ] segurança do IPC (validação de entrada, sem exposição indevida).

### Banco
- [ ] queries corretas e com parâmetros **posicionais** (`?1`, `?2` …);
- [ ] migrations: `IF NOT EXISTS`, append-only (nunca editar migration aplicada);
- [ ] índices adequados (quando a query exige);
- [ ] consistência entre schema, model Rust e type TS;
- [ ] soft delete (`is_deleted`) em read/remove de documentos;
- [ ] timestamps RFC 3339.

### Segurança
(Detalhamento completo na skill `security`; aqui, o foco da revisão de código.)
- [ ] validação de entrada em todo command exposto;
- [ ] filesystem: sem path traversal nem acesso fora dos diretórios permitidos;
- [ ] segredos/chaves nunca em logs nem retornados desnecessariamente;
- [ ] conteúdo não confiável tratado (ex.: documentos do usuário);
- [ ] prompt injection considerado quando aplicável.

### Performance
(Detalhamento na skill `performance`; aqui, o olhar de revisão.)
- [ ] sem IPC excessivo;
- [ ] sem queries repetidas (N+1);
- [ ] sem operações síncronas bloqueantes no caminho quente;
- [ ] sem reatividade/serialização desnecessária.

### Acessibilidade
- [ ] HTML semântico e navegação por teclado em UI interativa;
- [ ] foco gerenciado em dialogs/menus;
- [ ] ARIA apenas quando necessário;
- [ ] estados de loading/error/empty comunicados.

---

## Conclusão

Ao final, reporte:

1. **Veredito:** `PASS` ou `FAIL`.
2. **Problemas críticos** (bloqueiam a entrega) — lista objetiva.
3. **Problemas não-bloqueantes** (sugestões/nitpicks) — separados dos críticos.
4. **Nota quando aplicável:** se uma revisão especializada (`security` / `performance`) foi pulada e por quê (feature sem a superfície correspondente).

Regra: a revisão **não pode** ser "auto-aprovada" sem leitura real do diff/código. Problemas críticos exigem correção e nova revisão antes de reportar conclusão.