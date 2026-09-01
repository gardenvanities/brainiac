---
name: codebase-design
description: Qualidade estrutural no DESENHO do código do BRAINIAC — módulos coesos com interfaces pequenas (deep modules, shallow interfaces), reuso antes de abstrair, regras específicas de Svelte 5 e Tauri/Rust, e anti-padrões (wrappers sem valor, services/managers redundantes, múltiplas fontes de verdade). Usar ao planejar mudanças; o code-review verifica o mesmo depois.
disable-model-invocation: false
---

# Codebase Design — BRAINIAC

> **Nota:** Esta skill é o olhar **de desenho** (antes/durante); o `code-review` é o olhar **de revisão** (depois). Referencia `AGENTS.md` (Regra Reuse Before Create), `CONTEXT.md` (vocabulário) e as skills `svelte5-runes`, `tauri-rust-patterns`, `libsql-schema`, `design-system`, `dependency-policy`. Princípios: **reutilizar antes de abstrair** e **provar que funciona antes de considerar concluído**.

## Princípios

1. **Deep modules, shallow interfaces** — módulo coeso por trás de uma interface pequena. Regra de bom senso, não dogma: uma interface simples já exposta e usada vale mais que uma "profunda" reescrita.
2. **Reuse Before Create** (regra global em `AGENTS.md`) — antes de criar componente, store, service, utility, command, query, hook, helper ou adapter, procure o que existe. Fluxo:
   ```text
   Preciso de X
      ↓
   X já existe? ── SIM → reutilizar/estender
      ↓ NÃO
   Realmente precisa existir? ── NÃO → usar o que há, mesmo que imperfeito
      ↓ SIM
   Criar com UMA responsabilidade clara
   ```
3. **Uma fonte de verdade** — cada fato do domínio tem um lugar autoritativo (ver `CONTEXT.md`); derivados são calculados (`$derived`, query), nunca copiados.
4. **Alta localidade** — código relacionado perto; mudança de feature toca o mínimo de arquivos possível.
5. **Composição sobre camadas** — funções/componentes compostos; nova camada (service/manager/repository) só com justificativa concreta.
6. **Seams para teste** — lógica pura testável isolada de efeitos (ex.: helper puro `isShortcut` em `lib/utils/` testado com Vitest, sem UI).

## Anti-padrões que esta skill existe para impedir

- **Abstração prematura** — generalizar no segundo caso de uso; abstrair só no terceiro, quando o padrão é real.
- **Wrapper sem valor** — camada que só repassa (service chamando outro service; wrapper de wrapper de invoke).
- **Services/managers/controllers redundantes** — o BRAINIAC já tem camadas: Commands (orquestração) → Queries (SQL) → Stores (estado). Não inventar camadas paralelas.
- **Indirection excessiva** — saltar 4 arquivos para ler um fluxo simples.
- **Duplicação de lógica** — mesma regra em Rust e TS, em dois componentes, em duas queries.
- **Múltiplas fontes de verdade** — ex.: título do Document copiado em store, banco e arquivo sem dono único.

## Específico de Svelte 5 (complementa `svelte5-runes`)

- **Derivação declarativa > `$effect`**: se um valor é calculado de outro, use `$derived`. `$effect` é para sincronizar com o mundo externo (DOM, IPC, timers) — não para computar estado.
- **State ownership**: estado pertence a quem o usa. Local no componente; global em Store `.store.svelte.ts` **só** se compartilhado entre telas/painéis. Store global "por conveniência" é anti-padrão.
- **Boundaries de componente**: componente pequeno com props tipadas via `$props()`; não passar objetos gigantes "para facilitar".
- **Eventos e cleanup**: listeners/subscrições em `$effect` retornam a função de limpeza.
- **`untrack()`** para leituras que não devem criar dependência (caso `Editor.svelte`).
- **Acessibilidade embutida no design**: HTML semântico e teclado desde o desenho, não como patch (regra global de a11y em `AGENTS.md`).

## Específico de Tauri/Rust (complementa `tauri-rust-patterns`)

- **Fronteiras**: Commands orquestram, Queries fazem SQL, Models são o contrato IPC. Lógica de sistema (filesystem, segredos, processo) fica no backend; o frontend orquestra UI.
- **Mover para Rust?** Só quando a responsabilidade real pede: integridade de dados, segurança, operações de sistema, trabalho pesado. Não por "robustez" estética; nem manter no frontend algo que expõe sistema/segredos.
- **Payloads mínimos**: envie/retorne o necessário (contrato espelhado Rust ↔ TS); payloads gigantes são custo de IPC.
- **Erros**: `AppError` + `?`; `.unwrap()` só em testes.
- **Permissões**: nova superfície de acesso (fs, shell, http) exige checar capabilities e a skill `security`.

## Banco (complementa `libsql-schema`)

- Queries coesas por agregado (arquivo por entidade, como já organizado); sem query genérica "que faz tudo".
- Derivados do banco (contagens, somas) calculados em SQL, não em loop no frontend.

## Ao criar algo novo

Antes de escrever o primeiro arquivo novo, responder no plan (uma linha cada):

1. O que existente foi considerado e por que não serve?
2. Qual é a **única** responsabilidade do novo módulo?
3. Como será testado (seam)?
4. O que isto torna mais difícil no futuro? (se a resposta pesar, repensar)
