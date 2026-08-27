---
name: spec-driven-flow
description: Processo obrigatório do projeto BRAINIAC antes de implementar qualquer feature nova — spec.md (o quê/por quê), plan.md (como técnico) e tasks.md (passos atômicos), cada etapa aguardando aprovação do usuário antes de seguir para a implementação TDD.
disable-model-invocation: false
---

# Fluxo Spec-Driven — BRAINIAC

> **Nota:** Esta skill define o processo obrigatório antes de qualquer implementação de feature nova (não se aplica a bugfixes simples).

## Fluxo Spec-Driven — BRAINIAC

Para qualquer feature nova, siga esta sequência **SEM PULAR ETAPAS**:

---

### 1. `spec.md` — O QUÊ e POR QUÊ
Antes de qualquer código, crie `specs/NOME-DA-FEATURE/spec.md` respondendo:
- Qual problema isso resolve para o usuário?
- Qual o comportamento esperado (casos de uso concretos)?
- O que está FORA de escopo?

> **Pausa Obrigatória:** Pare aqui e aguarde aprovação do usuário sobre o `spec.md` antes de continuar.

---

### 2. `plan.md` — COMO tecnicamente
Após o `spec.md` ser aprovado, crie `specs/NOME-DA-FEATURE/plan.md`:
- Quais arquivos serão criados/modificados
- Quais tabelas/queries são necessárias
- Quais comandos Tauri são necessários
- Riscos técnicos e decisões de arquitetura

> **Pausa Obrigatória:** Pare aqui e aguarde aprovação do usuário sobre o `plan.md` antes de continuar.

---

### 3. `tasks.md` — Passos atômicos e testáveis
Quebre o plano em tarefas pequenas, cada uma com:
- O teste que valida a tarefa
- Critério de "pronto" claro

Cada tarefa deve ser implementável e testável isoladamente.

---

### 4. Implementação
Só agora o código é escrito, seguindo estritamente o workflow TDD (ver skill `tdd-workflow`), uma tarefa de `tasks.md` por vez.

---

## Templates de Referência

Utilize os templates localizados em `specs/_template/` como base para criar os arquivos `spec.md`, `plan.md` e `tasks.md`.