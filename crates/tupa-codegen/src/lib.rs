use std::collections::HashMap;

use tupa_parser::{Expr, ExprKind, Function, Item, Program, Stmt, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SimpleTy {
    I64,
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
    fmt_int_emitted: bool,
    printf_declared: bool,
    puts_declared: bool,
}

#[derive(Debug, Clone)]
struct LocalVar {
    ptr: String,
    ty: SimpleTy,
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
            Some(Type::Ident(name)) if name == "string" => SimpleTy::Str,
            Some(_) => SimpleTy::Unknown,
            None => SimpleTy::Void,
        };
        let llvm_ret = match ret_ty {
            SimpleTy::I64 => "i64",
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

        for stmt in &func.body {
            self.emit_stmt(stmt, &mut env, ret_ty);
        }

        if ret_ty == SimpleTy::I64 {
            self.lines.push("  ret i64 0".to_string());
        } else if ret_ty == SimpleTy::Str {
            self.lines.push("  ret i8* null".to_string());
        } else {
            self.lines.push("  ret void".to_string());
        }
        self.lines.push("}".to_string());
        self.lines.push(String::new());
    }

    fn emit_stmt(&mut self, stmt: &Stmt, env: &mut HashMap<String, LocalVar>, ret_ty: SimpleTy) {
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
            }
            Stmt::Expr(expr) => {
                self.emit_expr(expr, env);
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.emit_expr(expr, env);
                    let llvm_ty = self.llvm_ty(value.ty);
                    self.lines.push(format!("  ret {llvm_ty} {}", value.llvm_value));
                } else {
                    match ret_ty {
                        SimpleTy::I64 => self.lines.push("  ret i64 0".to_string()),
                        SimpleTy::Str => self.lines.push("  ret i8* null".to_string()),
                        _ => self.lines.push("  ret void".to_string()),
                    }
                }
            }
            _ => self.lines.push("  ; TODO: unsupported statement".to_string()),
        }
    }

    fn emit_expr(&mut self, expr: &Expr, env: &mut HashMap<String, LocalVar>) -> ExprValue {
        match &expr.kind {
            ExprKind::Int(value) => ExprValue::new(SimpleTy::I64, value.to_string()),
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
                            _ => None,
                        };
                        if let Some(op) = op {
                            let tmp = self.fresh_temp();
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
            Type::Ident(name) if name == "string" => "i8*".to_string(),
            _ => "void".to_string(),
        }
    }

    fn llvm_ty(&self, ty: SimpleTy) -> &'static str {
        match ty {
            SimpleTy::I64 => "i64",
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
