# Codegen

## Propósito

Describir el estado actual de `tupa-codegen` y el flujo `parse -> typecheck -> codegen`.

`tupa-codegen` genera un IR textual funcional (LLVM-like, no LLVM completo) que cubre todas las features del MVP, incluyendo funciones anónimas (lambdas), valores de función, print como built-in, concatenación de strings, arrays, flujo de control y más.

## Uso en el CLI

```bash
cargo run -p tupa-cli -- codegen examples/hello.tp
cargo run -p tupa-cli -- codegen examples/arith.tp
cargo run -p tupa-cli -- codegen examples/array_ops.tp

# Salida JSON
cargo run -p tupa-cli -- codegen --format json examples/hello.tp

# Pipelines: generar planes con backend híbrido
cargo run -p tupa-cli -- codegen --format llvm examples/pipeline/fraud_complete.tp
# Emite: fraud_complete.ll y fraud_complete.plan.json

# Solo plan
cargo run -p tupa-cli -- codegen --plan-only examples/pipeline/fraud_complete.tp
```

## Salida actual

Salida de IR textual (ejemplo simplificado):

```text
declare i32 @printf(i8*, ...)
@.fmt_int = private unnamed_addr constant [5 x i8] c"%ld\0A\00"

define void @main() {
entry:
  %t0 = alloca i64
  store i64 42, i64* %t0
  ret void
}
```

En JSON, la salida viene envuelta en un objeto:

```json
{
  "codegen": "declare i32 @printf(i8*, ...)\n..."
}
```

## Features soportadas

- Literales `i64`, `f64`, `bool` y `string` (strings son constantes globales)
- `let`, `return`, `print` (como built-in)
- Operadores aritméticos y comparaciones en `i64` y `f64`
- `if`/`match` (incluye guards, binding de identificadores y `match` en `string` vía `strcmp`)
- `while`, `for` sobre rangos, `break`/`continue`
- Arrays de `i64`, `f64` y `string`, indexación y asignación
- Funciones definidas por el usuario y funciones anónimas (lambdas)
- Llamadas de función/lambda como valores de función
- Concatenación de strings en runtime
- `+=` para strings (vía concatenación)
- Pruebas goldens automatizadas para asegurar estabilidad del IR
- Pipelines: ExecutionPlan JSON con `steps`, `constraints`, `metrics`, `metric_plans`
- Tiempo de ejecución `tupa run`: ejecuta el plan con entrada JSON y emite reporte

## Próximos pasos

- Reducir `TODO`s restantes en codegen
- Soportar más tipos, closures y optimizaciones
- Emitir binarios nativos vía `llvm`/`inkwell`
