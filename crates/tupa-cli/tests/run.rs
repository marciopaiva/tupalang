use assert_cmd::Command;
use std::time::Instant;

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run_e2e_enabled() -> bool {
    std::env::var("TUPA_RUN_E2E").as_deref() == Ok("1")
}

#[test]
fn run_fraud_detection_plan_only_and_run() {
    if !run_e2e_enabled() {
        eprintln!("skipping run_fraud_detection_plan_only_and_run (set TUPA_RUN_E2E=1)");
        return;
    }

    let root = repo_root();
    let mut gen = Command::new(env!("CARGO_BIN_EXE_tupa-cli"));
    gen.current_dir(&root)
        .args([
            "codegen",
            "--plan-only",
            "examples/pipeline/fraud_complete.tp",
        ])
        .assert()
        .success();

    let mut run = Command::new(env!("CARGO_BIN_EXE_tupa-cli"));
    run.current_dir(&root)
        .args([
            "run",
            "--plan",
            "fraud_complete.plan.json",
            "--pipeline",
            "FraudDetection",
            "--input",
            "examples/pipeline/tx.json",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("\"status\": \"pass\""));
}

#[test]
fn run_credit_decision_report_contains_approved() {
    if !run_e2e_enabled() {
        eprintln!("skipping run_credit_decision_report_contains_approved (set TUPA_RUN_E2E=1)");
        return;
    }

    let root = repo_root();
    let tmp_path = root.join("tmp_input_i64.json");
    std::fs::write(&tmp_path, "100").unwrap();

    let mut run = Command::new(env!("CARGO_BIN_EXE_tupa-cli"));
    run.current_dir(&root)
        .args([
            "run",
            "--pipeline",
            "CreditDecision",
            "--input",
            tmp_path.to_str().unwrap(),
            "examples/pipeline/credit_decision.tp",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("\"status\": \"pass\""));

    let _ = std::fs::remove_file(&tmp_path);
}

#[test]
fn perf_codegen_fraud_medium_under_target() {
    let root = repo_root();
    let start = Instant::now();
    let mut gen = Command::new(env!("CARGO_BIN_EXE_tupa-cli"));
    gen.current_dir(&root)
        .args(["codegen", "examples/pipeline/fraud_complete.tp"])
        .assert()
        .success();

    let elapsed = start.elapsed();
    println!(
        "perf: codegen fraud_complete took {} ms",
        elapsed.as_millis()
    );
    assert!(
        elapsed.as_millis() <= 500,
        "codegen took {} ms (> 500ms threshold)",
        elapsed.as_millis()
    );
}

#[test]
#[ignore]
fn perf_run_fraud_medium_under_target() {
    let root = repo_root();
    let start = Instant::now();
    let mut run = Command::new(env!("CARGO_BIN_EXE_tupa-cli"));
    run.current_dir(&root)
        .args([
            "run",
            "--pipeline",
            "FraudDetection",
            "--input",
            "examples/pipeline/tx.json",
            "examples/pipeline/fraud_complete.tp",
        ])
        .assert()
        .success();

    let elapsed = start.elapsed();
    println!("perf: run fraud_complete took {} ms", elapsed.as_millis());
    assert!(
        elapsed.as_millis() <= 500,
        "run took {} ms (> 500ms threshold)",
        elapsed.as_millis()
    );
}
