# Checklist: v0.8.2

## Theme

`v0.8.2` should be a focused release around applied policy-runtime ergonomics.

Primary release goal:

- make TupaLang better at expressing and supporting the kinds of policy flows that ViperTrade is already exercising in production-style local runtime

## Release Bar

Minimum bar for `v0.8.2`:

- at least one concrete improvement for temporal policy ergonomics
- at least one concrete improvement for structured decision outputs
- applied docs/examples showing the ViperTrade-driven usage model
- no regression in standalone CLI and crate publication workflow

## Epic 1: Temporal Policy Foundations

### Design

- [ ] Define the target semantics for policy stages such as `pending`, `degrading`, and `confirmed`.
- [ ] Decide what belongs in language/runtime versus host-side state management.
- [ ] Document the preferred contract shape for temporal policy outputs.

### Implementation

- [ ] Add one small, concrete improvement to temporal guard ergonomics.
- [ ] Ensure the improvement can be validated through examples or tests.
- [ ] Confirm the result still fits deterministic execution guarantees.

### Validation

- [ ] Add at least one applied example that mirrors a real temporal guard flow.
- [ ] Validate the example with local CI and release verification.

## Epic 2: Structured Decision Contracts

### Design

- [ ] Standardize a recommended output shape for policy decisions.
- [ ] Cover fields such as `action`, `stage`, `reason`, `score`, `components`, and `flags`.
- [ ] Decide which parts are conventions and which parts are stable public contracts.

### Implementation

- [ ] Improve one runtime/codegen/typecheck surface to better support structured decision outputs.
- [ ] Keep the change small enough to remain release-safe.

### Validation

- [ ] Add example outputs and docs that show how structured results should look.
- [ ] Verify compatibility with current crate consumers and CLI usage.

## Epic 3: External Typed Effects Foundations

This epic should remain narrow in `0.8.2`.

### Design

- [ ] Write the first technical specification for typed external effects.
- [ ] Cover typed input/output contracts, timeout behavior, fallback behavior, and audit metadata.
- [ ] Explicitly separate `advisory` versus `critical` external steps.

### Implementation

- [ ] If implementation starts in `0.8.2`, keep it experimental and narrow.
- [ ] Prefer one simple external-effect slice over broad provider integration.

### Validation

- [ ] Ensure every experimental external-effect path has deterministic fallback rules.
- [ ] Document how this supports advisory integrations such as ViperTrade AI analysis.

## Epic 4: Applied Documentation

- [ ] Add a concise applied architecture note connecting TupaLang and ViperTrade.
- [ ] Show the policy/runtime split explicitly.
- [ ] Document what belongs in Tupa policy and what remains in the host runtime.
- [ ] Include at least one example of structured outputs and one example of temporal policy.

## Crates and Release Operations

- [ ] Keep crate README parity aligned with the top-level positioning.
- [ ] Verify the publish workflow still covers all crates on tag-based release.
- [ ] Run release verification before cutting the tag.
- [ ] Confirm docs, changelog, and release notes are aligned before publication.

## What To Avoid In v0.8.2

- [ ] Do not expand scope into a full portfolio-selection DSL.
- [ ] Do not make external AI integration mandatory for normal runtime usage.
- [ ] Do not push large syntax changes without applied validation.
- [ ] Do not overload the release with multiple unrelated language experiments.

## Success Criteria

`v0.8.2` is successful if:

- TupaLang is meaningfully better for temporal policy and structured decision outputs
- the release remains small enough to ship confidently
- ViperTrade can point to at least one concrete simplification or clarification enabled by the new line
