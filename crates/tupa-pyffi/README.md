# tupa-pyffi

Python FFI bindings for Tupã — call Python functions from Rust pipelines.

## Overview

Enables interoperability with Python libraries (NumPy, PyTorch, TensorFlow) by allowing Tupã pipelines to invoke Python functions as steps.

**Status:** Alpha (0.9.x). API may change before 1.0.

## Installation

```toml
[dependencies]
tupa-pyffi = "0.9"
```

**Note:** Requires Python development headers installed on the system.

## Quick Example

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

// Call math.sqrt from Python
let result = call_python_function("math", "sqrt", json!(16.0))?;
assert_eq!(result, 4.0);
```

## Usage in Pipeline

```rust
use tupa_core::pipeline;
use serde_json::json;

fn python_step(input: &MyInput) -> Result<serde_json::Value, String> {
    call_python_function("my_module", "process", json!(input))
        .map_err(|e| e.to_string())
}

pipeline! {
    name: MyPipeline,
    input: MyInput,
    steps: [
        step("python") { python_step(&input)? }
    ],
    constraints: []
}
```

## Multi-Argument Calls

```rust
use tupa_pyffi::call_with_multiple_args;

let args = vec![
    json!(10.0),
    json!(20.0),
];
let result = call_with_multiple_args("my_module", "add", &args)?;
```

## Reset Global State

```rust
use tupa_pyffi::reset_python_bridge;

// Reset Python interpreter state (useful for testing)
reset_python_bridge();
```

## Supported Types

- `i32`, `u64`, `u32`, `f32` — numeric primitives
- `Vec<u8>` — byte arrays
- `Vec<Value>` — JSON arrays
- `serde_json::Value` — arbitrary JSON

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-pyffi)
