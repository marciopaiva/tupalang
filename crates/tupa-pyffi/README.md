# tupa-pyffi

Python FFI bridge for calling external Python functions from Tupa runtime.

## Usage

```rust
use serde_json::json;
use tupa_pyffi::call_python_function;

let result = call_python_function("math", "sqrt", json!(16.0))?;
assert_eq!(result, json!(4.0));
# Ok::<(), String>(())
```

## Notes

- Requires Python runtime/toolchain in build or runtime environment.
- Calls a Python function with a single JSON-like argument and converts the return value back to `serde_json::Value`.
- The target module must be importable from the active Python environment.

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade is the main applied integration reference for embedded Tupa pipelines; `tupa-pyffi` is available when those pipelines need Python-backed enrichment or interoperability.
