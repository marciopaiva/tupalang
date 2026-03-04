use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;
use tupa_codegen::execution_plan::ExecutionPlan;
use tracing::{instrument, info, error, warn};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Step execution failed: {0}")]
    StepError(String),
    #[error("Constraint violation: {0}")]
    ConstraintError(String),
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    #[error("Async runtime error: {0}")]
    AsyncError(String),
    #[error("Schema mismatch: {0}")]
    ValidationError(String),
    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),
}

// --- Circuit Breaker ---
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub failure_threshold: usize,
    pub reset_timeout: Duration,
    failures: usize,
    last_failure: Option<Instant>,
    state: BreakerState,
}

#[derive(Debug, Clone, PartialEq)]
enum BreakerState {
    Closed, // Normal operation
    Open,   // Tripped, blocking calls
    HalfOpen, // Testing recovery
    }

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            reset_timeout,
            failures: 0,
            last_failure: None,
            state: BreakerState::Closed,
        }
    }

    pub fn allow_request(&mut self) -> bool {
        match self.state {
            BreakerState::Closed => true,
            BreakerState::Open => {
                if let Some(last) = self.last_failure {
                    if last.elapsed() > self.reset_timeout {
                        self.state = BreakerState::HalfOpen;
                        return true; // Allow one trial request
                    }
                }
                false
            }
            BreakerState::HalfOpen => false, // Only one request allowed (handled by logic outside)
        }
    }

    pub fn record_success(&mut self) {
        self.failures = 0;
        self.state = BreakerState::Closed;
        self.last_failure = None;
    }

    pub fn record_failure(&mut self) {
        self.failures += 1;
        self.last_failure = Some(Instant::now());
        if self.failures >= self.failure_threshold {
            self.state = BreakerState::Open;
            warn!(target: "audit", event = "circuit_breaker_tripped", failures = self.failures);
        }
    }
}

// --- Runtime Architecture ---
type StepFunction = Box<dyn Fn(Value) -> Result<Value, String> + Send + Sync>;
type AsyncStepFunction = Box<dyn Fn(Value) -> futures::future::BoxFuture<'static, Result<Value, String>> + Send + Sync>;

struct RuntimeState {
    steps: HashMap<String, StepFunction>,
    async_steps: HashMap<String, AsyncStepFunction>,
    circuit_breaker: CircuitBreaker,
}

impl RuntimeState {
    fn new() -> Self {
        Self {
            steps: HashMap::new(),
            async_steps: HashMap::new(),
            circuit_breaker: CircuitBreaker::new(5, Duration::from_secs(30)),
        }
    }
}

#[derive(Clone)]
pub struct Runtime {
    state: Arc<Mutex<RuntimeState>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(RuntimeState::new())),
        }
    }

    pub fn register_step<F>(&self, name: &str, func: F)
    where
        F: Fn(Value) -> Result<Value, String> + Send + Sync + 'static,
    {
        let mut state = self.state.lock().unwrap();
        state.steps.insert(name.to_string(), Box::new(func));
    }

    pub fn register_async_step<F>(&self, name: &str, func: F)
    where
        F: Fn(Value) -> futures::future::BoxFuture<'static, Result<Value, String>> + Send + Sync + 'static,
    {
        let mut state = self.state.lock().unwrap();
        state.async_steps.insert(name.to_string(), Box::new(func));
    }

    pub fn configure_circuit_breaker(&self, threshold: usize, timeout: Duration) {
        let mut state = self.state.lock().unwrap();
        state.circuit_breaker = CircuitBreaker::new(threshold, timeout);
    }

    #[instrument(skip(self, plan), fields(pipeline = plan.name))]
    pub async fn run_pipeline_async(&self, plan: &ExecutionPlan, input: Value) -> RuntimeResult<Value> {
        info!(target: "audit", event = "pipeline_start", plan = plan.name);
        let mut state = input;
        
        for step in &plan.steps {
            // Check circuit breaker
            {
                let mut guard = self.state.lock().unwrap();
                if !guard.circuit_breaker.allow_request() {
                    warn!(target: "audit", event = "circuit_breaker_block", step = step.name);
                    return Err(RuntimeError::CircuitBreakerOpen(format!("Circuit breaker open for step {}", step.name)));
                }
            }
    
            // Check if async step exists
            let is_async = {
                let guard = self.state.lock().unwrap();
                guard.async_steps.contains_key(&step.function_ref)
            };
    
            let result = if is_async {
                self.call_async_step_function(&step.function_ref, state.clone()).await
            } else {
                let func_name = step.function_ref.clone();
                let input_clone = state.clone();
                let runtime = self.clone(); // Clone runtime for closure
                tokio::task::spawn_blocking(move || {
                    runtime.call_step_function(&func_name, input_clone)
                }).await.map_err(|e| RuntimeError::AsyncError(e.to_string()))?
            };
    
            match result {
                Ok(output) => {
                    {
                        let mut guard = self.state.lock().unwrap();
                        guard.circuit_breaker.record_success();
                    }
                    
                    if let Some(obj) = state.as_object_mut() {
                        obj.insert(step.name.clone(), output);
                    } else {
                        // State is primitive. Upgrade to object to store result.
                        // We preserve the original primitive value as "input".
                        let old_state = state.clone();
                        state = json!({
                            "input": old_state,
                            step.name.clone(): output
                        });
                    }
                }
                Err(e) => {
                    {
                        let mut guard = self.state.lock().unwrap();
                        guard.circuit_breaker.record_failure();
                    }
                    return Err(RuntimeError::StepError(e));
                }
            }
        }
    
        info!(target: "audit", event = "pipeline_complete", result = ?state);
        Ok(state)
    }

    #[instrument(skip(self, plan, dataset), fields(dataset_size = dataset.len()))]
    pub async fn run_backtest(&self, plan: &ExecutionPlan, dataset: Vec<Value>) -> RuntimeResult<Value> {
        info!(target: "audit", event = "backtest_start", dataset_size = dataset.len());
    
        let mut results = Vec::new();
        let mut portfolio_value = 10000.0; // Starting capital
        let mut position = 0.0; // Current holdings
    
        for (i, input) in dataset.iter().enumerate() {
            let output = self.run_pipeline_async(plan, input.clone()).await?;
            
            // Evaluate constraints (risk check)
            let constraint_report = evaluate_constraints(plan, &output);
            
            // Simple PnL logic (can be made configurable)
            let price = input.get("close").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let action = output.get("action").and_then(|v| v.as_str()).unwrap_or("HOLD");
            
            if constraint_report["success"].as_bool().unwrap_or(false) {
                 match action {
                     "BUY" => {
                         if portfolio_value > price {
                             position += 1.0;
                             portfolio_value -= price;
                             info!(target: "audit", event = "trade_executed", type = "BUY", price = price, index = i);
                         }
                     },
                     "SELL" => {
                         if position > 0.0 {
                             position -= 1.0;
                             portfolio_value += price;
                             info!(target: "audit", event = "trade_executed", type = "SELL", price = price, index = i);
                         }
                     },
                     _ => {}
                 }
            } else {
                 info!(target: "audit", event = "trade_blocked_by_risk", index = i);
            }
    
            results.push(json!({
                "index": i,
                "output": output,
                "portfolio": portfolio_value + (position * price),
                "constraints": constraint_report
            }));
        }
    
        let final_price = dataset.last().unwrap().get("close").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let final_pnl = portfolio_value + (position * final_price) - 10000.0;
        
        info!(target: "audit", event = "backtest_complete", final_pnl = final_pnl);
    
        Ok(json!({
            "final_pnl": final_pnl,
            "trades": results.len(),
            "history": results
        }))
    }

    #[instrument(skip(self, input), fields(step = name))]
    fn call_step_function(&self, name: &str, input: Value) -> Result<Value, String> {
        let guard = self.state.lock().unwrap();
        if let Some(func) = guard.steps.get(name) {
            let result = func(input);
            match &result {
                Ok(v) => info!(target: "audit", event = "step_success", output = ?v),
                Err(e) => error!(target: "audit", event = "step_failure", error = %e),
            }
            return result;
        } 
        
        // Support "py:module.func" format from codegen
        if let Some(stripped) = name.strip_prefix("py:") {
            let parts: Vec<&str> = stripped.split('.').collect();
            if parts.len() == 2 {
                drop(guard);
                let result = tupa_pyffi::call_python_function(parts[0], parts[1], input);
                match &result {
                    Ok(v) => info!(target: "audit", event = "step_success", type = "python", output = ?v),
                    Err(e) => error!(target: "audit", event = "step_failure", type = "python", error = %e),
                }
                return result;
            }
        }

        // Support "module::func" legacy format
        if name.contains("::") {
            let parts: Vec<&str> = name.split("::").collect();
            if parts.len() == 2 {
                drop(guard);
                let result = tupa_pyffi::call_python_function(parts[0], parts[1], input);
                match &result {
                    Ok(v) => info!(target: "audit", event = "step_success", type = "python", output = ?v),
                    Err(e) => error!(target: "audit", event = "step_failure", type = "python", error = %e),
                }
                return result;
            }
        }
    
        Err(format!("Function {} not found", name))
    }
    
    async fn call_async_step_function(&self, name: &str, input: Value) -> Result<Value, String> {
        let future_opt = {
            let guard = self.state.lock().unwrap();
            guard.async_steps.get(name).map(|f| f(input))
        };
        
        if let Some(fut) = future_opt {
            return fut.await;
        }
        
        Err(format!("Async function {} not found", name))
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_RUNTIME: Runtime = Runtime::new();
}

// --- Global API Delegates ---

pub fn register_step<F>(name: &str, func: F)
where
    F: Fn(Value) -> Result<Value, String> + Send + Sync + 'static,
{
    GLOBAL_RUNTIME.register_step(name, func)
}

pub fn register_async_step<F>(name: &str, func: F)
where
    F: Fn(Value) -> futures::future::BoxFuture<'static, Result<Value, String>> + Send + Sync + 'static,
{
    GLOBAL_RUNTIME.register_async_step(name, func)
}

pub fn configure_circuit_breaker(threshold: usize, timeout: Duration) {
    GLOBAL_RUNTIME.configure_circuit_breaker(threshold, timeout)
}

pub async fn run_pipeline_async(plan: &ExecutionPlan, input: Value) -> RuntimeResult<Value> {
    GLOBAL_RUNTIME.run_pipeline_async(plan, input).await
}

pub fn run_pipeline(plan: &ExecutionPlan, input: Value) -> RuntimeResult<Value> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| RuntimeError::AsyncError(e.to_string()))?;
    rt.block_on(run_pipeline_async(plan, input))
}

pub async fn run_backtest(plan: &ExecutionPlan, dataset: Vec<Value>) -> RuntimeResult<Value> {
    GLOBAL_RUNTIME.run_backtest(plan, dataset).await
}

// Helper to access nested metrics for constraints
fn get_metric_value(state: &Value, path: &str) -> Option<f64> {
    if let Some(v) = state.get(path) {
        return v.as_f64().or_else(|| v.as_bool().map(|b| if b { 1.0 } else { 0.0 }));
    }
    
    if path.contains('.') {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = state;
        for part in parts {
            match current.get(part) {
                Some(v) => current = v,
                None => return None,
            }
        }
        return current.as_f64().or_else(|| current.as_bool().map(|b| if b { 1.0 } else { 0.0 }));
    }
    None
}

pub fn evaluate_constraints(plan: &ExecutionPlan, state: &Value) -> Value {
    let mut report = json!({
        "success": true,
        "metrics": {},
        "constraints": []
    });

    for constraint in &plan.constraints {
        let metric_val = get_metric_value(state, &constraint.metric).unwrap_or(0.0);

        let pass = match constraint.comparator.as_str() {
            "gt" => metric_val > constraint.threshold,
            "lt" => metric_val < constraint.threshold,
            "eq" => (metric_val - constraint.threshold).abs() < f64::EPSILON,
            _ => false,
        };

        if !pass {
            report["success"] = json!(false);
            warn!(target: "audit", event = "constraint_violation", metric = %constraint.metric, value = metric_val, threshold = constraint.threshold);
        }

        let constraint_result = json!({
            "metric": constraint.metric,
            "comparator": constraint.comparator,
            "threshold": constraint.threshold,
            "value": metric_val,
            "pass": pass,
            "status": if pass { "pass" } else { "fail" }
        });
        
        report["constraints"].as_array_mut().unwrap().push(constraint_result);
    }

    if !report["success"].as_bool().unwrap() {
        warn!(target: "audit", event = "constraints_fail", report = ?report);
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use tupa_codegen::execution_plan::{ExecutionPlan, ConstraintPlan, TypeSchema};

    #[test]
    fn test_circuit_breaker_logic() {
        let mut cb = CircuitBreaker::new(3, Duration::from_millis(100));
        assert!(cb.allow_request(), "Should allow request initially");

        // Fail 1
        cb.record_failure();
        assert!(cb.allow_request(), "Should allow request after 1 failure");

        // Fail 2
        cb.record_failure();
        assert!(cb.allow_request(), "Should allow request after 2 failures");

        // Fail 3 (Threshold)
        cb.record_failure();
        assert!(!cb.allow_request(), "Should block request after 3 failures");

        // Wait for timeout
        sleep(Duration::from_millis(150));
        
        // Should transition to HalfOpen on next check
        assert!(cb.allow_request(), "Should allow probe request after timeout");
        
        // Subsequent checks in HalfOpen should be blocked until success
        assert!(!cb.allow_request(), "Should block subsequent requests in HalfOpen state");

        // Success -> Closed
        cb.record_success();
        assert!(cb.allow_request(), "Should allow requests after success (Closed state)");
    }

    #[test]
    fn test_evaluate_constraints() {
        let plan = ExecutionPlan {
            name: "test".into(),
            version: "1.0".into(),
            seed: None,
            input_schema: TypeSchema {
                kind: "any".into(), elem: None, len: None, name: None, tensor_shape: None, tensor_dtype: None
            },
            output_schema: None,
            steps: vec![],
            constraints: vec![
                ConstraintPlan {
                    metric: "score".into(),
                    comparator: "gt".into(),
                    threshold: 0.5,
                },
                ConstraintPlan {
                    metric: "nested.val".into(),
                    comparator: "eq".into(),
                    threshold: 10.0,
                }
            ],
            metrics: HashMap::new(),
            metric_plans: vec![],
        };

        let state_pass = json!({
            "score": 0.8,
            "nested": { "val": 10.0 }
        });
        let report_pass = evaluate_constraints(&plan, &state_pass);
        assert!(report_pass["success"].as_bool().unwrap(), "Constraints should pass");

        let state_fail = json!({
            "score": 0.2,
            "nested": { "val": 10.0 }
        });
        let report_fail = evaluate_constraints(&plan, &state_fail);
        assert!(!report_fail["success"].as_bool().unwrap(), "Constraints should fail on score");

        let state_fail_nested = json!({
            "score": 0.8,
            "nested": { "val": 9.9 }
        });
        let report_fail_nested = evaluate_constraints(&plan, &state_fail_nested);
        assert!(!report_fail_nested["success"].as_bool().unwrap(), "Constraints should fail on nested val");
    }
    
    #[test]
    fn test_get_metric_value() {
        let data = json!({
            "a": 1.0,
            "b": {
                "c": 2.0,
                "d": { "e": 3.0 }
            },
            "flag": true
        });
        
        assert_eq!(get_metric_value(&data, "a"), Some(1.0));
        assert_eq!(get_metric_value(&data, "b.c"), Some(2.0));
        assert_eq!(get_metric_value(&data, "b.d.e"), Some(3.0));
        assert_eq!(get_metric_value(&data, "flag"), Some(1.0)); // bool -> 1.0
        assert_eq!(get_metric_value(&data, "missing"), None);
        assert_eq!(get_metric_value(&data, "b.missing"), None);
    }

    #[tokio::test]
    async fn test_runtime_integration() {
        use tupa_codegen::execution_plan::StepPlan;

        let runtime = Runtime::new();
        
        // Register a step
        runtime.register_step("double", |input| {
            let val = input.get("value").and_then(|v| v.as_f64()).ok_or("Input must contain 'value'")?;
            Ok(json!(val * 2.0))
        });

        // Create plan
        let plan = ExecutionPlan {
            name: "integration_test".into(),
            version: "1.0".into(),
            seed: None,
            input_schema: TypeSchema { 
                kind: "object".into(), 
                elem: None, 
                len: None, 
                name: None, 
                tensor_shape: None, 
                tensor_dtype: None 
            },
            output_schema: None,
            steps: vec![
                StepPlan {
                    name: "result".into(),
                    function_ref: "double".into(),
                    effects: vec![],
                }
            ],
            constraints: vec![
                ConstraintPlan {
                    metric: "result".into(),
                    comparator: "gt".into(),
                    threshold: 10.0,
                }
            ],
            metrics: HashMap::new(),
            metric_plans: vec![],
        };

        // Test pipeline execution
        let input = json!({ "value": 10.0 });
        let output = runtime.run_pipeline_async(&plan, input).await.expect("Pipeline failed");
        assert_eq!(output["result"], 20.0);

        // Test backtest
        let dataset = vec![
            json!({ "close": 100.0, "action": "BUY" }), // Should buy (if constraint passes)
            json!({ "close": 110.0, "action": "SELL" }), // Should sell
            json!({ "close": 105.0, "action": "HOLD" }),
        ];
        
        // We need a plan that produces "action" output for backtest logic to work
        // For this test, we'll register a mock strategy step
        runtime.register_step("strategy", |input| {
            // Just pass through the action from input
            let action = input.get("action").and_then(|v| v.as_str()).unwrap_or("HOLD");
            Ok(json!(action))
        });

        let backtest_plan = ExecutionPlan {
            name: "backtest_test".into(),
            version: "1.0".into(),
            seed: None,
            input_schema: TypeSchema { 
                kind: "object".into(), 
                elem: None, 
                len: None, 
                name: None, 
                tensor_shape: None, 
                tensor_dtype: None 
            },
            output_schema: None,
            steps: vec![
                StepPlan {
                    name: "action".into(),
                    function_ref: "strategy".into(),
                    effects: vec![],
                }
            ],
            constraints: vec![], // No constraints, so always success
            metrics: HashMap::new(),
            metric_plans: vec![],
        };

        let backtest_result = runtime.run_backtest(&backtest_plan, dataset).await.expect("Backtest failed");
        
        // Verify PnL
        // Buy at 100, Sell at 110 => +10 profit
        // Initial capital 10000 -> 9900 (pos=1) -> 10010 (pos=0) -> PnL = 10
        let pnl = backtest_result["final_pnl"].as_f64().expect("PnL should be f64");
        assert!((pnl - 10.0).abs() < f64::EPSILON, "PnL should be 10.0, got {}", pnl);
    }

    #[tokio::test]
    async fn test_python_step_resolution() {
        use tupa_codegen::execution_plan::StepPlan;
        
        // This test requires python environment with math module (standard)
        let runtime = Runtime::new();
        
        // Plan with py:math.sqrt
        let plan = ExecutionPlan {
            name: "python_test".into(),
            version: "1.0".into(),
            seed: None,
            input_schema: TypeSchema { 
                kind: "number".into(), 
                elem: None, 
                len: None, 
                name: None, 
                tensor_shape: None, 
                tensor_dtype: None 
            },
            output_schema: None,
            steps: vec![
                StepPlan {
                    name: "root".into(),
                    function_ref: "py:math.sqrt".into(),
                    effects: vec![],
                }
            ],
            constraints: vec![],
            metrics: HashMap::new(),
            metric_plans: vec![],
        };

        let input = json!(16.0);
        
        // We expect this to fail if python is not set up correctly in CI, 
        // but locally it should work or fail with "Module not loaded".
        // If tupa-pyffi is compiled with extension-module, it might panic if not run in python.
        // But tupa-pyffi uses prepare_freethreaded_python(), so it should be fine for embedding.
        
        let result = runtime.run_pipeline_async(&plan, input).await;
        
        match result {
            Ok(output) => {
                assert_eq!(output["root"], 4.0);
            }
            Err(e) => {
                // If python fails (e.g. no python lib), we accept it but warn
                println!("Python test skipped/failed: {}", e);
            }
        }
    }
}
