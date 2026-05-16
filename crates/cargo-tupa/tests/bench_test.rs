use assert_cmd::Command;
use std::path::PathBuf;

#[test]
fn test_bench_runs_successfully() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("pipeline");

    let json_output = fixture_dir.parent().unwrap().join("bench_report.json");

    // Run cargo tupa bench with minimal runs
    let output = Command::cargo_bin("cargo-tupa")
        .unwrap()
        .arg("bench")
        .arg("--manifest-path")
        .arg(fixture_dir.join("Cargo.toml"))
        .arg("--runs")
        .arg("2")
        .arg("--json-output")
        .arg(&json_output)
        .output()
        .expect("failed to execute cargo-tupa bench");

    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();

    eprintln!("stdout: {}", stdout);
    eprintln!("stderr: {}", stderr);

    assert!(
        output.status.success(),
        "cargo tupa bench failed: {}",
        stderr
    );

    // Verify JSON output was created
    assert!(json_output.exists(), "bench JSON not created");

    // Cleanup
    let _ = std::fs::remove_file(&json_output);
}

#[test]
fn test_bench_requires_runs_ge_1() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("pipeline");

    let output = Command::cargo_bin("cargo-tupa")
        .unwrap()
        .arg("bench")
        .arg("--manifest-path")
        .arg(fixture_dir.join("Cargo.toml"))
        .arg("--runs")
        .arg("0")
        .output()
        .expect("failed to execute cargo-tupa bench");

    assert!(!output.status.success(), "bench with runs=0 should fail");
}
