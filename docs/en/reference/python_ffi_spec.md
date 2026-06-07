# Python FFI

> **Updated for 0.9.x.** The earlier `v0.8.2` `.tp` FFI contract (with
> `@external(python=...)` syntax and the `tupa-parser` / `tupa-typecheck` design)
> is obsolete. Python integration now lives in the **`tupa-pyffi`** crate.

Call Python functions from a step via `tupa-pyffi`:

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

let result = call_python_function("math", "sqrt", json!(16.0))?;
assert_eq!(result, json!(4.0));
```

Requires Python development headers installed. See the
[`tupa-pyffi` crate README](https://docs.rs/tupa-pyffi) and
[features/trading_support.md](../features/trading_support.md) for usage inside a
pipeline.
