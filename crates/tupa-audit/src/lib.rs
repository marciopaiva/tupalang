use serde_json::{Map, Value};
use sha3::{Digest, Sha3_256};
use tupa_parser::{
    BinaryOp, ElseBranch, EnumVariant, Expr, ExprKind, FieldAccess, Function, Item, MatchArm,
    Param, Pattern, Program, Stmt, Type, UnaryOp,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash(String);

impl Hash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn compiler_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn hash_execution(program: &Program, inputs: &[Value]) -> Hash {
    let payload = object(vec![
        ("version", Value::String(compiler_version().to_string())),
        ("ast", program_to_value(program)),
        ("inputs", Value::Array(inputs.to_vec())),
    ]);
    Hash(hash_value(&payload))
}

pub fn hash_ast(program: &Program) -> Hash {
    let payload = object(vec![
        ("version", Value::String(compiler_version().to_string())),
        ("ast", program_to_value(program)),
    ]);
    Hash(hash_value(&payload))
}

fn hash_value(value: &Value) -> String {
    let canonical = canonical_json(value);
    let digest = Sha3_256::digest(canonical.as_bytes());
    hex_bytes(&digest)
}

fn canonical_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(val) => {
            if *val {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Value::Number(num) => num.to_string(),
        Value::String(text) => serde_json::to_string(text).unwrap_or_else(|_| "\"\"".to_string()),
        Value::Array(items) => {
            let rendered = items.iter().map(canonical_json).collect::<Vec<_>>();
            format!("[{}]", rendered.join(","))
        }
        Value::Object(map) => {
            let mut keys = map.keys().collect::<Vec<_>>();
            keys.sort();
            let mut parts = Vec::with_capacity(keys.len());
            for key in keys {
                let value = map.get(key).unwrap_or(&Value::Null);
                let rendered_key =
                    serde_json::to_string(key).unwrap_or_else(|_| "\"\"".to_string());
                let rendered_value = canonical_json(value);
                parts.push(format!("{rendered_key}:{rendered_value}"));
            }
            format!("{{{}}}", parts.join(","))
        }
    }
}

fn hex_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn object(pairs: Vec<(&str, Value)>) -> Value {
    let mut map = Map::new();
    for (key, value) in pairs {
        map.insert(key.to_string(), value);
    }
    Value::Object(map)
}

fn program_to_value(program: &Program) -> Value {
    Value::Array(program.items.iter().map(item_to_value).collect())
}

fn item_to_value(item: &Item) -> Value {
    match item {
        Item::Function(func) => object(vec![
            ("kind", Value::String("Function".to_string())),
            ("name", Value::String(func.name.clone())),
            ("params", params_to_value(&func.params)),
            (
                "return_type",
                option_type_to_value(func.return_type.as_ref()),
            ),
            ("body", block_to_value(&func.body)),
        ]),
        Item::Enum(def) => object(vec![
            ("kind", Value::String("Enum".to_string())),
            ("name", Value::String(def.name.clone())),
            (
                "generics",
                Value::Array(
                    def.generics
                        .iter()
                        .map(|g| Value::String(g.clone()))
                        .collect(),
                ),
            ),
            ("variants", variants_to_value(&def.variants)),
        ]),
        Item::Trait(def) => object(vec![
            ("kind", Value::String("Trait".to_string())),
            ("name", Value::String(def.name.clone())),
            (
                "methods",
                Value::Array(def.methods.iter().map(function_to_value).collect()),
            ),
        ]),
    }
}

fn function_to_value(func: &Function) -> Value {
    object(vec![
        ("name", Value::String(func.name.clone())),
        ("params", params_to_value(&func.params)),
        (
            "return_type",
            option_type_to_value(func.return_type.as_ref()),
        ),
        ("body", block_to_value(&func.body)),
    ])
}

fn params_to_value(params: &[Param]) -> Value {
    Value::Array(
        params
            .iter()
            .map(|param| {
                object(vec![
                    ("name", Value::String(param.name.clone())),
                    ("ty", type_to_value(&param.ty)),
                ])
            })
            .collect(),
    )
}

fn variants_to_value(variants: &[EnumVariant]) -> Value {
    Value::Array(
        variants
            .iter()
            .map(|variant| {
                object(vec![
                    ("name", Value::String(variant.name.clone())),
                    (
                        "args",
                        Value::Array(variant.args.iter().map(type_to_value).collect()),
                    ),
                ])
            })
            .collect(),
    )
}

fn option_type_to_value(ty: Option<&Type>) -> Value {
    ty.map(type_to_value).unwrap_or(Value::Null)
}

fn type_to_value(ty: &Type) -> Value {
    match ty {
        Type::Ident(name) => object(vec![
            ("kind", Value::String("Ident".to_string())),
            ("name", Value::String(name.clone())),
        ]),
        Type::Generic { name, args } => object(vec![
            ("kind", Value::String("Generic".to_string())),
            ("name", Value::String(name.clone())),
            (
                "args",
                Value::Array(args.iter().map(type_to_value).collect()),
            ),
        ]),
        Type::Tuple(items) => object(vec![
            ("kind", Value::String("Tuple".to_string())),
            (
                "items",
                Value::Array(items.iter().map(type_to_value).collect()),
            ),
        ]),
        Type::Safe { base, constraints } => object(vec![
            ("kind", Value::String("Safe".to_string())),
            ("base", type_to_value(base)),
            (
                "constraints",
                Value::Array(
                    constraints
                        .iter()
                        .map(|c| Value::String(c.clone()))
                        .collect(),
                ),
            ),
        ]),
        Type::Array { elem, len } => object(vec![
            ("kind", Value::String("Array".to_string())),
            ("elem", type_to_value(elem)),
            ("len", Value::Number((*len).into())),
        ]),
        Type::Slice { elem } => object(vec![
            ("kind", Value::String("Slice".to_string())),
            ("elem", type_to_value(elem)),
        ]),
        Type::Func { params, ret } => object(vec![
            ("kind", Value::String("Func".to_string())),
            (
                "params",
                Value::Array(params.iter().map(type_to_value).collect()),
            ),
            ("ret", type_to_value(ret)),
        ]),
    }
}

fn block_to_value(block: &[Stmt]) -> Value {
    Value::Array(block.iter().map(stmt_to_value).collect())
}

#[cfg(test)]
mod tests {
    use super::{hash_ast, hash_execution};
    use serde_json::json;
    use tupa_parser::parse_program;

    #[test]
    fn audit_hash_is_deterministic_for_same_inputs() {
        let program = parse_program("fn main() { let x = 1; }").unwrap();
        let inputs = vec![json!({"b": 2, "a": 1}), json!(3)];
        let first = hash_execution(&program, &inputs);
        let second = hash_execution(&program, &inputs);
        assert_eq!(first, second);
    }

    #[test]
    fn ast_hash_is_stable_across_parses() {
        let program_a = parse_program("fn main() { let x = 1; }").unwrap();
        let program_b = parse_program("fn main() { let x = 1; }").unwrap();
        assert_eq!(hash_ast(&program_a), hash_ast(&program_b));
    }

    #[test]
    fn hash_changes_when_inputs_change() {
        let program = parse_program("fn main() { let x = 1; }").unwrap();
        let first = hash_execution(&program, &[json!(1)]);
        let second = hash_execution(&program, &[json!(2)]);
        assert_ne!(first, second);
    }
}

fn stmt_to_value(stmt: &Stmt) -> Value {
    match stmt {
        Stmt::Let { name, ty, expr } => object(vec![
            ("kind", Value::String("Let".to_string())),
            ("name", Value::String(name.clone())),
            ("ty", option_type_to_value(ty.as_ref())),
            ("expr", expr_to_value(expr)),
        ]),
        Stmt::Return(expr) => object(vec![
            ("kind", Value::String("Return".to_string())),
            (
                "expr",
                expr.as_ref().map(expr_to_value).unwrap_or(Value::Null),
            ),
        ]),
        Stmt::While { condition, body } => object(vec![
            ("kind", Value::String("While".to_string())),
            ("condition", expr_to_value(condition)),
            ("body", block_to_value(body)),
        ]),
        Stmt::For { name, iter, body } => object(vec![
            ("kind", Value::String("For".to_string())),
            ("name", Value::String(name.clone())),
            ("iter", expr_to_value(iter)),
            ("body", block_to_value(body)),
        ]),
        Stmt::Break => object(vec![("kind", Value::String("Break".to_string()))]),
        Stmt::Continue => object(vec![("kind", Value::String("Continue".to_string()))]),
        Stmt::Expr(expr) => object(vec![
            ("kind", Value::String("Expr".to_string())),
            ("expr", expr_to_value(expr)),
        ]),
        Stmt::Lambda { params, body } => object(vec![
            ("kind", Value::String("Lambda".to_string())),
            (
                "params",
                Value::Array(params.iter().map(|p| Value::String(p.clone())).collect()),
            ),
            ("body", expr_to_value(body)),
        ]),
    }
}

fn expr_to_value(expr: &Expr) -> Value {
    match &expr.kind {
        ExprKind::Lambda { params, body } => object(vec![
            ("kind", Value::String("Lambda".to_string())),
            (
                "params",
                Value::Array(params.iter().map(|p| Value::String(p.clone())).collect()),
            ),
            ("body", expr_to_value(body)),
        ]),
        ExprKind::Int(value) => object(vec![
            ("kind", Value::String("Int".to_string())),
            ("value", Value::Number((*value).into())),
        ]),
        ExprKind::Float(value) => object(vec![
            ("kind", Value::String("Float".to_string())),
            ("value", number_from_f64(*value)),
        ]),
        ExprKind::Str(value) => object(vec![
            ("kind", Value::String("Str".to_string())),
            ("value", Value::String(value.clone())),
        ]),
        ExprKind::Bool(value) => object(vec![
            ("kind", Value::String("Bool".to_string())),
            ("value", Value::Bool(*value)),
        ]),
        ExprKind::Null => object(vec![("kind", Value::String("Null".to_string()))]),
        ExprKind::Ident(name) => object(vec![
            ("kind", Value::String("Ident".to_string())),
            ("name", Value::String(name.clone())),
        ]),
        ExprKind::Tuple(items) => object(vec![
            ("kind", Value::String("Tuple".to_string())),
            (
                "items",
                Value::Array(items.iter().map(expr_to_value).collect()),
            ),
        ]),
        ExprKind::Assign { name, expr } => object(vec![
            ("kind", Value::String("Assign".to_string())),
            ("name", Value::String(name.clone())),
            ("expr", expr_to_value(expr)),
        ]),
        ExprKind::AssignIndex { expr, index, value } => object(vec![
            ("kind", Value::String("AssignIndex".to_string())),
            ("expr", expr_to_value(expr)),
            ("index", expr_to_value(index)),
            ("value", expr_to_value(value)),
        ]),
        ExprKind::ArrayLiteral(items) => object(vec![
            ("kind", Value::String("ArrayLiteral".to_string())),
            (
                "items",
                Value::Array(items.iter().map(expr_to_value).collect()),
            ),
        ]),
        ExprKind::Call { callee, args } => object(vec![
            ("kind", Value::String("Call".to_string())),
            ("callee", expr_to_value(callee)),
            (
                "args",
                Value::Array(args.iter().map(expr_to_value).collect()),
            ),
        ]),
        ExprKind::Field { expr, field } => object(vec![
            ("kind", Value::String("Field".to_string())),
            ("expr", expr_to_value(expr)),
            ("field", field_to_value(field)),
        ]),
        ExprKind::Index { expr, index } => object(vec![
            ("kind", Value::String("Index".to_string())),
            ("expr", expr_to_value(expr)),
            ("index", expr_to_value(index)),
        ]),
        ExprKind::Await(expr) => object(vec![
            ("kind", Value::String("Await".to_string())),
            ("expr", expr_to_value(expr)),
        ]),
        ExprKind::Block(block) => object(vec![
            ("kind", Value::String("Block".to_string())),
            ("body", block_to_value(block)),
        ]),
        ExprKind::If {
            condition,
            then_branch,
            else_branch,
        } => object(vec![
            ("kind", Value::String("If".to_string())),
            ("condition", expr_to_value(condition)),
            ("then", block_to_value(then_branch)),
            ("else", else_branch_to_value(else_branch.as_ref())),
        ]),
        ExprKind::Match { expr, arms } => object(vec![
            ("kind", Value::String("Match".to_string())),
            ("expr", expr_to_value(expr)),
            ("arms", match_arms_to_value(arms)),
        ]),
        ExprKind::Unary { op, expr } => object(vec![
            ("kind", Value::String("Unary".to_string())),
            ("op", Value::String(unary_op_to_string(op))),
            ("expr", expr_to_value(expr)),
        ]),
        ExprKind::Binary { op, left, right } => object(vec![
            ("kind", Value::String("Binary".to_string())),
            ("op", Value::String(binary_op_to_string(op))),
            ("left", expr_to_value(left)),
            ("right", expr_to_value(right)),
        ]),
    }
}

fn else_branch_to_value(branch: Option<&ElseBranch>) -> Value {
    match branch {
        None => Value::Null,
        Some(ElseBranch::Block(block)) => object(vec![
            ("kind", Value::String("Block".to_string())),
            ("body", block_to_value(block)),
        ]),
        Some(ElseBranch::If(expr)) => object(vec![
            ("kind", Value::String("If".to_string())),
            ("expr", expr_to_value(expr)),
        ]),
    }
}

fn match_arms_to_value(arms: &[MatchArm]) -> Value {
    Value::Array(
        arms.iter()
            .map(|arm| {
                object(vec![
                    ("pattern", pattern_to_value(&arm.pattern)),
                    (
                        "guard",
                        arm.guard.as_ref().map(expr_to_value).unwrap_or(Value::Null),
                    ),
                    ("expr", expr_to_value(&arm.expr)),
                ])
            })
            .collect(),
    )
}

fn pattern_to_value(pattern: &Pattern) -> Value {
    match pattern {
        Pattern::Wildcard => object(vec![("kind", Value::String("Wildcard".to_string()))]),
        Pattern::Int(value) => object(vec![
            ("kind", Value::String("Int".to_string())),
            ("value", Value::Number((*value).into())),
        ]),
        Pattern::Str(value) => object(vec![
            ("kind", Value::String("Str".to_string())),
            ("value", Value::String(value.clone())),
        ]),
        Pattern::Bool(value) => object(vec![
            ("kind", Value::String("Bool".to_string())),
            ("value", Value::Bool(*value)),
        ]),
        Pattern::Ident(name) => object(vec![
            ("kind", Value::String("Ident".to_string())),
            ("name", Value::String(name.clone())),
        ]),
        Pattern::Tuple(items) => object(vec![
            ("kind", Value::String("Tuple".to_string())),
            (
                "items",
                Value::Array(items.iter().map(pattern_to_value).collect()),
            ),
        ]),
        Pattern::Constructor { name, args } => object(vec![
            ("kind", Value::String("Constructor".to_string())),
            ("name", Value::String(name.clone())),
            (
                "args",
                Value::Array(args.iter().map(pattern_to_value).collect()),
            ),
        ]),
    }
}

fn field_to_value(field: &FieldAccess) -> Value {
    match field {
        FieldAccess::Ident(name) => object(vec![
            ("kind", Value::String("Ident".to_string())),
            ("name", Value::String(name.clone())),
        ]),
        FieldAccess::Index(value) => object(vec![
            ("kind", Value::String("Index".to_string())),
            ("value", Value::Number((*value).into())),
        ]),
    }
}

fn unary_op_to_string(op: &UnaryOp) -> String {
    match op {
        UnaryOp::Not => "Not".to_string(),
        UnaryOp::Neg => "Neg".to_string(),
    }
}

fn binary_op_to_string(op: &BinaryOp) -> String {
    match op {
        BinaryOp::Range => "Range".to_string(),
        BinaryOp::Or => "Or".to_string(),
        BinaryOp::And => "And".to_string(),
        BinaryOp::Equal => "Equal".to_string(),
        BinaryOp::NotEqual => "NotEqual".to_string(),
        BinaryOp::Less => "Less".to_string(),
        BinaryOp::LessEqual => "LessEqual".to_string(),
        BinaryOp::Greater => "Greater".to_string(),
        BinaryOp::GreaterEqual => "GreaterEqual".to_string(),
        BinaryOp::Add => "Add".to_string(),
        BinaryOp::Sub => "Sub".to_string(),
        BinaryOp::Mul => "Mul".to_string(),
        BinaryOp::Div => "Div".to_string(),
        BinaryOp::Pow => "Pow".to_string(),
    }
}

fn number_from_f64(value: f64) -> Value {
    serde_json::Number::from_f64(value)
        .map(Value::Number)
        .unwrap_or_else(|| Value::String(value.to_string()))
}
