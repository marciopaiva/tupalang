# Writing Tupã Plugins in Rust

Plugins allow you to extend Tupã pipelines with custom step functions written in Rust, loaded at runtime without recompiling the main binary.

## Plugin Architecture

A plugin is a dynamic library (`cdylib`) that exports two required symbols:

- `_tupa_plugin_name()` — returns the plugin name as a C string
- `_tupa_plugin_register(registry)` — registers step functions with the engine

The plugin system uses a C ABI for language-agnostic loading (future: Python via `tupa-pyffi`).

---

## Create a Plugin Project

```bash
cargo new my_tupa_plugin --lib
cd my_tupa_plugin
```

Edit `Cargo.toml`:

```toml
[lib]
name = "my_tupa_plugin"
crate-type = ["cdylib"]

[dependencies]
tupa-plugin = "0.10"
serde_json = "1.0"
```

---

## Implement the Plugin

Edit `src/lib.rs`:

```rust
use tupa_plugin::{PluginRegistry, PluginError};
use serde_json::Value;

/// Plugin name — displayed in logs.
#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const u8 {
    // SAFETY: static string lives forever
    b"my_plugin\0".as_ptr()
}

/// Registration function — called once at startup.
#[no_mangle]
pub extern "C" fn _tupa_plugin_register(registry: &mut PluginRegistry) {
    registry
        .register("double", double_step)
        .expect("failed to register 'double'");
}

/// Step function: doubles a numeric input.
pub fn double_step(input: Value) -> Result<Value, PluginError> {
    let num = input.as_f64().ok_or(PluginError::TypeError("expected f64".into()))?;
    Ok(Value::from(num * 2.0))
}
```

### Step Function Signature

All step functions must have the signature:

```rust
pub fn step_name(input: Value) -> Result<Value, PluginError>
```

- `Value` is `serde_json::Value` — dynamic JSON-like data
- Return `Ok(Value)` for success, `Err(PluginError)` for failure
- Use `PluginError::type_error("message")` for type mismatches
- Use `PluginError::runtime("message")` for other errors

---

## Build the Plugin

```bash
cargo build --release --crate-type cdylib
```

Output:

- Linux: `target/release/libmy_tupa_plugin.so`
- macOS: `target/release/libmy_tupa_plugin.dylib`
- Windows: `target/release/my_tupa_plugin.dll`

---

## Load the Plugin in Your Pipeline

In your Tupã project:

```rust
use tupa_plugin::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pm = PluginManager::new();
    pm.load_plugin("./target/release/libmy_tupa_plugin.so")?;

    // Use plugin in a step
    let input = Value::from(42.0);
    let output = pm.call("double", input)?;
    println!("Plugin result: {}", output); // 84.0

    Ok(())
}
```

Within a `pipeline!` macro:

```rust
pipeline! {
    name: WithPlugin,
    input: f64,
    steps: [
        step("double") {
            let ctx = tupa_plugin::context();
            ctx.call("double", Value::from(input))?
        }
    ],
    constraints: []
}
```

(Note: plugin access within `pipeline!` steps may require `tupa_plugin::get()` — check current API.)

---

## Complete Example

See `crates/tupa-plugin/tests/plugin_src/` for working examples:

- `simple_plugin/` — minimal plugin with one step
- `multi_step/` — plugin exposing multiple functions
- `error_handling/` — returning errors gracefully

---

## Debugging

Load errors:

- `PluginNotFound` — path incorrect or file missing
- `SymbolNotFound` — `_tupa_plugin_name` or `_tupa_plugin_register` not exported (check `#[no_mangle]`)
- Version mismatch — plugin compiled against different `tupa-plugin` version

Use `nm` (Linux/macOS) or `dumpbin` (Windows) to verify exported symbols:

```bash
nm target/release/libmy_tupa_plugin.so | grep _tupa_plugin
```

Expected output:

```text
0000000000001234 T _tupa_plugin_name
0000000000001256 T _tupa_plugin_register
```

---

## Best Practices

- Keep step functions pure (no side effects) for predictability
- Validate input types early with `as_f64()`, `as_str()`, etc.
- Return descriptive error messages via `PluginError::runtime()`
- Version your plugin — add `Plugin::version()` if needed (future API)
- Test plugins independently (unit test the step functions directly)

---

## Python Plugins

For Python-based steps, use the `tupa-pyffi` crate (in development). See [Python Plugin Tutorial](./plugin-python.md).

---

## Limitations

- Plugin calls have serialization overhead (JSON Value)
- No shared memory between host and plugin (data copied)
- GIL contention if using Python (future)
- Cannot access Rust types directly — only `Value`

---

## Next Steps

- Explore `tupa-plugin` crate docs: <https://docs.rs/tupa-plugin>
- Contribute your plugin to the ecosystem (publish as `tupa-plugin-*`)
- Read the [Pipeline Guide](../pipeline_guide.md) for integration patterns
