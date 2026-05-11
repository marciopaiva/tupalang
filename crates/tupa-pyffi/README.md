# tupa-pyffi

⚠️ **EARLY PHASE — NOT YET COMPATIBLE with `tupa-core`.** This crate currently targets the legacy `.tp` toolchain. It will be migrated to the new crate-first architecture in Phase 3 (FFI).

## Purpose

Python FFI bridge for calling external Python functions from Tupã pipelines (planned). Enables interoperability with Python libraries (NumPy, PyTorch, TensorFlow).

**Current status:** Prototype for legacy `.tp` pipelines only.

## Future (Phase 3)

Will provide:

- Python bindings to `tupa-core` DSL
- Ability to embed Python functions as pipeline steps
- `pip install tupa-pyffi` distribution

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
