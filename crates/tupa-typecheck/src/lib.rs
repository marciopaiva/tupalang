use std::collections::HashMap;

use thiserror::Error;
use tupa_lexer::Span;
use tupa_parser::{
    BinaryOp, Expr, ExprKind, Function, Item, Pattern, Program, Stmt, Type, UnaryOp,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    I64,
    F64,
    Bool,
    String,
    Null,
    Unit,
    Array {
        elem: Box<Ty>,
        len: i64,
    },
    Slice {
        elem: Box<Ty>,
    },
    Func {
        params: Vec<Ty>,
        ret: Box<Ty>,
    },
    Closure {
        params: Vec<Ty>,
        ret: Box<Ty>,
        captured: Vec<String>,
    },
    Enum(String),
    Trait(String),
    Unknown,
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("unknown type '{name}'{suggestion}")]
    UnknownType { name: String, suggestion: String },
    #[error("undefined variable '{name}'{suggestion}")]
    UnknownVar {
        name: String,
        suggestion: String,
        span: Option<Span>,
    },
    #[error("undefined function '{name}'{suggestion}")]
    UnknownFunction {
        name: String,
        suggestion: String,
        span: Option<Span>,
    },
    #[error("type mismatch: expected {expected:?}, got {found:?}")]
    Mismatch {
        expected: Ty,
        found: Ty,
        span: Option<Span>,
    },
    #[error("arity mismatch: expected {expected}, got {found}")]
    ArityMismatch {
        expected: usize,
        found: usize,
        span: Option<Span>,
    },
    #[error("invalid operand types for {op:?}: {left:?}, {right:?}")]
    InvalidBinary {
        op: BinaryOp,
        left: Ty,
        right: Ty,
        span: Option<Span>,
    },
    #[error("invalid operand type for {op:?}: {found:?}")]
    InvalidUnary {
        op: UnaryOp,
        found: Ty,
        span: Option<Span>,
    },
    #[error("invalid call target: {found:?}")]
    InvalidCallTarget { found: Ty, span: Option<Span> },
    #[error("return type mismatch: expected {expected:?}, got {found:?}")]
    ReturnMismatch {
        expected: Ty,
        found: Ty,
        span: Option<Span>,
    },
    #[error("expected function body to return a value")]
    MissingReturn { span: Option<Span> },
    #[error("invalid constraint '{constraint}' for base type {base:?}")]
    InvalidConstraint {
        constraint: String,
        base: Ty,
        span: Option<Span>,
    },
    #[error("cannot prove constraint '{constraint}' at compile time")]
    UnprovenConstraint {
        constraint: String,
        span: Option<Span>,
    },
    #[error("break outside of loop")]
    BreakOutsideLoop { span: Option<Span> },
    #[error("continue outside of loop")]
    ContinueOutsideLoop { span: Option<Span> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Warning {
    UnusedVar(String),
}

#[derive(Debug, Clone)]
struct ExpectedReturn {
    ty: Ty,
    constraints: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
struct TypeSig {
    ty: Ty,
    constraints: Option<Vec<String>>,
}

#[allow(clippy::result_large_err)]
pub fn typecheck_program(program: &Program) -> Result<(), TypeError> {
    let _ = typecheck_program_with_warnings(program)?;
    Ok(())
}

// Collect all variables referenced in an expression
fn collect_vars(expr: &Expr, vars: &mut std::collections::HashSet<String>) {
    match &expr.kind {
        ExprKind::Ident(name) => {
            vars.insert(name.clone());
        }
        ExprKind::Assign { name, expr } => {
            vars.insert(name.clone());
            collect_vars(expr, vars);
        }
        ExprKind::AssignIndex { expr, index, value } => {
            collect_vars(expr, vars);
            collect_vars(index, vars);
            collect_vars(value, vars);
        }
        ExprKind::Index { expr, index } => {
            collect_vars(expr, vars);
            collect_vars(index, vars);
        }
        ExprKind::Unary { expr, .. } => {
            collect_vars(expr, vars);
        }
        ExprKind::Binary { left, right, .. } => {
            collect_vars(left, vars);
            collect_vars(right, vars);
        }
        ExprKind::Call { callee, args } => {
            collect_vars(callee, vars);
            for arg in args {
                collect_vars(arg, vars);
            }
        }
        ExprKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            collect_vars(condition, vars);
            collect_vars_block(then_branch, vars);
            if let Some(else_branch) = else_branch {
                match else_branch {
                    tupa_parser::ElseBranch::Block(block) => collect_vars_block(block, vars),
                    tupa_parser::ElseBranch::If(expr) => collect_vars(expr, vars),
                }
            }
        }
        ExprKind::Match { expr, arms } => {
            collect_vars(expr, vars);
            for arm in arms {
                collect_vars(&arm.expr, vars);
                if let Some(guard) = &arm.guard {
                    collect_vars(guard, vars);
                }
            }
        }
        ExprKind::Block(stmts) => {
            collect_vars_block(stmts, vars);
        }
        ExprKind::Lambda { params, body } => {
            // Don't collect params as captured vars
            let mut body_vars = std::collections::HashSet::new();
            collect_vars(body, &mut body_vars);
            for param in params {
                body_vars.remove(param);
            }
            vars.extend(body_vars);
        }
        // Literals don't reference variables
        ExprKind::Int(_)
        | ExprKind::Float(_)
        | ExprKind::Bool(_)
        | ExprKind::Str(_)
        | ExprKind::Null => {}
        ExprKind::ArrayLiteral(items) => {
            for item in items {
                collect_vars(item, vars);
            }
        }
        ExprKind::Field { expr, .. } => {
            collect_vars(expr, vars);
        }
        ExprKind::Await(expr) => {
            collect_vars(expr, vars);
        }
    }
}

fn collect_vars_block(stmts: &[Stmt], vars: &mut std::collections::HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Stmt::Let { expr, .. } => {
                collect_vars(expr, vars);
            }
            Stmt::While { condition, body } => {
                collect_vars(condition, vars);
                collect_vars_block(body, vars);
            }
            Stmt::For { iter, body, .. } => {
                collect_vars(iter, vars);
                collect_vars_block(body, vars);
            }
            Stmt::Expr(expr) => {
                collect_vars(expr, vars);
            }
            Stmt::Return(Some(expr)) => {
                collect_vars(expr, vars);
            }
            // Other statements don't introduce new variables
            Stmt::Break | Stmt::Continue | Stmt::Return(None) | Stmt::Lambda { .. } => {}
        }
    }
}

// Infer parameter types for lambda by analyzing usage
#[allow(clippy::result_large_err)]
fn infer_lambda_param_types(
    expr: &Expr,
    env: &mut TypeEnv,
    param_types: &mut Vec<Ty>,
    functions: &HashMap<String, FuncSig>,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
    expected_return: &ExpectedReturn,
) -> Result<(), TypeError> {
    match &expr.kind {
        ExprKind::Ident(_name) => {
            // If this is a parameter, we can't infer its type from usage here
            // But we can collect constraints from its usage context
        }
        ExprKind::Binary { left, right, .. } => {
            infer_lambda_param_types(
                left,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
            infer_lambda_param_types(
                right,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
        }
        ExprKind::Unary { expr, .. } => {
            infer_lambda_param_types(
                expr,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
        }
        ExprKind::Call { callee, args } => {
            // Try to infer parameter types from function call arguments
            if let ExprKind::Ident(func_name) = &callee.kind {
                if let Some(sig) = functions.get(func_name) {
                    for (arg_expr, expected_ty) in args.iter().zip(&sig.params) {
                        infer_param_type_from_expr(
                            arg_expr,
                            expected_ty.ty.clone(),
                            param_types,
                            env,
                        )?;
                    }
                }
            }
            infer_lambda_param_types(
                callee,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
            for arg in args {
                infer_lambda_param_types(
                    arg,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
            }
        }
        ExprKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            infer_lambda_param_types(
                condition,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
            infer_lambda_param_types_block(
                then_branch,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
            if let Some(else_branch) = else_branch {
                match else_branch {
                    tupa_parser::ElseBranch::Block(block) => infer_lambda_param_types_block(
                        block,
                        env,
                        param_types,
                        functions,
                        enums,
                        traits,
                        expected_return,
                    )?,
                    tupa_parser::ElseBranch::If(expr) => infer_lambda_param_types(
                        expr,
                        env,
                        param_types,
                        functions,
                        enums,
                        traits,
                        expected_return,
                    )?,
                }
            }
        }
        ExprKind::Match { expr, arms } => {
            infer_lambda_param_types(
                expr,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
            for arm in arms {
                infer_lambda_param_types(
                    &arm.expr,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
                if let Some(guard) = &arm.guard {
                    infer_lambda_param_types(
                        guard,
                        env,
                        param_types,
                        functions,
                        enums,
                        traits,
                        expected_return,
                    )?;
                }
            }
        }
        ExprKind::Block(stmts) => {
            infer_lambda_param_types_block(
                stmts,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
        }
        ExprKind::Lambda { body, .. } => {
            // Nested lambdas - recurse
            infer_lambda_param_types(
                body,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
        }
        ExprKind::Field { expr, .. } => {
            infer_lambda_param_types(
                expr,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
        }
        ExprKind::Await(expr) => {
            infer_lambda_param_types(
                expr,
                env,
                param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;
        }
        // Other expressions don't provide parameter type information
        _ => {}
    }
    Ok(())
}

#[allow(clippy::result_large_err)]
fn infer_lambda_param_types_block(
    stmts: &[Stmt],
    env: &mut TypeEnv,
    param_types: &mut Vec<Ty>,
    functions: &HashMap<String, FuncSig>,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
    expected_return: &ExpectedReturn,
) -> Result<(), TypeError> {
    for stmt in stmts {
        match stmt {
            Stmt::Let { expr, .. } => {
                infer_lambda_param_types(
                    expr,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
            }
            Stmt::While { condition, body } => {
                infer_lambda_param_types(
                    condition,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
                infer_lambda_param_types_block(
                    body,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
            }
            Stmt::For { iter, body, .. } => {
                infer_lambda_param_types(
                    iter,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
                infer_lambda_param_types_block(
                    body,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
            }
            Stmt::Expr(expr) => {
                infer_lambda_param_types(
                    expr,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
            }
            Stmt::Return(Some(expr)) => {
                infer_lambda_param_types(
                    expr,
                    env,
                    param_types,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[allow(clippy::result_large_err)]
fn infer_param_type_from_expr(
    expr: &Expr,
    expected_ty: Ty,
    param_types: &mut Vec<Ty>,
    env: &TypeEnv,
) -> Result<(), TypeError> {
    match &expr.kind {
        ExprKind::Ident(name) => {
            // If this is a parameter, set its type
            if let Some(idx) = find_param_index(name, env) {
                if param_types[idx] == Ty::Unknown {
                    param_types[idx] = expected_ty;
                } else if param_types[idx] != expected_ty {
                    // Type conflict - for now, keep the first type we found
                    // In a more sophisticated system, we'd unify types
                }
            }
        }
        ExprKind::Binary { left, right, .. } => {
            infer_param_type_from_expr(left, expected_ty.clone(), param_types, env)?;
            infer_param_type_from_expr(right, expected_ty, param_types, env)?;
        }
        ExprKind::Unary { expr, .. } => {
            infer_param_type_from_expr(expr, expected_ty, param_types, env)?;
        }
        ExprKind::Call { args, .. } => {
            for arg in args {
                infer_param_type_from_expr(arg, expected_ty.clone(), param_types, env)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn find_param_index(name: &str, env: &TypeEnv) -> Option<usize> {
    env.param_indices.get(name).copied()
}

#[allow(clippy::result_large_err)]
fn validate_safe_constraints(
    constraints: &[String],
    base: &Ty,
    expr: &Expr,
    env: &TypeEnv,
    functions: &HashMap<String, FuncSig>,
) -> Result<(), TypeError> {
    let literal = eval_f64_const(expr);
    let proven = expr_constraints(expr, env, functions);
    for constraint in constraints {
        match constraint.as_str() {
            "nan" | "inf" => {
                if *base != Ty::F64 {
                    return Err(TypeError::InvalidConstraint {
                        constraint: constraint.clone(),
                        base: base.clone(),
                        span: Some(expr.span),
                    });
                }
                if proven
                    .as_ref()
                    .is_some_and(|known| known.iter().any(|known| known == constraint))
                {
                    continue;
                }
                let Some(value) = literal else {
                    return Err(TypeError::UnprovenConstraint {
                        constraint: constraint.clone(),
                        span: Some(expr.span),
                    });
                };
                if (constraint == "nan" && value.is_nan())
                    || (constraint == "inf" && value.is_infinite())
                {
                    return Err(TypeError::UnprovenConstraint {
                        constraint: constraint.clone(),
                        span: Some(expr.span),
                    });
                }
            }
            "hate_speech" | "misinformation" => {
                if *base != Ty::String {
                    return Err(TypeError::InvalidConstraint {
                        constraint: constraint.clone(),
                        base: base.clone(),
                        span: Some(expr.span),
                    });
                }
                if proven
                    .as_ref()
                    .is_some_and(|known| known.iter().any(|known| known == constraint))
                {
                    continue;
                }
                return Err(TypeError::UnprovenConstraint {
                    constraint: constraint.clone(),
                    span: Some(expr.span),
                });
            }
            _ => {
                return Err(TypeError::InvalidConstraint {
                    constraint: constraint.clone(),
                    base: base.clone(),
                    span: Some(expr.span),
                });
            }
        }
    }
    Ok(())
}

fn expr_constraints(
    expr: &Expr,
    env: &TypeEnv,
    functions: &HashMap<String, FuncSig>,
) -> Option<Vec<String>> {
    match &expr.kind {
        ExprKind::Ident(name) => env.get_var_constraints(name),
        ExprKind::Call { callee, .. } => {
            if let ExprKind::Ident(name) = &callee.kind {
                return functions
                    .get(name)
                    .and_then(|sig| sig.ret.constraints.clone());
            }
            None
        }
        _ => None,
    }
}

fn suggestion_message(suggestion: Option<String>) -> String {
    suggestion
        .map(|value| format!(" (did you mean '{value}'?)"))
        .unwrap_or_default()
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    for (i, ca) in a_chars.iter().enumerate() {
        let mut curr = vec![i + 1; b_chars.len() + 1];
        for (j, cb) in b_chars.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            let deletion = prev[j + 1] + 1;
            let insertion = curr[j] + 1;
            let substitution = prev[j] + cost;
            curr[j + 1] = deletion.min(insertion).min(substitution);
        }
        prev = curr;
    }
    prev[b_chars.len()]
}

fn best_suggestion(target: &str, candidates: impl IntoIterator<Item = String>) -> Option<String> {
    let mut best: Option<(String, usize)> = None;
    for candidate in candidates {
        if candidate == target {
            continue;
        }
        let distance = levenshtein(target, &candidate);
        let max_len = target.chars().count().max(candidate.chars().count());
        let threshold = if max_len <= 3 {
            1
        } else if max_len <= 6 {
            2
        } else {
            3
        };
        if distance <= threshold {
            match &best {
                None => best = Some((candidate, distance)),
                Some((_, best_distance)) if distance < *best_distance => {
                    best = Some((candidate, distance))
                }
                _ => {}
            }
        }
    }
    best.map(|(name, _)| name)
}

fn eval_f64_const(expr: &Expr) -> Option<f64> {
    match &expr.kind {
        ExprKind::Float(value) => Some(*value),
        ExprKind::Unary {
            op: UnaryOp::Neg,
            expr,
        } => eval_f64_const(expr).map(|value| -value),
        ExprKind::Binary { op, left, right } => {
            let lhs = eval_f64_const(left)?;
            let rhs = eval_f64_const(right)?;
            match op {
                BinaryOp::Add => Some(lhs + rhs),
                BinaryOp::Sub => Some(lhs - rhs),
                BinaryOp::Mul => Some(lhs * rhs),
                BinaryOp::Div => Some(lhs / rhs),
                BinaryOp::Pow => Some(lhs.powf(rhs)),
                _ => None,
            }
        }
        _ => None,
    }
}

#[allow(clippy::result_large_err)]
pub fn typecheck_program_with_warnings(program: &Program) -> Result<Vec<Warning>, TypeError> {
    let mut functions = HashMap::new();
    let mut enums = HashMap::new();
    let mut traits = HashMap::new();
    for item in &program.items {
        match item {
            Item::Function(func) => {
                let params = func
                    .params
                    .iter()
                    .map(|p| type_sig_from_ast(&p.ty, &enums, &traits))
                    .collect::<Result<Vec<_>, _>>()?;
                let ret = match func.return_type.as_ref() {
                    Some(ty) => type_sig_from_ast(ty, &enums, &traits)?,
                    None => TypeSig {
                        ty: Ty::Unit,
                        constraints: None,
                    },
                };
                functions.insert(func.name.clone(), FuncSig { params, ret });
            }
            Item::Enum(enum_def) => {
                enums.insert(enum_def.name.clone(), enum_def.variants.clone());
            }
            Item::Trait(trait_def) => {
                traits.insert(trait_def.name.clone(), trait_def.methods.clone());
            }
        }
    }
    let mut warnings = Vec::new();
    for item in &program.items {
        match item {
            Item::Function(func) => {
                warnings.extend(typecheck_function(func, &functions, &enums, &traits)?)
            }
            Item::Enum(_) => {} // enums don't need typechecking beyond declaration
            Item::Trait(_) => {} // traits don't need typechecking beyond declaration
        }
    }
    Ok(warnings)
}

#[allow(clippy::result_large_err)]
fn typecheck_function(
    func: &Function,
    functions: &HashMap<String, FuncSig>,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
) -> Result<Vec<Warning>, TypeError> {
    let mut env = TypeEnv::default();
    for param in &func.params {
        let sig = type_sig_from_ast(&param.ty, enums, traits)?;
        env.insert_var(param.name.clone(), sig.ty, sig.constraints);
    }

    let expected_return = match func.return_type.as_ref() {
        Some(Type::Safe { base, constraints }) => ExpectedReturn {
            ty: type_from_ast(base, enums, traits)?,
            constraints: Some(constraints.clone()),
        },
        Some(ty) => ExpectedReturn {
            ty: type_from_ast(ty, enums, traits)?,
            constraints: None,
        },
        None => ExpectedReturn {
            ty: Ty::Unit,
            constraints: None,
        },
    };

    for stmt in &func.body {
        typecheck_stmt(stmt, &mut env, functions, enums, traits, &expected_return)?;
    }

    if expected_return.ty != Ty::Unit && !block_returns(&func.body) {
        if let Some(Stmt::Expr(expr)) = func.body.last() {
            let found = type_of_expr(expr, &mut env, functions, enums, traits, &expected_return)?;
            if found != expected_return.ty {
                if found == Ty::Unit {
                    return Err(TypeError::MissingReturn {
                        span: Some(expr.span),
                    });
                }
                return Err(TypeError::ReturnMismatch {
                    expected: expected_return.ty.clone(),
                    found,
                    span: Some(expr.span),
                });
            }
            if let Some(constraints) = expected_return.constraints.as_ref() {
                validate_safe_constraints(constraints, &expected_return.ty, expr, &env, functions)?;
            }
        } else {
            let span = func.body.last().and_then(stmt_span);
            return Err(TypeError::MissingReturn { span });
        }
    }

    Ok(env.collect_warnings())
}

#[allow(clippy::result_large_err)]
fn typecheck_stmt(
    stmt: &Stmt,
    env: &mut TypeEnv,
    functions: &HashMap<String, FuncSig>,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
    expected_return: &ExpectedReturn,
) -> Result<(), TypeError> {
    match stmt {
        Stmt::Let { name, ty, expr } => {
            let expr_ty = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            if let Some(ty) = ty {
                let (declared, constraints) = match ty {
                    Type::Safe { base, constraints } => {
                        (type_from_ast(base, enums, traits)?, Some(constraints))
                    }
                    _ => (type_from_ast(ty, enums, traits)?, None),
                };
                if declared != expr_ty {
                    return Err(TypeError::Mismatch {
                        expected: declared,
                        found: expr_ty,
                        span: Some(expr.span),
                    });
                }
                if let Some(constraints) = constraints {
                    validate_safe_constraints(constraints, &declared, expr, env, functions)?;
                }
                let inferred_constraints = constraints
                    .cloned()
                    .or_else(|| expr_constraints(expr, env, functions));
                env.insert_var(name.clone(), declared, inferred_constraints);
            } else {
                let inferred_constraints = expr_constraints(expr, env, functions);
                env.insert_var(name.clone(), expr_ty, inferred_constraints);
            }
            Ok(())
        }
        Stmt::Return(expr) => {
            let found = if let Some(expr) = expr {
                type_of_expr(expr, env, functions, enums, traits, expected_return)?
            } else {
                Ty::Unit
            };
            if found != expected_return.ty {
                return Err(TypeError::ReturnMismatch {
                    expected: expected_return.ty.clone(),
                    found,
                    span: expr.as_ref().map(|e| e.span),
                });
            }
            if let (Some(constraints), Some(expr)) =
                (expected_return.constraints.as_ref(), expr.as_ref())
            {
                validate_safe_constraints(constraints, &expected_return.ty, expr, env, functions)?;
            }
            Ok(())
        }
        Stmt::While { condition, body } => {
            let cond_ty = type_of_expr(condition, env, functions, enums, traits, expected_return)?;
            if cond_ty != Ty::Bool {
                return Err(TypeError::Mismatch {
                    expected: Ty::Bool,
                    found: cond_ty,
                    span: Some(condition.span),
                });
            }
            let mut inner = env.child();
            inner.loop_depth += 1;
            for stmt in body {
                typecheck_stmt(stmt, &mut inner, functions, enums, traits, expected_return)?;
            }
            env.merge_used(&inner);
            Ok(())
        }
        Stmt::For { name, iter, body } => {
            let iter_ty = type_of_expr(iter, env, functions, enums, traits, expected_return)?;
            let elem_ty = match iter_ty {
                Ty::Array { elem, .. } => *elem,
                Ty::Slice { elem } => *elem,
                _ => {
                    return Err(TypeError::Mismatch {
                        expected: Ty::Slice {
                            elem: Box::new(Ty::Unknown),
                        },
                        found: iter_ty,
                        span: Some(iter.span),
                    })
                }
            };
            let mut inner = env.child();
            inner.loop_depth += 1;
            inner.insert_var(name.clone(), elem_ty, None);
            for stmt in body {
                typecheck_stmt(stmt, &mut inner, functions, enums, traits, expected_return)?;
            }
            env.merge_used(&inner);
            Ok(())
        }
        Stmt::Break => {
            if env.loop_depth == 0 {
                Err(TypeError::BreakOutsideLoop { span: None })
            } else {
                Ok(())
            }
        }
        Stmt::Continue => {
            if env.loop_depth == 0 {
                Err(TypeError::ContinueOutsideLoop { span: None })
            } else {
                Ok(())
            }
        }
        Stmt::Expr(expr) => {
            type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            Ok(())
        }
        Stmt::Lambda { .. } => {
            // Not supported as a statement; skip or error as needed
            Ok(())
        }
    }
}

#[allow(clippy::result_large_err)]
fn type_of_expr(
    expr: &Expr,
    env: &mut TypeEnv,
    functions: &HashMap<String, FuncSig>,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
    expected_return: &ExpectedReturn,
) -> Result<Ty, TypeError> {
    let span = Some(expr.span);
    match &expr.kind {
        ExprKind::Int(_) => Ok(Ty::I64),
        ExprKind::Float(_) => Ok(Ty::F64),
        ExprKind::Str(_) => Ok(Ty::String),
        ExprKind::Bool(_) => Ok(Ty::Bool),
        ExprKind::Null => Ok(Ty::Null),
        ExprKind::Ident(name) => {
            if let Some(ty) = env.get_var_and_mark(name) {
                return Ok(ty);
            }
            if let Some(sig) = functions.get(name) {
                return Ok(Ty::Func {
                    params: sig.params.iter().map(|param| param.ty.clone()).collect(),
                    ret: Box::new(sig.ret.ty.clone()),
                });
            }
            // Built-in: print
            if name == "print" {
                // Aceita qualquer tipo como argumento, mas para o typechecker, define como fn(T) -> unit
                // Aqui, para simplificar, aceita um parâmetro de tipo desconhecido
                return Ok(Ty::Func {
                    params: vec![Ty::Unknown],
                    ret: Box::new(Ty::Unit),
                });
            }
            Err(TypeError::UnknownVar {
                name: name.clone(),
                suggestion: suggestion_message(best_suggestion(
                    name,
                    env.vars.keys().cloned().collect::<Vec<_>>(),
                )),
                span,
            })
        }
        ExprKind::Lambda { params, body } => {
            // Collect all variables referenced in the lambda body
            let mut body_vars = std::collections::HashSet::new();
            collect_vars(body, &mut body_vars);

            // Remove parameters from captured vars
            for param in params {
                body_vars.remove(param);
            }

            // Check which variables are actually available in the current scope
            let mut captured_vars = Vec::new();
            for var in &body_vars {
                if let Some(var_ty) = env.get_var(var) {
                    captured_vars.push((var.clone(), var_ty.clone()));
                }
            }
            for (name, _) in &captured_vars {
                env.get_var_and_mark(name);
            }

            // Try to infer parameter types by analyzing usage in the body
            let mut param_types = vec![Ty::Unknown; params.len()];
            let mut inner = env.child();

            // First pass: assume all params are Unknown and collect constraints
            for (idx, (name, ty)) in params.iter().zip(param_types.iter()).enumerate() {
                inner.insert_param(name.clone(), ty.clone(), None, idx);
            }

            // Analyze the body to infer parameter types
            infer_lambda_param_types(
                body,
                &mut inner,
                &mut param_types,
                functions,
                enums,
                traits,
                expected_return,
            )?;

            // Second pass: use inferred types
            let mut inner_final = env.child();
            for (idx, (name, ty)) in params.iter().zip(param_types.iter()).enumerate() {
                inner_final.insert_param(name.clone(), ty.clone(), None, idx);
            }

            let ret_ty = type_of_expr(
                body,
                &mut inner_final,
                functions,
                enums,
                traits,
                expected_return,
            )?;

            // If there are captured variables, this is a closure
            if !captured_vars.is_empty() {
                Ok(Ty::Func {
                    params: param_types,
                    ret: Box::new(ret_ty),
                })
            } else {
                // Regular function type for lambdas without captures
                Ok(Ty::Func {
                    params: param_types,
                    ret: Box::new(ret_ty),
                })
            }
        }
        ExprKind::Assign { name, expr } => {
            let rhs = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            let lhs = env.get_var(name).ok_or_else(|| TypeError::UnknownVar {
                name: name.clone(),
                suggestion: suggestion_message(best_suggestion(
                    name,
                    env.vars.keys().cloned().collect::<Vec<_>>(),
                )),
                span,
            })?;
            if lhs != rhs {
                return Err(TypeError::Mismatch {
                    expected: lhs,
                    found: rhs,
                    span,
                });
            }
            Ok(lhs)
        }
        ExprKind::AssignIndex { .. } => {
            let ExprKind::AssignIndex { expr, index, value } = &expr.kind else {
                return Ok(Ty::Unknown);
            };
            let base_ty = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            let index_ty = type_of_expr(index, env, functions, enums, traits, expected_return)?;
            if index_ty != Ty::I64 && index_ty != Ty::Unknown {
                return Err(TypeError::Mismatch {
                    expected: Ty::I64,
                    found: index_ty,
                    span: Some(index.span),
                });
            }
            match base_ty {
                Ty::Array { elem, .. } | Ty::Slice { elem } => {
                    let value_ty =
                        type_of_expr(value, env, functions, enums, traits, expected_return)?;
                    if value_ty != *elem && value_ty != Ty::Unknown && *elem != Ty::Unknown {
                        return Err(TypeError::Mismatch {
                            expected: *elem.clone(),
                            found: value_ty,
                            span: Some(value.span),
                        });
                    }
                    Ok(*elem)
                }
                other => Err(TypeError::Mismatch {
                    expected: Ty::Array {
                        elem: Box::new(Ty::Unknown),
                        len: 0,
                    },
                    found: other,
                    span,
                }),
            }
        }
        ExprKind::ArrayLiteral(items) => {
            if items.is_empty() {
                return Ok(Ty::Array {
                    elem: Box::new(Ty::Unknown),
                    len: 0,
                });
            }
            let first = type_of_expr(&items[0], env, functions, enums, traits, expected_return)?;
            for item in &items[1..] {
                let ty = type_of_expr(item, env, functions, enums, traits, expected_return)?;
                if ty != first {
                    return Err(TypeError::Mismatch {
                        expected: first.clone(),
                        found: ty,
                        span: Some(item.span),
                    });
                }
            }
            Ok(Ty::Array {
                elem: Box::new(first),
                len: items.len() as i64,
            })
        }
        ExprKind::Call { callee, args } => {
            if let ExprKind::Ident(name) = &callee.kind {
                if env.get_var(name).is_none() && !functions.contains_key(name) && name != "print" {
                    let mut candidates = functions.keys().cloned().collect::<Vec<_>>();
                    candidates.push("print".to_string());
                    return Err(TypeError::UnknownFunction {
                        name: name.clone(),
                        suggestion: suggestion_message(best_suggestion(name, candidates)),
                        span,
                    });
                }
            }
            let callee_ty = type_of_expr(callee, env, functions, enums, traits, expected_return)?;
            match callee_ty {
                Ty::Func { params, ret } => {
                    if params.len() != args.len() {
                        return Err(TypeError::ArityMismatch {
                            expected: params.len(),
                            found: args.len(),
                            span,
                        });
                    }
                    for (arg, expected) in args.iter().zip(params.iter()) {
                        let found =
                            type_of_expr(arg, env, functions, enums, traits, expected_return)?;
                        if &found != expected && *expected != Ty::Unknown {
                            // Special case: allow Func with Unknown params to match Func with known params
                            let types_match = match (&found, expected) {
                                (
                                    Ty::Func {
                                        params: found_params,
                                        ret: found_ret,
                                    },
                                    Ty::Func {
                                        params: expected_params,
                                        ret: expected_ret,
                                    },
                                ) => {
                                    found_ret == expected_ret
                                        && found_params.len() == expected_params.len()
                                        && found_params
                                            .iter()
                                            .zip(expected_params.iter())
                                            .all(|(f, e)| f == e || *f == Ty::Unknown)
                                }
                                _ => false,
                            };
                            if !types_match {
                                return Err(TypeError::Mismatch {
                                    expected: expected.clone(),
                                    found,
                                    span: Some(arg.span),
                                });
                            }
                        }
                    }
                    Ok(*ret)
                }
                other => Err(TypeError::InvalidCallTarget { found: other, span }),
            }
        }
        ExprKind::Field { expr: base, field } => {
            let base_ty = type_of_expr(base, env, functions, enums, traits, expected_return)?;
            match field {
                tupa_parser::FieldAccess::Index(_) => match base_ty {
                    Ty::Array { elem, .. } | Ty::Slice { elem } => Ok(*elem),
                    Ty::Unknown => Ok(Ty::Unknown),
                    other => Err(TypeError::Mismatch {
                        expected: Ty::Array {
                            elem: Box::new(Ty::Unknown),
                            len: 0,
                        },
                        found: other,
                        span,
                    }),
                },
                tupa_parser::FieldAccess::Ident(_) => Ok(Ty::Unknown),
            }
        }
        ExprKind::Index { expr, index } => {
            let base = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            let index_ty = type_of_expr(index, env, functions, enums, traits, expected_return)?;
            if index_ty != Ty::I64 && index_ty != Ty::Unknown {
                return Err(TypeError::Mismatch {
                    expected: Ty::I64,
                    found: index_ty,
                    span: Some(index.span),
                });
            }
            match base {
                Ty::Array { elem, .. } => Ok(*elem),
                Ty::Slice { elem } => Ok(*elem),
                other => Err(TypeError::Mismatch {
                    expected: Ty::Array {
                        elem: Box::new(Ty::Unknown),
                        len: 0,
                    },
                    found: other,
                    span,
                }),
            }
        }
        ExprKind::Await(expr) => type_of_expr(expr, env, functions, enums, traits, expected_return),
        ExprKind::Block(stmts) => {
            type_of_block_expr(stmts, env, functions, enums, traits, expected_return)
        }
        ExprKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond = type_of_expr(condition, env, functions, enums, traits, expected_return)?;
            if cond != Ty::Bool {
                return Err(TypeError::Mismatch {
                    expected: Ty::Bool,
                    found: cond,
                    span: Some(condition.span),
                });
            }
            let mut then_env = env.child();
            let then_ty = type_of_block_expr(
                then_branch,
                &mut then_env,
                functions,
                enums,
                traits,
                expected_return,
            )?;
            env.merge_used(&then_env);
            let else_ty = match else_branch {
                Some(branch) => match branch {
                    tupa_parser::ElseBranch::Block(block) => {
                        let mut else_env = env.child();
                        let else_ty = type_of_block_expr(
                            block,
                            &mut else_env,
                            functions,
                            enums,
                            traits,
                            expected_return,
                        )?;
                        env.merge_used(&else_env);
                        else_ty
                    }
                    tupa_parser::ElseBranch::If(expr) => {
                        type_of_expr(expr, env, functions, enums, traits, expected_return)?
                    }
                },
                None => return Ok(Ty::Unit),
            };
            if then_ty != else_ty {
                return Err(TypeError::Mismatch {
                    expected: then_ty,
                    found: else_ty,
                    span,
                });
            }
            Ok(then_ty)
        }
        ExprKind::Match { expr, arms } => {
            let scrutinee_ty = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            let mut expected_arm_ty: Option<Ty> = None;
            for arm in arms {
                let mut inner = env.child();
                typecheck_pattern(&arm.pattern, arm.pattern_span, &scrutinee_ty, &mut inner)?;
                if let Some(guard) = &arm.guard {
                    let guard_ty =
                        type_of_expr(guard, &mut inner, functions, enums, traits, expected_return)?;
                    if guard_ty != Ty::Bool {
                        return Err(TypeError::Mismatch {
                            expected: Ty::Bool,
                            found: guard_ty,
                            span: Some(guard.span),
                        });
                    }
                }
                let arm_ty = type_of_expr(
                    &arm.expr,
                    &mut inner,
                    functions,
                    enums,
                    traits,
                    expected_return,
                )?;
                env.merge_used(&inner);
                match &expected_arm_ty {
                    Some(expected) if *expected != arm_ty => {
                        return Err(TypeError::Mismatch {
                            expected: expected.clone(),
                            found: arm_ty,
                            span: Some(arm.expr.span),
                        });
                    }
                    None => expected_arm_ty = Some(arm_ty),
                    _ => {}
                }
            }
            Ok(expected_arm_ty.unwrap_or(Ty::Unit))
        }
        ExprKind::Unary { op, expr } => {
            let inner = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            match op {
                UnaryOp::Neg => match inner {
                    Ty::I64 | Ty::F64 => Ok(inner),
                    _ => Err(TypeError::InvalidUnary {
                        op: op.clone(),
                        found: inner,
                        span,
                    }),
                },
                UnaryOp::Not => match inner {
                    Ty::Bool => Ok(Ty::Bool),
                    _ => Err(TypeError::InvalidUnary {
                        op: op.clone(),
                        found: inner,
                        span,
                    }),
                },
            }
        }
        ExprKind::Binary { op, left, right } => {
            let l = type_of_expr(left, env, functions, enums, traits, expected_return)?;
            let r = type_of_expr(right, env, functions, enums, traits, expected_return)?;
            match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
                    if l == r && (l == Ty::I64 || l == Ty::F64) {
                        Ok(l)
                    } else if l == Ty::Unknown && (r == Ty::I64 || r == Ty::F64) {
                        Ok(r)
                    } else if r == Ty::Unknown && (l == Ty::I64 || l == Ty::F64) {
                        Ok(l)
                    } else if *op == BinaryOp::Add && l == Ty::String && r == Ty::String {
                        Ok(Ty::String)
                    } else {
                        Err(TypeError::InvalidBinary {
                            op: op.clone(),
                            left: l,
                            right: r,
                            span,
                        })
                    }
                }
                BinaryOp::Range => {
                    if l == r && l == Ty::I64 {
                        Ok(Ty::Slice {
                            elem: Box::new(Ty::I64),
                        })
                    } else {
                        Err(TypeError::InvalidBinary {
                            op: op.clone(),
                            left: l,
                            right: r,
                            span,
                        })
                    }
                }
                BinaryOp::Equal
                | BinaryOp::NotEqual
                | BinaryOp::Less
                | BinaryOp::LessEqual
                | BinaryOp::Greater
                | BinaryOp::GreaterEqual => {
                    if l == r {
                        Ok(Ty::Bool)
                    } else {
                        Err(TypeError::InvalidBinary {
                            op: op.clone(),
                            left: l,
                            right: r,
                            span,
                        })
                    }
                }
                BinaryOp::And | BinaryOp::Or => {
                    if l == Ty::Bool && r == Ty::Bool {
                        Ok(Ty::Bool)
                    } else {
                        Err(TypeError::InvalidBinary {
                            op: op.clone(),
                            left: l,
                            right: r,
                            span,
                        })
                    }
                }
            }
        }
    }
}

#[allow(clippy::result_large_err)]
fn type_of_block_expr(
    stmts: &[Stmt],
    env: &mut TypeEnv,
    functions: &HashMap<String, FuncSig>,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
    expected_return: &ExpectedReturn,
) -> Result<Ty, TypeError> {
    let mut last_ty = Ty::Unit;
    for stmt in stmts {
        match stmt {
            Stmt::Return(expr) => {
                last_ty = if let Some(expr) = expr {
                    type_of_expr(expr, env, functions, enums, traits, expected_return)?
                } else {
                    Ty::Unit
                };
            }
            Stmt::Expr(expr) => {
                last_ty = type_of_expr(expr, env, functions, enums, traits, expected_return)?;
            }
            _ => {
                typecheck_stmt(stmt, env, functions, enums, traits, expected_return)?;
                last_ty = Ty::Unit;
            }
        }
    }
    Ok(last_ty)
}

fn stmt_span(stmt: &Stmt) -> Option<Span> {
    match stmt {
        Stmt::Let { expr, .. } => Some(expr.span),
        Stmt::Return(expr) => expr.as_ref().map(|expr| expr.span),
        Stmt::While { condition, .. } => Some(condition.span),
        Stmt::For { iter, .. } => Some(iter.span),
        Stmt::Break | Stmt::Continue => None,
        Stmt::Expr(expr) => Some(expr.span),
        Stmt::Lambda { body, .. } => Some(body.span),
    }
}

fn block_returns(stmts: &[Stmt]) -> bool {
    for stmt in stmts {
        if stmt_returns(stmt) {
            return true;
        }
    }
    false
}

fn stmt_returns(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Return(_) => true,
        Stmt::Expr(expr) => expr_returns(expr),
        _ => false,
    }
}

fn expr_returns(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::If {
            then_branch,
            else_branch,
            ..
        } => {
            let then_returns = block_returns(then_branch);
            let else_returns = match else_branch {
                Some(tupa_parser::ElseBranch::Block(block)) => block_returns(block),
                Some(tupa_parser::ElseBranch::If(expr)) => expr_returns(expr),
                None => false,
            };
            then_returns && else_returns
        }
        ExprKind::Match { arms, .. } => arms.iter().all(|arm| expr_returns(&arm.expr)),
        ExprKind::Block(stmts) => block_returns(stmts),
        _ => false,
    }
}

#[allow(clippy::result_large_err)]
fn typecheck_pattern(
    pattern: &Pattern,
    pattern_span: Span,
    scrutinee: &Ty,
    env: &mut TypeEnv,
) -> Result<(), TypeError> {
    match pattern {
        Pattern::Wildcard => Ok(()),
        Pattern::Ident(name) => {
            env.insert_var(name.clone(), scrutinee.clone(), None);
            Ok(())
        }
        Pattern::Int(_) => match scrutinee {
            Ty::I64 | Ty::Unknown => Ok(()),
            other => Err(TypeError::Mismatch {
                expected: Ty::I64,
                found: other.clone(),
                span: Some(pattern_span),
            }),
        },
        Pattern::Str(_) => match scrutinee {
            Ty::String | Ty::Unknown => Ok(()),
            other => Err(TypeError::Mismatch {
                expected: Ty::String,
                found: other.clone(),
                span: Some(pattern_span),
            }),
        },
    }
}

#[allow(clippy::result_large_err, clippy::only_used_in_recursion)]
fn type_from_ast(
    ty: &Type,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
) -> Result<Ty, TypeError> {
    match ty {
        Type::Ident(name) => match name.as_str() {
            "i64" => Ok(Ty::I64),
            "f64" => Ok(Ty::F64),
            "bool" => Ok(Ty::Bool),
            "string" => Ok(Ty::String),
            "null" => Ok(Ty::Null),
            _ => {
                if enums.contains_key(name) {
                    Ok(Ty::Enum(name.clone()))
                } else {
                    let mut candidates = vec![
                        "i64".to_string(),
                        "f64".to_string(),
                        "bool".to_string(),
                        "string".to_string(),
                        "null".to_string(),
                    ];
                    candidates.extend(enums.keys().cloned());
                    let suggestion = suggestion_message(best_suggestion(name, candidates));
                    Err(TypeError::UnknownType {
                        name: name.clone(),
                        suggestion,
                    })
                }
            }
        },
        Type::Safe { base, .. } => type_from_ast(base, enums, traits),
        Type::Array { elem, len } => Ok(Ty::Array {
            elem: Box::new(type_from_ast(elem, enums, traits)?),
            len: *len,
        }),
        Type::Slice { elem } => Ok(Ty::Slice {
            elem: Box::new(type_from_ast(elem, enums, traits)?),
        }),
        Type::Func { params, ret } => {
            let params = params
                .iter()
                .map(|p| type_from_ast(p, enums, traits))
                .collect::<Result<Vec<_>, _>>()?;
            let ret = type_from_ast(ret, enums, traits)?;
            Ok(Ty::Func {
                params,
                ret: Box::new(ret),
            })
        }
    }
}

#[allow(clippy::result_large_err)]
fn type_sig_from_ast(
    ty: &Type,
    enums: &HashMap<String, Vec<String>>,
    traits: &HashMap<String, Vec<Function>>,
) -> Result<TypeSig, TypeError> {
    match ty {
        Type::Safe { base, constraints } => {
            let base_ty = type_from_ast(base, enums, traits)?;
            validate_safe_annotation_constraints(constraints, &base_ty)?;
            Ok(TypeSig {
                ty: base_ty,
                constraints: Some(constraints.clone()),
            })
        }
        _ => Ok(TypeSig {
            ty: type_from_ast(ty, enums, traits)?,
            constraints: None,
        }),
    }
}

#[allow(clippy::result_large_err)]
fn validate_safe_annotation_constraints(
    constraints: &[String],
    base: &Ty,
) -> Result<(), TypeError> {
    for constraint in constraints {
        match constraint.as_str() {
            "nan" | "inf" => {
                if *base != Ty::F64 {
                    return Err(TypeError::InvalidConstraint {
                        constraint: constraint.clone(),
                        base: base.clone(),
                        span: None,
                    });
                }
            }
            "hate_speech" | "misinformation" => {
                if *base != Ty::String {
                    return Err(TypeError::InvalidConstraint {
                        constraint: constraint.clone(),
                        base: base.clone(),
                        span: None,
                    });
                }
            }
            _ => {
                return Err(TypeError::InvalidConstraint {
                    constraint: constraint.clone(),
                    base: base.clone(),
                    span: None,
                });
            }
        }
    }
    Ok(())
}

#[derive(Debug, Default, Clone)]
struct TypeEnv {
    vars: HashMap<String, VarInfo>,
    loop_depth: usize,
    param_indices: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
struct FuncSig {
    params: Vec<TypeSig>,
    ret: TypeSig,
}

impl TypeEnv {
    fn child(&self) -> TypeEnv {
        TypeEnv {
            vars: self.vars.clone(),
            loop_depth: self.loop_depth,
            param_indices: self.param_indices.clone(),
        }
    }

    fn insert_var(&mut self, name: String, ty: Ty, constraints: Option<Vec<String>>) {
        self.vars.insert(
            name,
            VarInfo {
                ty,
                constraints,
                used: false,
            },
        );
    }

    fn insert_param(
        &mut self,
        name: String,
        ty: Ty,
        constraints: Option<Vec<String>>,
        index: usize,
    ) {
        self.vars.insert(
            name.clone(),
            VarInfo {
                ty,
                constraints,
                used: false,
            },
        );
        self.param_indices.insert(name, index);
    }

    fn get_var(&self, name: &str) -> Option<Ty> {
        self.vars.get(name).map(|info| info.ty.clone())
    }

    fn get_var_and_mark(&mut self, name: &str) -> Option<Ty> {
        if let Some(info) = self.vars.get_mut(name) {
            info.used = true;
            return Some(info.ty.clone());
        }
        None
    }

    fn get_var_constraints(&self, name: &str) -> Option<Vec<String>> {
        self.vars
            .get(name)
            .and_then(|info| info.constraints.clone())
    }

    fn collect_warnings(&self) -> Vec<Warning> {
        self.vars
            .iter()
            .filter_map(|(name, info)| {
                if !info.used && !name.starts_with('_') {
                    Some(Warning::UnusedVar(name.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn merge_used(&mut self, other: &TypeEnv) {
        for (name, info) in &other.vars {
            if info.used {
                if let Some(current) = self.vars.get_mut(name) {
                    current.used = true;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct VarInfo {
    ty: Ty,
    constraints: Option<Vec<String>>,
    used: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_parser::parse_program;

    #[test]
    fn typecheck_simple_let() {
        let program = parse_program("fn main() { let x: i64 = 1; } ").unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn typecheck_safe_annotation() {
        let program =
            parse_program("fn main() { let x: Safe<f64, !nan, !inf> = 1.0; let y: f64 = x; } ")
                .unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn safe_constraint_invalid_base() {
        let program = parse_program("fn main() { let x: Safe<i64, !nan> = 1; }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::InvalidConstraint { .. })
        ));
    }

    #[test]
    fn safe_constraint_unproven() {
        let program =
            parse_program("fn main() { let y: f64 = 1.0; let x: Safe<f64, !nan> = y; }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::UnprovenConstraint { .. })
        ));
    }

    #[test]
    fn safe_string_constraint_unproven() {
        let program =
            parse_program("fn main() { let x: Safe<string, !hate_speech> = \"ok\"; }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::UnprovenConstraint { .. })
        ));
    }

    #[test]
    fn safe_string_constraint_propagates_from_param() {
        let program = parse_program(
            "fn accept(x: Safe<string, !hate_speech>) { let y: Safe<string, !hate_speech> = x; }",
        )
        .unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn safe_constraint_propagates_from_safe_var() {
        let program = parse_program(
            "fn main() { let x: Safe<f64, !nan> = 1.0; let y: Safe<f64, !nan> = x; }",
        )
        .unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn safe_constraint_propagates_from_safe_function() {
        let program = parse_program(
            "fn ok(): Safe<f64, !nan> { return 1.0; } fn main() { let x: Safe<f64, !nan> = ok(); }",
        )
        .unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn safe_constraint_unknown_is_invalid() {
        let program = parse_program("fn main() { let x: Safe<f64, !foo> = 1.0; }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::InvalidConstraint { constraint, .. }) if constraint == "foo"
        ));
    }

    #[test]
    fn safe_constraint_invalid_param_base() {
        let program =
            parse_program("fn accept(x: Safe<i64, !nan>) { print(x); } fn main() {}").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::InvalidConstraint { .. })
        ));
    }

    #[test]
    fn safe_constraint_invalid_return_base() {
        let program = parse_program("fn main(): Safe<i64, !nan> { return 1; }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::InvalidConstraint { .. })
        ));
    }

    #[test]
    fn safe_constraint_negative_literal() {
        let program = parse_program("fn main() { let x: Safe<f64, !nan, !inf> = -1.0; }").unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn safe_constraint_const_expr_ok() {
        let program =
            parse_program("fn main() { let x: Safe<f64, !nan, !inf> = 1.0 + 2.0; }").unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn safe_constraint_const_expr_inf_is_error() {
        let program = parse_program("fn main() { let x: Safe<f64, !inf> = 1.0 / 0.0; }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::UnprovenConstraint { .. })
        ));
    }

    #[test]
    fn typecheck_mismatch_is_error() {
        let program = parse_program("fn main() { let x: i64 = true; } ").unwrap();
        assert!(typecheck_program(&program).is_err());
    }

    #[test]
    fn typecheck_return_mismatch() {
        let program = parse_program("fn main(): i64 { return true; } ").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::ReturnMismatch { .. })
        ));
    }

    #[test]
    fn typecheck_while_condition_bool() {
        let program = parse_program("fn main() { let x: i64 = 1; while x { return; } } ").unwrap();
        assert!(typecheck_program(&program).is_err());
    }

    #[test]
    fn typecheck_array_literal_types() {
        let ok = parse_program("fn main() { let xs = [1, 2, 3]; } ").unwrap();
        let err = parse_program("fn main() { let xs = [1, true]; } ").unwrap();
        assert!(typecheck_program(&ok).is_ok());
        assert!(typecheck_program(&err).is_err());
    }

    #[test]
    fn typecheck_assign_index_types() {
        let ok = parse_program("fn main() { let xs = [1, 2, 3]; xs[1] = 4; }").unwrap();
        let bad_value = parse_program("fn main() { let xs = [1, 2, 3]; xs[1] = true; }").unwrap();
        let bad_index = parse_program("fn main() { let xs = [1, 2, 3]; xs[true] = 4; }").unwrap();
        assert!(typecheck_program(&ok).is_ok());
        assert!(typecheck_program(&bad_value).is_err());
        assert!(typecheck_program(&bad_index).is_err());
    }

    #[test]
    fn typecheck_index_types() {
        let ok = parse_program("fn main() { let xs = [1, 2, 3]; let y = xs[1]; }").unwrap();
        let bad_index =
            parse_program("fn main() { let xs = [1, 2, 3]; let y = xs[true]; }").unwrap();
        let bad_base = parse_program("fn main() { let x: i64 = 1; let y = x[0]; }").unwrap();
        assert!(typecheck_program(&ok).is_ok());
        assert!(typecheck_program(&bad_index).is_err());
        assert!(typecheck_program(&bad_base).is_err());
    }

    #[test]
    fn typecheck_field_access() {
        let program = parse_program("fn main() { let xs = [1, 2, 3]; let y = xs.len; }").unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn typecheck_await_expr() {
        let program =
            parse_program("fn foo(): i64 { return 1; } fn main() { let x = await foo(); }")
                .unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn warns_on_unused_vars() {
        let program = parse_program("fn main() { let x: i64 = 1; } ").unwrap();
        let warnings = typecheck_program_with_warnings(&program).unwrap();
        assert!(warnings.contains(&Warning::UnusedVar("x".into())));
    }

    #[test]
    fn no_warn_on_var_used_in_loop() {
        let program =
            parse_program("fn main() { let x: i64 = 1; while true { let y: i64 = x; break; } }")
                .unwrap();
        let warnings = typecheck_program_with_warnings(&program).unwrap();
        assert!(!warnings.contains(&Warning::UnusedVar("x".into())));
    }

    #[test]
    fn no_warn_on_var_used_in_lambda() {
        let program =
            parse_program("fn main() { let x: i64 = 1; let f = |z| x + z; let y: i64 = f(2); }")
                .unwrap();
        let warnings = typecheck_program_with_warnings(&program).unwrap();
        assert!(!warnings.contains(&Warning::UnusedVar("x".into())));
    }

    #[test]
    fn typecheck_match_arm_types() {
        let ok =
            parse_program("fn main() { let x: i64 = 1; let y = match x { 1 => 1, _ => 2 }; } ")
                .unwrap();
        let err =
            parse_program("fn main() { let x: i64 = 1; let y = match x { 1 => 1, _ => true }; } ")
                .unwrap();
        assert!(typecheck_program(&ok).is_ok());
        assert!(typecheck_program(&err).is_err());
    }

    #[test]
    fn typecheck_match_guard_bool() {
        let program = parse_program(
            "fn main() { let x: i64 = 1; let y = match x { v if 1 => v, _ => 0 }; } ",
        )
        .unwrap();
        assert!(typecheck_program(&program).is_err());
    }

    #[test]
    fn typecheck_match_pattern_type_mismatch() {
        let program = parse_program(
            "fn main() { let mood: string = \"ok\"; match mood { 1 => print(\"no\"), _ => print(\"ok\"), }; }",
        )
        .unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::Mismatch { .. })
        ));
    }

    #[test]
    fn typecheck_range_in_for_loop() {
        let program = parse_program("fn main() { for i in 0..10 { let x: i64 = i; } } ").unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn typecheck_function_calls() {
        let ok = parse_program(
            "fn add(a: i64, b: i64): i64 { return a + b; } fn main() { let x = add(1, 2); }",
        )
        .unwrap();
        let bad_arity = parse_program(
            "fn add(a: i64, b: i64): i64 { return a + b; } fn main() { let x = add(1); }",
        )
        .unwrap();
        let bad_type = parse_program(
            "fn add(a: i64, b: i64): i64 { return a + b; } fn main() { let x = add(true, 2); }",
        )
        .unwrap();
        assert!(typecheck_program(&ok).is_ok());
        assert!(typecheck_program(&bad_arity).is_err());
        assert!(typecheck_program(&bad_type).is_err());
    }

    #[test]
    fn typecheck_function_values() {
        let ok = parse_program(
            "fn add(a: i64, b: i64): i64 { return a + b; } fn main() { let f: fn(i64, i64) -> i64 = add; let x = f(1, 2); }",
        )
        .unwrap();
        let bad = parse_program(
            "fn add(a: i64, b: i64): i64 { return a + b; } fn main() { let f: fn(i64) -> i64 = add; }",
        )
        .unwrap();
        assert!(typecheck_program(&ok).is_ok());
        assert!(typecheck_program(&bad).is_err());
    }

    #[test]
    fn typecheck_lambda_param_inference_from_call() {
        let program = parse_program(
            "fn add(a: i64, b: i64): i64 { return a + b; } fn main() { let f: fn(i64) -> i64 = |x| add(x, 1); let y: i64 = f(2); }",
        )
        .unwrap();
        assert!(typecheck_program(&program).is_ok());
    }

    #[test]
    fn missing_return_on_some_paths() {
        let program = parse_program("fn main(x: i64): i64 { if x > 0 { return x; } }").unwrap();
        assert!(matches!(
            typecheck_program(&program),
            Err(TypeError::MissingReturn { .. })
        ));
    }

    #[test]
    fn return_inside_match_all_arms() {
        let ok =
            parse_program("fn main(x: i64): i64 { return match x { 0 => 1, _ => 2 }; }").unwrap();
        assert!(typecheck_program(&ok).is_ok());
    }
}
