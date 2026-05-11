# Config DSL — Referencia

## Propósito

El Config DSL proporciona una sintaxis declarativa para definir bloques de configuración tipados que sirven como pre-condiciones para los pasos de pipeline. Los bloques de config son nodos AST de primera clase, permitiendo validación estática de las entradas del pipeline antes de la ejecución.

## Sintaxis

```tupa
config Nombre {
    tipo campo_nombre: Tipo
    tipo otro_campo: OtroTipo
    ...
}
```text

- `config` — palabra clave que introduce una declaración de configuración.
- `Nombre` — identificador para el tipo de configuración (usado luego como `Nombre`).
- Dentro de las llaves: una o más declaraciones `tipo`, cada una con un nombre de campo y un tipo.
- Los campos son inmutables dentro del step; deben ser provistos por el llamador.

## Semántica

- Un bloque `config` declara un nuevo tipo similar a un registro que solo puede ser instanciado fuera del pipeline (por el runtime o capa de orquestación).
- Los tipos de configuración son **solo entrada**: no pueden crearse dentro del código Tupã; se completan a partir de datos externos (ej.: JSON) cuando el pipeline inicia.
- Los campos están fuertemente tipados y participan en el type checker. Todos los usos de los campos de config son validados.
- Los bloques de config pueden aparecer a nivel superior de un módulo (junto con `fn`, `step`, etc.).
- Múltiples declaraciones `config` están permitidas; cada una define un tipo distinto.

## Ejemplo: Config Simple

```tupa
config ParametrosEstrategia {
    type umbral: f64
    type posicion_maxima: i64
}

step evaluar {
    input: ParametrosEstrategia
    // usar campos: umbral, posicion_maxima
    let riesgo = ...
    // ...
}
```text

El step `evaluar` declara que requiere `ParametrosEstrategia` como entrada. En runtime, el llamador provee un objeto JSON correspondiente a `{ "umbral": 0.7, "posicion_maxima": 1000 }`. El type checker garantiza que `umbral` y `posicion_maxima` se usen con los tipos correctos dentro del step.

## Ejemplo: Pipeline Dirigido por Config (config_driven_strategy.tp)

Ejemplo realista combinando pipeline y config:

```tupa
config ConfigTrading {
    type capital_inicial: f64
    type riesgo_maximo: f64
    type posicion_maxima: i64
}

step inicializar {
    output: { capital: f64 } = config.capital_inicial
}

step verificar_riesgo {
    input: { capital: f64 }
    let exposicion = capital * 0.1
    guard exposicion <= config.riesgo_maximo
}

step ejecutar {
    input: { capital: f64 }
    let tamanio = min(config.posicion_maxima, capital as i64 / 2)
    // lógica de trading...
}
```text

En este pipeline:

- El `ConfigTrading` es provisto por el runner externo (ej.: `tupa run --config trading_config.json`).
- Cada step puede acceder a `config.campo` para leer valores de configuración.
- El `guard` en `verificar_riesgo` usa un campo de config para imponer una política.
- El type checker valida que `config.capital_inicial`, `config.riesgo_maximo` y `config.posicion_maxima` existan y tengan los tipos declarados.

## Buenas Prácticas

- Use nombres **CamelCase** para tipos de configuración.
- Mantenga configs pequeños y focados: agrupe parámetros relacionados.
- Documente cada campo con comentarios para claridad.
- Trate los configs como parte del contrato público del pipeline; versionénolos junto con el código.

## Relación con Otros Recursos

- **Schema Registry**: Para evolución avanzada, los tipos de config pueden registrarse y versionarse a través de `SchemaRegistry` al desplegar pipelines en múltiples servicios.
- **Plugins**: Los plugins pueden exponer step functions que aceptan parámetros de configuración, aumentando la reutilización.
- **Hot Reload**: Combinado con hot reload, cambiar un archivo de config puede disparar el recargado del pipeline con nuevos parámetros sin reiniciar el runtime.
