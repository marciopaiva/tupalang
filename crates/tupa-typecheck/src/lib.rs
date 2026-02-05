use std::collections::HashMap;

use thiserror::Error;
use tupa_parser::{BinaryOp, Expr, Function, Item, Program, Stmt, Type, UnaryOp};

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
    Unknown,
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("unknown type '{0}'")]
    UnknownType(String),
    #[error("undefined variable '{0}'")]
    UnknownVar(String),
    #[error("type mismatch: expected {expected:?}, got {found:?}")]
    Mismatch { expected: Ty, found: Ty },
    #[error("invalid operand types for {op:?}: {left:?}, {right:?}")]
    InvalidBinary { op: BinaryOp, left: Ty, right: Ty },
    #[error("invalid operand type for {op:?}: {found:?}")]
    InvalidUnary { op: UnaryOp, found: Ty },
    #[error("return type mismatch: expected {expected:?}, got {found:?}")]
    ReturnMismatch { expected: Ty, found: Ty },
    #[error("expected function body to return a value")]
    MissingReturn,
}

pub fn typecheck_program(program: &Program) -> Result<(), TypeError> {
    for item in &program.items {
        match item {
            Item::Function(func) => typecheck_function(func)?,
        }
    }
    Ok(())
}

fn typecheck_function(func: &Function) -> Result<(), TypeError> {
    let mut env = TypeEnv::default();
    for param in &func.params {
        let ty = type_from_ast(&param.ty)?;
        env.vars.insert(param.name.clone(), ty);
    }

    let expected_return = func
        .return_type
        .as_ref()
        .map(type_from_ast)
        .transpose()?
        .unwrap_or(Ty::Unit);

    let mut saw_return = false;
    for stmt in &func.body {
        match stmt {
            Stmt::Return(expr) => {
                let found = if let Some(expr) = expr {
                    type_of_expr(expr, &mut env)?
                } else {
                    Ty::Unit
                };
                if expected_return != Ty::Unit && found == Ty::Unit {
                    return Err(TypeError::ReturnMismatch {
                        expected: expected_return.clone(),
                        found,
                    });
                }
                if expected_return != Ty::Unit && found != expected_return {
                    return Err(TypeError::ReturnMismatch {
                        expected: expected_return.clone(),
                        found,
                    });
                }
                saw_return = true;
            }
            _ => {
                typecheck_stmt(stmt, &mut env)?;
            }
        }
    }

    if expected_return != Ty::Unit && !saw_return {
        return Err(TypeError::MissingReturn);
    }

    Ok(())
}

fn typecheck_stmt(stmt: &Stmt, env: &mut TypeEnv) -> Result<(), TypeError> {
    match stmt {
        Stmt::Let { name, ty, expr } => {
            let expr_ty = type_of_expr(expr, env)?;
            if let Some(ty) = ty {
                let declared = type_from_ast(ty)?;
                if declared != expr_ty {
                    return Err(TypeError::Mismatch {
                        expected: declared,
                        found: expr_ty,
                    });
                }
                env.vars.insert(name.clone(), declared);
            } else {
                env.vars.insert(name.clone(), expr_ty);
            }
            Ok(())
        }
        Stmt::Return(_) => Ok(()),
        Stmt::While { condition, body } => {
            let cond_ty = type_of_expr(condition, env)?;
            if cond_ty != Ty::Bool {
                return Err(TypeError::Mismatch {
                    expected: Ty::Bool,
                    found: cond_ty,
                });
            }
            let mut inner = env.child();
            for stmt in body {
                typecheck_stmt(stmt, &mut inner)?;
            }
            Ok(())
        }
        Stmt::For { name, iter, body } => {
            let iter_ty = type_of_expr(iter, env)?;
            let elem_ty = match iter_ty {
                Ty::Array { elem, .. } => *elem,
                Ty::Slice { elem } => *elem,
                _ => {
                    return Err(TypeError::Mismatch {
                        expected: Ty::Slice {
                            elem: Box::new(Ty::Unknown),
                        },
                        found: iter_ty,
                    })
                }
            };
            let mut inner = env.child();
            inner.vars.insert(name.clone(), elem_ty);
            for stmt in body {
                typecheck_stmt(stmt, &mut inner)?;
            }
            Ok(())
        }
        Stmt::Expr(expr) => {
            type_of_expr(expr, env)?;
            Ok(())
        }
    }
}

fn type_of_expr(expr: &Expr, env: &mut TypeEnv) -> Result<Ty, TypeError> {
    match expr {
        Expr::Int(_) => Ok(Ty::I64),
        Expr::Float(_) => Ok(Ty::F64),
        Expr::Str(_) => Ok(Ty::String),
        Expr::Bool(_) => Ok(Ty::Bool),
        Expr::Null => Ok(Ty::Null),
        Expr::Ident(name) => env
            .vars
            .get(name)
            .cloned()
            .ok_or_else(|| TypeError::UnknownVar(name.clone())),
        Expr::Assign { name, expr } => {
            let rhs = type_of_expr(expr, env)?;
            let lhs = env
                .vars
                .get(name)
                .cloned()
                .ok_or_else(|| TypeError::UnknownVar(name.clone()))?;
            if lhs != rhs {
                return Err(TypeError::Mismatch {
                    expected: lhs,
                    found: rhs,
                });
            }
            Ok(lhs)
        }
        Expr::ArrayLiteral(items) => {
            if items.is_empty() {
                return Ok(Ty::Array {
                    elem: Box::new(Ty::Unknown),
                    len: 0,
                });
            }
            let first = type_of_expr(&items[0], env)?;
            for item in &items[1..] {
                let ty = type_of_expr(item, env)?;
                if ty != first {
                    return Err(TypeError::Mismatch {
                        expected: first.clone(),
                        found: ty,
                    });
                }
            }
            Ok(Ty::Array {
                elem: Box::new(first),
                len: items.len() as i64,
            })
        }
        Expr::Call { .. } => Ok(Ty::Unknown),
        Expr::Field { .. } => Ok(Ty::Unknown),
        Expr::Index { expr, .. } => {
            let base = type_of_expr(expr, env)?;
            match base {
                Ty::Array { elem, .. } => Ok(*elem),
                Ty::Slice { elem } => Ok(*elem),
                other => Ok(other),
            }
        }
        Expr::Await(expr) => type_of_expr(expr, env),
        Expr::Block(stmts) => type_of_block_expr(stmts, env),
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond = type_of_expr(condition, env)?;
            if cond != Ty::Bool {
                return Err(TypeError::Mismatch {
                    expected: Ty::Bool,
                    found: cond,
                });
            }
            let then_ty = type_of_block_expr(then_branch, &mut env.child())?;
            let else_ty = match else_branch {
                Some(branch) => match branch {
                    tupa_parser::ElseBranch::Block(block) => {
                        type_of_block_expr(block, &mut env.child())?
                    }
                    tupa_parser::ElseBranch::If(expr) => type_of_expr(expr, env)?,
                },
                None => Ty::Unit,
            };
            if then_ty != else_ty {
                return Err(TypeError::Mismatch {
                    expected: then_ty,
                    found: else_ty,
                });
            }
            Ok(then_ty)
        }
        Expr::Match { .. } => Ok(Ty::Unknown),
        Expr::Unary { op, expr } => {
            let inner = type_of_expr(expr, env)?;
            match op {
                UnaryOp::Neg => match inner {
                    Ty::I64 | Ty::F64 => Ok(inner),
                    _ => Err(TypeError::InvalidUnary {
                        op: op.clone(),
                        found: inner,
                    }),
                },
                UnaryOp::Not => match inner {
                    Ty::Bool => Ok(Ty::Bool),
                    _ => Err(TypeError::InvalidUnary {
                        op: op.clone(),
                        found: inner,
                    }),
                },
            }
        }
        Expr::Binary { op, left, right } => {
            let l = type_of_expr(left, env)?;
            let r = type_of_expr(right, env)?;
            match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
                    if l == r && (l == Ty::I64 || l == Ty::F64) {
                        Ok(l)
                    } else {
                        Err(TypeError::InvalidBinary {
                            op: op.clone(),
                            left: l,
                            right: r,
                        })
                    }
                }
                BinaryOp::Range => {
                    if l == r && l == Ty::I64 {
                        Ok(Ty::Unknown)
                    } else {
                        Err(TypeError::InvalidBinary {
                            op: op.clone(),
                            left: l,
                            right: r,
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
                        })
                    }
                }
            }
        }
    }
}

fn type_of_block_expr(stmts: &[Stmt], env: &mut TypeEnv) -> Result<Ty, TypeError> {
    let mut last_ty = Ty::Unit;
    for stmt in stmts {
        match stmt {
            Stmt::Return(expr) => {
                last_ty = if let Some(expr) = expr {
                    type_of_expr(expr, env)?
                } else {
                    Ty::Unit
                };
            }
            _ => {
                typecheck_stmt(stmt, env)?;
            }
        }
    }
    Ok(last_ty)
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
        Type::Array { elem, len } => Ok(Ty::Array {
            elem: Box::new(type_from_ast(elem)?),
            len: *len,
        }),
        Type::Slice { elem } => Ok(Ty::Slice {
            elem: Box::new(type_from_ast(elem)?),
        }),
    }
}

#[derive(Debug, Default, Clone)]
struct TypeEnv {
    vars: HashMap<String, Ty>,
}

impl TypeEnv {
    fn child(&self) -> TypeEnv {
        TypeEnv {
            vars: self.vars.clone(),
        }
    }
}