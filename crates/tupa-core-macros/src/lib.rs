//! Tupã core procedural macro — the `pipeline!` DSL for defining typed policy pipelines.
//!
//! This crate implements the `pipeline!` macro that generates pipeline structs
//! implementing `tupa_core::Pipeline`, `tupa_engine::ExecutorPipeline`, and
//! `tupa_engine::ParallelPipeline` traits.
//!
//! ## Example
//!
//! ```rust,ignore
//! use tupa_core::pipeline;
//!
//! pipeline! {
//!     name: MyPipeline,
//!     input: MyInput,
//!     steps: [
//!         step("process") { process(input) }
//!     ],
//!     constraints: [
//!         metric("score").ge(0.0)
//!     ]
//! }
//! ```
//!
//! The macro expands to a struct with associated methods for execution.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Token;
use syn::{braced, punctuated::Punctuated, token::Comma, Expr, Ident, Lit, LitStr, Type};

// ============================================================================
// AST
// ============================================================================

struct PipelineInput {
    name: Ident,
    input_type: Type,
    steps: Vec<StepDecl>,
    constraints: Vec<ConstraintDecl>,
}

struct StepDecl {
    id: String,
    body: Box<Expr>,
    produces: Option<Vec<String>>,
    requires: Option<Vec<String>>,
}

struct ConstraintDecl {
    metric_name: String,
    op: ConstraintOp,
    value: f64,
}

#[derive(Clone, Copy)]
enum ConstraintOp {
    Ge,
    Le,
    Eq,
    Ne,
    Gt,
    Lt,
}

// ============================================================================
// Parsers
// ============================================================================

impl Parse for StepDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func_ident: Ident = input.parse()?;
        if func_ident != "step" {
            return Err(syn::Error::new_spanned(
                func_ident,
                "expected keyword `step`",
            ));
        }

        let content;
        let _parens = syn::parenthesized!(content in input);
        let id_lit: LitStr = content.parse()?;
        let id = id_lit.value();

        let body_content;
        let _braces = braced!(body_content in input);
        let body: Expr = body_content.parse()?;

        // Parse optional produces / requires annotations after body
        let mut produces = None;
        let mut requires = None;

        while !input.is_empty() {
            // If a trailing comma is next, stop parsing step-specific annotations
            if input.peek(Comma) {
                break;
            }
            let kw: Ident = input.parse()?;
            match kw.to_string().as_str() {
                "produces" => {
                    let content;
                    syn::bracketed!(content in input);
                    let items = Punctuated::<LitStr, Comma>::parse_terminated(&content)?;
                    produces = Some(items.iter().map(|s| s.value()).collect());
                }
                "requires" => {
                    let content;
                    syn::bracketed!(content in input);
                    let items = Punctuated::<LitStr, Comma>::parse_terminated(&content)?;
                    requires = Some(items.iter().map(|s| s.value()).collect());
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        &kw,
                        "expected `produces` or `requires`",
                    ))
                }
            }
        }

        Ok(StepDecl {
            id,
            body: Box::new(body),
            produces,
            requires,
        })
    }
}

#[allow(clippy::collapsible_match)]
impl Parse for ConstraintDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let call_expr: Expr = input.parse()?;

        if let Expr::MethodCall(method_call) = &call_expr {
            let method_name = method_call.method.to_string();
            let op = match method_name.as_str() {
                "ge" => ConstraintOp::Ge,
                "le" => ConstraintOp::Le,
                "eq" => ConstraintOp::Eq,
                "ne" => ConstraintOp::Ne,
                "gt" => ConstraintOp::Gt,
                "lt" => ConstraintOp::Lt,
                _ => {
                    return Err(syn::Error::new_spanned(
                        &method_call.method,
                        format!("unknown constraint method '{}'", method_name),
                    ))
                }
            };

            let metric_call = match method_call.receiver.as_ref() {
                Expr::Call(call) => call,
                _ => {
                    return Err(syn::Error::new_spanned(
                        &method_call.receiver,
                        "expected metric(\"name\") call",
                    ))
                }
            };

            let func_path = match metric_call.func.as_ref() {
                Expr::Path(path) => path,
                _ => {
                    return Err(syn::Error::new_spanned(
                        &metric_call.func,
                        "expected identifier `metric`",
                    ))
                }
            };

            if func_path.path.segments.last().unwrap().ident != "metric" {
                return Err(syn::Error::new_spanned(
                    func_path,
                    "expected function `metric`",
                ));
            }

            if let Some(arg) = metric_call.args.first() {
                if let Expr::Lit(lit) = arg {
                    if let Lit::Str(lit_str) = &lit.lit {
                        let metric_name = lit_str.value();

                        if let Some(value_arg) = method_call.args.first() {
                            let value = match &value_arg {
                                Expr::Lit(l) => match &l.lit {
                                    Lit::Int(i) => i.base10_parse::<f64>().map_err(|_| {
                                        syn::Error::new_spanned(i, "failed to parse integer as f64")
                                    })?,
                                    Lit::Float(f) => f.base10_parse::<f64>().map_err(|_| {
                                        syn::Error::new_spanned(f, "failed to parse float")
                                    })?,
                                    _ => {
                                        return Err(syn::Error::new_spanned(
                                            value_arg,
                                            "expected numeric literal for constraint value",
                                        ))
                                    }
                                },
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        value_arg,
                                        "expected numeric literal",
                                    ))
                                }
                            };
                            return Ok(ConstraintDecl {
                                metric_name,
                                op,
                                value,
                            });
                        }
                    }
                }
            }
        }

        Err(syn::Error::new_spanned(
            call_expr,
            "constraint must be: metric(\"name\").ge(value)",
        ))
    }
}

impl Parse for PipelineInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_type;
        let steps;

        let name_keyword: Ident = input.parse()?;
        if name_keyword != "name" {
            return Err(syn::Error::new_spanned(
                name_keyword,
                "expected keyword `name`",
            ));
        }
        input.parse::<Token![:]>()?;
        let name = input.parse()?;
        input.parse::<Comma>()?;

        let input_keyword: Ident = input.parse()?;
        if input_keyword != "input" {
            return Err(syn::Error::new_spanned(
                input_keyword,
                "expected keyword `input`",
            ));
        }
        input.parse::<Token![:]>()?;
        input_type = input.parse()?;
        input.parse::<Comma>()?;

        let steps_keyword: Ident = input.parse()?;
        if steps_keyword != "steps" {
            return Err(syn::Error::new_spanned(
                steps_keyword,
                "expected keyword `steps`",
            ));
        }
        input.parse::<Token![:]>()?;
        let steps_content;
        let _ = syn::bracketed!(steps_content in input);
        steps = Punctuated::<StepDecl, Comma>::parse_terminated(&steps_content)?
            .into_iter()
            .collect();
        input.parse::<Comma>()?;

        let constraints_keyword: Ident = input.parse()?;
        if constraints_keyword != "constraints" {
            return Err(syn::Error::new_spanned(
                constraints_keyword,
                "expected keyword `constraints`",
            ));
        }
        input.parse::<Token![:]>()?;
        let constraints_content;
        let _ = syn::bracketed!(constraints_content in input);
        let constraints =
            Punctuated::<ConstraintDecl, Comma>::parse_terminated(&constraints_content)?
                .into_iter()
                .collect();

        Ok(PipelineInput {
            name,
            input_type,
            steps,
            constraints,
        })
    }
}

// ============================================================================
// Code generation
// ============================================================================

fn generate_step_methods(steps: &[StepDecl]) -> proc_macro2::TokenStream {
    let mut methods = Vec::new();

    for step in steps {
        let id = &step.id;
        let body = &step.body;
        let method_name = Ident::new(&format!("step_{}", id), proc_macro2::Span::call_site());

        methods.push(quote! {
            pub fn #method_name(&self, input: &<Self as tupa_core::Pipeline>::Input) -> Result<tupa_core::serde_json::Value, tupa_engine::EngineError> {
                let result = #body;
                tupa_core::serde_json::to_value(&result)
                    .map_err(|e| tupa_engine::EngineError::Other(e.to_string()))
            }
        });
    }

    quote! { #(#methods)* }
}

fn generate_metadata_methods(steps: &[StepDecl]) -> proc_macro2::TokenStream {
    let mut impls = Vec::new();

    // Generate step_ids()
    let step_id_strs = steps.iter().map(|s| s.id.as_str()).collect::<Vec<_>>();
    impls.push(quote! {
        fn step_ids(&self) -> &'static [&'static str] {
            &[#(#step_id_strs),*]
        }
    });

    for step in steps {
        let id = &step.id;
        let method_name_produces =
            Ident::new(&format!("produces_{}", id), proc_macro2::Span::call_site());
        let method_name_requires =
            Ident::new(&format!("requires_{}", id), proc_macro2::Span::call_site());

        // If `produces` is not explicitly provided, default to the step id itself.
        let produces_literals = if let Some(v) = &step.produces {
            v.iter().map(|s| quote! { #s }).collect::<Vec<_>>()
        } else {
            vec![quote! { #id }]
        };

        let requires_literals = step
            .requires
            .as_ref()
            .map(|v| v.iter().map(|s| quote! { #s }).collect::<Vec<_>>())
            .unwrap_or_default();

        impls.push(quote! {
            fn #method_name_produces(&self) -> &'static [&'static str] {
                &[#(#produces_literals),*]
            }
        });
        impls.push(quote! {
            fn #method_name_requires(&self) -> &'static [&'static str] {
                &[#(#requires_literals),*]
            }
        });
    }

    quote! { #(#impls)* }
}

fn generate_step_calls(steps: &[StepDecl]) -> proc_macro2::TokenStream {
    let calls = steps.iter().map(|step| {
        let id = &step.id;
        let method_name = Ident::new(&format!("step_{}", id), proc_macro2::Span::call_site());
        let produces = step
            .produces
            .as_ref()
            .map(|v| v.iter().collect::<Vec<_>>())
            .unwrap_or_else(|| vec![id]); // default: step id itself is the metric name

        quote! {
            let val = self.#method_name(input)?;
            #(
                values.insert(#produces.to_string(), val.clone());
            )*
        }
    });
    quote! { #(#calls)* }
}

fn generate_constraint_checks(constraints: &[ConstraintDecl]) -> proc_macro2::TokenStream {
    let checks = constraints.iter().map(|c| {
        let metric_name = &c.metric_name;
        let value = c.value;
        let (op_str, condition) = match c.op {
            ConstraintOp::Ge => (">=", quote! { v >= #value }),
            ConstraintOp::Le => ("<=", quote! { v <= #value }),
            ConstraintOp::Eq => ("==", quote! { v == #value }),
            ConstraintOp::Ne => ("!=", quote! { v != #value }),
            ConstraintOp::Gt => (">", quote! { v > #value }),
            ConstraintOp::Lt => ("<", quote! { v < #value }),
        };

        quote! {
            if let Some(actual) = values.get(#metric_name) {
                if let Some(v) = actual.as_f64() {
                    if !(#condition) {
                        failures.push(tupa_engine::ConstraintFailure {
                            metric: #metric_name.to_string(),
                            operator: #op_str.to_string(),
                            expected: tupa_core::serde_json::json!(#value),
                            actual: actual.clone(),
                        });
                    }
                }
            }
        }
    });

    quote! { #(#checks)* }
}

/// Procedural macro defining a Tupã policy pipeline.
///
/// ## Syntax
///
/// ```rust,ignore
/// pipeline! {
///     name: MyPipeline,
///     input: InputType,
///     steps: [
///         step("step1") { step1_func(input) },
///         step("step2") { step2_func(input) }
///     ],
///     constraints: [
///         metric("sharpe").ge(1.5),
///         metric("max_drawdown").le(0.2)
///     ]
/// }
/// ```
///
/// The macro expands to a `pub struct MyPipeline` implementing `tupa_core::Pipeline`
/// and `tupa_engine::ExecutorPipeline`.
#[proc_macro]
pub fn pipeline(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as PipelineInput);

    let name = &ast.name;
    let input_type = &ast.input_type;
    let steps = &ast.steps;
    let constraints = &ast.constraints;

    // Generate step methods
    let step_methods = generate_step_methods(steps);

    // Generate metadata methods (produces_*/requires_*)
    let metadata_methods = generate_metadata_methods(steps);

    // Generate step calls for execute (sequential)
    let step_calls = generate_step_calls(steps);

    // Prepare identifiers for match arms in ParallelPipeline impl
    let step_id_lits: Vec<LitStr> = steps
        .iter()
        .map(|s| LitStr::new(&s.id, proc_macro2::Span::call_site()))
        .collect();

    let produces_method_idents: Vec<Ident> = steps
        .iter()
        .map(|s| {
            Ident::new(
                &format!("produces_{}", s.id),
                proc_macro2::Span::call_site(),
            )
        })
        .collect();

    let requires_method_idents: Vec<Ident> = steps
        .iter()
        .map(|s| {
            Ident::new(
                &format!("requires_{}", s.id),
                proc_macro2::Span::call_site(),
            )
        })
        .collect();

    let execute_step_arms: Vec<proc_macro2::TokenStream> = steps
        .iter()
        .map(|step| {
            let id_lit = LitStr::new(&step.id, proc_macro2::Span::call_site());
            let method_name =
                Ident::new(&format!("step_{}", step.id), proc_macro2::Span::call_site());
            quote! {
                #id_lit => self.#method_name(input).map_err(|e| e.into()),
            }
        })
        .collect();

    // Generate runtime constraint check code
    let constraint_checks = generate_constraint_checks(constraints);

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub struct #name;

        // Implement core Pipeline trait
        impl tupa_core::Pipeline for #name {
            type Input = #input_type;

            fn name(&self) -> &'static str {
                stringify!(#name)
            }
        }

        // Constructor and constraint checker
        impl #name {
            /// Create a new pipeline instance.
            pub fn new() -> Self {
                Self
            }

            /// Check constraints against collected metric values.
            #[doc(hidden)]
            pub fn check_constraints(
                values: &std::collections::HashMap<String, tupa_core::serde_json::Value>,
            ) -> (bool, Vec<tupa_engine::ConstraintFailure>) {
                let mut failures = Vec::new();
                #constraint_checks
                (failures.is_empty(), failures)
            }

            // Metadata methods for parallel executor
            #metadata_methods
        }

        // Implement engine execution trait
        impl tupa_engine::ExecutorPipeline for #name {
            fn execute(
                &self,
                input: &<Self as tupa_core::Pipeline>::Input,
            ) -> Result<tupa_engine::PipelineResult, tupa_engine::EngineError> {
                use tupa_engine::PipelineResult;
                let mut values = std::collections::HashMap::new();
                #step_calls
                let (passed, failures) = Self::check_constraints(&values);
                Ok(PipelineResult { values, passed, failures, metrics: Vec::new() })
            }
        }

        // Implement parallel pipeline capability
        impl tupa_engine::ParallelPipeline for #name {
            fn step_ids(&self) -> &'static [&'static str] {
                Self::step_ids(self)
            }

            fn produces(&self, step_id: &str) -> &'static [&'static str] {
                match step_id {
                    #(
                        #step_id_lits => Self::#produces_method_idents(self),
                    )*
                    _ => &[],
                }
            }

            fn requires(&self, step_id: &str) -> &'static [&'static str] {
                match step_id {
                    #(
                        #step_id_lits => Self::#requires_method_idents(self),
                    )*
                    _ => &[],
                }
            }

            fn execute_step(
                &self,
                input: &<Self as tupa_core::Pipeline>::Input,
                step_id: &str,
            ) -> Result<serde_json::Value, tupa_engine::EngineError> {
                match step_id {
                    #(#execute_step_arms)*
                    _ => Err(tupa_engine::EngineError::Other(format!("unknown step '{}'", step_id))),
                }
            }

            fn check_constraints(
                values: &std::collections::HashMap<String, tupa_core::serde_json::Value>,
            ) -> (bool, Vec<tupa_engine::ConstraintFailure>) {
                Self::check_constraints(values)
            }
        }

        // Step methods
        impl #name {
            #step_methods
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pipeline() {
        let input = r#"
            name: TestPipeline,
            input: i64,
            steps: [
                step("double") { input * 2 }
            ],
            constraints: [
                metric("result").ge(10)
            ]
        "#;

        let ast: PipelineInput = syn::parse_str(input).unwrap();
        assert_eq!(ast.name.to_string(), "TestPipeline");
        assert_eq!(ast.steps.len(), 1);
        assert_eq!(ast.steps[0].id, "double");
        assert_eq!(ast.constraints.len(), 1);
        assert_eq!(ast.constraints[0].metric_name, "result");
        assert_eq!(ast.constraints[0].value, 10.0);
        assert!(matches!(ast.constraints[0].op, ConstraintOp::Ge));
    }
}
