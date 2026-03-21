# RFC: v0.8.1 Trading Strategy Support

## Status

- Proposed
- Target version: `0.8.1`
- Primary driver: ViperTrade production strategy modeling

## Summary

TupaLang already supports deterministic pipelines, auditability, and controlled runtime integration. For production trading systems such as ViperTrade, the next gap is not infrastructure. The gap is expressive strategy modeling.

Version `0.8.1` should focus on the language and runtime features needed to model real trading policies declaratively, instead of leaving core strategy semantics hardcoded in application code.

This RFC proposes a narrow, pragmatic release theme:

- structured step outputs
- first-class policy reasons
- reusable predicates
- typed config bindings
- weighted score support
- declarative temporal policy support

## Problem

ViperTrade currently uses TupaLang as a pipeline shell, while the real strategy semantics still live in Rust:

- entry gates
- hold reasons
- position health score
- thesis invalidation rules
- part of signal confirmation and cooldown policy

That split reduces the value of the language in the exact area where it should help most:

- auditability
- explainability
- safe iteration
- strategy review
- release confidence

## Goals

`0.8.1` should make TupaLang materially better for production strategy systems.

### Primary goals

- Allow steps to return typed structured results instead of only primitive values.
- Allow strategy rules to produce machine-readable reasons directly.
- Allow policies to be composed from reusable predicates.
- Allow strategy policies to read typed runtime configuration without routing all semantics through host code.
- Allow weighted scores to be expressed declaratively.
- Allow temporal decision policies to be described without embedding all semantics in host code.

### Non-goals

- Replace host runtime responsibilities such as Redis, database, exchange IO, or process orchestration.
- Move all state handling into TupaLang.
- Build a complete trading DSL in one release.

## Proposed Features

### 1. Structured step outputs

Support record-like step outputs with explicit fields and type validation.

Examples of desired outputs:

- `EntryPolicyResult`
- `SizingResult`
- `PositionHealthResult`
- `ThesisExitResult`
- `StrategyDecision`

Illustrative shape:

```text
step("entry_policy") {
  {
    eligible: true,
    side: long,
    entry_score: 72,
    reason: "entry_confirmed_consensus_and_momentum"
  }
}
```

### 2. First-class reasons

Support policy outcomes that naturally carry reasons.

Illustrative shape:

```text
{
  passed: false,
  reason: "entry_blocked_low_volume"
}
```

This avoids duplicating logic in host applications just to explain why a rule failed.

### 3. Reusable predicates

Allow policies to be built from named reusable predicates, such as:

- `passes_consensus(side)`
- `passes_momentum(side)`
- `passes_macro(side)`
- `passes_liquidity(side)`

This reduces rule duplication across:

- long and short
- entry and hold
- health and exit policies

### 4. Weighted score support

Support weighted score composition for policies such as position health.

Illustrative use cases:

- consensus alignment with high weight
- exchange anchor regime with medium/high weight
- BTC macro alignment with medium weight
- MACD histogram sign with low/medium weight

Desired capabilities:

- additive score components
- penalties and bonuses
- clamp/range handling
- threshold comparisons

### 5. Typed config bindings

Support typed access to host-provided configuration values used by production strategy systems.

Illustrative use cases:

- per-symbol thresholds
- mode-specific overlays
- trailing parameters
- confirmation thresholds
- macro filter thresholds

Desired capabilities:

- typed reads from a host-provided config object
- explicit defaults or fallback rules
- shape validation before runtime execution

### 6. Declarative temporal policy support

Support policy semantics that depend on persistence across evaluation cycles.

Target patterns:

- confirm for `N` ticks
- degrade for `N` ticks before exit
- cooldown after stop loss
- block re-entry until side flip

The host runtime may still own the state store, but the policy itself should be declared in TupaLang.

## Why this matters for ViperTrade

With these features, ViperTrade can move its core strategy semantics into TupaLang:

- entry eligibility
- hold reasons
- position health score
- thesis invalidation
- part of re-entry and confirmation policy

That would leave Rust focused on the right concerns:

- runtime orchestration
- state persistence
- exchange integration
- event transport
- risk plumbing

## Priority refinement after ViperTrade integration

The ViperTrade `0.8.1` migration clarified the next real bottleneck.

What the language already solves well enough:

- typed records
- record literals
- first-class reason helpers
- weighted score helpers
- ordinary function reuse for many predicate-like helpers

What still forces too much host-side policy wiring:

- configuration access
- mode/profile overlays
- threshold selection from strategy config
- temporal confirmation and cooldown semantics

As a result, the next implementation priority should be:

1. typed config bindings
2. declarative temporal policy support
3. reusable predicate ergonomics where plain functions still feel too blunt

## Proposed delivery order

### Phase 1

- structured step outputs
- first-class reasons
- weighted score support

This is the minimum needed to move entry policy, hold reasons, and scoring models into TupaLang.

### Phase 2

- typed config bindings

This enables production strategies such as ViperTrade to read thresholds and profile overlays declaratively.

### Phase 3

- declarative temporal policy support

This enables signal confirmation, thesis persistence windows, and cooldown semantics.

### Phase 4

- reusable predicate ergonomics

Named predicate helpers are still useful, but ViperTrade showed that ordinary functions already cover part of this need today.

## Expected impact

If `0.8.1` delivers this scope, TupaLang becomes materially more useful for production policy systems, not just pipeline orchestration.

Expected outcomes:

- less strategy duplication in host code
- clearer release reviews
- easier strategy tuning
- better audit trails
- better fit for real trading applications

## Open questions

- Should weighted score support be generic language syntax or a standard library helper first?
- Should first-class reasons be modeled as a convention over structured records, or as a dedicated language primitive?
- How far should temporal policy support go in `0.8.1` before it becomes state-machine design?

## Recommendation

Adopt this RFC as the planning anchor for `0.8.1`, with Phase 1 as the minimum release bar and Phases 2-3 as targeted scope when implementation cost remains controlled.
