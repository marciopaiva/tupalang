//! tupa-core-macros unit tests — TDD coverage for Sprint 3.
//!
//! Classe 1 — PipelineInput parsing (10 testes TM-01..TM-10)
//! Classe 2 — ConstraintDecl parsing (11 testes TM-11..TM-21)
//! Classe 3 — StepDecl parsing (5 testes TM-22..TM-26)
//! Classe 4 — codegen expand (6 testes TM-27..TM-32)

use super::*;

// =============================================================================
// Classe 1 — PipelineInput parsing
// =============================================================================
mod class1 {
    use super::*;

    #[test]
    fn tm01_parse_simple_pipeline() {
        let ast: PipelineInput = syn::parse_str(
            r#"
            name: TestPipeline,
            input: i64,
            steps: [
                step("process") { input * 2 }
            ],
            constraints: [
                metric("result").ge(10)
            ]
        "#,
        )
        .unwrap();
        assert_eq!(ast.name.to_string(), "TestPipeline");
        assert_eq!(ast.steps.len(), 1);
        assert_eq!(ast.constraints.len(), 1);
    }

    #[test]
    fn tm02_parse_two_steps_and_constraints() {
        let ast: PipelineInput = syn::parse_str(
            r#"
            name: TwoSteps,
            input: f64,
            steps: [
                step("a") { 1.0 },
                step("b") { 2.0 }
            ],
            constraints: [
                metric("a").ge(0.0),
                metric("b").le(5.0)
            ]
        "#,
        )
        .unwrap();
        assert_eq!(ast.steps.len(), 2);
        assert_eq!(ast.constraints.len(), 2);
    }

    #[test]
    fn tm03_parse_no_constraints() {
        let ast: PipelineInput = syn::parse_str(
            r#"
            name: NoConstraints,
            input: (),
            steps: [
                step("solo") { 42 }
            ],
            constraints: []
        "#,
        )
        .unwrap();
        assert_eq!(ast.constraints.len(), 0);
    }

    #[test]
    fn tm04_parse_step_with_produces() {
        let ast: PipelineInput = syn::parse_str(
            r#"
            name: WithProduces,
            input: i32,
            steps: [
                step("calc") { 10 } produces ["out", "extra"]
            ],
            constraints: []
        "#,
        )
        .unwrap();
        assert_eq!(ast.steps[0].produces.as_ref().unwrap().len(), 2);
        assert_eq!(ast.steps[0].produces.as_ref().unwrap()[0], "out");
    }

    #[test]
    fn tm05_parse_step_with_requires() {
        let ast: PipelineInput = syn::parse_str(
            r#"
            name: WithRequires,
            input: i32,
            steps: [
                step("b") { 10 } requires ["x"]
            ],
            constraints: []
        "#,
        )
        .unwrap();
        assert_eq!(ast.steps[0].requires.as_ref().unwrap().len(), 1);
        assert_eq!(ast.steps[0].requires.as_ref().unwrap()[0], "x");
    }

    #[test]
    fn tm06_parse_step_with_both_metadata() {
        let ast: PipelineInput = syn::parse_str(
            r#"
            name: BothMeta,
            input: i32,
            steps: [
                step("step1") { 1 } produces ["a"] requires ["b"]
            ],
            constraints: []
        "#,
        )
        .unwrap();
        assert_eq!(ast.steps[0].produces.as_ref().unwrap().len(), 1);
        assert_eq!(ast.steps[0].requires.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn tm07_parse_missing_name_errors() {
        let result: syn::Result<PipelineInput> = syn::parse_str(
            r#"
            input: i32,
            steps: [
                step("s") { 1 }
            ],
            constraints: []
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn tm08_parse_missing_input_errors() {
        let result: syn::Result<PipelineInput> = syn::parse_str(
            r#"
            name: MissingInput,
            steps: [
                step("s") { 1 }
            ],
            constraints: []
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn tm09_parse_unknown_keyword_errors() {
        let result: syn::Result<PipelineInput> = syn::parse_str(
            r#"
            name: Test,
            input: i32,
            foo: bar,
            steps: [
                step("s") { 1 }
            ],
            constraints: []
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn tm10_parse_bad_step_keyword_errors() {
        let result: syn::Result<PipelineInput> = syn::parse_str(
            r#"
            name: Test,
            input: i32,
            steps: [
                func("s") { 1 }
            ],
            constraints: []
        "#,
        );
        assert!(result.is_err());
    }
}

// =============================================================================
// Classe 2 — ConstraintDecl parsing
// =============================================================================
mod class2 {
    use super::*;

    fn parse_constraint(input: &str) -> ConstraintDecl {
        syn::parse_str(input).unwrap()
    }

    #[test]
    fn tm11_parse_ge_constraint() {
        let c = parse_constraint(r#"metric("x").ge(10)"#);
        assert_eq!(c.metric_name, "x");
        assert!(
            matches!(c.threshold, ConstraintThreshold::Literal(v) if (v - 10.0).abs() < f64::EPSILON)
        );
        assert!(matches!(c.op, ConstraintOp::Ge));
    }

    #[test]
    fn tm12_parse_le_constraint() {
        let c = parse_constraint(r#"metric("y").le(5.0)"#);
        assert!(matches!(c.op, ConstraintOp::Le));
    }

    #[test]
    fn tm13_parse_eq_constraint() {
        let c = parse_constraint(r#"metric("z").eq(0.0)"#);
        assert!(matches!(c.op, ConstraintOp::Eq));
    }

    #[test]
    fn tm14_parse_ne_constraint() {
        let c = parse_constraint(r#"metric("a").ne(0.0)"#);
        assert!(matches!(c.op, ConstraintOp::Ne));
    }

    #[test]
    fn tm15_parse_gt_constraint() {
        let c = parse_constraint(r#"metric("b").gt(1.0)"#);
        assert!(matches!(c.op, ConstraintOp::Gt));
    }

    #[test]
    fn tm16_parse_lt_constraint() {
        let c = parse_constraint(r#"metric("c").lt(100.0)"#);
        assert!(matches!(c.op, ConstraintOp::Lt));
    }

    #[test]
    fn tm17_parse_constraint_int_value() {
        let c = parse_constraint(r#"metric("score").ge(42)"#);
        assert!(
            matches!(c.threshold, ConstraintThreshold::Literal(v) if (v - 42.0).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn tm18_parse_constraint_float_value() {
        let c = parse_constraint(r#"metric("ratio").ge(0.5)"#);
        assert!(
            matches!(c.threshold, ConstraintThreshold::Literal(v) if (v - 0.5).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn tm19_parse_constraint_missing_metric() {
        let result: syn::Result<ConstraintDecl> = syn::parse_str(r#"ge(1.0)"#);
        assert!(result.is_err());
    }

    #[test]
    fn tm20_parse_constraint_non_numeric_becomes_input_expr() {
        // Non-numeric thresholds are now accepted as InputExpr (0.11.0).
        // The generated `let threshold: f64 = "hello";` would fail at compile
        // time, but macro parsing succeeds.
        let result: syn::Result<ConstraintDecl> = syn::parse_str(r#"metric("x").ge("hello")"#);
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap().threshold,
            ConstraintThreshold::InputExpr(_)
        ));
    }

    #[test]
    fn tm21_parse_constraint_unknown_method() {
        let result: syn::Result<ConstraintDecl> = syn::parse_str(r#"metric("x").foo(1.0)"#);
        assert!(result.is_err());
    }
}

// =============================================================================
// Classe 3 — StepDecl parsing
// =============================================================================
mod class3 {
    use super::*;

    fn parse_step(input: &str) -> StepDecl {
        syn::parse_str(input).unwrap()
    }

    #[test]
    fn tm22_parse_step_expression() {
        let step = parse_step(r#"step("calc") { input * 2 }"#);
        assert_eq!(step.id, "calc");
    }

    #[test]
    fn tm23_parse_step_without_metadata() {
        let step = parse_step(r#"step("simple") { 42 }"#);
        assert_eq!(step.id, "simple");
        assert!(step.produces.is_none());
        assert!(step.requires.is_none());
    }

    #[test]
    fn tm24_parse_step_missing_parens_errors() {
        let result: syn::Result<StepDecl> = syn::parse_str(r#"step "x" { 1 }"#);
        assert!(result.is_err());
    }

    #[test]
    fn tm25_parse_step_missing_braces_errors() {
        let result: syn::Result<StepDecl> = syn::parse_str(r#"step("x") 1"#);
        assert!(result.is_err());
    }

    #[test]
    fn tm26_parse_unexpected_trailing_errors() {
        let result: syn::Result<StepDecl> = syn::parse_str(r#"step("x") { 1 } + 2"#);
        assert!(result.is_err());
    }
}

// =============================================================================
// Classe 4 — Codegen expand
// =============================================================================
mod class4 {
    use super::*;
    use proc_macro2::TokenStream;

    fn expand_pipeline(src: &str) -> TokenStream {
        let ast: PipelineInput = syn::parse_str(src).unwrap();
        let name = &ast.name;
        let input_type = &ast.input_type;
        let steps = &ast.steps;
        let constraints = &ast.constraints;

        let step_methods = generate_step_methods(steps);
        let metadata_methods = generate_metadata_methods(steps);
        let constraint_checks = generate_constraint_checks(constraints);

        quote! {
            pub struct #name;
            impl tupa_core::Pipeline for #name {
                type Input = #input_type;
                fn name(&self) -> &'static str { stringify!(#name) }
            }
            impl #name {
                #metadata_methods
                fn check(&self, values: &std::collections::HashMap<String, serde_json::Value>) {
                    #constraint_checks
                }
            }
            #step_methods
        }
    }

    #[test]
    fn tm27_expand_basic_pipeline_contains_impl() {
        let ts = expand_pipeline(
            r#"
            name: TestP,
            input: i32,
            steps: [step("s") { 1 }],
            constraints: []
        "#,
        );
        let s = ts.to_string();
        assert!(s.contains("impl tupa_core :: Pipeline"));
    }

    #[test]
    fn tm28_expand_produces_default_uses_step_id() {
        let ts = expand_pipeline(
            r#"
            name: TestP,
            input: i32,
            steps: [step("my_step") { 1 }],
            constraints: []
        "#,
        );
        let s = ts.to_string();
        assert!(s.contains("my_step"));
    }

    #[test]
    fn tm29_expand_produces_explicit() {
        let ts = expand_pipeline(
            r#"
            name: TestP,
            input: i32,
            steps: [step("s") { 1 } produces ["m1"]],
            constraints: []
        "#,
        );
        let s = ts.to_string();
        assert!(s.contains("m1"));
    }

    #[test]
    fn tm30_expand_constraint_op_ge() {
        let ts = expand_pipeline(
            r#"
            name: TestP,
            input: i32,
            steps: [step("s") { 10 } produces ["score"]],
            constraints: [metric("score").ge(5)]
        "#,
        );
        let s = ts.to_string();
        assert!(s.contains(">="));
    }

    #[test]
    fn tm31_expand_constraint_all_ops() {
        let ts = expand_pipeline(
            r#"
            name: TestP,
            input: i32,
            steps: [step("a") { 1 } produces ["a"], step("b") { 2 } produces ["b"], step("c") { 3 } produces ["c"]],
            constraints: [
                metric("a").ge(0),
                metric("b").le(5),
                metric("c").eq(3)
            ]
        "#,
        );
        let s = ts.to_string();
        assert!(s.contains(">="));
        assert!(s.contains("<="));
        assert!(s.contains("=="));
    }

    #[test]
    fn tm32_idempotent_expansion() {
        let src = r#"
            name: TestP,
            input: i32,
            steps: [step("s") { 1 }],
            constraints: []
        "#;
        let ts1 = expand_pipeline(src);
        let ts2 = expand_pipeline(src);
        assert_eq!(ts1.to_string(), ts2.to_string());
    }
}
