//! Tupã lints — Clippy-like lint definitions for pipeline definitions.
//!
//! This crate provides `#[must_use]` annotations and lint-like functions
//! that can be called from `#[allow]` / `#[warn]` attributes via
//! `#[deny]` at the crate root.
//!
//! ## Example
//!
//! ```rust,ignore
//! #![deny(tupa_lints::pipeline_unused)]
//!
//! use tupa_core::pipeline;
//!
//! pipeline! {
//!     name: MyPipeline,
//!     input: MyInput,
//!     steps: [
//!         step("unused_step") { expensive_compute(input) }
//!     ],
//!     constraints: []
//! }
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
