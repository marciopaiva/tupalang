# tupa-plugin

**Tupã dynamic plugin loader** — load custom step functions from shared libraries at runtime.

## Purpose

Enables pipeline steps to call functions implemented in dynamically loaded shared libraries (`.so`, `.dll`, `.dylib`). Plugin functions follow the C ABI and can be written in any language that can export C symbols.

**Status:** Alpha (0.9.x). API subject to change before 1.0.

## Dependencies

- `tupa-core` — types and pipeline definitions
- `tupa-engine` — execution engine (no direct dependency on plugin, but pipelines call plugin functions)

## Usage

### Load a plugin

```rust
use tupa_plugin::PluginManager;
use serde_json::json;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/my_strategy_plugin.so")?;
```

### Call a plugin function from a step

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use tupa_plugin::PluginManager;
use serde::{Serialize, json};

#[derive(Debug, Clone, Serialize)]
struct Input { /* ... */ }

// Wrap plugin call in a function the pipeline can use
fn my_plugin_step(pm: &PluginManager, input: &Input) -> Result<serde_json::Value, String> {
    pm.call("my_step", json!(input)).map_err(|e| e.to_string())
}

pipeline! {
    name: MyPipeline,
    input: Input,
    steps: [
        step("plugin") { my_plugin_step(&pm, input)? }
    ],
    constraints: []
}
```

You can also capture `pm` directly in the step expression if it's in scope:

```rust
let pm = PluginManager::new();
pm.load_plugin("./plugin.so")?;

pipeline! {
    steps: [
        step("my_plugin_step") {
            pm.call("my_step", json!(input))?
        }
    ]
}
```

### Plugin development

Get a template:

```rust
let template = tupa_plugin::create_plugin_template();
std::fs::write("my_plugin.rs", template)?;
```

A plugin must export two C symbols:

- `_tupa_plugin_name()` — returns a C string with the plugin name
- `_tupa_plugin_register(ctx)` — registers step functions with the loader

Each step function must have the exact signature:

```rust
#[no_mangle]
pub extern "C" fn my_step(input: serde_json::Value) -> serde_json::Value {
    // compute and return a value
}
```

## API

- `PluginManager::new()` — create an empty manager
- `PluginManager::load_plugin<P: AsRef<Path>>(&mut self, path: P) -> Result<&Plugin, PluginError>` — load a shared library; returns the loaded plugin
- `PluginManager::call(&self, name: &str, input: serde_json::Value) -> Result<serde_json::Value, PluginError>` — invoke a loaded plugin function by name
- `PluginManager::list_functions(&self) -> Vec<(String, Vec<String>)>` — enumerate all functions across loaded plugins

`Plugin` struct: `name`, `library`, `functions`.

## Notes

- Libraries are loaded via `libloading` and remain loaded for the lifetime of the `PluginManager`.
- `call()` performs a dynamic symbol lookup on each invocation. Cache function pointers manually for hot paths (unsafe).
- Calls use the C ABI; ensure the plugin's function signature matches `fn(serde_json::Value) -> serde_json::Value`.

## Crates

- Source: https://github.com/marciopaiva/tupalang
- License: Apache-2.0
- Docs: https://docs.rs/tupa-plugin
