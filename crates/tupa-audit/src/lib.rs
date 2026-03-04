use serde_json::{Map, Value};
use sha3::{Digest, Sha3_256};
use tupa_parser::{
    Item, Program,
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
        Item::Function(f) => object(vec![
            ("kind", Value::String("Function".to_string())),
            ("name", Value::String(f.name.clone())),
        ]),
        Item::Pipeline(p) => object(vec![
            ("kind", Value::String("Pipeline".to_string())),
            ("name", Value::String(p.name.clone())),
        ]),
        Item::Enum(e) => object(vec![
            ("kind", Value::String("Enum".to_string())),
            ("name", Value::String(e.name.clone())),
        ]),
        Item::Trait(t) => object(vec![
            ("kind", Value::String("Trait".to_string())),
            ("name", Value::String(t.name.clone())),
        ]),
    }
}
