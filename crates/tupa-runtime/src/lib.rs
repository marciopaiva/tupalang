use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use thiserror::Error;
use tupa_codegen::execution_plan::ExecutionPlan;

type StepFn = fn(Value) -> Result<Value, String>;

static REGISTRY: Lazy<Mutex<HashMap<String, StepFn>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static PRNG_STATE: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0x9E3779B97F4A7C15));

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("step function not found: {0}")]
    NotFound(String),
    #[error("step function failed: {0}")]
    StepFailed(String),
    #[error("invalid input/output: {0}")]
    Invalid(String),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub fn register_step(name: &str, func: StepFn) {
    REGISTRY.lock().unwrap().insert(name.to_string(), func);
}

pub fn configure_seed(seed: Option<u64>) {
    let mut st = PRNG_STATE.lock().unwrap();
    *st = seed.unwrap_or(0x9E3779B97F4A7C15);
}

fn rand_u64() -> u64 {
    let mut s = PRNG_STATE.lock().unwrap();
    // xorshift64*
    let mut x = *s;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *s = x;
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

pub fn rand_f64() -> f64 {
    (rand_u64() as f64) / (u64::MAX as f64)
}

pub fn register_default_examples() {
    // Identity
    register_step("api_with_pipeline::step_noop", Ok);
    // FraudDetection demo
    register_step("fraud_complete::step_enrich", |mut v| {
        if let Some(obj) = v.as_object_mut() {
            obj.entry("enriched").or_insert(Value::Bool(true));
        }
        Ok(v)
    });
    register_step("fraud_complete::step_score", |mut v| {
        if let Some(obj) = v.as_object_mut() {
            obj.insert("score".to_string(), Value::from(42));
        }
        Ok(v)
    });
    register_step("fraud_complete::step_decide", |mut v| {
        let approved = v
            .get("score")
            .and_then(|s| s.as_i64())
            .map(|s| s > 10)
            .unwrap_or(false);
        if let Some(obj) = v.as_object_mut() {
            obj.insert("approved".to_string(), Value::from(approved));
        }
        Ok(v)
    });
    register_step("fraud_complete::compute_fpr", |_v| Ok(Value::from(0.009)));
    register_step("fraud_complete::compute_fnr", |_v| Ok(Value::from(0.049)));
    // Credit decision demo
    register_step("credit_decision::step_enrich", |mut v| {
        if let Some(obj) = v.as_object_mut() {
            obj.entry("income").or_insert(Value::from(5000));
        }
        Ok(v)
    });
    register_step("credit_decision::step_score", |mut v| {
        if let Some(obj) = v.as_object_mut() {
            obj.insert("score".to_string(), Value::from(720));
        }
        Ok(v)
    });
    register_step("credit_decision::step_decide", |mut v| {
        let approved = v
            .get("score")
            .and_then(|s| s.as_i64())
            .map(|s| s >= 700)
            .unwrap_or(false);
        if let Some(obj) = v.as_object_mut() {
            obj.insert("approved".to_string(), Value::from(approved));
        }
        Ok(v)
    });
}

fn call_step_function(function_ref: &str, state: Value) -> RuntimeResult<Value> {
    let reg = REGISTRY.lock().unwrap();
    let f = reg
        .get(function_ref)
        .ok_or_else(|| RuntimeError::NotFound(function_ref.to_string()))?;
    f(state).map_err(RuntimeError::StepFailed)
}

pub fn run_pipeline(plan: &ExecutionPlan, input: Value) -> RuntimeResult<Value> {
    configure_seed(plan.seed);
    let mut state = input;
    for step in &plan.steps {
        state = call_step_function(&step.function_ref, state)?;
    }
    Ok(state)
}

pub fn evaluate_constraints(plan: &ExecutionPlan, output: &Value) -> Value {
    // Evaluate metric plans
    let mut computed: std::collections::HashMap<String, f64> = plan.metrics.clone();
    for mp in &plan.metric_plans {
        let v = call_step_function(&mp.function_ref, mp.args.clone()).ok();
        if let Some(Value::Number(n)) = v {
            if let Some(f) = n.as_f64() {
                computed.insert(mp.name.clone(), f);
            }
        } else if let Some(Value::Bool(b)) = v {
            computed.insert(mp.name.clone(), if b { 1.0 } else { 0.0 });
        }
    }
    let mut items = Vec::new();
    for c in &plan.constraints {
        let value_opt = computed
            .get(&c.metric)
            .cloned()
            .or_else(|| match output.get(&c.metric) {
                Some(Value::Number(n)) => n.as_f64(),
                _ => None,
            });
        let (status, pass) = match value_opt {
            Some(v) => {
                let ok = match c.comparator.as_str() {
                    "lt" => v < c.threshold,
                    "le" => v <= c.threshold,
                    "eq" => (v - c.threshold).abs() < f64::EPSILON,
                    "ge" => v >= c.threshold,
                    "gt" => v > c.threshold,
                    _ => false,
                };
                (if ok { "pass" } else { "fail" }, ok)
            }
            None => ("unknown", false),
        };
        items.push(serde_json::json!({
            "metric": c.metric,
            "value": value_opt,
            "comparator": c.comparator,
            "threshold": c.threshold,
            "status": status,
            "pass": pass
        }));
    }
    serde_json::json!({ "constraints": items, "metrics": computed })
}

use tupa_codegen::execution_plan::TypeSchema;

pub fn validate_input(value: &Value, schema: &TypeSchema) -> RuntimeResult<()> {
    match schema.kind.as_str() {
        "i64" => {
            if value.as_i64().is_none() {
                return Err(RuntimeError::Invalid("expected i64".into()));
            }
        }
        "f64" => {
            if value.as_f64().is_none() {
                return Err(RuntimeError::Invalid("expected f64".into()));
            }
        }
        "bool" => {
            if value.as_bool().is_none() {
                return Err(RuntimeError::Invalid("expected bool".into()));
            }
        }
        "string" => {
            if !value.is_string() {
                return Err(RuntimeError::Invalid("expected string".into()));
            }
        }
        "array" => {
            let arr = value
                .as_array()
                .ok_or_else(|| RuntimeError::Invalid("expected array".into()))?;
            if let Some(len) = schema.len {
                if arr.len() as i64 != len {
                    return Err(RuntimeError::Invalid(format!(
                        "array length mismatch: expected {len}, got {}",
                        arr.len()
                    )));
                }
            }
            if let Some(elem) = &schema.elem {
                for v in arr {
                    validate_input(v, elem)?;
                }
            }
        }
        "slice" => {
            let arr = value
                .as_array()
                .ok_or_else(|| RuntimeError::Invalid("expected slice (array)".into()))?;
            if let Some(elem) = &schema.elem {
                for v in arr {
                    validate_input(v, elem)?;
                }
            }
        }
        "ident" => {
            // Accept any JSON object for domain types
            if !value.is_object() {
                return Err(RuntimeError::Invalid(
                    "expected object for ident type".into(),
                ));
            }
        }
        _ => {}
    }
    Ok(())
}
