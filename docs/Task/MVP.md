# 🧠 BRAINIAC — Próximos Passos (Plano de Desenvolvimento)

---

## ✅ Passo 3 — Banco de Dados

- [ ] Integrar **LibSQL** no backend Rust (Tauri)
- [ ] Criar `database/connection.rs` com pool e inicialização
- [ ] Popular `0001_initial.sql` com o schema aprovado (tabelas: notas, chats, memórias, configurações)
- [ ] Executar migrações automaticamente na inicialização do app

---

## ✅ Passo 4 — Models e Error Handling

- [ ] Implementar structs em `models/` (ex: `Note`, `ChatMessage`, `Memory`, `Config`)
- [ ] Criar `error.rs` com um tipo `AppError` global (usando `thiserror`)
- [ ] Registrar todos os módulos no `lib.rs`

---

## ✅ Passo 5 — Command Layer (base)

- [ ] Implementar comandos Tauri para **documentos** (CRUD básico)
- [ ] Implementar comandos para **configurações** (get/set)
- [ ] Testar comunicação Tauri ↔ Svelte com um `invoke()` real (ex: `get_notes()`)

---

## ✅ Passo 6 — Layout 3 Painéis (UI)

- [ ] Criar `AppShell.svelte` com **CSS Grid** (3 colunas)
- [ ] Implementar **painel esquerdo** (lista de notas/arquivos)
- [ ] Implementar **painel central** (editor Markdown)
- [ ] Implementar **painel direito** (chat com IA)
- [ ] **Sem lógica ainda** — apenas estrutura visual funcional

---

## ✅ Passo 7 — Editor Markdown (Milkdown)

- [ ] Integrar **Milkdown** no painel central
- [ ] Conectar com os comandos Tauri:
  - Criar novo arquivo `.md`
  - Abrir arquivo existente (carregar conteúdo)
  - Salvar arquivo (persistir no disco/banco)
- [ ] Tratar atalhos (Ctrl+S, etc.)

---

## ✅ Passo 8 — Chat com IA (LiteLLM + Claude)

- [ ] Integrar **LiteLLM** no backend Rust (ou via HTTP)
- [ ] Conectar à **API Claude** (Anthropic)
- [ ] Persistir **histórico de conversas** no LibSQL (por documento/contexto)
- [ ] Injetar **contexto do documento aberto** no prompt do chat (ex: `@nota.md`)

---

## ✅ Passo 9 — Memória Adaptativa

- [ ] Extrair automaticamente **resumos/memórias** após cada conversa (via IA)
- [ ] Armazenar memórias no LibSQL (tabela `memories`)
- [ ] Injetar memórias relevantes no **system prompt** do Claude (via RAG simples ou similaridade)

---

### 📌 Observações

- Os passos **3 a 5** são a base do backend (Rust/Tauri).
- Os passos **6 a 9** constroem a interface e as funcionalidades principais.
- Cada passo deve ser validado com testes manuais antes de avançar ao próximo.

---

**Início estimado:** assim que o ambiente estiver 100% configurado.
