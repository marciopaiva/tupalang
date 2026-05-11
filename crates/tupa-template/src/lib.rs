use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::{Serialize, Deserialize};

/// Input to your policy pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// Transaction amount
    pub amount: f64,
    /// User risk score (0-1)
    pub risk_score: f64,
}

/// Example step: enrich input with computed fields
fn enrich(input: &Input) -> Input {
    // Add derived fields if needed
    Input { ..*input }
}

/// Example step: compute a risk score
fn compute_score(input: &Input) -> f64 {
    input.risk_score * 100.0
}

/// Define the pipeline with constraints.
pipeline! {
    name: MyPolicy,
    input: Input,
    steps: [
        step("enrich")   { enrich(input) } produces ["enriched"],
        step("score")    { compute_score(input) } requires ["enriched"] produces ["score_val"]
    ],
    constraints: [
        metric("score_val").ge(0.0)
    ]
}
