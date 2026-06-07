# Python FFI

> **Actualizado para 0.9.x.** El antiguo contrato FFI `.tp` `v0.8.2` (con sintaxis
> `@external(python=...)` y el diseño `tupa-parser` / `tupa-typecheck`) es obsoleto.
> La integración con Python ahora vive en el crate **`tupa-pyffi`**.

Llama funciones de Python desde un step vía `tupa-pyffi`:

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

let result = call_python_function("math", "sqrt", json!(16.0))?;
assert_eq!(result, json!(4.0));
```

Requiere los headers de desarrollo de Python instalados. Ver el
[README del crate `tupa-pyffi`](https://docs.rs/tupa-pyffi) y
[features/trading_support.md](../features/trading_support.md) para uso dentro de
un pipeline.
