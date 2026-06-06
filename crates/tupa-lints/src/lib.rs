//! Tupã lints — named identifiers for pipeline-quality rules.
//!
//! This crate exposes `&'static str` constants that name recommended
//! pipeline-quality lints. They are stable string identifiers shared across
//! Tupã tooling and reports.
//!
//! These are plain string constants, **not** `rustc`/Clippy lints: they cannot
//! be used with `#[deny(...)]` / `#[warn(...)]` attributes. Enforcement against
//! pipeline code is performed by external tooling, not the compiler.
//!
//! ## Example
//!
//! ```rust
//! assert_eq!(tupa_lints::PIPELINE_UNUSED_METRIC, "tupa_pipeline_unused_metric");
//! ```

#![deny(missing_docs)]

/// Lint: every produced metric should be consumed by at least one constraint.
#[allow(missing_docs)]
pub const PIPELINE_UNUSED_METRIC: &str = "tupa_pipeline_unused_metric";

/// Lint: step count in a pipeline should be ≤ 20 for maintainability.
#[allow(missing_docs)]
pub const PIPELINE_TOO_LARGE: &str = "tupa_pipeline_too_large";

/// Lint: constraint value should not be a literal (use a named constant).
#[allow(missing_docs)]
pub const CONSTRAINT_LITERAL: &str = "tupa_constraint_literal";

#[cfg(test)]
mod tests {
    #[test]
    fn lint_constants_are_defined() {
        assert_eq!(super::PIPELINE_UNUSED_METRIC, "tupa_pipeline_unused_metric");
        assert_eq!(super::PIPELINE_TOO_LARGE, "tupa_pipeline_too_large");
        assert_eq!(super::CONSTRAINT_LITERAL, "tupa_constraint_literal");
    }
}
