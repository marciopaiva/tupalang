# Codegen

## Propósito

Descrever o estado atual de `tupa-codegen` e o fluxo `parse -> typecheck -> codegen`.

`tupa-codegen` gera um IR textual funcional (LLVM-like, não LLVM completo) cobrindo todas as features do MVP, incluindo funções anônimas (lambdas), valores de função, print como built-in, concatenação de strings, arrays, fluxo de controle e mais.

## Uso no CLI

```bash
cargo run -p tupa-cli -- codegen examples/hello.tp
cargo run -p tupa-cli -- codegen examples/arith.tp
cargo run -p tupa-cli -- codegen examples/array_ops.tp

# Saída JSON
cargo run -p tupa-cli -- codegen --format json examples/hello.tp

# Pipelines: gerar planos com backend híbrido
cargo run -p tupa-cli -- codegen --format llvm examples/pipeline/fraud_complete.tp
# Emite: fraud_complete.ll e fraud_complete.plan.json

# Somente plano
cargo run -p tupa-cli -- codegen --plan-only examples/pipeline/fraud_complete.tp
```

## Saída atual

Saída de IR textual (exemplo simplificado):

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

Em JSON, a saída vem embrulhada em um objeto:

```json
{
  "codegen": "declare i32 @printf(i8*, ...)\n..."
}
```

## Features suportadas

- Literais `i64`, `f64`, `bool` e `string` (strings são constantes globais)
- `let`, `return`, `print` (como built-in)
- Operadores aritméticos e comparações em `i64` e `f64`
- `if`/`match` (incluindo guards, binding de identificadores e `match` em `string` via `strcmp`)
- `while`, `for` sobre ranges, `break`/`continue`
- Arrays de `i64`, `f64` e `string`, indexação e atribuição
- Funções definidas pelo usuário e funções anônimas (lambdas)
- Chamadas de função/lambda como valores de função
- Concatenação de strings em runtime
- `+=` para strings (via concatenação)
- Testes goldens automatizados para garantir estabilidade do IR
- Pipelines: ExecutionPlan JSON com `steps`, `constraints`, `metrics`, `metric_plans`
- Tempo de execução `tupa run`: executa o plano com entrada JSON e emite relatório

## Próximos passos

- Reduzir `TODO`s restantes no codegen
- Suportar mais tipos, closures e otimizações
- Emitir binários nativos via `llvm`/`inkwell`
