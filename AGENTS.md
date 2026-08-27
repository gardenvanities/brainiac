# AGENTS.md — Fonte da Verdade do BRAINIAC

> **Nota:** Este arquivo é a **fonte da verdade** do projeto BRAINIAC. Toda Skill ou instrução de agente deve referenciar o `AGENTS.md` ao invés de duplicar estas regras.

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

Nenhuma feature ou alteração arquitetural é implementada sem que documentos `spec.md` e `plan.md` estejam escritos e aprovados antes de qualquer edição no código-fonte. A especificação deve detalhar o comportamento esperado e casos de borda, enquanto o plano de execução deve quebrar as etapas técnicas necessárias.

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

Se qualquer item falhar, a tarefa NÃO está pronta — o agente deve 
corrigir antes de reportar conclusão ou pedir input do usuário sobre 
como proceder.

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
