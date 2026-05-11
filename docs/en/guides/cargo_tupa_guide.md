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

### `cargo tupa fmt` (future)

Formats legacy `.tp` files.

### `cargo tupa lint` (future)

Runs static analysis on pipeline definitions.

## Project Template

Generate a new project from the template:

```bash
cargo generate --git https://github.com/marciopaiva/tupalang#crates/tupa-template
# or local:
cargo generate --path crates/tupa-template
```text

The template includes a sample pipeline, Cargo.toml with dependencies, and a `main.rs` that integrates with `cargo-tupa run`.

## How It Works

- `check`: Delegates to `cargo check --message-format=json` and surfaces Tupã macro errors.
- `run`: Builds and executes your binary with `TUPA_INPUT` set, enabling quick iteration.
- Future subcommands will leverage `tupa-lint` and `tupa-fmt`.

## Notes

- Workspace-aware: use `--manifest-path` to point to a specific package.
- Parallel execution: set `TUPA_PARALLEL=1` or pass `--parallel` flag (when implemented).
- Output is printed to stdout; errors to stderr.
