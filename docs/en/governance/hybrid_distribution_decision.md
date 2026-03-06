# Hybrid Distribution Decision (Standalone Binary + Public Rust Crates)

## Status

Accepted for `v0.8.0-rc`.

## Context

Tupa must serve two distinct audiences:

- End users (ML/compliance/platform teams) who need a simple executable distribution.
- Rust integrators who need embeddability through stable crates.

A binary-only strategy hurts embeddability. A crate-only strategy hurts adoption and operations.

## Decision

Tupa adopts a hybrid distribution model:

1. Primary distribution: standalone CLI binary (`tupa-cli`) published as release artifacts.
2. Secondary distribution: selected public Rust crates for embedding (`tupa-parser`, `tupa-typecheck`, `tupa-runtime`).
3. Shared core: one codebase and one CI policy for both surfaces.

## Scope for `v0.8.0-rc`

- Add release workflow to build and publish multi-platform binaries plus checksums.
- Document binary installation and Rust embedding flows.
- Keep API-stability commitments limited to the selected crates above.

## Out of Scope for this RC

- Homebrew tap automation.
- Official Docker image publishing.
- External installer endpoint (for example, `tupa.dev/install.sh`).

## Consequences

- Better adoption path for non-Rust users.
- Maintained embeddability for Rust ecosystems.
- Clearer release responsibilities (artifacts + checksums + docs).
