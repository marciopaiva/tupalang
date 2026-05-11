# Tupã Language — Type System Semantics (Formal Summary)

> **Normative reference**: Section 3 of the Tupã Language Specification (`docs/reference/spec.md`).
> This document provides a concise, standalone description of the type rules.

---

## 1. Primitive Types

| Type   | Description            | Size/Representation |
|--------|------------------------|---------------------|
| `i64`  | Signed 64-bit integer  | two's complement    |
| `f64`  | IEEE 754 double        | 64-bit              |
| `f32`  | IEEE 754 single        | 32-bit              |
| `f16`  | IEEE 754 half          | 16-bit              |
| `bool` | Boolean                | 1-bit (logical)     |
| `string` | Immutable UTF-8      | heap-allocated      |
| `null` | Null value             | singleton           |
| `unit` | Unit type `()`         | zero-size           |

**Numeric conversions**: Implicit conversions are **forbidden**. Use explicit `as` operator, e.g. `i64 as f64`.

---

## 2. Composite Types

### 2.1 Tuples

- **Form**: `(T1, T2, ..., Tn)`
- **Fields**: accessed by index `.0`, `.1`, ...
- **Type**: heterogeneous, fixed-size.

### 2.2 Function Types

- **Form**: `fn(A1, ..., An) -> R`
- **Values**: named functions, closures, function pointers.
- **Closure capture**: inferred; captures by reference or value depending on context.

### 2.3 Enums (with generics)

- **Form**: `enum Name<T...> { V1(T1), V2(T2), ... }`
- **Variants**: may carry data or be unit-like.
- **Generics**: type parameters are inferred or annotated.
- **Example**: `Option<T>`, `Result<T, E>` are standard library types.

### 2.4 Arrays and Slices

- **Fixed-size array**: `[T; N]` — length `N` known at compile time, stack-allocated when possible.
- **Slice (dynamic)**: `[T]` — heap-allocated, length known only at runtime.
- **Literals**: `[1, 2, 3]` infers `[i64; 3]`.

### 2.5 Tensors (AI-first)

- **Form**: `Tensor<T, shape=[D1, D2, ...], density=d>`
  - `T` — element type (`f16`, `f32`, `f64`)
  - `shape` — list of integers or `...` (dynamic dimension)
  - `density` — optional sparsity hint (0.0–1.0), default `1.0`
- **Semantics**:
  - Compile-time shape checking where dimensions are constant.
  - Runtime shape validation for dynamic dimensions.
  - Operations (`.map`, `.reduce`, matmul, etc.) require shape compatibility.

### 2.6 Alignment Types (`Safe<T, !constraint>`)

- **Form**: `Safe<T, !c1, !c2, ...>`
- **Constraints** (current):
  - `!nan`, `!inf` — numeric stability (proved via interval analysis)
  - `!hate_speech`, `!misinformation` — content safety (proved via RLHF scorer ≥ 0.95)
- **Rule**: If compiler **cannot prove** constraint, compilation fails. Use `unsafe { ... }` to bypass with explicit audit.

---

## 3. Type Inference and Unification

- **Algorithm**: Hindley-Milner with extension for:
  - Generic enum types (`Option<T>`, `Result<T,E>`)
  - Constraint solving for `Safe<...>` types
  - Effect inference (side-effect classification)

- **Unification**:
  - Occurs check prevents circular references.
  - Type variables are instantiated to monotypes during inference.
  - If two types fail to unify, produce `E2001` with diagnostic showing expected vs found.

- **Generalization**:
  - let-bound variables are generalized to polymorphic types when no enclosing `lambda` introduces mutable state.

- **Specialization**:
  - Function arguments are specialized at call site.

---

## 4. Subtyping and Coercion

**No implicit subtyping** except:

- **Numeric widening**: `i64` → `f64` is **not implicit**; explicit `as` required.
- **`Safe` refinement**: `Safe<T, !c>` is a **subtype** of `T`. A value of type `Safe<i64, !nan>` can be used where `i64` is expected.
- **Tuple/record width**: No structural subtyping; types must match exactly.

---

## 5. Effect System (Summary)

- Effects tracked: `io`, `state`, `exception`, `non_determinism`, `side_effects`.
- Pure functions: no effects (except `∇` which is effect-free by definition).
- `Safe<T, !c>` expressions must be pure (no I/O, no mutation).
- Effect constraints are part of type checking; effect mismatches produce `E3xxx` errors.

---

## 6. Type Checking Rules (Key Judgments)

### 6.1 Expressions

```
Γ ⊢ e : τ   means "expression e has type τ in environment Γ"
```

- **Literal**: `Γ ⊢ 42 : i64` (by token)
- **Variable**: `Γ(x) = τ  ⇒  Γ ⊢ x : τ`
- **Binary op**: `Γ ⊢ e1 : τ1`, `Γ ⊢ e2 : τ2`, `τ1 == τ2` (or coercible), result type depends on op.
- **If**: `Γ ⊢ cond : bool`, `Γ ⊢ then : τ`, `Γ ⊢ else : τ` ⇒ `Γ ⊢ if cond {then} else {else} : τ`.
- **Match**: all arms must have same type; patterns bind variables whose types are checked in extended environment.
- **Function call**: `Γ ⊢ f : fn(A...) -> R`, `Γ ⊢ args... : A...` ⇒ `Γ ⊢ f(args...) : R`.

### 6.2 Declarations

- **Let**: `Γ ⊢ expr : τ` ⇒ `Γ, x: τ ⊢ ...`
- **Function**:
  1. Parameter types are from signature.
  2. Body checked under `Γ, params...` must yield return type (or unit if no return).
  3. If `: τ` annotation present, function type is `fn(params...) -> τ`; else infer.

### 6.3 Constraints (`Safe`)

For `Safe<τ, !c>`:
- **Proving**: compiler runs constraint solver.
  - `!nan`: prove value ∈ ℝ \ {NaN}
  - `!inf`: prove value finite
  - `!hate_speech`: RLHF score ≥ threshold (only for constant string expressions)
- If proof fails → `E3002: cannot prove constraint '!c'`.

---

## 7. Diagnostics (Normative)

All type errors must include:

- **Code**: `E2xxx` (type errors range E2000–E2999)
- **Message**: concise description
- **Location**: file, line, column
- **Span**: character offsets for highlighting
- **Hint**: suggested fix (when available)

Example:

```
E2001: type mismatch
   --> main.tp:12:5
    |
12  | let x: i64 = "text"
    |            ^^^^^^^ expected i64, found string
```

---

## 8. Version

This document corresponds to Tupã Language Specification v0.1.
