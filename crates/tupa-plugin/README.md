# tupa-plugin

**Tupã dynamic plugin loader** — load custom step functions from shared libraries at runtime.

## Overview

Enables pipeline steps to call functions implemented in dynamically loaded shared libraries (`.so`, `.dll`, `.dylib`). Plugin functions follow the C ABI and can be written in any language that can export C symbols.

**Status:** Alpha (0.9.x). API subject to change before 1.0.

## Installation

```toml
[dependencies]
tupa-plugin = "0.9"
```

## Quick Example

```rust
use tupa_plugin::PluginManager;
use serde_json::json;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/my_plugin.so")?;

// Call a function from the loaded plugin
let result = pm.call("my_step", json!({"value": 42}))?;
```

## Usage in Pipeline

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use tupa_plugin::PluginManager;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Input { /* ... */ }

let pm = PluginManager::new();
pm.load_plugin("./plugins/my_plugin.so")?;

pipeline! {
    name: MyPipeline,
    input: Input,
    steps: [
        step("plugin_step") {
            pm.call("my_step", json!(&input))?
        }
    ],
    constraints: []
}
```

## Plugin Development

Plugins must export two C symbols:

```rust
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const std::os::raw::c_char {
    CString::new("my_plugin").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn my_step(input: serde_json::Value) -> serde_json::Value {
    // compute and return a value
    json!({"result": 42})
}
```

## API

- `PluginManager::new()` — create empty manager
- `PluginManager::load_plugin<P: AsRef<Path>>(&mut self, path: P)` — load shared library
- `PluginManager::call(&self, name: &str, input: serde_json::Value)` — invoke plugin function
- `PluginManager::list_functions(&self)` — enumerate all loaded functions

## Note

Function signatures must match `fn(serde_json::Value) -> serde_json::Value` for the C ABI.

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-plugin)
