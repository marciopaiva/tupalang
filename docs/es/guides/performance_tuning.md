# Ajuste de Rendimiento

Guía para optimizar la ejecución de pipelines de Tupã.

## Ejecución Paralela

### Habilitar Runtime de Tokio

La ejecución paralela de pasos (`Executor::run_parallel`) requiere un runtime de Tokio. Asegúrate de que tu binary use `#[tokio::main]` o crea manualmente un runtime:

```rust
#[tokio::main]
async fn main() {
    let plan = MyPipeline::new();
    let engine = Executor::new();
    let result = engine.run_parallel(&plan, &input).await?;
}
```

### Anotaciones de Dependencias

Anotaciones precisas de `produces` y `requires` habilitan máxima paralelización. Especificar dependencias de más serializa la ejecución.

```rust
pipeline! {
    steps: [
        step("fetch")   { fetch_data(input) }  produces ["raw"],
        step("parse")   { parse(&raw) }       requires ["raw"] produces ["parsed"],
        step("validate"){ validate(&parsed) } requires ["parsed"],
        // métricas independientes pueden ejecutarse en paralelo con parse
        step("log_count") { count_logs(input) }
    ]
}
```

### Grado de Paralelismo

Los hilos de trabajo por defecto de Tokio equivalen al número de núcleos de CPU. Sobrescribe usando `tokio::runtime::Builder` si es necesario:

```rust
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()?;
```

## Eficiencia de Memoria

### Evitar Clonaciones Innecesarias

Las funciones de paso que toman `&Input` y devuelven valores owning ya son óptimas. Evita clonar estructuras grandes dentro de los pasos; usa referencias cuando sea posible.

```rust
fn process(data: &LargeStruct) -> Metric {
    // presta, no clona
    data.compute_metric()
}
```

### Reutilizar Datos de Entrada

Si múltiples pasos necesitan los mismos datos derivados, calcula una vez en un paso early y produce una métrica para los pasos downstream.

## Overhead de FFI de Plugin

Llamadas dinámicas de plugin (`PluginManager::call`) incurren en costo de transición FFI (serialización/deserialización + llamada C). Para escenarios de alto throughput:

- Operaciones por lotes: diseña plugins que acepten arrays de entradas y devuelvan arrays de salidas.
- Mantén la lógica del plugin liviana; delega trabajo pesado a pasos Rust-native cuando sea posible.
- Perfila con `cargo bench --bench plugin_bench` para medir overhead.

Overhead esperado: ~0.5–2µs por llamada en x86_64 (varía por tamaño de entrada). Si es significativo, considera funciones de paso in-process en vez de FFI.

## Benchmarking

Usa benchmarks `criterion` para medir rendimiento:

```bash
# Benchmarks de engine
cargo bench --bench engine_bench -p tupa-engine

# Benchmarks de FFI de plugin
cargo bench --bench plugin_bench -p tupa-plugin
```

Métricas clave a monitorear:

- Throughput secuencial de pasos (pasos/seg)
- Speedup paralelo vs secuencial (ideal: casi-lineal para pasos independientes)
- Overhead de chequeo de constraints (por constraint)
- Latencia de llamada de plugin (µs)

## Optimización de Constraints

Las constraints se evalúan after todos los pasos. Para pipelines con muchos pasos y constraints:

- Coloca constraints en métricas producidas por pasos directamente (evita recomputar valores derivados).
- Usa operadores de comparación simples (`ge`, `le`, `eq`, `ne`, `gt`, `lt`) — están optimizados.
- Evita cálculos costosos dentro de expresiones de constraint; computa una vez en un paso y referencia la métrica.

## Optimización Guiada por Perfil (PGO)

Para pipelines en producción críticos:

```bash
# 1. Build con instrumentación
cargo build --release -p tupa-engine --profile=pgo-instrument

# 2. Ejecutar workload representativo para recolectar perfiles
./target/release/my_pipeline < input.json

# 3. Build con datos PGO
cargo build --release -p tupa-engine --profile=pgo-opt
```

## Configuración de Canales (Avanzado)

El engine usa canales MPSC sin límite para notificaciones de completación de paso. Para recuentos de pasos extremadamente altos (1000+), considera:

- Ajustar semántica de canales Tokio (actualmente sin límite, sin backpressure).
- Agrupar escrituras de métricas en pasos que producen muchos valores.

## Problemas Comunes

| Síntoma | Causa Probable | Solución |
|---------|----------------|----------|
| Ejecución paralela no más rápida que secuencial | Dependencias over-specified (falsas) | Audita `requires`/`produces`; mantén minimal |
| Alto uso de memoria | Pasos retienen asignaciones grandes después de ejecución | Libera valores pesados al final del paso; usa asignaciones con scope |
| Llamadas a plugin lentas | FFI frecuente de llamadas pequeñas | Agrupa llamadas o mueve lógica a pasos nativos |

## Lectura Adicional

- Documentación de runtime Tokio: <https://docs.rs/tokio/latest/tokio/runtime/>
- Libro de Criterion: <https://bheisler.github.io/criterion.rs/book/>
- Rust Performance Book: <https://nnethercote.github.io/perf-book/>
