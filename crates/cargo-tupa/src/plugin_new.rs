use anyhow::{Context, Result};
use std::fs;

/// Generate a plugin scaffold file.
///
/// Writes the plugin template from `tupa_plugin::create_plugin_template()`
/// to the specified path (or default `my_plugin.rs` in current directory).
pub fn run(filename: Option<String>) -> Result<()> {
    let template = tupa_plugin::create_plugin_template();

    let default_name = "my_plugin.rs";
    let path = filename.as_deref().unwrap_or(default_name);

    fs::write(path, template)
        .with_context(|| format!("failed to write plugin file to {}", path))?;

    println!("Plugin scaffold written to {}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_plugin_generation_to_tempdir() {
        let tmp = tempdir().unwrap();
        let file_path = tmp.path().join("my_plugin.rs");
        let filename = file_path.to_str().unwrap();

        run(Some(filename.to_string())).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("_tupa_plugin_name"));
        assert!(content.contains("_tupa_plugin_register"));
        assert!(content.contains("my_step"));
        assert!(content.contains("extern \"C\""));
    }
}
