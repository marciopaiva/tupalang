# Guía de Migración: de `.tp` a Rust-DSL

**Estado:** La cadena de herramientas `.tp` heredada fue **eliminada** en la v0.9.0. El desarrollo activo es exclusivamente Rust-DSL.

El lenguaje `.tp` independiente y su compilador (`tupa-cli`, `tupa-parser`, `tupa-typecheck`, etc.) fueron eliminados permanentemente del workspace. Todo el desarrollo nuevo usa la macro `pipeline!` en archivos fuente Rust.

Esta guía le ayuda a migrar sus pipelines `.tp` existentes al Rust DSL.

---

## ¿Por qué migrar?

- ✅ Sin cadena de herramientas separada — usa `cargo` y `rustc` directamente
- ✅ Soporte completo de IDE (rust-analyzer funciona de inmediato)
- ✅ Mejores mensajes de error (diagnósticos de rustc con spans)
- ✅ Acceso al ecosistema Rust (crates, macros, traits)
- ✅ Iteración más rápida — sin depuración de límites entre lenguajes
- ✅ Listo para producción — ViperTrade usa Rust DSL exclusivamente

---

## Lista de verificación de migración

Para cada archivo `.tp` en su proyecto:

- [ ] Crear un módulo `.rs` con la macro `pipeline!`
- [ ] Convertir funciones de paso a funciones Rust
- [ ] Convertir la definición de pipeline a la sintaxis Rust DSL
- [ ] Actualizar `Cargo.toml` con `tupa-core` y `tupa-engine`
- [ ] Eliminar el archivo `.tp`
- [ ] Ejecutar `cargo tupa check` para validar
- [ ] Ejecutar `cargo tupa run` para probar la ejecución

---

## Migración paso a paso

### 1. Identificar archivos heredados

```bash
find . -name "*.tp"
```

Salida de ejemplo:

```text
strategies/risk_limits.tp
strategies/position_sizing.tp
```

### 2. Crear el esqueleto del módulo Rust

Para `strategies/risk_limits.tp`, cree `strategies/risk_limits.rs`:

```rust
use tupa_core::{pipeline, step, constraint, metric};

pipeline! {
    name: RiskLimits,           // igual que el nombre del pipeline .tp
    input: Trade,               // reemplace con su tipo de entrada
    steps: [
        // TODO: convertir cada cuerpo de paso
    ],
    constraints: [
        // TODO: convertir cada restricción
    ]
}
```

### 3. Convertir funciones de paso

Copie el cuerpo de cada función de paso de `.tp` a Rust. Ajuste la sintaxis:

| Sintaxis `.tp` | Equivalente Rust DSL |
|------------|----------------------|
| `fn score(s: Signal): i64` | `fn score(s: &Signal) -> i64` |
| `fn validate(t: Trade): bool` | `fn validate(t: &Trade) -> bool` |
| `let x = 42` | igual |
| `match s { Buy => 100 }` | igual (sintaxis de patrones Rust) |
| `if cond { a } else { b }` | igual |

**Cambios importantes:**

- Anotación de tipo de retorno: `: T` → `-> T`
- Los parámetros se pasan por referencia (`&T`) para evitar clones (la entrada es `&Input`)
- Sin retornos implícitos — la última expresión se devuelve (igual que Rust)

**Ejemplo de conversión:**

```tupa
// .tp
fn compute_risk(trade: Trade): f64 {
    trade.size * trade.price / 1_000_000.0
}
```

↓

```rust
// .rs
fn compute_risk(trade: &Trade) -> f64 {
    trade.size * trade.price / 1_000_000.0
}
```

### 4. Convertir la definición de pipeline

La macro `pipeline!` usa una sintaxis casi idéntica a `.tp`:

| Constructo | `.tp` | Rust DSL |
|-----------|-------|----------|
| Inicio de pipeline | `pipeline MyPolicy {` | `pipeline! { name: MyPolicy,` |
| Tipo de entrada | `input: Trade` | `input: Trade,` (idéntico) |
| Bloque de paso | `step("name") { expr }` | `step("name") { expr }` (idéntico) |
| Restricción | `metric("x").ge(10)` | `metric("x").ge(10)` (idéntico) |
| Fin de pipeline | `}` | `}` (la coma después del último elemento es opcional) |

**Ejemplo completo:**

`.tp`:

```tupa
pipeline PreTradeCheck {
    input: Trade,
    steps: [
        step("risk") { compute_risk(input) },
        step("limit") { input.size <= 1_000_000.0 }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0),
        metric("max_leverage").le(2.0)
    ]
}
```

Rust DSL:

```rust
pipeline! {
    name: PreTradeCheck,
    input: Trade,
    steps: [
        step("risk") { compute_risk(input) },
        step("limit") { input.size <= 1_000_000.0 }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0),
        metric("max_leverage").le(2.0)
    ]
}
```

### 5. Actualizar Cargo.toml

Agregar las dependencias:

```toml
[dependencies]
tupa-core = "0.10"
tupa-engine = "0.10"
```

Si usó plugins en `.tp`, agregue:

```toml
tupa-plugin = "0.10"
```

### 6. Crear el binario principal (si aún no existe)

Su paquete necesita un binario que ejecute el pipeline:

```rust
// src/main.rs
use your_crate::{YourPipeline, Trade};  // ajustar importaciones
use tupa_engine::Executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = YourPipeline::new();
    let engine = Executor::new();

    // Construir la entrada (de JSON, configuración, etc.)
    let input = Trade {
        symbol: "AAPL".into(),
        size: 1_000_000.0,
        price: 170.0,
    };

    let result = engine.run(pipeline, &input)?;
    println!("Todas las restricciones pasaron: {}", result.passed);
    Ok(())
}
```

### 7. Validar

```bash
# Verificar tipos del pipeline
cargo tupa check

# Ejecutar el pipeline
cargo tupa run

# Ejecutar pruebas (si las hay)
cargo test
```

Si `cargo tupa check` pasa, el pipeline es sintáctica y típicamente correcto.

### 8. Eliminar el archivo `.tp`

Una vez validado, elimine el archivo heredado:

```bash
rm strategies/risk_limits.tp
```

---

## Resumen de diferencias de sintaxis

| Característica | `.tp` | Rust DSL | Notas |
|---------|-------|----------|-------|
| Extensión de archivo | `.tp` | `.rs` | Fuente Rust estándar |
| Firma de función | `fn nombre(args): TipoRetorno` | `fn nombre(args) -> TipoRetorno` | Rust usa `->` |
| Expresión de paso | `{ expr }` | `{ expr }` | idéntico |
| Bloque de restricciones | `constraints: [ ... ]` | `constraints: [ ... ]` | idéntico |
| Acceso a métricas | `metric("nombre")` | `metric("nombre")` | idéntico |
| Nombres de tipo | incorporados (`i64`, `f64`) | tipos estándar Rust (`i64`, `f64`) | idéntico |
| Tuplas | `(i32, i64)` | `(i32, i64)` | idéntico |
| Enums | `enum Lado { Compra, Venta }` | `enum Lado { Compra, Venta }` | idéntico |
| Structs | `struct Operacion { ... }` | `struct Operacion { ... }` | idéntico |
| Coincidencia de patrones | `match x { ... }` | `match x { ... }` | idéntico |
| Comentarios | `//` o `/* */` | `//` o `/* */` | idéntico |

**No hay diferencias semánticas** — el DSL dentro de `pipeline!` está diseñado para ser casi idéntico a `.tp`. El cambio principal es que está incrustado en Rust y verificado por `rustc` en lugar de por un verificador de tipos separado.

---

## Problemas comunes de migración

### Problema: "no se puede encontrar la macro `pipeline`"

**Causa:** Falta `use tupa_core::pipeline;` o `tupa-core` no está en `Cargo.toml`.

**Solución:**

```toml
[dependencies]
tupa-core = "0.10"
```

```rust
use tupa_core::pipeline;
```

---

### Problema: Función de paso no encontrada en el ámbito

**Causa:** Función de paso definida después de la macro `pipeline!` o no es `pub` si está en otro módulo.

**Solución:** Asegúrese de que las funciones de paso estén declaradas `pub` si son entre módulos, o reordene las definiciones (las funciones deben estar en el ámbito antes de invocar la macro).

---

### Problema: La restricción no se puede demostrar en tiempo de compilación

**Causa:** En `.tp` algunas restricciones eran demostradas por el antiguo verificador de tipos; la demostración del Rust DSL es más limitada.

**Solución:** Esto es un **aviso**, no un error. La restricción se verificará en tiempo de ejecución. Si necesita demostración en tiempo de compilación, simplifique la expresión a plegado constante.

---

### Problema: Faltan funciones de biblioteca estándar `.tp`

**Causa:** `.tp` tenía funciones incorporadas como `abs`, `max`, `min` que pueden no existir en el Rust DSL.

**Solución:** Use los equivalentes de la biblioteca estándar de Rust (`f64::abs`, `f64::max`, etc.) o defina sus propias funciones auxiliares.

---

### Problema: Las funciones de paso de plugin (Python) ya no funcionan

**Causa:** El sistema de plugins `.tp` era diferente.

**Solución:** Reescriba los plugins como plugins Rust (crate `tupa-plugin`) o use `tupa-pyffi` para la integración con Python. Vea [Tutorial de Plugins](../tutorials/plugin-rust.md).

---

## Estrategia de validación

Después de migrar un pipeline:

1. **Verificación en tiempo de compilación:**

   ```bash
   cargo tupa check
   ```

   No debe emitir errores.

2. **Pruebas unitarias:** Pruebe funciones de paso individuales con `#[test]`.

3. **Prueba de integración:** Ejecute el pipeline completo con una entrada representativa:

   ```rust
   #[tokio::test]
   async fn test_pipeline_migrado() {
       let pipeline = TuPipeline::new();
       let engine = Executor::new();
       let entrada = construir_entrada_de_prueba();
       let resultado = engine.run(pipeline, &entrada).unwrap();
       assert!(resultado.pasado);
   }
   ```

4. **Comparar salidas:** Si tiene la salida del pipeline `.tp` anterior guardada, compare los resultados para garantizar la equivalencia semántica.

---

## ¿Necesita ayuda?

- Abra un issue: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)
- Vea [Guía de Pipeline](../guides/pipeline_guide.md) para patrones avanzados
- Explore ejemplos en `crates/tupa-engine/examples/`

---

## Migración completada

Una vez que todos los archivos `.tp` heredados estén convertidos:

- Elimine las referencias restantes a `tupa-cli` o `.tp` en sus scripts de compilación
- Actualice su documentación para reflejar el uso de Rust-DSL
- ¡Considere contribuir su experiencia de migración al proyecto!
