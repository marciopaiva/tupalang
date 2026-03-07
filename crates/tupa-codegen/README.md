# tupa-codegen

Transforms typed AST into executable artifacts (stubs and execution plans).

## Usage

```rust
use tupa_parser::parse_program;
use tupa_codegen::execution_plan::codegen_pipeline;

let src = r#"pipeline P { input: string, steps: [], output: string }"#;
let program = parse_program(src)?;
let pipeline = program.items.iter().find_map(|i| match i { tupa_parser::Item::Pipeline(p) => Some(p), _ => None }).unwrap();
let plan_json = codegen_pipeline("demo", pipeline, &program)?;
println!("{}", plan_json);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)

