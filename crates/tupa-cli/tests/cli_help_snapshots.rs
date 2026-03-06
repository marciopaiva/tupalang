use std::process::Command;

fn run_help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_tupa"))
        .args(args)
        .output()
        .expect("failed to run tupa");

    assert!(output.status.success(), "tupa --help failed");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn help_root_snapshot() {
    let out = run_help(&["--help"]);
    let expected = include_str!("snapshots/help_root.txt");
    assert_eq!(out, expected);
}

#[test]
fn help_check_snapshot() {
    let out = run_help(&["check", "--help"]);
    let expected = include_str!("snapshots/help_check.txt");
    assert_eq!(out, expected);
}

#[test]
fn help_run_snapshot() {
    let out = run_help(&["run", "--help"]);
    let expected = include_str!("snapshots/help_run.txt");
    assert_eq!(out, expected);
}

#[test]
fn help_codegen_snapshot() {
    let out = run_help(&["codegen", "--help"]);
    let expected = include_str!("snapshots/help_codegen.txt");
    assert_eq!(out, expected);
}

#[test]
fn help_audit_snapshot() {
    let out = run_help(&["audit", "--help"]);
    let expected = include_str!("snapshots/help_audit.txt");
    assert_eq!(out, expected);
}
