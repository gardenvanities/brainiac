---
name: prototype
description: Prototipagem descartável no BRAINIAC — responder uma pergunta de design/UX/arquitetura antes de comprometer o código de produção, inclusive com variantes A/B/C de UI. Protótipo não é feature — por padrão é descartado ou convertido conscientemente. Usar em incerteza real; não usar quando o caminho já está determinado.
disable-model-invocation: false
---

# Prototype — BRAINIAC

> **Nota:** Esta skill referencia `AGENTS.md`, `spec-driven-flow` (o resultado alimenta o research/spec) e `codebase-design` (a conversão consciente para produção segue seus princípios). Princípio: **entender antes de implementar** — um protótipo responde perguntas, não entrega features.

## Objetivo

Explorar soluções **antes** de comprometer a arquitetura de produção. Um protótipo é, por padrão, **código descartável**: ele existe para produzir uma decisão, não para virar a feature.

## Quando usar

- UX/UI com múltiplas soluções plausíveis (layout, interação, fluxo);
- arquitetura incerta (duas abordagens viáveis, custo de reversão alto);
- API/tecnologia desconhecida (validar antes de desenhar em cima);
- experimentos e comparação de alternativas de implementação.

## Quando NÃO usar

- Mudança determinada (o caminho é único e conhecido — implemente direto);
- mudança trivial ou reversível de baixo custo (protótipo é burocracia aqui);
- "explorar" sem pergunta — protótipo sem pergunta é procrastinação com código.

## Fluxo

```text
Problema
   ↓
Pergunta que precisamos responder
   ↓
Protótipo (menor que responda a pergunta)
   ↓
Avaliação (critério explícito)
   ↓
Decisão
   ↓
Spec (spec-driven-flow)
   ↓
Implementação real (TDD)
```

## Regras

1. **Defina a pergunta antes do código.** "Sidebar fixa ou recolhível?" é uma pergunta; "melhorar a sidebar" não é.
2. **Menor que a feature.** O protótipo cobre só o necessário para responder — sem testes, sem polish, sem tratamentos de erro completos (isto é exceção consciente ao TDD e ao DoD, válida **apenas** enquanto protótipo).
3. **Descartável por padrão.** Não vai para a branch de trabalho como se fosse feature. Opções, em ordem de preferência:
   - branch dedicada (`prototype/<tema>`) ou commits isolados com prefixo `prototype:`;
   - nunca commitar código de protótipo misturado a código de produção.
4. **Avalie contra um critério explícito.** O que decide: usabilidade medida, tempo, complexidade do código resultante, impacto no IPC/banco? Registre a resposta à pergunta.
5. **Decida e registre.** A decisão (e o porquê) entra no `research.md`/`spec.md` da feature — ou num ADR se for decisão arquitetural difícil de reverter.
6. **Converta conscientemente ou descarte.** Implementação de produção recomeça do spec, com TDD — reutilizar trechos do protótipo é permitido, mas com testes e qualidade normal, nunca por arraste.

## Protótipos de UI — variantes

Para decisões de UI com várias soluções plausíveis, construa variantes pequenas e comparáveis:

```text
A — sidebar fixa
B — sidebar recolhível com atalho
C — sidebar contextual
```

- Cada variante responde à mesma pergunta com o mesmo cenário de uso.
- Avalie com o usuário quando a decisão for de gosto/produto (conecta com `grilling`) e com critério técnico quando for de desempenho/complexidade.
- Depois da decisão: **descarte as variantes** (e a escolhida vira implementação de produção via spec) — não mantenha código experimental vivo indefinidamente.

## Sinais de que o protótipo virou dívida

- Commits `prototype:` acumulando sem decisão registrada;
- código de protótipo importado por código de produção;
- "já que está pronto, vou entregar assim" — sem testes nem spec, isto é proibido.

Nesses casos: ou converte conscientemente (spec + TDD + reviews) ou remove.
