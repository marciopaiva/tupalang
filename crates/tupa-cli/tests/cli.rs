use std::path::Path;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

#[test]
fn lex_outputs_tokens() {
    let mut cmd = cargo_bin_cmd!("tupa-cli");
    cmd.current_dir(repo_root())
        .args(["lex", "examples/hello.tp"])
        .assert()
        .success()
        .stdout(contains("Fn"));
}

#[test]
fn parse_outputs_ast() {
    let mut cmd = cargo_bin_cmd!("tupa-cli");
    cmd.current_dir(repo_root())
        .args(["parse", "examples/hello.tp"])
        .assert()
        .success()
        .stdout(contains("Program"));
}

#[test]
fn check_outputs_ok() {
    let mut cmd = cargo_bin_cmd!("tupa-cli");
    cmd.current_dir(repo_root())
        .args(["check", "examples/hello.tp"])
        .assert()
        .success()
        .stdout(contains("OK"));
}

#[test]
fn audit_is_deterministic() {
    let root = repo_root();
    let mut first = cargo_bin_cmd!("tupa-cli");
    first.current_dir(root).args([
        "audit",
        "examples/audit_hello.tp",
        "--input",
        "examples/audit_inputs.json",
    ]);
    let first_out = first.output().unwrap();
    assert!(first_out.status.success());

    let mut second = cargo_bin_cmd!("tupa-cli");
    second.current_dir(root).args([
        "audit",
        "examples/audit_hello.tp",
        "--input",
        "examples/audit_inputs.json",
    ]);
    let second_out = second.output().unwrap();
    assert!(second_out.status.success());
    assert_eq!(first_out.stdout, second_out.stdout);
}

#[test]
fn audit_rejects_invalid_inputs() {
    let mut cmd = cargo_bin_cmd!("tupa-cli");
    cmd.current_dir(repo_root())
        .args([
            "audit",
            "examples/audit_hello.tp",
            "--input",
            "examples/audit_inputs_invalid.json",
        ])
        .assert()
        .failure()
        .stderr(contains("expected a JSON array"));
}
