use std::collections::HashMap;

use tupa_parser::{Expr, ExprKind, Function, Item, Program, Stmt, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SimpleTy {
    I64,
    Bool,
    Str,
    Void,
    Unknown,
}

pub fn generate_stub(program: &Program) -> String {
    let mut codegen = Codegen::default();
    codegen.emit_program(program);
    codegen.finish()
}

#[derive(Default)]
struct Codegen {
    lines: Vec<String>,
    globals: Vec<String>,
    string_pool: HashMap<String, (String, usize)>,
    temp: usize,
    label: usize,
    loop_stack: Vec<LoopLabels>,
    fmt_int_emitted: bool,
    printf_declared: bool,
    puts_declared: bool,
}

#[derive(Debug, Clone)]
struct LocalVar {
    ptr: String,
    ty: SimpleTy,
}

#[derive(Debug, Clone)]
struct LoopLabels {
    break_label: String,
    continue_label: String,
}

impl Codegen {
    fn emit_program(&mut self, program: &Program) {
        for item in &program.items {
            match item {
                Item::Function(func) => self.emit_function(func),
            }
        }
    }

    fn emit_function(&mut self, func: &Function) {
        let mut env: HashMap<String, LocalVar> = HashMap::new();
        let ret_ty = match func.return_type.as_ref() {
            Some(Type::Ident(name)) if name == "i64" => SimpleTy::I64,
            Some(Type::Ident(name)) if name == "bool" => SimpleTy::Bool,
            Some(Type::Ident(name)) if name == "string" => SimpleTy::Str,
            Some(_) => SimpleTy::Unknown,
            None => SimpleTy::Void,
        };
        let llvm_ret = match ret_ty {
            SimpleTy::I64 => "i64",
            SimpleTy::Bool => "i1",
            SimpleTy::Str => "i8*",
            _ => "void",
        };

        let params = func
            .params
            .iter()
            .map(|p| {
                let ty = self.map_type(&p.ty);
                format!("{} %{}", ty, p.name)
            })
            .collect::<Vec<_>>()
            .join(", ");

        self.lines
            .push(format!("define {llvm_ret} @{}({params}) {{", func.name));
        self.lines.push("entry:".to_string());

        for param in &func.params {
            let ty = match self.map_type(&param.ty).as_str() {
                "i64" => SimpleTy::I64,
                "i1" => SimpleTy::Bool,
                "i8*" => SimpleTy::Str,
                _ => SimpleTy::Unknown,
            };
            let ptr = format!("%{}", param.name);
            let alloca = self.fresh_temp();
            self.lines
                .push(format!("  {alloca} = alloca {}", self.map_type(&param.ty)));
            self.lines
                .push(format!("  store {} {ptr}, {}* {alloca}", self.map_type(&param.ty), self.map_type(&param.ty)));
            env.insert(param.name.clone(), LocalVar { ptr: alloca, ty });
        }

        let mut returned = false;
        let last_index = func.body.len().saturating_sub(1);
        for (idx, stmt) in func.body.iter().enumerate() {
            if returned {
                break;
            }
            if idx == last_index {
                if let Stmt::Expr(expr) = stmt {
                    if ret_ty != SimpleTy::Void {
                        let value = self.emit_expr(expr, &mut env);
                        let llvm_ty = self.llvm_ty(value.ty);
                        self.lines
                            .push(format!("  ret {llvm_ty} {}", value.llvm_value));
                        returned = true;
                        break;
                    }
                }
            }
            if self.emit_stmt(stmt, &mut env, ret_ty) == ControlFlow::Return {
                returned = true;
            }
        }

        if returned {
            self.lines.push("}".to_string());
            self.lines.push(String::new());
            return;
        }

        if ret_ty == SimpleTy::I64 {
            self.lines.push("  ret i64 0".to_string());
        } else if ret_ty == SimpleTy::Bool {
            self.lines.push("  ret i1 0".to_string());
        } else if ret_ty == SimpleTy::Str {
            self.lines.push("  ret i8* null".to_string());
        } else {
            self.lines.push("  ret void".to_string());
        }
        self.lines.push("}".to_string());
        self.lines.push(String::new());
    }

    fn emit_stmt(
        &mut self,
        stmt: &Stmt,
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ControlFlow {
        match stmt {
            Stmt::Let { name, expr, .. } => {
                let value = self.emit_expr(expr, env);
                let alloca = self.fresh_temp();
                let llvm_ty = self.llvm_ty(value.ty);
                self.lines.push(format!("  {alloca} = alloca {llvm_ty}"));
                self.lines
                    .push(format!("  store {llvm_ty} {}, {llvm_ty}* {alloca}", value.llvm_value));
                env.insert(
                    name.clone(),
                    LocalVar {
                        ptr: alloca,
                        ty: value.ty,
                    },
                );
                ControlFlow::None
            }
            Stmt::While { condition, body } => {
                let head = self.fresh_label("while.head");
                let body_label = self.fresh_label("while.body");
                let end_label = self.fresh_label("while.end");

                self.loop_stack.push(LoopLabels {
                    break_label: end_label.clone(),
                    continue_label: head.clone(),
                });

                self.lines.push(format!("  br label %{head}"));
                self.lines.push(format!("{head}:"));
                let cond_val = self.emit_expr(condition, env);
                let cond = if cond_val.ty == SimpleTy::Bool {
                    cond_val.llvm_value
                } else {
                    "0".to_string()
                };
                self.lines
                    .push(format!("  br i1 {cond}, label %{body_label}, label %{end_label}"));
                self.lines.push(format!("{body_label}:"));
                let body_flow = self.emit_block(body, env, ret_ty);
                let body_terminates = matches!(body_flow, ControlFlow::Break | ControlFlow::Continue | ControlFlow::Return);
                if !body_terminates {
                    self.lines.push(format!("  br label %{head}"));
                }
                self.lines.push(format!("{end_label}:"));
                self.loop_stack.pop();
                match body_flow {
                    ControlFlow::Return => ControlFlow::Return,
                    _ => ControlFlow::None,
                }
            }
            Stmt::For { name, iter, body } => {
                let (start, end) = match self.extract_range(iter, env) {
                    Some(range) => range,
                    None => {
                        self.lines.push("  ; TODO: unsupported for iterator".to_string());
                        return ControlFlow::None;
                    }
                };

                let idx_alloca = self.fresh_temp();
                self.lines.push("  ; for-range".to_string());
                self.lines.push(format!("  {idx_alloca} = alloca i64"));
                self.lines
                    .push(format!("  store i64 {}, i64* {idx_alloca}", start.llvm_value));

                let loop_var_alloca = self.fresh_temp();
                self.lines.push(format!("  {loop_var_alloca} = alloca i64"));
                let previous = env.insert(
                    name.clone(),
                    LocalVar {
                        ptr: loop_var_alloca.clone(),
                        ty: SimpleTy::I64,
                    },
                );

                let head = self.fresh_label("for.head");
                let body_label = self.fresh_label("for.body");
                let step_label = self.fresh_label("for.step");
                let end_label = self.fresh_label("for.end");

                self.loop_stack.push(LoopLabels {
                    break_label: end_label.clone(),
                    continue_label: step_label.clone(),
                });

                self.lines.push(format!("  br label %{head}"));
                self.lines.push(format!("{head}:"));
                let idx_val = self.fresh_temp();
                self.lines
                    .push(format!("  {idx_val} = load i64, i64* {idx_alloca}"));
                let cmp = self.fresh_temp();
                self.lines.push(format!(
                    "  {cmp} = icmp slt i64 {idx_val}, {}",
                    end.llvm_value
                ));
                self.lines
                    .push(format!("  br i1 {cmp}, label %{body_label}, label %{end_label}"));

                self.lines.push(format!("{body_label}:"));
                self.lines
                    .push(format!("  store i64 {idx_val}, i64* {loop_var_alloca}"));
                let body_flow = self.emit_block(body, env, ret_ty);
                if !matches!(body_flow, ControlFlow::Break | ControlFlow::Continue | ControlFlow::Return) {
                    self.lines.push(format!("  br label %{step_label}"));
                }

                self.lines.push(format!("{step_label}:"));
                let idx_next = self.fresh_temp();
                self.lines.push(format!("  {idx_next} = add i64 {idx_val}, 1"));
                self.lines
                    .push(format!("  store i64 {idx_next}, i64* {idx_alloca}"));
                self.lines.push(format!("  br label %{head}"));

                self.lines.push(format!("{end_label}:"));
                self.loop_stack.pop();
                if let Some(prev) = previous {
                    env.insert(name.clone(), prev);
                } else {
                    env.remove(name);
                }
                match body_flow {
                    ControlFlow::Return => ControlFlow::Return,
                    _ => ControlFlow::None,
                }
            }
            Stmt::Break => {
                if let Some(labels) = self.loop_stack.last() {
                    self.lines
                        .push(format!("  br label %{}", labels.break_label));
                    ControlFlow::Break
                } else {
                    self.lines.push("  ; TODO: break outside loop".to_string());
                    ControlFlow::None
                }
            }
            Stmt::Continue => {
                if let Some(labels) = self.loop_stack.last() {
                    self.lines
                        .push(format!("  br label %{}", labels.continue_label));
                    ControlFlow::Continue
                } else {
                    self.lines.push("  ; TODO: continue outside loop".to_string());
                    ControlFlow::None
                }
            }
            Stmt::Expr(expr) => {
                self.emit_expr_stmt(expr, env, ret_ty)
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.emit_expr(expr, env);
                    let llvm_ty = self.llvm_ty(value.ty);
                    self.lines.push(format!("  ret {llvm_ty} {}", value.llvm_value));
                } else {
                    match ret_ty {
                        SimpleTy::I64 => self.lines.push("  ret i64 0".to_string()),
                        SimpleTy::Bool => self.lines.push("  ret i1 0".to_string()),
                        SimpleTy::Str => self.lines.push("  ret i8* null".to_string()),
                        _ => self.lines.push("  ret void".to_string()),
                    }
                }
                ControlFlow::Return
            }
        }
    }

    fn emit_expr_stmt(
        &mut self,
        expr: &Expr,
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ControlFlow {
        match &expr.kind {
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => self.emit_if_stmt(condition, then_branch, else_branch.as_ref(), env, ret_ty),
            ExprKind::Match { expr, arms } => self.emit_match_stmt(expr, arms, env, ret_ty),
            _ => {
                self.emit_expr(expr, env);
                ControlFlow::None
            }
        }
    }

    fn emit_block(
        &mut self,
        stmts: &[Stmt],
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ControlFlow {
        for stmt in stmts {
            let flow = self.emit_stmt(stmt, env, ret_ty);
            if flow != ControlFlow::None {
                return flow;
            }
        }
        ControlFlow::None
    }

    fn emit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &[Stmt],
        else_branch: Option<&tupa_parser::ElseBranch>,
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ControlFlow {
        let cond_val = self.emit_expr(condition, env);
        let cond = if cond_val.ty == SimpleTy::Bool {
            cond_val.llvm_value
        } else {
            "0".to_string()
        };
        let then_label = self.fresh_label("if.then");
        let else_label = self.fresh_label("if.else");
        let end_label = self.fresh_label("if.end");

        self.lines.push(format!(
            "  br i1 {cond}, label %{then_label}, label %{else_label}"
        ));
        self.lines.push(format!("{then_label}:"));
        let then_flow = self.emit_block(then_branch, env, ret_ty);
        if then_flow == ControlFlow::None {
            self.lines.push(format!("  br label %{end_label}"));
        }
        self.lines.push(format!("{else_label}:"));
        let else_flow = match else_branch {
            Some(tupa_parser::ElseBranch::Block(block)) => self.emit_block(block, env, ret_ty),
            Some(tupa_parser::ElseBranch::If(expr)) => self.emit_expr_stmt(expr, env, ret_ty),
            None => ControlFlow::None,
        };
        if else_flow == ControlFlow::None {
            self.lines.push(format!("  br label %{end_label}"));
        }
        self.lines.push(format!("{end_label}:"));
        match (then_flow, else_flow) {
            (ControlFlow::Return, ControlFlow::Return) => ControlFlow::Return,
            _ => ControlFlow::None,
        }
    }

    fn emit_match_stmt(
        &mut self,
        scrutinee: &Expr,
        arms: &[tupa_parser::MatchArm],
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ControlFlow {
        let value = self.emit_expr(scrutinee, env);
        if value.ty != SimpleTy::I64 && value.ty != SimpleTy::Str {
            self.lines.push("  ; TODO: match on non-i64/str".to_string());
            return ControlFlow::None;
        }
        let end_label = self.fresh_label("match.end");
        let mut arm_labels: Vec<String> = Vec::new();
        let mut fallthrough_labels: Vec<String> = Vec::new();

        for _ in arms {
            arm_labels.push(self.fresh_label("match.arm"));
            fallthrough_labels.push(self.fresh_label("match.next"));
        }

        for (idx, arm) in arms.iter().enumerate() {
            let next_label = if idx + 1 < arms.len() {
                &fallthrough_labels[idx]
            } else {
                &end_label
            };
            match (&arm.pattern, value.ty) {
                (tupa_parser::Pattern::Int(value_literal), SimpleTy::I64) => {
                    let cmp = self.fresh_temp();
                    self.lines.push(format!(
                        "  {cmp} = icmp eq i64 {}, {}",
                        value.llvm_value, value_literal
                    ));
                    self.lines.push(format!(
                        "  br i1 {cmp}, label %{}, label %{}",
                        arm_labels[idx], next_label
                    ));
                }
                (tupa_parser::Pattern::Str(lit), SimpleTy::Str) => {
                    let (global, len) = self.intern_string(lit);
                    let ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {global}, i64 0, i64 0"
                    ));
                    let cmp = self.fresh_temp();
                    self.lines.push(format!(
                        "  {cmp} = icmp eq i8* {}, {ptr}",
                        value.llvm_value
                    ));
                    self.lines.push(format!(
                        "  br i1 {cmp}, label %{}, label %{}",
                        arm_labels[idx], next_label
                    ));
                }
                (tupa_parser::Pattern::Wildcard, _) | (tupa_parser::Pattern::Ident(_), _) => {
                    self.lines.push(format!("  br label %{}", arm_labels[idx]));
                }
                _ => {
                    self.lines
                        .push(format!("  br label %{}", next_label));
                }
            }

            self.lines.push(format!("{}:", arm_labels[idx]));
            let returned = match &arm.expr.kind {
                ExprKind::Block(stmts) => self.emit_block(stmts, env, ret_ty),
                _ => self.emit_expr_stmt(&arm.expr, env, ret_ty),
            };
            if returned == ControlFlow::None {
                self.lines.push(format!("  br label %{end_label}"));
            }
            if idx + 1 < arms.len() {
                self.lines.push(format!("{}:", fallthrough_labels[idx]));
            }
        }

        self.lines.push(format!("{end_label}:"));
        ControlFlow::None
    }

    fn emit_expr(&mut self, expr: &Expr, env: &mut HashMap<String, LocalVar>) -> ExprValue {
        match &expr.kind {
            ExprKind::Int(value) => ExprValue::new(SimpleTy::I64, value.to_string()),
            ExprKind::Bool(value) => {
                let literal = if *value { "1" } else { "0" };
                ExprValue::new(SimpleTy::Bool, literal.to_string())
            }
            ExprKind::Str(value) => {
                let (global, len) = self.intern_string(value);
                let ptr = self.fresh_temp();
                self.lines.push(format!(
                    "  {ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {global}, i64 0, i64 0"
                ));
                ExprValue::new(SimpleTy::Str, ptr)
            }
            ExprKind::Ident(name) => {
                if let Some(var) = env.get(name) {
                    let llvm_ty = self.llvm_ty(var.ty);
                    let tmp = self.fresh_temp();
                    self.lines
                        .push(format!("  {tmp} = load {llvm_ty}, {llvm_ty}* {}", var.ptr));
                    return ExprValue::new(var.ty, tmp);
                }
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
            ExprKind::Call { callee, args } => {
                if let ExprKind::Ident(name) = &callee.kind {
                    if name == "print" && args.len() == 1 {
                        let value = self.emit_expr(&args[0], env);
                        match value.ty {
                            SimpleTy::Str => {
                                self.declare_puts();
                                self.lines.push(format!("  call i32 @puts(i8* {})", value.llvm_value));
                            }
                            SimpleTy::I64 => {
                                self.declare_printf();
                                let (fmt, len) = self.intern_format_int();
                                let fmt_ptr = self.fresh_temp();
                                self.lines.push(format!(
                                    "  {fmt_ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {fmt}, i64 0, i64 0"
                                ));
                                self.lines.push(format!(
                                    "  call i32 (i8*, ...) @printf(i8* {fmt_ptr}, i64 {})",
                                    value.llvm_value
                                ));
                            }
                            _ => self.lines.push("  ; TODO: unsupported print type".to_string()),
                        }
                        return ExprValue::new(SimpleTy::Void, "0".to_string());
                    }
                }
                self.lines.push("  ; TODO: unsupported call".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
            ExprKind::Binary { op, left, right } => {
                if matches!(op, tupa_parser::BinaryOp::Range) {
                    self.lines.push("  ; TODO: range expression".to_string());
                    return ExprValue::new(SimpleTy::Unknown, "0".to_string());
                }
                if matches!(op, tupa_parser::BinaryOp::Add) {
                    if let (ExprKind::Str(lhs), ExprKind::Str(rhs)) = (&left.kind, &right.kind) {
                        let combined = format!("{lhs}{rhs}");
                        let (global, len) = self.intern_string(&combined);
                        let ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {global}, i64 0, i64 0"
                        ));
                        return ExprValue::new(SimpleTy::Str, ptr);
                    }
                }
                let left_val = self.emit_expr(left, env);
                let right_val = self.emit_expr(right, env);
                match (left_val.ty, right_val.ty) {
                    (SimpleTy::I64, SimpleTy::I64) => {
                        let op = match op {
                            tupa_parser::BinaryOp::Add => Some("add"),
                            tupa_parser::BinaryOp::Sub => Some("sub"),
                            tupa_parser::BinaryOp::Mul => Some("mul"),
                            tupa_parser::BinaryOp::Div => Some("sdiv"),
                            tupa_parser::BinaryOp::Equal => Some("icmp eq"),
                            tupa_parser::BinaryOp::NotEqual => Some("icmp ne"),
                            tupa_parser::BinaryOp::Less => Some("icmp slt"),
                            tupa_parser::BinaryOp::LessEqual => Some("icmp sle"),
                            tupa_parser::BinaryOp::Greater => Some("icmp sgt"),
                            tupa_parser::BinaryOp::GreaterEqual => Some("icmp sge"),
                            _ => None,
                        };
                        if let Some(op) = op {
                            let tmp = self.fresh_temp();
                            if op.starts_with("icmp") {
                                self.lines.push(format!(
                                    "  {tmp} = {op} i64 {}, {}",
                                    left_val.llvm_value, right_val.llvm_value
                                ));
                                return ExprValue::new(SimpleTy::Bool, tmp);
                            }
                            self.lines.push(format!(
                                "  {tmp} = {op} i64 {}, {}",
                                left_val.llvm_value, right_val.llvm_value
                            ));
                            return ExprValue::new(SimpleTy::I64, tmp);
                        }
                    }
                    (SimpleTy::Str, SimpleTy::Str) => {
                        self.lines.push("  ; TODO: runtime string concat".to_string());
                        return ExprValue::new(SimpleTy::Str, "null".to_string());
                    }
                    _ => {}
                }
                self.lines.push("  ; TODO: unsupported binary".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
            _ => {
                self.lines.push("  ; TODO: unsupported expression".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
        }
    }

    fn map_type(&self, ty: &Type) -> String {
        match ty {
            Type::Ident(name) if name == "i64" => "i64".to_string(),
            Type::Ident(name) if name == "bool" => "i1".to_string(),
            Type::Ident(name) if name == "string" => "i8*".to_string(),
            _ => "void".to_string(),
        }
    }

    fn llvm_ty(&self, ty: SimpleTy) -> &'static str {
        match ty {
            SimpleTy::I64 => "i64",
            SimpleTy::Bool => "i1",
            SimpleTy::Str => "i8*",
            SimpleTy::Void => "void",
            SimpleTy::Unknown => "i64",
        }
    }

    fn intern_string(&mut self, value: &str) -> (String, usize) {
        if let Some((name, len)) = self.string_pool.get(value) {
            return (name.clone(), *len);
        }
        let (escaped, len) = escape_llvm_string(value);
        let name = format!("@.str{}", self.string_pool.len());
        let literal = format!(
            "{name} = private unnamed_addr constant [{len} x i8] c\"{escaped}\\00\""
        );
        self.globals.push(literal);
        self.string_pool
            .insert(value.to_string(), (name.clone(), len));
        (name, len)
    }

    fn intern_format_int(&mut self) -> (String, usize) {
        if self.fmt_int_emitted {
            return ("@.fmt_int".to_string(), 5);
        }
        self.fmt_int_emitted = true;
        let literal = "@.fmt_int = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\"";
        self.globals.push(literal.to_string());
        ("@.fmt_int".to_string(), 5)
    }

    fn declare_printf(&mut self) {
        if !self.printf_declared {
            self.globals
                .push("declare i32 @printf(i8*, ...)".to_string());
            self.printf_declared = true;
        }
    }

    fn declare_puts(&mut self) {
        if !self.puts_declared {
            self.globals.push("declare i32 @puts(i8*)".to_string());
            self.puts_declared = true;
        }
    }

    fn fresh_temp(&mut self) -> String {
        let name = format!("%t{}", self.temp);
        self.temp += 1;
        name
    }

    fn fresh_label(&mut self, prefix: &str) -> String {
        let name = format!("{prefix}{}", self.label);
        self.label += 1;
        name
    }

    fn extract_range(
        &mut self,
        iter: &Expr,
        env: &mut HashMap<String, LocalVar>,
    ) -> Option<(ExprValue, ExprValue)> {
        if let ExprKind::Binary { op, left, right } = &iter.kind {
            if *op != tupa_parser::BinaryOp::Range {
                return None;
            }
            let start = self.emit_expr(left, env);
            let end = self.emit_expr(right, env);
            if start.ty == SimpleTy::I64 && end.ty == SimpleTy::I64 {
                return Some((start, end));
            }
        }
        None
    }

    fn finish(mut self) -> String {
        if !self.globals.is_empty() {
            self.globals.push(String::new());
        }
        self.globals.extend(self.lines);
        self.globals.join("\n")
    }
}

struct ExprValue {
    ty: SimpleTy,
    llvm_value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlFlow {
    None,
    Break,
    Continue,
    Return,
}

impl ExprValue {
    fn new(ty: SimpleTy, llvm_value: String) -> Self {
        Self { ty, llvm_value }
    }
}

fn escape_llvm_string(value: &str) -> (String, usize) {
    let mut out = String::new();
    let mut len = 0usize;
    for &byte in value.as_bytes() {
        len += 1;
        match byte {
            b'\\' => out.push_str("\\5C"),
            b'"' => out.push_str("\\22"),
            b'\n' => out.push_str("\\0A"),
            b'\r' => out.push_str("\\0D"),
            b'\t' => out.push_str("\\09"),
            0x20..=0x7E => out.push(byte as char),
            _ => out.push_str(&format!("\\{:02X}", byte)),
        }
    }
    (out, len + 1)
}
