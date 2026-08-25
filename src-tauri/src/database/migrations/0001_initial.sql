-- ================================================================
-- SCHEMA MIGRATIONS
-- Controla quais migrations já foram executadas
-- ================================================================
CREATE TABLE IF NOT EXISTS schema_migrations (
    version     TEXT PRIMARY KEY,
    executed_at TEXT NOT NULL
);

-- ================================================================
-- AGENTS
-- Personas com especialidades, modelos e personalidades próprias
-- ================================================================
CREATE TABLE IF NOT EXISTS agents (
    id            TEXT PRIMARY KEY,
    name          TEXT NOT NULL,
    description   TEXT,
    system_prompt TEXT NOT NULL,
    model_default TEXT NOT NULL,
    avatar_path   TEXT,
    is_default    INTEGER NOT NULL DEFAULT 0,
    is_active     INTEGER NOT NULL DEFAULT 1,
    created_at    TEXT NOT NULL,
    updated_at    TEXT NOT NULL
);

-- ================================================================
-- CONVERSATIONS
-- Sessões de chat vinculadas a um agente e opcionalmente a um documento
-- ================================================================
CREATE TABLE IF NOT EXISTS conversations (
    id          TEXT PRIMARY KEY,
    agent_id    TEXT NOT NULL,
    document_id TEXT,
    title       TEXT,
    model_used  TEXT NOT NULL,
    is_archived INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL,
    FOREIGN KEY (agent_id) REFERENCES agents (id),
    FOREIGN KEY (document_id) REFERENCES documents (id)
);

-- ================================================================
-- MESSAGES
-- Mensagens individuais com auditoria de tokens
-- ================================================================
CREATE TABLE IF NOT EXISTS messages (
    id              TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content         TEXT NOT NULL,
    model_used      TEXT,
    tokens_input    INTEGER,
    tokens_output   INTEGER,
    created_at      TEXT NOT NULL,
    FOREIGN KEY (conversation_id) REFERENCES conversations (id)
);

-- ================================================================
-- MEMORIES
-- Fatos extraídos automaticamente sobre o usuário
-- ================================================================
CREATE TABLE IF NOT EXISTS memories (
    id                     TEXT PRIMARY KEY,
    category               TEXT NOT NULL CHECK (category IN ('preferencia', 'contexto', 'habito', 'projeto')),
    fact                   TEXT NOT NULL,
    relevance              REAL NOT NULL DEFAULT 1.0,
    source_conversation_id TEXT,
    is_confirmed           INTEGER NOT NULL DEFAULT 0,
    is_active              INTEGER NOT NULL DEFAULT 1,
    created_at             TEXT NOT NULL,
    updated_at             TEXT NOT NULL,
    FOREIGN KEY (source_conversation_id) REFERENCES conversations (id)
);

-- ================================================================
-- DOCUMENTS
-- Metadados dos arquivos .md — conteúdo vive no filesystem
-- ================================================================
CREATE TABLE IF NOT EXISTS documents (
    id          TEXT PRIMARY KEY,
    path        TEXT NOT NULL UNIQUE,
    title       TEXT NOT NULL,
    frontmatter TEXT,
    word_count  INTEGER NOT NULL DEFAULT 0,
    is_deleted  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

-- ================================================================
-- LLM USAGE LOG
-- Auditoria de uso por modelo
-- ================================================================
CREATE TABLE IF NOT EXISTS llm_usage_log (
    id            TEXT PRIMARY KEY,
    message_id    TEXT,
    agent_id      TEXT,
    model         TEXT NOT NULL,
    tokens_input  INTEGER NOT NULL,
    tokens_output INTEGER NOT NULL,
    latency_ms    INTEGER,
    created_at    TEXT NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages (id),
    FOREIGN KEY (agent_id) REFERENCES agents (id)
);

-- ================================================================
-- INDEXES
-- ================================================================
CREATE INDEX IF NOT EXISTS idx_conversations_agent    ON conversations (agent_id);
CREATE INDEX IF NOT EXISTS idx_conversations_document ON conversations (document_id);
CREATE INDEX IF NOT EXISTS idx_messages_conversation  ON messages (conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_created       ON messages (created_at);
CREATE INDEX IF NOT EXISTS idx_memories_category      ON memories (category);
CREATE INDEX IF NOT EXISTS idx_memories_relevance     ON memories (relevance);
CREATE INDEX IF NOT EXISTS idx_documents_path         ON documents (path);
CREATE INDEX IF NOT EXISTS idx_llm_usage_model        ON llm_usage_log (model);
CREATE INDEX IF NOT EXISTS idx_llm_usage_created      ON llm_usage_log (created_at);

-- ================================================================
-- AGENT PADRÃO
-- BRAINIAC inserido na inicialização
-- ================================================================
INSERT OR IGNORE INTO agents (
    id, name, description, system_prompt, model_default,
    is_default, is_active, created_at, updated_at
) VALUES (
    '00000000-0000-0000-0000-000000000001',
    'BRAINIAC',
    'Assistente pessoal padrão. Aprende e se adapta ao usuário com o tempo.',
    'Você é o BRAINIAC, um segundo cérebro pessoal e adaptativo. Você conhece profundamente o usuário e personaliza cada resposta com base no que aprendeu sobre ele. Seja direto, preciso e útil. Aprenda continuamente com cada interação.',
    'claude-sonnet-4-6',
    1, 1,
    '2024-01-01T00:00:00Z',
    '2024-01-01T00:00:00Z'
);
