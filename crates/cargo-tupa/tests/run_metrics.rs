use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_cargo_tupa_run_with_metrics_output() {
    // Locate the test fixture (pipeline example with proper dependencies)
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("pipeline");

    let metrics_file = fixture_dir.parent().unwrap().join("metrics.json");

    // Run cargo tupa with metrics output
    let output = Command::cargo_bin("cargo-tupa")
        .unwrap()
        .arg("run")
        .arg("--manifest-path")
        .arg(fixture_dir.join("Cargo.toml"))
        .arg("--metrics-output")
        .arg(&metrics_file)
        .output()
        .expect("failed to execute cargo-tupa");

    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();

    eprintln!("stdout: {}", stdout);
    eprintln!("stderr: {}", stderr);

    assert!(output.status.success(), "cargo tupa run failed: {}", stderr);
    assert!(
        metrics_file.exists(),
        "metrics file not created at {:?}",
        metrics_file
    );

    let metrics_content = fs::read_to_string(&metrics_file).unwrap();
    let metrics: Value = serde_json::from_str(&metrics_content).expect("invalid JSON");
    assert!(metrics.is_array(), "metrics should be an array");
    let arr = metrics.as_array().unwrap();
    assert_eq!(arr.len(), 2, "should have 2 step metrics");
    // Check that each metric has required fields
    for m in arr {
        assert!(m.get("step_id").is_some(), "missing step_id");
        assert!(m.get("state").is_some(), "missing state");
        assert!(m.get("duration_nanos").is_some(), "missing duration_nanos");
        assert!(m.get("start_nanos").is_some(), "missing start_nanos");
    }
}
