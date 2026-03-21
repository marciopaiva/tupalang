# Notas Post-Release: v0.8.1

## Propósito

Esta nota resume lo que `v0.8.1` nos enseñó después de publicar la integración entre el
lenguaje, el runtime y `ViperTrade`.

## Lo que funcionó bien

- `ViperTrade` sirvió como prueba funcional real para `TupaLang`.
- Los slices pequeños y mergeables mantuvieron el release en movimiento sin perder confianza.
- La secuencia de features fue coherente:
  - outputs estructurados
  - reasons de política de primera clase
  - soporte de weighted score
  - patrón de input tipado para config
  - soporte declarativo de política temporal
- La documentación del release se ajustó antes del tag público, no después.

## Lecciones principales

### 1. El cuello de botella ya no era el shape de los datos

Una vez que `record types`, `record literals`, field access tipado, validación estructurada
de runtime y outputs de política ponderados estuvieron listos, las siguientes ganancias ya
no vinieron de más maquinaria de shape.

La palanca real pasó a ser:

- hacer la política reutilizable
- modelar la política temporal de forma explícita
- pasar estado tipado provisto por el host al pipeline

### 2. El host debe conservar el estado operacional

El trabajo de `0.8.1` confirmó que el lenguaje se vuelve más útil cuando modela política,
no cuando intenta absorber todo el estado del host.

Las preocupaciones stateful siguen perteneciendo a la aplicación host:

- contadores de confirmación de señales
- seguimiento de cooldown
- estado del trailing stop
- persistencia y side effects externos

El lenguaje ayudó más cuando pudo describir cómo interpretar ese estado.

### 3. El pipeline ganó valor cuando reflejó shapes reales

La capa `.tp` se volvió materialmente más útil cuando dejó de ser solo un placeholder
arquitectónico y empezó a recibir:

- inputs estructurados
- snapshots reales de estado temporal
- outputs estructurados consumidos por el runtime de la aplicación

Eso convirtió el pipeline en un contrato real y no en un boceto de diseño futuro.

### 4. El tooling standalone debe validarse temprano

El flujo de validación local expuso una lección operacional importante: si la ruta de
validación usa un binario `tupa` desactualizado, el pipeline parece roto aunque los cambios
del lenguaje sean correctos.

El próximo trabajo debería alinear temprano:

- `tupa-cli`
- scripts locales de validación
- imágenes de contenedor usadas por CI o compose

## Qué haríamos antes la próxima vez

- Alinear antes el CLI standalone y la ruta de validación local.
- Documentar antes el límite entre política declarativa y estado gestionado por el host.
- Añadir antes algunos tests enfocados para los nuevos outputs estructurados expuestos.

## Qué logró v0.8.1

`v0.8.1` movió a `TupaLang` desde un runtime de pipelines con gobernanza básica hacia un
lenguaje más útil para sistemas reales de estrategia:

- los outputs pueden ser estructurados y tipados
- las reasons pueden ser de primera clase
- la política se puede componer con weighted scores
- la política temporal se puede expresar de forma declarativa
- la config tipada provista por el host se puede modelar sin sintaxis nueva en el core

## Próximos puntos de presión

Las siguientes ganancias relevantes no vienen de más primitivas de shape. Vienen de:

- mejor ergonomía para bloques reutilizables de política
- seguir refinando el límite entre estado del host y política declarativa
- decidir si algunos breakdowns estructurados internos deben convertirse en contratos públicos
