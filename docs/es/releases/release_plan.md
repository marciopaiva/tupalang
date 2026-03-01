# Plan de Lanzamiento (0.4.x → 1.0)

## Propósito

Definir los hitos de lanzamiento desde v0.4.x hasta v1.0, alineados con la hoja de ruta y las fases de adopción.

## Referencias

- [Hoja de ruta](roadmap.md)
- [Plan de adopción](../governance/adoption_plan.md)
- [Guía de versionado](versioning.md)
- [Changelog](changelog.md)

## Base (actual)

- v0.6.0 lanzado con genéricos en enums, destructuring/guards en match y prototipo de auditoría.
- Diagnósticos con spans y salida JSON.
- SPEC v0.1 y documentación consolidada.

## Hitos

### 0.5.x — Confiabilidad del compilador

- Completar las restricciones y validaciones restantes del verificador de tipos.
- Mejorar la consistencia de los diagnósticos y la claridad de los errores.
- Expandir la cobertura de pruebas, especialmente casos negativos.

### 0.6.x — Estabilidad de la generación de código y base de pipelines

- Optimizaciones básicas de IR (eliminación de código muerto, simplificaciones).
- Salida estable para `fn main()` y ejemplos principales.
- Benchmarks iniciales y pruebas de regresión.
- Sintaxis de pipeline en borrador (orquestación, validación, hooks de auditoría).

#### 0.6.0 — Plan estratégico

**Tema central**: Máquinas de estado con garantías formales.

##### Prioridades técnicas

1. Enums con restricciones éticas (parser/verificador de tipos)
   - Sintaxis EBNF para enums con genéricos.
   - Inferencia de tipo de variant.
   - Propagación de restricciones en variants (`Safe<T>` dentro de `Enum<Safe<T>>`).
   - Errores claros cuando las restricciones se violan dentro de `match`.
   - Estado: hecho
2. Coincidencia de patrones con destructuring completo
   - Destructuring de tuplas dentro de patrones.
   - Guards con acceso a bindings.
   - Chequeo de exhaustividad.
   - Span preciso para patrón no cubierto.
   - Estado: hecho
3. Motor de auditoría v0.1 (prototipo determinístico)
   - CLI `tupa audit` con salida JSON (hash + fingerprint de AST).
   - Reproducibilidad: misma entrada → mismo hash en distintas máquinas.
   - Documentación en `docs/en/governance/audit_engine.md`.
   - Estado: hecho
4. Diagnósticos con sugerencias accionables
   - Código de error específico para restricciones no comprobadas.
   - Sugerencias contextuales con atributos de seguridad.
   - Enlaces a documentación de restricciones.

##### Criterios de aceptación

- Genéricos de enum hacen parse y chequeo de tipos con inferencia correcta. (hecho)
- Restricciones Safe preservadas a través de variants de enum y brazos de `match`. (hecho)
- Matches no exhaustivos se rechazan con spans accionables. (hecho)
- La salida JSON de `tupa audit` incluye hash SHA3-256 y fingerprint de AST.
- La salida de auditoría es estable en dos ejecuciones independientes.
- Los diagnósticos incluyen un hint de ayuda cuando falta una prueba de seguridad.
- `examples/audit/fraud_pipeline.tp` compila solo con `@safety` válido.

##### Hoja de ruta semanal

- Semana 1: Enums + genéricos en parser/verificador de tipos.
- Semana 1: Enums + genéricos en parser/verificador de tipos, EBNF actualizado, pruebas de parsing.
- Semana 2: Propagación de restricciones en enums, 15+ pruebas con `Safe<T>` en variants.
- Semana 3: Exhaustividad + destructuring en match, pruebas negativas.
- Semana 4: Prototipo del motor de auditoría + CLI, comando `tupa audit`, docs iniciales.
- Semana 5: Refinamiento de diagnósticos con sugerencias, pruebas goldens.
- Semana 6: RC + docs, CHANGELOG, ejemplos reales en `examples/audit/`.

##### Fuera de alcance

- Backend LLVM completo.
- FFI de Python.
- Operador `∇`.
- Async/await.

##### Métrica de éxito

- Un pipeline de decisión de crédito con estados approve/review/reject compila con una prueba formal de seguridad en menos de 50 líneas.

### 0.7.x — Base de tooling y orquestación

- Formateador oficial (`fmt`) con conjunto de reglas mínimo.
- Linter mínimo (`lint`) para chequeos de estilo y seguridad.
- Estabilización del CLI con `build`, `run`, `fmt`, `check`.

### 0.8.x — Integración controlada de Python y auditabilidad

- Ejecución de PyTorch/TensorFlow vía adaptadores controlados.
- Llamadas Python rastreables con hooks de validación.
- Esquema de log de auditoría para ejecución externa (integraciones Python).

### 0.9.x — Interoperabilidad

- FFI con C/Rust y ABI documentada.
- Bindings mínimos para bibliotecas esenciales y ejemplos.
- Servidor de lenguaje con autocomplete, diagnósticos e ir a definición.

### 1.0.0 — Calidad y confianza

- Benchmarks públicos y reproducibles.
- Política de compatibilidad auditada y aplicada.
- SPEC finalizada con EBNF, ejemplos validados y diagnósticos normativos.
- Workflows de gobernanza validados para entornos regulados.

## Puertas de lanzamiento (todas las versiones)

- CHANGELOG actualizado con cambios visibles para usuarios.
- Pruebas y lint de docs pasando.
- Ejemplos principales validados.
- CI en verde antes de taguear.
