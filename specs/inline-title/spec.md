# Spec: Inline Title — título inline e sincronização com o painel lateral

> Fonte: adendo obrigatório do usuário + estado atual do código (revisado em 2026-08-28).

## Problema

Hoje o BRAINIAC não permite renomear um documento nem editar seu título sem recriá-lo:

- O painel lateral exibe `doc.title` (coluna `title` do banco), que é o nome dado na **criação** — não o nome real do arquivo em disco. O usuário não consegue distinguir o identificador físico (arquivo `.md`) do rótulo.
- Não existe comando de rename no backend (`commands/documents.rs` só tem create/get/get_all/save/delete).
- Nenhuma parte da UI lê o frontmatter `title:` do documento.
- O título e o arquivo podem dessincronizar da percepção do usuário: editar um título não tem efeito claro sobre "qual arquivo é este".

A feature **Inline Title** resolve isso: o título editável na área de edição passa a ser a fonte visível do nome, com regras claras de distinção entre *título lógico* (frontmatter) e *identificador físico* (arquivo), e sincronização instantânea com o explorador de arquivos.

## Comportamento esperado

### 1. Regra de exibição (distinção crucial)

- **Título inline** (área de edição): exibe o valor da propriedade `title:` do frontmatter, **se existir**; caso contrário, exibe o nome do arquivo **sem extensão**.
- **Painel lateral** (explorador): exibe **sempre** o nome real do arquivo físico (ex.: `meu-arquivo.md`). **Nunca** exibe o valor do frontmatter `title` no lugar do nome do arquivo — o nome físico é o identificador real do sistema.
  - ⚠️ Mudança sobre o comportamento atual: o sidebar hoje mostra `doc.title`; passa a mostrar o basename de `doc.path`.

### 2. Cenário A — Renomear arquivo

- Condição: **não havia** `title:` no frontmatter e o usuário editou o título inline e confirmou.
- Ação: o arquivo é **renomeado no disco** (ex.: `antigo.md` → `novo.md`).
- O painel lateral atualiza **imediatamente** (sem refresh da página) o nome daquele nó na árvore.
- Metadados do banco que refletem o caminho (`path`) são atualizados na mesma operação.

### 3. Cenário B — Editar frontmatter

- Condição: **já existia** `title:` no frontmatter e o usuário editou o título inline e confirmou.
- Ação: o frontmatter `title:` é atualizado; **o nome do arquivo não muda**.
- O painel lateral **não sofre alteração alguma** — apenas o título inline muda.

### 4. Reatividade e atualização instantânea

- A edição confirmada no título inline reflete no painel lateral **em tempo real**, sem refresh.
- O update é **cirúrgico**: apenas o nó afetado é atualizado (lista com chave por `id`, `{#each ... (doc.id)}`).
- A seleção/foco atual do usuário e a posição de scroll do painel são preservados.
- Quando diretórios pastarem a existir na árvore, a atualização **não deve colapsar** pastas abertas (hoje a lista é plana — requisito válido para a evolução em árvore).

### 5. Gerenciamento de estado

- Estado centralizado e reativo: o componente Inline Title dispara a ação (`file:renamed` ou `file:metadata-updated`) e o painel lateral reage automaticamente.
- **Adaptação de stack (divergência do adendo):** o adendo cita "Zustand, Pinia ou Context API" — exemplos de outros ecossistemas. No BRAINIAC o mecanismo equivalente e obrigatório (AGENTS.md) é o **store Svelte 5 runes** (`documentsStore`, classe com `$state` em `.store.svelte.ts`), consumido por ambos os componentes. A exigência funcional (atualização instantânea, update cirúrgico) é preservada; o mecanismo técnico fica para o `plan.md`.

### 6. Tratamento de conflitos

- Se o usuário tentar renomear (Cenário A) para um nome que **já existe na mesma pasta**, o sistema:
  1. **Impede** a ação (o arquivo não é renomeado);
  2. Exibe erro ao usuário: **"Já existe um arquivo com este nome"**;
  3. **Reverte** o título inline para o nome anterior;
  4. Mantém estado consistente entre editor e painel lateral (nenhum mutate parcial).

## Fora de escopo

- Árvore de diretórios / pastas (criar, mover entre pastas, drag & drop) — o rename é dentro da mesma pasta plana atual.
- Editar outros campos de frontmatter além de `title:`.
- Renomear em lote ou via painel lateral (o rename só ocorre via título inline).
- Editor do frontmatter estruturado (UI de key/value).
- Temas claros (design-system é dark only).

## Critérios de aceite

- [ ] Título inline mostra `title:` do frontmatter quando existe; senão, nome do arquivo sem extensão
- [ ] Painel lateral sempre mostra o nome físico do arquivo (basename de `path`), nunca o frontmatter title
- [ ] Cenário A: arquivo renomeado no disco + nó atualizado instantaneamente + seleção/foco preservados
- [ ] Cenário B: frontmatter atualizado, nome físico intocado, sidebar intocada
- [ ] Conflito de nome: ação bloqueada, erro "Já existe um arquivo com este nome" exibido, título inline revertido, estado consistente
- [ ] Toda lógica de negócio (resolução de título, detecção de conflito, rename) implementada via TDD (`tdd-workflow`)
- [ ] `bun run check` e `cargo check` passam (DoD)
