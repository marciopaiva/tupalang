# tupa-runtime

Execution engine for TupaLang pipelines.

## Features

- Built-in step functions: `tupa::weighted`, `tupa::warn`, `tupa::pass`, `tupa::confirm`, `tupa::cooldown`
- Custom extension API via `TupaExtension` trait
- Hot reload support via `watch_and_reload()` (requires `hot-reload` feature)
- Async pipeline execution with structured audit output

## Usage

```rust
use serde_json::json;
use tupa_codegen::execution_plan::{ExecutionPlan, TypeSchema, StepPlan};
use tupa_runtime::{Runtime, TupaExtension};

// Create runtime and register extensions
let runtime = Runtime::new();
MyExtensions.register(&runtime);

let plan = ExecutionPlan {
    name: "demo".into(),
    version: "0.8.2".into(),
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

## Extension Example

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MyHelpers;
impl TupaExtension for MyHelpers {
    fn name(&self) -> &str { "my_project" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("my::custom", |input| {
            // custom logic
            Ok(input)
        });
    }
}
```

## Hot Reload

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// Receiver yields () on file changes; call reload_pipeline() to apply
```

Enable with feature flag:

```bash
cargo add tupa-runtime --features hot-reload
```

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-runtime` as an embedded execution engine inside the strategy and AI analyst services, rather than spawning `tupa run` per event.
