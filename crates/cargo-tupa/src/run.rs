use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

/// Run `cargo tupa run` — executes the pipeline with JSON input.
///
/// Supports running a binary target (`--bin`), an example (`--example`),
/// or the default package binary. Input is passed via TUPA_INPUT environment
/// variable. Parallel mode via TUPA_PARALLEL=1 (or `--parallel` flag).
pub fn run(
    manifest_path: &Option<PathBuf>,
    input_path: Option<PathBuf>,
    parallel: bool,
    example: Option<String>,
    bin: Option<String>,
) -> Result<()> {
    // Read input JSON
    let input_value: Value = match input_path {
        Some(ref path) => {
            let data = fs::read_to_string(path).context("failed to read input file")?;
            serde_json::from_str(&data).context("input is not valid JSON")?
        }
        None => {
            // Read from stdin if available, otherwise default empty object
            let mut buffer = String::new();
            let stdin_available = std::io::stdin().read_to_string(&mut buffer).is_ok();
            if stdin_available && !buffer.is_empty() {
                serde_json::from_str(&buffer).context("stdin input is not valid JSON")?
            } else {
                Value::Object(serde_json::Map::new())
            }
        }
    };

    // Build cargo run command
    let mut cmd = Command::new("cargo");
    cmd.arg("run");

    if let Some(ref path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }

    // Select target: --example or --bin
    if let Some(example_name) = example {
        cmd.arg("--example").arg(example_name);
    } else if let Some(bin_name) = bin {
        cmd.arg("--bin").arg(bin_name);
    }
    // else: default (package's default binary)

    // Pass input via environment variables
    cmd.env("TUPA_INPUT", serde_json::to_string(&input_value)?);
    cmd.env("TUPA_PARALLEL", if parallel { "1" } else { "0" });

    // Execute
    let status = cmd
        .status()
        .context("failed to execute `cargo run` — is the project built?")?;

    if !status.success() {
        anyhow::bail!(
            "pipeline execution failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }

    Ok(())
}
