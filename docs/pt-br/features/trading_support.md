# Suporte a Bot de Trade no Tupã

Este documento detalha as funcionalidades implementadas no Runtime do Tupã para a linha `0.8.1`, especificamente para suportar aplicações de trading algorítmico como o `ViperTrade`.

## Visão Geral

A linguagem e o runtime do Tupã foram aprimorados para suportar operações financeiras críticas, garantindo segurança, resiliência e auditabilidade.

## Funcionalidades Chave

### 1. Engine de Backtesting

A função `run_backtest` fornece um ambiente de simulação integrado para estratégias de trading.

- **Propósito**: Validar a lógica da estratégia contra dados históricos antes do deploy em produção.
- **Mecanismo**:
  - Itera sobre um dataset de candles/ticks históricos.
  - Executa o pipeline para cada passo de tempo.
  - Avalia restrições de risco (ex: `MaxDrawdown`, `PositionSize`).
  - Rastreia PnL (Lucro e Perda) do portfólio baseado em sinais `BUY`/`SELL` e preços de fechamento (`close`).
- **Auditoria**: Cada trade e ação bloqueada é registrada com uma trilha de auditoria estruturada.

### 2. Circuit Breaker

Um padrão de resiliência para prevenir falhas em cascata durante volatilidade de mercado ou interrupções de API.

- **Configuração**:
  - `failure_threshold`: Número de erros consecutivos permitidos (ex: 3).
  - `reset_timeout`: Tempo de espera antes de testar a conexão novamente (ex: 30s).
- **Comportamento**:
  - **Closed**: Operação normal.
  - **Open**: Bloqueia execução imediatamente quando o limite é atingido.
  - **Half-Open**: Permite uma única requisição de teste para verificar recuperação.

### 3. Integração Python AI (`tupa-pyffi`)

Integração perfeita com modelos de ML baseados em Python (PyTorch/TensorFlow) para geração de sinais.

- **Sintaxe**: Passos definidos como `py:module.func` (ex: `py:viper_model.predict`).
- **Segurança**: Entradas e saídas são validadas contra esquemas estritos (ex: Tensor shapes `[1, 60, 4]`).
- **Desempenho**: Transferência de dados Zero-copy (onde possível) via FFI.

### 4. Logging de Auditoria Estruturado

Logging pronto para conformidade usando a crate `tracing`.

- **Formato**: Logs estruturados em JSON.
- **Eventos**:
  - `pipeline_start` / `pipeline_complete`
  - `trade_executed` (com preço, tipo e índice)
  - `trade_blocked_by_risk` (quando restrições falham)
  - `circuit_breaker_tripped`

### 5. Config tipada fornecida pelo host via input estruturado

O Tupã já suporta um padrão prático de config binding para sistemas de estratégia em produção:

- declarar o input do pipeline como um record aninhado
- passar dados de mercado e config no mesmo objeto tipado
- usar field access comum dentro das funções de policy

Isso já cobre muitos casos de estratégia, como:

- thresholds por símbolo
- overlays por modo/perfil
- parâmetros de trailing
- thresholds de confirmação

Shape de exemplo:

```text
input: {
  symbol: string,
  signal: { spread_pct: f64, trend_score: f64 },
  config: {
    entry: {
      max_spread_pct: f64,
      min_trend_score_long: f64
    }
  }
}
```

Veja:

- `examples/pipeline/config_driven_strategy.tp`
- `examples/pipeline/config_driven_strategy.json`

### 6. Política temporal declarativa via estado fornecido pelo host

O Tupã já consegue modelar um primeiro slice de política temporal sem mover o estado do host para
o runtime da linguagem:

- o host mantém contadores e timers
- o pipeline recebe esse estado temporal como input estruturado
- built-ins expressam o shape do resultado para confirmação e cooldown

Built-ins atuais:

- `confirm(observed, consecutive_hits, required_hits, reason)`
- `cooldown(active, remaining_ticks, reason)`

Isso é útil para casos como:

- confirmação de sinal após `N` observações consecutivas
- cooldown após stop loss
- persistência de tese dirigida por contadores mantidos no host

Shape de exemplo:

```text
input: {
  temporal: {
    signal_confirmation: {
      observed: bool,
      consecutive_hits: i64,
      required_hits: i64
    },
    cooldown_guard: {
      active: bool,
      remaining_seconds: i64
    },
    thesis_confirmation: {
      observed: bool,
      consecutive_hits: i64,
      required_hits: i64
    }
  }
}
```

Veja:

- `examples/pipeline/temporal_policy.tp`
- `examples/pipeline/temporal_policy.json`

## Exemplo de Uso

```rust
// Configurando o runtime para um bot de trade
let runtime = Runtime::new();
runtime.configure_circuit_breaker(3, Duration::from_secs(10));

// Rodando um backtest
let result = runtime.run_backtest(&plan, historical_data).await?;
println!("PnL Final: {}", result["final_pnl"]);
```
