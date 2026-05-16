//! Expand `pipeline!` macro blocks to readable Rust source code.
//!
//! Usage (as cargo-tupa subcommand):
//!   cargo tupa expand [--pretty] [--file <path>]
//!
//! Reads Rust source files, finds `pipeline! { ... }` blocks, and prints
//! the generated Rust code that the macro would emit at compile time.
//!
//! Use `--pretty` for indented, human-readable output (via prettyplease).
//!
//! This is a self-contained reimplementation of the `pipeline!` macro's
//! code generation — it does **not** depend on `tupa-core-macros` exporting
//! any public types (proc-macro crates cannot export types to other crates).

use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{braced, Expr, Ident, Lit, LitStr, Type};
use walkdir::WalkDir;

// =============================================================================
// Internal AST (mirrors tupa-core-macros/src/lib.rs private types)
// =============================================================================

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

// =============================================================================
// Parsers
// =============================================================================

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
        let mut produces = None;
        let mut requires = None;
        while !input.is_empty() {
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
                    ));
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
        input.parse::<syn::Token![:]>()?;
        let name = input.parse()?;
        input.parse::<Comma>()?;
        let input_keyword: Ident = input.parse()?;
        if input_keyword != "input" {
            return Err(syn::Error::new_spanned(
                input_keyword,
                "expected keyword `input`",
            ));
        }
        input.parse::<syn::Token![:]>()?;
        input_type = input.parse()?;
        input.parse::<Comma>()?;
        let steps_keyword: Ident = input.parse()?;
        if steps_keyword != "steps" {
            return Err(syn::Error::new_spanned(
                steps_keyword,
                "expected keyword `steps`",
            ));
        }
        input.parse::<syn::Token![:]>()?;
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
        input.parse::<syn::Token![:]>()?;
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

// =============================================================================
// Code generation (standalone — mirrors tupa-core-macros/src/generate.rs)
// =============================================================================

fn generate_step_methods(steps: &[StepDecl]) -> TokenStream {
    let mut methods = Vec::new();
    for step in steps {
        let id = &step.id;
        let body = &step.body;
        let method_name = Ident::new(&format!("step_{}", id), proc_macro2::Span::call_site());
        methods.push(quote! {
            pub fn #method_name(
                &self,
                input: &<Self as tupa_core::Pipeline>::Input,
            ) -> Result<tupa_core::serde_json::Value, tupa_engine::EngineError> {
                let result = #body;
                tupa_core::serde_json::to_value(&result)
                    .map_err(|e| tupa_engine::EngineError::Other(e.to_string()))
            }
        });
    }
    quote! { #(#methods)* }
}

fn generate_metadata_methods(steps: &[StepDecl]) -> TokenStream {
    let mut impls = Vec::new();
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

fn generate_step_calls(steps: &[StepDecl]) -> TokenStream {
    let calls = steps.iter().map(|step| {
        let id = &step.id;
        let method_name = Ident::new(&format!("step_{}", id), proc_macro2::Span::call_site());
        let produces = step
            .produces
            .as_ref()
            .map(|v| v.iter().collect::<Vec<_>>())
            .unwrap_or_else(|| vec![&step.id]);
        quote! {
            let val = self.#method_name(input)?;
            #(
                values.insert(#produces.to_string(), val.clone());
            )*
        }
    });
    quote! { #(#calls)* }
}

fn generate_constraint_checks(constraints: &[ConstraintDecl]) -> TokenStream {
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
            if let Some(v) = values.get(#metric_name).and_then(|val| val.as_f64()) {
                if !(#condition) {
                    failures.push(tupa_engine::ConstraintFailure {
                        metric: #metric_name.to_string(),
                        expected: #op_str.to_string(),
                        actual: v.to_string(),
                    });
                }
            }
        }
    });
    quote! { #(#checks)* }
}

fn generate_pipeline_impl(
    name: &Ident,
    input_type: &Type,
    steps: &[StepDecl],
    constraints: &[ConstraintDecl],
) -> Result<TokenStream> {
    let step_methods = generate_step_methods(steps);
    let metadata_methods = generate_metadata_methods(steps);
    let step_calls = generate_step_calls(steps);
    let constraint_checks = generate_constraint_checks(constraints);
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
    let execute_step_arms: Vec<TokenStream> = steps
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
    Ok(quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub struct #name;
        impl tupa_core::Pipeline for #name {
            type Input = #input_type;
            fn name(&self) -> &'static str {
                stringify!(#name)
            }
        }
        impl #name {
            pub fn new() -> Self {
                Self
            }
            #[doc(hidden)]
            pub fn check_constraints(
                values: &std::collections::HashMap<String, tupa_core::serde_json::Value>,
            ) -> (bool, Vec<tupa_engine::ConstraintFailure>) {
                let mut failures = Vec::new();
                #constraint_checks
                (failures.is_empty(), failures)
            }
            #metadata_methods
        }
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
                    _ => Err(tupa_engine::EngineError::Other(
                        format!("unknown step '{}'", step_id)
                    )),
                }
            }
            fn check_constraints(
                values: &std::collections::HashMap<String, tupa_core::serde_json::Value>,
            ) -> (bool, Vec<tupa_engine::ConstraintFailure>) {
                Self::check_constraints(values)
            }
        }
        impl #name {
            #step_methods
        }
    })
}

// =============================================================================
// CLI entry point
// =============================================================================

/// Expand pipeline! blocks in the given file(s).
///
/// If `file` is None, walk `src/` for `.rs` files.
/// If `pretty` is true, format output with `prettyplease`.
pub fn expand_pipeline_block(file: Option<PathBuf>, pretty: bool) -> Result<()> {
    let files = if let Some(ref f) = file {
        vec![f.clone()]
    } else {
        let mut files = Vec::new();
        for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path.to_path_buf());
            }
        }
        if files.is_empty() {
            eprintln!("No .rs files found in src/");
            return Ok(());
        }
        files
    };

    for file in files {
        match expand_file(&file) {
            Ok(Some(output)) => {
                if pretty {
                    match syn::parse_file(&output) {
                        Ok(parsed_file) => {
                            println!(
                                "// === {} ===\n{}",
                                file.display(),
                                prettyplease::unparse(&parsed_file)
                            );
                        }
                        Err(_) => {
                            // Fallback: print raw
                            println!("// === {} ===\n{}", file.display(), output);
                        }
                    }
                } else {
                    println!("// === {} ===\n{}", file.display(), output);
                }
            }
            Ok(None) => {
                if file.as_os_str().to_string_lossy().contains("stdin") {
                    eprintln!("No pipeline! blocks found.");
                }
            }
            Err(e) => eprintln!("Error expanding {}: {}", file.display(), e),
        }
    }
    Ok(())
}

/// Try to expand pipeline! blocks in a single file.
/// Returns `Ok(None)` if no pipeline! blocks are found.
fn expand_file(file: &PathBuf) -> Result<Option<String>> {
    let content = fs::read_to_string(file)?;
    if !content.contains("pipeline!") {
        return Ok(None);
    }
    let mut result = String::new();
    let mut lines = content.lines().peekable();
    let mut found_any = false;
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.starts_with("pipeline!") {
            found_any = true;
            let mut block = line.to_string();
            let mut brace_depth = block.matches('{').count() - block.matches('}').count();
            while brace_depth > 0 {
                match lines.next() {
                    Some(next) => {
                        block.push('\n');
                        block.push_str(next);
                        brace_depth += next.matches('{').count();
                        brace_depth -= next.matches('}').count();
                    }
                    None => anyhow::bail!("Unclosed pipeline! macro in {}", file.display()),
                }
            }
            match do_expand(&block) {
                Ok(expanded) => {
                    result.push_str(&expanded);
                    result.push('\n');
                }
                Err(e) => {
                    eprintln!("// Expansion failed: {}", e);
                    result.push_str("// [expand failed: ");
                    result.push_str(&e.to_string());
                    result.push_str("]\n");
                }
            }
        }
    }
    if !found_any {
        return Ok(None);
    }
    Ok(Some(result))
}

/// Parse a `pipeline! { ... }` block and emit generated Rust via codegen.
fn do_expand(block: &str) -> Result<String> {
    let inner = block.find('{').ok_or_else(|| anyhow!("no opening brace"))?;
    let inner = &block[inner + 1..];
    let end = inner
        .rfind('}')
        .ok_or_else(|| anyhow!("no closing brace"))?;
    let inner = &inner[..end];
    let ast: PipelineInput = syn::parse_str(inner)?;
    let tokens = expand_pipeline_block_impl(&ast)?;
    Ok(tokens.to_string())
}

/// Generate Rust tokens from a parsed pipeline AST.
fn expand_pipeline_block_impl(ast: &PipelineInput) -> Result<TokenStream> {
    let name = &ast.name;
    let input_type = &ast.input_type;
    let steps = &ast.steps;
    let constraints = &ast.constraints;
    generate_pipeline_impl(name, input_type, steps, constraints)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_simple_pipeline() -> Result<()> {
        let block = r#"pipeline! {
            name: TestPipe,
            input: i32,
            steps: [step("a"){input+1}],
            constraints: [metric("out").ge(0)]
        }"#;
        let inner = block.find('{').unwrap() + 1;
        let end = block.rfind('}').unwrap();
        let inner = &block[inner..end];
        let ast: PipelineInput = syn::parse_str(inner)?;
        let tokens = expand_pipeline_block_impl(&ast)?;
        let out = tokens.to_string();
        assert!(
            out.contains("TestPipe"),
            "output must contain pipeline name"
        );
        assert!(
            out.contains("tupa_core :: Pipeline"),
            "must contain Pipeline impl: {}",
            out
        );
        assert!(
            out.contains("tupa_engine :: ExecutorPipeline"),
            "must contain ExecutorPipeline impl: {}",
            out
        );
        assert!(
            out.contains("step_a"),
            "must contain step_a method: {}",
            out
        );
        Ok(())
    }

    #[test]
    fn test_expand_with_constraint_metric() -> Result<()> {
        let block = r#"pipeline! {
            name: CreditCheck,
            input: LoanApp,
            steps: [step("score") { credit_score(input) }],
            constraints: [metric("risk").le(0.5)]
        }"#;
        let inner = block.find('{').unwrap() + 1;
        let end = block.rfind('}').unwrap();
        let inner = &block[inner..end];
        let ast: PipelineInput = syn::parse_str(inner)?;
        let tokens = expand_pipeline_block_impl(&ast)?;
        let out = tokens.to_string();
        assert!(out.contains("CreditCheck"));
        assert!(out.contains("risk"));
        assert!(out.contains("step_score"));
        Ok(())
    }

    #[test]
    fn test_expand_idempotent() -> Result<()> {
        let block = r#"pipeline! {
            name: Idem,
            input: (),
            steps: [step("a"){1}],
            constraints: []
        }"#;
        let inner = block.find('{').unwrap() + 1;
        let end = block.rfind('}').unwrap();
        let inner = &block[inner..end];
        let ast: PipelineInput = syn::parse_str(inner)?;
        let t1 = expand_pipeline_block_impl(&ast)?.to_string();
        let t2 = expand_pipeline_block_impl(&ast)?.to_string();
        assert_eq!(t1, t2, "expansion must be idempotent");
        Ok(())
    }
}
