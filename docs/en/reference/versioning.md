# Versioning Policy

**Effective:** 2026-05-10  
**Applies to:** All Tupã crates

---

## Semantic Versioning (SemVer 2.0.0)

All published crates follow SemVer 2.0.0:

- **Major** (X.0.0): Breaking changes
- **Minor** (0.X.0): Backward-compatible features
- **Patch** (0.0.X): Bug fixes

---

## Crate Lifecycle & Version Matrix

### New Cores (crate-first, alpha)

These crates are **new implementations** using Rust DSL approach. They start at 0.9.0 and will reach 1.0.0 when stable.

| Crate | Initial Ver | Target 1.0 | Breaking Change Policy (pre-1.0) |
|---|---|---|---|
| `tupa-core` | 0.9.0 | 2026-Q4 | Minor releases may break compat; migration guide in CHANGELOG |
| `tupa-core-macros` | 0.9.0 | 2026-Q4 | Same |
| `tupa-engine` | 0.9.0 | 2026-Q4 | Same |
| `tupa-plugin` | 0.9.0 | 2026-Q4 | Same |
| `cargo-tupa` | 0.9.0 | 2026-Q4 | Same |

**Advice:** Pin to a minor version in production until 1.0:

```toml
tupa-core = "0.9"
tupa-engine = "0.9"
tupa-plugin = "0.9"
cargo-tupa = "0.9"
```text

---

### Active Support Crates (stable, will align to 1.0)

These crates already exist and are stable. They will receive bug fixes and minor features, eventually converging to 1.0 alongside `tupa-core`.

| Crate | Current Ver | 1.0 Target | Action |
|---|---|---|---|
| `tupa-audit` | 0.8.x | bump to 1.0.0 | Maintained |
| `tupa-conformance` | 0.8.x | 1.0.0 | Maintained |
| `tupa-fmt` | 0.8.x | 1.0.0 | Maintained (legacy `.tp` only) |
| `tupa-lint` | 0.8.x | 1.0.0 | Maintained; will add DSL rules |
| `tupa-pyffi` | 0.8.x | 1.0.0 | Requires migration to `tupa-core` (Phase 3) |

**Guarantee:** No breaking changes within same major version. Minor releases add features backward-compatibly.

---

### Deprecated Crates (legacy, EOL 2027)

These crates implement the old standalone `.tp` compiler pipeline. They are **frozen** — no new features, security fixes only until 2027-01-01. After that they will be removed from the workspace and crates.io.

| Crate | Last Release | Reason |
|---|---|---|
| `tupa-parser` | 0.8.x | Replaced by `tupa-core` macro |
| `tupa-typecheck` | 0.8.x | Integrated into macro expansion |
| `tupa-lexer` | 0.8.x | Parser dependency; no longer needed for new code |
| `tupa-codegen` | 0.8.x | LLVM backend dead; executor interprets |
| `tupa-runtime` | 0.8.x | Merged into `tupa-engine` |
| `tupa-effects` | 0.8.x | Merged into `tupa-core` |
| `tupa-cli` | 0.8.x | Replaced by `cargo tupa` wrapper (not yet implemented) |

**Warning:** Do not start new projects with these crates. They exist only for maintaining existing `.tp` codebases until 2027.

---

## Deprecation Process

For any feature or entire crate:

1. **Announce** in `CHANGELOG.md` with `Deprecated` tag.
2. **Emit warning** at compile time (Rust APIs) or runtime (CLI) when feature is used.
3. **Document** migration path in next minor release notes.
4. **Remove** after:
    - **Pre-1.0:** at least one minor version (e.g., deprecated in 0.9.0, removed in 0.10.0)
    - **Post-1.0:** at least 6 months

Deprecated crates will receive security patches only until EOL date, then removed from crates.io.

---

## Compatibility Guarantees

### Pre-1.0 (0.x)

Between 0.9.0 and 1.0.0:

- Minor releases (0.1 → 0.2, 0.2 → 0.3) **may** contain breaking changes to the DSL macro or executor API.
- Each minor release includes a **migration guide** in its `CHANGELOG.md`.
- Patch releases (0.1.1, 0.1.2) are backward-compatible within the same minor series.

### Post-1.0

After a crate reaches 1.0.0:

- Major version reserved for breaking changes.
- At least **6 months deprecation cycle** for any removed feature.
- Migration guide required for every major bump.

---

## Release Cadence

- **`tupa-core` / `tupa-engine` (alpha):** biweekly patch, monthly minor until stable.
- **Active support crates:** sync with core major versions; minor releases as needed.
- **Deprecated crates:** security-only, as-needed.

---

## Changelog Format

Each crate has a `CHANGELOG.md` following [keepachangelog.com](https://keepachangelog.com/) format:

- `Added` (new feature)
- `Changed` (modification to existing feature)
- `Deprecated` (feature slated for removal)
- `Removed` (feature removed)
- `Fixed` (bug fix)
- `Security` (vulnerability fix)

---

## See Also

- [Adoption Plan](../governance/adoption_plan.md) — milestones to 1.0
- [Roadmap](../releases/roadmap.md) — timeline
- [Compatibility Guide](../reference/compatibility.md) — platform support
