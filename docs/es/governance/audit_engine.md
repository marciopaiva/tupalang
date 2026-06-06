# Motor de Auditoría (descontinuado)

> **Eliminado en 0.9.0.** La auditoría por hash determinista descrita aquí era
> parte del toolchain `.tp` independiente (crates `tupa-audit` y `tupa-parser`),
> eliminado en 0.9.0. La arquitectura actual basada en crates **no** incluye esta
> función de hash de ejecución.

## Qué existía

La función generaba una huella SHA3-256 estable de una ejecución combinando el
AST normalizado, las entradas JSON canónicas y la versión del compilador (vía
`tupa-audit::hash_execution`). Como el compilador `.tp` fue eliminado, esa función
y sus crates ya no existen en el workspace.

## Mecanismo actual de observabilidad

Para rastrear ejecuciones en el Rust-DSL actual, usa las métricas por paso de
`tupa-engine`:

- `PipelineResult::metrics` — un `Vec<StepMetrics>` con `step_id`, timestamps de
  inicio/fin y duración de cada paso.
- `PipelineResult::passed` — resultado agregado de la evaluación de constraints.

Para reproducibilidad o integridad estilo auditoría, realiza el hashing a nivel de
tu aplicación sobre la entrada serializada y los valores resultantes
(`PipelineResult::values`). Consulta también [TRANSITION.md](../TRANSITION.md) para
la migración del flujo `.tp` heredado.
