---
name: handoff
description: Handoff leve entre sessões de agente no BRAINIAC — registra em um único arquivo (specs/<iniciativa>/handoff.md) o objetivo, estado, decisões, fontes de verdade e a próxima ação concreta, sem duplicar specs/ADRs. Usar quando o trabalho atravessa sessões (wayfinding, feature grande, debugging longo); desnecessário para tarefa concluída em uma sessão.
disable-model-invocation: false
---

# Handoff — Continuidade entre sessões — BRAINIAC

> **Nota:** Esta skill referencia `AGENTS.md` e a skill `wayfinding` (que a convoca por padrão). Objetivo: outro agente (ou você mesmo, numa próxima sessão) retomar o trabalho **sem reler a conversa inteira** e **sem descobrir de novo o que já foi decidido**.

## Quando usar

- Trabalho que sabe (ou descobriu) que atravessará sessões:
  - iniciativa grande em `wayfinding`;
  - feature grande com spec/plan/tasks em andamento;
  - debugging longo com hipóteses abertas (`diagnosing-bugs`);
  - refatoração multi-etapa.
- Fim de sessão com trabalho não concluído e não commitado de forma autoexplicativa.

## Quando NÃO usar

- Tarefa concluída em uma sessão (o commit + spec já são o registro);
- "por garantia" — handoff desatualizado é pior que ausente: registra um estado que não existe mais.

## Localização e formato

Um arquivo por iniciativa: `specs/<iniciativa>/handoff.md` (template em `specs/_template/handoff.md`).

```text
# Handoff: <iniciativa>
Atualizado: <data> · Sessão: <o que esta sessão fez>

## Current objective
[1–2 frases: o objetivo de término desta iniciativa]

## Current state
[Em uma frase: onde a iniciativa está (ex.: "tasks 1–4 prontas, task 5 falhando")]

## Completed
- [o que está pronto e verificado, com commit/ref]

## Decisions
- [decisão → onde está registrada (ADR-NNN / spec § / conversa aprovada)]

## Relevant files
- [arquivos centrais agora, com uma frase de papel]

## Relevant specs / ADRs
- [caminhos — NÃO copie o conteúdo]

## Known problems
- [o que está quebrado/aberto (ex.: teste vermelho, hipótese H2 não eliminada)]

## Next action
[UMA ação concreta e autocontida para a próxima sessão]
```

## Regras

1. **Aponte, não duplique.** Specs, ADRs, tasks e commits são as fontes de verdade; o handoff só referencia. Se um conteúdo precisa existir, ele pertence à sua fonte (spec, ADR, CONTEXT.md) — não ao handoff.
2. **Curto e atual.** Tela única. Atualize no fim de cada sessão de trabalho — handoff vale pelo estado mais recente, não pelo histórico (histórico mora no Git).
3. **Next action única e executável.** "Implementar task 5 (persistir favoritos) — teste já esboçado em X" e não "continuar a feature".
4. **Estado real, não desejado.** Teste vermelho é registrado como vermelho; decisão não tomada fica aberta.
5. **Início de sessão:** leia o handoff antes de agir; valide-o contra o código (o Git manda se divergirem) e corrija-o se estiver obsoleto.
6. **Fim da iniciativa:** o handoff se aposenta — o spec finalizado + commits + ADRs passam a ser o registro; apague ou marque `Concluído` no topo.

## Rastreabilidade (mudanças grandes)

O encadeamento `Requirement → Spec → Tasks → Implementation → Tests → Commit/PR` deve poder ser percorrido: o handoff aponta o elo atual dessa cadeia, o Git preserva os anteriores.
