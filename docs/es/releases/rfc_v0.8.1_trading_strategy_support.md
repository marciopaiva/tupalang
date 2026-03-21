# RFC: v0.8.1 Soporte para Estrategias de Trading

## Estado

- Propuesta
- Version objetivo: `0.8.1`
- Impulsor principal: modelado de la estrategia de produccion de ViperTrade

## Resumen

TupaLang ya soporta pipelines deterministas, auditoria e integracion controlada en runtime. Para sistemas de trading de produccion como ViperTrade, la siguiente brecha no es de infraestructura. La brecha es de modelado expresivo de estrategia.

La version `0.8.1` debe enfocarse en las capacidades de lenguaje y runtime necesarias para modelar politicas reales de trading de forma declarativa, en lugar de dejar la semantica central de la estrategia hardcoded en el codigo de la aplicacion.

Esta RFC propone un tema de release acotado y pragmatico:

- salidas estructuradas por step
- `reason` como concepto de primera clase
- predicados reutilizables
- bindings tipados para configuracion
- soporte para score ponderado
- soporte declarativo para politicas temporales

## Problema

Hoy ViperTrade usa TupaLang mas como una carcasa de pipeline, mientras que la semantica real de la estrategia todavia vive en Rust:

- gates de entrada
- reasons de hold
- position health score
- reglas de thesis invalidation
- parte de la politica de confirmacion de senal y cooldown

Esa division reduce el valor del lenguaje exactamente en el area donde deberia ayudar mas:

- auditabilidad
- explainability
- iteracion segura
- revision de estrategia
- confianza de release

## Objetivos

La `0.8.1` debe hacer que TupaLang sea materialmente mejor para sistemas de estrategia de produccion.

### Objetivos principales

- Permitir que los steps devuelvan resultados estructurados tipados, en lugar de solo valores primitivos.
- Permitir que las reglas de estrategia emitan reasons legibles por maquina directamente.
- Permitir composicion de politicas con predicados reutilizables.
- Permitir que las politicas de estrategia lean configuracion de runtime tipada sin empujar toda la semantica al host.
- Permitir scores ponderados de forma declarativa.
- Permitir describir politicas temporales sin incrustar toda la semantica en el host.

### Fuera de alcance

- Reemplazar responsabilidades del runtime host, como Redis, base de datos, IO de exchange u orquestacion de procesos.
- Mover todo el estado dentro de TupaLang.
- Construir una DSL completa de trading en una sola release.

## Features propuestas

### 1. Salidas estructuradas por step

Soportar salidas en formato record/objeto con campos explicitos y validacion de tipos.

Ejemplos deseados:

- `EntryPolicyResult`
- `SizingResult`
- `PositionHealthResult`
- `ThesisExitResult`
- `StrategyDecision`

Forma ilustrativa:

```text
step("entry_policy") {
  {
    eligible: true,
    side: long,
    entry_score: 72,
    reason: "entry_confirmed_consensus_and_momentum"
  }
}
```

### 2. `reason` de primera clase

Permitir que el resultado de una politica cargue naturalmente el motivo.

Forma ilustrativa:

```text
{
  passed: false,
  reason: "entry_blocked_low_volume"
}
```

Esto evita duplicacion de logica en el host solo para explicar por que una regla fallo.

### 3. Predicados reutilizables

Permitir politicas compuestas por predicados reutilizables con nombre, como:

- `passes_consensus(side)`
- `passes_momentum(side)`
- `passes_macro(side)`
- `passes_liquidity(side)`

Esto reduce duplicacion entre:

- long y short
- entry y hold
- health y exit

### 4. Soporte para score ponderado

Soportar composicion de score ponderado para politicas como position health.

Casos de uso:

- alineacion de consenso con peso alto
- regimen de la exchange ancla con peso medio/alto
- alineacion macro de BTC con peso medio
- senal del histograma MACD con peso bajo/medio

Capacidades deseadas:

- componentes aditivos
- penalizaciones y bonos
- clamp/rango
- comparacion con thresholds

### 5. Bindings tipados para configuracion

Soportar acceso tipado a valores de configuracion provistos por el host y usados por sistemas de estrategia en produccion.

Casos de uso ilustrativos:

- thresholds por simbolo
- overlays por modo
- parametros de trailing
- thresholds de confirmacion
- thresholds de filtros macro

Capacidades deseadas:

- lecturas tipadas desde un objeto de configuracion provisto por el host
- defaults explicitos o reglas claras de fallback
- validacion de shape antes de la ejecucion del runtime

### 6. Soporte declarativo para politicas temporales

Soportar semanticas que dependen de persistencia a lo largo de ciclos de evaluacion.

Patrones objetivo:

- confirmar por `N` ticks
- degradar por `N` ticks antes de salir
- cooldown despues de stop loss
- bloquear reentrada hasta flip del lado

El runtime host todavia puede mantener el estado, pero la politica en si deberia declararse en TupaLang.

## Por que esto importa para ViperTrade

Con estas features, ViperTrade puede mover la semantica central de la estrategia a TupaLang:

- elegibilidad de entrada
- reasons de hold
- position health score
- thesis invalidation
- parte de la politica de reentrada y confirmacion

Eso dejaria a Rust enfocado en las responsabilidades correctas:

- orquestacion de runtime
- persistencia de estado
- integracion con exchange
- transporte de eventos
- plumbing de riesgo

## Refinamiento de prioridad despues de la integracion con ViperTrade

La migracion `0.8.1` de ViperTrade dejo claro el siguiente cuello de botella real.

Lo que el lenguaje ya resuelve lo suficientemente bien:

- records tipados
- record literals
- helpers de `reason`
- helpers de score ponderado
- reutilizacion mediante funciones normales para buena parte de los helpers tipo predicado

Lo que todavia fuerza demasiado wiring del lado del host:

- acceso a configuracion
- overlays de modo/perfil
- seleccion de thresholds desde la configuracion de la estrategia
- semantica temporal de confirmacion y cooldown

Como resultado, la siguiente prioridad de implementacion deberia ser:

1. bindings tipados para configuracion
2. soporte declarativo para politicas temporales
3. mejor ergonomia para predicados reutilizables cuando las funciones comunes sigan siendo insuficientes

## Orden de entrega propuesta

### Fase 1

- salidas estructuradas por step
- `reason` de primera clase
- soporte para score ponderado

Este es el minimo necesario para mover politica de entrada, reasons de `HOLD` y modelos de score a TupaLang.

### Fase 2

- bindings tipados para configuracion

Esto habilita estrategias de produccion como ViperTrade a leer thresholds y overlays de perfil de forma declarativa.

### Fase 3

- soporte declarativo para politicas temporales

Esto habilita confirmacion de senal, ventanas de persistencia de tesis y semantica de cooldown.

### Fase 4

- ergonomia para predicados reutilizables

Los predicados con nombre siguen siendo utiles, pero ViperTrade mostro que las funciones normales ya cubren parte de esa necesidad hoy.

## Impacto esperado

Si la `0.8.1` entrega este alcance, TupaLang pasa a ser mucho mas util para sistemas reales de politica, y no solo para orquestacion de pipelines.

Resultados esperados:

- menos duplicacion de estrategia en el host
- revisiones de release mas claras
- ajuste de estrategia mas facil
- mejores trazas de auditoria
- mejor encaje para aplicaciones reales de trading

## Preguntas abiertas

- Debe el soporte de score ponderado nacer como sintaxis del lenguaje o primero como helper de biblioteca estandar?
- Debe `reason` modelarse como una convencion sobre records estructurados o como una primitiva propia del lenguaje?
- Hasta donde debe llegar el soporte temporal en `0.8.1` antes de convertirse en diseno de maquina de estados?

## Recomendacion

Adoptar esta RFC como ancla de planificacion para la `0.8.1`, con la Fase 1 como barra minima del release y las Fases 2-3 como alcance objetivo cuando el costo de implementacion siga siendo controlado.
