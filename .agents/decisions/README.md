# Architettura Decision Records (ADRs) — BRAINIAC

> **Nota:** Esta documentação define **quando** uma decisão merece um ADR e **como** registrá-lo. Referencia `AGENTS.md` (Anexo C) e o template `_template.md` neste mesmo diretório.

## O que é um ADR

Um **ADR (Architecture Decision Record)** é um registro imutável de uma decisão arquitetural **significativa** — que seja difícil ou cara de reverter, ou que afete múltiplas partes do sistema e precise ser lembrada no futuro.

## Quando criar um ADR

Crie um ADR quando a decisão:

- escolhe uma tecnologia central (ex.: banco de dados, motor de UI, runtime);
- define um modelo conceitual (ex.: modelo de memória, arquitetura do editor);
- estabelece uma estratégia transversal (ex.: estratégia de busca, abstração de LLM);
- define persistência ou sincronização;
- define a forma de comunicação frontend/backend (contrato IPC);
- é **difícil ou cara de reverter** (migrar depois custaria muito);
- resolve um trade-off não óbvio que futuros agentes precisarão entender.

### Quando NÃO criar um ADR

- Decisões **triviais** ou puramente locais (nome de variável, detalhe de implementação).
- Decisões que já estão bem cobertas por uma convenção global (`AGENTS.md`) ou Skill.
- Mudanças reversíveis de baixo custo (um refactor pequeno).

> Regra de ouro: se um agente futuro, lendo o código, não conseguir entender **por que** algo foi feito daquele jeito e a reversão for cara, provavelmente merece um ADR.

---

## Formato

Copie `_template.md` e renomeie para `ADR-NNN-titulo-curto.md` (NNN sequencial, 3 dígitos, zero-padded):

```markdown
# ADR-NNN — Título

## Contexto
## Decisão
## Alternativas consideradas
## Consequências
## Status
```

- **Contexto:** a necessidade e as restrições, sem pressupor a solução.
- **Decisão:** o que decidimos e por quê.
- **Alternativas consideradas:** o que foi avaliado e rejeitado (e por quê).
- **Consequências:** trade-offs (mais fácil/difícil/caro/barato).
- **Status:** `Proposto` → `Aceito` → (`Substituído por ADR-NNN` | `Obsoleto`).

---

## Regras

1. **Append-only:** nunca edite um ADR já aceito para mudar seu sentido — crie um novo ADR e marque o antigo como substituído/obsoleto.
2. **Numeração sequencial:** escolha o próximo número livre (`ADR-001`, `ADR-002`, …).
3. **Um ADR = uma decisão:** não agrupe decisões não relacionadas.
4. **Referencie no fluxo:** quando uma feature envolver uma decisão arquitetural, referencie o ADR no `plan.md` (ver skill `spec-driven-flow`).
5. Este diretório fica dentro do sistema de contexto (`.agents/`), coerente com a hierarquia do `AGENTS.md`.