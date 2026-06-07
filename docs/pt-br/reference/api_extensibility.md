# API e Extensibilidade

> **Atualizado para 0.9.x.** Os crates `tupa-parser`, `tupa-typecheck`,
> `tupa-codegen` e `tupa-runtime`, junto com o trait `TupaExtension`, o Schema
> Registry e o Hot Reload, foram **removidos** na 0.9.0.

## Extensão na arquitetura atual

- **Step functions:** funções Rust comuns chamadas do corpo de um `step` na macro
  `pipeline!`.
- **Plugins dinâmicos:** carga em tempo de execução via `tupa-plugin`
  (`PluginManager`); os plugins exportam `_tupa_plugin_name` e
  `_tupa_plugin_register`. Veja o README do crate `tupa-plugin`.
- **Python:** via `tupa-pyffi` (veja [python_ffi_spec.md](python_ffi_spec.md)).
- **Embedding:** veja [embedding.md](embedding.md).
