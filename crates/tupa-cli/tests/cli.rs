use std::path::Path;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap()
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
