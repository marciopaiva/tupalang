use anyhow::Result;
use std::path::PathBuf;

/// Run `cargo tupa test` — execute pipeline integration tests.
pub fn run(_manifest_path: &Option<PathBuf>, _filter: Option<String>) -> Result<()> {
    anyhow::bail!(
        "cargo tupa test not yet implemented.\n\
         Planned for 0.9.3 — discover and run pipeline integration tests."
    )
}
