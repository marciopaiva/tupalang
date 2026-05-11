# API del Compilador y Extensibilidad

## Propósito

Explicar cómo usar la API del compilador de Tupã, extender funcionalidad y hacer embedding de Tupã en sistemas Rust.

## Superficie estable de embedding (`v0.9.0`)

La superficie estable de embedding para esta release es:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`
- `tupa-codegen`

Para ejemplos mínimos, ver [Embedding](embedding.md).

## Uso como biblioteca

Cada crate puede usarse como biblioteca Rust independiente:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Puntos de extensión

### Built-in Functions

TupaLang proporciona helpers incorporados accesibles vía el namespace `tupa::`:

- `tupa::weighted(score, weight, reason)` — score ponderado con reason
- `tupa::warn(reason)` — pase con advertencia
- `tupa::pass(reason)` — pase puro con razón
- `tupa::confirm(observed, consecutive, required, reason)` — lógica de confirmación consecutiva
- `tupa::cooldown(active, remaining_seconds, reason)` — bloqueo temporal por cooldown

Estas funciones están registradas en `Runtime::new()` y pueden ser llamadas desde cualquier step del pipeline.

### Custom Extensions

Implementar el trait `TupaExtension` (`tupa-runtime/src/extensions.rs`):

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MisExtensiones;
impl TupaExtension for MisExtensiones {
    fn name(&self) -> &str { "mi_proyecto" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("mi::helper", |input| {
            // lógica customizada
            Ok(input)
        });
    }
}
```

Llamar `MisExtensiones.register(&runtime)` durante la inicialización.

### Plugin System

Carga dinámica de plugins (crate `tupa-plugin`):

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/mi_plugin.so")?;

// Funciones del plugin se invocan con `pm.call("nombre", json!(input))` dentro de los pasos.
```

Los plugins son bibliotecas compartidas que exportan `_tupa_plugin_name` y `_tupa_plugin_register`.

### Schema Registry

Schemas versionados con soporte de migraciones (`tupa-codegen/src/schema_registry.rs`):

```rust
use tupa_codegen::schema_registry::{SchemaRegistry, SchemaVersion};

let mut registry = SchemaRegistry::new();
registry.register_schema(
    "TradingConfig",
    "0.1.0",
    schema,
    migrations,
)?;
```

Los schemas evolucionan entre versiones de pipeline con advertencias de deprecación.

### Hot Reload

Observación de archivos para hot reload (`tupa-runtime/src/hot_reload.rs`):

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// El receptor notifica cambios; llamar a reload_pipeline() para aplicar
```

Habilitado con feature flag:

```bash
cargo add tupa-runtime --features hot-reload
```

## Ejemplo: Agregar un Backend WASM

1. Crear una nueva crate `tupa-backend-wasm`.
2. Implementar el trait `CodegenBackend`.
3. Integrarlo en el CLI.

## Enlaces útiles

- [Embedding](embedding.md)
- [Codegen](codegen.md)
- [Contribución](../../CONTRIBUTING.md)
