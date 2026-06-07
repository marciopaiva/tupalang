# Installation

## For Rust Projects (Recommended)

Add Tupã crates to your `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.10"      # DSL macros and policy types
tupa-engine = "0.10"    # Pipeline executor
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

## CLI (cargo-tupa)

The `cargo tupa` command provides subcommands for working with Tupã pipelines:

```bash
cargo install cargo-tupa   # optional, if you want the CLI globally

# In any Tupã project:
cargo tupa check           # type-check pipeline (if applicable)
cargo tupa run             # run pipeline with JSON input
cargo tupa fmt             # format pipeline! blocks
cargo tupa lint            # lint for common issues
cargo tupa discover        # auto-detect binary target
```

Note: `cargo tupa` is **optional** — your project builds without it. It is a developer convenience wrapper around the crates.

---

## Version Matrix

Always use matching major versions (SemVer):

| `tupa-core` | `tupa-engine` | Rust MSRV | Notes |
|---|---|---|---|
| 0.9.x | 0.9.x | 1.83 | Current (Rust-DSL only) |
| 0.8.x | 0.8.x | 1.75 | Legacy (standalone `.tp` toolchain) — EOL |

The 0.9.x series is the Rust-DSL era. Legacy 0.8.x and earlier are no longer supported.
