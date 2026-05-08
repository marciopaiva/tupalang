# Compiler API and Extensibility

## Purpose

This document explains how to use Tupã's compiler API, extend functionality, and embed Tupã in Rust systems.

## Stable Embedding Surface (`v0.8.2`)

The stable embedding surface for this release is:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`
- `tupa-codegen`

For minimal embedding examples, see [Embedding](embedding.md).

## Extension Points

### Built-in Functions

TupaLang provides built-in helpers accessible via the `tupa::` namespace:

- `tupa::weighted(score, weight, reason)` — weighted score aggregation
- `tupa::warn(reason)` — pass with warning severity
- `tupa::pass(reason)` — informational pass
- `tupa::confirm(observed, consecutive, required, reason)` — consecutive confirmation logic
- `tupa::cooldown(active, remaining_seconds, reason)` — temporal block remaining

These are registered in `Runtime::new()` and can be called from any pipeline step.

### Custom Extensions

Implement the `TupaExtension` trait (`tupa-runtime/src/extensions.rs`):

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MyExtensions;
impl TupaExtension for MyExtensions {
    fn name(&self) -> &str { "my_project" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("my::helper", |input| {
            // custom logic
            Ok(input)
        });
    }
}
```

Call `MyExtensions.register(&runtime)` during initialization.

### Plugin System

Dynamic loading of step functions from shared libraries:

```rust
use tupa_plugin::PluginManager;

let mut manager = PluginManager::new();
manager.load_plugin("./plugins/my_plugin.so")?;
manager.register_all(&runtime);
```

Plugins export `_tupa_plugin_name` and `_tupa_plugin_register` entry points.

### Schema Registry

Versioned schemas with migration support (`tupa-codegen/src/schema_registry.rs`):

```rust
use tupa_codegen::schema_registry::{SchemaRegistry, SchemaVersion};

let mut registry = SchemaRegistry::new();
registry.register_schema("StrategyConfig", "0.1.0", schema, migrations)?;
```

Warns on deprecated fields and supports forward compatibility.

### Hot Reload

File watching for pipeline hot reload (`tupa-runtime/src/hot_reload.rs`):

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// On file change, runtime reloads pipeline plan automatically
```

Enable with `--features hot-reload`.

## Example: Adding a WASM Backend

1. Create a new crate `tupa-backend-wasm`.
2. Implement the `CodegenBackend` trait.
3. Integrate it into the CLI.

## Library Usage

Each crate can be used as an independent Rust library:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## New types

Implement and register in `tupa-typecheck`. New diagnostics are added per-crate in error modules. New backends implement the `CodegenBackend` trait. Custom CLI commands extend `tupa-cli`.

## Applied Usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-runtime` as an embedded execution engine inside the strategy and AI analyst services, rather than spawning `tupa run` per event.

## Useful Links

- [Embedding](embedding.md)
- [Codegen](codegen.md)
- [Contribution](../../CONTRIBUTING.md)
