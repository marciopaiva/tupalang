# RFC 0001 — From Language Spec to Crate Package

- **Status:** Draft (for review)
- **Created:** 2026-06-06
- **Audience:** Tupã maintainers
- **Related:** `docs/en/reference/spec.md`, `docs/en/PROPOSAL.md`, `docs/en/releases/roadmap.md`

## Summary

Tupã began as a standalone `.tp` language with its own compiler. Since 0.9.0 it
is **crate-first**: a set of Rust crates centered on the `pipeline!` macro. This
RFC proposes to finish that transition at the level of the *specification and
positioning*:

1. **Reposition Tupã as a package** (a suite of Rust crates you add to a project),
   not a new programming language.
2. **Salvage the valuable, language-agnostic ideas from the spec** — the
   *type-level guarantees* — and implement them as idiomatic Rust crates.
3. **Retire the compiler-machinery parts of the spec** (lexer, EBNF, expression
   grammar, standalone codegen/FFI), which Rust already provides.

Phase 0 of (2) has shipped as an experimental proof of concept in 0.10.0: real
`Safe<T, C>` constraints and a `safe!` macro (see `tupa-core`).

## Motivation

The current `spec.md` is a ~1000-line specification of a *language* (lexical
structure, grammar, statements, expressions, modules, FFI, gradients). But the
implementation is a small embedded DSL: `pipeline! { steps, constraints }` over
ordinary Rust. The gap creates two problems:

- **Inaccuracy:** the spec describes features the crates do not implement
  (shaped/sparse tensors, `∇`, `extern "C"`, a module system).
- **Mispositioning:** users think they must learn/install a language, when they
  only need `cargo add tupa-core tupa-engine`.

The differentiator of Tupã is **not** a new syntax — it is **auditable,
compile-time-checked policy with type-level guarantees**. That part is worth
keeping and is expressible in idiomatic Rust.

## Positioning change

> **Before:** "Tupã is a typed policy & strategy language for critical systems."
>
> **After:** "Tupã is a Rust crate suite for typed, auditable policy pipelines —
> compile-time guarantees, no separate toolchain. `cargo add tupa-core`."

Concretely, prefer "package / crate suite / DSL (embedded in Rust)" over
"language" in: `README.md`, `docs/*/index.md`, `spec.md` §1, and crate
descriptions. (Most of this framing already landed in 0.10.0; this RFC records the
intent so it stays consistent.)

## What to salvage vs. retire

| Spec area | Decision | Rationale / Rust mechanism |
|---|---|---|
| `Safe<T, !constraint>` (alignment types) | **Implement** | Marker traits + `safe!` proc-macro (const proof) + typestate. **PoC shipped.** |
| `Tensor<T, shape, density>` | **Implement** | Const generics (`Tensor<f32, 28, 28>`) and/or wrap `ndarray`; density as `Dense`/`Sparse` marker. |
| Constraint / interval solver | **Implement** | Port the old `tupa-typecheck` literal analysis into the proc-macro. **PoC: f64 const-fold.** |
| Diagnostics with stable codes (`E2001`, `E3002`) | **Implement** | proc-macro errors + a code registry in `tupa-lints`. |
| Effects / `@deterministic` | **Consider** | Typestate `Pure`/`Effectful`; engine already enforces determinism. |
| Differentiability (`∇`) | **Optional** | `tupa-autodiff` via dual numbers (forward-mode) or integrate `candle`; **do not** rebuild LLVM codegen. |
| Lexer (§2), Expressions (§4), Statements (§5), EBNF (§8) | **Retire** | The expression/statement language is now Rust. |
| Modules & FFI (§7: `import`/`export`/`extern "C"`) | **Retire** | Rust modules + native FFI + `tupa-pyffi`. |
| Standalone codegen / LLVM / native binary (§9.1) | **Retire** | Compilation is `rustc`. |
| Primitives, tuples, enums, `Option`/`Result` (§3.1–3.2.4) | **Retire** | Rust std already provides these. |

## Roadmap

### Phase 0 — Alignment types PoC ✅ (0.10.0, experimental)

- `Constraint<T>` + `ConstraintError`; markers `NonNan`/`NonInf`/`Finite`.
- `Safe::try_new` / `Safe::new_unchecked`.
- `safe!(Marker, expr)`: compile-time proof for constant `f64` expressions
  (`E3002` on violation), runtime guard otherwise.

### Phase 1 — Stabilize constraints

- More numeric markers (`Positive`, `InRange<…>` via const generics) and an
  `all!`/composition combinator (`Safe<f64, (NonNan, NonInf)>`).
- **String/content constraints** (`!hate_speech`, `!misinformation`) via a
  pluggable `Validator` trait — runtime scorer, optionally backed by `tupa-pyffi`
  (RLHF model). No false compile-time promises: these are runtime-checked.
- Integrate `Safe` into `pipeline!` step outputs and constraints.
- Align diagnostic codes with `spec.md` §11; host the registry in `tupa-lints`.

### Phase 2 — Shaped / sparse tensors

- `Tensor<T, const R: usize, const C: usize>` with compile-time shape checks
  (e.g. `matmul` dimension agreement); `Dense`/`Sparse` density marker.
- Optional `ndarray` interop behind a feature.

### Phase 3 — Effects & determinism (optional)

- Typestate for pure vs. effectful steps; tie into the engine's determinism
  constraint and `tupa-pyffi` (external calls are effectful).

### Phase 4 — Autodiff (optional, headline feature)

- `tupa-autodiff`: forward-mode dual numbers for pure `f64`/`Tensor` functions;
  a `grad!` macro. Evaluate `candle`/`std::autodiff` before building bespoke.

## Spec disposition

- Reduce `spec.md` to a **DSL reference**: the `pipeline!` grammar (derived from
  `tupa-core-macros`), the type guarantees (`Safe`, `Tensor`), constraint
  semantics, executor semantics, and diagnostics.
- Move the standalone-language sections (lexer, full EBNF, expressions,
  statements, modules/FFI, gradient codegen) to `docs/*/archive/` as historical.
- Keep `grammar.ebnf` as a historical artifact of the `.tp` grammar.

## Versioning

The Phase 0 additions are additive and backward-compatible but introduce new
public API. They ship in **0.10.0** (a feature minor over 0.9.5), together with
the repository cleanup and crate-distribution reframing. The constraint API is
marked experimental until Phase 1 stabilizes it.

## Open questions

1. Should string/content constraints live in `tupa-core` (behind a feature) or a
   separate `tupa-align` crate?
2. Do we want shaped tensors in `tupa-core`, or a dedicated `tupa-tensor` crate?
3. Is autodiff in scope for 1.0, or post-1.0?
4. Archive vs. delete for the retired spec sections?

## Appendix — Phase 0 example

```rust
use tupa_core::{safe, constraints::NonNan, Safe};

// Proven at compile time (constant expression):
let ok = safe!(NonNan, 1.0 + 2.0);
assert_eq!(ok.get(), 3.0);

// Runtime-checked for dynamic values:
let v: f64 = "0.5".parse().unwrap();
let s = Safe::<f64, NonNan>::try_new(v).expect("must not be NaN");

// A constant violation fails to compile:
// let bad = safe!(NonNan, 0.0_f64 / 0.0_f64); // E3002: cannot prove `NonNan`
```
