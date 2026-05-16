use assert_cmd::Command;
use std::path::PathBuf;

#[test]
fn test_watch_help_works() {
    // Verify watch command exists via help
    let output = Command::cargo_bin("cargo-tupa")
        .unwrap()
        .arg("--help")
        .output()
        .expect("failed to get help");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Watch"), "help should show watch command");
}

#[test]
fn test_discover_works_for_pipeline() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("pipeline");

    // Verify discover works (smoke test for subcommands)
    let output = Command::cargo_bin("cargo-tupa")
        .unwrap()
        .arg("discover")
        .arg("--manifest-path")
        .arg(fixture_dir.join("Cargo.toml"))
        .output()
        .expect("failed to execute cargo-tupa discover");

    assert!(output.status.success(), "discover should work");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "test-pipeline");
}

#[test]
fn test_expand_works_for_pipeline() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("pipeline");

    let output = Command::cargo_bin("cargo-tupa")
        .unwrap()
        .arg("expand")
        .arg("--manifest-path")
        .arg(fixture_dir.join("Cargo.toml"))
        .output()
        .expect("failed to execute cargo-tupa expand");

    assert!(output.status.success(), "expand should work");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("impl"), "expand output should contain impl");
}
