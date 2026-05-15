# Escribiendo Plugins Tupã en Rust

Los plugins extienden pipelines Tupã con funciones de step personalizadas en Rust, cargadas en runtime sin recompilar el binario.

## Arquitectura

Un plugin es una biblioteca dinámica (`cdylib`) que exporta dos símbolos obligatorios:

- `_tupa_plugin_name()` — retorna el nombre del plugin como string C
- `_tupa_plugin_register(registry)` — registra funciones de step en el engine

Se usa ABI C para carga independiente de lenguaje.

---

## Crear Proyecto

```bash
cargo new mi_plugin_tupa --lib
cd mi_plugin_tupa
```

`Cargo.toml`:

```toml
[lib]
name = "mi_plugin_tupa"
crate-type = ["cdylib"]

[dependencies]
tupa-plugin = "0.9"
serde_json = "1.0"
```

---

## Implementar

`src/lib.rs`:

```rust
use tupa_plugin::{PluginRegistry, PluginError};
use serde_json::Value;
```

### Firma de Step Function

```rust
pub fn nombre_step(input: Value) -> Result<Value, PluginError>
```

- `Value` = `serde_json::Value` (datos JSON dinámicos)
- `Ok(Value)` éxito, `Err(PluginError)` error

---

## Build

```bash
cargo build --release --crate-type cdylib
```

Salida:

- Linux: `target/release/libmi_plugin_tupa.so`
- macOS: `target/release/libmi_plugin_tupa.dylib`
- Windows: `target/release/mi_plugin_tupa.dll`

---

## Cargar en Rust

```rust
use tupa_plugin::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pm = PluginManager::new();
    pm.load_plugin("./target/release/libmi_plugin_tupa.so")?;

    let result = pm.call("doblar", serde_json::json!(21.0))?;
    println!("Resultado: {}", result); // 42.0

    Ok(())
}
```

Dentro de `pipeline!`:

```rust
pipeline! {
    name: ConPlugin,
    input: f64,
    steps: [
        step("doblar") {
            let ctx = tupa_plugin::context();
            ctx.call("doblar", Value::from(input))?
        }
    ],
    constraints: []
}
```

---

## Ejemplo Completo

Ver `crates/tupa-plugin/tests/plugin_src/` para ejemplos.

---

## Depuración

Verificar símbolos:

```bash
nm target/release/libmi_plugin_tupa.so | grep _tupa_plugin
```

Errores comunes:

- `PluginNotFound` — ruta incorrecta
- `SymbolNotFound` — falta `#[no_mangle]` o nombre mal
- Versión incompatible — recompile con misma versión `tupa-plugin`

---

## Buenas Prácticas

- Steps puros (sin side effects)
- Valide tipos al inicio
- Erros descriptivos
- Teste funções unitariamente
- Rust para paths críticos; Python solo para ML/I/O

---

## Plugins Python

Ver [Plugin Python](./plugin-python.md) (usa `tupa-pyffi`).

---

## Próximos Pasos

- Docs de `tupa-plugin`: <https://docs.rs/tupa-plugin>
- [Pipeline Guide](../guides/pipeline_guide.md)
- Contribuya su plugin al ecosistema!
