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
use syn::{
    braced, punctuated::Punctuated, token::Comma, BinOp, Expr, Ident, Lit, LitStr, Type, UnOp,
};

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
///
/// ## Diagnostic codes
///
/// | Code | Description |
/// |------|-------------|
/// | E100 | Duplicate step name |
/// | E101 | Cycle detected in `requires`/`produces` dependencies |
/// | E102 | `requires` references a step that doesn't exist |
/// | E103 | `produces` references a step that doesn't exist |
/// | E104 | Constraint references a metric not produced by any step |
/// | E105 | Constraint value is not a numeric literal |
/// | E106 | Unknown constraint operator (use `.ge()`, `.le()`, `.eq()`, `.ne()`, `.gt()`, `.lt()`) |
/// | E107 | Empty pipeline — no steps defined |
/// | E108 | Step name is a Rust keyword |
/// | E109 | Step name starts with a digit |
#[proc_macro]
pub fn pipeline(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as PipelineInput);

    // --- Validation: E100 duplicate step names ---
    let mut seen_names = std::collections::HashSet::new();
    for step in &ast.steps {
        if !seen_names.insert(&step.id) {
            return syn::Error::new_spanned(
                &ast.name,
                format!("[E100] Duplicate step name: '{}'", step.id),
            )
            .to_compile_error()
            .into();
        }
    }

    // --- Validation: E107 empty steps ---
    if ast.steps.is_empty() {
        return syn::Error::new_spanned(
            &ast.name,
            "[E107] Empty pipeline — at least one step is required",
        )
        .to_compile_error()
        .into();
    }

    // --- Validation: E102 requires references a metric not produced by any step ---
    // Collect all metrics produced by any step (explicit or default)
    let produced_metrics: std::collections::HashSet<_> = ast
        .steps
        .iter()
        .flat_map(|s| {
            s.produces
                .as_ref()
                .map(|v| v.iter().map(|m| m.as_str()).collect::<Vec<_>>())
                .unwrap_or_else(|| vec![s.id.as_str()])
        })
        .collect();
    for step in &ast.steps {
        if let Some(ref reqs) = step.requires {
            for req in reqs {
                if !produced_metrics.contains(req.as_str()) {
                    return syn::Error::new_spanned(
                        &ast.name,
                        format!("[E102] requires() references undefined metric '{}'", req),
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
    }

    // --- Validation: E108 step name is a Rust keyword ---
    let rust_keywords: &[&str] = &[
        "fn",
        "let",
        "match",
        "if",
        "else",
        "for",
        "while",
        "loop",
        "return",
        "struct",
        "enum",
        "impl",
        "trait",
        "type",
        "const",
        "static",
        "mod",
        "use",
        "crate",
        "self",
        "Self",
        "super",
        "move",
        "ref",
        "as",
        "break",
        "continue",
        "dyn",
        "where",
        "async",
        "await",
        "unsafe",
        "extern",
        "true",
        "false",
        "in",
        "select",
        "select_neq",
    ];
    for step in &ast.steps {
        if rust_keywords.contains(&step.id.as_str()) {
            return syn::Error::new_spanned(
                &ast.name,
                format!("[E108] Step name '{}' is a Rust keyword", step.id),
            )
            .to_compile_error()
            .into();
        }
    }

    // --- Validation: E109 step name starts with a digit ---
    for step in &ast.steps {
        if step.id.starts_with(|c: char| c.is_ascii_digit()) {
            return syn::Error::new_spanned(
                &ast.name,
                format!("[E109] Step name '{}' starts with a digit", step.id),
            )
            .to_compile_error()
            .into();
        }
    }

    // --- Validation: E101 cycle detection via topological sort ---
    // Build a map: metric -> step index that produces it
    let metric_to_step: std::collections::HashMap<&str, usize> = ast
        .steps
        .iter()
        .enumerate()
        .flat_map(|(idx, s)| {
            let metrics = s
                .produces
                .as_ref()
                .map(|v| v.iter().map(|m| m.as_str()).collect::<Vec<_>>())
                .unwrap_or_else(|| vec![s.id.as_str()]);
            metrics.into_iter().map(move |m| (m, idx))
        })
        .collect();

    for (step_idx, step) in ast.steps.iter().enumerate() {
        if let Some(ref reqs) = step.requires {
            for req in reqs {
                if let Some(&producer_idx) = metric_to_step.get(req.as_str()) {
                    if producer_idx >= step_idx {
                        return syn::Error::new_spanned(
                            &ast.name,
                            format!(
                                "[E101] Cycle detected: step '{}' requires metric '{}' produced by later step",
                                step.id, req
                            ),
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
        }
    }

    // --- Validation: E104 constraint references unknown metric ---
    // Collect all metric names produced by steps (explicit produces or step id default)
    let produced_metrics: std::collections::HashSet<_> = ast
        .steps
        .iter()
        .flat_map(|s| {
            s.produces
                .as_ref()
                .map(|v| v.iter().map(|m| m.as_str()).collect::<Vec<_>>())
                .unwrap_or_else(|| vec![s.id.as_str()])
        })
        .collect();
    for c in &ast.constraints {
        if !produced_metrics.contains(c.metric_name.as_str()) {
            return syn::Error::new_spanned(
                &ast.name,
                format!(
                    "[E104] Constraint on '{}' but no step produces it (produces: {:?})",
                    c.metric_name,
                    produced_metrics.iter().collect::<Vec<_>>()
                ),
            )
            .to_compile_error()
            .into();
        }
    }

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

// ============================================================================
// `safe!` macro — compile-time-proven constrained values (experimental)
// ============================================================================

/// Input grammar: `safe!(MarkerType, expr)`.
struct SafeInput {
    marker: Type,
    expr: Expr,
}

impl Parse for SafeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let marker: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let expr: Expr = input.parse()?;
        Ok(SafeInput { marker, expr })
    }
}

/// Constant-fold an `f64` expression: literals, parentheses, unary `-`, and
/// `+ - * /` between folded operands. Returns `None` when not foldable.
fn fold_f64(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Float(f) => f.base10_parse::<f64>().ok(),
            Lit::Int(i) => i.base10_parse::<f64>().ok(),
            _ => None,
        },
        Expr::Paren(p) => fold_f64(&p.expr),
        Expr::Group(g) => fold_f64(&g.expr),
        Expr::Unary(u) => match u.op {
            UnOp::Neg(_) => fold_f64(&u.expr).map(|v| -v),
            _ => None,
        },
        Expr::Binary(b) => {
            let l = fold_f64(&b.left)?;
            let r = fold_f64(&b.right)?;
            match b.op {
                BinOp::Add(_) => Some(l + r),
                BinOp::Sub(_) => Some(l - r),
                BinOp::Mul(_) => Some(l * r),
                BinOp::Div(_) => Some(l / r),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Last path segment of a marker type, e.g. `tupa_core::constraints::NonNan` -> `"NonNan"`.
fn marker_ident(ty: &Type) -> Option<String> {
    if let Type::Path(tp) = ty {
        tp.path.segments.last().map(|s| s.ident.to_string())
    } else {
        None
    }
}

/// Construct a `Safe<f64, Marker>`, proving built-in constraints at compile time
/// for constant expressions and falling back to a runtime check otherwise.
///
/// ```ignore
/// let a = safe!(NonNan, 1.0 + 2.0); // proven at compile time
/// let b = safe!(NonNan, x);         // runtime-checked guard
/// ```
#[proc_macro]
pub fn safe(input: TokenStream) -> TokenStream {
    let SafeInput { marker, expr } = syn::parse_macro_input!(input as SafeInput);

    if let Some(value) = fold_f64(&expr) {
        // Constant expression: try to prove a built-in constraint at compile time.
        let proven = match marker_ident(&marker).as_deref() {
            Some("NonNan") => Some(!value.is_nan()),
            Some("NonInf") => Some(!value.is_infinite()),
            Some("Finite") => Some(value.is_finite()),
            _ => None, // unknown marker: cannot reason at compile time
        };
        if proven == Some(false) {
            let name = marker_ident(&marker).unwrap_or_default();
            let msg = format!(
                "E3002: cannot prove constraint `{}` — constant expression evaluates to {}",
                name, value
            );
            return syn::Error::new_spanned(&expr, msg)
                .to_compile_error()
                .into();
        }
        quote! { tupa_core::Safe::<f64, #marker>::new_unchecked(#expr) }.into()
    } else {
        // Non-constant expression: fall back to a runtime guard.
        quote! {
            tupa_core::Safe::<f64, #marker>::try_new(#expr)
                .expect(concat!("constraint violated at runtime: ", stringify!(#marker)))
        }
        .into()
    }
}

#[cfg(test)]
mod tests;
