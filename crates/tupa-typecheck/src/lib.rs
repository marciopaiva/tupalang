use std::collections::HashMap;

use thiserror::Error;
use tupa_lexer::Span;
use tupa_parser::{BinaryOp, Expr, ExprKind, Function, Item, Pattern, Program, Stmt, Type, UnaryOp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    I64,
    F64,
    Bool,
    String,
    Null,
    Unit,
    Array { elem: Box<Ty>, len: i64 },
    Slice { elem: Box<Ty> },
    Func { params: Vec<Ty>, ret: Box<Ty> },
    Unknown,
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("unknown type '{0}'")]
    UnknownType(String),
    #[error("undefined variable '{name}'")]
    UnknownVar { name: String, span: Option<Span> },
    #[error("undefined function '{name}'")]
    UnknownFunction { name: String, span: Option<Span> },
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
    MissingReturn,
    #[error("invalid constraint '{constraint}' for base type {base:?}")]
    InvalidConstraint {
        constraint: String,
        base: Ty,
        span: Option<Span>,
    },
    #[error("cannot prove constraint '{constraint}' at compile time")]
    UnprovenConstraint { constraint: String, span: Option<Span> },
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

pub fn typecheck_program(program: &Program) -> Result<(), TypeError> {
    let _ = typecheck_program_with_warnings(program)?;
    Ok(())
}

fn validate_safe_constraints(
    constraints: &[String],
    base: &Ty,
    expr: &Expr,
) -> Result<(), TypeError> {
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
                match expr.kind {
                    ExprKind::Float(value) => {
                        if !value.is_finite() {
                            return Err(TypeError::UnprovenConstraint {
                                constraint: constraint.clone(),
                                span: Some(expr.span),
                            });
                        }
                    }
                    _ => {
                        return Err(TypeError::UnprovenConstraint {
                            constraint: constraint.clone(),
                            span: Some(expr.span),
                        })
                    }
                }
            }
            _ => {
                return Err(TypeError::UnprovenConstraint {
                    constraint: constraint.clone(),
                    span: Some(expr.span),
                })
            }
        }
    }
    Ok(())
}

pub fn typecheck_program_with_warnings(
    program: &Program,
) -> Result<Vec<Warning>, TypeError> {
    let mut functions = HashMap::new();
    for item in &program.items {
        let Item::Function(func) = item;
        let params = func
            .params
            .iter()
            .map(|p| type_from_ast(&p.ty))
            .collect::<Result<Vec<_>, _>>()?;
        let ret = func
            .return_type
            .as_ref()
            .map(type_from_ast)
            .transpose()?
            .unwrap_or(Ty::Unit);
        functions.insert(func.name.clone(), FuncSig { params, ret });
    }
    let mut warnings = Vec::new();
    for item in &program.items {
        match item {
            Item::Function(func) => warnings.extend(typecheck_function(func, &functions)?),
        }
    }
    Ok(warnings)
}

fn typecheck_function(
    func: &Function,
    functions: &HashMap<String, FuncSig>,
) -> Result<Vec<Warning>, TypeError> {
    let mut env = TypeEnv::default();
    for param in &func.params {
        let ty = type_from_ast(&param.ty)?;
        env.insert_var(param.name.clone(), ty);
    }

    let expected_return = match func.return_type.as_ref() {
        Some(Type::Safe { base, constraints }) => ExpectedReturn {
            ty: type_from_ast(base)?,
            constraints: Some(constraints.clone()),
        },
        Some(ty) => ExpectedReturn {
            ty: type_from_ast(ty)?,
            constraints: None,
        },
        None => ExpectedReturn {
            ty: Ty::Unit,
            constraints: None,
        },
    };

    for stmt in &func.body {
        typecheck_stmt(stmt, &mut env, functions, &expected_return)?;
    }

    if expected_return.ty != Ty::Unit && !block_returns(&func.body) {
        if let Some(Stmt::Expr(expr)) = func.body.last() {
            let found = type_of_expr(expr, &mut env, functions, &expected_return)?;
            if found != expected_return.ty {
                if found == Ty::Unit {
                    return Err(TypeError::MissingReturn);
                }
                return Err(TypeError::ReturnMismatch {
                    expected: expected_return.ty.clone(),
                    found,
                    span: Some(expr.span),
                });
            }
            if let Some(constraints) = expected_return.constraints.as_ref() {
                validate_safe_constraints(constraints, &expected_return.ty, expr)?;
            }
        } else {
            return Err(TypeError::MissingReturn);
        }
    }

    Ok(env.collect_warnings())
}

fn typecheck_stmt(
    stmt: &Stmt,
    env: &mut TypeEnv,
    functions: &HashMap<String, FuncSig>,
    expected_return: &ExpectedReturn,
) -> Result<(), TypeError> {
    match stmt {
        Stmt::Let { name, ty, expr } => {
            let expr_ty = type_of_expr(expr, env, functions, expected_return)?;
            if let Some(ty) = ty {
                let (declared, constraints) = match ty {
                    Type::Safe { base, constraints } => (type_from_ast(base)?, Some(constraints)),
                    _ => (type_from_ast(ty)?, None),
                };
                if declared != expr_ty {
                    return Err(TypeError::Mismatch {
                        expected: declared,
                        found: expr_ty,
                        span: Some(expr.span),
                    });
                }
                if let Some(constraints) = constraints {
                    validate_safe_constraints(constraints, &declared, expr)?;
                }
                env.insert_var(name.clone(), declared);
            } else {
                env.insert_var(name.clone(), expr_ty);
            }
            Ok(())
        }
        Stmt::Return(expr) => {
            let found = if let Some(expr) = expr {
                type_of_expr(expr, env, functions, expected_return)?
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
                validate_safe_constraints(constraints, &expected_return.ty, expr)?;
            }
            Ok(())
        }
        Stmt::While { condition, body } => {
            let cond_ty = type_of_expr(condition, env, functions, expected_return)?;
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
                typecheck_stmt(stmt, &mut inner, functions, expected_return)?;
            }
            Ok(())
        }
        Stmt::For { name, iter, body } => {
            let iter_ty = type_of_expr(iter, env, functions, expected_return)?;
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
            inner.insert_var(name.clone(), elem_ty);
            for stmt in body {
                typecheck_stmt(stmt, &mut inner, functions, expected_return)?;
            }
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
            type_of_expr(expr, env, functions, expected_return)?;
            Ok(())
        }
    }
}

fn type_of_expr(
    expr: &Expr,
    env: &mut TypeEnv,
    functions: &HashMap<String, FuncSig>,
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
                    params: sig.params.clone(),
                    ret: Box::new(sig.ret.clone()),
                });
            }
            Err(TypeError::UnknownVar {
                name: name.clone(),
                span,
            })
        }
        ExprKind::Assign { name, expr } => {
            let rhs = type_of_expr(expr, env, functions, expected_return)?;
            let lhs = env
                .get_var(name)
                .ok_or_else(|| TypeError::UnknownVar {
                    name: name.clone(),
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
        ExprKind::ArrayLiteral(items) => {
            if items.is_empty() {
                return Ok(Ty::Array {
                    elem: Box::new(Ty::Unknown),
                    len: 0,
                });
            }
            let first = type_of_expr(&items[0], env, functions, expected_return)?;
            for item in &items[1..] {
                let ty = type_of_expr(item, env, functions, expected_return)?;
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
            let callee_ty = match &callee.kind {
                ExprKind::Ident(name) => {
                    if name == "print" {
                        if args.len() != 1 {
                            return Err(TypeError::ArityMismatch {
                                expected: 1,
                                found: args.len(),
                                span,
                            });
                        }
                        let _ = type_of_expr(&args[0], env, functions, expected_return)?;
                        return Ok(Ty::Unit);
                    }
                    if let Some(ty) = env.get_var_and_mark(name) {
                        ty
                    } else if let Some(sig) = functions.get(name) {
                        Ty::Func {
                            params: sig.params.clone(),
                            ret: Box::new(sig.ret.clone()),
                        }
                    } else {
                        return Err(TypeError::UnknownFunction {
                            name: name.clone(),
                            span,
                        });
                    }
                }
                _ => type_of_expr(callee, env, functions, expected_return)?,
            };
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
                        let found = type_of_expr(arg, env, functions, expected_return)?;
                        if &found != expected {
                            return Err(TypeError::Mismatch {
                                expected: expected.clone(),
                                found,
                                span: Some(arg.span),
                            });
                        }
                    }
                    Ok(*ret)
                }
                other => Err(TypeError::InvalidCallTarget { found: other, span }),
            }
        }
        ExprKind::Field { .. } => Ok(Ty::Unknown),
        ExprKind::Index { expr, .. } => {
            let base = type_of_expr(expr, env, functions, expected_return)?;
            match base {
                Ty::Array { elem, .. } => Ok(*elem),
                Ty::Slice { elem } => Ok(*elem),
                other => Ok(other),
            }
        }
        ExprKind::Await(expr) => type_of_expr(expr, env, functions, expected_return),
        ExprKind::Block(stmts) => type_of_block_expr(stmts, env, functions, expected_return),
        ExprKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond = type_of_expr(condition, env, functions, expected_return)?;
            if cond != Ty::Bool {
                return Err(TypeError::Mismatch {
                    expected: Ty::Bool,
                    found: cond,
                    span: Some(condition.span),
                });
            }
            let then_ty =
                type_of_block_expr(then_branch, &mut env.child(), functions, expected_return)?;
            let else_ty = match else_branch {
                Some(branch) => match branch {
                    tupa_parser::ElseBranch::Block(block) => {
                        type_of_block_expr(block, &mut env.child(), functions, expected_return)?
                    }
                    tupa_parser::ElseBranch::If(expr) => {
                        type_of_expr(expr, env, functions, expected_return)?
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
            let scrutinee_ty = type_of_expr(expr, env, functions, expected_return)?;
            let mut expected_arm_ty: Option<Ty> = None;
            for arm in arms {
                let mut inner = env.child();
                typecheck_pattern(&arm.pattern, &scrutinee_ty, &mut inner)?;
                if let Some(guard) = &arm.guard {
                    let guard_ty = type_of_expr(guard, &mut inner, functions, expected_return)?;
                    if guard_ty != Ty::Bool {
                        return Err(TypeError::Mismatch {
                            expected: Ty::Bool,
                            found: guard_ty,
                            span: Some(guard.span),
                        });
                    }
                }
                let arm_ty = type_of_expr(&arm.expr, &mut inner, functions, expected_return)?;
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
            let inner = type_of_expr(expr, env, functions, expected_return)?;
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
            let l = type_of_expr(left, env, functions, expected_return)?;
            let r = type_of_expr(right, env, functions, expected_return)?;
            match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
                    if l == r && (l == Ty::I64 || l == Ty::F64) {
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

fn type_of_block_expr(
    stmts: &[Stmt],
    env: &mut TypeEnv,
    functions: &HashMap<String, FuncSig>,
    expected_return: &ExpectedReturn,
) -> Result<Ty, TypeError> {
    let mut last_ty = Ty::Unit;
    for stmt in stmts {
        match stmt {
            Stmt::Return(expr) => {
                last_ty = if let Some(expr) = expr {
                    type_of_expr(expr, env, functions, expected_return)?
                } else {
                    Ty::Unit
                };
            }
            _ => {
                typecheck_stmt(stmt, env, functions, expected_return)?;
            }
        }
    }
    Ok(last_ty)
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

fn typecheck_pattern(
    pattern: &Pattern,
    scrutinee: &Ty,
    env: &mut TypeEnv,
) -> Result<(), TypeError> {
    match pattern {
        Pattern::Wildcard => Ok(()),
        Pattern::Ident(name) => {
            env.insert_var(name.clone(), scrutinee.clone());
            Ok(())
        }
        Pattern::Int(_) => match scrutinee {
            Ty::I64 | Ty::Unknown => Ok(()),
            other => Err(TypeError::Mismatch {
                expected: Ty::I64,
                found: other.clone(),
                span: None,
            }),
        },
        Pattern::Str(_) => match scrutinee {
            Ty::String | Ty::Unknown => Ok(()),
            other => Err(TypeError::Mismatch {
                expected: Ty::String,
                found: other.clone(),
                span: None,
            }),
        },
    }
}

fn type_from_ast(ty: &Type) -> Result<Ty, TypeError> {
    match ty {
        Type::Ident(name) => match name.as_str() {
            "i64" => Ok(Ty::I64),
            "f64" => Ok(Ty::F64),
            "bool" => Ok(Ty::Bool),
            "string" => Ok(Ty::String),
            "null" => Ok(Ty::Null),
            _ => Err(TypeError::UnknownType(name.clone())),
        },
        Type::Safe { base, .. } => type_from_ast(base),
        Type::Array { elem, len } => Ok(Ty::Array {
            elem: Box::new(type_from_ast(elem)?),
            len: *len,
        }),
        Type::Slice { elem } => Ok(Ty::Slice {
            elem: Box::new(type_from_ast(elem)?),
        }),
        Type::Func { params, ret } => {
            let params = params
                .iter()
                .map(type_from_ast)
                .collect::<Result<Vec<_>, _>>()?;
            let ret = type_from_ast(ret)?;
            Ok(Ty::Func {
                params,
                ret: Box::new(ret),
            })
        }
    }
}

#[derive(Debug, Default, Clone)]
struct TypeEnv {
    vars: HashMap<String, VarInfo>,
    loop_depth: usize,
}

#[derive(Debug, Clone)]
struct FuncSig {
    params: Vec<Ty>,
    ret: Ty,
}

impl TypeEnv {
    fn child(&self) -> TypeEnv {
        TypeEnv {
            vars: self.vars.clone(),
            loop_depth: self.loop_depth,
        }
    }

    fn insert_var(&mut self, name: String, ty: Ty) {
        self.vars.insert(
            name,
            VarInfo {
                ty,
                used: false,
            },
        );
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
}

#[derive(Debug, Clone)]
struct VarInfo {
    ty: Ty,
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
        let program = parse_program(
            "fn main() { let x: Safe<f64, !nan, !inf> = 1.0; let y: f64 = x; } ",
        )
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
        let program = parse_program(
            "fn main() { let y: f64 = 1.0; let x: Safe<f64, !nan> = y; }",
        )
        .unwrap();
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
    fn warns_on_unused_vars() {
        let program = parse_program("fn main() { let x: i64 = 1; } ").unwrap();
        let warnings = typecheck_program_with_warnings(&program).unwrap();
        assert!(warnings.contains(&Warning::UnusedVar("x".into())));
    }

    #[test]
    fn typecheck_match_arm_types() {
        let ok = parse_program(
            "fn main() { let x: i64 = 1; let y = match x { 1 => 1, _ => 2 }; } ",
        )
        .unwrap();
        let err = parse_program(
            "fn main() { let x: i64 = 1; let y = match x { 1 => 1, _ => true }; } ",
        )
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
    fn typecheck_range_in_for_loop() {
        let program = parse_program(
            "fn main() { for i in 0..10 { let x: i64 = i; } } ",
        )
        .unwrap();
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
    fn missing_return_on_some_paths() {
        let program = parse_program(
            "fn main(x: i64): i64 { if x > 0 { return x; } }",
        )
        .unwrap();
        assert!(matches!(typecheck_program(&program), Err(TypeError::MissingReturn)));
    }

    #[test]
    fn return_inside_match_all_arms() {
        let ok = parse_program(
            "fn main(x: i64): i64 { return match x { 0 => 1, _ => 2 }; }",
        )
        .unwrap();
        assert!(typecheck_program(&ok).is_ok());
    }
}