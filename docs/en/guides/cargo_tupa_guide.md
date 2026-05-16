# cargo-tupa

Cargo subcommand for Tupã policy development.

## Installation

```bash
cargo install cargo-tupa
```text

## Commands

### `cargo tupa check`

Validates the `pipeline!` macro expansion and type checking. Runs `cargo check` and filters Tupã-relevant messages.

```bash
cargo tupa check          # check current package
cargo tupa check -v       # verbose
cargo tupa check --manifest-path path/to/Cargo.toml
```text

### `cargo tupa run`

Executes the pipeline defined in the current package with optional JSON input.

```bash
# Use default input (if pipeline supports unit-like input)
cargo tupa run

# Provide JSON input via environment variable
TUPA_INPUT='{"amount":1000.0,"risk_score":0.5}' cargo tupa run

# Enable parallel step execution
TUPA_INPUT='{"x":42}' TUPA_PARALLEL=1 cargo tupa run

# With a file
cargo tupa run --input data.json

# Export step execution metrics as JSON
cargo tupa run --metrics-output metrics.json
```

Your `src/main.rs` should read `TUPA_INPUT` (or use the default) and call `Executor::run` or `Executor::run_parallel`.

**Options:**

- `--input <FILE>` — JSON input file (default: stdin or `TUPA_INPUT`)
- `--parallel` — enable parallel step execution (overrides `TUPA_PARALLEL`)
- `--metrics-output <FILE>` — write per-step metrics JSON (timestamps, state) after execution

### `cargo tupa fmt`

Formats Rust-DSL pipeline code (`pipeline!` blocks) in source files.

```bash
cargo tupa fmt                # format all pipeline code in src/
cargo tupa fmt --dry-run       # show what would change
cargo tupa fmt --check         # fail if any file needs formatting
```

> **Note:** The legacy `.tp` toolchain was removed in v0.9.0. This command operates exclusively on Rust DSL code.

### `cargo tupa lint`

Runs static analysis on Rust-DSL pipeline definitions (`pipeline!` macros).

```bash
cargo tupa lint                # lint current package
cargo tupa lint --json         # machine-readable output
cargo tupa lint --deny warnings # treat warnings as errors
```

> **Note:** The legacy `.tp` toolchain was removed in v0.9.0. This command analyzes Rust DSL code only.

### `cargo tupa test`

Alias for `cargo test --examples`, convenient for running example pipelines and pipeline integration tests.

```bash
cargo tupa test                # run all tests
cargo tupa test --example credit_decision  # test specific example
cargo tupa test -- --nocapture  # pass args through to cargo test
```

### `cargo tupa plugin new`

Generates a new plugin scaffold for custom step functions.

```bash
cargo tupa plugin new my_plugin.rs  # creates my_plugin.rs template
```

This creates a template `my_plugin.rs` exporting:

- `_tupa_plugin_name()`: returns plugin name
- `_tupa_plugin_register(ctx)`: registers step functions
- Sample step function `my_step(input: Value) -> Value`

Build as a cdylib:

```bash
cargo build --crate-type cdylib --release
# target/release/libmy_plugin.so (or .dll/.dylib)
```

Load in your pipeline:

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./target/release/libmy_plugin.so")?;

fn use_plugin(pm: &PluginManager, input: &MyInput) -> Result<Value, String> {
    pm.call("my_step", serde_json::to_value(input)?).map_err(|e| e.to_string())
}
```

### `cargo tupa discover`

Discovers and prints the binary target name for the current Cargo package.

```bash
cargo tupa discover
cargo tupa discover --manifest-path path/to/Cargo.toml
```

The command scans `Cargo.toml` for a `[[bin]]` section with a `name` field. If none is found, it falls back to `src/main.rs` and uses the `package.name` as the binary name.

### `cargo tupa expand`

Expand pipeline! macro to generated Rust code.

```bash
cargo tupa expand --pretty
cargo tupa expand --file src/pipeline.rs
```

**Options:**

- `--pretty` — enable pretty-print (indentation)
- `--file <FILE>` — specific file to expand (default: all src/**/*.rs)

### `cargo tupa bench`

Benchmark a pipeline by running it multiple times and aggregating timing metrics.

```bash
cargo tupa bench                    # run with default iterations
cargo tupa bench --iterations 100   # custom iteration count
cargo tupa bench --metrics          # include step metrics in output
```

**Options:**

- `--iterations <N>` — number of benchmark iterations (default: 10)
- `--metrics` — include per-step metrics from the pipeline run

Output includes total time, average per-step duration, and throughput estimates.

### `cargo tupa watch`

Watch source files for changes and re-run the pipeline automatically.

```bash
cargo tupa watch                    # watch src/**/*.rs for changes
cargo tupa watch --debounce 500     # custom debounce delay in ms
```

**Options:**

- `--debounce <MS>` — delay before re-running after changes (default: 300ms)

## Project Template

Generate a new project from the template:

```bash
cargo generate --git https://github.com/marciopaiva/tupalang#crates/tupa-template
# or local:
cargo generate --path crates/tupa-template
```text

The template includes a sample pipeline, Cargo.toml with dependencies, and a `main.rs` that integrates with `cargo-tupa run`.

## How It Works

- `check`: Delegates to `cargo check` and filters Tupã macro expansion errors.
- `run`: Builds and executes your binary with `TUPA_INPUT` set; your binary calls `Executor::run` or `Executor::run_parallel`.
- `test`: Alias for `cargo test --examples`, runs example pipelines as tests.
- `fmt`: Formats Rust-DSL pipeline code (`pipeline!` blocks) with basic indentation rules.
- `lint`: Performs static analysis on Rust-DSL pipeline definitions (detects duplicate steps, missing names, undefined requires/produces).
- `plugin new`: Generates a plugin template (`_tupa_plugin_name`, `_tupa_plugin_register`, sample step function).
- `discover`: Prints the binary target name from `[[bin]]` or `src/main.rs`.
- `expand`: Expands `pipeline!` macro to generated Rust code (`--pretty` for indented output).
- `bench`: Benchmarks pipeline by running multiple iterations and reporting aggregated timing metrics.
- `watch`: Watches source files and re-runs pipeline on changes with configurable debounce.

## Notes

- Workspace-aware: use `--manifest-path` to point to a specific package.
- Parallel execution: set `TUPA_PARALLEL=1` or pass `--parallel` flag (when implemented).
- Output is printed to stdout; errors to stderr.
