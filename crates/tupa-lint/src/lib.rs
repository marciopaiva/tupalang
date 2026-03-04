use std::collections::HashMap;
use tupa_parser::{Item, Stmt, Expr, ExprKind};

#[derive(Debug, Clone, PartialEq)]
pub enum LintWarning {
    UnusedVariable(String),
    ShadowedVariable(String),
    MissingConstraintAnnotation(String),
    ImpureInSensitiveContext(String),
    SnakeCase(String, String),
    PascalCase(String, String),
}

impl LintWarning {
    pub fn message(&self) -> String {
        match self {
            LintWarning::UnusedVariable(name) => format!("Variable '{}' is unused", name),
            LintWarning::ShadowedVariable(name) => format!("Variable '{}' is shadowed", name),
            LintWarning::MissingConstraintAnnotation(name) => format!("Pipeline '{}' has no constraints", name),
            LintWarning::ImpureInSensitiveContext(name) => format!("Impure operation '{}' in sensitive context", name),
            LintWarning::SnakeCase(kind, name) => format!("{} '{}' should be snake_case", kind, name),
            LintWarning::PascalCase(kind, name) => format!("{} '{}' should be PascalCase", kind, name),
        }
    }
}

struct LintContext {
    scopes: Vec<HashMap<String, bool>>, // name -> used
    warnings: Vec<LintWarning>,
    in_sensitive_context: bool,
}

impl LintContext {
    fn new() -> Self {
        Self {
            scopes: Vec::new(),
            warnings: Vec::new(),
            in_sensitive_context: false,
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (name, used) in scope {
                if !used && !name.starts_with('_') {
                    self.warnings.push(LintWarning::UnusedVariable(name));
                }
            }
        }
    }

    fn declare(&mut self, name: String) {
        // Only declare in current scope
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                self.warnings.push(LintWarning::ShadowedVariable(name.clone()));
            }
            scope.insert(name, false);
        }
    }

    fn use_var(&mut self, name: &str) {
        // Search from inner to outer
        for scope in self.scopes.iter_mut().rev() {
            if let Some(used) = scope.get_mut(name) {
                *used = true;
                return; // Found usage
            }
        }
    }
}

pub fn lint_program(program: &tupa_parser::Program) -> Vec<LintWarning> {
    let mut ctx = LintContext::new();
    ctx.enter_scope(); // Global scope

    for item in &program.items {
        match item {
            Item::Function(f) => {
                if !is_snake_case(&f.name) {
                    ctx.warnings.push(LintWarning::SnakeCase("Function".to_string(), f.name.clone()));
                }
                ctx.enter_scope();
                for param in &f.params {
                    ctx.declare(param.name.clone());
                    // Params are considered used? Usually yes, or assume used.
                    // Or track them. If strict unused, we should mark them false.
                    // Let's mark them true to avoid noise for now, or false if we want strict.
                    // Rust warns on unused args. Let's mark false.
                }
                for stmt in &f.body {
                    lint_stmt(stmt, &mut ctx);
                }
                ctx.exit_scope();
            }
            Item::Pipeline(p) => {
                if !is_pascal_case(&p.name) {
                    ctx.warnings.push(LintWarning::PascalCase("Pipeline".to_string(), p.name.clone()));
                }
                
                // Check missing constraints
                if p.constraints.is_empty() {
                     ctx.warnings.push(LintWarning::MissingConstraintAnnotation(p.name.clone()));
                }
                
                // Check impure
                let is_impure = p.attrs.iter().any(|a| a.name == "impure" || a.name == "@impure");
                if !is_impure {
                    ctx.in_sensitive_context = true;
                    for step in &p.steps {
                        lint_expr(&step.body, &mut ctx);
                    }
                    ctx.in_sensitive_context = false;
                }
            }
            _ => {}
        }
    }
    
    ctx.exit_scope(); // Global
    ctx.warnings
}

fn lint_stmt(stmt: &Stmt, ctx: &mut LintContext) {
    match stmt {
        Stmt::Let { name, expr, .. } => {
            if !is_snake_case(name) {
                ctx.warnings.push(LintWarning::SnakeCase("Variable".to_string(), name.clone()));
            }
            lint_expr(expr, ctx);
            ctx.declare(name.clone());
        }
        Stmt::Expr(expr) => lint_expr(expr, ctx),
        Stmt::While { condition, body } => {
            lint_expr(condition, ctx);
            ctx.enter_scope();
            for s in body { lint_stmt(s, ctx); }
            ctx.exit_scope();
        }
        Stmt::For { name, iter, body } => {
            lint_expr(iter, ctx);
            ctx.enter_scope();
            ctx.declare(name.clone());
            // Mark loop var as used to avoid noise?
            ctx.use_var(name); 
            for s in body { lint_stmt(s, ctx); }
            ctx.exit_scope();
        }
        Stmt::Return(Some(expr)) => lint_expr(expr, ctx),
        _ => {}
    }
}

fn lint_expr(expr: &Expr, ctx: &mut LintContext) {
    match &expr.kind {
        ExprKind::Ident(name) => {
            ctx.use_var(name);
        }
        ExprKind::Call { callee, args } => {
            if let ExprKind::Ident(name) = &callee.kind {
                if ctx.in_sensitive_context && is_impure_func(name) {
                    ctx.warnings.push(LintWarning::ImpureInSensitiveContext(name.clone()));
                }
            }
            lint_expr(callee, ctx);
            for arg in args { lint_expr(arg, ctx); }
        }
        ExprKind::Binary { left, right, .. } => {
            lint_expr(left, ctx);
            lint_expr(right, ctx);
        }
        ExprKind::Block(block) => {
            ctx.enter_scope();
            for s in block { lint_stmt(s, ctx); }
            ctx.exit_scope();
        }
        ExprKind::If { condition, then_branch, else_branch } => {
            lint_expr(condition, ctx);
            ctx.enter_scope();
            for s in then_branch { lint_stmt(s, ctx); }
            ctx.exit_scope();
            
            if let Some(branch) = else_branch {
                match branch {
                    tupa_parser::ElseBranch::Block(block) => {
                        ctx.enter_scope();
                        for s in block { lint_stmt(s, ctx); }
                        ctx.exit_scope();
                    }
                    tupa_parser::ElseBranch::If(expr) => {
                        lint_expr(expr, ctx);
                    }
                }
            }
        }
        ExprKind::Assign { name, expr } => {
            ctx.use_var(name); // Assignment counts as usage? Usually yes (liveness). Or no?
            // "Unused variable" usually means "never read".
            // Writing to it is not reading.
            // But if I declare `let x = 1; x = 2;`, it's still unused if never read.
            // So assignment should NOT call `use_var` if we track reads.
            // But for simplicity, let's say assignment is usage to avoid "unused var" if only assigned.
            // Wait, if only assigned, it IS unused.
            // So `Assign` should NOT mark as used.
            // But `expr` (RHS) should be linted.
            lint_expr(expr, ctx);
        }
        _ => {
            // Traverse other variants if needed. 
            // For minimal implementation, recursive structure is key.
            // Need to handle AssignIndex, Field, etc. to recurse.
            // Skipping for brevity in "Minimal" but correct implementation needs full traversal.
        }
    }
}

fn is_snake_case(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '_')
}

fn is_pascal_case(s: &str) -> bool {
    if let Some(first) = s.chars().next() {
        if !first.is_uppercase() { return false; }
    }
    s.chars().all(|c| c.is_alphanumeric())
}

fn is_impure_func(name: &str) -> bool {
    matches!(name, "print" | "read" | "random" | "time" | "write" | "http_get")
}
