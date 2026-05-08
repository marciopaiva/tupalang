# tupa-plugin

Dynamic plugin loading for TupaLang step functions.

## Purpose

Load custom step functions from shared libraries (`.so`/`.dll`) at runtime without recompiling the pipeline.

## Usage

```rust
use tupa_plugin::{PluginManager, PluginError};

let mut manager = PluginManager::new();

// Load a plugin from a shared library
manager.load_plugin("./plugins/my_strategy_plugin.so")?;

// List available functions
for (plugin, functions) in manager.list_functions() {
    println!("Plugin: {}", plugin);
    for func in functions {
        println!("  - {}", func);
    }
}

// Register all plugin functions with the runtime
manager.register_all(&runtime);
```

## Plugin Development

Create a shared library that exports two C-compatible symbols:

### 1. Plugin Name

```rust
#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const i8 {
    CString::new("my_plugin").unwrap().into_raw()
}
```

### 2. Registration Function

```rust
#[no_mangle]
pub extern "C" fn _tupa_plugin_register(ctx: &mut PluginRegisterContext) {
    // Register step functions
    unsafe {
        (ctx.register_step.unwrap())(
            CString::new("my_plugin::custom_step").unwrap().as_ptr(),
            my_step_function_ptr as *const u8,
        );
    }

    // Track function names for listing
    ctx.functions.push("my_plugin::custom_step".to_string());
}
```

### 3. Step Function Signature

```rust
pub fn my_step_function(input: serde_json::Value) -> Result<serde_json::Value, String> {
    // Your logic here
    Ok(serde_json::json!({ "result": "ok" }))
}
```

### Complete Example

See `tupa-plugin/tests/plugin_example/` for a minimal working plugin.

## Build

```bash
# Build the example plugin
cargo build -p tupa-plugin --example test_plugin --release

# The plugin will be at:
# target/release/libtest_plugin.so
```

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied Usage

- ViperTrade can ship complex trailing logic and risk checks as plugins.
- Plugins are loaded at strategy startup and registered as pipeline steps.
