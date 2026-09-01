---
name: spec-driven-flow
description: Processo obrigatório do projeto BRAINIAC antes de implementar qualquer feature nova — pesquisa (research.md) → spec.md → plan.md → tasks.md → TDD → implementação → verification → code review. Etapas de aprovação pelo usuário antes de seguir; proporcional à complexidade da feature.
disable-model-invocation: false
---

# Fluxo Spec-Driven — BRAINIAC

> **Nota:** Esta skill define o processo obrigatório antes de qualquer implementação de feature nova (não se aplica a bugfixes simples). Referencia `AGENTS.md` (fonte da verdade) e as skills `tdd-workflow`, `verification` e `code-review`.

## Princípio: Research Before Design

> O agente deve **compreender a implementação existente antes de propor uma nova arquitetura**.

Nunca escreva `spec.md` sem antes pesquisar o que já existe. Reutilizar é mais barato e mais seguro do que criar.

---

## Fluxo completo

```text
grilling (se houver ambiguidade — skill `grilling`)
    ↓
research.md
    ↓
spec.md
    ↓
plan.md
    ↓
tasks.md
    ↓
TDD
    ↓
implementação
    ↓
verification
    ↓
code review
```

Iniciativas **grandes** entram antes disso pela skill `wayfinding` (decomposição em perguntas, prototypes, ADRs, handoff entre sessões) e chegam aqui com a árvore de decisões respondida.

A **profundidade** de cada etapa é proporcional à complexidade e ao risco (ver "Fluxo reduzido vs. completo" no final).

---

### 1. `research.md` — O QUE JÁ EXISTE (factual)

Antes de qualquer código, crie `specs/NOME-DA-FEATURE/research.md` investigando, **quando relevante**:

- arquivos existentes relacionados;
- componentes reutilizáveis;
- stores existentes;
- utilities;
- commands;
- queries;
- models;
- types;
- database schema;
- migrations;
- Skills relevantes;
- decisões arquiteturais relevantes (`.agents/decisions/`);
- dependências existentes;
- padrões semelhantes já implementados;
- riscos técnicos;
- possíveis conflitos;
- alternativas de implementação.

**Regras do `research.md`:**
- É **factual** — descreve o que existe, não começa a implementar a feature.
- Não é um `spec.md` disfarçado; não define "comportamento esperado".
- Pode ser curto para features pequenas, mas nunca deve ser pulado quando há código relacionado a descobrir.
- Respeita a regra global **Reuse Before Create** (ver `AGENTS.md`): antes de propor algo novo, aponte o que já existe e poderia ser reutilizado/estendido.

---

### 2. `spec.md` — O QUÊ e POR QUÊ
Após o `research.md`, crie `specs/NOME-DA-FEATURE/spec.md` respondendo:
- Qual problema isso resolve para o usuário?
- Qual o comportamento esperado (casos de uso concretos)?
- O que está FORA de escopo?

> **Pausa Obrigatória:** Pare aqui e aguarde aprovação do usuário sobre o `spec.md` antes de continuar.

---

### 3. `plan.md` — COMO tecnicamente
Após o `spec.md` ser aprovado, crie `specs/NOME-DA-FEATURE/plan.md`:
- Quais arquivos serão criados/modificados
- Quais tabelas/queries são necessárias
- Quais comandos Tauri são necessários
- Riscos técnicos e decisões de arquitetura
- Se houver uma decisão difícil ou cara de reverter, referencie/registre um ADR (ver `.agents/decisions/`)

> **Pausa Obrigatória:** Pare aqui e aguarde aprovação do usuário sobre o `plan.md` antes de continuar.

---

### 4. `tasks.md` — Passos atômicos e testáveis
Quebre o plano em tarefas pequenas, cada uma com:
- O teste que valida a tarefa
- Critério de "pronto" claro

Cada tarefa deve ser implementável e testável isoladamente.

---

### 5. Implementação (TDD)
Só agora o código é escrito, uma tarefa de `tasks.md` por vez, seguindo estritamente a skill `tdd-workflow`.

---

### 6. Verification
Aplicar a skill `verification` no nível apropriado à mudança (unit → integration → UI/E2E → full flow).

---

### 7. Code review
Aplicar a skill `code-review`. Ativar também as revisões de `security`, `performance` e acessibilidade **apenas quando aplicáveis** ao risco da feature. Concluir com `PASS` ou `FAIL` (problemas críticos exigem correção antes de entregar).

---

## Fluxo reduzido vs. completo

A burocracia é proporcional ao risco — a **classificação oficial de complexidade** (trivial / pequena / média / grande) vive em `AGENTS.md`; esta tabela apenas mapeia cada nível ao pipeline desta skill:

| Nível (ver `AGENTS.md`) | Fluxo |
|---|---|
| **Trivial** | Fora do escopo desta skill: research mínima → implementar → verificar |
| **Pequena** | Research (curta) → implementação/testes → verification → review |
| **Média** | Grilling (se ambíguo) → Research → Spec (aprovação) → Plan (aprovação) → Tasks → TDD → implementação → verification → reviews aplicáveis → code review |
| **Grande** | Entrar pela skill `wayfinding` → depois o fluxo Média completo, com ADR(s) e handoff entre sessões |

| Cenário especial | Fluxo |
|---|---|
| **Bug** | Skill `diagnosing-bugs` (reproduzir → hipóteses → causa raiz → regression test) — não abre spec |

---

## Arquitetura conceitual do fluxo (integração entre Skills)

```text
AGENTS.md (regras globais + classificação de complexidade)
    │
    ├── CONTEXT.md (vocabulário de domínio)
    ├── wayfinding (só iniciativas grandes)
    ├── grilling (quando há ambiguidade)
    └── Definition of Done
            │
            ▼
       spec-driven-flow
            │
            ▼
         research
            │
            ▼
           spec
            │
            ▼
           plan
            │
            ▼
          tasks
            │
            ▼
       tdd-workflow
            │
            ▼
       implementation
            │
       ┌────┼───────────────┐
       ▼    ▼               ▼
   security performance  accessibility
       │    │               │
       └────┼───────────────┘
            ▼
       verification
            │
            ▼
       code-review
            │
            ▼
      Definition of Done
```

As Skills são ativadas apenas quando relevantes — nem toda feature exige todas.

---

## Templates de Referência

Utilize os templates localizados em `specs/_template/` como base: `research.md`, `spec.md`, `plan.md` e `tasks.md`.