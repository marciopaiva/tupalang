use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Format legacy `.tp` files using `tupa-fmt`.
///
/// If no files specified, formats all `.tp` files under the current package.
pub fn run(manifest_path: &Option<PathBuf>, files: Vec<PathBuf>) -> Result<()> {
    let project_root = manifest_path
        .as_ref()
        .map(|p| p.parent().unwrap())
        .unwrap_or_else(|| std::path::Path::new("."));

    // Collect files to format
    let files_to_format: Vec<PathBuf> = if files.is_empty() {
        WalkDir::new(project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("tp"))
            .map(|e| e.path().to_path_buf())
            .collect()
    } else {
        files
    };

    if files_to_format.is_empty() {
        println!("No .tp files found to format");
        return Ok(());
    }

    // Use tupa-fmt library
    use tupa_fmt::format_source;

    for file in &files_to_format {
        let content = std::fs::read_to_string(file)?;
        let formatted = format_source(&content);
        if !formatted.is_empty() && formatted != content {
            std::fs::write(file, formatted)?;
            println!("✓ Formatted {}", file.display());
        } else {
            println!("✓ {} already formatted", file.display());
        }
    }

    Ok(())
}
