# Notas Pós-Release: v0.8.1

## Propósito

Esta nota registra o que a `v0.8.1` nos ensinou depois de publicarmos a integração entre a
linguagem, o runtime e o `ViperTrade`.

## O que funcionou bem

- o `ViperTrade` serviu como prova funcional real do `TupaLang`
- slices pequenos e mergeáveis mantiveram o release andando sem perder confiança
- a sequência de features foi coerente:
  - outputs estruturados
  - reasons de política como first-class
  - suporte a weighted score
  - padrão de input tipado para config
  - suporte declarativo a política temporal
- a documentação do release foi ajustada antes da tag pública, não depois

## Lições principais

### 1. O gargalo deixou de ser shape de dados

Depois que `record types`, `record literals`, field access tipado, validação estruturada no
runtime e outputs ponderados de política ficaram prontos, os próximos ganhos deixaram de vir
de mais maquinaria de shape.

A alavanca real passou a ser:

- tornar a política reutilizável
- modelar política temporal de forma explícita
- passar estado tipado provido pelo host para o pipeline

### 2. O host deve manter o estado operacional

O trabalho da `0.8.1` confirmou que a linguagem fica mais útil quando modela política, e não
quando tenta absorver todo o estado do host.

Preocupações stateful continuam pertencendo à aplicação host:

- contadores de confirmação de sinal
- controle de cooldown
- estado de trailing stop
- persistência e efeitos externos

A linguagem ajudou mais quando conseguiu descrever como esse estado deve ser interpretado.

### 3. O pipeline ganhou valor quando passou a refletir shapes reais

A camada `.tp` ficou materialmente mais útil quando deixou de ser só um placeholder
arquitetural e passou a receber:

- inputs estruturados
- snapshots reais de estado temporal
- outputs estruturados consumidos pelo runtime da aplicação

Isso transformou o pipeline em um contrato real, e não em um rascunho de design futuro.

### 4. O tooling standalone precisa entrar cedo na validação

O fluxo de validação local expôs uma lição operacional importante: se o caminho de validação
usa um binário `tupa` antigo, o pipeline parece quebrado mesmo quando as mudanças da linguagem
estão corretas.

No próximo ciclo, vale alinhar cedo:

- `tupa-cli`
- scripts locais de validação
- imagens de contêiner usadas por CI ou compose

## O que faríamos mais cedo na próxima vez

- alinhar antes o CLI standalone e o caminho de validação local
- documentar antes a fronteira entre política declarativa e estado gerenciado pelo host
- adicionar antes alguns testes focados para os novos outputs estruturados expostos

## O que a v0.8.1 entregou

A `v0.8.1` moveu o `TupaLang` de um runtime de pipelines com governança básica para uma
linguagem mais útil para sistemas reais de estratégia:

- outputs podem ser estruturados e tipados
- reasons podem ser first-class
- política pode ser composta com weighted scores
- política temporal pode ser expressa declarativamente
- config tipada provida pelo host pode ser modelada sem nova sintaxe no core

## Próximos pontos de pressão

Os próximos ganhos relevantes não vêm de mais primitivas de shape. Eles vêm de:

- melhor ergonomia para blocos reutilizáveis de política
- continuar refinando a fronteira entre estado do host e política declarativa
- decidir se alguns breakdowns estruturados internos devem virar contratos públicos
