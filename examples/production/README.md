# Production Example — MNIST (Legacy `.tp`)

**⚠️ DEPRECATED:** This example uses the legacy `.tp` toolchain (`tupa-cli`) which was removed in Tupã 0.9.0. It is kept for historical reference only.

## Legacy Setup (not applicable to Rust-DSL)

**Prerequisites (old):**

- Rust (latest stable)
- Python 3.8+
- Tupã Compiler (built from source) — no longer available

## Legacy Files

- `mnist_pipeline.tp` — Tupã pipeline definition (legacy syntax)
- `mnist_cnn_model.py` — PyTorch model
- `mnist_utils.py` — helper functions
- `mnist_sample.json` — sample input

## Legacy Command (do not use)

```bash
source .venv/bin/activate
export PYTHONPATH=$PYTHONPATH:$(pwd)
cargo run -p tupa-cli -- run --pipeline MNISTAudit --input mnist_sample.json mnist_pipeline.tp
```

## Modern Equivalent (Rust-DSL)

With Tupã 0.9+, the same pattern is expressed as a Rust crate using the `pipeline!` macro. Python integration is achieved via `tupa-pyffi` (in development) or by registering Python functions as step plugins.

See:

- `crates/tupa-engine/examples/minimal.rs` for pipeline structure
- `crates/tupa-pyffi/` for Python binding examples (when available)
- [docs/en/guides/pipeline_guide.md](../docs/en/guides/pipeline_guide.md) for step function patterns

**Migration:** Legacy `.tp` pipelines must be manually converted to Rust-DSL; no automatic migration tool exists yet. See [docs/en/TRANSITION.md](../docs/en/TRANSITION.md) for guidance.
