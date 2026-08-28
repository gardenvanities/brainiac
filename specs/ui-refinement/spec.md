# Spec: Refinamento minimalista da UI principal

> **Princípio norteador:** o conteúdo é o protagonista. A interface deve desaparecer quando não for necessária.

---

## 1. Problema

A tela principal exibe uma quantidade fixa de "chrome" (barras, cabeçalhos, rodapés, bordas, banners e botões) que **nunca recua**, mesmo quando parte dele é decorativo ou sem função real. Consequências concretas, medidas no código atual:

- O texto do editor disputa atenção com: toolbar fixa de 48px, header fixo de 48px da sidebar de IA, banner de contexto fixo, footer fixo da sidebar esquerda com 3 botões, nav fixa com 3 seções, e 4 linhas de separação redundantes (borda do aside + resize-handle, em ambos os lados).
- Parte desse chrome está **morta ou enganosa**: botões "Memórias"/"Agentes" sem ação, botão "+" de nova conversa sem ação, seções "Buscar"/"Tags" sem implementação, modelo "claude-sonnet-4-6" hardcoded que não corresponde ao provider real, banner "Sem documento aberto" permanente e estático.
- **Não existe nenhum modo de foco**: as duas sidebars não podem ser recolhidas, apesar de o `uiStore` já declarar `sidebarLeftOpen`/`sidebarRightOpen` (estado pronto, sem fio para a UI).

---

## 2. Objetivos (o que muda no comportamento da interface)

### 2.1 Sidebars recolhíveis

- A sidebar esquerda e a sidebar direita podem ser **abertas/fechadas independentemente**:
  - por **atalho de teclado**;
  - por um **controle discreto** visível apenas quando relevante (borda do app / hover na margem).
- Estado default na abertura do app: **ambas abertas** (comportamento atual preservado).
- Com uma sidebar fechada, o espaço dela é cedido integralmente ao editor (as colunas do grid colapsam para 0; os resize-handles correspondentes desaparecem).
- Com ambas fechadas = **modo foco**: apenas o editor centralizado ocupa a janela.

### 2.2 Atalhos de teclado (visíveis nas dicas)

| Atalho | Ação |
|---|---|
| `Ctrl/Cmd+B` | alternar sidebar esquerda |
| `Ctrl/Cmd+J` | alternar sidebar de IA (direita) |
| `Ctrl/Cmd+Shift+F` | modo foco (alterna ambas juntas) |

Respeitar o sistema: não capturar quando o foco estiver em um campo de texto já tratado por atalho nativo do editor (o handler global ignora se o target for `contentEditable`/input e a tecla tiver handler próprio do editor — regra: só atalhos com modificador, que não colidem).

### 2.3 Subtração de chrome morto

- **Sidebar esquerda:** o footer passa a ter apenas o acesso a **Configurações** (ícone discreto). "Memórias" e "Agentes" (inertes) saem da barra permanente. A nav de seções exibe apenas o que tem conteúdo real (Arquivos; Buscar/Tags saem enquanto não implementadas).
- **Sidebar de IA:** o header mostra o **provider/modelo real** (do estado de config existente) ou apenas o nome da aplicação; o botão "+" de nova conversa ganha função real (limpa a conversa atual) **ou** é removido — decisão: **removido** nesta iteração para não prometer ação não testada.
- **Banner de contexto do chat** sõ aparece **quando há documento aberto**, e passa a mostrar o nome real do documento ativo (reativo); sem documento, não ocupa espaço nenhum.

### 2.4 Separações visuais mais leves

- As bordas laterais dos asides (`border-right`/`border-left`) são **removidas**: o resize-handle (4px) é o único separador, visível em repouso como linha ultra-sutil (`--color-border-subtle`) e colorido com o accent no hover/drag (comportamento de cor já existe).
- Bordas internas que dividem regiões que o usuário não operacionaliza são rebaixadas de `--color-border-default` para `--color-border-subtle` (toolbar do editor, headers de sidebar).

### 2.5 Editor como palco

- A hierarquia de paddings do editor é preservada; com as duas sidebars fechadas, o `max-width` de leitura (760px) permanece (linhas não esticam demais).
- Nenhuma mudança de cor/tema/tipografia de conteúdo — apenas densidade e visibilidade de chrome.

### 2.6 Limpeza acoplada (pequena, justificada pelo research)

- Remover o `console.log` solto em `messages.store.svelte.ts` (viola AGENTS.md).
- Remover flags parasitas do `uiStore` que não têm consumidor (`activeDocumentId`, `activeAgentId` se permanecerem sem uso na implementação).

---

## 3. Comportamentos esperados (aceite)

1. **Alternância por teclado:** com um documento aberto, `Ctrl/Cmd+B` fecha a esquerda; repete-se, ela volta com o mesmo conteúdo/estado (seleção do documento preservada).
2. **Alternância da IA:** `Ctrl/Cmd+J` abre/fecha a direita; a conversa em andamento (mensagens) não é destruída ao fechar e reabrir.
3. **Modo foco:** `Ctrl/Cmd+Shift+F` esconde ambas; o editor ocupa toda a janela com o mesmo conteúdo; repetir o atalho restaura o layout anterior (esquerda+direita abertas).
4. **Sem chrome morto:** os botões sem ação ("Memórias", "Agentes", "+" da conversa) não aparecem mais; "Modelo" exibido corresponde à configuração real ou o header é simplificado.
5. **Banner contextual:** aberto um documento → o chat mostra "Contexto: <nome do arquivo>"; fechado o documento → o banner some (sem deixar faixa vazia).
6. **Separação única:** entre sidebar e editor existe **uma só** linha de separação visual por lado, que reage no hover/drag; nada de borda dupla.
7. **Drag das larguras continua funcionando** quando a respectiva sidebar está aberta; não deve ser possível "arrastar" um handle de painel fechado.
8. **Nenhuma regressão de conteúdo:** criar documento, editar (inline title Cenários A/B continuam funcionando), autosave, enviar mensagem de chat — todos continuam operando.

---

## 4. Fora de escopo (não nesta entrega)

- Persistência do estado de UI entre sessões (larguras/aberturas ficam na sessão).
- Implementação real de Busca/Tags/Memórias/Agentes (apenas a remoção do chrome morto).
- Editor distraction-free com auto-hide de toolbar por inatividade (fase futura).
- Layout responsivo/mobile (o app é desktop).
- Novas cores ou novos tokens de cor (nenhum necessário).
- Remoção dos arquivos de rotas legados vazios (`Settings/Home/Agents/Memories.svelte`, etc.) — será avaliada no plano como chore separado.

---

## 5. Riscos e cuidados

- **Editor (Svelte 5 + Milkdown):** ao recolher/reexpandir o painel, o Editor **não pode ser destruído/recriado** (o `$effect` de `Editor.svelte` reage a `active?.id` — colapso de largura não pode tocar em `active`; se a sidebar for desmontada com `{#if}` o conteúdo dos stores precisa permanecer intacto — os dados vivem nos stores, portanto remontar é seguro; testar explicitamente).
- **Resize por CSS grid:** colapsar para `0` deve ser feito com transição suave de largura e com os handles fora do fluxo quando a sidebar estiver fechada (senão sobra a alça flutuante).
- **Atalhos globais:** o handler deve viver em um único lugar (layout/AppShell) com cleanup correto (`$effect` retornando removeEventListener) e não interceptar digitação (guard por target).
- **Drag region do Tauri:** com headers de sidebar removidos/rebaixados, a janela continua arrastável pela toolbar central (`data-tauri-drag-region`) — rebaixamento não pode zerar todas as áreas de arraste.
