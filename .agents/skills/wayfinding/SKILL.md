---
name: wayfinding
description: Navegação de iniciativas grandes no BRAINIAC — trata a iniciativa como árvore/grafo de decisões (decompor perguntas → research → grilling → prototypes → decisões/ADRs → arquitetura → spec → plan → tasks → implementação), com handoff entre sessões. Somente para subsistemas novos ou mudanças arquiteturais; nunca para bugs, ajustes de CSS ou componentes pequenos.
disable-model-invocation: false
---

# Wayfinding — Iniciativas grandes — BRAINIAC

> **Nota:** Esta skill orquestra as outras skills; ela não substitui `spec-driven-flow` (o pipeline spec→plan→tasks→TDD continua valendo) nem `grilling`, `domain-modeling`, `prototype` e `handoff`, que ela convoca. Referencia `AGENTS.md` e `.agents/decisions/README.md` (regras de ADR).

## O que é

Iniciativa grande é aquela que **não cabe em um spec único bem definido de início**: existem perguntas em aberto cujas respostas mudam o desenho. Wayfinding é o método de navegar essa incerteza: a iniciativa é tratada como **árvore/grafo de decisões** — cada pergunta respondida desbloqueia os ramos seguintes — e não como uma lista de tarefas.

## Quando usar

- Sistema completo de memória (extração/relevância/injeção end-to-end);
- sistema de agentes (perfis, execução, ferramentas);
- sincronização (arquivos ↔ índice, multi-janela, backup);
- arquitetura de contexto da IA (o que entra no prompt, orçamento, priorização);
- nova camada de persistência ou busca;
- grandes mudanças no editor.

## Quando NÃO usar

- Bugs (usar `diagnosing-bugs`);
- ajustes de CSS, labels, pequenos componentes;
- feature média bem entendida (basta `spec-driven-flow` + `grilling`);
- mudanças triviais (workflow mínimo — ver `AGENTS.md`).

## Fluxo

```text
Grande iniciativa
       ↓
Decompor em perguntas (árvore de decisões)
       ↓
Research (por pergunta/frente — factual)
       ↓
Grilling (decisões de produto com o usuário)
       ↓
Prototype (quando houver incerteza real)
       ↓
Decisões (ADRs onde couber)
       ↓
Arquitetura (modelo de domínio + fronteiras)
       ↓
Spec (consolidada)
       ↓
Plan → Tasks
       ↓
Implementation (TDD) → Verification → Review
```

As etapas não são estritamente lineares: um prototype pode reabrir uma pergunta; um grilling pode gerar nova pesquisa. O que não pode faltar é o **estado explícito da árvore** (abaixo).

## 1. Decompor em perguntas

Quebre a iniciativa em perguntas cujas respostas **destravam** o desenho, marcando dependências:

```text
"Memória é extraída automaticamente ou só confirmada pelo usuário?"
   ├── sim → extractor roda quando? (fim de conversa? ao vivo?)
   └── não → como o usuário cria memórias?
"Onde mora a relevância — score no banco ou re-ranking no prompt?"
"Contexto da IA tem orçamento de tokens? Quem prioriza: memories ou documento?"
```

- Perguntas **pesquisáveis** (o código/resposta técnica responde) → research, não usuário.
- Perguntas de **produto/arquitetura** → grilling com o usuário.
- Perguntas com **alternativas caras de desfazer** → prototype e/ou ADR.

## 2. Research por frente

Um `research.md` (template em `specs/_template/`) por pergunta ou frente de investigação, dentro de `specs/<iniciativa>/`. Factual: o que existe, o que reutilizar (Reuse Before Create), riscos e alternativas.

## 3. Grilling, prototypes, decisões

- Decisões de produto → `grilling` (rodada única de perguntas agrupadas).
- Incerteza técnica real → `prototype` descartável com pergunta explícita.
- Decisão arquitetural significativa (alternativas múltiplas, reversão cara, multi-subsistema) → **ADR** (formato e regras em `.agents/decisions/README.md`).

## 4. Arquitetura e domínio

- `domain-modeling` para entidades/estados/relações/invariantes do desenho escolhido.
- Fronteiras explícitas: o que fica em cada limite de contexto (Documentos/Editor, Chat/IA, Memória, Config) e nas fronteiras frontend/backend/IPC.

## 5. Spec, plan, tasks

Consolide a árvore respondida em um `spec.md` aprovável (comportamento, escopo, critérios), depois `plan.md` técnico e `tasks.md` atômico — o pipeline normal de `spec-driven-flow`, agora sem ambiguidades abertas.

## 6. Acompanhamento da árvore de decisões

Mantenha no diretório da iniciativa (`specs/<iniciativa>/`) um resumo do estado — normalmente na frente do `research.md` principal ou do `handoff.md`:

```text
Pergunta                          Status       Decisão/Onde
──────────────────────────────────────────────────────────────
Extração automática?              Decidida     ADR-003
Estratégia de relevância          Decidida     spec §3 (score no banco)
Orçamento de contexto             Aberta       — bloqueia plan do injector
Formato de exportação             Pendente     não bloqueia
```

## 7. Handoff entre sessões

Iniciativas grandes atravessam sessões. Use a skill `handoff` (`specs/<iniciativa>/handoff.md`) no fim de **cada sessão** — a árvore de decisões e o "próximo passo" são o núcleo do handoff.

## Anti-padrões

- **Big design up front:** resolver todas as perguntas no papel antes de tocar o código quando um prototype responderia mais barato.
- **Lista de tarefas disfarçada:** pular a decomposição em perguntas e partir direto para tasks — incerteza não respondida vira retrabalho no meio da implementação.
- **Wayfinding para tudo:** aplicá-lo a feature média é burocracia; o gatilho é incerteza estrutural, não tamanho do número de tasks.
- **Árvore abandonada:** decisões tomadas na conversa e nunca registradas no estado da árvore/ADRs.
