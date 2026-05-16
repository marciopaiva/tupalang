# {{crate_name}}

A Tupã policy pipeline project.

## Installation

```bash
# Install cargo-tupa CLI
cargo install cargo-tupa

# Clone this template and work with it
git clone <repo-url>
cd {{crate_name}}
```

## Usage

```bash
# Build and typecheck
cargo tupa check

# Run with default input
cargo tupa run

# Run with custom JSON input
TUPA_INPUT='{"amount":5000.0,"risk_score":0.3}' cargo tupa run

# Run with parallel execution
TUPA_INPUT='{"amount":1000.0,"risk_score":0.1}' TUPA_PARALLEL=1 cargo tupa run
```

## Development

Edit `src/lib.rs` to modify the pipeline definition. The `pipeline!` macro provides:

- Compile-time type checking for inputs and step signatures
- Automatic `Executor::run` (sequential) and `Executor::run_parallel` (concurrent) implementations
- Metric collection and constraint validation

### Input Format

The pipeline expects a JSON object matching the `Input` struct:

```json
{
  "amount": 1500.0,
  "risk_score": 0.45
}
```

### Output

**Success:**
```
✅ Pipeline passed all constraints
score_val = 45.0
enriched = {"amount":1500.0,"risk_score":0.45}
```

**Constraint failure:**
```
❌ Pipeline failed constraints:
  - score_val: expected >= 10.0, got 5.0
```

## License

Apache-2.0

## Links

- [Tupã Documentation](https://github.com/marciopaiva/tupalang)
- [cargo-tupa](https://crates.io/crates/cargo-tupa)