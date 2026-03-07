# tupa-pyffi

Python FFI bridge for calling external Python functions from Tupa runtime.

## Usage

```rust
use serde_json::json;
use tupa_pyffi::call_python_function;

let _ = call_python_function("module", "func", json!({"x": 1}));
```

## Notes

- Requires Python runtime/toolchain in build or runtime environment.
