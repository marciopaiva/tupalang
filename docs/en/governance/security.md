# Security Policy

## Purpose

Responsible disclosure process for vulnerabilities in Tupã crates and tools.

## Reporting

Report vulnerabilities via [GitHub Issues](https://github.com/marciopaiva/tupalang/issues) (private report via security advisory if sensitive).

## Scope

- `tupa-core` (DSL macros, types)
- `tupa-engine` (executor)
- `tupa-audit`, `tupa-plugin`, `tupa-fmt`, `tupa-lint` (supporting crates)
- `tupa-conformance` (test suite)
- Legacy: `tupa-parser`, `tupa-typecheck` (maintained for backward compatibility)

## SLA

- Acknowledgment: within 5 business days
- Initial update: within 10 business days
- Fix timeline: depends on severity (P0 critical: <72h; P1 high: <7d; P2 medium: <30d)
