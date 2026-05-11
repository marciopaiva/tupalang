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
```text

Your `src/main.rs` should read `TUPA_INPUT` (or use the default) and call `Executor::run` or `Executor::run_parallel`.

### `cargo tupa fmt`

Formats legacy `.tp` pipeline files using the Tupã formatter.

```bash
cargo tupa fmt                # format all .tp files in the package
cargo tupa fmt --dry-run       # show what would change
cargo tupa fmt --check         # fail if any file needs formatting
```

### `cargo tupa lint`

Runs static analysis on pipeline definitions (both `.tp` and Rust DSL).

```bash
cargo tupa lint                # lint current package
cargo tupa lint --json         # machine-readable output
cargo tupa lint --deny warnings # treat warnings as errors
```

### `cargo tupa test`

Runs pipeline unit tests and example validations.

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
cargo build --crate-type=cdylib --release
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

## Project Template

Generate a new project from the template:

```bash
cargo generate --git https://github.com/marciopaiva/tupalang#crates/tupa-template
# or local:
cargo generate --path crates/tupa-template
```text

The template includes a sample pipeline, Cargo.toml with dependencies, and a `main.rs` that integrates with `cargo-tupa run`.

## How It Works

- `check`: Delegates to `cargo check --message-format=json`, filters Tupã macro errors.
- `run`: Builds and executes your binary with `TUPA_INPUT` set; your binary calls `Executor::run` or `Executor::run_parallel`.
- `test`: Runs `cargo test --examples` to validate pipeline unit tests.
- `fmt`: Calls `tupa-fmt` on `.tp` files; uses `rustfmt` for Rust DSL.
- `lint`: Runs `tupa-lint` on legacy files and surfaces Rust warnings.
- `plugin new`: Generates a plugin template (`_tupa_plugin_name`, `_tupa_plugin_register`, sample step function).

## Notes

- Workspace-aware: use `--manifest-path` to point to a specific package.
- Parallel execution: set `TUPA_PARALLEL=1` or pass `--parallel` flag (when implemented).
- Output is printed to stdout; errors to stderr.
