# RFC: v0.8.1 Suporte a Estratégias de Trading

## Status

- Proposta
- Versão alvo: `0.8.1`
- Motivador principal: modelagem da estratégia de produção do ViperTrade

## Resumo

O TupaLang já suporta pipelines determinísticas, auditoria e integração controlada em runtime. Para sistemas de trading de produção como o ViperTrade, o próximo gap não é de infraestrutura. O gap é de modelagem expressiva da estratégia.

A versão `0.8.1` deve focar nas features de linguagem e runtime necessárias para modelar políticas reais de trading de forma declarativa, em vez de deixar a semântica central da estratégia hardcoded no código da aplicação.

Esta RFC propõe um tema de release objetivo:

- outputs estruturados por step
- `reason` como conceito de primeira classe
- predicados reutilizáveis
- bindings tipados para configuração
- suporte a score ponderado
- suporte declarativo a políticas temporais

## Problema

Hoje o ViperTrade usa o TupaLang mais como casca de pipeline, enquanto a semântica real da estratégia ainda mora no Rust:

- gates de entrada
- reasons de hold
- position health score
- regras de thesis invalidation
- parte da política de confirmação de sinal e cooldown

Essa divisão reduz o valor do Tupa justamente na área em que ele deveria ajudar mais:

- auditabilidade
- explainability
- iteração segura
- revisão de estratégia
- confiança de release

## Objetivos

A `0.8.1` deve tornar o TupaLang materialmente melhor para sistemas de estratégia de produção.

### Objetivos principais

- Permitir que steps retornem resultados estruturados tipados, em vez de apenas valores primitivos.
- Permitir que regras de estratégia emitam reasons legíveis por máquina diretamente.
- Permitir composição de políticas com predicados reutilizáveis.
- Permitir que políticas de estratégia leiam configuração de runtime tipada sem empurrar toda a semântica para o host.
- Permitir scores ponderados de forma declarativa.
- Permitir descrever políticas temporais sem embutir toda a semântica no host.

### Fora de escopo

- Substituir responsabilidades do runtime host, como Redis, banco, IO de exchange ou orquestração de processos.
- Mover todo o estado para dentro do TupaLang.
- Criar uma DSL completa de trading em uma única release.

## Features propostas

### 1. Outputs estruturados por step

Suportar outputs em formato de record/objeto com campos explícitos e validação de tipo.

Exemplos desejados:

- `EntryPolicyResult`
- `SizingResult`
- `PositionHealthResult`
- `ThesisExitResult`
- `StrategyDecision`

Forma ilustrativa:

```text
step("entry_policy") {
  {
    eligible: true,
    side: long,
    entry_score: 72,
    reason: "entry_confirmed_consensus_and_momentum"
  }
}
```

### 2. `reason` de primeira classe

Permitir que o resultado de uma política carregue naturalmente o motivo.

Forma ilustrativa:

```text
{
  passed: false,
  reason: "entry_blocked_low_volume"
}
```

Isso evita duplicação de lógica no host apenas para explicar por que uma regra falhou.

### 3. Predicados reutilizaveis

Permitir políticas compostas por predicados nomeados reutilizáveis, como:

- `passes_consensus(side)`
- `passes_momentum(side)`
- `passes_macro(side)`
- `passes_liquidity(side)`

Isso reduz duplicação entre:

- long e short
- entry e hold
- health e exit

### 4. Suporte a score ponderado

Suportar composição de score ponderado para políticas como position health.

Casos de uso:

- alinhamento de consenso com peso alto
- regime da exchange âncora com peso médio/alto
- alinhamento macro de BTC com peso médio
- sinal do histograma do MACD com peso baixo/médio

Capacidades desejadas:

- componentes aditivos
- penalidades e bônus
- clamp/faixa
- comparação com thresholds

### 5. Bindings tipados para configuração

Suportar acesso tipado a valores de configuração fornecidos pelo host e usados por sistemas de estratégia em produção.

Casos de uso ilustrativos:

- thresholds por símbolo
- overlays por modo
- parâmetros de trailing
- thresholds de confirmação
- thresholds de filtros macro

Capacidades desejadas:

- leituras tipadas de um objeto de configuração fornecido pelo host
- defaults explícitos ou regras claras de fallback
- validação de shape antes da execução do runtime

### 6. Suporte declarativo a politicas temporais

Suportar semânticas que dependem de persistência por ciclos de avaliação.

Padroes alvo:

- confirmar por `N` ticks
- degradar por `N` ticks antes de sair
- cooldown após stop loss
- bloquear reentrada até flip do lado

O runtime host ainda pode manter o estado, mas a política em si deveria ser declarada no TupaLang.

## Por que isso importa para o ViperTrade

Com essas features, o ViperTrade consegue mover a semântica central da estratégia para o TupaLang:

- elegibilidade de entrada
- reasons de hold
- position health score
- thesis invalidation
- parte da política de reentrada e confirmação

Assim o Rust fica concentrado no papel correto:

- orquestração de runtime
- persistência de estado
- integracao com exchange
- transporte de eventos
- plumbing de risco

## Refinamento de prioridade após a integração com o ViperTrade

A migração `0.8.1` do ViperTrade deixou o próximo gargalo bem claro.

O que a linguagem já resolve bem o suficiente:

- records tipados
- record literals
- helpers de `reason`
- helpers de score ponderado
- reutilização via funções normais para boa parte dos helpers tipo predicado

O que ainda força wiring excessivo no host:

- acesso a configuração
- overlays de modo/perfil
- seleção de thresholds a partir da configuração da estratégia
- semântica temporal de confirmação e cooldown

Como resultado, a próxima prioridade de implementação deveria ser:

1. bindings tipados para configuração
2. suporte declarativo a políticas temporais
3. ergonomia melhor para predicados reutilizáveis quando funções comuns ainda forem insuficientes

## Ordem de entrega proposta

### Fase 1

- outputs estruturados por step
- `reason` de primeira classe
- suporte a score ponderado

Esse é o mínimo necessário para mover política de entrada, reasons de `HOLD` e modelos de score para o TupaLang.

### Fase 2

- bindings tipados para configuração

Isso habilita estratégias de produção como o ViperTrade a ler thresholds e overlays de perfil de forma declarativa.

### Fase 3

- suporte declarativo a políticas temporais

Isso habilita confirmação de sinal, janelas de persistência de tese e semântica de cooldown.

### Fase 4

- ergonomia para predicados reutilizáveis

Predicados nomeados continuam úteis, mas o ViperTrade mostrou que funções normais já cobrem parte dessa necessidade hoje.

## Impacto esperado

Se a `0.8.1` entregar esse escopo, o TupaLang passa a ser muito mais útil para sistemas reais de política, e não apenas para orquestração de pipelines.

Resultados esperados:

- menos duplicação de estratégia no host
- revisões de release mais claras
- ajuste de estratégia mais fácil
- trilhas de auditoria melhores
- melhor aderência a aplicações reais de trading

## Questoes em aberto

- O suporte a score ponderado deve nascer como sintaxe da linguagem ou como helper de biblioteca padrão?
- `reason` deve ser convenção sobre records estruturados ou primitiva própria da linguagem?
- Até onde o suporte temporal deve ir na `0.8.1` antes de virar desenho de máquina de estados?

## Recomendacao

Adotar esta RFC como âncora de planejamento da `0.8.1`, com a Fase 1 como barra mínima de release e as Fases 2-3 como escopo alvo quando o custo de implementação permanecer controlado.
