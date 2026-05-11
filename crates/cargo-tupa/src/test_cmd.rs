use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Run `cargo tupa test` — execute pipeline example tests.
///
/// This runs `cargo test --examples` on the current package by default,
/// ensuring example pipelines are validated. Use `--filter` to select tests.
pub fn run(manifest_path: &Option<PathBuf>, filter: Option<String>) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    cmd.arg("--examples"); // Test example binaries (which contain pipeline logic)

    if let Some(ref path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }

    if let Some(filter) = filter {
        cmd.arg("--").arg(filter);
    }

    let status = cmd
        .status()
        .context("failed to run `cargo test --examples` — is Cargo installed?")?;

    if !status.success() {
        anyhow::bail!(
            "tests failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}
