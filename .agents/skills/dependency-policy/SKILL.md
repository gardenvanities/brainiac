---
name: dependency-policy
description: Política de dependências do BRAINIAC — antes de adicionar qualquer dependência (Rust ou JS), esgotar o que já existe, preferir APIs nativas e código existente, e documentar a decisão. Complementa a regra Reuse Before Create do AGENTS.md.
disable-model-invocation: false
---

# Dependency Policy — BRAINIAC

> **Nota:** Esta skill materializa a regra global **Reuse Before Create** (`AGENTS.md`) no contexto de dependências. Consultar antes de tocar `Cargo.toml` ou `package.json`.

## Princípio

> Não adicionar dependências simplesmente para evitar algumas linhas de código.

Uma dependência nova é um **custo permanente de manutenção** (atualizações, segurança, compatibilidade, superfície de ataque). Deve ser uma decisão consciente e documentada.

---

## Fluxo obrigatório (antes de adicionar)

```text
Preciso de uma dependência?
    ↓
1. Já existe nas dependências atuais?
    ↓
2. Pode ser resolvido com APIs nativas?
    ↓
3. Pode ser implementado com código existente do projeto?
    ↓
4. Qual o custo de manutenção se eu adicionar?
    ↓
Adicionar (somente se 1–3 falharam e o custo se justifica)
```

1. **Verificar dependências atuais** — muitas vezes a funcionalidade já está no `Cargo.toml` / `package.json` (ex.: o projeto já tem `serde_json`, `chrono`, `gray_matter`, `notify`, `thiserror`, `reqwest`, `uuid`).
2. **APIs nativas primeiro** — antes de um crate/package, considere `std`/APIs web (ex.: manipulação de texto, path, datas simples).
3. **Código existente** — um helper que já existe no projeto (em `src/lib/utils/` ou `filesystem/`) pode resolver sem dep nova.
4. **Custo de manutenção** — atualizações, breaking changes, segurança, peso no binário/bundle.

---

## Quando a dependência é realmente necessária

Documente a decisão (no `research.md`/`plan.md` da feature, ou no PR):

```text
Dependência:
Motivo:
Alternativas consideradas:
Por que as alternativas não são adequadas:
Impacto:
```

Exemplo de como preencher:

```text
Dependência: vitest (dev)
Motivo: testes unitários do frontend (código puro em src/lib/utils/)
Alternativas consideradas: none (sem runner), bun test, jest
Por que as alternativas não são adequadas: bun test não integra com o setup Svelte atual; jest mais pesado e menos alinhado ao ecossistema Vite
Impacto: dev-only; adiciona script "test" e integração Vitest; sem custo de runtime
```

---

## Regras de red line

- Nunca adicionar dependência **sem** ter passado pelos passos 1–4.
- Nunca adicionar dependência para evitar "algumas linhas de código" triviais.
- Dev-dependências (ex.: test runner, tipos) têm custo menor, mas ainda exigem a verificação.
- Preferir dependências já usadas no projeto (consistência) quando houver escolha equivalente.

Quando a decisão envolver arquitetura difícil de reverter (ex.: trocar motor de banco, abandonar uma lib central), considerar um ADR em `.agents/decisions/`.