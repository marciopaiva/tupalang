use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tupa_parser::{Comparator, Expr, ExprKind, PipelineDecl, Stmt, Type};
use tupa_typecheck::analyze_effects;

#[derive(Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub name: String,
    pub version: String,
    pub seed: Option<u64>,
    pub input_schema: TypeSchema,
    pub steps: Vec<StepPlan>,
    pub constraints: Vec<ConstraintPlan>,
    pub metrics: HashMap<String, f64>,
    pub metric_plans: Vec<MetricPlan>,
}

#[derive(Serialize, Deserialize)]
pub struct StepPlan {
    pub name: String,
    pub function_ref: String,
    pub effects: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConstraintPlan {
    pub metric: String,
    pub comparator: String,
    pub threshold: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TypeSchema {
    pub kind: String,
    pub elem: Option<Box<TypeSchema>>,
    pub len: Option<i64>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MetricPlan {
    pub name: String,
    pub function_ref: String,
    pub args: serde_json::Value,
}

pub fn type_to_schema(ty: &Type) -> TypeSchema {
    match ty {
        Type::Array { elem, len } => TypeSchema {
            kind: "array".into(),
            elem: Some(Box::new(type_to_schema(elem))),
            len: Some(*len),
            name: None,
        },
        Type::Slice { elem } => TypeSchema {
            kind: "slice".into(),
            elem: Some(Box::new(type_to_schema(elem))),
            len: None,
            name: None,
        },
        Type::Safe { base, .. } => type_to_schema(base),
        Type::Ident(name) => match name.as_str() {
            "i64" => TypeSchema {
                kind: "i64".into(),
                elem: None,
                len: None,
                name: None,
            },
            "f64" => TypeSchema {
                kind: "f64".into(),
                elem: None,
                len: None,
                name: None,
            },
            "bool" => TypeSchema {
                kind: "bool".into(),
                elem: None,
                len: None,
                name: None,
            },
            "string" => TypeSchema {
                kind: "string".into(),
                elem: None,
                len: None,
                name: None,
            },
            _ => TypeSchema {
                kind: "ident".into(),
                elem: None,
                len: None,
                name: Some(name.clone()),
            },
        },
        _ => TypeSchema {
            kind: "unknown".into(),
            elem: None,
            len: None,
            name: None,
        },
    }
}

fn constraint_to_plan(c: &tupa_parser::Constraint) -> ConstraintPlan {
    ConstraintPlan {
        metric: c.metric.clone(),
        comparator: match c.comparator {
            Comparator::Lt => "lt".into(),
            Comparator::Le => "le".into(),
            Comparator::Eq => "eq".into(),
            Comparator::Ge => "ge".into(),
            Comparator::Gt => "gt".into(),
        },
        threshold: c.threshold,
    }
}

fn extract_metrics(pipeline: &PipelineDecl) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    if let Some(block) = &pipeline.validation {
        for stmt in block {
            if let Stmt::Let { name, expr, .. } = stmt {
                match &expr.kind {
                    ExprKind::Int(n) => {
                        map.insert(name.clone(), *n as f64);
                    }
                    ExprKind::Float(f) => {
                        map.insert(name.clone(), *f);
                    }
                    _ => {}
                }
            }
        }
    }
    map
}

fn expr_to_json(expr: &Expr) -> Option<serde_json::Value> {
    match &expr.kind {
        ExprKind::Int(n) => Some(serde_json::json!(*n)),
        ExprKind::Float(f) => Some(serde_json::json!(*f)),
        ExprKind::Bool(b) => Some(serde_json::json!(*b)),
        ExprKind::ArrayLiteral(items) => {
            let mut arr = Vec::new();
            for it in items {
                if let Some(v) = expr_to_json(it) {
                    arr.push(v);
                } else {
                    return None;
                }
            }
            Some(serde_json::Value::Array(arr))
        }
        _ => None,
    }
}

fn extract_metric_plans(module_name: &str, pipeline: &PipelineDecl) -> Vec<MetricPlan> {
    let mut list = Vec::new();
    if let Some(block) = &pipeline.validation {
        for stmt in block {
            if let Stmt::Let { name, expr, .. } = stmt {
                if let ExprKind::Call { callee, args } = &expr.kind {
                    if let ExprKind::Ident(func) = &callee.kind {
                        // build args JSON as array of supported literals
                        let json_args = if args.len() == 1 {
                            expr_to_json(&args[0]).unwrap_or(serde_json::Value::Null)
                        } else {
                            let mut arr = Vec::new();
                            for a in args {
                                if let Some(v) = expr_to_json(a) {
                                    arr.push(v);
                                }
                            }
                            serde_json::Value::Array(arr)
                        };
                        list.push(MetricPlan {
                            name: name.clone(),
                            function_ref: format!("{module_name}::{func}"),
                            args: json_args,
                        });
                    }
                }
            }
        }
    }
    list
}

pub fn codegen_pipeline(module_name: &str, pipeline: &PipelineDecl) -> serde_json::Result<String> {
    let steps: Vec<StepPlan> = pipeline
        .steps
        .iter()
        .map(|step| {
            let effects = analyze_effects(&step.body).to_names();
            StepPlan {
                name: step.name.clone(),
                function_ref: format!("{module_name}::step_{}", step.name),
                effects,
            }
        })
        .collect();
    let plan = ExecutionPlan {
        name: pipeline.name.clone(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        seed: pipeline.seed,
        input_schema: type_to_schema(&pipeline.input_ty),
        steps,
        constraints: pipeline
            .constraints
            .iter()
            .map(constraint_to_plan)
            .collect(),
        metrics: extract_metrics(pipeline),
        metric_plans: extract_metric_plans(module_name, pipeline),
    };
    serde_json::to_string_pretty(&plan)
}
