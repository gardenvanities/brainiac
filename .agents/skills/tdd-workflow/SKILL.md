---
name: tdd-workflow
description: Processo TDD obrigatório do projeto BRAINIAC — escrever o teste que falha primeiro, implementação mínima e confirmar o teste passando antes de concluir qualquer task. Aplica-se a commands Tauri, queries de banco e stores.
disable-model-invocation: false
---

# Workflow TDD — BRAINIAC

> **Nota:** Esta skill é referenciada por `AGENTS.md` e é inegociável para qualquer código que toque lógica de negócio (commands, queries, stores).

## Processo TDD — BRAINIAC

1. Antes de escrever qualquer implementação, escreva o teste que descreve o comportamento esperado.
2. Rode o teste e confirme que ele **FALHA** (mostre o output do erro).
3. Escreva o código mínimo necessário para o teste passar.
4. Rode o teste novamente e confirme que **PASSA**.
5. Refatore se necessário, rodando o teste a cada mudança.
6. Nunca marque uma tarefa como concluída sem mostrar o teste passando.

---

## Localização dos Testes

- **Para Rust (Backend):** Testes em `#[cfg(test)]` no mesmo arquivo ou em `tests/rust/`.
- **Para TypeScript (Frontend):** Testes com Vitest em `tests/frontend/`.

---

## Exemplo de Fluxo Esperado na Resposta do Agente

> "Vou escrever o teste primeiro:
> [código do teste]
> Rodando... FALHOU como esperado (função não existe ainda).
> Agora implemento:
> [código da implementação]
> Rodando novamente... PASSOU."