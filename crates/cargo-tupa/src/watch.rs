//! Watch mode: re-run pipeline automatically when source files change.
//!
//! Usage:
//!   cargo tupa watch                      # watch src/, re-run on change
//!   cargo tupa watch --input data.json   # with input file
//!   cargo tupa watch --parallel           # with parallel execution

use anyhow::{Context, Result};
use glob::glob;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use super::run::run;

/// Watch `src/**/*.rs` for changes and re-run the pipeline.
pub fn watch(input: Option<PathBuf>, parallel: bool) -> Result<()> {
    let mut last_digest = compute_src_digest();

    println!("👁️  Watching for changes in src/**/*.rs ... (Ctrl+C to stop)");

    loop {
        thread::sleep(Duration::from_millis(500));
        let current_digest = compute_src_digest();
        if current_digest != last_digest {
            println!("\n📝 Change detected — rebuilding and running...\n");
            if let Err(e) = run(&None, input.clone(), parallel, None) {
                eprintln!("❌ Run failed: {}", e);
            } else {
                println!("\n✅ Run complete. Watching for changes...");
            }
            last_digest = current_digest;
        }
    }
}

/// Compute a simple digest of all `src/**/*.rs` files: (mtime, size) per path.
fn compute_src_digest() -> String {
    let mut entries = Vec::new();
    if let Ok(paths) = glob("src/**/*.rs") {
        for path in paths.flatten() {
            if let Ok(meta) = std::fs::metadata(&path) {
                let mtime = meta
                    .modified()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                entries.push((path.display().to_string(), mtime, meta.len()));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
        .iter()
        .map(|(p, m, s)| format!("{}:{}:{}", p, m, s))
        .collect::<Vec<_>>()
        .join("|")
}
