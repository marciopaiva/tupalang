# Guia de Pipeline

## Objetivo

Executar um pipeline Tupã de ponta a ponta: gerar um ExecutionPlan e executar com entrada JSON.

## Passos

- Escreva o pipeline com a macro `pipeline!` (veja [getting_started.md](getting_started.md)).
- Verifique os tipos: `cargo tupa check`.
- Execute com entrada JSON: `cargo tupa run --input data.json`.
- Execução paralela: `cargo tupa run --parallel --input data.json`.
- Persistir métricas por passo: `cargo tupa run --input data.json --metrics-output metrics.json`.

## Estrutura do ExecutionPlan

- name, version, seed (opcional), input_schema
- steps: name, function_ref, effects
- constraints: metric, comparator, threshold
- metrics: valores literais capturados do bloco de validação
- metric_plans: { name, function_ref, args } para calcular métricas em runtime

## Notas

- Formato de function_ref: `<file>::step_<name>`.
- Efeitos (random/time) são identificados pelo typechecker.
- O runtime avalia restrições e emite um relatório JSON com métricas/restrições.

---

## StepContext — Lendo Saídas de Steps Anteriores

Cada corpo de step tem acesso a `ctx: &StepContext`, que carrega as saídas dos steps anteriores. Isso permite que steps posteriores leiam resultados prévios sem re-executá-los.

```rust
pipeline! {
    name: ScoringPipeline,
    input: MarketData,
    steps: [
        step("base_score") { compute_base(input) },
        step("adjusted_score") {
            // ler a saída de "base_score" do contexto
            let base = ctx.get_f64("base_score").unwrap_or(0.0);
            base * input.volatility_multiplier
        } requires ["base_score"],
    ],
    constraints: []
}
```

### API do StepContext

| Método | Retorno | Descrição |
|--------|---------|-----------|
| `ctx.get("name")` | `Option<&Value>` | Valor JSON bruto |
| `ctx.get_f64("name")` | `Option<f64>` | Parsear como f64 |
| `ctx.get_bool("name")` | `Option<bool>` | Parsear como bool |
| `ctx.get_str("name")` | `Option<&str>` | Parsear como &str |
| `ctx.get_as::<T>("name")` | `Option<T>` | Deserializar em T |

### Declarar Dependências

Use `requires ["step_name"]` para que o executor paralelo saiba propagar a saída:

```rust
step("decision") {
    let score = ctx.get_f64("score").unwrap_or(0.0);
    let valid = ctx.get_bool("validate").unwrap_or(false);
    if valid && score > threshold { "ENTER" } else { "HOLD" }
} requires ["score", "validate"]
```

Se `requires` for omitido, `ctx` pode estar vazio (o step executa concorrentemente com outros). O parâmetro `ctx` está sempre presente — use `_ctx` se não precisar dele.

---

## Limiares de Constraints Calculados

Os limiares de constraints agora aceitam qualquer expressão Rust — a variável `input` está em escopo:

```rust
pipeline! {
    name: DynamicThreshold,
    input: RiskParams,
    steps: [ step("equity_floor") { input.account_equity_usdt } ],
    constraints: [
        metric("equity_floor").ge(input.min_equity_threshold)
    ]
}
```

---

## Constraints Fail-Fast

Adicione `.fail_fast()` para abortar imediatamente em caso de violação — útil para invariantes rígidas:

```rust
constraints: [
    metric("equity_floor").ge(0.0).fail_fast(),  // abortar se negativo
    metric("score").le(input.max_score),          // só verificado se o anterior passar
]
```

Sem `.fail_fast()`, todos os constraints são avaliados e todas as falhas coletadas. Com ele, o pipeline para na primeira violação.

---

## Acessores Tipados de PipelineResult

```rust
let result = executor.run_parallel(&pipeline, &input).await?;
let score: Option<f64>           = result.get_f64("score");
let ok: Option<bool>             = result.get_bool("validate");
let label: Option<&str>          = result.get_str("label");
let decision: Option<MyDecision> = result.get_as::<MyDecision>("decision");

// O acesso direto ao mapa ainda funciona
let raw: &Value = &result.values["score"];
```
