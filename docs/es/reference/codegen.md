# Codegen (descontinuado)

> **Eliminado en 0.9.0.** El backend de codegen `.tp` (`tupa-codegen`, que emitía
> IR textual tipo LLVM y planes JSON desde archivos `.tp`) fue eliminado junto con
> el toolchain `.tp` independiente.

## Equivalente actual

En la arquitectura basada en crates, los pipelines se escriben con la macro
`pipeline!` y `rustc` genera el código. Para inspeccionar el código Rust generado
por la macro, use `cargo tupa expand`:

```bash
cargo tupa expand                       # expande pipeline! en el paquete actual
cargo tupa expand --file src/my_pipeline.rs
```

La ejecución se realiza con `tupa_engine::Executor` (o `cargo tupa run --input
data.json`), no con un binario de codegen separado. Vea
[cargo_tupa_guide.md](../guides/cargo_tupa_guide.md).
