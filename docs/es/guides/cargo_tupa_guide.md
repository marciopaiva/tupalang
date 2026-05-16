# cargo-tupa

Subcomando de Cargo para desarrollo de políticas Tupã.

## Instalación

```bash
cargo install cargo-tupa
```text

## Comandos

### `cargo tupa check`

Valida la expansión del macro `pipeline!` y la verificación de tipos. Ejecuta `cargo check` y filtra mensajes relevantes de Tupã.

```bash
cargo tupa check          # check current package
cargo tupa check -v       # verbose
cargo tupa check --manifest-path path/to/Cargo.toml
```text

### `cargo tupa discover`

Descubre e imprime el nombre del objetivo binario del paquete Cargo actual.

```bash
cargo tupa discover
cargo tupa discover --manifest-path path/to/Cargo.toml
```

El comando escanea `Cargo.toml` buscando una sección `[[bin]]` con un campo `name`. Si no se encuentra ninguna, cae en `src/main.rs` y usa el `package.name` como nombre del binario.

### `cargo tupa expand`

Expande el macro pipeline! a código Rust generado

```bash
cargo tupa expand --pretty
cargo tupa expand --file src/pipeline.rs
```

### `cargo tupa bench`

Benchmark del pipeline ejecutando múltiples veces y agregando métricas de timing.

```bash
cargo tupa bench                    # ejecuta con iteraciones por defecto
cargo tupa bench --iterations 100   # número custom de iteraciones
cargo tupa bench --metrics          # incluye métricas de step en la salida
```

**Opciones:**

- `--iterations <N>` — número de iteraciones del benchmark (por defecto: 10)
- `--metrics` — incluye métricas por-step de la ejecución del pipeline

La salida incluye tiempo total, duración promedio por step, y estimados de throughput.

### `cargo tupa watch`

Observa archivos fuente por cambios y re-ejecuta el pipeline automáticamente.

```bash
cargo tupa watch                    # observa src/**/*.rs por cambios
cargo tupa watch --debounce 500     # delay custom en ms
```

**Opciones:**

- `--debounce <MS>` — delay antes de re-ejecutar tras cambios (por defecto: 300ms)

### `cargo tupa run`

Ejecuta el pipeline definido en el paquete actual con entrada JSON opcional.

```bash
# Use entrada por defecto (si el pipeline soporta input unitario)
cargo tupa run

# Proporcione entrada JSON via variable de entorno
TUPA_INPUT='{"amount":1000.0,"risk_score":0.5}' cargo tupa run

# Habilita ejecución paralela de steps
TUPA_INPUT='{"x":42}' TUPA_PARALLEL=1 cargo tupa run

# Con un archivo
cargo tupa run --input data.json

# Exportar métricas de ejecución de pasos como JSON
cargo tupa run --metrics-output metrics.json
```text

El `src/main.rs` debe leer `TUPA_INPUT` (o usar el defecto) y llamar a `Executor::run` o `Executor::run_parallel`.

### `cargo tupa fmt`

Formatea código de pipeline Rust-DSL (bloques `pipeline!`) en archivos fuente.

```bash
cargo tupa fmt                # formatea todo el código de pipeline en src/
cargo tupa fmt --dry-run       # muestra lo que cambiaría
cargo tupa fmt --check         # falla si algún archivo necesita formateo
```

> **Nota:** La toolchain `.tp` legada fue removida en v0.9.0. Este comando opera exclusivamente en código Rust DSL.

### `cargo tupa lint`

Ejecuta análisis estático en definiciones de pipeline Rust-DSL (macros `pipeline!`).

```bash
cargo tupa lint                # lint del paquete actual
cargo tupa lint --json         # salida legible por máquina
cargo tupa lint --deny warnings # trata warnings como errores
```

> **Nota:** La toolchain `.tp` legada fue removida en v0.9.0. Este comando analiza solo código Rust DSL.

### `cargo tupa test`

Alias para `cargo test --examples`, conveniente para ejecutar ejemplos de pipeline y tests de integración.

```bash
cargo tupa test                # ejecuta todos los tests
cargo tupa test --example credit_decision  # test de ejemplo específico
cargo tupa test -- --nocapture  # pasa argumentos a cargo test
```

### `cargo tupa plugin new`

Genera un scaffold de plugin para funciones de step personalizadas.

```bash
cargo tupa plugin new my_plugin.rs  # crea plantilla my_plugin.rs
```

Esto crea una plantilla `my_plugin.rs` exportando:

- `_tupa_plugin_name()`: retorna el nombre del plugin
- `_tupa_plugin_register(ctx)`: registra funciones de step
- Función de step de ejemplo `my_step(input: Value) -> Value`

Compila como cdylib:

```bash
cargo build --crate-type=cdylib --release
# target/release/libmy_plugin.so (o .dll/.dylib)
```

Uso en el pipeline:

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./target/release/libmy_plugin.so")?;

fn use_plugin(pm: &PluginManager, input: &MyInput) -> Result<Value, String> {
    pm.call("my_step", serde_json::to_value(input)?).map_err(|e| e.to_string())
}
```

## Plantilla de Proyecto

Genera un nuevo proyecto desde la plantilla:

```bash
cargo generate --git https://github.com/marciopaiva/tupalang#crates/tupa-template
# o local:
cargo generate --path crates/tupa-template
```text

La plantilla incluye un pipeline de ejemplo, Cargo.toml con dependencias, y un `main.rs` que integra con `cargo-tupa run`.

## Cómo Funciona

- `check`: Delega a `cargo check` y filtra errores de macros Tupã.
- `run`: Compila y ejecuta tu binario con `TUPA_INPUT` seteado; el binario llama a `Executor::run` o `Executor::run_parallel`.
- `test`: Alias de `cargo test --examples`, ejecuta tests de ejemplos.
- `fmt`: Formatea código Rust-DSL (macros `pipeline!`) con reglas básicas de indentación.
- `lint`: Realiza análisis estático en definiciones de pipeline Rust-DSL (detecta steps duplicados, requires/produces indefinidos, nombres/inputs ausentes).
- `plugin new`: Genera plantilla de plugin (`_tupa_plugin_name`, `_tupa_plugin_register`, step function de ejemplo).
- `discover`: Imprime el nombre del objetivo binario desde `[[bin]]` o `src/main.rs`.
- `expand`: Expande el macro `pipeline!` a código Rust generado (usa `--pretty` para salida indentada).
- `bench`: Benchmark del pipeline ejecutando múltiples iteraciones y reportando métricas de timing agregadas.
- `watch`: Observa archivos fuente y re-ejecuta el pipeline en cambios con debounce configurables.

## Notas

- Workspace-aware: usa `--manifest-path` para apuntar a un Cargo.toml específico.
- Ejecución paralela: define `TUPA_PARALLEL=1` o usa la flag `--parallel` (cuando implementada).
- La salida se imprime en stdout; errores en stderr.
