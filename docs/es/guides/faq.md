
# FAQ

## Propósito

Responder preguntas comunes sobre el proyecto y el lenguaje.

## Preguntas frecuentes

### 1) ¿El proyecto está listo para producción?

Todavía no. La especificación v0.1 está completa, pero el compilador aún se está implementando.

### 2) ¿Cuál es el enfoque principal del lenguaje?

Gobernanza y determinismo para pipelines de IA en sistemas críticos, con seguridad formal y rendimiento predecible.

### 3) ¿Cómo puedo contribuir?

Consulta [CONTRIBUTING.md](../../CONTRIBUTING.md) y abre una issue con contexto.

### 4) ¿Dónde puedo encontrar ejemplos?

En [examples](../../examples/README.md).

### 4.1) ¿Hay ejemplos de safe/alineación?

Sí. Mira los ejemplos `safe_*` en [examples](../../examples/README.md).

### 5) ¿Qué son los tipos `Safe<T, ...>`?

Tipos con restricciones probadas en tiempo de compilación, por ejemplo `Safe<f64, !nan>` o `Safe<string, !misinformation>`.

### 6) ¿Cómo ejecuto el CLI?

Usa `cargo run -p tupa-cli -- <command>` y revisa [docs/getting_started.md](getting_started.md).

### 7) ¿Hay una hoja de ruta?

Sí, están en desarrollo.
Ver [Plan MVP](../overview/mvp_plan.md) y [Plan de adopción](../governance/adoption_plan.md).

### 8) ¿Puedo proponer cambios a la spec?

Sí. Abre una issue con el prefijo `[RFC]`.

### 9) ¿Cómo funciona la interoperabilidad con otros lenguajes?

El diseño incluye FFI (Foreign Function Interface) para integrarse con Rust, C y Python.

### 10) ¿Cómo es el rendimiento comparado con otros lenguajes?

El objetivo es un rendimiento predecible, cercano a Rust/C para código crítico. Benchmarks y ejemplos se publicarán en releases futuras.

### 11) ¿Cómo depuro u obtengo diagnósticos detallados?

Consulta [docs/diagnostics_checklist.md](../reference/diagnostics_checklist.md) y [docs/common_errors.md](../reference/common_errors.md) para ejemplos de mensajes y consejos.

### 12) ¿Hay consejos de uso o buenas prácticas?

Consulta [docs/README.md](../index.md) para enlaces rápidos.

### 13) ¿Cómo contribuyo con ejemplos o documentación?

Consulta [CONTRIBUTING.md](../../CONTRIBUTING.md) y [docs/docs_contributing.md](docs_contributing.md).
