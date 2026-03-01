# Guía de Pruebas

## Propósito

Describir comandos estándar de pruebas y consejos de triage de fallas.

## Comandos principales

```bash
# suite completa
cargo test

# por crate
cargo test -p tupa-lexer
cargo test -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli
```

## Pruebas del CLI

```bash
# salidas golden
cargo test -p tupa-cli -- tests::cli_golden
```

## Pruebas de rendimiento

- Objetivo: verificar tiempo de ejecución para ejemplos medianos (objetivo < 200ms).
- Cómo ejecutar con logs:
  - `cargo test -p tupa-cli perf -- --nocapture`
- Qué se verifica:
  - Codegen do exemplo `fraud_complete` abaixo de 500ms (limite não-frágil).
  - Execução de `tupa run` para `FraudDetection` abaixo de 500ms.
- Observaciones:
  - Los valores impresos son ilustrativos y varían por máquina.
  - Para mediciones más rigurosas, usa `hyperfine` con calentamiento (`--warmup`).
  - Prefiere Rust stable y builds de release para mediciones de producto.

## Restricciones éticas

```bash
cargo run -p tupa-cli -- check examples/invalid_safe_misinformation.tp
cargo run -p tupa-cli -- check examples/invalid_safe_misinformation_base.tp
```

## Consejos de triage

- Ejecuta la prueba aislada antes de la suite completa.
- Verifica si el error está en parsing o typecheck.
- Compara spans y mensajes con la salida esperada.
- Reproduce vía `tupa-cli -- parse|check`.
