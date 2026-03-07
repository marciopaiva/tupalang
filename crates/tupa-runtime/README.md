# tupa-runtime

Execution engine for TupaLang pipelines.

## Usage

```rust
use serde_json::json;
use tupa_runtime::Runtime;

let runtime = Runtime::new();
runtime.register_step("demo::step_echo", |state| Ok(state));
let _ = json!({"ok": true});
```

Use this crate together with execution plans produced by `tupa-codegen`.

## Crate

- Source: https://github.com/marciopaiva/tupalang