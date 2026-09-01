# Agentic Workflow — Teste de integração do sistema de Skills

> **Propósito:** validar que o sistema de Skills do BRAINIAC tem **comportamento coerente** — dado um pedido típico, o caminho esperado é previsível, proporcional e sem etapas desnecessárias. Este documento é referência de treinamento/verificação, não regra nova: as fontes são o `AGENTS.md` (§8, §10) e as Skills.

## Visão geral do fluxo

```text
                User Request
                     │
                     ▼
          Complexity Check (AGENTS.md §8)
          + Consulta a CONTEXT.md (vocabulário)
          + Research antes de modificar (Reuse Before Create)
                     │
      ┌──────────────┼──────────────────┐
      ▼              ▼                  ▼
   Trivial        Pequena/Média          Grande
      │              │                  │
      ▼              ▼                  ▼
  Implement     spec-driven-flow      wayfinding
      │         (+ grilling se          │
      │          ambíguo)                │
      ▼              ▼                  ▼
  Verify        bugs? → diagnosing-bugs
      │              │
      └──────────────┴──► verification → code-review → DoD
```

Regra de ouro: **o mínimo de processo para alta confiança** — nenhum passo é obrigatório quando a mudança não o justifica.

---

## Cenário 1 — Feature trivial

> **Pedido:** "Corrija o padding desse botão."

| Passo | Skill | Observação |
|---|---|---|
| Classificação | — | **Trivial** (§8) |
| Research mínima | — | olhar o componente afetado; `design-system` se tocar token de espaçamento |
| Implementar | — | — |
| Verify | `verification` | nível mínimo (visual/compilação; sem TDD pois não há lógica) |

**Não aciona:** grilling, spec, plan, ADR, prototype, wayfinding, handoff.

---

## Cenário 2 — Feature média

> **Pedido:** "Adicione suporte para favoritos nos documentos."

| Passo | Skill | Observação |
|---|---|---|
| Classificação | — | **Média**: nova feature, altera modelo |
| Grilling | `grilling` | decisões de usuário: favorito é por Document? global ou por pasta? aparece onde? Persiste onde? Perguntas pesquisáveis (schema, stores existentes) são resolvidas no código, não com o usuário |
| Research | `spec-driven-flow` | `research.md`: documentos, stores, commands, queries existentes; Reuse Before Create |
| Domain check | `domain-modeling` | conceito novo ("favorito")? Se sim: definir e atualizar `CONTEXT.md`; impacto no schema (migration) |
| Spec | `spec-driven-flow` | `spec.md` com decisões do grilling → **aprovação do usuário** |
| Plan | `spec-driven-flow` | `plan.md` técnico → **aprovação** |
| Tasks | `spec-driven-flow` | tasks atômicas com testes |
| Implementação | `tdd-workflow` | teste que falha primeiro, por task |
| Verify | `verification` | integração (store + command + banco) e fluxo na UI |
| Reviews | `code-review` (+ `security`/`performance` se aplicável) | veredito PASS/FAIL |
| DoD | `AGENTS.md` | checks, commit |

**Não aciona:** prototype (sem incerteza de design plausível múltipla), wayfinding (não é subsistema), ADR (decisão reversível), handoff (cabe em sessão — se esticar, entra `handoff`).

---

## Cenário 3 — Feature grande

> **Pedido:** "Implemente o sistema de memória do Brainiac."

| Passo | Skill | Observação |
|---|---|---|
| Classificação | — | **Grande**: novo subsistema, domínio e arquitetura |
| Wayfinding | `wayfinding` | árvore de decisões: extração automática? onde mora a relevância? orçamento de contexto? quando injetar? |
| Grilling | `grilling` | decisões de produto (categorias de Memory já existem em `CONTEXT.md` — reutilizar vocabulário) |
| Domain modeling | `domain-modeling` | entidades/estados/relações/invariantes; atualizar `CONTEXT.md` com conceitos novos |
| Research | `spec-driven-flow` | por frente: `memory/` (extractor, injector, relevance), prompt_builder, schema |
| Prototype | `prototype` | só onde houver incerteza real (ex.: estratégia de relevância); descartável, com pergunta explícita |
| Decisões | ADRs | escolhas difíceis de reverter (ex.: quando extrair, como pontuar) em `.agents/decisions/` |
| Spec → Plan → Tasks | `spec-driven-flow` | consolidado, com aprovações |
| Implementação | `tdd-workflow` | por task, entre sessões |
| Handoff | `handoff` | `specs/<iniciativa>/handoff.md` ao fim de cada sessão |
| Verify → Reviews | `verification`, `code-review`, `security`, `performance` | IA envolvida: prompt injection / custo de contexto revisados |

---

## Cenário 4 — Bug

> **Pedido:** "O documento às vezes não salva."

| Passo | Skill | Observação |
|---|---|---|
| Classificação | — | Bug → `diagnosing-bugs` (não abre spec) |
| Reproduzir | `diagnosing-bugs` | reprodução determinística antes de qualquer alteração |
| Isolar | `diagnosing-bugs` | em que lado da fronteira (store → wrapper → command → query → banco) o estado diverge |
| Hipóteses | `diagnosing-bugs` | H1 persistência falha em silêncio · H2 estado obsoleto · H3 evento/watcher · H4 race (ex.: autosave vs. rename) |
| Causa raiz | `diagnosing-bugs` | evidência por hipótese; instrumentação temporária removida depois |
| Correção | `tdd-workflow` | fix mínimo da causa raiz |
| Regression test | `diagnosing-bugs` | teste que falhava antes do fix (ou justificativa manual documentada) |
| Verify → Review | `verification`, `code-review` | fluxo vizinho (rename, watcher) também verificado |

**Não aciona:** grilling, spec/plan, prototype, wayfinding.

---

## Tabela de decisão rápida

| Pedido contém… | Aciona primeiro |
|---|---|
| typo, label, padding, ajuste local | nada — fluxo trivial (§8) |
| "não funciona", "às vezes", "quebrou" | `diagnosing-bugs` |
| "adicione X" (feature média) | `grilling` (se ambíguo) → `spec-driven-flow` |
| "favorito/nota/etiqueta" (conceito novo?) | `domain-modeling` + `CONTEXT.md` |
| "sistema de…", "arquitetura de…", "reescrever…", multi-subsystem | `wayfinding` |
| "está demorando/travando/lento" | `diagnosing-bugs` → depois `performance` |
| duas abordagens plausíveis e caras | `prototype` (+ ADR se arquitetural) |
| trabalho que não termina hoje | `handoff` |
| nova dependência | `dependency-policy` |
| CSS/visual | `design-system` |

## Limitações conhecidas do Harness (DeepSeek Harness)

- Skills são **sumarizadas** (nome + descrição) para o modelo e **carregadas sob demanda** via ferramenta `skill`; não existe corpo "sempre carregado" além de `AGENTS.md` (instruções de workspace). Por isso `AGENTS.md` é índice, não repositório de detalhe.
- Uma skill não "chama" outra automaticamente: o encadeamento acontece porque o corpo de uma skill instrui o agente a carregar a seguinte (ex.: `wayfinding` → `grilling`). O agente precisa executar essas chamadas.
- Skills inválidas (frontmatter YAML malformado, `name` fora de kebab-case, chaves legadas tipo `disableModelInvocation`) são **ignoradas silenciosamente** na descoberta (apenas log) — validar frontmatter ao criar skill.
- Descoberta vigia `.dsh/skills/` e `.agents/skills/` do projeto (profundidade 1: `<nome>/SKILL.md` ou `<nome>.md`); arquivos de apoio dentro da pasta da skill são recursos, não skills.
- Verificação destes pontos: simulada com o provider real `@deepseek-ai/dsh-skill-filesystem` (ver relatório da tarefa); a ativação end-to-end pelo catálogo da sessão foi confirmada empiricamente.
