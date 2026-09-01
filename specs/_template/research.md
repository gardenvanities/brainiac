# Research: [Nome da Feature]

> Documento **factual** — descreve o que já existe e o contexto investigado.
> NÃO define comportamento esperado (isso é o `spec.md`) e NÃO começa a implementar.
> Princípio: **Research Before Design** e **Reuse Before Create** (ver `AGENTS.md`).

## Contexto e objetivo
[1–2 frases: o que se pretende resolver e por que investigar antes de especificar.]

## Código existente relacionado
- **Arquivos:** [caminhos relevantes já existentes]
- **Componentes reutilizáveis:** [components/ que fazem algo parecido]
- **Stores existentes:** [stores/ que podem ser reutilizadas/estendidas]
- **Utilities:** [lib/utils helpers que já resolvem parte do problema]
- **Commands / queries / models / types:** [peças de backend/frontend relacionadas]

## Estado do banco de dados
- **Schema:** [tabelas/colunas relevantes]
- **Migrations:** [migrations existentes e se algo precisa evoluir — lembrar: append-only]

## Padrões semelhantes já implementados
[Como outras features semelhantes foram resolvidas no projeto — aponte o padrão a reutilizar.]

## Skills e decisões relevantes
- **Skills:** [ex.: `tauri-rust-patterns`, `svelte5-runes`, `libsql-schema`, ...]
- **ADRs:** [decisões em `.agents/decisions/` que afetam esta feature]

## Dependências existentes
[O que já está no `Cargo.toml` / `package.json` que pode ser usado, sem adicionar dependência.]

## Riscos técnicos e possíveis conflitos
- [risco 1]
- [conflito potencial com feature/fluxo existente]

## Alternativas de implementação
- **Alternativa A:** [reutilizar X] — prós/contra
- **Alternativa B:** [criar Y] — prós/contra
- **Recomendação:** [qual seguir e por quê]

## Conclusão da pesquisa
[Resumo factual do que foi descoberto — sem decidir o "como" final, que pertence ao `plan.md`.]