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
fn golden_lex_invalid_char() {
    let stderr = run_cli_err(&["lex", example_path("invalid_lex_char.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("lex_invalid_char.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_lex_invalid_char_json() {
    let stderr = run_cli_err(&[
        "lex",
        "--format",
        "json",
        example_path("invalid_lex_char.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("lex_invalid_char.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_parse_hello() {
    let stdout = run_cli(&["parse", example_path("hello.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("parse_hello.txt")).unwrap();
    assert_eq!(stdout, expected);
}

#[test]
fn golden_parse_invalid_missing_semicolon() {
    let stderr = run_cli_err(&[
        "parse",
        example_path("invalid_parse_missing_semicolon.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("parse_invalid_missing_semicolon.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_parse_invalid_missing_semicolon_json() {
    let stderr = run_cli_err(&[
        "parse",
        "--format",
        "json",
        example_path("invalid_parse_missing_semicolon.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("parse_invalid_missing_semicolon.json")).unwrap();
    assert_eq!(stderr, expected);
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
fn golden_check_invalid_unknown_var_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_unknown_var.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unknown_var.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unknown_function_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_unknown_function.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_unknown_function.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unknown_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_unknown_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unknown_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_return() {
    let stderr = run_cli_err(&["check", example_path("invalid_return.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_invalid_return.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_hate_speech() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_safe_hate_speech.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_safe_hate_speech.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_hate_speech_base() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_safe_hate_speech_base.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_hate_speech_base.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_misinformation() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_safe_misinformation.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_misinformation.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_misinformation_base() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_safe_misinformation_base.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_misinformation_base.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_param_base() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_safe_param_base.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_safe_param_base.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_return_base() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_safe_return_base.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_safe_return_base.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_call() {
    let stderr = run_cli_err(&["check", example_path("invalid_call.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_invalid_call.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unknown_var() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_unknown_var.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unknown_var.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unknown_function() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_unknown_function.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unknown_function.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unknown_type() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_unknown_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unknown_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_call_target() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_call_target.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_call_target.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_binary_op() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_binary_op.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_binary_op.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unary_op() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_unary_op.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unary_op.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_break() {
    let stderr = run_cli_err(&["check", example_path("invalid_break.tp").to_str().unwrap()]);
    let expected = fs::read_to_string(expected_path("check_invalid_break.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_continue() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_continue.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_continue.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_print_arity() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_print_arity.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_print_arity.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_match_guard() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_match_guard.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_match_guard.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_match_pattern() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_match_pattern.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_match_pattern.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_match_arm_type() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_match_arm_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_match_arm_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_index_type() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_index_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_index_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_array_mixed() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_array_mixed.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_array_mixed.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_return_type() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_return_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_return_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_assign_type() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_assign_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_assign_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_while_condition() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_while_condition.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_while_condition.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_for_range_type() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_for_range_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_for_range_type.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_range_bounds() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_range_bounds.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_range_bounds.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_if_condition() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_if_condition.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_if_condition.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_assign_index_value() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_assign_index_value.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_assign_index_value.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_index_base() {
    let stderr = run_cli_err(&[
        "check",
        example_path("invalid_index_base.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_index_base.txt")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_return_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_return.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_return.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_hate_speech_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_safe_hate_speech.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_hate_speech.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_hate_speech_base_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_safe_hate_speech_base.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_hate_speech_base.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_misinformation_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_safe_misinformation.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_misinformation.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_misinformation_base_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_safe_misinformation_base.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_misinformation_base.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_param_base_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_safe_param_base.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_safe_param_base.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_safe_return_base_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_safe_return_base.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_safe_return_base.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_call_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_call.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_call.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_call_target_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_call_target.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_call_target.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_binary_op_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_binary_op.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_binary_op.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_unary_op_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_unary_op.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_unary_op.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_break_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_break.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_break.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_continue_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_continue.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_continue.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_print_arity_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_print_arity.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_print_arity.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_match_guard_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_match_guard.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_match_guard.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_match_pattern_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_match_pattern.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_match_pattern.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_match_arm_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_match_arm_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_match_arm_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_index_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_index_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_index_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_array_mixed_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_array_mixed.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_array_mixed.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_return_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_return_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_return_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_assign_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_assign_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_assign_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_while_condition_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_while_condition.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_while_condition.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_for_range_type_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_for_range_type.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_for_range_type.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_range_bounds_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_range_bounds.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_range_bounds.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_if_condition_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_if_condition.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_if_condition.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_assign_index_value_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_assign_index_value.tp")
            .to_str()
            .unwrap(),
    ]);
    let expected =
        fs::read_to_string(expected_path("check_invalid_assign_index_value.json")).unwrap();
    assert_eq!(stderr, expected);
}

#[test]
fn golden_check_invalid_index_base_json() {
    let stderr = run_cli_err(&[
        "check",
        "--format",
        "json",
        example_path("invalid_index_base.tp").to_str().unwrap(),
    ]);
    let expected = fs::read_to_string(expected_path("check_invalid_index_base.json")).unwrap();
    assert_eq!(stderr, expected);
}
