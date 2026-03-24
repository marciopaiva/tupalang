# Checklist: v0.8.2

## Tema

La `v0.8.2` debe ser una release enfocada en ergonomía aplicada de policy runtime.

Objetivo principal de la release:

- hacer que TupaLang sea mejor para expresar y apoyar los flujos de policy que ViperTrade ya está ejercitando en un runtime local con características de producción

## Barra de release

Barra mínima para `v0.8.2`:

- al menos una mejora concreta en ergonomía de policy temporal
- al menos una mejora concreta para outputs estructurados de decisión
- docs y ejemplos aplicados mostrando el modelo de uso guiado por ViperTrade
- ninguna regresión en el workflow de publicación del CLI standalone y de los crates

## Épico 1: Fundamentos de Policy Temporal

### Diseño

- [ ] Definir la semántica objetivo para estados como `pending`, `degrading` y `confirmed`.
- [ ] Decidir qué pertenece al lenguaje/runtime y qué sigue en el estado del host.
- [ ] Documentar el shape preferido del contrato para outputs de policy temporal.

### Implementación

- [ ] Añadir una mejora pequeña y concreta en ergonomía de guards temporales.
- [ ] Garantizar que la mejora pueda validarse con ejemplos o tests.
- [ ] Confirmar que el resultado sigue siendo compatible con ejecución determinista.

### Validación

- [ ] Añadir al menos un ejemplo aplicado que refleje un flujo temporal real.
- [ ] Validar el ejemplo con CI local y verificación de release.

## Épico 2: Contratos Estructurados de Decisión

### Diseño

- [ ] Estandarizar un shape recomendado para outputs de decisión.
- [ ] Cubrir campos como `action`, `stage`, `reason`, `score`, `components` y `flags`.
- [ ] Decidir qué parte es convención y qué parte será contrato público estable.

### Implementación

- [ ] Mejorar una superficie de runtime/codegen/typecheck para apoyar mejor outputs estructurados.
- [ ] Mantener el cambio lo bastante pequeño para seguir siendo seguro para release.

### Validación

- [ ] Añadir ejemplos y docs que muestren cómo deben verse los resultados estructurados.
- [ ] Verificar compatibilidad con consumidores actuales de crates y del CLI.

## Épico 3: Fundamentos de External Typed Effects

Este épico debe mantenerse acotado en la `0.8.2`.

### Diseño

- [ ] Escribir la primera especificación técnica para external effects tipados.
- [ ] Cubrir contratos tipados de input/output, timeout, fallback y metadata de auditoría.
- [ ] Separar explícitamente pasos externos `advisory` de pasos `critical`.

### Implementación

- [ ] Si la implementación empieza en la `0.8.2`, mantenerla experimental y acotada.
- [ ] Preferir un slice simple de external effect en vez de una integración amplia con providers.

### Validación

- [ ] Garantizar reglas deterministas de fallback en todos los caminos experimentales.
- [ ] Documentar cómo esto apoya integraciones advisory como el AI Analyst de ViperTrade.

## Épico 4: Documentación Aplicada

- [ ] Añadir una nota corta de arquitectura aplicada conectando TupaLang y ViperTrade.
- [ ] Mostrar explícitamente la separación entre policy y runtime.
- [ ] Documentar qué queda en policy Tupa y qué sigue en el runtime host.
- [ ] Incluir al menos un ejemplo de output estructurado y un ejemplo de policy temporal.

## Crates y Operación de Release

- [ ] Mantener la paridad de READMEs de crates alineada con el posicionamiento del README principal.
- [ ] Verificar que el workflow de publish siga cubriendo todos los crates en releases por tag.
- [ ] Ejecutar la verificación de release antes de cortar la tag.
- [ ] Confirmar alineación entre docs, changelog y release notes antes de publicar.

## Qué evitar en la v0.8.2

- [ ] No expandir el alcance hacia una DSL completa de selección de portafolio.
- [ ] No volver obligatoria la integración con AI externa para el uso normal del runtime.
- [ ] No empujar cambios grandes de sintaxis sin validación aplicada.
- [ ] No sobrecargar la release con múltiples experimentos de lenguaje no relacionados.

## Criterios de éxito

La `v0.8.2` será exitosa si:

- TupaLang queda materialmente mejor para policy temporal y outputs estructurados
- la release sigue siendo lo bastante pequeña para publicarse con confianza
- ViperTrade puede señalar al menos una simplificación o clarificación concreta habilitada por esta línea
