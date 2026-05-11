use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

/// Run `cargo tupa lint` — lint Tupã source files.
///
/// For legacy `.tp` files, uses `tupa-lint`.
/// For Rust DSL, runs `cargo check` and surface warnings.
pub fn run(manifest_path: &Option<PathBuf>, deny_warnings: bool) -> Result<()> {
    // Find .tp files in the current package
    let project_root = manifest_path
        .as_ref()
        .map(|p| p.parent().unwrap())
        .unwrap_or_else(|| std::path::Path::new("."));

    let mut tp_files = Vec::new();

    for entry in WalkDir::new(project_root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("tp") {
            tp_files.push(path.to_path_buf());
        }
    }

    if !tp_files.is_empty() {
        println!("Linting {} .tp file(s)...", tp_files.len());
        for file in &tp_files {
            lint_tp_file(file)?;
        }
    } else {
        // No .tp files: lint the Rust DSL by running cargo check and showing warnings
        println!("No .tp files found. Linting Rust DSL via `cargo check`...");
        lint_rust_dsl(manifest_path, deny_warnings)?;
    }

    Ok(())
}

/// Lint a single `.tp` file using `tupa-lint`.
fn lint_tp_file(file: &PathBuf) -> Result<()> {
    use tupa_lint::lint_program;
    use tupa_parser::parse_program;

    let content = std::fs::read_to_string(file)?;
    let program = parse_program(&content).map_err(|e| anyhow::anyhow!("parse error: {}", e))?;

    let warnings = lint_program(&program);
    for w in &warnings {
        println!("{}: {}", file.display(), w.message());
    }

    Ok(())
}

/// Lint Rust DSL by invoking `cargo check` and filtering warnings.
fn lint_rust_dsl(manifest_path: &Option<PathBuf>, deny_warnings: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("check");

    if let Some(ref path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Filter for warnings (but not errors)
        for line in stderr.lines() {
            if line.contains("warning:") || line.contains("warn[") {
                eprintln!("{}", line);
            }
        }
        return Ok(());
    }

    // Check for warnings in stdout/stderr
    let combined = String::from_utf8_lossy(&output.stdout);
    let mut has_warnings = false;

    for line in combined.lines() {
        if line.contains("warning:") {
            eprintln!("{}", line);
            has_warnings = true;
        }
    }

    if !has_warnings {
        println!("✅ No lint warnings found");
    } else if deny_warnings {
        anyhow::bail!("warnings found and --deny-warnings enabled");
    }

    Ok(())
}
