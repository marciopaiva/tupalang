# tupa-runtime

Execution engine for TupaLang pipelines.

## Usage

```rust
use serde_json::json;
use tupa_codegen::execution_plan::{ExecutionPlan, TypeSchema, StepPlan};
use tupa_runtime::Runtime;

let runtime = Runtime::new();
runtime.register_step("demo::step_echo", |state| Ok(state));

let plan = ExecutionPlan {
    name: "demo".into(),
    version: "0.8.1".into(),
    seed: None,
    input_schema: TypeSchema {
        kind: "string".into(),
        elem: None,
        fields: None,
        len: None,
        name: None,
        tensor_shape: None,
        tensor_dtype: None,
    },
    output_schema: None,
    steps: vec![StepPlan {
        name: "echo".into(),
        function_ref: "demo::step_echo".into(),
        effects: vec![],
    }],
    constraints: vec![],
    metrics: Default::default(),
    metric_plans: vec![],
};

# tokio_test::block_on(async {
let output = runtime.run_pipeline_async(&plan, json!("hello")).await?;
assert_eq!(output, json!("hello"));
# Ok::<(), tupa_runtime::RuntimeError>(())
# });
```

Use this crate together with validated execution plans produced by `tupa-codegen`.

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
