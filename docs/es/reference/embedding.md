# Embedding de Tupã en Rust

## Propósito

Describir la superficie soportada de embedding para `v0.8.2`.

## Crates públicas soportadas

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`
- `tupa-codegen`

Estas crates son la superficie estable de embedding para esta release.

## API de Extensiones

Los proyectos pueden definir funciones de paso personalizadas a través del trait `TupaExtension`:

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MisExtensiones;
impl TupaExtension for MisExtensiones {
    fn name(&self) -> &str { "mi_proyecto" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("mi::helper", |input| {
            // lógica de negocio
            Ok(serde_json::json!({ "status": "ok" }))
        });
    }
}

// Durante la inicialización
MisExtensiones.register(&runtime);
```

## Sistema de Plugins

Carga dinámica de plugins (`tupa-plugin`):

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/mi_plugin.so")?;

// En un paso del pipeline, llamar a una función del plugin:
// pm.call("mi_step", json!(input))?
```

Los plugins son bibliotecas compartidas que exportan `_tupa_plugin_name` y `_tupa_plugin_register`.

## Hot Reload

Habilitar feature `hot-reload` para observar cambios en archivos:

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// Notifica cambios automáticamente
```

## Ejemplo mínimo

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

## Notas de compatibilidad

- Sigue SemVer según [Versionado](versioning.md).
- Evita depender de crates internas no listadas arriba si necesitas estabilidad de API.
