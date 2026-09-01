# Tipografia do Editor — BRAINIAC

## Princípio central
O editor é majoritariamente texto. Toda decisão tipográfica prioriza 
legibilidade e ritmo vertical consistente sobre densidade de informação.

## Regras obrigatórias

1. Largura de leitura: o conteúdo de prosa nunca ultrapassa 
   var(--prose-measure) de largura. Elementos de largura total 
   (tabelas largas, imagens) podem escapar dessa medida pontualmente.

2. Hierarquia por peso E tamanho, nunca só um dos dois. H1 é maior 
   E mais pesado que H2, e assim sucessivamente.

3. Espaçamento antes de heading é sempre maior que depois — cria 
   agrupamento visual do heading com o conteúdo que ele introduz, 
   não com o conteúdo anterior.

4. Blockquote e callout usam borda lateral colorida + leve mudança 
   de fundo, nunca itálico como único diferenciador (itálico sozinho 
   é fraco visualmente em telas).

5. Código inline nunca quebra o ritmo vertical da linha — padding 
   vertical mínimo, cor de fundo sutil (--color-bg-elevated), nunca 
   borda.

6. Bloco de código sempre usa --font-code (Google Sans Code) com 
   ligaturas ativas, nunca a fonte de corpo.

7. Links usam --color-link, sublinhado sutil (text-decoration: underline, 
   text-underline-offset para não colar no texto), --color-link-visited 
   após clicado.

8. Listas usam var(--prose-indent) consistente em todos os níveis de 
   aninhamento, marcadores na cor --color-text-muted (não --color-text-primary, 
   para não competir visualmente com o conteúdo).

Toda nova regra de estilo do editor deve ser validada contra este 
documento antes de ser escrita.
