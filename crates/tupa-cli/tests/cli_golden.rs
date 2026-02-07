use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn expected_path(name: &str) -> PathBuf {
    workspace_root()
        .join("examples")
        .join("expected")
        .join(name)
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
fn golden_codegen_for_range() {
    let stdout = run_cli(&["codegen", example_path("for_range.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_for_range.txt")).unwrap();
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
fn golden_codegen_break_continue() {
    let stdout = run_cli(&[
        "codegen",
        example_path("break_continue.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_break_continue.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_array_ops() {
    let stdout = run_cli(&["codegen", example_path("array_ops.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_array_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_bool_print() {
    let stdout = run_cli(&["codegen", example_path("bool_print.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_bool_print.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_bool_ops() {
    let stdout = run_cli(&["codegen", example_path("bool_ops.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_bool_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_unary_ops() {
    let stdout = run_cli(&["codegen", example_path("unary_ops.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_unary_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_pow_ops() {
    let stdout = run_cli(&["codegen", example_path("pow_ops.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_pow_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_string_eq() {
    let stdout = run_cli(&["codegen", example_path("string_eq.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_string_eq.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_match_guard() {
    let stdout = run_cli(&["codegen", example_path("match_guard.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_match_guard.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_function_call() {
    let stdout = run_cli(&[
        "codegen",
        example_path("function_call.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_function_call.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_match_bind() {
    let stdout = run_cli(&["codegen", example_path("match_bind.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_match_bind.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_float_ops() {
    let stdout = run_cli(&["codegen", example_path("float_ops.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_float_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_match_expr() {
    let stdout = run_cli(&["codegen", example_path("match_expr.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_match_expr.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_if_expr() {
    let stdout = run_cli(&["codegen", example_path("if_expr.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_if_expr.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_return_if_expr() {
    let stdout = run_cli(&[
        "codegen",
        example_path("return_if_expr.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_return_if_expr.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_match_guard_if_expr() {
    let stdout = run_cli(&[
        "codegen",
        example_path("match_guard_if_expr.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_match_guard_if_expr.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_string_concat() {
    let stdout = run_cli(&[
        "codegen",
        example_path("string_concat.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_string_concat.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_float_array_ops() {
    let stdout = run_cli(&[
        "codegen",
        example_path("float_array_ops.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_float_array_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_string_plus_eq() {
    let stdout = run_cli(&[
        "codegen",
        example_path("string_plus_eq.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_string_plus_eq.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_string_array_ops() {
    let stdout = run_cli(&[
        "codegen",
        example_path("string_array_ops.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("codegen_string_array_ops.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_lambda_basic() {
    let stdout = run_cli(&["codegen", example_path("lambda_basic.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_lambda_basic.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_codegen_if_unit_expr() {
    let stdout = run_cli(&["codegen", example_path("if_unit_expr.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("codegen_if_unit_expr.txt")).unwrap();
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
