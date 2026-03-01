# Guia de Testes

## Propósito

Descrever comandos padrão de teste e dicas de triagem de falhas.

## Comandos principais

```bash
# suite completa
cargo test

# por crate
cargo test -p tupa-lexer
cargo test -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli
```

## Testes do CLI

```bash
# saídas golden
cargo test -p tupa-cli -- tests::cli_golden
```

## Testes de desempenho

- Objetivo: verificar tempo de execução para exemplos médios (alvo < 200ms).
- Como rodar com logs:
  - `cargo test -p tupa-cli perf -- --nocapture`
- O que é verificado:
  - Codegen do exemplo `fraud_complete` abaixo de 500ms (limite não-frágil).
  - Execução de `tupa run` para `FraudDetection` abaixo de 500ms.
- Observações:
  - Os valores impressos são ilustrativos e variam por máquina.
  - Para medições mais rigorosas, use `hyperfine` com aquecimento (`--warmup`).
  - Preferir Rust stable e builds de release para medições de produto.

## Restrições éticas

```bash
cargo run -p tupa-cli -- check examples/invalid_safe_misinformation.tp
cargo run -p tupa-cli -- check examples/invalid_safe_misinformation_base.tp
```

## Dicas de triagem

- Rode o teste isolado antes da suite completa.
- Verifique se o erro é em parsing ou typecheck.
- Compare spans e mensagens com a saída esperada.
- Reproduza via `tupa-cli -- parse|check`.
