# Installation

## For Rust Projects (Recommended)

Add Tupã crates to your `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.2"      # DSL macros and policy types
tupa-engine = "0.2"    # Pipeline executor
```

Run:

```bash
cargo build
```

That's it — there's **no separate toolchain to install**. The crates integrate directly into your Rust build.

**Minimum Rust version:** 1.83

---

## Verify

Create `src/lib.rs`:

```rust
use tupa_core::pipeline;

pipeline! {
    name: Hello,
    input: (),
    steps: [
        step("hello") { println!("Hello, Tupã!") }
    ],
    constraints: []
}
```

Build:

```bash
cargo check
```

Should succeed with no errors.

---

## Optional: Standalone CLI (Legacy)

The `tupa` binary is still available for:

- Validating legacy `.tp` files
- One-off checks via CI
- Migration tooling (future)

### Install binary

```bash
# Linux/macOS
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-linux-x86_64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa

# macOS (Apple Silicon)
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-macos-aarch64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa
```

### Install via Cargo (legacy CLI)

```bash
cargo install tupa-cli
tupa --help
```

**Note:** The CLI does not support the new `pipeline!` DSL directly. Use `cargo check` for that. The CLI is only for `.tp` files (deprecated).

---

## Which Path Should I Choose?

| Use case | Choose |
|---|---|
| New Rust application needing policy/strategy logic | **Crates** (`tupa-core`, `tupa-engine`) |
| Existing ViperTrade project migrating from `.tp` | **Crates** (see [Transition Guide](../TRANSITION.md)) |
- One-off validation of a `.tp` file | **Standalone CLI** (`tupa check file.tp`) |
- CI pipeline that checks legacy `.tp` files | **Standalone CLI** installed via curl or `cargo install tupa-cli` |
- Building a new non-Rust system that needs Tupã | **FFI** via `tupa-sys` (coming in Phase 3) |

**Bottom line:** New Rust projects should **never** use the standalone compiler. The crates are the primary delivery mechanism.

---

## Uninstallation

To remove the standalone binary:

```bash
rm /usr/local/bin/tupa   # if installed via curl
# or
cargo uninstall tupa-cli  # if installed via cargo
```

Crates are removed like any Rust dependency: delete from `Cargo.toml` and `cargo update`.

---

## Version Matrix

| `tupa-core` | `tupa-engine` | Rust MSRV | Notes |
|---|---|---|---|
| 0.2.x | 0.2.x | 1.83 | Current stable |
| 0.1.x | 0.1.x | 1.75 | Legacy (pre-crate-first) |

Always use matching major versions (SemVer).

---

## Next

- [Getting Started](getting_started.md) — write and run your first pipeline
- [Pipeline Guide](pipeline_guide.md) — advanced features (async, plugins, tensors)
- [Transition Guide](../TRANSITION.md) — migrating from `.tp` files
