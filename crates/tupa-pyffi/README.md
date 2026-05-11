# tupa-pyffi

Python FFI bindings for Tupã — call Python functions from Rust pipelines.

## Purpose

Enables interoperability with Python libraries (NumPy, PyTorch, TensorFlow) by allowing Tupã pipelines to invoke Python functions as steps.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
tupa-pyffi = "0.9.0"
```

In your pipeline step:

```rust
use tupa_pyffi::call_python_function;

step("python_step") {
    let input = serde_json::json!(42);
    match call_python_function("math", "sqrt", input) {
        Ok(result) => result,
        Err(e) => panic!("Python call failed: {}", e),
    }
}
```

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
