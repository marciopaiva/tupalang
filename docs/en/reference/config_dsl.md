# Config DSL Reference

## Purpose

The Config DSL provides a declarative syntax for defining typed configuration blocks that serve as pre-conditions for pipeline steps. Config blocks are first-class AST nodes, enabling static validation of pipeline inputs before execution.

## Syntax

```tupa
config Name {
    type field_name: Type
    type another_field: AnotherType
    ...
}
```

- `config` — keyword introducing a configuration declaration.
- `Name` — identifier for the config type (used later as `Name`).
- Inside braces: one or more `type` declarations, each with a field name and a type.
- Fields are immutable within the pipeline step; they must be provided by the caller.

## Semantics

- A `config` block declares a new record-like type that can only be instantiated from outside the pipeline (typically by the runtime or orchestration layer).
- Config types are **input-only**: they cannot be created inside Tupã code; they are populated from external data (e.g., JSON input) when the pipeline starts.
- The fields are strongly typed and participate in the type checker. All uses of config fields are validated.
- Config blocks can appear at the top level of a module (alongside `fn`, `step`, etc.).
- Multiple config declarations are allowed; each defines a distinct type.

## Example: Simple Config

```tupa
config StrategyParams {
    type threshold: f64
    type max_position: i64
}

step evaluate {
    input: StrategyParams
    let risk = ... // use threshold and max_position
    // ...
}
```

The `evaluate` step declares that it requires `StrategyParams` as input. At runtime, the caller supplies a JSON object matching the shape `{ "threshold": 0.7, "max_position": 1000 }`. The type checker ensures that `threshold` and `max_position` are used with correct types inside the step.

## Example: Config-Driven Pipeline (config_driven_strategy.tp)

A realistic example combining a pipeline with a config and multiple steps:

```tupa
config TradingConfig {
    type initial_capital: f64
    type risk_limit: f64
    type max_position: i64
}

step initialize {
    output: { capital: f64 } = config.initial_capital
}

step risk_check {
    input: { capital: f64 }
    let exposure = capital * 0.1
    guard exposure <= config.risk_limit
}

step execute {
    input: { capital: f64 }
    let position_size = min(config.max_position, capital as i64 / 2)
    // trading logic...
}
```

In this pipeline:

- The `TradingConfig` is provided by the external runner (e.g., `tupa run --config trading_config.json`).
- Each step can reference `config.field` to access configuration values.
- The `guard` in `risk_check` uses a config field to enforce a policy.
- The type checker verifies that `config.initial_capital`, `config.risk_limit`, and `config.max_position` exist and have the declared types.

## Best Practices

- Use **CamelCase** names for config types.
- Keep configs small and focused: group related parameters together.
- Document each field with comments for clarity.
- Treat configs as part of your pipeline's public contract; version them alongside your code.

## Relationship to Other Features

- **Schema Registry**: For advanced evolution, config types can be registered and versioned via `SchemaRegistry` when pipelines are deployed across multiple services.
- **Plugins**: Plugins can expose additional step functions that accept config parameters, making them reusable across pipelines.
- **Hot Reload**: When combined with hot reload, changing a config file can trigger a pipeline reload with new parameters without restarting the runtime.
