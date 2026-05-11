use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Run `cargo tupa check` — validates pipeline macro expansion and type checking.
///
/// This invokes `cargo check` on the current package and filters for
/// Tupã-related errors. The actual typechecking happens during normal
/// Rust compilation via the `pipeline!` macro.
pub fn run(manifest_path: &Option<PathBuf>, verbose: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");

    cmd.arg("check");
    cmd.arg("--message-format=json");

    if let Some(ref path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }

    if verbose {
        cmd.arg("--verbose");
    }

    let output = cmd
        .output()
        .context("failed to run `cargo check` — is Cargo installed?")?;

    if !output.status.success() {
        // Parse JSON lines and filter for tupa-related errors
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut has_tupa_error = false;

        for line in stderr.lines() {
            if line.contains("tupa_core")
                || line.contains("tupa_engine")
                || line.contains("pipeline")
            {
                eprintln!("{}", line);
                has_tupa_error = true;
            }
        }

        if !has_tupa_error {
            eprintln!("{}", stderr);
        }

        std::process::exit(output.status.code().unwrap_or(1));
    }

    if verbose {
        println!("✅ Pipeline typecheck passed");
    }

    Ok(())
}
