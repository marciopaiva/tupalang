use anyhow::{Result, Context};
use std::path::PathBuf;

use super::discover::{discover_binary_target, build_binary, execute_binary};

/// Run `cargo tupa run` — build and execute the project's pipeline binary.
pub fn run(
    manifest_path: &Option<PathBuf>,
    input: Option<PathBuf>,
    parallel: bool,
) -> Result<()> {
    // Resolve Cargo.toml path
    let manifest = if let Some(ref p) = manifest_path {
        p.clone()
    } else {
        // Search upward for Cargo.toml
        let mut dir = std::env::current_dir().context("Failed to get current dir")?;
        loop {
            if dir.join("Cargo.toml").exists() {
                break;
            }
            let next = dir.parent().map(|p| p.to_path_buf());
            match next {
                Some(ref p) if *p != dir => dir = p.clone(),
                _ => anyhow::bail!("Cargo.toml not found in current directory or parents"),
            }
        }
        dir.join("Cargo.toml")
    };

    if !manifest.exists() {
        anyhow::bail!("Cargo.toml not found at {}", manifest.display());
    }

    println!("📦 Using manifest: {}", manifest.display());

    // Discover binary target
    let bin_name = discover_binary_target(&manifest)
        .context("Failed to discover binary target")?;

    println!("🎯 Target binary: {}", bin_name);

    // Build
    let binary_path = build_binary(&manifest, &bin_name)
        .context("Build failed")?;

    println!("✅ Built: {}", binary_path.display());

    // Execute
    execute_binary(&binary_path, input.as_ref(), parallel)
        .context("Execution failed")?;

    Ok(())
}
