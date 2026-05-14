# Changelog

All notable changes to `tupa-plugin` will be documented in this file.

## [0.9.0] - 2026-05-11

### Added

- Dynamic plugin loading system for extending pipelines with custom step functions
- `PluginManager` struct for loading and managing plugin libraries
- `PluginRegisterContext` ABI for plugin registration
- `create_plugin_template()` helper to generate plugin scaffolding
- `call` method for invoking plugin functions by name with JSON I/O
- FFI-safe extern "C" interface for plugin entry points (`_tupa_plugin_name`, `_tupa_plugin_register`)
- Integration test suite compiling and loading test plugins (Linux/macOS/Windows)

### Notes

- Plugins must be compiled as `cdylib` (e.g., `cargo build --crate-type=cdylib`)
- Plugin functions receive and return `serde_json::Value` via the C ABI
- This crate is alpha; API may change before 1.0
- See `README.md` for complete plugin development guide

### Migration Path

- Legacy `.tp` FFI extensions are handled by `tupa-effects` (deprecated)
- New plugin system is the recommended way to add custom behavior

## [0.9.1] - 2026-05-11

### Added

- Plugin FFI overhead benchmark (`benches/plugin_bench.rs`) measuring `PluginManager::call` latency

### Documentation

- cargo-tupa guide updated with `plugin new` subcommand
- Performance tuning guide added (multi-language)

### Fixed

- Template Cargo.toml pinned dependencies

## [0.9.2] - 2026-05-13

### Changed

- Workspace version alignment to 0.9.2. No functional changes in this crate.

## [0.9.3] - 2026-05-14

### Changed

- Version bump to 0.9.3 with legacy-free workspace
- Ready for publish with `tupa-core` 0.9.3 and `tupa-engine` 0.9.3

## [Unreleased]

- Planned: Plugin hot-reload support
- Planned: Versioned plugin ABI for compatibility
- Planned: Sandboxing/isolation options
