# AGENTS.md — Fonte da Verdade do BRAINIAC

> **Nota:** Este arquivo é a **fonte da verdade** do projeto BRAINIAC. Toda Skill ou instrução de agente deve referenciar o `AGENTS.md` ao invés de duplicar estas regras.

---

## Contexto hierárquico

Este `AGENTS.md` é a fonte **global** de verdade. Regras aplicáveis a um escopo específico de diretório podem viver em `AGENTS.md` subordinados (ex.: `src/AGENTS.md`, `src-tauri/AGENTS.md`, `src-tauri/database/AGENTS.md`), mas **apenas** quando houver necessidade real — nunca para copiar regras globais.

Regras de aplicação:

1. Um `AGENTS.md` subordinado contém **somente** regras específicas daquele escopo.
2. Regras globais **não** são copiadas para arquivos subordinados — são herdadas automaticamente.
3. Ao modificar um arquivo, respeite a hierarquia de contexto **aplicável ao caminho** do arquivo: regras globais (aqui) + regras do(s) diretório(s) ancestrais + regras do diretório do próprio arquivo.
4. Quando uma regra global precisar ser citada por uma Skill, **referencie** este arquivo em vez de copiá-la (princípio: **uma regra → um local de autoridade**).

---

## 1. Visão do projeto

O BRAINIAC é uma aplicação desktop local ("segundo cérebro") focada em produtividade, gestão do conhecimento e assistência contextual por IA. Ele combina um editor de documentos Markdown em tempo real com um chat integrado capaz de compreender o contexto das notas e das memórias do usuário. Construído com prioridade para privacidade e execução local, toda a persistência de dados ocorre em um banco LibSQL embarcado.

---

## 2. Stack obrigatória

- **Desktop Backend:** Tauri 2 (Rust)
- **Frontend:** Svelte 5 + TypeScript (Runes: `$state`, `$derived`, `$effect`, `untrack`)
- **Banco de Dados:** LibSQL (SQLite-compatible, UUIDs v4 como PKs TEXT, sem IDs sequenciais)
- **Linter & Formatter:** Biome 2.x
- **Package Manager / Runtime:** Bun

---

## 3. Padrões Rust

- **Tratamento de Erros:** Erros do backend são sempre tratados via `AppError` centralizado em `src-tauri/src/error.rs` (usando `thiserror`). Usar sempre `?` para propagação de erros, nunca `.unwrap()` em código de produção.
- **Separação de Responsabilidades:** Handlers de invoke ficam em `src-tauri/src/commands/`, enquanto a lógica SQL e interações de banco ficam estritamente em `src-tauri/src/database/queries/`. Commands chamam queries, queries acessam o banco.
- **Identificadores (PKs):** Chaves primárias (PKs) são sempre UUID v4 gravados como `TEXT`.
- **Execução de PRAGMAs:** PRAGMAs do LibSQL/SQLite que retornam linhas devem utilizar `execute_batch()` ao invés de `execute()`.
- **Serde & Naming:** Structs recebidas ou enviadas ao frontend usam `#[serde(rename_all = "camelCase")]`.
- **Concorrência:** Locks de `Mutex` devem ser obtidos, utilizados e liberados naturalmente através do ciclo de vida da variável (`drop`).
- **Organização de Imports:** Ordenar imports em blocos organizados: `std`, crates externos, crates internos do projeto.

---

## 4. Padrões Svelte 5 & TypeScript

- **Reatividade com Runes:** Toda a reatividade no frontend utiliza Runes (`$state`, `$derived`, `$effect`). Nunca utilizar sintaxe legada do Svelte 4 como declarações reativas (`$:`).
- **Leitura sem Dependência Reativa:** Utilizar `untrack()` para ler valores de estado sem registrar dependências reativas dentro de um `$effect`.
- **Gerenciamento de Estado Global:** Stores são implementadas como classes com `$state` interno em arquivos nomeados com a extensão `.store.svelte.ts` (em `src/stores/`).
- **Imports de Componentes:** Imports de componentes Svelte não devem incluir extensão `.ts` explícita.
- **Props e CSS:** Props de componentes utilizam `$props()` com tipagem TypeScript explícita. O CSS deve ser sempre escopado dentro do próprio componente (evitar classes globais exceto em `:global()` estritamente necessário).
- **Efeitos e Limpeza:** Eventos e subscrições que exigem limpeza devem retornar a função de limpeza no próprio `$effect`.
- **Sem Console Logs:** Proibido manter `console.log` em código commitado.
- **Camada de Comunicação Tauri ↔ Svelte:**
  - O frontend consome os comandos Rust **exclusivamente** via wrappers tipados em `src/lib/tauri/`. Nunca chamar `invoke()` diretamente dentro dos componentes.
  - Tipos TypeScript em `src/types/` devem espelhar exatamente as structs dos models Rust (`src-tauri/src/models/`), convertendo campos de `snake_case` (Rust) para `camelCase` (TypeScript).
  - Eventos de streaming utilizam canais dedicados: `message_chunk`, `message_done`, `app_error`.

---

## 5. Regra inegociável: TDD

Nenhuma função de lógica de negócio é escrita sem um teste que falhe primeiro (**Test-Driven Development**). Todo fluxo de dados, lógica de cálculo, transformação de estado ou regra de validação no backend (Rust) ou no frontend (Svelte/TS) exige primeiramente a criação do teste correspondente e a confirmação de sua falha antes da escrita da implementação.

---

## 6. Regra inegociável: Especificações e Planos de Implementação

Nenhuma feature ou alteração arquitetural é implementada sem que documentos `spec.md` e `plan.md` estejam escritos e aprovados antes de qualquer edição no código-fonte. A especificação deve detalhar o comportamento esperado e casos de borda, enquanto o plano de execução deve quebrar as etapas técnicas necessárias. O fluxo completo (com pesquisa prévia) é definido pelas Skills `spec-driven-flow` e `tdd-workflow`. A profundidade é proporcional à complexidade (ver §8): mudanças triviais e pequenas seguem o fluxo reduzido definido na skill `spec-driven-flow`.

---

## 6.1. Regra inegociável: Governança de Regras

- Mudanças em AGENTS.md ou em qualquer skills/*.md seguem obrigatoriamente 
  o processo descrito em skills/rule-governance.md — nunca editadas 
  como efeito colateral de outra tarefa.

---

## 7. Regra: Reuse Before Create

> Antes de criar qualquer componente, store, utility, command, query, model, abstraction ou mecanismo novo, procure primeiro uma implementação existente que possa ser reutilizada ou estendida.

Fluxo obrigatório:

```text
Preciso de X
    ↓
X já existe?
    ↓
   SIM ──→ reutilizar/estender
    │
   NÃO
    ↓
avaliar necessidade de criar
    ↓
criar
```

Objetivo: combater duplicação, componentes semelhantes, stores redundantes, utilities paralelas, abstrações concorrentes e lógica repetida. Não é burocracia — é **reutilização inteligente**. Novas abstrações são bem-vindas quando justificadas.

---

## 8. Regra: Complexity-aware workflow

Toda mudança começa com uma **classificação honesta de complexidade** — o processo cresce com o risco, nunca com o entusiasmo. Use o mínimo de processo necessário para obter alta confiança.

| Nível | Exemplos | Fluxo |
|---|---|---|
| **Trivial** | typo, label, ajuste de CSS localizado, correção determinada | Research mínima → implementar → verificar. Sem spec, sem grilling |
| **Pequena** | componente isolado, comportamento pontual, bug localizado | Research curta → implementação + testes (TDD) → verification → code review. Bug segue a skill `diagnosing-bugs` |
| **Média** | nova feature, alteração de fluxo ou de estado | Grilling (se ambíguo) → Research → Spec (aprovação) → Plan (aprovação) → Tasks → TDD → Implementation → Verification → reviews aplicáveis → code review (skill `spec-driven-flow`) |
| **Grande** | novo subsistema, alteração de domínio/arquitetural, integração importante | Skill `wayfinding`: Grilling → Domain Modeling → Research → Prototype (quando necessário) → ADR(s) → Spec → Plan → Tasks → TDD → Implementation → Verification → Review, com Handoff entre sessões |

Anti-overengineering: não exigir grilling para typo, ADR para CSS, domain modeling para label, prototype para mudança trivial, wayfinding para bug simples ou handoff para tarefa de sessão única. Em dúvida entre dois níveis, escolha o **menor** e escale quando a pesquisa revelar mais alcance do que o previsto — nunca por antecipação.

---

## Definition of Done

Nenhuma tarefa, feature ou correção é considerada concluída até que 
TODOS os itens abaixo sejam verdadeiros:

1. O teste que valida o comportamento está escrito e passando
2. `cd src-tauri && cargo check` roda sem erros (warnings de código 
   ainda não usado são aceitáveis apenas se o código será usado em 
   passo imediatamente seguinte)
3. `bun run check` (Biome + svelte-check) roda sem erros
4. O agente mostrou o output de sucesso de ambos os comandos acima 
   na resposta, não apenas afirmou que "deve funcionar"
5. Commit foi feito com mensagem descritiva no padrão 
   `tipo: descrição breve` (feat, fix, chore, refactor)

Além dos itens acima, **nenhum** dos seguintes deve estar pendente:

- Nenhuma violação das regras deste `AGENTS.md`
- Nenhuma violação das Skills aplicáveis à mudança
- Nenhuma dependência nova sem justificativa (ver Skill `dependency-policy`)
- Nenhum `TODO`/`FIXME` temporário introduzido
- Nenhum `console.log`, debug print ou código de diagnóstico deixado acidentalmente
- Testes relevantes executados e passando
- Bug corrigido possui regression test, ou justificativa documentada da verificação manual (Skill `diagnosing-bugs`)
- Code review executado (Skill `code-review`)
- Security review executado quando aplicável (Skill `security`)
- Performance review executada quando aplicável (Skill `performance`)
- Verification executada no nível apropriado (Skill `verification`)
- UI verificada quando houver alteração visual
- Migration verificada quando houver alteração de schema
- Contrato Tauri ↔ frontend verificado quando houver alteração de IPC

Se qualquer item falhar, a tarefa NÃO está pronta — o agente deve 
corrigir antes de reportar conclusão ou pedir input do usuário sobre 
como proceder.

### Quality Gate (auto-verificação antes de reportar conclusão)

Antes de declarar qualquer alteração concluída, responda honestamente — nas categorias **aplicáveis à mudança** (as demais são puladas sem cerimônia):

- **Entendimento:** entendi o requisito? Restou ambiguidade não resolvida? (skill `grilling`)
- **Arquitetura:** reutilizei o que já existia? Criei abstração desnecessária? Existe uma única fonte de verdade? (skills `codebase-design`, `dependency-policy`)
- **Código:** segue as convenções do projeto? Responsabilidades claras? Sem duplicação? (skills de camada técnica)
- **Testes:** existe teste adequado passando? Regression test quando foi bug? (skills `tdd-workflow`, `diagnosing-bugs`)
- **Segurança:** a alteração cria novos riscos? (skill `security`, quando aplicável)
- **Performance:** adiciona trabalho desnecessário? (skill `performance`, quando aplicável)
- **UX:** a interface ficou mais simples ou mais complexa?
- **Verificação:** eu realmente verifiquei o comportamento, nos estados relevantes? (skill `verification`)

O Quality Gate **não substitui** o Definition of Done — é a checagem consciente que o precede. Como todo o processo, é proporcional à complexidade (§8).

---

## 9. Acessibilidade

Regras proporcionais à UI afetada (não um checklist burocrático para elementos irrelevantes):

- Navegação por teclado e gerenciamento de foco para toda UI interativa
- HTML semântico (`<button>`, `<input>`, `<nav>`, etc.) antes de `div`/`span` clicáveis
- ARIA **apenas quando necessário** (role/label/descrição em widgets customizados)
- Dialogs e menus com foco aprisionado, tecla `Escape` e retorno de foco
- Command palette com navegação por teclado completa
- Editor e atalhos documentados e acessíveis via teclado
- Estados de loading/error/empty anunciados de forma compreensível
- Contraste e legibilidade seguindo os tokens do design-system

A regra é ativada pela Skill `code-review` (item de revisão) e pelo bom senso em features de UI.

---

## 10. Hierarquia das fontes de verdade

```text
AGENTS.md          (regras globais, processo, DoD)
    ↓
Skills             (detalhe de processo e de camada — .agents/skills/)
    ↓
CONTEXT.md         (vocabulário e significado do domínio)
    ↓
ADRs               (decisões arquiteturais — .agents/decisions/)
    ↓
Specs              (verdade por feature — specs/)
    ↓
Tasks              (passos atômicos — specs/<feature>/tasks.md)
    ↓
Implementação      (código + testes)
```

Validação contra a arquitetura atual: não existe camada "Rules" separada neste projeto — as regras vivem no `AGENTS.md` e nas Skills, que ocupam essa posição na hierarquia.

- **Conflito de regras/processo** → prevalece o nível mais alto da hierarquia.
- **Conflito de vocabulário de domínio** → `CONTEXT.md` é a autoridade.
- **Conflito de decisão arquitetural** → o ADR registrado é a autoridade; suplantá-lo exige novo ADR (append-only).
- **Código divergente da documentação** → é bug de documentação: corrija explicitamente a fonte obsoleta, nunca silenciosamente.

---

## Anexo A: Regras de Banco de Dados (SQL)

- Sempre utilizar `IF NOT EXISTS` em comandos `CREATE TABLE` e `CREATE INDEX`.
- Soft delete é obrigatório via coluna `is_deleted = 1`; remoção física (`DELETE FROM`) é proibida em documentos.
- Timestamps são armazenados em formato ISO 8601 (RFC 3339).
- Parâmetros em queries SQL são obrigatoriamente posicionais (`?1`, `?2`, ...) nunca interpolação de strings.

---

## Anexo B: Estrutura de Pastas de Referência

```
src-tauri/src/
├── commands/     ← Handlers de comandos Tauri (invokes)
├── database/
│   ├── queries/  ← Operações SQL e persistência
│   └── migrations/
├── llm/          ← Cliente HTTP e comunicação com provedores de IA
├── models/       ← Structs Serde e entidades de domínio
└── error.rs      ← Tipo de erro global AppError

src/
├── components/   ← Componentes de interface (Svelte 5)
├── lib/tauri/    ← Wrappers tipados para chamadas invoke()
├── stores/       ← Estado global da aplicação (.store.svelte.ts)
└── types/        ← Definições de tipos TypeScript
```

---

## Anexo C: Sistema de contexto (Skills, vocabulário, specs e decisões)

- **Skills** — conhecimento especializado em `.agents/skills/<nome>/SKILL.md` (descobertas automaticamente pelo Harness). Consulte a skill aplicável antes de atuar em áreas que ela cobre. Skills disponíveis, por papel:

  - **Processo e fluxo:** `spec-driven-flow` (pipeline de features), `tdd-workflow` (toda lógica de negócio), `grilling` (requisito ambíguo, antes de implementar), `diagnosing-bugs` (bugs e comportamento inesperado), `verification` (após implementar), `code-review` (antes de reportar conclusão), `wayfinding` (iniciativas grandes — ver §8), `handoff` (trabalho atravessando sessões), `prototype` (incerteza de design/UX/arquitetura).
  - **Domínio e desenho:** `domain-modeling` (impacto no modelo de domínio), `codebase-design` (desenho estrutural de mudanças não-triviais), `project-architecture` (visão geral).
  - **Camadas técnicas:** `svelte5-runes` (frontend), `tauri-rust-patterns` (backend), `libsql-schema` (banco), `design-system` (qualquer CSS).
  - **Reviews e políticas:** `security`, `performance`, `dependency-policy`.

  Qual skill acionar por nível de complexidade: ver o fluxo de cada nível no §8. Exemplos práticos por cenário: ver `docs/agentic-workflow.md`.

- **Vocabulário de domínio (`CONTEXT.md`)** — autoridade para o **significado** dos conceitos (Document, Memory, Agent, Conversation, Context etc.). Antes de nomear entidade, estado ou abstração, consulte-o; conceitos novos entram nele na mesma tarefa em que aparecem no código (skills `domain-modeling`, `grilling`).

- **Specs** — documentos por feature em `specs/<feature>/` (`research.md`, `spec.md`, `plan.md`, `tasks.md`; templates em `specs/_template/`), regidos pela skill `spec-driven-flow`.

- **Handoff entre sessões** — `specs/<iniciativa>/handoff.md` (template em `specs/_template/handoff.md`), regido pela skill `handoff`.

- **Decisões arquiteturais (ADRs)** — em `.agents/decisions/`. Uma decisão difícil ou cara de reverter merece um ADR (ver template `.agents/decisions/_template.md`). Não criar ADR para decisões triviais ou locais.

- **Hierarquia de conflito entre fontes** — definida no §10.

A arquitetura conceitual do fluxo de desenvolvimento (Skills e sua ordem de ativação) está documentada na Skill `spec-driven-flow`.
