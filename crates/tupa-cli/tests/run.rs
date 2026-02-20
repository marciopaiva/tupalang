use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn run_fraud_detection_plan_only_and_run() {
    let root = repo_root();
    // Generate plan
    let mut gen = Command::cargo_bin("tupa-cli").unwrap();
    gen.current_dir(&root)
        .args([
            "codegen",
            "--plan-only",
            "examples/pipeline/fraud_complete.tp",
        ])
        .assert()
        .success();
    // Run from plan
    let mut run = Command::cargo_bin("tupa-cli").unwrap();
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
    let root = repo_root();
    // create a temp input file with i64
    let tmp_path = root.join("tmp_input_i64.json");
    std::fs::write(&tmp_path, "100").unwrap();
    let mut run = Command::cargo_bin("tupa-cli").unwrap();
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
