---
name: diagnosing-bugs
description: Diagnóstico disciplinado de bugs no BRAINIAC — reproduzir de forma determinística antes de alterar código, isolar com hipóteses verificáveis, criar regression test da correção e remover instrumentação temporária. Usar para bugs, regressões, comportamento inesperado e problemas de integração; substitui o "achismo" por evidência.
disable-model-invocation: false
---

# Diagnosing Bugs — BRAINIAC

> **Nota:** Esta skill referencia `AGENTS.md` (TDD, DoD), `tdd-workflow` (o teste que falha primeiro) e `verification` (nível de verificação). Princípios: **investigar antes de modificar** e **provar que a alteração funciona antes de considerá-la concluída**.

## Quando usar

- Bug reportado ou observado;
- regressão (algo que funcionava parou);
- comportamento inesperado ou inconsistente (ex.: "o documento às vezes não salva");
- erro difícil de reproduzir;
- problema de integração (IPC, streaming, watcher, LLM).

## Fluxo obrigatório

```text
Bug report
   ↓
Reproduzir (determinístico)
   ↓
Isolar (menor caso)
   ↓
Criar feedback loop (teste/observação rápida)
   ↓
Formular hipóteses (poucas e verificáveis)
   ↓
Instrumentar (temporário)
   ↓
Testar hipóteses → causa raiz
   ↓
Corrigir
   ↓
Regression test
   ↓
Verificar
   ↓
Remover instrumentação temporária
```

**Regra crítica:** não comece alterando código com base em hipótese. O ciclo proibido é:

```text
"acho que é aqui" → alteração → teste → outro problema → outra alteração ...
```

## 1. Reproduzir

- Obtenha uma reprodução **determinística** (passos fixos, dados fixos). "Não reproduziu" encerra a rodada — não a correção.
- Se não reproduz de imediato, reduza o acaso: dados de teste fixos, reinício limpo do app, banco de teste, sequência de passos anotada.
- Reprodução intermitente → trate como sinal de race/estado obsoleto (hipóteses H4/H2 abaixo) e tente ampliar a janela do bug, não atirar consertos.

## 2. Isolar

- Menor caso que exibe o bug: qual camada? UI, store, wrapper Tauri, command, query, migration, evento?
- Mapa rápido das fronteiras do BRAINIAC: **Componente → Store → Wrapper `lib/tauri/` → Command → Query → LibSQL**, mais **eventos** (`message_chunk`, `message_done`, `app_error`, watcher) e **LLM** (streaming, prompt).
- Confirmar em que lado da fronteira o estado diverge do esperado (inspecionar store e payload no limite IPC).

## 3. Feedback loop

Monte a repetição mais rápida que mostra o erro: teste Vitest (`tests/frontend/`), teste Rust (`#[cfg(test)]`), ou sequência manual anotada com observação objetiva (log/evento/UI). É este loop que dirá quando a correção funciona — sem ele, tudo é opinião.

## 4. Hipóteses (quando a causa não é evidente)

Formule **poucas** hipóteses verificáveis, nomeadas e com predição observável:

```text
H1 — estado não está sendo persistido (query/save não roda ou falha em silêncio)
H2 — componente lê estado obsoleto (falta reatividade/untrack errado/store antigo)
H3 — evento não chega (listener ausente, cleanup no $effect removeu, nome divergente)
H4 — race condition (streaming vs. save, watcher vs. edição, dois invokes concorrentes)
```

Cada hipótese gera **uma evidência** que a confirma ou elimina. Alteração de código só depois de causa raiz com evidência. Não faça alterações aleatórias "para ver se resolve".

## 5. Instrumentar (temporário)

- Frontend: observação pontual no ponto suspeito; **lembre**: `console.log` é proibido em commit (`AGENTS.md`) — instrumentação é temporária e removida.
- Backend: log/tracing pontual em Rust; observar payload de command/evento.
- Registro da investigação: hipóteses e evidências vão na conversa (e no commit/PR quando relevantes), não viram arquivos permanentes.

## 6. Corrigir + Regression test

- Corrija a **causa raiz**, não o sintoma.
- **Regression test obrigatório (avaliado sempre):** o teste deve falhar sem a correção e passar com ela — conecta com o TDD de `tdd-workflow` (bug = o teste vermelho que faltava).
  - Lógica/store/query → Vitest (`tests/frontend/`) ou `#[cfg(test)]` (Rust).
  - Se não houver teste automatizado possível (ex.: comportamento visual no webview), **documente em uma linha por que a verificação é manual** e como executá-la.
- Não considere o bug corrigido só porque "parou de acontecer": mostre o loop do passo 3 passando.

## 7. Verificar e limpar

- Verificação no nível adequado (`verification`): unit/integration/UI conforme o alcance do bug — incluindo fluxo vizinho que possa ter sido afetado.
- Remova toda instrumentação temporária; `bun run check` e testes passando (DoD).
- Mensagem de commit `fix: ...` citando a causa raiz (ex.: `fix: salvar documento antes de renomear evita perda de frontmatter`).

## Erros comuns

- Corrigir o primeiro palpite sem reprodução.
- "Corrigir" dobrando validação no frontend quando o bug era no command.
- Teste que não reproduz o bug (passa antes e depois) — não é regression test.
- Instrumentação esquecida no commit.
- Bug intermitente "resolvido" por retry sem entender a corrida.
