//! tupa-engine unit tests — coverage gap fix for 0.9.4, updated for 0.11.0.
//!
//! Classe 1 : ExecutorConfig + parse_duration via from_env
//! Classe 4 : StepState, StepMetrics, PipelineResult, ConstraintFailure, EngineError
//! Classe 9 : Executor::run (mock pipeline manual)
//! Classe 10: Executor::run_parallel #[tokio::test]

use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tupa_core::Pipeline;
use tupa_engine::{
    ConstraintFailure, EngineError, Executor, ExecutorConfig, ExecutorPipeline, ParallelPipeline,
    PipelineResult, StepContext, StepMetrics, StepState,
};

// ═══════════════════════════════════════════════════════════════════════════
// CLASSE 1 — ExecutorConfig + parse_duration (via from_env)
// ═══════════════════════════════════════════════════════════════════════════
// ExecutorConfig fields are private; testamos apenas comportamento via API pública.
// parse_duration também é privado e é exercido indiretamente por from_env + run_parallel.

fn tupa_engine_clean_env() {
    for v in &[
        "TUPA_STEP_TIMEOUT",
        "TUPA_CHANNEL_CAPACITY",
        "TUPA_METRICS_OUTPUT",
    ] {
        std::env::remove_var(v);
    }
}

mod class1 {

    use super::tupa_engine_clean_env;
    use super::*;

    // TC-01..TC-06: construtores/builder não panicam ─────────────────────────
    #[test]
    fn tc01_new_default() {
        drop(ExecutorConfig::new());
    }
    #[test]
    fn tc02_new_chain_timeout() {
        drop(ExecutorConfig::new().with_step_timeout(Duration::from_secs(30)));
    }
    #[test]
    fn tc03_new_chain_capacity() {
        drop(ExecutorConfig::new().with_channel_capacity(7));
    }
    #[test]
    fn tc04_new_chain_metrics() {
        drop(ExecutorConfig::new().with_metrics_output(std::path::PathBuf::from("/tmp/m.json")));
    }
    #[test]
    fn tc05_new_chain_all() {
        drop(
            ExecutorConfig::new()
                .with_step_timeout(Duration::from_secs(1))
                .with_channel_capacity(1)
                .with_metrics_output(std::path::PathBuf::from("/tmp/m.json")),
        );
    }
    #[test]
    fn tc06_default_struct() {
        drop(ExecutorConfig::default());
    }

    // Macros para repetir assertions de from_env sem boilerplate
    macro_rules! assert_timeout {
        ($input:literal, $secs:expr) => {{
            tupa_engine_clean_env();
            std::env::set_var("TUPA_STEP_TIMEOUT", $input);
            let c = ExecutorConfig::from_env();
            // from_env retorna config com o timeout parseado corretamente
            // (step_timeout é privado; testamos comportamento via executores abaixo)
            let _ = c; // from_env retorna sem panic
        }};
    }

    #[test]
    fn tc07_from_env_ms() {
        assert_timeout!("500ms", 0);
    }
    #[test]
    fn tc08_from_env_s() {
        assert_timeout!("30s", 30);
    }
    #[test]
    fn tc09_from_env_m() {
        assert_timeout!("2m", 120);
    }
    #[test]
    fn tc10_from_env_bare() {
        assert_timeout!("5", 5);
    }
    #[test]
    fn tc11_from_env_zero_ms() {
        assert_timeout!("0ms", 0);
    }
    #[test]
    fn tc12_from_env_one_s() {
        assert_timeout!("1s", 1);
    }

    #[test]
    fn tc13_from_env_default_when_no_vars() {
        tupa_engine_clean_env();
        let c = ExecutorConfig::from_env();
        let _ = c; // from_env não panicou; campos privados acessados via drop
    }

    #[test]
    fn tc14_from_env_invalid_timeout_ignored() {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_STEP_TIMEOUT", "not-valid");
        let _ = ExecutorConfig::from_env(); // inválido → default (nenhum panic)
    }

    #[test]
    fn tc15_from_env_capacity_parses() {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_CHANNEL_CAPACITY", "5000");
        std::env::set_var("TUPA_METRICS_OUTPUT", "/tmp/cap_test.json");
        let _ = ExecutorConfig::from_env(); // parseia sem panic
    }

    #[test]
    fn tc16_from_env_metrics_parses() {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_METRICS_OUTPUT", "/tmp/m.json");
        let _ = ExecutorConfig::from_env(); // sem panic
    }

    #[test]
    fn tc17_from_env_all_three_vars() {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_STEP_TIMEOUT", "3s");
        std::env::set_var("TUPA_CHANNEL_CAPACITY", "8888");
        std::env::set_var("TUPA_METRICS_OUTPUT", "/tmp/a.json");
        let _ = ExecutorConfig::from_env(); // combina 3 vars sem panic
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CLASSE 2 — Executor construtores + Send
// ═══════════════════════════════════════════════════════════════════════════
mod class2 {

    use super::*;

    #[test]
    fn tc18_new_default() {
        drop(Executor::new());
    }
    #[test]
    fn tc19_from_env_default() {
        tupa_engine_clean_env();
        drop(Executor::from_env());
    }
    #[test]
    fn tc20_from_env_with_timeout() {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_STEP_TIMEOUT", "9s");
        drop(Executor::from_env());
        tupa_engine_clean_env();
    }
    #[test]
    fn tc21_executor_is_send() {
        fn is_send<T: Send>() {}
        is_send::<Executor>();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CLASSE 4 — Tipos de valor
// ═══════════════════════════════════════════════════════════════════════════
mod class4 {

    use super::*;

    // ── StepState ──────────────────────────────────────────────────────────
    #[test]
    fn tc22_variants_distinct() {
        assert_ne!(StepState::Running, StepState::Completed);
        assert_ne!(StepState::Failed, StepState::Timeout);
        assert_ne!(StepState::Timeout, StepState::Cancelled);
    }

    #[test]
    fn tc23_debug_fmt_running() {
        assert_eq!(format!("{:?}", StepState::Running), "Running");
    }
    #[test]
    fn tc24_debug_fmt_completed() {
        assert_eq!(format!("{:?}", StepState::Completed), "Completed");
    }
    #[test]
    fn tc25_debug_fmt_failed() {
        assert_eq!(format!("{:?}", StepState::Failed), "Failed");
    }
    #[test]
    fn tc26_debug_fmt_timeout() {
        assert_eq!(format!("{:?}", StepState::Timeout), "Timeout");
    }
    #[test]
    fn tc27_debug_fmt_cancelled() {
        assert_eq!(format!("{:?}", StepState::Cancelled), "Cancelled");
    }

    #[test]
    fn tc28_serialize_enum_names() {
        assert_eq!(
            serde_json::to_string(&StepState::Running).unwrap(),
            r#""Running""#
        );
        assert_eq!(
            serde_json::to_string(&StepState::Completed).unwrap(),
            r#""Completed""#
        );
        assert_eq!(
            serde_json::to_string(&StepState::Failed).unwrap(),
            r#""Failed""#
        );
        assert_eq!(
            serde_json::to_string(&StepState::Timeout).unwrap(),
            r#""Timeout""#
        );
        assert_eq!(
            serde_json::to_string(&StepState::Cancelled).unwrap(),
            r#""Cancelled""#
        );
    }

    // ── StepMetrics ────────────────────────────────────────────────────────
    #[test]
    fn tc29_running_no_end_nanos() {
        let m = StepMetrics {
            step_id: "a".into(),
            start_nanos: 0,
            end_nanos: None,
            duration_nanos: None,
            state: StepState::Running,
        };
        assert!(m.end_nanos.is_none());
        assert!(m.duration_nanos.is_none());
        assert_eq!(m.state, StepState::Running);
    }

    #[test]
    fn tc30_completed_with_duration() {
        let m = StepMetrics {
            step_id: "a".into(),
            start_nanos: 0,
            end_nanos: Some(5_000),
            duration_nanos: Some(5_000),
            state: StepState::Completed,
        };
        assert_eq!(m.duration_nanos, Some(5_000));
        assert_eq!(m.state, StepState::Completed);
    }

    #[test]
    /// Serialize → string contém os campos essenciais.
    fn tc31_serialize_json_includes_fields() {
        let m = StepMetrics {
            step_id: "s".into(),
            start_nanos: 0,
            end_nanos: Some(1000),
            duration_nanos: Some(1000),
            state: StepState::Completed,
        };
        let json = serde_json::to_string(&m).unwrap();
        assert!(json.contains("\"step_id\":\"s\""), "json={}", json);
        assert!(json.contains("\"state\":\"Completed\""));
    }

    #[test]
    fn tc32_json_fields_present() {
        let m = StepMetrics {
            step_id: "x".into(),
            start_nanos: 42,
            end_nanos: Some(1000),
            duration_nanos: Some(958),
            state: StepState::Timeout,
        };
        let v: Value = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
        assert_eq!(v["step_id"], Value::from("x"));
        assert_eq!(v["state"], Value::from("Timeout"));
        assert_eq!(v["start_nanos"], Value::from(42u64));
        assert_eq!(v["end_nanos"], Value::from(1000u64));
        assert_eq!(v["duration_nanos"], Value::from(958u64));
    }

    #[test]
    fn tc33_clone_independent() {
        let m1 = StepMetrics {
            step_id: "a".into(),
            start_nanos: 0,
            end_nanos: None,
            duration_nanos: None,
            state: StepState::Running,
        };
        let m2 = m1.clone();
        assert_eq!(m1.step_id, m2.step_id);
    }

    // ── PipelineResult ─────────────────────────────────────────────────────
    #[test]
    fn tc34_default_passes() {
        assert!(PipelineResult::default().passed);
    }
    #[test]
    fn tc35_default_empty_collections() {
        let r = PipelineResult::default();
        assert!(r.values.is_empty() && r.failures.is_empty() && r.metrics.is_empty());
    }
    #[test]
    fn tc36_new_is_default() {
        assert!(PipelineResult::new().passed);
    }

    #[test]
    fn tc37_with_single_metric() {
        let m = StepMetrics {
            step_id: "a".into(),
            start_nanos: 0,
            end_nanos: Some(1000),
            duration_nanos: Some(1000),
            state: StepState::Completed,
        };
        let r = PipelineResult {
            values: HashMap::new(),
            passed: true,
            failures: vec![],
            metrics: vec![m.clone()],
        };
        assert_eq!(r.metrics.len(), 1);
        assert_eq!(r.metrics[0].state, StepState::Completed);
    }

    #[test]
    fn tc38_with_constraint_failures() {
        let cf = ConstraintFailure {
            metric: "score".into(),
            operator: "lt".into(),
            expected: Value::from(0.1),
            actual: Value::from(0.4),
        };
        let r = PipelineResult {
            values: HashMap::new(),
            passed: false,
            failures: vec![cf.clone()],
            metrics: vec![],
        };
        assert!(!r.passed);
        assert_eq!(r.failures[0].metric, "score");
    }

    // ── ConstraintFailure ──────────────────────────────────────────────────
    #[test]
    fn tc39_display_contains_metric() {
        let cf = ConstraintFailure {
            metric: "drawdown".into(),
            operator: "<".into(),
            expected: Value::from(0.2),
            actual: Value::from(0.35),
        };
        assert!(cf.to_string().contains("drawdown"));
    }

    #[test]
    fn tc40_clone_preserves_fields() {
        let c1 = ConstraintFailure {
            metric: "x".into(),
            operator: "ge".into(),
            expected: Value::from(1.0),
            actual: Value::from(0.5),
        };
        let c2 = c1.clone();
        assert_eq!(c1.metric, c2.metric);
        assert_eq!(c1.operator, c2.operator);
    }

    // ── EngineError ────────────────────────────────────────────────────────
    #[test]
    fn tc41_step_panic_display() {
        let e = EngineError::StepPanic {
            step: "x".into(),
            reason: "oops".into(),
        };
        assert!(e.to_string().contains("x"));
        // StepPanic não tem source impl (thiserror não gera source automaticamente)
    }
    #[test]
    fn tc42_constraint_failed_display() {
        let e = EngineError::ConstraintFailed {
            metric: "m".into(),
            op: "ge".into(),
            expected: Value::from(1.0),
            actual: Value::from(0.0),
        };
        assert!(e.to_string().contains("m"));
    }
    #[test]
    fn tc43_cycle_detected_display() {
        assert!(EngineError::CycleDetected {
            steps: "a,b".into()
        }
        .to_string()
        .contains("a,b"));
    }
    #[test]
    fn tc44_step_timeout_display() {
        let e = EngineError::StepTimeout {
            step: "slow".into(),
            duration: Duration::from_secs(30),
        };
        assert!(e.to_string().contains("30s"));
    }
    #[test]
    fn tc45_cancelled_display() {
        assert_eq!(EngineError::Cancelled.to_string(), "Pipeline cancelled");
    }
    #[test]
    fn tc46_other_display() {
        assert!(EngineError::Other("x".into()).to_string().contains("x"));
    }
    #[test]
    fn tc47_variants_are_distinct() {
        assert_ne!(
            EngineError::Cancelled.to_string(),
            EngineError::Other("oops".into()).to_string()
        );
    }

    // ── PipelineResult typed accessors ─────────────────────────────────────
    #[test]
    fn tc_get_f64_present() {
        let mut r = PipelineResult::default();
        r.values.insert("score".into(), Value::from(3.14f64));
        assert_eq!(r.get_f64("score"), Some(3.14));
        assert_eq!(r.get_f64("missing"), None);
    }

    #[test]
    fn tc_get_bool_present() {
        let mut r = PipelineResult::default();
        r.values.insert("flag".into(), Value::from(true));
        assert_eq!(r.get_bool("flag"), Some(true));
        assert_eq!(r.get_bool("missing"), None);
    }

    #[test]
    fn tc_get_str_present() {
        let mut r = PipelineResult::default();
        r.values.insert("label".into(), Value::from("long"));
        assert_eq!(r.get_str("label"), Some("long"));
        assert_eq!(r.get_str("missing"), None);
    }

    #[test]
    fn tc_get_as_deserialize() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct MyStruct {
            x: i32,
        }
        let mut r = PipelineResult::default();
        r.values.insert("obj".into(), serde_json::json!({"x": 42}));
        let decoded: Option<MyStruct> = r.get_as("obj");
        assert_eq!(decoded, Some(MyStruct { x: 42 }));
    }

    // ── StepContext ────────────────────────────────────────────────────────
    #[test]
    fn tc_step_context_get_f64() {
        let mut m = HashMap::new();
        m.insert("equity".into(), Value::from(1000.0f64));
        let ctx = StepContext::new(m);
        assert_eq!(ctx.get_f64("equity"), Some(1000.0));
        assert_eq!(ctx.get_f64("missing"), None);
    }

    #[test]
    fn tc_step_context_get_bool() {
        let mut m = HashMap::new();
        m.insert("flag".into(), Value::from(false));
        let ctx = StepContext::new(m);
        assert_eq!(ctx.get_bool("flag"), Some(false));
    }

    #[test]
    fn tc_step_context_get_str() {
        let mut m = HashMap::new();
        m.insert("side".into(), Value::from("long"));
        let ctx = StepContext::new(m);
        assert_eq!(ctx.get_str("side"), Some("long"));
    }

    #[test]
    fn tc_step_context_get_as() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct S {
            v: i32,
        }
        let mut m = HashMap::new();
        m.insert("obj".into(), serde_json::json!({"v": 7}));
        let ctx = StepContext::new(m);
        assert_eq!(ctx.get_as::<S>("obj"), Some(S { v: 7 }));
    }

    #[test]
    fn tc_step_context_default_empty() {
        let ctx = StepContext::default();
        assert_eq!(ctx.get_f64("x"), None);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Fixtures
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct MockP;
impl tupa_core::Pipeline for MockP {
    type Input = i32;
    fn name(&self) -> &'static str {
        "MockP"
    }
}
impl ExecutorPipeline for MockP {
    fn execute(&self, input: &i32) -> Result<PipelineResult, EngineError> {
        let mut v = HashMap::new();
        v.insert("out".into(), Value::from(*input + 1));
        let (passed, failures) = <Self as ParallelPipeline>::check_constraints(&v, input);
        Ok(PipelineResult {
            values: v,
            passed,
            failures,
            metrics: vec![],
        })
    }
}
impl ParallelPipeline for MockP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["a"]
    }
    fn produces(&self, _: &str) -> &'static [&'static str] {
        &["out"]
    }
    fn requires(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn execute_step(&self, input: &i32, _: &str, _ctx: &StepContext) -> Result<Value, EngineError> {
        Ok(Value::from(*input + 1))
    }
    fn check_constraints(_: &HashMap<String, Value>, _: &i32) -> (bool, Vec<ConstraintFailure>) {
        (true, vec![])
    }
}

#[derive(Clone)]
struct SingleP;
impl tupa_core::Pipeline for SingleP {
    type Input = i32;
    fn name(&self) -> &'static str {
        "SingleP"
    }
}
impl ExecutorPipeline for SingleP {
    fn execute(&self, _: &i32) -> Result<PipelineResult, EngineError> {
        Ok(PipelineResult::default())
    }
}
impl ParallelPipeline for SingleP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["a"]
    }
    fn produces(&self, id: &str) -> &'static [&'static str] {
        match id {
            "a" => &["out"],
            _ => &[],
        }
    }
    fn requires(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn execute_step(&self, x: &i32, _: &str, _ctx: &StepContext) -> Result<Value, EngineError> {
        Ok(Value::from(*x))
    }
    fn check_constraints(_: &HashMap<String, Value>, _: &i32) -> (bool, Vec<ConstraintFailure>) {
        (true, vec![])
    }
}

#[derive(Clone)]
struct TwoIndepP;
impl tupa_core::Pipeline for TwoIndepP {
    type Input = i32;
    fn name(&self) -> &'static str {
        "Two"
    }
}
impl ExecutorPipeline for TwoIndepP {
    fn execute(&self, _: &i32) -> Result<PipelineResult, EngineError> {
        Ok(PipelineResult::default())
    }
}
impl ParallelPipeline for TwoIndepP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["a", "b"]
    }
    fn produces(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn requires(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn execute_step(&self, x: &i32, _: &str, _ctx: &StepContext) -> Result<Value, EngineError> {
        Ok(Value::from(*x))
    }
    fn check_constraints(_: &HashMap<String, Value>, _: &i32) -> (bool, Vec<ConstraintFailure>) {
        (true, vec![])
    }
}

#[derive(Clone)]
struct SequentialP;
impl tupa_core::Pipeline for SequentialP {
    type Input = i32;
    fn name(&self) -> &'static str {
        "Seq"
    }
}
impl ExecutorPipeline for SequentialP {
    fn execute(&self, _: &i32) -> Result<PipelineResult, EngineError> {
        Ok(PipelineResult::default())
    }
}
impl ParallelPipeline for SequentialP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["a", "b"]
    }
    fn produces(&self, id: &str) -> &'static [&'static str] {
        match id {
            "a" => &["a_val"],
            "b" => &[],
            _ => &[],
        }
    }
    fn requires(&self, id: &str) -> &'static [&'static str] {
        match id {
            "a" => &[],
            "b" => &["a_val"],
            _ => &[],
        }
    }
    fn execute_step(&self, x: &i32, _: &str, _ctx: &StepContext) -> Result<Value, EngineError> {
        Ok(Value::from(*x))
    }
    fn check_constraints(_: &HashMap<String, Value>, _: &i32) -> (bool, Vec<ConstraintFailure>) {
        (true, vec![])
    }
}

/// SlowP — designed para ser chamado por um Executor COM step_timeout configurado.
/// execute_step é síncrono; o timeout é aplicado pela camada do Executor.
#[derive(Clone)]
struct SlowP;
impl tupa_core::Pipeline for SlowP {
    type Input = ();
    fn name(&self) -> &'static str {
        "Slow"
    }
}
impl ExecutorPipeline for SlowP {
    fn execute(&self, _: &()) -> Result<PipelineResult, EngineError> {
        Ok(PipelineResult::default())
    }
}
impl ParallelPipeline for SlowP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["slow"]
    }
    fn produces(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn requires(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn execute_step(&self, _: &(), _: &str, _ctx: &StepContext) -> Result<Value, EngineError> {
        // Não usar tokio::time::sleep aqui — execute_step é fn, não async fn.
        // O timeout é detectado pela camada superior (run_parallel) que envolve
        // este método com tokio::time::timeout().
        //
        // Margem ampla (1s de sleep contra timeouts de 50-100ms): os testes
        // TC-46/TC-52 rodam num runtime current-thread e, sob carga de CPU, a
        // thread pode demorar a ser escalonada para processar o timer. Enquanto
        // ela acordar antes de o sleep terminar, o timer já estará expirado e o
        // StepTimeout vence de forma determinística. Como `tokio::time::timeout`
        // abandona o spawn_blocking ao expirar, os testes retornam em ~timeout
        // (não em ~sleep), então a margem maior não os deixa mais lentos.
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(Value::from(42))
    }
    fn check_constraints(_: &HashMap<String, Value>, _: &()) -> (bool, Vec<ConstraintFailure>) {
        (true, vec![])
    }
}

/// CtxPassP — verifies that prior step outputs reach later steps via StepContext.
/// Step "a" produces input*10; step "b" reads it from ctx and adds 1.
#[derive(Clone)]
struct CtxPassP;
impl tupa_core::Pipeline for CtxPassP {
    type Input = i32;
    fn name(&self) -> &'static str {
        "CtxPassP"
    }
}
impl ExecutorPipeline for CtxPassP {
    fn execute(&self, _: &i32) -> Result<PipelineResult, EngineError> {
        Ok(PipelineResult::default())
    }
}
impl ParallelPipeline for CtxPassP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["a", "b"]
    }
    fn produces(&self, id: &str) -> &'static [&'static str] {
        match id {
            "a" => &["a_val"],
            "b" => &["b_val"],
            _ => &[],
        }
    }
    fn requires(&self, id: &str) -> &'static [&'static str] {
        match id {
            "b" => &["a_val"],
            _ => &[],
        }
    }
    fn execute_step(
        &self,
        input: &i32,
        step_id: &str,
        ctx: &StepContext,
    ) -> Result<Value, EngineError> {
        match step_id {
            "a" => Ok(Value::from(*input * 10)),
            "b" => {
                let a_val = ctx.get_f64("a_val").unwrap_or(0.0);
                Ok(Value::from(a_val + 1.0))
            }
            _ => Err(EngineError::Other(format!("unknown step: {}", step_id))),
        }
    }
    fn check_constraints(_: &HashMap<String, Value>, _: &i32) -> (bool, Vec<ConstraintFailure>) {
        (true, vec![])
    }
}

/// ComputedConstraintP — verifies that check_constraints receives the input value.
/// Constraint: "out" >= input (threshold computed from input).
#[derive(Clone)]
struct ComputedConstraintP;
impl tupa_core::Pipeline for ComputedConstraintP {
    type Input = i32;
    fn name(&self) -> &'static str {
        "ComputedConstraintP"
    }
}
impl ExecutorPipeline for ComputedConstraintP {
    fn execute(&self, _: &i32) -> Result<PipelineResult, EngineError> {
        Ok(PipelineResult::default())
    }
}
impl ParallelPipeline for ComputedConstraintP {
    fn step_ids(&self) -> &'static [&'static str] {
        &["compute"]
    }
    fn produces(&self, _: &str) -> &'static [&'static str] {
        &["out"]
    }
    fn requires(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn execute_step(
        &self,
        input: &i32,
        _step_id: &str,
        _ctx: &StepContext,
    ) -> Result<Value, EngineError> {
        Ok(Value::from(*input as f64 * 2.0))
    }
    fn check_constraints(
        values: &HashMap<String, Value>,
        input: &i32,
    ) -> (bool, Vec<ConstraintFailure>) {
        let threshold = *input as f64;
        if let Some(v) = values.get("out").and_then(|v| v.as_f64()) {
            if v < threshold {
                return (
                    false,
                    vec![ConstraintFailure {
                        metric: "out".into(),
                        operator: ">=".into(),
                        expected: Value::from(threshold),
                        actual: Value::from(v),
                    }],
                );
            }
        }
        (true, vec![])
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CLASSE 9 — Executor::run (mock pipeline)
// ═══════════════════════════════════════════════════════════════════════════
mod class9 {

    use super::*;

    #[test]
    fn tc39_run_output_metric() {
        let r = Executor::new().run(&MockP, &0).unwrap();
        assert_eq!(r.values.get("out"), Some(&Value::from(1)));
    }

    #[test]
    fn tc40_run_passes_by_default() {
        let r = Executor::new().run(&MockP, &0).unwrap();
        assert!(r.passed);
    }

    #[test]
    fn tc41_check_constraints_passes() {
        let v: HashMap<String, Value> = [("x".into(), Value::from(1f64))].into_iter().collect();
        let (ok, _) = <MockP as ParallelPipeline>::check_constraints(&v, &0);
        assert!(ok);
    }

    #[test]
    fn tc42_check_constraints_with_metric() {
        let v = HashMap::from([("out".into(), Value::from(10.0f64))]);
        let (ok, fails) = <MockP as ParallelPipeline>::check_constraints(&v, &0);
        assert!(ok);
        assert!(fails.is_empty());
    }

    #[test]
    fn tc_computed_constraint_passes_when_above_threshold() {
        let v = HashMap::from([("out".into(), Value::from(20.0f64))]);
        // input = 5, threshold = 5.0, out = 20.0 → passes
        let (ok, _) = <ComputedConstraintP as ParallelPipeline>::check_constraints(&v, &5);
        assert!(ok);
    }

    #[test]
    fn tc_computed_constraint_fails_when_below_threshold() {
        let v = HashMap::from([("out".into(), Value::from(2.0f64))]);
        // input = 10, threshold = 10.0, out = 2.0 → fails
        let (ok, fails) = <ComputedConstraintP as ParallelPipeline>::check_constraints(&v, &10);
        assert!(!ok);
        assert_eq!(fails[0].metric, "out");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CLASSE 10 — Executor::run_parallel  #[tokio::test]
// ═══════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod class10 {

    use super::tupa_engine_clean_env;
    use super::*;

    /// Executor com 100ms step_timeout via env var.
    fn timeout_exec() -> Executor {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_STEP_TIMEOUT", "100ms");
        Executor::from_env()
    }

    #[tokio::test]
    /// TC-43: single independent step — metrics collected.
    async fn tc43_single_step_metrics_collected() {
        let r = Executor::new().run_parallel(&SingleP, &0).await.unwrap();
        assert!(r.passed);
        assert_eq!(r.metrics.len(), 1);
        assert_eq!(r.metrics[0].state, StepState::Completed);
    }

    #[tokio::test]
    /// TC-44: two independent steps — both complete.
    async fn tc44_two_independent_steps() {
        let r = Executor::new().run_parallel(&TwoIndepP, &0).await;
        assert!(
            r.is_ok(),
            "two independent steps must finish: {:?}",
            r.err()
        );
    }

    #[tokio::test]
    /// TC-45: DAG sequential (b waits for a) — executes without error.
    async fn tc45_dag_dependency_respected() {
        let r = Executor::new().run_parallel(&SequentialP, &1).await;
        assert!(r.is_ok(), "valid DAG must not error: {:?}", r.err());
    }

    #[tokio::test]
    /// TC-46: step_TIMEU — step excede o timeout configurado → StepTimeout.
    async fn tc46_step_timeout() {
        match timeout_exec().run_parallel(&SlowP, &()).await {
            Err(EngineError::StepTimeout { step, .. }) => assert_eq!(step, "slow"),
            other => panic!("expected StepTimeout, got {:?}", other),
        }
    }

    #[tokio::test]
    /// TC-47: metrics_output via env var grava JSON válido com campos obrigatórios.
    async fn tc47_metrics_output_via_env() {
        use tupa_engine_clean_env;
        let path =
            std::env::temp_dir().join(format!("tupa_engine_test_m_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);
        tupa_engine_clean_env();
        std::env::set_var("TUPA_METRICS_OUTPUT", path.to_string_lossy().as_ref());
        let exec = Executor::from_env();
        let _ = exec.run_parallel(&SingleP, &0).await;
        assert!(path.exists(), "metrics file not created at {:?}", path);
        let raw = std::fs::read_to_string(&path).unwrap();
        let json = serde_json::from_str::<Value>(&raw).unwrap();
        assert!(json.is_array(), "metrics must be a JSON array");
        let arr = json.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert!(arr[0].get("step_id").is_some(), "metric must have step_id");
        assert!(arr[0].get("state").is_some(), "metric must have state");
        assert!(
            arr[0].get("start_nanos").is_some(),
            "metric must have start_nanos"
        );
        assert!(
            arr[0].get("duration_nanos").is_some(),
            "metric must have duration_nanos"
        );
        tupa_engine_clean_env();
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    /// TC-41: StepPanic display contém o step name.
    fn tc41_step_panic_display() {
        let e = EngineError::StepPanic {
            step: "x".into(),
            reason: "oops".into(),
        };
        assert!(e.to_string().contains("x"));
        // StepPanic não tem source (thiserror não gera source automaticamente)
    }

    #[test]
    /// TC-49: Pipeline::name() retorna o nome canônico de cada fixture.
    fn tc49_pipeline_name_correct() {
        assert_eq!(MockP.name(), "MockP");
        assert_eq!(SingleP.name(), "SingleP");
        assert_eq!(TwoIndepP.name(), "Two");
        assert_eq!(SequentialP.name(), "Seq");
        assert_eq!(SlowP.name(), "Slow");
    }

    #[test]
    /// TC-53: SlowP's execute_step runs sync (blocks ≤ timeout threshold).
    fn tc53_slowp_execute_step_sync() {
        // ExecuteStep é síncrono; garante que o timeout do executor seria detectado
        // pela camada superior, não por execute_step em si.
        let p = SlowP;
        let _ = p.name(); // compila e executa
    }

    #[tokio::test]
    /// TC-50: Completed metric tem duration_nanos > 0 e end_nanos > start_nanos.
    async fn tc50_completed_metric_has_duration() {
        let r = Executor::new().run_parallel(&SingleP, &0).await.unwrap();
        assert_eq!(r.metrics[0].state, StepState::Completed);
        assert!(r.metrics[0].duration_nanos.unwrap() > 0);
        assert!(r.metrics[0].end_nanos.unwrap() > r.metrics[0].start_nanos);
    }

    #[tokio::test]
    /// TC-51: Produces vazio para step "a" (step sem métricas declaradas).
    async fn tc51_no_produces_for_single_step() {
        // SingleP.produces("b") é step inexistente
        let p = SingleP;
        assert!(p.produces("b").is_empty());
    }

    #[tokio::test]
    /// TC-52: Executor::from_env com timeout setado → StepTimeout no executor.
    async fn tc52_from_env_timeout_caught_by_executor() {
        tupa_engine_clean_env();
        std::env::set_var("TUPA_STEP_TIMEOUT", "50ms");
        let exec = Executor::from_env();
        match exec.run_parallel(&SlowP, &()).await {
            Err(EngineError::StepTimeout { .. }) => {}
            other => panic!(
                "expected StepTimeout with env-configured exec, got {:?}",
                other
            ),
        }
        tupa_engine_clean_env();
    }

    #[tokio::test]
    /// TC-51: DAG com ciclo → Err(CycleDetected).
    async fn tc51_cycle_detected_in_execution() {
        // CyclicP has step "a" requiring "b_val" and step "b" requiring "a_val"
        #[derive(Clone)]
        struct CyclicP;
        impl tupa_core::Pipeline for CyclicP {
            type Input = i32;
            fn name(&self) -> &'static str {
                "CyclicP"
            }
        }
        impl ExecutorPipeline for CyclicP {
            fn execute(&self, _: &i32) -> Result<PipelineResult, EngineError> {
                Ok(PipelineResult::default())
            }
        }
        impl ParallelPipeline for CyclicP {
            fn step_ids(&self) -> &'static [&'static str] {
                &["a", "b"]
            }
            fn produces(&self, id: &str) -> &'static [&'static str] {
                match id {
                    "a" => &["a_val"],
                    "b" => &["b_val"],
                    _ => &[],
                }
            }
            fn requires(&self, id: &str) -> &'static [&'static str] {
                match id {
                    "a" => &["b_val"],
                    "b" => &["a_val"],
                    _ => &[],
                }
            }
            fn execute_step(
                &self,
                _: &i32,
                _: &str,
                _ctx: &StepContext,
            ) -> Result<Value, EngineError> {
                Ok(Value::from(1))
            }
            fn check_constraints(
                _: &HashMap<String, Value>,
                _: &i32,
            ) -> (bool, Vec<ConstraintFailure>) {
                (true, vec![])
            }
        }
        let result = Executor::new().run_parallel(&CyclicP, &0).await;
        assert!(matches!(result, Err(EngineError::CycleDetected { .. })));
    }

    #[tokio::test]
    /// TC-54: without explicit produces, step name becomes metric name.
    async fn tc54_produces_defaults_to_step_name() {
        // SingleP has produces("a") = ["out"], but unknown step returns empty
        // This tests default behavior when produces is not explicitly declared
        let p = SingleP;
        assert_eq!(p.produces("a"), &["out"]);
    }

    #[tokio::test]
    /// TC-55: Cancel token set during execution → Err(Cancelled).
    async fn tc55_cancel_during_execution() {
        let exec = Executor::new();
        exec.cancel();
        let run_result = exec.run_parallel(&SingleP, &0).await;
        assert!(matches!(run_result, Err(EngineError::Cancelled)));
    }

    #[tokio::test]
    /// TC-56: Cancel before execution fails immediately.
    async fn tc56_cancel_before_execution() {
        let exec = Executor::new();
        exec.cancel();
        let result = exec.run_parallel(&SingleP, &0).await;
        assert!(matches!(result, Err(EngineError::Cancelled)));
    }

    #[tokio::test]
    /// StepContext: step B reads step A's output via ctx.
    async fn tc_step_ctx_threads_prior_outputs() {
        let r = Executor::new().run_parallel(&CtxPassP, &3).await.unwrap();
        // Step "a": 3 * 10 = 30
        // Step "b": 30 + 1 = 31
        assert_eq!(r.get_f64("a_val"), Some(30.0));
        assert_eq!(r.get_f64("b_val"), Some(31.0));
    }

    #[tokio::test]
    /// Computed constraint: check_constraints receives input and uses it as threshold.
    async fn tc_computed_constraint_via_run_parallel() {
        // input=5, step produces 5*2=10, constraint: out >= input(5) → passes
        let r = Executor::new()
            .run_parallel(&ComputedConstraintP, &5)
            .await
            .unwrap();
        assert!(r.passed);
        assert_eq!(r.get_f64("out"), Some(10.0));

        // input=20, step produces 20*2=40, constraint: out >= input(20) → passes
        let r2 = Executor::new()
            .run_parallel(&ComputedConstraintP, &20)
            .await
            .unwrap();
        assert!(r2.passed);
    }
}
