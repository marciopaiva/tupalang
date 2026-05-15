# Versioning Policy

**Effective:** 2026-05-14  
**Applies to:** All Tupã crates (0.9.x series)

---

## Semantic Versioning (SemVer 2.0.0)

All published crates follow SemVer 2.0.0:

- **Major** (X.0.0): Breaking changes
- **Minor** (0.X.0): Backward-compatible features
- **Patch** (0.0.X): Bug fixes

---

## Crate Lifecycle & Version Matrix

### Active Crates (crate-first, alpha)

These are the **currently active crates** in the workspace (0.9.x series). They are published to crates.io and under active development. API may change before 1.0.

| Crate | Version | Target 1.0 | Status |
|---|---|---|---|
| `tupa-core` | 0.9.x | 2026-Q4 | Alpha |
| `tupa-core-macros` | 0.9.x | 2026-Q4 | Alpha (internal) |
| `tupa-engine` | 0.9.x | 2026-Q4 | Alpha |
| `tupa-plugin` | 0.9.x | 2026-Q4 | Alpha |
| `tupa-pyffi` | 0.9.x | TBD | Alpha (unstable) |
| `cargo-tupa` | 0.9.x | 2026-Q4 | Alpha (CLI) |

**Advice:** Pin to a minor version in production until 1.0:

```toml
tupa-core = "0.9"
tupa-engine = "0.9"
tupa-plugin = "0.9"
cargo-tupa = "0.9"
```

---

### Removed Crates (pre-0.9.0)

These crates were **removed** from the workspace in 0.9.0. They are no longer maintained or published:

| Crate | Last Release | Reason |
|---|---|---|
| `tupa-parser` | 0.8.x | Replaced by `pipeline!` macro in `tupa-core` |
| `tupa-lexer` | 0.8.x | No longer needed |
| `tupa-typecheck` | 0.8.x | Integrated into macro expansion |
| `tupa-codegen` | 0.8.x | LLVM backend removed |
| `tupa-runtime` (old) | 0.8.x | Merged into `tupa-engine` |
| `tupa-cli` | 0.8.x | Replaced by `cargo-tupa` |
| `tupa-fmt` | 0.8.x | Legacy `.tp` formatter removed |
| `tupa-lint` | 0.8.x | Legacy `.tp` linter removed |
| `tupa-audit` | 0.8.x | Audit integrated into engine |
| `tupa-conformance` | 0.8.x | SPEC validator folded into CI |
| `tupa-effects` | 0.8.x | Effect system merged into core |
| `tupa-sys` | — | C ABI not yet implemented |
| `tupa-lsp` | — | Language server never released |
| `tupa-ad` | — | Auto-diff (planned for future) |

These crates are **not available** in the current workspace. Do not depend on them for new projects.

---

## Deprecation Process (Future)

If a feature or crate must be deprecated in the future:

1. Announce in CHANGELOG and docs
2. Mark with `#[deprecated]` attribute (if in code)
3. Provide migration path
4. Keep functional for at least one minor version
5. Remove only in a major version bump (1.0 → 2.0)

---

## Stability Guarantees

- Before 1.0: **No stability guarantees**. Breaking changes may occur in minor releases. Migration guidance will be provided in CHANGELOG.
- After 1.0: **Full SemVer**. Breaking changes only in major versions; deprecations announced at least one major version ahead.

See individual crate CHANGELOGs for specific breaking changes.
