# Suporte a Bot de Trade no Tupã

Este documento detalha as funcionalidades implementadas no Runtime do Tupã (v0.8.1-dev) especificamente para suportar aplicações de trading algorítmico como o `ViperTrade`.

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

## Exemplo de Uso

```rust
// Configurando o runtime para um bot de trade
let runtime = Runtime::new();
runtime.configure_circuit_breaker(3, Duration::from_secs(10));

// Rodando um backtest
let result = runtime.run_backtest(&plan, historical_data).await?;
println!("PnL Final: {}", result["final_pnl"]);
```
