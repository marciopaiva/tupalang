use std::path::Path;

use assert_cmd::Command;
use predicates::str::contains;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap()
}

#[test]
fn lex_outputs_tokens() {
    Command::cargo_bin("tupa-cli")
        .unwrap()
        .current_dir(repo_root())
        .args(["lex", "examples/hello.tp"])
        .assert()
        .success()
        .stdout(contains("Fn"));
}

#[test]
fn parse_outputs_ast() {
    Command::cargo_bin("tupa-cli")
        .unwrap()
        .current_dir(repo_root())
        .args(["parse", "examples/hello.tp"])
        .assert()
        .success()
        .stdout(contains("Program"));
}

#[test]
fn check_outputs_ok() {
    Command::cargo_bin("tupa-cli")
        .unwrap()
        .current_dir(repo_root())
        .args(["check", "examples/hello.tp"])
        .assert()
        .success()
        .stdout(contains("OK"));
}
