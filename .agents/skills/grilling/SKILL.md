---
name: grilling
description: Esclarecimento obrigatório de requisitos antes de implementar features com ambiguidade — identifica, classifica e resolve ambiguidades perguntando ao usuário em vez de assumir silenciosamente. Usar antes do research/spec em features novas, mudanças de comportamento, UX, domínio ou arquitetura. Não usar em mudanças triviais e totalmente determinadas.
disable-model-invocation: false
---

# Grilling — Esclarecer antes de implementar — BRAINIAC

> **Nota:** Esta skill referencia `AGENTS.md` (fonte da verdade), `CONTEXT.md` (vocabulário de domínio) e a skill `spec-driven-flow` (onde o resultado do grilling desemboca). Princípio: **o agente deve entender antes de implementar**.

## Problema que resolve

Requisito ambíguo + agente ansioso = feature implementada a partir de uma **interpretação arbitrária**. O grilling força o entendimento explícito antes do design.

## Quando usar

- Feature nova ou alteração de comportamento;
- mudança de UX com mais de uma solução plausível;
- alteração arquitetural ou de modelo de domínio;
- integração nova (backend, IA, filesystem);
- requisito com ambiguidades relevantes percebidas.

## Quando NÃO usar

- Mudanças triviais e totalmente determinadas (typo, label, ajuste de CSS localizado);
- bug com reprodução clara (usar `diagnosing-bugs`);
- quando o spec já responde às perguntas (verificar antes de perguntar).

## Fluxo

```text
Requisito
   ↓
Identificar ambiguidades
   ↓
Classificar (pesquisável × decisão do usuário × irrelevante)
   ↓
Perguntar ao usuário (apenas o que ele decide)
   ↓
Consolidar decisões
   ↓
Entendimento compartilhado
   ↓
Research / Spec (skill spec-driven-flow)
```

## 1. Identificar ambiguidades

Percorra o requisito e liste o que não está definido, nos eixos:

- **Objetivo** — qual problema do usuário isso resolve? Como se parece resolver?
- **Comportamento esperado** — o que acontece no fluxo principal?
- **Casos extremos** — documento vazio, sem rede, lista gigante, concorrência, caracteres especiais?
- **Estados** — loading, erro, vazio, desabilitado?
- **Permissões** — quem pode fazer o quê? (no BRAINIAC: app local de usuário único, mas há fronteiras filesystem/IA)
- **Persistência** — o que é salvo, onde, sobrevive a reinício? (Documento? Memory? AppConfig?)
- **Erros** — o que falha e como o usuário descobre? (ver eventos `app_error`)
- **UX** — onde entra na interface? Atalhos? Estados visuais?
- **Integração** — toca Commands, Stores, eventos, LLM, filesystem?
- **Compatibilidade** — afeta dados existentes (migrations, documentos já salvos)?
- **Critérios de sucesso** — como saberemos que está pronto e correto?

## 2. Classificar cada ambiguidade

| Classe | Exemplo | Ação |
|---|---|---|
| **Pesquisável** | "Qual store guarda o estado X?", "Já existe command para Y?" | Resolva com pesquisa no código/`CONTEXT.md`/specs — **não pergunte ao usuário** |
| **Decisão do usuário** | "Favorito é por documento ou global?", "Modo foco restaura estado anterior ou abre ambas?" | Pergunte; **nunca responda por ele** |
| **Irrelevante para o resultado** | detalhe interno sem impacto visível | Registre a premissa escolhida, explicitamente, no spec |

A classe **Pesquisável** é a mais comum: agente que pergunta ao usuário o que o código responde está desperdiçando o usuário. A classe **Decisão do usuário** é a que a skill existe para proteger: escolha de produto ou arquitetura que ninguém definiu ainda **não pode ser assumida em silêncio**.

## 3. Perguntar

- Agrupe as perguntas de decisão em uma rodada única (não interrogar em gotejamento).
- Para cada pergunta, ofereça as opções reais com o trade-off em uma frase — perguntas abertas sem opções empurram o trabalho de volta ao usuário.
- Limite: o que não muda o comportamento entregue não vira pergunta.
- Use a ferramenta de perguntas do Harness (`ask_user_question`) quando disponível; caso contrário, pergunte em texto direto e aguarde a resposta antes de prosseguir.

## 4. Consolidar

Produza um bloco de **decisões consolidadas**: pergunta → resposta → consequência em uma linha. Ele vai para o `spec.md` (seção "Decisões (grilling)" no template) — ou, em features pequenas sem spec, fica registrado na conversa antes da implementação.

Premissas assumidas (classe "irrelevante") devem aparecer marcadas como premissas, nunca misturadas às decisões do usuário.

## Anti-padrões

- **Auto-grilling:** responder as próprias perguntas de produto ("vou assumir que...") — proibido para decisão de usuário.
- **Interrogação:** perguntar o que uma busca de 30 segundos no código responde.
- **Grilling teatral:** perguntas cuja resposta não muda nada, só para "seguir processo".
- **Spec prematuro:** escrever `spec.md` com ambiguidades abertas dentro dele em vez de resolvê-las antes.
