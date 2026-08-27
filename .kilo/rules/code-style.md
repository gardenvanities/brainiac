# Regras de código — BRAINIAC

## Rust
- Sempre usar `?` para propagação de erros, nunca `.unwrap()` em produção
- Locks de Mutex: obter, usar, e deixar o drop acontecer naturalmente
- Não misturar responsabilidades: command chama query, query fala com banco
- Imports organizados: std, crates externos, crates internos

## Svelte / TypeScript
- CSS sempre escopado dentro do componente (não usar classes globais exceto no :global necessário)
- Props de componentes via `$props()` com tipagem explícita
- Eventos de limpeza sempre no return do $effect
- Sem console.log em código commitado

## SQL
- Sempre usar `IF NOT EXISTS` em CREATE TABLE e CREATE INDEX
- Soft delete com `is_deleted = 1`, nunca DELETE físico em documents
- Timestamps em formato ISO 8601 (RFC 3339)
- Parâmetros sempre posicionais (?1, ?2...) nunca interpolação de string
