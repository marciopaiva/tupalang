# Roadmap

## Propósito

Resumir el plan de evolución del proyecto.

## Corto plazo

- v0.8.1: fortalecer el soporte para estrategias de trading en sistemas reales de politica.
- Agregar salidas estructuradas por step y `reason` de primera clase.
- Agregar predicados reutilizables para composicion de estrategia.
- Agregar soporte para score ponderado en evaluacion de politica.
- Agregar primitivas declarativas para confirmacion y cooldown.

- Consolidar la SPEC v0.1 (ajustes finos y ejemplos validados).
- Mejorar el typechecker (restricciones y diagnósticos).
- Estabilizar el IR textual de codegen y las salidas del CLI.
- Expandir ejemplos safe y goldens negativos.

## Mediano plazo

- Lenguaje de pipeline MVP con ejecución determinista.
- Primitivas de auditoría y hashing para reproducibilidad.
- Integración Python controlada y auditable (PyTorch/TensorFlow).
- Formateador oficial y linter mínimo.
- Language server básico.

## Largo plazo

- FFI con C/Rust.
- ABI documentada.
- Benchmarks públicos.
- Herramientas de nivel enterprise y flujos de compliance.
