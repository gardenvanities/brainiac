# Governança de Regras — BRAINIAC

## Princípio
AGENTS.md e os arquivos em skills/ são a constituição do projeto. 
Eles não são estáticos — o projeto evolui e as regras devem evoluir 
junto. Mas mudanças nesses arquivos NUNCA são feitas como efeito 
colateral de uma tarefa não relacionada.

## Quando propor uma mudança de regra
Durante qualquer tarefa, se você perceber que:
- Uma regra em AGENTS.md ou em uma Skill contradiz o que está sendo 
  pedido ou o que já foi implementado
- Um padrão se repetiu 2+ vezes de forma diferente do que a regra 
  escrita determina
- Uma decisão do usuário nesta conversa invalida uma regra existente
- Uma tecnologia/prática documentada como "futura" já deveria ser 
  promovida a "atual", ou vice-versa

Então PARE a tarefa em andamento e reporte a divergência antes de 
continuar. Não silenciosamente ignore a regra nem silenciosamente 
a quebre.

## Como propor (nunca aplicar direto)
1. Cite a regra atual (arquivo + trecho exato)
2. Explique a divergência observada (o quê especificamente não bate)
3. Proponha o novo texto da regra
4. Aguarde aprovação explícita do usuário
5. Só então edite o arquivo de regra

Formato da proposta:

  ⚠ Divergência de regra detectada

  Arquivo: [caminho]
  Regra atual: "[trecho exato]"
  Observado: [o que motivou a percepção de divergência]
  Proposta: "[novo texto sugerido]"

  Deseja que eu atualize a regra?

## Exceção — mudanças explicitamente solicitadas
Se o usuário pedir diretamente "atualize a regra X para Y", a 
aprovação já está implícita na instrução — pode editar diretamente, 
sem o formato de proposta acima.

## Registro de mudanças
Toda alteração em AGENTS.md ou em qualquer skills/*.md, seja proposta 
e aprovada ou diretamente solicitada, deve gerar uma linha em 
CHANGELOG.md na seção "Regras e arquitetura", no formato:
[DATA] Regra alterada em [arquivo]: [resumo em uma frase]

## O que NUNCA é permitido, mesmo com aprovação implícita
- Remover a regra de TDD (skills/tdd-workflow.md)
- Remover a regra de fluxo spec-driven (skills/spec-driven-flow.md)
- Remover esta própria skill (rule-governance.md)
Essas três são a fundação do processo e só devem mudar por decisão 
explícita e deliberada do usuário, nunca por proposta espontânea 
do agente.
