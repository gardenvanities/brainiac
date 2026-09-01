---
name: verification
description: Verificação pós-implementação no BRAINIAC — responde 'a feature completa funciona dentro do sistema?' escolhendo o nível certo (unit → integration → UI/E2E → full flow). Complementa o tdd-workflow; não assuma que 'compilou = funciona'.
disable-model-invocation: false
---

# Verification — BRAINIAC

> **Nota:** Esta skill complementa `tdd-workflow` e antecede `code-review` no fluxo. TDD responde *"esta unidade funciona?"*; Verification responde *"a feature completa funciona dentro do sistema?"*.

## Diferença TDD × Verification

| | TDD | Verification |
|---|---|---|
| Pergunta | Esta unidade de comportamento funciona? | A feature completa funciona dentro do sistema? |
| Momento | Antes/durante a implementação | Após a implementação |
| Escopo | Função/query/store isolada | Integração entre peças e com o sistema |

---

## Níveis de verificação

```text
Unit
  ↓
Integration
  ↓
UI / E2E
  ↓
Full flow
```

Escolha o nível **apropriado à mudança** — não force o nível máximo para um bugfix trivial.

### Mapeamento por tipo de mudança

| Mudança | Nível mínimo |
|---|---|
| Função pura | Unit test |
| Store | Unit/integration test |
| Tauri command + database | Integration test |
| Frontend + store + Tauri | Integration test |
| Nova interação visual | UI/browser verification |
| Feature completa | End-to-end quando justificável |

---

## Verificação de UI (quando o dev server for iniciado)

**Nunca** assumir que "compilou = funciona". Verificar explicitamente:

- [ ] carregamento sem erro;
- [ ] erros de console (nenhum `console.error`/exceção não tratada);
- [ ] elementos principais presentes;
- [ ] fluxo afetado pela mudança funciona ponta a ponta;
- [ ] estado inicial correto;
- [ ] estados de erro/loading corretos;
- [ ] interação principal (clique/teclado/confirm) funciona.

---

## Verificação de contrato Tauri ↔ frontend

Quando a mudança toca IPC (commands, payloads, events):

- [ ] nomes dos commands e campos (camelCase no TS vs snake_case/param no Rust) coerentes;
- [ ] payloads de entrada/saída espelham os models (ver `src/types/` vs `src-tauri/src/models/`);
- [ ] eventos de streaming (`message_chunk`, `message_done`, `app_error`) ainda corretos.

## Verificação de migration

Quando a mudança altera schema:

- [ ] nova migration criada (append-only), nunca editada a existente;
- [ ] `IF NOT EXISTS` em CREATE TABLE/INDEX;
- [ ] aplicação idempotente (roda e confirma que não duplica).

---

## Resultado

Reporte:

1. **Nível aplicado** e justificativa (por que este nível é suficiente).
2. **O que foi verificado** (comandos/outputs observados — ver Definition of Done no `AGENTS.md`: mostrar output, não afirmar).
3. **Resultado:** `PASS` ou `FAIL`. `FAIL` em fluxo afetado exige correção antes de seguir para `code-review`.