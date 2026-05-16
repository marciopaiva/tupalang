//! Tupã pipeline executor.
//!
//! This crate provides the runtime engine for executing Tupã pipelines:
//! - Constraint evaluation
//! - Metric collection
//! - Step orchestration (sequential or channel-based parallel)
//!
//! ## Parallel execution
//!
//! The executor can run pipeline steps in parallel based on their
//! declared `produces` and `requires` metric dependencies. Steps that
//! do not depend on each other may run simultaneously.
//!
//! Use `Executor::run_parallel` for data-parallel execution.

use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};

/// Main executor for Tupã pipelines.
///
/// The executor is responsible for scheduling step execution, managing
/// shared metric values, and evaluating constraints. It supports both
/// sequential (`run`) and parallel (`run_parallel`) execution modes.
///
/// # Examples
///
/// ```rust
/// use tupa_engine::{Executor, ExecutorConfig};
/// use std::time::Duration;
///
/// // Default executor (no timeout, large channel)
/// let engine = Executor::new();
///
/// // Configured executor with 30s step timeout and custom channel capacity
/// let config = ExecutorConfig::new()
///     .with_step_timeout(Duration::from_secs(30))
///     .with_channel_capacity(5000);
/// let engine = Executor::with_config(config);
/// ```
#[derive(Debug, Clone)]
pub struct Executor {
    step_timeout: Option<std::time::Duration>,
    channel_capacity: usize,
    metrics_output: Option<std::path::PathBuf>,
    cancel_token: Arc<AtomicBool>,
}

/// Configuration for an [`Executor`].
///
/// Set options like step timeouts, channel capacity, and metrics output.
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    step_timeout: Option<std::time::Duration>,
    channel_capacity: usize,
    metrics_output: Option<std::path::PathBuf>,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            step_timeout: None,
            channel_capacity: 10000, // generous default
            metrics_output: None,
        }
    }
}

impl ExecutorConfig {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration from environment variables.
    ///
    /// Environment variables:
    /// - `TUPA_STEP_TIMEOUT`: duration string (e.g., "30s", "1m", "500ms")
    /// - `TUPA_CHANNEL_CAPACITY`: usize (default: 10000)
    /// - `TUPA_METRICS_OUTPUT`: path to write metrics JSON (optional)
    ///
    /// If variables are unset or invalid, defaults are used.
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(timeout_str) = std::env::var("TUPA_STEP_TIMEOUT") {
            if let Ok(duration) = parse_duration(&timeout_str) {
                config.step_timeout = Some(duration);
            }
        }

        if let Ok(capacity_str) = std::env::var("TUPA_CHANNEL_CAPACITY") {
            if let Ok(capacity) = capacity_str.parse::<usize>() {
                config.channel_capacity = capacity;
            }
        }

        if let Ok(metrics_path) = std::env::var("TUPA_METRICS_OUTPUT") {
            config.metrics_output = Some(std::path::PathBuf::from(metrics_path));
        }

        config
    }

    /// Set a timeout for each step execution.
    /// If a step does not complete within this duration, `EngineError::StepTimeout` is returned.
    pub fn with_step_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.step_timeout = Some(timeout);
        self
    }

    /// Set the channel capacity for step completion notifications.
    /// A bounded channel applies backpressure when full. Default is 10000.
    pub fn with_channel_capacity(mut self, capacity: usize) -> Self {
        self.channel_capacity = capacity;
        self
    }

    /// Set a path to write metrics JSON after pipeline execution.
    /// If set, `Executor::run_parallel` will serialize `PipelineResult::metrics` to this file.
    pub fn with_metrics_output(mut self, path: std::path::PathBuf) -> Self {
        self.metrics_output = Some(path);
        self
    }
}

/// Parse a duration string like "30s", "1m", "500ms".
///
/// Supported units: ms, s, m. If no unit given, assumes seconds.
fn parse_duration(s: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let s = s.trim();
    if let Some(ms_str) = s.strip_suffix("ms") {
        let ms = ms_str.parse::<u64>()?;
        Ok(std::time::Duration::from_millis(ms))
    } else if let Some(s_str) = s.strip_suffix("s") {
        let secs = s_str.parse::<u64>()?;
        Ok(std::time::Duration::from_secs(secs))
    } else if let Some(m_str) = s.strip_suffix("m") {
        let mins = m_str.parse::<u64>()?;
        Ok(std::time::Duration::from_secs(mins * 60))
    } else {
        // Assume seconds if no unit
        let secs = s.parse::<u64>()?;
        Ok(std::time::Duration::from_secs(secs))
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    /// Create a new executor with default configuration.
    ///
    /// Defaults:
    /// - No step timeout (steps may run indefinitely)
    /// - Channel capacity: 10000
    pub fn new() -> Self {
        Self::with_config(ExecutorConfig::default())
    }

    /// Create an executor with the given configuration.
    pub fn with_config(config: ExecutorConfig) -> Self {
        Self {
            step_timeout: config.step_timeout,
            channel_capacity: config.channel_capacity,
            metrics_output: config.metrics_output,
            cancel_token: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Create a new executor with configuration from environment variables.
    ///
    /// Environment variables:
    /// - `TUPA_STEP_TIMEOUT`: duration string (e.g., "30s", "1m", "500ms")
    /// - `TUPA_CHANNEL_CAPACITY`: usize (default: 10000)
    /// - `TUPA_METRICS_OUTPUT`: path to write metrics JSON (optional)
    ///
    /// If variables are unset or invalid, defaults are used.
    ///
    /// # Examples
    ///
    /// ```
    /// // Use environment configuration
    /// let engine = tupa_engine::Executor::from_env();
    /// ```
    pub fn from_env() -> Self {
        Self::with_config(ExecutorConfig::from_env())
    }

    /// Cancel any in-progress pipeline execution.
    ///
    /// Subsequent calls to `run_parallel` will return `EngineError::Cancelled`.
    pub fn cancel(&self) {
        self.cancel_token.store(true, Ordering::SeqCst);
    }

    /// Execute a pipeline synchronously (sequential step execution).
    ///
    /// # Arguments
    /// - `pipeline`: a reference to a type implementing `ExecutorPipeline`
    /// - `input`: reference to input data
    ///
    /// # Returns
    /// `PipelineResult` with metric values and constraint pass/fail.
    ///
    /// # Errors
    /// Returns `EngineError` if step panics or constraint fails.
    pub fn run<P, I>(&self, pipeline: &P, input: &I) -> Result<PipelineResult, EngineError>
    where
        P: ExecutorPipeline<Input = I>,
        I: Send + Sync + 'static + Clone + Serialize,
    {
        pipeline.execute(input)
    }

    /// Execute a pipeline with parallel step execution.
    ///
    /// Steps are scheduled concurrently based on their declared
    /// `produces` and `requires` metric dependencies. Steps that
    /// do not depend on each other may run simultaneously.
    ///
    /// Requires a Tokio runtime.
    pub async fn run_parallel<P, I>(
        &self,
        pipeline: &P,
        input: &I,
    ) -> Result<PipelineResult, EngineError>
    where
        P: ParallelPipeline<Input = I> + Clone + Send + 'static,
        I: Send + Sync + 'static + Clone + Serialize,
    {
        use std::collections::HashMap;

        let step_ids = pipeline.step_ids();
        let n_steps = step_ids.len();

        // Build dependency graph
        let mut produces_map: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut dependents: HashMap<&str, Vec<&str>> = HashMap::new();

        for &id in step_ids {
            let produces: Vec<&str> = pipeline.produces(id).to_vec();
            let requires: Vec<&str> = pipeline.requires(id).to_vec();
            produces_map.insert(id, produces);
            for &req in &requires {
                dependents.entry(req).or_default().push(id);
            }
        }

        // In-degree map
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        for &id in step_ids {
            in_degree.insert(id, pipeline.requires(id).len());
        }

        // Validate DAG (topological sort) before execution
        {
            let mut temp_in_degree = in_degree.clone();
            let mut ready: Vec<&str> = step_ids
                .iter()
                .copied()
                .filter(|id| temp_in_degree[*id] == 0)
                .collect();
            let mut visited = 0;
            while let Some(node) = ready.pop() {
                visited += 1;
                if let Some(produced_metrics) = produces_map.get(node) {
                    for &metric in produced_metrics {
                        if let Some(deps) = dependents.get(metric) {
                            for &dep in deps {
                                let deg = temp_in_degree.get_mut(dep).unwrap();
                                *deg = deg.saturating_sub(1);
                                if *deg == 0 {
                                    ready.push(dep);
                                }
                            }
                        }
                    }
                }
            }
            if visited != n_steps {
                let mut stuck: Vec<&str> = temp_in_degree
                    .iter()
                    .filter_map(|(&id, &deg)| if deg > 0 { Some(id) } else { None })
                    .collect();
                stuck.sort();
                return Err(EngineError::CycleDetected {
                    steps: stuck.join(", "),
                });
            }
        }

        // Shared state
        let values = Arc::new(Mutex::new(HashMap::new()));
        let step_metrics: Arc<Mutex<HashMap<String, StepMetrics>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // Channel for step completion notifications: (step_id, result)
        // Use bounded channel to apply backpressure; capacity configurable.
        let (complete_tx, mut complete_rx) = mpsc::channel::<(
            String,
            Result<serde_json::Value, EngineError>,
        )>(self.channel_capacity);

        // Prepare owned pipeline and input for workers
        let pipeline_owned = <P as Clone>::clone(pipeline);
        let input_owned = <I as Clone>::clone(input);

        // Cancellation token
        let cancellation = self.cancel_token.clone();

        // Manager gets its own Arc for values and a clone of complete_tx to pass to workers
        let manager_values = values.clone();
        let manager_metrics = step_metrics.clone();
        let manager_complete_tx = complete_tx.clone();
        let step_timeout = self.step_timeout;
        let manager_cancellation = cancellation.clone();

        // Spawn manager task
        let manager_handle = tokio::spawn(async move {
            // Helper to spawn a worker for a step
            let spawn_worker =
                |step_id: String,
                 pipeline: P,
                 input: I,
                 complete_tx: mpsc::Sender<(String, Result<serde_json::Value, EngineError>)>,
                 metrics: Arc<Mutex<HashMap<String, StepMetrics>>>| {
                    tokio::spawn(async move {
                        // Record start time
                        let start = std::time::SystemTime::now();
                        let start_nanos = start
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos() as u64;
                        {
                            let mut guard = metrics.lock().await;
                            guard.insert(
                                step_id.clone(),
                                StepMetrics {
                                    step_id: step_id.clone(),
                                    start_nanos,
                                    end_nanos: None,
                                    duration_nanos: None,
                                    state: StepState::Running,
                                },
                            );
                        }

                        // Execute step with optional timeout
                        // Use spawn_blocking for sync execute_step to prevent blocking the async runtime
                        let result: Result<Value, EngineError> = if let Some(timeout) = step_timeout
                        {
                            let step_id_for_timeout = step_id.clone();
                            match tokio::time::timeout(timeout, async move {
                                // Execute blocking code on a blocking thread pool
                                let input_clone = input.clone();
                                let step_id_str = step_id_for_timeout.clone();
                                tokio::task::spawn_blocking(move || {
                                    pipeline.execute_step(&input_clone, &step_id_str)
                                })
                                .await
                                .unwrap_or(Err(EngineError::Other("spawn blocking failed".into())))
                            })
                            .await
                            {
                                Ok(inner_result) => inner_result,
                                Err(_) => Err(EngineError::StepTimeout {
                                    step: step_id.clone(),
                                    duration: timeout,
                                }),
                            }
                        } else {
                            // No timeout - use spawn_blocking to avoid blocking async runtime
                            let input_clone = input.clone();
                            let step_id_str = step_id.clone();
                            tokio::task::spawn_blocking(move || {
                                pipeline.execute_step(&input_clone, &step_id_str)
                            })
                            .await
                            .unwrap_or(Err(EngineError::Other("spawn blocking failed".into())))
                        };

                        // Record completion
                        let end = std::time::SystemTime::now();
                        let end_nanos = end
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_nanos() as u64;
                        let duration_nanos =
                            end.duration_since(start).ok().map(|d| d.as_nanos() as u64);
                        let state = match &result {
                            Ok(_) => StepState::Completed,
                            Err(EngineError::StepTimeout { .. }) => StepState::Timeout,
                            Err(_) => StepState::Failed,
                        };
                        {
                            let mut guard = metrics.lock().await;
                            if let Some(entry) = guard.get_mut(&step_id) {
                                entry.end_nanos = Some(end_nanos);
                                entry.duration_nanos = duration_nanos;
                                entry.state = state;
                            }
                        }

                        // Do NOT insert values here; manager does it after collecting produces metadata
                        let _ = complete_tx.send((step_id, result)).await;
                    });
                };

            // Spawn initial ready steps
            for step_id in step_ids.iter().copied().filter(|&id| in_degree[id] == 0) {
                spawn_worker(
                    step_id.to_string(),
                    pipeline_owned.clone(),
                    input_owned.clone(),
                    manager_complete_tx.clone(),
                    manager_metrics.clone(),
                );
            }

            let mut completed = 0;
            // Process completions
            while let Some((step_id, step_res)) = complete_rx.recv().await {
                // Check for cancellation
                if manager_cancellation.load(Ordering::SeqCst) {
                    return Err(EngineError::Cancelled);
                }

                // If this step failed, return error immediately
                if let Err(e) = step_res {
                    return Err(e);
                }

                // Insert step result into values map using produces metadata
                if let Ok(val) = &step_res {
                    let mut guard = manager_values.lock().await;
                    for metric in pipeline_owned.produces(&step_id) {
                        guard.insert(metric.to_string(), val.clone());
                    }
                }

                // Update in-degree for dependents of this step
                if let Some(produced) = produces_map.get(step_id.as_str()) {
                    for &metric in produced {
                        if let Some(deps) = dependents.get(metric) {
                            for &dep in deps {
                                if let Some(count) = in_degree.get_mut(dep) {
                                    *count = count.saturating_sub(1);
                                    if *count == 0 {
                                        // spawn worker for newly ready step
                                        spawn_worker(
                                            dep.to_string(),
                                            pipeline_owned.clone(),
                                            input_owned.clone(),
                                            manager_complete_tx.clone(),
                                            manager_metrics.clone(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                completed += 1;
                if completed == n_steps {
                    break;
                }
            }

            // Drop our sender so workers won't block on send
            drop(manager_complete_tx);
            Ok(())
        });

        // Wait for manager to finish
        match manager_handle.await {
            Ok(Ok(())) => {
                // All steps completed successfully; evaluate constraints
                let values_guard = values.lock().await;
                let (passed, failures) = P::check_constraints(&values_guard);

                // Collect metrics from Arc
                let metrics_guard = step_metrics.lock().await;
                let metrics_vec: Vec<StepMetrics> = metrics_guard.values().cloned().collect();

                let result = PipelineResult {
                    values: values_guard.clone(),
                    passed,
                    failures,
                    metrics: metrics_vec.clone(),
                };

                // Write metrics to file if configured
                if let Some(ref path) = self.metrics_output {
                    let json = serde_json::to_string_pretty(&metrics_vec).map_err(|e| {
                        EngineError::Other(format!("Failed to serialize metrics: {}", e))
                    })?;
                    std::fs::write(path, json).map_err(|e| {
                        EngineError::Other(format!("Failed to write metrics file: {}", e))
                    })?;
                }

                Ok(result)
            }
            Ok(Err(e)) => Err(e),
            Err(join_err) => Err(EngineError::Other(join_err.to_string())),
        }
    }
}

/// A pipeline that can be executed by the engine.
///
/// This trait is implemented automatically by the `pipeline!` macro.
pub trait ExecutorPipeline: tupa_core::Pipeline {
    /// Execute the pipeline with the given input and return the result.
    fn execute(&self, input: &Self::Input) -> Result<PipelineResult, EngineError>;
}

/// Trait for parallel-capable pipelines. Automatically implemented
/// by the `pipeline!` macro when metadata (produces/requires) is available.
pub trait ParallelPipeline: ExecutorPipeline {
    /// Returns the list of step IDs in the pipeline.
    fn step_ids(&self) -> &'static [&'static str];
    /// Returns the metrics produced by the given step.
    fn produces(&self, step_id: &str) -> &'static [&'static str];
    /// Returns the metrics required by the given step.
    fn requires(&self, step_id: &str) -> &'static [&'static str];
    /// Execute a single step independently (used by parallel scheduler).
    fn execute_step(
        &self,
        input: &Self::Input,
        step_id: &str,
    ) -> Result<serde_json::Value, EngineError>;
    /// Check constraints against collected metric values. Returns `(passed, failures)`.
    fn check_constraints(
        values: &std::collections::HashMap<String, serde_json::Value>,
    ) -> (bool, Vec<ConstraintFailure>);
}

/// Result of pipeline execution.
#[derive(Debug, Clone)]
#[must_use]
pub struct PipelineResult {
    /// All collected metric values (keyed by metric or step name)
    pub values: HashMap<String, Value>,
    /// True if all constraints passed
    pub passed: bool,
    /// Details of any constraint failures
    pub failures: Vec<ConstraintFailure>,
    /// Step-by-step execution metrics (timings, state)
    pub metrics: Vec<StepMetrics>,
}

/// Per-step execution metrics.
#[derive(Debug, Clone, serde::Serialize)]
pub struct StepMetrics {
    /// Step identifier
    pub step_id: String,
    /// Start timestamp (nanoseconds since UNIX epoch)
    pub start_nanos: u64,
    /// End timestamp (nanoseconds since UNIX epoch), if finished
    pub end_nanos: Option<u64>,
    /// Total duration in nanoseconds (computed as end - start), if finished
    pub duration_nanos: Option<u64>,
    /// Final execution state
    pub state: StepState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum StepState {
    /// Step is currently running
    Running,
    /// Step completed successfully
    Completed,
    /// Step failed (non-timeout error)
    Failed,
    /// Step timed out
    Timeout,
    /// Step was cancelled
    Cancelled,
}

impl PipelineResult {
    /// Create a new empty (passing) pipeline result.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for PipelineResult {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            passed: true,
            failures: Vec::new(),
            metrics: Vec::new(),
        }
    }
}

/// Information about a single constraint that failed.
#[derive(Debug, Clone)]
pub struct ConstraintFailure {
    /// The metric name that violated the constraint.
    pub metric: String,
    /// The operator that was used (e.g., "ge", "le").
    pub operator: String,
    /// The expected value (threshold).
    pub expected: Value,
    /// The actual value observed.
    pub actual: Value,
}

impl std::fmt::Display for ConstraintFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "constraint failed: {} {} {} (actual: {})",
            self.metric, self.operator, self.expected, self.actual
        )
    }
}

/// Execution engine errors.
#[derive(Error, Debug)]
pub enum EngineError {
    /// A step function panicked during execution.
    #[error("Step '{step}' panicked: {reason}")]
    StepPanic {
        /// The ID of the step that panicked.
        step: String,
        /// The panic reason or backtrace.
        reason: String,
    },

    /// A constraint was violated. Contains metric name, operator, expected and actual values.
    #[error("Constraint failed: {metric} {op} {expected} (actual {actual})")]
    ConstraintFailed {
        /// The metric name that violated the constraint.
        metric: String,
        /// The comparison operator that was used (e.g., "ge", "le", "eq").
        op: String,
        /// The expected threshold value.
        expected: serde_json::Value,
        /// The actual value observed.
        actual: serde_json::Value,
    },

    /// A dependency cycle was detected in the pipeline DAG.
    #[error("Dependency cycle detected: unsatisfied steps: {steps}")]
    CycleDetected {
        /// Comma-separated list of steps involved in the cycle.
        steps: String,
    },

    /// A step execution exceeded its configured timeout.
    #[error("Step '{step}' timed out after {duration:?}")]
    StepTimeout {
        /// The step ID that timed out.
        step: String,
        /// The timeout duration that was exceeded.
        duration: std::time::Duration,
    },

    /// A generic pipeline execution error (fallback).
    #[error("Pipeline execution error: {0}")]
    Other(String),

    /// Pipeline execution was cancelled (user signal or programmatic cancel).
    #[error("Pipeline cancelled")]
    Cancelled,
}
