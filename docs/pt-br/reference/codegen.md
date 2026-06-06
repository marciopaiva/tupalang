# Codegen (descontinuado)

> **Removido na 0.9.0.** O backend de codegen `.tp` (`tupa-codegen`, que emitia IR
> textual no estilo LLVM e planos JSON a partir de arquivos `.tp`) foi removido
> junto com o toolchain `.tp` standalone.

## Equivalente atual

Na arquitetura baseada em crates, os pipelines são escritos com a macro
`pipeline!` e o `rustc` gera o código. Para inspecionar o código Rust gerado pela
macro, use `cargo tupa expand`:

```bash
cargo tupa expand                       # expande pipeline! no pacote atual
cargo tupa expand --file src/my_pipeline.rs
```

A execução é feita com `tupa_engine::Executor` (ou `cargo tupa run --input
data.json`), não com um binário de codegen separado. Veja
[cargo_tupa_guide.md](../guides/cargo_tupa_guide.md).
