# Changelog

All notable changes to `tupa-pyffi` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-05-11

### Added

- Python FFI bindings for Tupã (`pyo3`-based)
- `call_python_function` utility to invoke Python functions from Rust pipelines
- Support for NumPy, PyTorch, TensorFlow via Python interpreter
- Auto-initialization of Python GIL with `pyo3` features

### Notes

- This crate enables interoperability with Python ML/data science ecosystems.
- Requires Python development headers installed on the system.
- API subject to change before 1.0.

## [0.9.1] - 2026-05-11

### Documentation

- Updated to reflect workspace documentation changes (cargo-tupa guide, performance tuning)

### Fixed

- Template pinned dependency fix (workspace-wide)

## [0.9.2] - 2026-05-13

### Changed

- Workspace version alignment to 0.9.2. No functional changes in this crate.

## [0.9.3] - 2026-05-14

### Changed

- Bump to 0.9.3 with clean workspace (no legacy deps)
- Fixed Cargo.toml format (ensured [dependencies] section present)

## [0.9.6] - 2026-06-06

### Changed

- Version bump to 0.9.6. No functional changes; this release focuses on removing the legacy `.tp` toolchain artifacts from the repository.

## [0.9.5] - 2026-05-16

### Added

- **`call_with_multiple_args`**: Support for multi-argument Python function calls from pipelines
- **`reset_python_bridge`**: Global state reset capability for Python GIL management
- **Extended type support**: `i32`, `u64`, `u32`, `f32`, `Vec<u8>`, `Vec<Value>` conversions via `FromPython` trait

## [0.9.4] - 2026-05-14

### Changed

- Version bump to 0.9.4. No functional changes.

## [Unreleased]

No unreleased changes.
