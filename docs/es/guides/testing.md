# Guía de Pruebas

## Propósito

Comandos de prueba estándar y consejos de triaje de fallos para Tupã 0.9.x (era Rust-DSL).

---

## Comandos principales

```bash
# Suite completa del workspace
cargo test --workspace --locked

# Por crate (solo crates activos)
cargo test -p tupa-core
cargo test -p tupa-core-macros
cargo test -p tupa-engine
cargo test -p tupa-plugin
cargo test -p tupa-pyffi
cargo test -p cargo-tupa
```

---

## Pruebas de cargo-tupa

```bash
# Pruebas unitarias de los subcomandos del CLI
cargo test -p cargo-tupa

# Prueba de integración (salida de métricas)
cargo test -p cargo-tupa --test run_metrics
```

---

## Benchmarks de rendimiento

La suite de benchmarks de `tupa-engine` (con `criterion`) se ejecuta con:

```bash
cargo bench -p tupa-engine
```

Para mediciones rigurosas use builds de release y `hyperfine` con calentamiento.

---

## Consejos de triaje

- Ejecute la prueba aislada antes de la suite completa.
- Distinga errores de compilación (`rustc` / macro `pipeline!`) de errores de
  ejecución (`Executor::run` devuelve `PipelineResult`).
- Compare mensajes y códigos de diagnóstico con la salida esperada.
