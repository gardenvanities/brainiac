# CONTEXT.md — Vocabulário conceitual do BRAINIAC

> **O que é:** a fonte de verdade do vocabulário de domínio. Define **o que cada conceito significa** dentro do BRAINIAC — não como é implementado.
>
> **O que NÃO é:** documentação técnica, referência de APIs, lista de arquivos ou guia operacional. Detalhes de implementação vivem no código, nas Skills e no `AGENTS.md`.

## Autoridade e manutenção

1. Ao nomear ou discutir um conceito, use **este vocabulário**. Não crie sinônimos para conceitos já definidos (ver regra de consistência ao final).
2. Antes de criar entidades, estados ou abstrações novas, consulte este documento (Skill `domain-modeling`).
3. Se um conceito novo aparecer de verdade no domínio, **adicione-o aqui na mesma tarefa** que o introduz no código — vocabulário e código evoluem juntos.
4. Hierarquia de conflito: `AGENTS.md` > Skills > `CONTEXT.md` > ADRs > Specs > Código. Para **significado de domínio**, porém, este documento é a autoridade; conflitos devem ser resolvidos explicitamente, nunca silenciosamente (corrija a documentação obsoleta).

## Domínio do produto

### Document
- **Definição:** uma nota em Markdown do usuário, armazenada como arquivo `.md` no disco e indexada no banco.
- **Responsabilidade:** unidade principal de conhecimento; o conteúdo que o usuário edita e que a IA pode usar como contexto.
- **Relações:** vive na Pasta de Documentos; tem Frontmatter opcional; abre no Editor; pode ancorar uma Conversation; gera word_count.
- **Estados:** ativo ↔ deletado (soft delete — nunca remoção física); com/sem Frontmatter.
- **Distinções:** `Document` não é "Note", "Page" ou "MarkdownFile". O registro no banco é o índice; o arquivo em disco é a fonte do conteúdo. `DocumentWithContent` é o Document + conteúdo carregado, não um conceito novo.

### Frontmatter
- **Definição:** bloco de metadados no topo de um Document.
- **Responsabilidade:** guardar metadados do documento (ex.: título usado na sincronização entre Editor e painel lateral).
- **Relações:** pertence a um Document; serializado como JSON no índice.
- **Distinções:** metadado estruturado do arquivo ≠ conteúdo do documento.

### Pasta de Documentos (`documents_path`)
- **Definição:** diretório raiz, configurado no AppConfig, onde os Documents vivem em disco.
- **Responsabilidade:** fronteira do filesystem observada pelo watcher e pelo acesso a arquivos.
- **Relações:** contém Documents; monitorada por eventos de mudança externa.
- **Distinções:** é um diretório real do usuário, não um conceito interno — acesso fora dela é violação de segurança.

### Editor
- **Definição:** componente de edição de Markdown em tempo real (base Milkdown/Crepe).
- **Responsabilidade:** exibir e editar o conteúdo de um Document.
- **Relações:** edita um Document; sincroniza título (InlineTitle) e metadados com o backend.
- **Distinções:** Editor é a superfície de UI; Document é o conceito persistido.

### Agent
- **Definição:** persona de assistência configurável pelo usuário.
- **Responsabilidade:** definir comportamento da IA: prompt de sistema, modelo padrão, identidade visual.
- **Relações:** conduz Conversations; referencia um modelo padrão (`model_default`).
- **Estados:** ativo/inativo; padrão (`is_default` — único).
- **Distinções:** Agent ≠ Conversation: o Agent é a configuração persistente; a Conversation é uma sessão de uso.

### Conversation
- **Definição:** sessão de chat entre o usuário e um Agent.
- **Responsabilidade:** agrupar Messages em sequência com contexto.
- **Relações:** pertence a um Agent; pode estar ancorada a um Document (`document_id`); contém Messages.
- **Estados:** ativa ↔ arquivada (`is_archived`); com/sem documento âncora.
- **Distinções:** no código pode aparecer como "chat" (superfície de UI) — Conversation é o conceito de domínio.

### Message
- **Definição:** uma entrada individual dentro de uma Conversation.
- **Responsabilidade:** carregar o conteúdo trocado e o registro de uso (modelo, tokens).
- **Relações:** pertence a uma Conversation; chega via streaming (eventos).
- **Estados:** papel `user` | `assistant` | `system`; parcial (streaming) → completa.
- **Distinções:** role `system` ≠ configuração do Agent: o prompt do Agent é do Agent; mensagens `system` são da Conversation.

### Memory
- **Definição:** fato persistido sobre o usuário ou seu contexto, extraído ou informado, que a IA reutiliza entre Conversations.
- **Responsabilidade:** dar continuidade de conhecimento ao assistente ("segundo cérebro").
- **Relações:** origina-se de uma Conversation (`source_conversation_id`); injetada no Context; gerenciada pelo subsistema de memória (extração, relevância, injeção).
- **Estados:** categoria fixa (`preferencia` | `contexto` | `habito` | `projeto`); confirmada ou não (`is_confirmed`); ativa/inativa (`is_active`); relevância numérica.
- **Distinções:** Memory ≠ Message (fato consolidado ≠ entrada de chat); Memory ≠ Frontmatter (metadado do Document).

### Context
- **Definição:** o conjunto de informações efetivamente fornecido à IA em uma resposta: documento âncora, Memories relevantes e histórico.
- **Responsabilidade:** determinar o que a IA "enxerga" ao gerar uma resposta.
- **Relações:** montado a partir de Conversation + Document âncora + Memories; materializado no prompt.
- **Distinções:** Context (domínio, o que a IA recebe) ≠ estado do app ≠ contexto de janela/IPC. No código aparece como `document_context` e no montador de prompt.

### LlmProvider
- **Definição:** serviço de IA configurado pelo usuário (nome + URL base).
- **Responsabilidade:** originar Models disponíveis e receber as requisições.
- **Relações:** possui Models; autenticado por API key; registrador de uso (`llm_usage_log`).
- **Estados:** ativo/inativo.
- **Distinções:** Provider é a conexão; Model é o item escolhível dentro dela.

### LlmModel
- **Definição:** modelo de IA oferecido por um Provider e selecionável pelo usuário.
- **Responsabilidade:** identidade do modelo usado em conversas (`model_used`) e padrões (`is_default`, `default_model`).
- **Relações:** pertence a um Provider; referenciado por Agent (`model_default`) e AppConfig.
- **Distinções:** **LlmModel** (modelo de IA) ≠ "model" no sentido de struct Rust em `src-tauri/src/models/` (camada técnica de contrato IPC). Ao falar de IA, diga "modelo de LLM" ou `LlmModel`.

### AppConfig (Configurações)
- **Definição:** preferências da aplicação persistidas (URL do LLM, modelo padrão, Pasta de Documentos, tema, larguras de painéis).
- **Responsabilidade:** configuração global do app, editável em Settings.
- **Relações:** referencia Pasta de Documentos e LlmModel padrão; alimenta o layout.
- **Distinções:** config do app ≠ API key (segredo tratado separadamente) ≠ configuração de um Agent.

## Vocabulário de arquitetura (uso consistente em código e conversas)

### Command
- **Definição:** handler de `invoke()` exposto pelo backend Tauri ao frontend.
- **Responsabilidade:** orquestrar uma operação; nunca contém SQL.
- **Distinções:** Command (backend, invoke) ≠ Command palette (UI, ainda não existente).

### Query
- **Definição:** camada de acesso ao banco (SQL e persistência), chamada pelos Commands.
- **Distinções:** Query é código do backend; não existe "query" no frontend.

### Wrapper (wrapper Tauri)
- **Definição:** função tipada em `src/lib/tauri/` que o frontend usa para chamar um Command.
- **Responsabilidade:** ponto único tipado de comunicação com o backend; componentes nunca chamam `invoke()` direto.

### Store
- **Definição:** estado global reativo do frontend, classe com `$state` interno em `.store.svelte.ts`.
- **Distinções:** Store (global) ≠ estado local de componente. Criar Store sem necessidade de compartilhamento é violação (ver Skill `svelte5-runes`).

### Event
- **Definição:** comunicação assíncrona backend → frontend (ex.: streaming `message_chunk`, `message_done`, `app_error`; eventos do watcher de documentos).
- **Distinções:** Event (push do backend) ≠ Command (pull do frontend).

### Migration
- **Definição:** evolução de schema do banco, append-only, nunca editada após aplicada.

## Limites de contexto do domínio

O BRAINIAC tem quatro frentes conceituais com vocabulário próprio:

1. **Documentos & Editor** — Document, Frontmatter, Pasta de Documentos, Editor.
2. **Chat & IA** — Agent, Conversation, Message, Context, LlmProvider, LlmModel.
3. **Memória** — Memory (e seu ciclo extrair → pontuar → injetar).
4. **Configuração** — AppConfig.

Conceitos com nomes parecidos entre frentes (ex.: "título" de Document vs. de Conversation) devem sempre ser qualificados pelo conceito a que pertencem.

## Regra de consistência conceitual

Ao encontrar (ou precisar nomear) um conceito:

1. Verifique `CONTEXT.md`.
2. Verifique documentação e Skills existentes.
3. Verifique a implementação (models Rust, types TS, schema).
4. Identifique inconsistências e resolva **explicitamente** — não escolha um nome novo em silêncio.
5. Se um termo já definido serve, use-o. Não crie `Note`, `Page`, `DocumentEntity`, `MarkdownFile`, `DocumentModel` para o que já é **Document**.
6. Um conceito genuinamente novo exige: entrada neste arquivo + justificativa no spec da feature (+ ADR se for decisão arquitetural).
