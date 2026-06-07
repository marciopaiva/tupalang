//! Tupã core library — types, traits, and pipeline macro re-export.

// Allow `tupa_core::...` paths (emitted by our proc-macros) to resolve within
// this crate itself, so the macros work in unit tests and doctests alike.
extern crate self as tupa_core;

/// Re-export the procedural macro so users can write `use tupa_core::pipeline;`
pub use tupa_core_macros::pipeline;

/// Re-export the `safe!` macro for compile-time-proven constrained values.
///
/// `safe!(Marker, expr)` proves the constraint at compile time when `expr` is a
/// constant `f64` expression, or falls back to a runtime check otherwise.
///
/// ```
/// use tupa_core::{safe, constraints::NonNan};
/// // Proven at compile time (constant expression):
/// let ok = safe!(NonNan, 1.0 + 2.0);
/// assert_eq!(ok.get(), 3.0);
/// // Runtime-checked for a non-constant value:
/// let x = "0.5".parse::<f64>().unwrap();
/// let y = safe!(NonNan, x);
/// assert_eq!(y.get(), 0.5);
/// ```
///
/// A constant expression that violates the constraint fails to compile:
///
/// ```compile_fail
/// use tupa_core::{safe, constraints::NonNan};
/// let bad = safe!(NonNan, 0.0_f64 / 0.0_f64); // NaN — E3002 at compile time
/// ```
pub use tupa_core_macros::safe;

/// Re-export serde_json for generated code.
pub use serde_json;

// ============================================================================
// Core types
// ============================================================================

/// A value with a compile-time proven constraint marker `C`.
///
/// The constraint type `C` is a zero-sized marker. Example:
/// ```rust,ignore
/// type X = Safe<f64, !nan>;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Safe<T, C>(pub T, std::marker::PhantomData<C>);

impl<T, C> Safe<T, C> {
    /// Create a new `Safe` value (no constraint check).
    ///
    /// Prefer [`Safe::try_new`] (runtime-checked) or the [`crate::safe!`] macro
    /// (compile-time proof for constant expressions) when the constraint `C`
    /// implements [`Constraint`].
    pub fn new(value: T) -> Self {
        Safe(value, std::marker::PhantomData)
    }

    /// Create a `Safe` without checking the constraint.
    ///
    /// Use only when the constraint is already established by construction
    /// (e.g. proven at compile time by the [`crate::safe!`] macro).
    pub fn new_unchecked(value: T) -> Self {
        Safe(value, std::marker::PhantomData)
    }

    /// Extract the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, C> Safe<T, C>
where
    C: Constraint<T>,
{
    /// Construct a `Safe`, checking the constraint `C` at runtime.
    ///
    /// Returns [`ConstraintError`] if `value` does not satisfy `C`.
    pub fn try_new(value: T) -> Result<Self, ConstraintError> {
        if C::satisfied(&value) {
            Ok(Safe(value, std::marker::PhantomData))
        } else {
            Err(ConstraintError {
                constraint: C::NAME,
            })
        }
    }
}

impl<T, C> Safe<T, C>
where
    T: Copy,
{
    /// Get a reference to the inner value.
    pub fn get(&self) -> T {
        self.0
    }

    /// Map the inner value to a new `Safe` with a different constraint marker.
    pub fn map<U, F>(self, f: F) -> Safe<U, C>
    where
        F: FnOnce(T) -> U,
    {
        Safe::new(f(self.0))
    }
}

/// Arithmetic operators for Safe<T, C>
macro_rules! impl_safe_arith {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident) => {
        impl<T, C> std::ops::$trait for Safe<T, C>
        where
            T: std::ops::$trait<Output = T> + Copy,
        {
            type Output = Safe<T, C>;

            fn $method(self, rhs: Self) -> Self::Output {
                Safe::new(self.0.$method(rhs.0))
            }
        }
    };
}

impl_safe_arith!(Add, add, AddAssign, add_assign);
impl_safe_arith!(Sub, sub, SubAssign, sub_assign);
impl_safe_arith!(Mul, mul, MulAssign, mul_assign);
impl_safe_arith!(Div, div, DivAssign, div_assign);

// Support RHS as raw value
impl<T, C> std::ops::Add<T> for Safe<T, C>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Safe<T, C>;
    fn add(self, rhs: T) -> Self::Output {
        Safe::new(self.0 + rhs)
    }
}

impl<T, C> std::ops::Sub<T> for Safe<T, C>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Safe<T, C>;
    fn sub(self, rhs: T) -> Self::Output {
        Safe::new(self.0 - rhs)
    }
}

impl<T, C> std::ops::Mul<T> for Safe<T, C>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Safe<T, C>;
    fn mul(self, rhs: T) -> Self::Output {
        Safe::new(self.0 * rhs)
    }
}

impl<T, C> std::ops::Div<T> for Safe<T, C>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Safe<T, C>;
    fn div(self, rhs: T) -> Self::Output {
        Safe::new(self.0 / rhs)
    }
}

/// Negation for Safe<T, C>
impl<T, C> std::ops::Neg for Safe<T, C>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Safe<T, C>;
    fn neg(self) -> Self::Output {
        Safe::new(-self.0)
    }
}

// ============================================================================
// Constraints (alignment types) — experimental
// ============================================================================

/// A constraint that a [`Safe<T, C>`] value must satisfy.
///
/// Implement this for a zero-sized marker type `C` to make it usable with
/// [`Safe::try_new`] and the [`crate::safe!`] macro.
pub trait Constraint<T> {
    /// Stable, human-readable identifier (e.g. `"!nan"`).
    const NAME: &'static str;

    /// Returns `true` if `value` satisfies the constraint.
    fn satisfied(value: &T) -> bool;
}

/// Error returned by [`Safe::try_new`] when a value violates its [`Constraint`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintError {
    /// The [`Constraint::NAME`] that was violated.
    pub constraint: &'static str,
}

impl std::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value violates constraint `{}`", self.constraint)
    }
}

impl std::error::Error for ConstraintError {}

/// Built-in constraint marker types.
///
/// These are kept in a submodule (not re-exported at the crate root) so they do
/// not collide with user-defined markers.
pub mod constraints {
    use super::Constraint;

    /// `!nan` — the value is not `NaN`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct NonNan;

    impl Constraint<f64> for NonNan {
        const NAME: &'static str = "!nan";
        fn satisfied(value: &f64) -> bool {
            !value.is_nan()
        }
    }

    /// `!inf` — the value is not `±∞`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct NonInf;

    impl Constraint<f64> for NonInf {
        const NAME: &'static str = "!inf";
        fn satisfied(value: &f64) -> bool {
            !value.is_infinite()
        }
    }

    /// `!nan, !inf` — the value is finite.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Finite;

    impl Constraint<f64> for Finite {
        const NAME: &'static str = "!nan,!inf";
        fn satisfied(value: &f64) -> bool {
            value.is_finite()
        }
    }
}

/// Tensor placeholder with shape information.
///
/// In full implementation, shape will be encoded as const generics.
/// For Sprint 1, this is a simple wrapper.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tensor<T>(pub T);

impl<T> Tensor<T> {
    /// Create a new Tensor.
    pub fn new(value: T) -> Self {
        Tensor(value)
    }

    /// Extract the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Tensor<T>
where
    T: Copy,
{
    /// Get a reference to the inner value.
    pub fn get(&self) -> T {
        self.0
    }
}

// ============================================================================
// Pipeline trait
// ============================================================================

/// Trait implemented by all pipelines (generated by the `pipeline!` macro).
///
/// You should not implement this manually.
pub trait Pipeline {
    /// The input data type for this pipeline.
    type Input: Send + Sync + 'static;

    /// Returns the human-readable name of the pipeline.
    fn name(&self) -> &'static str;
}

/// Test-only pipeline for compile-time trait bound verification.
#[derive(Debug, Clone)]
pub struct TestPipeline;

impl Pipeline for TestPipeline {
    type Input = i32;
    fn name(&self) -> &'static str {
        "TestPipeline"
    }
}

#[cfg(test)]
mod tests;
