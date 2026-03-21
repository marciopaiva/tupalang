# Post-Release Notes: v0.8.1

## Purpose

This note captures what `v0.8.1` taught us after shipping the language, runtime, and
`ViperTrade` integration together.

## What Worked Well

- `ViperTrade` served as a real functional proving ground for `TupaLang`.
- Small, mergeable slices kept the release moving without losing confidence.
- The sequence of features was coherent:
  - structured outputs
  - first-class reasons
  - weighted score support
  - typed config input pattern
  - declarative temporal policy support
- Release docs were tightened before the public tag, not after it.

## Main Lessons

### 1. The bottleneck was no longer data shape

Once `record types`, `record literals`, typed field access, structured runtime validation,
and weighted policy outputs were in place, the next gains did not come from more shape
machinery.

The real leverage came from:

- making policy reusable
- modeling temporal policy explicitly
- passing typed host-provided state into the pipeline

### 2. The host should keep operational state

The `0.8.1` work confirmed that the language becomes more useful when it models policy,
not when it tries to absorb all host state.

Stateful concerns still belong in the host application:

- signal confirmation counters
- cooldown tracking
- trailing-stop state
- persistence and external side effects

The language helped most when it could describe how that state should be interpreted.

### 3. The pipeline became valuable once it reflected real shapes

The `.tp` layer became materially more useful after it stopped being just an architectural
placeholder and started receiving:

- structured inputs
- real temporal state snapshots
- structured outputs consumed by the application runtime

That turned the pipeline into a real contract instead of a future-looking design sketch.

### 4. Standalone tooling must be validated early

The local validation flow exposed an important operational lesson: if the validation path
uses an outdated `tupa` binary, the pipeline appears broken even when the language changes
are correct.

Future feature work should keep these aligned early:

- `tupa-cli`
- local validation scripts
- container images used by CI or compose workflows

## What We Would Do Earlier Next Time

- Align the standalone CLI and local validation path sooner.
- Document the boundary between declarative policy and host-managed state earlier.
- Add a few focused tests earlier for newly exposed structured outputs.

## What v0.8.1 Achieved

`v0.8.1` moved `TupaLang` from a pipeline runtime with basic governance into a more useful
language for real strategy systems:

- outputs can be structured and typed
- reasons can be first-class
- policy can be composed with weighted scores
- temporal policy can be expressed declaratively
- typed host-provided config can be modeled without new core syntax

## Next Pressure Points

The next meaningful gains are not more output-shape primitives. They are:

- better ergonomics for reusable policy building blocks
- continued refinement of the boundary between host state and declarative policy
- deciding whether some internal structured breakdowns should become public contracts
