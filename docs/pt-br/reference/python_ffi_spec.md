# Python FFI

> **Atualizado para 0.9.x.** O antigo contrato FFI `.tp` `v0.8.2` (com sintaxe
> `@external(python=...)` e o design `tupa-parser` / `tupa-typecheck`) está
> obsoleto. A integração com Python agora vive no crate **`tupa-pyffi`**.

Chame funções Python a partir de um step via `tupa-pyffi`:

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

let result = call_python_function("math", "sqrt", json!(16.0))?;
assert_eq!(result, json!(4.0));
```

Requer os headers de desenvolvimento do Python instalados. Veja o
[README do crate `tupa-pyffi`](https://docs.rs/tupa-pyffi) e
[features/trading_support.md](../features/trading_support.md) para uso dentro de
um pipeline.
