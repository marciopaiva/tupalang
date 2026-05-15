use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Format Rust-DSL pipeline code.
///
/// Currently applies basic formatting to `pipeline!` macro blocks:
/// - Indents inner content by 2 spaces
/// - Ensures closing brace is on its own line
/// - Normalizes trailing commas
pub fn format_pipeline(file: Option<PathBuf>) -> Result<()> {
    let files = if let Some(ref f) = file {
        vec![f.clone()]
    } else {
        // Walk src/ directory for .rs files
        let mut files = Vec::new();
        for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path.to_path_buf());
            }
        }
        files
    };

    let mut changed = 0;

    for file in files {
        if let Ok(content) = fs::read_to_string(&file) {
            if content.contains("pipeline!") {
                let formatted = format_file(&content)?;
                if formatted != content {
                    fs::write(&file, &formatted)?;
                    println!("📝 Formatted {}", file.display());
                    changed += 1;
                }
            }
        }
    }

    if changed == 0 {
        println!("✅ All pipeline files are formatted");
    } else {
        println!("✨ Formatted {} file(s)", changed);
    }

    Ok(())
}

fn format_file(content: &str) -> Result<String> {
    let mut result = String::new();
    let mut lines = content.lines().peekable();

    while let Some(line) = lines.next() {
        if line.trim().starts_with("pipeline!") {
            // Capture entire macro block
            let mut block = line.to_string();
            let mut brace_count = block.matches('{').count() - block.matches('}').count();

            while brace_count > 0 {
                if let Some(next) = lines.next() {
                    block.push('\n');
                    block.push_str(next);
                    brace_count += next.matches('{').count();
                    brace_count -= next.matches('}').count();
                } else {
                    anyhow::bail!("Unclosed pipeline! macro");
                }
            }

            result.push_str(&reformat_pipeline_block(&block)?);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

fn reformat_pipeline_block(block: &str) -> Result<String> {
    let start = block
        .find('{')
        .ok_or_else(|| anyhow::anyhow!("no opening brace"))?
        + 1;
    let end = block
        .rfind('}')
        .ok_or_else(|| anyhow::anyhow!("no closing brace"))?;
    let inner = &block[start..end];

    let mut formatted = String::new();
    formatted.push_str("pipeline! {\n");

    let mut current = String::new();
    let mut paren_depth = 0;
    let mut bracket_depth = 0;

    for ch in inner.chars() {
        match ch {
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            ',' if paren_depth == 0 && bracket_depth == 0 => {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    formatted.push_str("  ");
                    formatted.push_str(trimmed);
                    formatted.push_str(",\n");
                }
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(ch);
    }

    let trimmed = current.trim();
    if !trimmed.is_empty() {
        formatted.push_str("  ");
        formatted.push_str(trimmed);
        formatted.push('\n');
    }

    formatted.push('}');
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reformat_pipeline_block() -> Result<()> {
        let input = "pipeline! {\nname: MyPipe,\ninput: i32,\nsteps: [ step(\"a\") { a+1 }, step(\"b\") { b*2 } ],\nconstraints: [ metric(\"out\").ge(0) ]\n}";

        let output = reformat_pipeline_block(input)?;

        assert!(output.contains("pipeline! {"));
        assert!(output.contains("  name: MyPipe,"));
        assert!(output.contains("  input: i32,"));
        assert!(output.contains("  steps: ["));
        assert!(output.contains("  constraints: ["));
        Ok(())
    }

    #[test]
    fn test_fmt_idempotent_on_clean_input() {
        let clean = "pipeline! {\n  name: X,\n  input: i32,\n  steps: [],\n  constraints: []\n}";
        let once = format_file(clean).unwrap();
        let twice = format_file(&once).unwrap();
        assert_eq!(once, twice, "fmt must be idempotent");
    }
}
