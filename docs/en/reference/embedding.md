# Embedding Tupã in Rust

## Purpose

Describe the supported embedding surface for `v0.8.2`.

## Supported Public Crates

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`
- `tupa-codegen`

These crates are the stable embedding surface for this release.

## Extension API

Projects can define custom step functions via the `TupaExtension` trait:

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MyExtensions;
impl TupaExtension for MyExtensions {
    fn name(&self) -> &str { "my_project" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("my::helper", |input| {
            // business logic
            Ok(serde_json::json!({ "status": "ok" }))
        });
    }
}

// During startup
MyExtensions.register(&runtime);
```

## Plugin System

Dynamic plugin loading (`tupa-plugin` crate):

```rust
use tupa_plugin::PluginManager;

let mut manager = PluginManager::new();
manager.load_plugin("./plugins/my_plugin.so")?;
manager.register_all(&runtime);
```

Plugins are shared libraries exporting `_tupa_plugin_name` and `_tupa_plugin_register` C symbols.

## Minimal Example

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;

fn main() -> anyhow::Result<()> {
    let src = "fn main() { print(1) }";
    let ast = parse(src)?;
    let _typed = typecheck(&ast)?;
    Ok(())
}
```

## Hot Reload

Enable the `hot-reload` feature to watch for file changes:

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// Automatically reloads when `.tp` files change
```

## Compatibility Notes

- Follow SemVer constraints from [Versioning](versioning.md).
- Avoid depending on internal crates not listed above if you need API stability.
