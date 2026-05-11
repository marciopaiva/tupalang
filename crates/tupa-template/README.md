# {{crate_name}}

A Tupã policy pipeline project.

## Setup

```bash
# If you haven't already, install cargo-tupa
cargo install cargo-tupa

# Build and typecheck
cargo tupa check

# Run with default input
cargo tupa run

# Run with custom JSON input
TUPA_INPUT='{"amount":5000.0,"risk_score":0.3}' cargo tupa run

# Enable parallel execution
TUPA_INPUT='{"amount":1000.0,"risk_score":0.1}' TUPA_PARALLEL=1 cargo tupa run
```text

## Development

Edit `src/lib.rs` to modify the pipeline definition. The `pipeline!` macro provides compile-time type checking and auto-implements both sequential (`Executor::run`) and parallel (`Executor::run_parallel`) execution.

## Input format

The pipeline expects a JSON object matching the `Input` struct in `src/lib.rs`. Example:

```json
{
  "amount": 1500.0,
  "risk_score": 0.45
}
```text

## Output

On success:

```text
✅ Pipeline passed all constraints
score_val = 45.0
enriched = {"amount":1500.0,"risk_score":0.45}
```text

On constraint failure:

```text
❌ Pipeline failed constraints:
  - score_val: expected >= 10.0, got 5.0
```text

## License

Apache-2.0
