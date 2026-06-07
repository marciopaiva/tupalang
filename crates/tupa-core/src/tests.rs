//! tupa-core unit tests — TDD coverage for Sprint 3.
//!
//! Classe 1 — Safe<T,C> (7 testes TC-C57..TC-C63)
//! Classe 2 — Tensor<T> (5 testes TC-C64..TC-C68)
//! Classe 3 — Re-exports e compatibilidade (5 testes TC-C69..TC-C73)
//! Classe 4 — Trait bounds (2 testes TC-C74..TC-C75)

use super::*;

// Phantom data marker types for tests
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct NonNan;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct AnyVal;

// =============================================================================
// Classe 1 — Safe<T, C>
// =============================================================================
mod class1 {
    use super::*;

    #[test]
    fn tc57_new_creates_value() {
        let s = Safe::<f64, NonNan>::new(42.0);
        assert_eq!(s.into_inner(), 42.0);
    }

    #[test]
    fn tc58_into_inner_extracts() {
        let s = Safe::<i32, AnyVal>::new(123);
        assert_eq!(s.into_inner(), 123);
    }

    #[test]
    fn tc59_new_and_into_inner_roundtrip() {
        let original = 99.5;
        let s = Safe::<f64, AnyVal>::new(original);
        assert_eq!(s.into_inner(), original);
    }

    #[test]
    fn tc60_clone_independent() {
        let s1 = Safe::<f64, AnyVal>::new(1.0);
        let s2 = s1;
        assert_eq!(s1.into_inner(), s2.into_inner());
    }

    #[test]
    fn tc58bis_safe_equality() {
        let s1 = Safe::<f64, AnyVal>::new(5.0);
        let s2 = Safe::<f64, AnyVal>::new(5.0);
        let s3 = Safe::<f64, AnyVal>::new(6.0);
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn tc59bis_different_constraint_markers_same_t() {
        let s1 = Safe::<f64, NonNan>::new(1.0);
        let s2 = Safe::<f64, AnyVal>::new(1.0);
        assert_eq!(s1.into_inner(), s2.into_inner());
    }

    #[test]
    fn tc60bis_tens_commutative() {
        let s1 = Safe::<f64, NonNan>::new(1.0);
        let s2 = Safe::<f64, AnyVal>::new(1.0);
        assert_eq!(s1.into_inner(), s2.into_inner());
    }

    #[test]
    fn tc73_safe_get_returns_inner() {
        let s = Safe::<f64, AnyVal>::new(42.0);
        assert_eq!(s.get(), 42.0);
    }

    #[test]
    fn tc74_safe_map_transforms_value() {
        let s = Safe::<f64, AnyVal>::new(5.0);
        let mapped = s.map(|x| x * 2.0);
        assert_eq!(mapped.into_inner(), 10.0);
    }

    #[test]
    fn tc75_safe_map_preserves_marker() {
        let s = Safe::<i32, NonNan>::new(10);
        let mapped: Safe<String, NonNan> = s.map(|x| format!("val-{}", x));
        assert_eq!(mapped.into_inner(), "val-10");
    }

    #[test]
    fn tc76_safe_add_operator() {
        let a = Safe::<i32, AnyVal>::new(10);
        let b = Safe::<i32, AnyVal>::new(5);
        assert_eq!((a + b).into_inner(), 15);
    }

    #[test]
    fn tc77_safe_sub_operator() {
        let a = Safe::<f64, AnyVal>::new(10.0);
        let b = Safe::<f64, AnyVal>::new(3.0);
        assert_eq!((a - b).into_inner(), 7.0);
    }

    #[test]
    fn tc78_safe_mul_operator() {
        let a = Safe::<i32, AnyVal>::new(6);
        let b = Safe::<i32, AnyVal>::new(7);
        assert_eq!((a * b).into_inner(), 42);
    }

    #[test]
    fn tc79_safe_div_operator() {
        let a = Safe::<f64, AnyVal>::new(15.0);
        let b = Safe::<f64, AnyVal>::new(3.0);
        assert_eq!((a / b).into_inner(), 5.0);
    }

    #[test]
    fn tc80_safe_add_rhs_raw() {
        let a = Safe::<i32, AnyVal>::new(10);
        assert_eq!((a + 5).into_inner(), 15);
    }

    #[test]
    fn tc81_safe_neg_operator() {
        let a = Safe::<i32, AnyVal>::new(10);
        assert_eq!((-a).into_inner(), -10);
    }
}

// =============================================================================
// Classe 1b — Constraints (Safe::try_new + safe! macro) — experimental
// =============================================================================
mod class_constraints {
    use crate::constraints::{Finite, NonInf, NonNan};
    use crate::{safe, Safe};

    #[test]
    fn safe_macro_proves_constant_expr() {
        let s = safe!(NonNan, 1.0 + 2.0);
        assert_eq!(s.get(), 3.0);
    }

    #[test]
    fn safe_macro_runtime_ok() {
        let x = "0.5".parse::<f64>().unwrap();
        let s = safe!(NonNan, x);
        assert_eq!(s.get(), 0.5);
    }

    #[test]
    #[should_panic(expected = "constraint violated at runtime")]
    fn safe_macro_runtime_violation_panics() {
        let x = f64::NAN;
        let _ = safe!(NonNan, x);
    }

    // User-defined marker the macro cannot reason about at compile time.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct Positive;
    impl crate::Constraint<f64> for Positive {
        const NAME: &'static str = "positive";
        fn satisfied(value: &f64) -> bool {
            *value > 0.0
        }
    }

    // Regression: a constant with an unknown (user-defined) marker must fall
    // back to the runtime guard, not silently construct via `new_unchecked`.
    // Previously `safe!(Positive, -1.0)` bypassed the constraint entirely.
    #[test]
    #[should_panic(expected = "constraint violated at runtime")]
    fn safe_macro_unknown_marker_constant_violation_panics() {
        let _ = safe!(Positive, -1.0);
    }

    #[test]
    fn safe_macro_unknown_marker_constant_ok() {
        let s = safe!(Positive, 1.0 + 1.0);
        assert_eq!(s.get(), 2.0);
    }

    #[test]
    fn try_new_enforces_constraint() {
        assert!(Safe::<f64, NonNan>::try_new(1.0).is_ok());
        assert!(Safe::<f64, NonNan>::try_new(f64::NAN).is_err());
        assert!(Safe::<f64, Finite>::try_new(f64::INFINITY).is_err());
        // NaN is not infinite, so it satisfies `!inf`.
        assert!(Safe::<f64, NonInf>::try_new(f64::NAN).is_ok());
    }

    #[test]
    fn constraint_error_reports_name() {
        let err = Safe::<f64, NonNan>::try_new(f64::NAN).unwrap_err();
        assert_eq!(err.constraint, "!nan");
        assert!(err.to_string().contains("!nan"));
    }
}

// =============================================================================
// Classe 2 — Tensor<T>
// =============================================================================
mod class2 {
    use super::*;

    #[test]
    fn tc61_new_wraps_value() {
        let t = Tensor(42.0);
        assert_eq!(t.0, 42.0);
    }

    #[test]
    fn tc62_into_inner_not_needed() {
        let t = Tensor(100);
        assert_eq!(t.0, 100);
    }

    #[test]
    fn tc63_clone_copies_inner() {
        let t1 = Tensor(std::f64::consts::PI);
        let t2 = t1;
        assert!((t1.0 - t2.0).abs() < 0.001);
    }

    #[test]
    fn tc64_tensor_f32() {
        let t = Tensor(std::f32::consts::PI);
        assert!((t.0 - std::f32::consts::PI).abs() < 0.001);
    }

    #[test]
    fn tc65_tensor_i32() {
        let t = Tensor(42i32);
        assert_eq!(t.0, 42);
    }

    #[test]
    fn tc66_tensor_new_factory() {
        let t = Tensor::new(42.0_f64);
        assert_eq!(t.get(), 42.0);
    }

    #[test]
    fn tc67_tensor_into_inner() {
        let t = Tensor(42_i32);
        assert_eq!(t.into_inner(), 42);
    }
}

// =============================================================================
// Classe 3 — Re-exports e compatibilidade de tipos
// =============================================================================
mod class3 {
    use super::*;

    #[test]
    fn tc66_pipeline_macro_importable() {
        fn _assert_pipeline_importable<P: Pipeline>() {}
        _assert_pipeline_importable::<TestPipeline>();
    }

    #[test]
    fn tc67_serde_json_reexport() {
        let v = serde_json::json!({"key": "value"});
        assert_eq!(v["key"], "value");
    }

    #[test]
    fn tc68_safe_debug_format() {
        let s = Safe(1.0, std::marker::PhantomData::<AnyVal>);
        let fmt = format!("{:?}", s);
        assert!(fmt.contains("1.0"));
    }

    #[test]
    fn tc69_tensor_debug_format() {
        let t = Tensor(42.0_f64);
        let fmt = format!("{:?}", t);
        assert!(fmt.contains("Tensor"));
    }

    #[test]
    fn tc70_safe_copy_trait() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<Safe<f64, AnyVal>>();
    }
}

// =============================================================================
// Classe 4 — Trait bounds de Pipeline
// =============================================================================
mod class4 {
    use super::*;

    #[test]
    fn tc71_pipeline_trait_object_safe() {
        fn _check_object_safe(_: &dyn Pipeline<Input = i32>) {}
    }

    #[test]
    fn tc72_executor_pipeline_from_core() {
        fn _impl_check<P: Pipeline<Input = i32>>() {}
    }
}
