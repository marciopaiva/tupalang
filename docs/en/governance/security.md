# Security Policy

## Purpose

Responsible disclosure process for vulnerabilities in Tupã crates and tools.

## Reporting

Report vulnerabilities via [GitHub Issues](https://github.com/marciopaiva/tupalang/issues) (private report via security advisory if sensitive).

## Scope

Security reviews and patches apply to **active crates** only:

- `tupa-core` (DSL macros, types)
- `tupa-engine` (executor)
- `tupa-plugin` (plugin system)
- `tupa-pyffi` (Python bindings)
- `cargo-tupa` (CLI)

Legacy crates (`tupa-parser`, `tupa-typecheck`, `tupa-cli`, `tupa-fmt`, `tupa-lint`, `tupa-audit`, `tupa-conformance`, and others) were **removed from the workspace** in version 0.9.0 and are no longer maintained. They are out of scope.

## SLA

- Acknowledgment: within 5 business days
- Initial update: within 10 business days
- Fix timeline: depends on severity (P0 critical: <72h; P1 high: <7d; P2 medium: <30d)
