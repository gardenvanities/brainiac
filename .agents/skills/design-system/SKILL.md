---
name: design-system
description: Fonte de verdade para QUALQUER estilo do projeto BRAINIAC — tokens de cor em OKLCH em três camadas (primitivo → semântico → componente), dark mode only, cascata via @layer e regras de uso de accent. Consultar antes de escrever CSS em qualquer componente.
disable-model-invocation: false
---

# Design System — BRAINIAC

## Princípios
- Apenas dark mode. Nunca criar lógica ou tokens de light mode.
- Fundo primário é preto absoluto (#000 / oklch(0% 0 0)).
- Cores em OKLCH em todo o projeto — nunca HEX ou HSL em novo código.
- Três camadas de token: primitivo → semântico → componente.
- Componentes SEMPRE usam tokens semânticos (--color-bg-surface), 
  NUNCA primitivos diretos (--gray-100) e NUNCA valores hardcoded.
- Cascata controlada via @layer, nesta ordem fixa: 
  reset, tokens, base, components, utilities.

## Paleta de marca
- Verde neon: sucesso e marca
- Azul: interativo — botões, foco, links, info
- Violeta neon: IA/agente — badges, ênfase, link visitado
- Vermelho: EXCLUSIVO para erro/perigo — nunca usado decorativamente
- Âmbar: exclusivo para warning/atenção

## Regras de uso de cor
- Texto de corpo nunca usa cores de accent — sempre a escala de cinza
- Accent colors aparecem em: bordas de foco, ícones ativos, botões primários,
  indicadores de estado, barra de progresso, badges
- Nunca usar accent em áreas grandes de fundo — só em elementos pontuais
- Hover/active states são gerados via color-mix(), nunca hardcoded

## Estrutura de arquivos
src/styles/tokens/primitives.css  — escalas cruas (cor, espaçamento, radius, movimento)
src/styles/tokens/semantic.css    — tokens com propósito, o que componentes usam
src/styles/tokens/typography.css  — tipografia (famílias, pesos, escala em rem)
src/styles/tokens/legacy.css      — transição: tokens ainda não migrados
src/styles/base.css                — reset + defaults
src/styles/layers.css              — ordem dos @layer

## Ao criar um componente novo
1. Nunca inicie um bloco <style> sem antes checar se o token semântico 
   necessário já existe em semantic.css
2. Se precisar de um token que não existe, adicione em semantic.css 
   primeiro (camada semântica), não invente valor local no componente
3. CSS nesting nativo é permitido e encorajado (sem preprocessador)
4. Use color-mix() para variações de hover/active de accent colors
5. Prefira propriedades lógicas (padding-inline, margin-block) sobre 
   físicas (padding-left, margin-top) mesmo em app LTR-only — é o 
   padrão moderno e futuro-compatível