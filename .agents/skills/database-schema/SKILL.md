---
name: database-schema
description: Schema LibSQL do projeto BRAINIAC — tabelas, PKs, timestamps, soft delete e convenções de armazenamento.
disable-model-invocation: false
---

# BRAINIAC — Schema LibSQL

Tabelas: `agents`, `conversations`, `messages`, `memories`, `documents`, `llm_usage_log`
Todas PKs são UUID v4 (`TEXT`)
Timestamps em ISO 8601 (`TEXT`) via `chrono::Utc::now().to_rfc3339()`
Soft delete: `is_deleted = 1` (nunca `DELETE` físico em documents)
Frontmatter armazenado como JSON string

Agent padrão: `id = '00000000-0000-0000-0000-000000000001'`
Arquivos `.md`: `~/.local/share/project.brainiac/files/`
