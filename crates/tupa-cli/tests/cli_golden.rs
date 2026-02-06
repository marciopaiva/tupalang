use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn expected_path(name: &str) -> PathBuf {
    workspace_root().join("examples").join("expected").join(name)
}

fn example_path(name: &str) -> PathBuf {
    let path = workspace_root().join("examples").join(name);
    path.canonicalize().unwrap_or(path)
}

fn run_cli(args: &[&str]) -> String {
    let exe = env!("CARGO_BIN_EXE_tupa-cli");
    let output = Command::new(exe)
        .args(args)
        .output()
        .expect("failed to run tupa-cli");
    assert!(output.status.success(), "tupa-cli failed: {output:?}");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_cli_err(args: &[&str]) -> String {
    let exe = env!("CARGO_BIN_EXE_tupa-cli");
    let output = Command::new(exe)
        .args(args)
        .output()
        .expect("failed to run tupa-cli");
    assert!(!output.status.success(), "tupa-cli expected to fail");
    normalize_output(&String::from_utf8_lossy(&output.stderr).to_string())
}

fn normalize_output(text: &str) -> String {
    let root = workspace_root()
        .canonicalize()
        .unwrap_or_else(|_| workspace_root());
    let mut normalized = text.replace(root.to_str().unwrap_or_default(), "");
    normalized = normalized.replace("/examples/", "examples/");
    if normalized.starts_with('/') {
        normalized = normalized.trim_start_matches('/').to_string();
    }
    normalized
}

#[test]
fn golden_lex_hello() {
    let stdout = run_cli(&["lex", example_path("hello.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("lex_hello.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_lex_hello_json() {
    let stdout = run_cli(&[
        "lex",
        "--format",
        "json",
        example_path("hello.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("lex_hello.json")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_parse_hello() {
    let stdout = run_cli(&["parse", example_path("hello.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("parse_hello.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_check_hello() {
    let stdout = run_cli(&["check", example_path("hello.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_hello.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_check_hello_json() {
    let stdout = run_cli(&[
        "check",
        "--format",
        "json",
        example_path("hello.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_hello.json")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_hello() {
    let stdout = run_cli(&["codegen", example_path("hello.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_hello.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_arith() {
    let stdout = run_cli(&["codegen", example_path("arith.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_arith.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_if_match() {
    let stdout = run_cli(&["codegen", example_path("if_match.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_if_match.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_while() {
    let stdout = run_cli(&["codegen", example_path("while.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_while.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_match_string() {
    let stdout = run_cli(&["codegen", example_path("match_string.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_match_string.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_check_invalid_type() {
    let stderr = run_cli_err(&["check", example_path("invalid_type.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_invalid_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_return() {
    let stderr = run_cli_err(&["check", example_path("invalid_return.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_invalid_return.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_call() {
    let stderr = run_cli_err(&["check", example_path("invalid_call.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_invalid_call.txt")).unwrap();
    assert_eq!(stderr, expected);
}
