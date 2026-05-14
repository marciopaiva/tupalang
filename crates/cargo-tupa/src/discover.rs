use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Discover the main binary target in a Cargo package.
///
/// Returns the binary name (as defined in [[bin]] or inferred from src/main.rs).
pub fn discover_binary_target(manifest_path: &PathBuf) -> Result<String> {
    let manifest_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let src_dir = manifest_dir.join("src");

    let toml_content =
        std::fs::read_to_string(manifest_path).context("Failed to read Cargo.toml")?;

    if let Some(bin_name) = parse_bin_names(&toml_content) {
        return Ok(bin_name);
    }

    if src_dir.join("main.rs").exists() {
        return extract_package_name(&toml_content).ok_or_else(|| {
            anyhow::anyhow!("Cannot infer binary name: missing package.name in Cargo.toml")
        });
    }

    anyhow::bail!(
        "No suitable binary target found. Create [[bin]] in Cargo.toml or use src/main.rs"
    )
}

fn parse_bin_names(toml: &str) -> Option<String> {
    let mut in_bin_section = false;
    for line in toml.lines() {
        let line = line.trim();
        if line.starts_with("[[bin]]") {
            in_bin_section = true;
            continue;
        }
        if in_bin_section {
            if line.starts_with('[') && !line.starts_with("[[bin]]") {
                in_bin_section = false;
                continue;
            }
            if line.starts_with("name =") {
                let quote = line.chars().find(|c| *c == '"' || *c == '\'')?;
                let name = line.split(quote).nth(1)?;
                return Some(name.to_string());
            }
        }
    }
    None
}

fn extract_package_name(toml: &str) -> Option<String> {
    for line in toml.lines() {
        let line = line.trim();
        if line.starts_with("name =") {
            let quote = line.chars().find(|c| *c == '"' || *c == '\'')?;
            let rest = line.split(quote).nth(1)?;
            return Some(rest.to_string());
        }
    }
    None
}

/// Build the binary with cargo and return path to executable.
pub fn build_binary(manifest_path: &PathBuf, bin_name: &str) -> Result<PathBuf> {
    let status = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--bin")
        .arg(bin_name)
        .status()
        .context("Failed to run cargo build")?;

    if !status.success() {
        anyhow::bail!("cargo build failed");
    }

    let target_dir = manifest_path
        .parent()
        .unwrap()
        .join("target")
        .join("release");
    #[cfg(windows)]
    let binary_path = target_dir.join(format!("{}.exe", bin_name));
    #[cfg(not(windows))]
    let binary_path = target_dir.join(bin_name);

    if !binary_path.exists() {
        anyhow::bail!("Binary not found at {}", binary_path.display());
    }

    Ok(binary_path)
}

/// Execute the built binary with the given arguments.
///
/// If `metrics_output` is provided, sets `TUPA_METRICS_OUTPUT` environment variable
/// so the pipeline can write metrics JSON to that path.
pub fn execute_binary(
    binary_path: &PathBuf,
    input: Option<&PathBuf>,
    parallel: bool,
    metrics_output: Option<&PathBuf>,
) -> Result<()> {
    let mut cmd = std::process::Command::new(binary_path);
    cmd.arg("run");

    if let Some(ref input_file) = input {
        cmd.arg("--input").arg(input_file);
    }

    if parallel {
        cmd.arg("--parallel");
    }

    // Pass metrics output path via environment variable
    if let Some(ref path) = metrics_output {
        cmd.env("TUPA_METRICS_OUTPUT", path);
    }

    let output = cmd.output().context("Failed to execute pipeline binary")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Pipeline execution failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_discover_binary_from_main_rs() -> Result<()> {
        let tmp = tempfile::TempDir::new()?;
        let project = tmp.path();

        fs::write(
            project.join("Cargo.toml"),
            r#"[package]
name = "test-pipeline"
version = "0.1.0"
edition = "2021"
"#,
        )?;

        fs::create_dir_all(project.join("src"))?;
        fs::write(
            project.join("src").join("main.rs"),
            r#"use tupa_core::pipeline;

pipeline! {
    name: TestPipe,
    input: String,
    steps: [
        step("hello") { println!("Hello, {}!", input) }
    ],
    constraints: []
}
"#,
        )?;

        let bin_name = discover_binary_target(&project.join("Cargo.toml"))?;
        assert_eq!(bin_name, "test-pipeline");
        Ok(())
    }

    #[test]
    fn test_discover_binary_from_bin_section() -> Result<()> {
        let tmp = tempfile::TempDir::new()?;
        let project = tmp.path();

        fs::write(
            project.join("Cargo.toml"),
            r#"[package]
name = "my-pkg"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "custom-bin"
path = "src/bin/custom.rs"
"#,
        )?;

        fs::create_dir_all(project.join("src").join("bin"))?;
        fs::write(
            project.join("src").join("bin").join("custom.rs"),
            r#"fn main() {}
"#,
        )?;

        let bin_name = discover_binary_target(&project.join("Cargo.toml"))?;
        assert_eq!(bin_name, "custom-bin");
        Ok(())
    }
}
