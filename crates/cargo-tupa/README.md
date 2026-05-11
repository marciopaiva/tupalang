# cargo-tupa

**Cargo subcommand for Tupã pipelines** — typecheck, format, lint, test, and execute your policies.

## Installation

```bash
cargo install cargo-tupa
```text

After installation, `cargo tupa` will be available as a Cargo subcommand.

## Quick Start

### 1. Create a new project from template

```bash
cargo generate --git https://github.com/marciopaiva/tupalang --alias tupa-template
# or locally:
cargo generate --path crates/tupa-template
```text

### 2. Develop your pipeline

Edit `src/lib.rs`:

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;

pipeline! {
    name: MyPolicy,
    input: Input,
    steps: [
        step("enrich") { enrich(input) } produces ["enriched"],
        step("score")  { score(input) }  requires ["enriched"]
    ],
    constraints: [
        metric("score").ge(0.0)
    ]
}
```text

### 3. Build & Check

```bash
cargo tupa check        # typecheck pipeline macro
cargo tupa lint         # lint for issues
cargo tupa test         # run example tests (includes pipeline assertions)
```text

### 4. Run

```bash
# With a binary target (default)
cargo tupa run

# With a specific example
cargo tupa run --example minimal

# With a specific binary target
cargo tupa run --bin my-pipeline

# With JSON input from environment variable
TUPA_INPUT='{"amount":1000.0,"risk_score":0.5}' cargo tupa run

# With JSON input from file
cargo tupa run --input data.json

# With JSON from stdin
echo '{"x":42}' | cargo tupa run --example minimal

# Parallel execution ( Tokio runtime required in your binary )
TUPA_PARALLEL=1 cargo tupa run --example fraud_complete
```text

Your `src/main.rs` (or example binary) should read the `TUPA_INPUT` environment variable:

```rust
use std::env;
let input: Input = if let Ok(json) = env::var("TUPA_INPUT") {
    serde_json::from_str(&json).unwrap_or_else(|_| Input::default())
} else {
    Input::default()
};
```text

**Or** read from stdin if `TUPA_INPUT` not set:

```rust
let mut buffer = String::new();
std::io::stdin().read_to_string(&mut buffer).ok();
let input: Input = if buffer.is_empty() {
    Input::default()
} else {
    serde_json::from_str(&buffer).unwrap()
};
```text

## Subcommands

| Command | Description |
|---------|-------------|
| `cargo tupa check` | Validate the `pipeline!` macro (runs `cargo check`) |
| `cargo tupa run` | Execute a pipeline binary or example with JSON input |
| `cargo tupa test` | Run tests (including example pipelines) |
| `cargo tupa lint` | Lint `.tp` legacy files and surface Rust warnings |
| `cargo tupa fmt` | Format legacy `.tp` source files |
| `cargo tupa plugin new [FILENAME]` | Generate a plugin scaffold for dynamic step functions |

## Run Examples (Workspace)

```bash
# From workspace root, run an example from tupa-engine
cargo tupa run --example minimal -m crates/tupa-engine/Cargo.toml

# With input and parallel mode
cargo tupa run --example credit_decision -m crates/tupa-engine/Cargo.toml --parallel

# Test all examples
cargo tupa test -m crates/tupa-engine/Cargo.toml

# Check the engine examples (Rust DSL)
cargo tupa check -m crates/tupa-engine/Cargo.toml
```text

## How It Works

- **check**: Delegates to `cargo check --message-format=json`, filters Tupã macro errors.
- **run**: Spawns `cargo run --example` or `--bin` with `TUPA_INPUT` set; your binary calls `Executor::run` or `Executor::run_parallel`. Reads stdin if `--input` not provided.
- **test**: Runs `cargo test --examples` to validate example pipelines.
- **lint**: Scans for `.tp` files and runs `tupa-lint`; also shows `cargo check` warnings.
- **fmt**: Calls `tupa-fmt` on all `.tp` files found in the project.

## Project Template

The `tupa-template` crate provides a complete scaffold:

```bash
cargo generate --path crates/tupa-template
```text

Includes:

- `src/lib.rs` with a sample pipeline
- `src/main.rs` integrating with `cargo-tupa run` and `TUPA_INPUT`
- `Cargo.toml` with dependencies: `tupa-core`, `tupa-engine`, `serde`, `tokio`
- README with usage instructions

## Notes

- `cargo tupa` works on the current Cargo package; use `-m/--manifest-path` to point elsewhere.
- Parallel execution requires Tokio runtime (`#[tokio::main]`) and step-level `produces`/`requires` annotations.
- For `.tp` legacy files, use `cargo tupa fmt` and `cargo tupa lint` (migration to Rust DSL recommended).
- `cargo tupa run --input <file>` reads JSON from file; falls back to `TUPA_INPUT` then stdin.
- `cargo tupa test` runs `cargo test --examples`, ensuring pipeline unit tests pass.

## Examples

```bash
# Check the engine examples (Rust DSL)
cargo tupa check -m crates/tupa-engine/Cargo.toml

# Run fraud_complete example with custom input and parallel mode
cargo tupa run --example fraud_complete -m crates/tupa-engine/Cargo.toml --parallel

# Lint all .tp files in workspace
cargo tupa lint -m crates/tupa-engine/Cargo.toml
```text

## Plugin Development

Create a new dynamic plugin scaffold:

```bash
cargo tupa plugin new my_plugin.rs
```text

This generates a template `my_plugin.rs` that exports two C symbols (`_tupa_plugin_name`, `_tupa_plugin_register`) and a step function `my_step`. Build it as a cdylib:

```bash
cargo build --crate-type=cdylib --release
# produces target/release/libmy_plugin.so (or .dll/.dylib)
```text

Load the plugin in your pipeline using `tupa_plugin::PluginManager`:

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./target/release/libmy_plugin.so")?;

// In a step:
fn use_plugin(pm: &PluginManager, input: &MyInput) -> Result<serde_json::Value, String> {
    pm.call("my_step", serde_json::to_value(input)?).map_err(|e| e.to_string())
}

pipeline! {
    name: MyPipeline,
    input: MyInput,
    steps: [
        step("plugin") { use_plugin(&pm, input)? }
    ],
    constraints: []
}
```text

For more details, see the `tupa-plugin` crate documentation.

---

*0.9.0 — API may change before 1.0.*
