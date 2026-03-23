# tupa-codegen

Transforms typed AST into executable artifacts (stubs and execution plans).

## Usage

```rust
use tupa_parser::parse_program;
use tupa_codegen::execution_plan::codegen_pipeline;
use tupa_typecheck::typecheck_program;

let src = r#"pipeline P { input: string, steps: [], output: string }"#;
let program = parse_program(src)?;
typecheck_program(&program)?;
let pipeline = program.items.iter().find_map(|i| match i { tupa_parser::Item::Pipeline(p) => Some(p), _ => None }).unwrap();
let plan_json = codegen_pipeline("demo", pipeline, &program)?;
println!("{}", plan_json);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Run `tupa-typecheck` before generating plans so the execution plan is produced from a validated program.

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-codegen` in-process to build execution plans for strategy and analyst pipelines that run inside Rust services.
