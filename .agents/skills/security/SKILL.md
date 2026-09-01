---
name: security
description: Revisão de segurança proporcional ao risco da feature no BRAINIAC — filesystem (path traversal, symlinks), Tauri (commands/IPC/permissions), IA (prompt injection, context poisoning, secrets) e dados (API keys, logs, persistência). Instrumento de revisão, não de implementação.
disable-model-invocation: false
---

# Security Review — BRAINIAC

> **Nota:** Esta skill complementa `code-review` e `AGENTS.md`. É uma **revisão** — não converte features sem superfície de ataque em burocracia. Ative pelo risco real da mudança.

## Princípio

A revisão deve ser **proporcional ao risco da feature**. Uma função pura de exibição não precisa de análise de IPC; um novo command que escreve no filesystem precisa. Não introduza complexidade de segurança irrelevante para o caso.

---

## Áreas de revisão

### Filesystem
- [ ] path traversal: entrada do usuário nunca compõe caminho sem sanitização;
- [ ] paths arbitrários: resolvidos dentro dos diretórios permitidos (ex.: `app_data_dir()/files`);
- [ ] symlinks: leitura/escrita não sai do diretório pretendido por link simbólico;
- [ ] operações destrutivas (rename/delete/overwrite) validam alvo e existência;
- [ ] nome de arquivo validado (ver `normalize_document_name` em `commands/documents.rs`).

### Tauri
- [ ] commands expostos são mínimos e necessários;
- [ ] permissões/capabilities (`src-tauri/capabilities/`) limitadas ao necessário;
- [ ] argumentos de IPC validados no backend (não confiar no frontend);
- [ ] respostas não vazam dados sensíveis sem necessidade.

### IA
- [ ] prompt injection: conteúdo de documentos/memórias é tratado como **não confiável**;
- [ ] context poisoning: conteúdo não controlado não dita instruções ao modelo;
- [ ] vazamento de informação: contexto não envia mais do que o necessário;
- [ ] exposição de secrets: chaves nunca entram no prompt nem em logs;
- [ ] ferramentas/agentes com permissões excessivas (menos privilégio).

### Dados
- [ ] API keys/tokens: nunca em logs, nunca retornadas a mais do que o mínimo;
- [ ] logs: sem dados privados, conteúdo de memória ou segredos;
- [ ] persistência: dados sensíveis no banco/filesystem com o cuidado devido;
- [ ] conteúdo de memória: leitura/modificação controlada (sem exfiltração acidental).

---

## Resultado

Reporte de forma objetiva:

1. **Surface de ataque identificada** (ou "nenhuma significativa" — com justificativa).
2. **Problemas críticos** — exigem correção antes de entregar (ver Definition of Done).
3. **Problemas não-bloqueantes** — recomendações.
4. **Conclusão:** `PASS` ou `FAIL`.

Se a feature não tiver a superfície correspondente a uma área, registre explicitamente que a área foi considerada e dispensada (não apenas omitida).