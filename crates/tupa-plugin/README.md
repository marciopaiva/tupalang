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

let mut pm = PluginManager::new();
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

Build the plugin as a `cdylib` and export two C symbols: `_tupa_plugin_name`
(returns the plugin name) and `_tupa_plugin_register` (registers each step
function with the host). Step functions use the `extern "C" fn(Value) -> Value`
ABI. Generate a ready-to-edit scaffold with `tupa_plugin::create_plugin_template()`
or `cargo tupa plugin new`:

```rust
// Build as: cargo build --crate-type=cdylib
use serde_json::Value;
use tupa_plugin::PluginRegisterContext;

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const i8 {
    static NAME: &str = "my_plugin";
    NAME.as_ptr() as *const i8
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: *mut PluginRegisterContext) {
    unsafe {
        let name = b"my_step\0".as_ptr() as *const i8;
        let func: extern "C" fn(Value) -> Value = my_step;
        (*ctx).register_step.unwrap()(name, func as *const () as *const u8);
        (*ctx).functions.push("my_step".to_string());
    }
}

#[no_mangle]
pub extern "C" fn my_step(input: Value) -> Value {
    // Transform input or return a computed value
    input
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
