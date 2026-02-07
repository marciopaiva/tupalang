use std::collections::HashMap;

use tupa_parser::{Expr, ExprKind, Function, Item, Program, Stmt, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SimpleTy {
    I64,
    F64,
    Bool,
    I64Ptr,
    F64Ptr,
    Str,
    StrPtr,
    BoolPtr,
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
    fmt_float_emitted: bool,
    printf_declared: bool,
    puts_declared: bool,
    strcmp_declared: bool,
    strlen_declared: bool,
    malloc_declared: bool,
    strcpy_declared: bool,
    strcat_declared: bool,
    snprintf_declared: bool,
    function_sigs: HashMap<String, FuncSig>,
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

#[derive(Debug, Clone)]
struct FuncSig {
    #[allow(dead_code)]
    params: Vec<SimpleTy>,
    ret: SimpleTy,
}

impl Codegen {
    fn declare_snprintf(&mut self) {
        if !self.snprintf_declared {
            self.globals
                .push("declare i32 @snprintf(i8*, i64, i8*, ...)".to_string());
            self.snprintf_declared = true;
        }
    }
    // Implementação real de concatenação de strings
    fn emit_string_concat(&mut self, left: ExprValue, right: ExprValue) -> ExprValue {
        self.declare_strlen();
        self.declare_malloc();
        self.declare_strcpy();
        self.declare_strcat();
        // strlen(left)
        let len_left = self.fresh_temp();
        self.lines.push(format!(
            "  {len_left} = call i64 @strlen(i8* {})",
            left.llvm_value
        ));
        // strlen(right)
        let len_right = self.fresh_temp();
        self.lines.push(format!(
            "  {len_right} = call i64 @strlen(i8* {})",
            right.llvm_value
        ));
        // total = len_left + len_right + 1
        let total = self.fresh_temp();
        self.lines
            .push(format!("  {total} = add i64 {len_left}, {len_right}"));
        let total1 = self.fresh_temp();
        self.lines.push(format!("  {total1} = add i64 {total}, 1"));
        // malloc(total1)
        let buf = self.fresh_temp();
        self.lines
            .push(format!("  {buf} = call i8* @malloc(i64 {total1})"));
        // strcpy(buf, left)
        self.lines.push(format!(
            "  call i8* @strcpy(i8* {buf}, i8* {})",
            left.llvm_value
        ));
        // strcat(buf, right)
        self.lines.push(format!(
            "  call i8* @strcat(i8* {buf}, i8* {})",
            right.llvm_value
        ));
        ExprValue::new(SimpleTy::Str, buf)
    }

    // Implementação real de formatação de valor como string
    fn emit_format_value(&mut self, val: ExprValue) -> Option<ExprValue> {
        match val.ty {
            SimpleTy::I64 => {
                self.declare_printf();
                let (fmt, len) = self.intern_format_int();
                let fmt_ptr = self.fresh_temp();
                self.lines.push(format!("  {fmt_ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {fmt}, i64 0, i64 0"));
                // Aloca buffer
                self.declare_malloc();
                let buf = self.fresh_temp();
                self.lines
                    .push(format!("  {buf} = call i8* @malloc(i64 32)"));
                // snprintf
                self.declare_snprintf();
                self.lines.push(format!(
                    "  call i32 @snprintf(i8* {buf}, i64 32, i8* {fmt_ptr}, i64 {})",
                    val.llvm_value
                ));
                Some(ExprValue::new(SimpleTy::Str, buf))
            }
            SimpleTy::F64 => {
                self.declare_printf();
                let (fmt, len) = self.intern_format_float();
                let fmt_ptr = self.fresh_temp();
                self.lines.push(format!("  {fmt_ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {fmt}, i64 0, i64 0"));
                self.declare_malloc();
                let buf = self.fresh_temp();
                self.lines
                    .push(format!("  {buf} = call i8* @malloc(i64 32)"));
                self.declare_snprintf();
                self.lines.push(format!(
                    "  call i32 @snprintf(i8* {buf}, i64 32, i8* {fmt_ptr}, double {})",
                    val.llvm_value
                ));
                Some(ExprValue::new(SimpleTy::Str, buf))
            }
            SimpleTy::Bool => {
                // bool para string: "true" ou "false"
                let true_str = self.intern_string("true").0;
                let false_str = self.intern_string("false").0;
                let select = self.fresh_temp();
                self.lines.push(format!(
                    "  {select} = select i1 {}, i8* {}, i8* {}",
                    val.llvm_value, true_str, false_str
                ));
                Some(ExprValue::new(SimpleTy::Str, select))
            }
            SimpleTy::Str => Some(val),
            _ => None,
        }
    }
    fn emit_program(&mut self, program: &Program) {
        for item in &program.items {
            let func = match item {
                Item::Function(func) => func,
                Item::Enum(_) => continue, // enums don't have functions
                Item::Trait(_) => continue, // traits don't have functions
            };
            let params = func
                .params
                .iter()
                .map(|p| self.simple_ty_from_type(&p.ty))
                .collect::<Vec<_>>();
            let ret = match func.return_type.as_ref() {
                Some(ty) => self.simple_ty_from_type(ty),
                None => SimpleTy::Void,
            };
            self.function_sigs
                .insert(func.name.clone(), FuncSig { params, ret });
        }
        for item in &program.items {
            match item {
                Item::Function(func) => self.emit_function(func),
                Item::Enum(_) => {} // enums don't emit code yet
                Item::Trait(_) => {} // traits don't emit code yet
            }
        }
    }

    fn emit_function(&mut self, func: &Function) {
        let mut env: HashMap<String, LocalVar> = HashMap::new();
        let ret_ty = match func.return_type.as_ref() {
            Some(Type::Ident(name)) if name == "i64" => SimpleTy::I64,
            Some(Type::Ident(name)) if name == "f64" => SimpleTy::F64,
            Some(Type::Ident(name)) if name == "bool" => SimpleTy::Bool,
            Some(Type::Ident(name)) if name == "string" => SimpleTy::Str,
            Some(Type::Array { elem, .. }) if matches!(**elem, Type::Ident(ref n) if n == "i64") => {
                SimpleTy::I64Ptr
            }
            Some(Type::Array { elem, .. }) if matches!(**elem, Type::Ident(ref n) if n == "f64") => {
                SimpleTy::F64Ptr
            }
            Some(Type::Array { elem, .. }) if matches!(**elem, Type::Ident(ref n) if n == "string") => {
                SimpleTy::StrPtr
            }
            Some(Type::Array { elem, .. }) if matches!(**elem, Type::Ident(ref n) if n == "bool") => {
                SimpleTy::BoolPtr
            }
            Some(Type::Slice { elem }) if matches!(**elem, Type::Ident(ref n) if n == "i64") => {
                SimpleTy::I64Ptr
            }
            Some(Type::Slice { elem }) if matches!(**elem, Type::Ident(ref n) if n == "f64") => {
                SimpleTy::F64Ptr
            }
            Some(Type::Slice { elem }) if matches!(**elem, Type::Ident(ref n) if n == "string") => {
                SimpleTy::StrPtr
            }
            Some(Type::Slice { elem }) if matches!(**elem, Type::Ident(ref n) if n == "bool") => {
                SimpleTy::BoolPtr
            }
            Some(_) => SimpleTy::Unknown,
            None => SimpleTy::Void,
        };
        let llvm_ret = match ret_ty {
            SimpleTy::I64 => "i64",
            SimpleTy::F64 => "double",
            SimpleTy::Bool => "i1",
            SimpleTy::I64Ptr => "i64*",
            SimpleTy::F64Ptr => "double*",
            SimpleTy::Str => "i8*",
            SimpleTy::StrPtr => "i8**",
            SimpleTy::BoolPtr => "i1*",
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
                "double" => SimpleTy::F64,
                "i1" => SimpleTy::Bool,
                "i64*" => SimpleTy::I64Ptr,
                "double*" => SimpleTy::F64Ptr,
                "i8*" => SimpleTy::Str,
                "i8**" => SimpleTy::StrPtr,
                "i1*" => SimpleTy::BoolPtr,
                _ => SimpleTy::Unknown,
            };
            let ptr = format!("%{}", param.name);
            let alloca = self.fresh_temp();
            self.lines
                .push(format!("  {alloca} = alloca {}", self.map_type(&param.ty)));
            self.lines.push(format!(
                "  store {} {ptr}, {}* {alloca}",
                self.map_type(&param.ty),
                self.map_type(&param.ty)
            ));
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
        } else if ret_ty == SimpleTy::F64 {
            self.lines.push("  ret double 0.0".to_string());
        } else if ret_ty == SimpleTy::Bool {
            self.lines.push("  ret i1 0".to_string());
        } else if ret_ty == SimpleTy::I64Ptr {
            self.lines.push("  ret i64* null".to_string());
        } else if ret_ty == SimpleTy::F64Ptr {
            self.lines.push("  ret double* null".to_string());
        } else if ret_ty == SimpleTy::Str {
            self.lines.push("  ret i8* null".to_string());
        } else if ret_ty == SimpleTy::StrPtr {
            self.lines.push("  ret i8** null".to_string());
        } else if ret_ty == SimpleTy::BoolPtr {
            self.lines.push("  ret i1* null".to_string());
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
                self.lines.push(format!(
                    "  store {llvm_ty} {}, {llvm_ty}* {alloca}",
                    value.llvm_value
                ));
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
                self.lines.push(format!(
                    "  br i1 {cond}, label %{body_label}, label %{end_label}"
                ));
                self.lines.push(format!("{body_label}:"));
                let body_flow = self.emit_block(body, env, ret_ty);
                let body_terminates = matches!(
                    body_flow,
                    ControlFlow::Break | ControlFlow::Continue | ControlFlow::Return
                );
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
                        self.lines
                            .push("  ; TODO: unsupported for iterator".to_string());
                        return ControlFlow::None;
                    }
                };

                let idx_alloca = self.fresh_temp();
                self.lines.push("  ; for-range".to_string());
                self.lines.push(format!("  {idx_alloca} = alloca i64"));
                self.lines.push(format!(
                    "  store i64 {}, i64* {idx_alloca}",
                    start.llvm_value
                ));

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
                self.lines.push(format!(
                    "  br i1 {cmp}, label %{body_label}, label %{end_label}"
                ));

                self.lines.push(format!("{body_label}:"));
                self.lines
                    .push(format!("  store i64 {idx_val}, i64* {loop_var_alloca}"));
                let body_flow = self.emit_block(body, env, ret_ty);
                if !matches!(
                    body_flow,
                    ControlFlow::Break | ControlFlow::Continue | ControlFlow::Return
                ) {
                    self.lines.push(format!("  br label %{step_label}"));
                }

                self.lines.push(format!("{step_label}:"));
                let idx_next = self.fresh_temp();
                self.lines
                    .push(format!("  {idx_next} = add i64 {idx_val}, 1"));
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
                    self.lines
                        .push("  ; TODO: continue outside loop".to_string());
                    ControlFlow::None
                }
            }
            Stmt::Expr(expr) => self.emit_expr_stmt(expr, env, ret_ty),
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.emit_expr(expr, env);
                    let llvm_ty = self.llvm_ty(value.ty);
                    self.lines
                        .push(format!("  ret {llvm_ty} {}", value.llvm_value));
                } else {
                    match ret_ty {
                        SimpleTy::I64 => self.lines.push("  ret i64 0".to_string()),
                        SimpleTy::Bool => self.lines.push("  ret i1 0".to_string()),
                        SimpleTy::I64Ptr => self.lines.push("  ret i64* null".to_string()),
                        SimpleTy::Str => self.lines.push("  ret i8* null".to_string()),
                        _ => self.lines.push("  ret void".to_string()),
                    }
                }
                ControlFlow::Return
            }
            Stmt::Lambda { .. } => {
                // Not supported as a statement; skip or error as needed
                ControlFlow::None
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
            self.lines
                .push("  ; TODO: match on non-i64/str".to_string());
            return ControlFlow::None;
        }
        let end_label = self.fresh_label("match.end");
        let mut arm_labels: Vec<String> = Vec::new();
        let mut fallthrough_labels: Vec<String> = Vec::new();
        let mut guard_labels: Vec<Option<String>> = Vec::new();

        for _ in arms {
            arm_labels.push(self.fresh_label("match.arm"));
            fallthrough_labels.push(self.fresh_label("match.next"));
            guard_labels.push(None);
        }

        for (idx, arm) in arms.iter().enumerate() {
            if arm.guard.is_some() {
                guard_labels[idx] = Some(self.fresh_label("match.guard"));
            }
        }

        for (idx, arm) in arms.iter().enumerate() {
            let next_label = if idx + 1 < arms.len() {
                &fallthrough_labels[idx]
            } else {
                &end_label
            };
            let arm_target = guard_labels[idx].as_ref().unwrap_or(&arm_labels[idx]);
            let binding_name = match &arm.pattern {
                tupa_parser::Pattern::Ident(name) => Some(name.clone()),
                _ => None,
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
                        arm_target, next_label
                    ));
                }
                (tupa_parser::Pattern::Str(lit), SimpleTy::Str) => {
                    let (global, len) = self.intern_string(lit);
                    let ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {global}, i64 0, i64 0"
                    ));
                    self.declare_strcmp();
                    let cmp = self.fresh_temp();
                    self.lines.push(format!(
                        "  {cmp} = call i32 @strcmp(i8* {}, i8* {ptr})",
                        value.llvm_value
                    ));
                    let is_eq = self.fresh_temp();
                    self.lines.push(format!("  {is_eq} = icmp eq i32 {cmp}, 0"));
                    self.lines.push(format!(
                        "  br i1 {is_eq}, label %{}, label %{}",
                        arm_target, next_label
                    ));
                }
                (tupa_parser::Pattern::Wildcard, _) | (tupa_parser::Pattern::Ident(_), _) => {
                    self.lines.push(format!("  br label %{}", arm_target));
                }
                _ => {
                    self.lines.push(format!("  br label %{}", next_label));
                }
            }

            let mut prev_binding: Option<LocalVar> = None;
            let mut bound = false;
            if let Some(guard_label) = &guard_labels[idx] {
                self.lines.push(format!("{guard_label}:"));
                if let Some(name) = &binding_name {
                    let llvm_ty = self.llvm_ty(value.ty);
                    let alloca = self.fresh_temp();
                    self.lines.push(format!("  {alloca} = alloca {llvm_ty}"));
                    self.lines.push(format!(
                        "  store {llvm_ty} {}, {llvm_ty}* {alloca}",
                        value.llvm_value
                    ));
                    prev_binding = env.insert(
                        name.clone(),
                        LocalVar {
                            ptr: alloca,
                            ty: value.ty,
                        },
                    );
                    bound = true;
                }
                let guard_value = self.emit_expr(arm.guard.as_ref().unwrap(), env);
                let cond = if guard_value.ty == SimpleTy::Bool {
                    guard_value.llvm_value
                } else {
                    "0".to_string()
                };
                self.lines.push(format!(
                    "  br i1 {cond}, label %{}, label %{}",
                    arm_labels[idx], next_label
                ));
            }

            self.lines.push(format!("{}:", arm_labels[idx]));
            if !bound {
                if let Some(name) = &binding_name {
                    let llvm_ty = self.llvm_ty(value.ty);
                    let alloca = self.fresh_temp();
                    self.lines.push(format!("  {alloca} = alloca {llvm_ty}"));
                    self.lines.push(format!(
                        "  store {llvm_ty} {}, {llvm_ty}* {alloca}",
                        value.llvm_value
                    ));
                    prev_binding = env.insert(
                        name.clone(),
                        LocalVar {
                            ptr: alloca,
                            ty: value.ty,
                        },
                    );
                }
            }
            let returned = match &arm.expr.kind {
                ExprKind::Block(stmts) => self.emit_block(stmts, env, ret_ty),
                _ => self.emit_expr_stmt(&arm.expr, env, ret_ty),
            };
            if returned == ControlFlow::None {
                self.lines.push(format!("  br label %{end_label}"));
            }
            if let Some(name) = &binding_name {
                if let Some(prev) = prev_binding.take() {
                    env.insert(name.clone(), prev);
                } else {
                    env.remove(name);
                }
            }
            if idx + 1 < arms.len() {
                self.lines.push(format!("{}:", fallthrough_labels[idx]));
            }
        }

        self.lines.push(format!("{end_label}:"));
        ControlFlow::None
    }

    fn emit_block_expr(
        &mut self,
        stmts: &[Stmt],
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ExprValue {
        if stmts.is_empty() {
            return ExprValue::new(SimpleTy::Void, "0".to_string());
        }
        let last_index = stmts.len().saturating_sub(1);
        for (idx, stmt) in stmts.iter().enumerate() {
            if idx == last_index {
                if let Stmt::Expr(expr) = stmt {
                    return self.emit_expr(expr, env);
                }
                self.emit_stmt(stmt, env, ret_ty);
                return ExprValue::new(SimpleTy::Void, "0".to_string());
            }
            if self.emit_stmt(stmt, env, ret_ty) == ControlFlow::Return {
                return ExprValue::new(SimpleTy::Void, "0".to_string());
            }
        }
        ExprValue::new(SimpleTy::Void, "0".to_string())
    }

    fn infer_block_expr_ty(&self, stmts: &[Stmt], env: &HashMap<String, LocalVar>) -> SimpleTy {
        for stmt in stmts.iter().rev() {
            match stmt {
                Stmt::Expr(expr) => return self.infer_expr_ty(expr, env),
                Stmt::Return(Some(expr)) => return self.infer_expr_ty(expr, env),
                Stmt::Return(None) => return SimpleTy::Void,
                _ => continue,
            }
        }
        SimpleTy::Void
    }

    fn infer_expr_ty(&self, expr: &Expr, env: &HashMap<String, LocalVar>) -> SimpleTy {
        match &expr.kind {
            ExprKind::Int(_) => SimpleTy::I64,
            ExprKind::Float(_) => SimpleTy::F64,
            ExprKind::Bool(_) => SimpleTy::Bool,
            ExprKind::Str(_) => SimpleTy::Str,
            ExprKind::ArrayLiteral(items) => {
                if items.is_empty() {
                    return SimpleTy::I64Ptr;
                }
                let first = self.infer_expr_ty(&items[0], env);
                if first == SimpleTy::I64 {
                    SimpleTy::I64Ptr
                } else if first == SimpleTy::F64 {
                    SimpleTy::F64Ptr
                } else if first == SimpleTy::Str {
                    SimpleTy::StrPtr
                } else if first == SimpleTy::Bool {
                    SimpleTy::BoolPtr
                } else {
                    SimpleTy::Unknown
                }
            }
            ExprKind::Ident(name) => env.get(name).map(|v| v.ty).unwrap_or(SimpleTy::Unknown),
            ExprKind::Assign { expr, .. } => self.infer_expr_ty(expr, env),
            ExprKind::AssignIndex { expr, .. } => match self.infer_expr_ty(expr, env) {
                SimpleTy::F64Ptr => SimpleTy::F64,
                SimpleTy::I64Ptr => SimpleTy::I64,
                SimpleTy::StrPtr => SimpleTy::Str,
                SimpleTy::BoolPtr => SimpleTy::Bool,
                _ => SimpleTy::Unknown,
            },
            ExprKind::Index { expr, .. } => match self.infer_expr_ty(expr, env) {
                SimpleTy::F64Ptr => SimpleTy::F64,
                SimpleTy::I64Ptr => SimpleTy::I64,
                SimpleTy::StrPtr => SimpleTy::Str,
                SimpleTy::BoolPtr => SimpleTy::Bool,
                _ => SimpleTy::Unknown,
            },
            ExprKind::Unary { op, expr } => {
                let inner = self.infer_expr_ty(expr, env);
                match (op, inner) {
                    (tupa_parser::UnaryOp::Neg, SimpleTy::I64) => SimpleTy::I64,
                    (tupa_parser::UnaryOp::Neg, SimpleTy::F64) => SimpleTy::F64,
                    (tupa_parser::UnaryOp::Not, SimpleTy::Bool) => SimpleTy::Bool,
                    _ => SimpleTy::Unknown,
                }
            }
            ExprKind::Binary { op, left, right } => {
                let l = self.infer_expr_ty(left, env);
                let r = self.infer_expr_ty(right, env);
                match (l, r) {
                    (SimpleTy::I64, SimpleTy::I64) => match op {
                        tupa_parser::BinaryOp::Equal
                        | tupa_parser::BinaryOp::NotEqual
                        | tupa_parser::BinaryOp::Less
                        | tupa_parser::BinaryOp::LessEqual
                        | tupa_parser::BinaryOp::Greater
                        | tupa_parser::BinaryOp::GreaterEqual => SimpleTy::Bool,
                        _ => SimpleTy::I64,
                    },
                    (SimpleTy::F64, SimpleTy::F64) => match op {
                        tupa_parser::BinaryOp::Equal
                        | tupa_parser::BinaryOp::NotEqual
                        | tupa_parser::BinaryOp::Less
                        | tupa_parser::BinaryOp::LessEqual
                        | tupa_parser::BinaryOp::Greater
                        | tupa_parser::BinaryOp::GreaterEqual => SimpleTy::Bool,
                        _ => SimpleTy::F64,
                    },
                    (SimpleTy::Bool, SimpleTy::Bool) => match op {
                        tupa_parser::BinaryOp::And
                        | tupa_parser::BinaryOp::Or
                        | tupa_parser::BinaryOp::Equal
                        | tupa_parser::BinaryOp::NotEqual => SimpleTy::Bool,
                        _ => SimpleTy::Unknown,
                    },
                    (SimpleTy::Str, SimpleTy::Str) => match op {
                        tupa_parser::BinaryOp::Add => SimpleTy::Str,
                        tupa_parser::BinaryOp::Equal | tupa_parser::BinaryOp::NotEqual => {
                            SimpleTy::Bool
                        }
                        _ => SimpleTy::Unknown,
                    },
                    (SimpleTy::Str, SimpleTy::I64 | SimpleTy::F64 | SimpleTy::Bool)
                    | (SimpleTy::I64 | SimpleTy::F64 | SimpleTy::Bool, SimpleTy::Str)
                        if matches!(op, tupa_parser::BinaryOp::Add) =>
                    {
                        SimpleTy::Str
                    }
                    _ => SimpleTy::Unknown,
                }
            }
            ExprKind::Call { callee, .. } => {
                if let ExprKind::Ident(name) = &callee.kind {
                    if name == "print" {
                        return SimpleTy::Void;
                    }
                    if let Some(sig) = self.function_sigs.get(name) {
                        return sig.ret;
                    }
                }
                SimpleTy::Unknown
            }
            ExprKind::If {
                then_branch,
                else_branch,
                ..
            } => match else_branch {
                Some(tupa_parser::ElseBranch::Block(block)) => {
                    let then_ty = self.infer_block_expr_ty(then_branch, env);
                    let else_ty = self.infer_block_expr_ty(block, env);
                    if then_ty == else_ty {
                        then_ty
                    } else {
                        SimpleTy::Unknown
                    }
                }
                Some(tupa_parser::ElseBranch::If(expr)) => {
                    let then_ty = self.infer_block_expr_ty(then_branch, env);
                    let else_ty = self.infer_expr_ty(expr, env);
                    if then_ty == else_ty {
                        then_ty
                    } else {
                        SimpleTy::Unknown
                    }
                }
                None => SimpleTy::Void,
            },
            ExprKind::Match { arms, .. } => {
                let mut expected: Option<SimpleTy> = None;
                for arm in arms {
                    let arm_ty = match &arm.expr.kind {
                        ExprKind::Block(stmts) => self.infer_block_expr_ty(stmts, env),
                        _ => self.infer_expr_ty(&arm.expr, env),
                    };
                    match expected {
                        None => expected = Some(arm_ty),
                        Some(prev) if prev != arm_ty => return SimpleTy::Unknown,
                        _ => {}
                    }
                }
                expected.unwrap_or(SimpleTy::Void)
            }
            ExprKind::Block(stmts) => self.infer_block_expr_ty(stmts, env),
            _ => SimpleTy::Unknown,
        }
    }

    fn emit_if_expr(
        &mut self,
        condition: &Expr,
        then_branch: &[Stmt],
        else_branch: Option<&tupa_parser::ElseBranch>,
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ExprValue {
        if else_branch.is_none() {
            self.emit_if_stmt(condition, then_branch, else_branch, env, ret_ty);
            return ExprValue::new(SimpleTy::Void, "0".to_string());
        }
        let result_ty = match else_branch {
            Some(tupa_parser::ElseBranch::Block(block)) => {
                let then_ty = self.infer_block_expr_ty(then_branch, env);
                let else_ty = self.infer_block_expr_ty(block, env);
                if then_ty == else_ty {
                    then_ty
                } else {
                    SimpleTy::Unknown
                }
            }
            Some(tupa_parser::ElseBranch::If(expr)) => {
                let then_ty = self.infer_block_expr_ty(then_branch, env);
                let else_ty = self.infer_expr_ty(expr, env);
                if then_ty == else_ty {
                    then_ty
                } else {
                    SimpleTy::Unknown
                }
            }
            None => SimpleTy::Void,
        };
        if matches!(result_ty, SimpleTy::Void | SimpleTy::Unknown) {
            self.emit_if_stmt(condition, then_branch, else_branch, env, ret_ty);
            return ExprValue::new(SimpleTy::Unknown, "0".to_string());
        }

        let result_alloca = self.fresh_temp();
        let llvm_ty = self.llvm_ty(result_ty);
        self.lines
            .push(format!("  {result_alloca} = alloca {llvm_ty}"));

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
        let then_val = self.emit_block_expr(then_branch, env, ret_ty);
        if then_val.ty == result_ty {
            self.lines.push(format!(
                "  store {llvm_ty} {}, {llvm_ty}* {result_alloca}",
                then_val.llvm_value
            ));
        }
        self.lines.push(format!("  br label %{end_label}"));

        self.lines.push(format!("{else_label}:"));
        let else_val = match else_branch {
            Some(tupa_parser::ElseBranch::Block(block)) => self.emit_block_expr(block, env, ret_ty),
            Some(tupa_parser::ElseBranch::If(expr)) => self.emit_expr(expr, env),
            None => ExprValue::new(SimpleTy::Void, "0".to_string()),
        };
        if else_val.ty == result_ty {
            self.lines.push(format!(
                "  store {llvm_ty} {}, {llvm_ty}* {result_alloca}",
                else_val.llvm_value
            ));
        }
        self.lines.push(format!("  br label %{end_label}"));
        self.lines.push(format!("{end_label}:"));
        let out = self.fresh_temp();
        self.lines.push(format!(
            "  {out} = load {llvm_ty}, {llvm_ty}* {result_alloca}"
        ));
        ExprValue::new(result_ty, out)
    }

    fn emit_match_expr(
        &mut self,
        scrutinee: &Expr,
        arms: &[tupa_parser::MatchArm],
        env: &mut HashMap<String, LocalVar>,
        ret_ty: SimpleTy,
    ) -> ExprValue {
        let value = self.emit_expr(scrutinee, env);
        if value.ty != SimpleTy::I64 && value.ty != SimpleTy::Str {
            self.lines
                .push("  ; TODO: match on non-i64/str".to_string());
            return ExprValue::new(SimpleTy::Unknown, "0".to_string());
        }

        let mut result_ty: Option<SimpleTy> = None;
        for arm in arms {
            let arm_ty = match &arm.expr.kind {
                ExprKind::Block(stmts) => self.infer_block_expr_ty(stmts, env),
                _ => self.infer_expr_ty(&arm.expr, env),
            };
            match result_ty {
                None => result_ty = Some(arm_ty),
                Some(prev) if prev != arm_ty => {
                    result_ty = Some(SimpleTy::Unknown);
                    break;
                }
                _ => {}
            }
        }
        let result_ty = result_ty.unwrap_or(SimpleTy::Void);
        if matches!(result_ty, SimpleTy::Void | SimpleTy::Unknown) {
            self.emit_match_stmt(scrutinee, arms, env, ret_ty);
            return ExprValue::new(SimpleTy::Unknown, "0".to_string());
        }

        let result_alloca = self.fresh_temp();
        let llvm_ty = self.llvm_ty(result_ty);
        self.lines
            .push(format!("  {result_alloca} = alloca {llvm_ty}"));

        let end_label = self.fresh_label("match.end");
        let mut arm_labels: Vec<String> = Vec::new();
        let mut fallthrough_labels: Vec<String> = Vec::new();
        let mut guard_labels: Vec<Option<String>> = Vec::new();

        for _ in arms {
            arm_labels.push(self.fresh_label("match.arm"));
            fallthrough_labels.push(self.fresh_label("match.next"));
            guard_labels.push(None);
        }

        for (idx, arm) in arms.iter().enumerate() {
            if arm.guard.is_some() {
                guard_labels[idx] = Some(self.fresh_label("match.guard"));
            }
        }

        for (idx, arm) in arms.iter().enumerate() {
            let next_label = if idx + 1 < arms.len() {
                &fallthrough_labels[idx]
            } else {
                &end_label
            };
            let arm_target = guard_labels[idx].as_ref().unwrap_or(&arm_labels[idx]);
            let binding_name = match &arm.pattern {
                tupa_parser::Pattern::Ident(name) => Some(name.clone()),
                _ => None,
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
                        arm_target, next_label
                    ));
                }
                (tupa_parser::Pattern::Str(lit), SimpleTy::Str) => {
                    let (global, len) = self.intern_string(lit);
                    let ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {global}, i64 0, i64 0"
                    ));
                    self.declare_strcmp();
                    let cmp = self.fresh_temp();
                    self.lines.push(format!(
                        "  {cmp} = call i32 @strcmp(i8* {}, i8* {ptr})",
                        value.llvm_value
                    ));
                    let is_eq = self.fresh_temp();
                    self.lines.push(format!("  {is_eq} = icmp eq i32 {cmp}, 0"));
                    self.lines.push(format!(
                        "  br i1 {is_eq}, label %{}, label %{}",
                        arm_target, next_label
                    ));
                }
                (tupa_parser::Pattern::Wildcard, _) | (tupa_parser::Pattern::Ident(_), _) => {
                    self.lines.push(format!("  br label %{}", arm_target));
                }
                _ => {
                    self.lines.push(format!("  br label %{}", next_label));
                }
            }

            let mut prev_binding: Option<LocalVar> = None;
            let mut bound = false;
            if let Some(guard_label) = &guard_labels[idx] {
                self.lines.push(format!("{guard_label}:"));
                if let Some(name) = &binding_name {
                    let llvm_val_ty = self.llvm_ty(value.ty);
                    let alloca = self.fresh_temp();
                    self.lines
                        .push(format!("  {alloca} = alloca {llvm_val_ty}"));
                    self.lines.push(format!(
                        "  store {llvm_val_ty} {}, {llvm_val_ty}* {alloca}",
                        value.llvm_value
                    ));
                    prev_binding = env.insert(
                        name.clone(),
                        LocalVar {
                            ptr: alloca,
                            ty: value.ty,
                        },
                    );
                    bound = true;
                }
                let guard_value = self.emit_expr(arm.guard.as_ref().unwrap(), env);
                let cond = if guard_value.ty == SimpleTy::Bool {
                    guard_value.llvm_value
                } else {
                    "0".to_string()
                };
                self.lines.push(format!(
                    "  br i1 {cond}, label %{}, label %{}",
                    arm_labels[idx], next_label
                ));
            }

            self.lines.push(format!("{}:", arm_labels[idx]));
            if !bound {
                if let Some(name) = &binding_name {
                    let llvm_val_ty = self.llvm_ty(value.ty);
                    let alloca = self.fresh_temp();
                    self.lines
                        .push(format!("  {alloca} = alloca {llvm_val_ty}"));
                    self.lines.push(format!(
                        "  store {llvm_val_ty} {}, {llvm_val_ty}* {alloca}",
                        value.llvm_value
                    ));
                    prev_binding = env.insert(
                        name.clone(),
                        LocalVar {
                            ptr: alloca,
                            ty: value.ty,
                        },
                    );
                }
            }
            let arm_val = match &arm.expr.kind {
                ExprKind::Block(stmts) => self.emit_block_expr(stmts, env, ret_ty),
                _ => self.emit_expr(&arm.expr, env),
            };
            if arm_val.ty == result_ty {
                self.lines.push(format!(
                    "  store {llvm_ty} {}, {llvm_ty}* {result_alloca}",
                    arm_val.llvm_value
                ));
            }
            self.lines.push(format!("  br label %{end_label}"));

            if let Some(name) = &binding_name {
                if let Some(prev) = prev_binding.take() {
                    env.insert(name.clone(), prev);
                } else {
                    env.remove(name);
                }
            }
            if idx + 1 < arms.len() {
                self.lines.push(format!("{}:", fallthrough_labels[idx]));
            }
        }

        self.lines.push(format!("{end_label}:"));
        let out = self.fresh_temp();
        self.lines.push(format!(
            "  {out} = load {llvm_ty}, {llvm_ty}* {result_alloca}"
        ));
        ExprValue::new(result_ty, out)
    }

    fn emit_expr(&mut self, expr: &Expr, env: &mut HashMap<String, LocalVar>) -> ExprValue {
        match &expr.kind {
            ExprKind::Int(value) => ExprValue::new(SimpleTy::I64, value.to_string()),
            ExprKind::Float(value) => ExprValue::new(SimpleTy::F64, value.to_string()),
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
            ExprKind::Lambda { params, body } => {
                // Minimal: emit a unique function for each lambda
                let lambda_name = format!("lambda_{}", self.temp);
                let mut lambda_codegen = Codegen::default();
                let mut lambda_params = Vec::new();
                for param in params.iter() {
                    lambda_params.push(format!("i64 %{}", param));
                }
                let param_decls = lambda_params.join(", ");
                lambda_codegen
                    .lines
                    .push(format!("define i64 @{lambda_name}({param_decls}) {{"));
                lambda_codegen.lines.push("entry:".to_string());
                let mut lambda_env = HashMap::new();
                for param in params {
                    let alloca = lambda_codegen.fresh_temp();
                    lambda_codegen
                        .lines
                        .push(format!("  {alloca} = alloca i64"));
                    lambda_codegen
                        .lines
                        .push(format!("  store i64 %{param}, i64* {alloca}"));
                    lambda_env.insert(
                        param.clone(),
                        LocalVar {
                            ptr: alloca,
                            ty: SimpleTy::I64,
                        },
                    );
                }
                let result = lambda_codegen.emit_expr(body, &mut lambda_env);
                lambda_codegen
                    .lines
                    .push(format!("  ret i64 {}", result.llvm_value));
                lambda_codegen.lines.push("}".to_string());
                self.lines.extend(lambda_codegen.lines);
                // Return a function pointer as an integer (for test purposes)
                ExprValue::new(SimpleTy::I64, format!("@{}", lambda_name))
            }
            ExprKind::ArrayLiteral(items) => {
                if items.is_empty() {
                    self.lines.push("  ; TODO: empty array literal".to_string());
                    return ExprValue::new(SimpleTy::I64Ptr, "null".to_string());
                }
                let mut values = Vec::new();
                for item in items {
                    let value = self.emit_expr(item, env);
                    values.push(value);
                }
                let elem_ty = values.first().map(|v| v.ty).unwrap_or(SimpleTy::Unknown);
                if !values.iter().all(|v| v.ty == elem_ty) {
                    self.lines
                        .push("  ; TODO: non-uniform array literal".to_string());
                    return ExprValue::new(SimpleTy::Unknown, "0".to_string());
                }
                let len = values.len();
                match elem_ty {
                    SimpleTy::I64 => {
                        let arr = self.fresh_temp();
                        self.lines.push(format!("  {arr} = alloca [{len} x i64]"));
                        for (idx, value) in values.into_iter().enumerate() {
                            let elem_ptr = self.fresh_temp();
                            self.lines.push(format!(
                                "  {elem_ptr} = getelementptr inbounds [{len} x i64], [{len} x i64]* {arr}, i64 0, i64 {idx}"
                            ));
                            self.lines
                                .push(format!("  store i64 {}, i64* {elem_ptr}", value.llvm_value));
                        }
                        let data_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {data_ptr} = getelementptr inbounds [{len} x i64], [{len} x i64]* {arr}, i64 0, i64 0"
                        ));
                        ExprValue::new(SimpleTy::I64Ptr, data_ptr)
                    }
                    SimpleTy::F64 => {
                        let arr = self.fresh_temp();
                        self.lines
                            .push(format!("  {arr} = alloca [{len} x double]"));
                        for (idx, value) in values.into_iter().enumerate() {
                            let elem_ptr = self.fresh_temp();
                            self.lines.push(format!(
                                "  {elem_ptr} = getelementptr inbounds [{len} x double], [{len} x double]* {arr}, i64 0, i64 {idx}"
                            ));
                            self.lines.push(format!(
                                "  store double {}, double* {elem_ptr}",
                                value.llvm_value
                            ));
                        }
                        let data_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {data_ptr} = getelementptr inbounds [{len} x double], [{len} x double]* {arr}, i64 0, i64 0"
                        ));
                        ExprValue::new(SimpleTy::F64Ptr, data_ptr)
                    }
                    SimpleTy::Str => {
                        let arr = self.fresh_temp();
                        self.lines.push(format!("  {arr} = alloca [{len} x i8*]"));
                        for (idx, value) in values.into_iter().enumerate() {
                            let elem_ptr = self.fresh_temp();
                            self.lines.push(format!(
                                "  {elem_ptr} = getelementptr inbounds [{len} x i8*], [{len} x i8*]* {arr}, i64 0, i64 {idx}"
                            ));
                            self.lines
                                .push(format!("  store i8* {}, i8** {elem_ptr}", value.llvm_value));
                        }
                        let data_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {data_ptr} = getelementptr inbounds [{len} x i8*], [{len} x i8*]* {arr}, i64 0, i64 0"
                        ));
                        ExprValue::new(SimpleTy::StrPtr, data_ptr)
                    }
                    SimpleTy::Bool => {
                        let arr = self.fresh_temp();
                        self.lines.push(format!("  {arr} = alloca [{len} x i1]"));
                        for (idx, value) in values.into_iter().enumerate() {
                            let elem_ptr = self.fresh_temp();
                            self.lines.push(format!(
                                "  {elem_ptr} = getelementptr inbounds [{len} x i1], [{len} x i1]* {arr}, i64 0, i64 {idx}"
                            ));
                            self.lines
                                .push(format!("  store i1 {}, i1* {elem_ptr}", value.llvm_value));
                        }
                        let data_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {data_ptr} = getelementptr inbounds [{len} x i1], [{len} x i1]* {arr}, i64 0, i64 0"
                        ));
                        ExprValue::new(SimpleTy::BoolPtr, data_ptr)
                    }
                    _ => {
                        self.lines
                            .push("  ; TODO: non-i64/f64/str/bool array literal".to_string());
                        ExprValue::new(SimpleTy::Unknown, "0".to_string())
                    }
                }
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
            ExprKind::Assign { name, expr } => {
                if let Some(var) = env.get(name).cloned() {
                    let value = self.emit_expr(expr, env);
                    let llvm_ty = self.llvm_ty(var.ty);
                    self.lines.push(format!(
                        "  store {llvm_ty} {}, {llvm_ty}* {}",
                        value.llvm_value, var.ptr
                    ));
                    return ExprValue::new(var.ty, value.llvm_value);
                }
                self.lines
                    .push("  ; TODO: assign to unknown var".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
            ExprKind::AssignIndex { expr, index, value } => {
                let base = self.emit_expr(expr, env);
                let idx = self.emit_expr(index, env);
                let rhs = self.emit_expr(value, env);
                if idx.ty != SimpleTy::I64 {
                    self.lines
                        .push("  ; TODO: non-i64 index assign".to_string());
                    return ExprValue::new(SimpleTy::Unknown, "0".to_string());
                }
                match (base.ty, rhs.ty) {
                    (SimpleTy::I64Ptr, SimpleTy::I64) => {
                        let elem_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {elem_ptr} = getelementptr inbounds i64, i64* {}, i64 {}",
                            base.llvm_value, idx.llvm_value
                        ));
                        self.lines
                            .push(format!("  store i64 {}, i64* {elem_ptr}", rhs.llvm_value));
                        ExprValue::new(SimpleTy::I64, rhs.llvm_value)
                    }
                    (SimpleTy::F64Ptr, SimpleTy::F64) => {
                        let elem_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {elem_ptr} = getelementptr inbounds double, double* {}, i64 {}",
                            base.llvm_value, idx.llvm_value
                        ));
                        self.lines.push(format!(
                            "  store double {}, double* {elem_ptr}",
                            rhs.llvm_value
                        ));
                        ExprValue::new(SimpleTy::F64, rhs.llvm_value)
                    }
                    (SimpleTy::StrPtr, SimpleTy::Str) => {
                        let elem_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {elem_ptr} = getelementptr inbounds i8*, i8** {}, i64 {}",
                            base.llvm_value, idx.llvm_value
                        ));
                        self.lines
                            .push(format!("  store i8* {}, i8** {elem_ptr}", rhs.llvm_value));
                        ExprValue::new(SimpleTy::Str, rhs.llvm_value)
                    }
                    (SimpleTy::BoolPtr, SimpleTy::Bool) => {
                        let elem_ptr = self.fresh_temp();
                        self.lines.push(format!(
                            "  {elem_ptr} = getelementptr inbounds i1, i1* {}, i64 {}",
                            base.llvm_value, idx.llvm_value
                        ));
                        self.lines
                            .push(format!("  store i1 {}, i1* {elem_ptr}", rhs.llvm_value));
                        ExprValue::new(SimpleTy::Bool, rhs.llvm_value)
                    }
                    _ => {
                        self.lines
                            .push("  ; TODO: unsupported index assignment".to_string());
                        ExprValue::new(SimpleTy::Unknown, "0".to_string())
                    }
                }
            }
            ExprKind::Index { expr, index } => {
                let base = self.emit_expr(expr, env);
                let idx = self.emit_expr(index, env);
                if base.ty == SimpleTy::I64Ptr && idx.ty == SimpleTy::I64 {
                    let elem_ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {elem_ptr} = getelementptr inbounds i64, i64* {}, i64 {}",
                        base.llvm_value, idx.llvm_value
                    ));
                    let tmp = self.fresh_temp();
                    self.lines
                        .push(format!("  {tmp} = load i64, i64* {elem_ptr}"));
                    return ExprValue::new(SimpleTy::I64, tmp);
                }
                if base.ty == SimpleTy::F64Ptr && idx.ty == SimpleTy::I64 {
                    let elem_ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {elem_ptr} = getelementptr inbounds double, double* {}, i64 {}",
                        base.llvm_value, idx.llvm_value
                    ));
                    let tmp = self.fresh_temp();
                    self.lines
                        .push(format!("  {tmp} = load double, double* {elem_ptr}"));
                    return ExprValue::new(SimpleTy::F64, tmp);
                }
                if base.ty == SimpleTy::StrPtr && idx.ty == SimpleTy::I64 {
                    let elem_ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {elem_ptr} = getelementptr inbounds i8*, i8** {}, i64 {}",
                        base.llvm_value, idx.llvm_value
                    ));
                    let tmp = self.fresh_temp();
                    self.lines
                        .push(format!("  {tmp} = load i8*, i8** {elem_ptr}"));
                    return ExprValue::new(SimpleTy::Str, tmp);
                }
                if base.ty == SimpleTy::BoolPtr && idx.ty == SimpleTy::I64 {
                    let elem_ptr = self.fresh_temp();
                    self.lines.push(format!(
                        "  {elem_ptr} = getelementptr inbounds i1, i1* {}, i64 {}",
                        base.llvm_value, idx.llvm_value
                    ));
                    let tmp = self.fresh_temp();
                    self.lines
                        .push(format!("  {tmp} = load i1, i1* {elem_ptr}"));
                    return ExprValue::new(SimpleTy::Bool, tmp);
                }
                self.lines.push("  ; TODO: unsupported index".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => self.emit_if_expr(
                condition,
                then_branch,
                else_branch.as_ref(),
                env,
                SimpleTy::Void,
            ),
            ExprKind::Match { expr, arms } => self.emit_match_expr(expr, arms, env, SimpleTy::Void),
            ExprKind::Unary { op, expr } => {
                let inner = self.emit_expr(expr, env);
                match (op, inner.ty) {
                    (tupa_parser::UnaryOp::Neg, SimpleTy::I64) => {
                        let tmp = self.fresh_temp();
                        self.lines
                            .push(format!("  {tmp} = sub i64 0, {}", inner.llvm_value));
                        ExprValue::new(SimpleTy::I64, tmp)
                    }
                    (tupa_parser::UnaryOp::Neg, SimpleTy::F64) => {
                        let tmp = self.fresh_temp();
                        self.lines
                            .push(format!("  {tmp} = fneg double {}", inner.llvm_value));
                        ExprValue::new(SimpleTy::F64, tmp)
                    }
                    (tupa_parser::UnaryOp::Not, SimpleTy::Bool) => {
                        let tmp = self.fresh_temp();
                        self.lines
                            .push(format!("  {tmp} = xor i1 {}, 1", inner.llvm_value));
                        ExprValue::new(SimpleTy::Bool, tmp)
                    }
                    _ => {
                        self.lines.push("  ; TODO: unsupported unary".to_string());
                        ExprValue::new(SimpleTy::Unknown, "0".to_string())
                    }
                }
            }
            ExprKind::Call { callee, args } => {
                // Suporte a print
                if let ExprKind::Ident(name) = &callee.kind {
                    if name == "print" {
                        if let Some(arg) = args.first() {
                            let val = self.emit_expr(arg, env);
                            match val.ty {
                                SimpleTy::Str => {
                                    self.declare_puts();
                                    self.lines
                                        .push(format!("  call i32 @puts(i8* {})", val.llvm_value));
                                }
                                SimpleTy::I64 => {
                                    self.declare_printf();
                                    let (fmt, len) = self.intern_format_int();
                                    let fmt_ptr = self.fresh_temp();
                                    self.lines.push(format!("  {fmt_ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {fmt}, i64 0, i64 0"));
                                    self.lines.push(format!(
                                        "  call i32 (i8*, ...) @printf(i8* {fmt_ptr}, i64 {})",
                                        val.llvm_value
                                    ));
                                }
                                SimpleTy::F64 => {
                                    self.declare_printf();
                                    let (fmt, len) = self.intern_format_float();
                                    let fmt_ptr = self.fresh_temp();
                                    self.lines.push(format!("  {fmt_ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {fmt}, i64 0, i64 0"));
                                    self.lines.push(format!(
                                        "  call i32 (i8*, ...) @printf(i8* {fmt_ptr}, double {})",
                                        val.llvm_value
                                    ));
                                }
                                SimpleTy::Bool => {
                                    self.declare_printf();
                                    let (fmt, len) = self.intern_format_int();
                                    let fmt_ptr = self.fresh_temp();
                                    self.lines.push(format!("  {fmt_ptr} = getelementptr inbounds [{len} x i8], [{len} x i8]* {fmt}, i64 0, i64 0"));
                                    let zext = self.fresh_temp();
                                    self.lines.push(format!(
                                        "  {zext} = zext i1 {} to i64",
                                        val.llvm_value
                                    ));
                                    self.lines.push(format!(
                                        "  call i32 (i8*, ...) @printf(i8* {fmt_ptr}, i64 {zext})"
                                    ));
                                }
                                _ => {
                                    self.lines
                                        .push("  ; print de tipo não suportado".to_string());
                                }
                            }
                        }
                        return ExprValue::new(SimpleTy::Void, "0".to_string());
                    }
                }
                // Chamada de função/lambda
                // Se for identificador de função conhecida, use @nome
                let (is_direct_fn, fn_name) = match &callee.kind {
                    ExprKind::Ident(name) => {
                        if self.function_sigs.contains_key(name) {
                            (true, name.clone())
                        } else {
                            (false, String::new())
                        }
                    }
                    _ => (false, String::new()),
                };
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.emit_expr(arg, env));
                }
                let args_llvm = arg_values
                    .iter()
                    .map(|v| match v.ty {
                        SimpleTy::I64 => format!("i64 {}", v.llvm_value),
                        SimpleTy::F64 => format!("double {}", v.llvm_value),
                        SimpleTy::Bool => format!("i1 {}", v.llvm_value),
                        SimpleTy::Str => format!("i8* {}", v.llvm_value),
                        _ => format!("i64 {}", v.llvm_value),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let tmp = self.fresh_temp();
                if is_direct_fn {
                    self.lines
                        .push(format!("  {tmp} = call i64 @{fn_name}({})", args_llvm));
                } else {
                    let callee_val = self.emit_expr(callee, env);
                    self.lines.push(format!(
                        "  {tmp} = call i64 {}({})",
                        callee_val.llvm_value, args_llvm
                    ));
                }
                ExprValue::new(SimpleTy::I64, tmp)
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
                        if matches!(op, tupa_parser::BinaryOp::Pow) {
                            let result_alloca = self.fresh_temp();
                            self.lines.push(format!("  {result_alloca} = alloca i64"));
                            self.lines
                                .push(format!("  store i64 1, i64* {result_alloca}"));

                            let exp_alloca = self.fresh_temp();
                            self.lines.push(format!("  {exp_alloca} = alloca i64"));
                            self.lines.push(format!(
                                "  store i64 {}, i64* {exp_alloca}",
                                right_val.llvm_value
                            ));

                            let head = self.fresh_label("pow.head");
                            let body = self.fresh_label("pow.body");
                            let end = self.fresh_label("pow.end");

                            self.lines.push(format!("  br label %{head}"));
                            self.lines.push(format!("{head}:"));
                            let exp_val = self.fresh_temp();
                            self.lines
                                .push(format!("  {exp_val} = load i64, i64* {exp_alloca}"));
                            let cond = self.fresh_temp();
                            self.lines
                                .push(format!("  {cond} = icmp sgt i64 {exp_val}, 0"));
                            self.lines
                                .push(format!("  br i1 {cond}, label %{body}, label %{end}"));

                            self.lines.push(format!("{body}:"));
                            let res_val = self.fresh_temp();
                            self.lines
                                .push(format!("  {res_val} = load i64, i64* {result_alloca}"));
                            let mul = self.fresh_temp();
                            self.lines.push(format!(
                                "  {mul} = mul i64 {res_val}, {}",
                                left_val.llvm_value
                            ));
                            self.lines
                                .push(format!("  store i64 {mul}, i64* {result_alloca}"));
                            let next_exp = self.fresh_temp();
                            self.lines
                                .push(format!("  {next_exp} = sub i64 {exp_val}, 1"));
                            self.lines
                                .push(format!("  store i64 {next_exp}, i64* {exp_alloca}"));
                            self.lines.push(format!("  br label %{head}"));

                            self.lines.push(format!("{end}:"));
                            let out = self.fresh_temp();
                            self.lines
                                .push(format!("  {out} = load i64, i64* {result_alloca}"));
                            return ExprValue::new(SimpleTy::I64, out);
                        }
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
                    (SimpleTy::F64, SimpleTy::F64) => {
                        let op = match op {
                            tupa_parser::BinaryOp::Add => Some("fadd"),
                            tupa_parser::BinaryOp::Sub => Some("fsub"),
                            tupa_parser::BinaryOp::Mul => Some("fmul"),
                            tupa_parser::BinaryOp::Div => Some("fdiv"),
                            tupa_parser::BinaryOp::Equal => Some("fcmp oeq"),
                            tupa_parser::BinaryOp::NotEqual => Some("fcmp one"),
                            tupa_parser::BinaryOp::Less => Some("fcmp olt"),
                            tupa_parser::BinaryOp::LessEqual => Some("fcmp ole"),
                            tupa_parser::BinaryOp::Greater => Some("fcmp ogt"),
                            tupa_parser::BinaryOp::GreaterEqual => Some("fcmp oge"),
                            _ => None,
                        };
                        if let Some(op) = op {
                            let tmp = self.fresh_temp();
                            if op.starts_with("fcmp") {
                                self.lines.push(format!(
                                    "  {tmp} = {op} double {}, {}",
                                    left_val.llvm_value, right_val.llvm_value
                                ));
                                return ExprValue::new(SimpleTy::Bool, tmp);
                            }
                            self.lines.push(format!(
                                "  {tmp} = {op} double {}, {}",
                                left_val.llvm_value, right_val.llvm_value
                            ));
                            return ExprValue::new(SimpleTy::F64, tmp);
                        }
                    }
                    (SimpleTy::Bool, SimpleTy::Bool) => match op {
                        tupa_parser::BinaryOp::And => {
                            let result_alloca = self.fresh_temp();
                            self.lines.push(format!("  {result_alloca} = alloca i1"));
                            self.lines
                                .push(format!("  store i1 0, i1* {result_alloca}"));
                            let rhs_label = self.fresh_label("and.rhs");
                            let end_label = self.fresh_label("and.end");
                            self.lines.push(format!(
                                "  br i1 {}, label %{rhs_label}, label %{end_label}",
                                left_val.llvm_value
                            ));
                            self.lines.push(format!("{rhs_label}:"));
                            let rhs_val = self.emit_expr(right, env);
                            if rhs_val.ty == SimpleTy::Bool {
                                self.lines.push(format!(
                                    "  store i1 {}, i1* {result_alloca}",
                                    rhs_val.llvm_value
                                ));
                            }
                            self.lines.push(format!("  br label %{end_label}"));
                            self.lines.push(format!("{end_label}:"));
                            let out = self.fresh_temp();
                            self.lines
                                .push(format!("  {out} = load i1, i1* {result_alloca}"));
                            return ExprValue::new(SimpleTy::Bool, out);
                        }
                        tupa_parser::BinaryOp::Or => {
                            let result_alloca = self.fresh_temp();
                            self.lines.push(format!("  {result_alloca} = alloca i1"));
                            self.lines
                                .push(format!("  store i1 1, i1* {result_alloca}"));
                            let rhs_label = self.fresh_label("or.rhs");
                            let end_label = self.fresh_label("or.end");
                            self.lines.push(format!(
                                "  br i1 {}, label %{end_label}, label %{rhs_label}",
                                left_val.llvm_value
                            ));
                            self.lines.push(format!("{rhs_label}:"));
                            let rhs_val = self.emit_expr(right, env);
                            if rhs_val.ty == SimpleTy::Bool {
                                self.lines.push(format!(
                                    "  store i1 {}, i1* {result_alloca}",
                                    rhs_val.llvm_value
                                ));
                            }
                            self.lines.push(format!("  br label %{end_label}"));
                            self.lines.push(format!("{end_label}:"));
                            let out = self.fresh_temp();
                            self.lines
                                .push(format!("  {out} = load i1, i1* {result_alloca}"));
                            return ExprValue::new(SimpleTy::Bool, out);
                        }
                        tupa_parser::BinaryOp::Equal | tupa_parser::BinaryOp::NotEqual => {
                            let op = if matches!(op, tupa_parser::BinaryOp::Equal) {
                                "icmp eq"
                            } else {
                                "icmp ne"
                            };
                            let tmp = self.fresh_temp();
                            self.lines.push(format!(
                                "  {tmp} = {op} i1 {}, {}",
                                left_val.llvm_value, right_val.llvm_value
                            ));
                            return ExprValue::new(SimpleTy::Bool, tmp);
                        }
                        _ => {}
                    },
                    (SimpleTy::Str, SimpleTy::Str) => {
                        match op {
                            tupa_parser::BinaryOp::Add => {
                                return self.emit_string_concat(left_val, right_val);
                            }
                            tupa_parser::BinaryOp::Equal | tupa_parser::BinaryOp::NotEqual => {
                                let op = if matches!(op, tupa_parser::BinaryOp::Equal) {
                                    "icmp eq"
                                } else {
                                    "icmp ne"
                                };
                                self.declare_strcmp();
                                let tmp = self.fresh_temp();
                                self.lines.push(format!(
                                    "  {tmp} = call i32 @strcmp(i8* {}, i8* {})",
                                    left_val.llvm_value, right_val.llvm_value
                                ));
                                let bool_tmp = self.fresh_temp();
                                self.lines.push(format!("  {bool_tmp} = {op} i32 {tmp}, 0"));
                                return ExprValue::new(SimpleTy::Bool, bool_tmp);
                            }
                            _ => {}
                        }
                        self.lines
                            .push("  ; TODO: unsupported string binary".to_string());
                        return ExprValue::new(SimpleTy::Unknown, "0".to_string());
                    }
                    (SimpleTy::Str, SimpleTy::I64 | SimpleTy::F64 | SimpleTy::Bool)
                        if matches!(op, tupa_parser::BinaryOp::Add) =>
                    {
                        if let Some(rhs_str) = self.emit_format_value(right_val) {
                            return self.emit_string_concat(left_val, rhs_str);
                        }
                    }
                    (SimpleTy::I64 | SimpleTy::F64 | SimpleTy::Bool, SimpleTy::Str)
                        if matches!(op, tupa_parser::BinaryOp::Add) =>
                    {
                        if let Some(lhs_str) = self.emit_format_value(left_val) {
                            return self.emit_string_concat(lhs_str, right_val);
                        }
                    }
                    _ => {}
                }
                self.lines.push("  ; TODO: unsupported binary".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
            _ => {
                self.lines
                    .push("  ; TODO: unsupported expression".to_string());
                ExprValue::new(SimpleTy::Unknown, "0".to_string())
            }
        }
    }

    fn map_type(&self, ty: &Type) -> String {
        match ty {
            Type::Ident(name) if name == "i64" => "i64".to_string(),
            Type::Ident(name) if name == "f64" => "double".to_string(),
            Type::Ident(name) if name == "bool" => "i1".to_string(),
            Type::Ident(name) if name == "string" => "i8*".to_string(),
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "i64") => {
                "i64*".to_string()
            }
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "f64") => {
                "double*".to_string()
            }
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "string") => {
                "i8**".to_string()
            }
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "bool") => {
                "i1*".to_string()
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "i64") => {
                "i64*".to_string()
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "f64") => {
                "double*".to_string()
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "string") => {
                "i8**".to_string()
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "bool") => {
                "i1*".to_string()
            }
            _ => "void".to_string(),
        }
    }

    fn simple_ty_from_type(&self, ty: &Type) -> SimpleTy {
        match ty {
            Type::Ident(name) if name == "i64" => SimpleTy::I64,
            Type::Ident(name) if name == "f64" => SimpleTy::F64,
            Type::Ident(name) if name == "bool" => SimpleTy::Bool,
            Type::Ident(name) if name == "string" => SimpleTy::Str,
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "i64") => {
                SimpleTy::I64Ptr
            }
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "f64") => {
                SimpleTy::F64Ptr
            }
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "string") => {
                SimpleTy::StrPtr
            }
            Type::Array { elem, .. } if matches!(**elem, Type::Ident(ref n) if n == "bool") => {
                SimpleTy::BoolPtr
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "i64") => {
                SimpleTy::I64Ptr
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "f64") => {
                SimpleTy::F64Ptr
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "string") => {
                SimpleTy::StrPtr
            }
            Type::Slice { elem } if matches!(**elem, Type::Ident(ref n) if n == "bool") => {
                SimpleTy::BoolPtr
            }
            _ => SimpleTy::Unknown,
        }
    }

    fn llvm_ty(&self, ty: SimpleTy) -> &'static str {
        match ty {
            SimpleTy::I64 => "i64",
            SimpleTy::F64 => "double",
            SimpleTy::Bool => "i1",
            SimpleTy::I64Ptr => "i64*",
            SimpleTy::F64Ptr => "double*",
            SimpleTy::Str => "i8*",
            SimpleTy::StrPtr => "i8**",
            SimpleTy::BoolPtr => "i1*",
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
        let literal =
            format!("{name} = private unnamed_addr constant [{len} x i8] c\"{escaped}\\00\"");
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

    fn intern_format_float(&mut self) -> (String, usize) {
        if self.fmt_float_emitted {
            return ("@.fmt_float".to_string(), 4);
        }
        self.fmt_float_emitted = true;
        let literal = "@.fmt_float = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\"";
        self.globals.push(literal.to_string());
        ("@.fmt_float".to_string(), 4)
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

    fn declare_strcmp(&mut self) {
        if !self.strcmp_declared {
            self.globals
                .push("declare i32 @strcmp(i8*, i8*)".to_string());
            self.strcmp_declared = true;
        }
    }

    fn declare_strlen(&mut self) {
        if !self.strlen_declared {
            self.globals.push("declare i64 @strlen(i8*)".to_string());
            self.strlen_declared = true;
        }
    }

    fn declare_malloc(&mut self) {
        if !self.malloc_declared {
            self.globals.push("declare i8* @malloc(i64)".to_string());
            self.malloc_declared = true;
        }
    }

    fn declare_strcpy(&mut self) {
        if !self.strcpy_declared {
            self.globals
                .push("declare i8* @strcpy(i8*, i8*)".to_string());
            self.strcpy_declared = true;
        }
    }

    fn declare_strcat(&mut self) {
        if !self.strcat_declared {
            self.globals
                .push("declare i8* @strcat(i8*, i8*)".to_string());
            self.strcat_declared = true;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_lexer::Span;
    use tupa_parser::{Expr, ExprKind, Function, Item, Program, Stmt, Type};

    #[test]
    fn test_empty_function_codegen() {
        let program = Program {
            items: vec![Item::Function(Function {
                name: "main".to_string(),
                params: vec![],
                return_type: None,
                body: vec![],
            })],
        };
        let code = generate_stub(&program);
        assert!(code.contains("define void @main()"));
        assert!(code.contains("entry:"));
        assert!(code.contains("ret void"));
    }

    #[test]
    fn test_function_with_return_codegen() {
        let program = Program {
            items: vec![Item::Function(Function {
                name: "test".to_string(),
                params: vec![],
                return_type: Some(Type::Ident("i64".to_string())),
                body: vec![Stmt::Return(Some(Expr {
                    kind: ExprKind::Int(42),
                    span: Span { start: 0, end: 0 },
                }))],
            })],
        };
        let code = generate_stub(&program);
        assert!(code.contains("define i64 @test()"));
        assert!(code.contains("ret i64 42"));
    }

    #[test]
    fn test_string_literal_codegen() {
        let program = Program {
            items: vec![Item::Function(Function {
                name: "test".to_string(),
                params: vec![],
                return_type: None,
                body: vec![Stmt::Expr(Expr {
                    kind: ExprKind::Str("hello".to_string()),
                    span: Span { start: 0, end: 0 },
                })],
            })],
        };
        let code = generate_stub(&program);
        assert!(code.contains("@.str"));
        assert!(code.contains("hello"));
    }
}
