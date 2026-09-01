---
name: domain-modeling
description: Modelagem de domínio do BRAINIAC antes de criar entidades, estados, relações ou abstrações — parte do vocabulário de CONTEXT.md, mapeia o modelo para o stack (LibSQL, models Rust, types TS, stores) e impede conceitos duplicados. Usar só quando a mudança tem impacto real de domínio; não é cerimônia DDD para toda feature.
disable-model-invocation: false
---

# Domain Modeling — BRAINIAC

> **Nota:** Esta skill referencia `CONTEXT.md` (autoridade do vocabulário), `AGENTS.md` (regras globais) e as skills técnicas por camada: `libsql-schema`, `tauri-rust-patterns`, `svelte5-runes`. Princípio: **o agente deve entender o domínio antes de criar entidades, estados, relações ou abstrações**.

## Quando usar

- Nova entidade persistida ou conceito de domínio novo;
- mudança no modelo de dados (tabela, coluna, estado, categoria);
- nova relação entre conceitos existentes (ex.: Conversation ↔ Document);
- regra de negócio nova com invariante;
- evento relevante do domínio (algo que acontece e importa registrar ou reagir).

## Quando NÃO usar

- Ajuste de UI/UX sem impacto no modelo (label, espaçamento, atalho);
- refactoring estrutural sem mudança de conceitos (usar `codebase-design`);
- bug fix que não altera o modelo (usar `diagnosing-bugs`).

Não transforme toda feature em cerimônia de DDD: se a mudança não introduz nem altera conceitos, esta skill não se aplica.

## Fluxo

```text
Mudança proposta
   ↓
Ler CONTEXT.md — o conceito já existe?
   ↓ (novo)                          ↓ (existente, mas muda)
Identificar modelo                Avaliar impacto no modelo atual
   ↓                                  ↓
Validar com usuário (grilling)    ────────────┐
   ↓                                          ↓
Mapear para o stack        Documentar no spec (seção Modelo de domínio)
   ↓                                          ↓
Spec + (ADR se arquitetural) ←──────────────┘
```

## 1. Partir do vocabulário, não do contrário

- `CONTEXT.md` é a autoridade: se o conceito já tem nome e definição, **use-os**.
- Nunca crie sinônimo (`Note`, `Page`, `DocumentEntity`, `MarkdownFile`) para conceito estabelecido.
- Conceito genuinamente novo → defina-o no spec **e adicione ao `CONTEXT.md` na mesma tarefa**.

## 2. Identificar os elementos de modelo

Para a mudança em questão, respondê-los quando relevantes (não é checklist obrigatório):

- **Entidades/objetos** — quais conceitos participam?
- **Estados** — cada entidade tem ciclo de vida? (ex.: Memory `is_confirmed`/`is_active`; Document soft delete)
- **Relações** — 1:1, 1:N, N:N? Opcional ou obrigatória? (ex.: Conversation → Agent obrigatória; → Document opcional)
- **Invariantes** — o que precisa ser sempre verdadeiro? (ex.: único Agent `is_default`; `path` único de Document)
- **Responsabilidades** — o que cada conceito faz e o que **não** faz?
- **Limites de contexto** — a mudança fica em qual frente (Documentos/Editor, Chat/IA, Memória, Config)? Atravessa fronteiras?
- **Eventos de domínio** — algo acontece e outras partes precisam saber? (ex.: documento alterado externamente → watcher → UI)
- **Regras de negócio** — condições, validações, padrões.

## 3. Mapear para o stack do BRAINIAC

| Elemento de modelo | Destino no stack | Regra |
|---|---|---|
| Entidade persistida | tabela LibSQL + model Rust + type TS | PK UUID v4 TEXT; espelho snake_case ↔ camelCase; soft delete; ver `libsql-schema` |
| Invariante | constraint SQL e/ou validação no Command | nunca validar só no frontend |
| Ciclo de vida (estado) | colunas de flag/enum + transições nos Commands | transição inválida é erro via `AppError` |
| Relação | FK + queries na camada `queries/` | SQL só em `queries/`, parâmetros posicionais |
| Evento de domínio | evento Tauri (push) ou nova conversa/prompt | push backend→frontend usa eventos, não polling |
| Estado de UI | store (global) ou `$state` (local) | só global se compartilhado; ver `svelte5-runes` |

Regra de fronteira frontend/backend: a lógica mora onde está sua **responsabilidade real** — persistência, segurança, integridade e acesso ao sistema ficam no backend; orquestração de UI no frontend. Nem "Rust porque é mais robusto", nem "TS porque é mais rápido de escrever" (ver `codebase-design`).

## 4. Registrar

- **Spec** — seção "Modelo de domínio": entidades, estados, relações e invariantes da feature.
- **`CONTEXT.md`** — conceitos novos ou alterados (mesma tarefa).
- **ADR** — apenas se a modelagem for decisão arquitetural significativa (difícil de reverter, múltiplas alternativas, afeta várias frentes) — ver `.agents/decisions/README.md`.
- **Migration** — se há schema novo, append-only (ver `libsql-schema`).

## Anti-padrões

- **Entidade-fantasma:** criar tabela/struct para algo que já existe com outro nome.
- **Estado órfão:** flag sem transições definidas (nunca é lida ou nunca é alterada).
- **Invariantes no frontend:** validação só na UI, sem contraparte no backend.
- **Sinônimo casual:** nomear conceito novo sem verificar `CONTEXT.md`.
- **DDD decorativo:** value objects, repositorios e camadas de domínio sem necessidade real no tamanho atual do projeto.
