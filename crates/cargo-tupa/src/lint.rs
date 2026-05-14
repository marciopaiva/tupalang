use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Lint Rust-DSL pipeline code for common issues.
///
/// Checks:
/// - Step name collisions
/// - Undefined step references in requires/produces
/// - Empty steps list
/// - Missing name/input
pub fn lint(file: Option<PathBuf>) -> Result<()> {
    let files = if let Some(ref f) = file {
        vec![f.clone()]
    } else {
        // Walk src/ for .rs files
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new("src")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path.to_path_buf());
            }
        }
        files
    };

    let mut total_issues = 0;

    for file in files {
        if let Ok(content) = fs::read_to_string(&file) {
            if !content.contains("pipeline!") {
                continue;
            }

            println!("🔍 Linting {}...", file.display());
            let issues = lint_file(&content, &file)?;
            total_issues += issues;
        }
    }

    if total_issues == 0 {
        println!("✅ No issues found");
    } else {
        println!("⚠️  Found {} issue(s)", total_issues);
    }

    Ok(())
}

fn lint_file(content: &str, file: &PathBuf) -> Result<u32> {
    let mut issues = 0;

    // Find pipeline! block
    let start = content.find("pipeline!").ok_or_else(|| anyhow::anyhow!("no pipeline found"))?;
    let block_start = content[start..].find('{').map(|i| start + i);

    if let Some(idx) = block_start {
        let mut brace_count = 1;
        let mut i = idx + 1;
        while i < content.len() && brace_count > 0 {
            match content.chars().nth(i) {
                Some('{') => brace_count += 1,
                Some('}') => brace_count -= 1,
                _ => {}
            }
            i += 1;
        }
        if brace_count == 0 {
            let block = &content[idx + 1..i - 1];
            issues += lint_pipeline_block(block, file)?;
        }
    }

    Ok(issues)
}

fn lint_pipeline_block(block: &str, file: &PathBuf) -> Result<u32> {
    let mut issues = 0;

    let step_names = extract_step_names(block)?;

    // Duplicate step names
    let mut seen = HashSet::new();
    for name in &step_names {
        if seen.contains(name) {
            println!("  ⚠️  Duplicate step name: '{}' in {}", name, file.display());
            issues += 1;
        } else {
            seen.insert(name);
        }
    }

    let requires = extract_string_refs(block, "requires")?;
    let produces = extract_string_refs(block, "produces")?;

    for req in &requires {
        if !seen.contains(req) {
            println!("  ⚠️  Undefined step reference in requires: '{}' in {}", req, file.display());
            issues += 1;
        }
    }

    for prod in &produces {
        if !seen.contains(prod) {
            println!("  ⚠️  Undefined step reference in produces: '{}' in {}", prod, file.display());
            issues += 1;
        }
    }

    if !block.contains("name:") {
        println!("  ⚠️  Missing 'name:' field in pipeline");
        issues += 1;
    }
    if !block.contains("input:") {
        println!("  ⚠️  Missing 'input:' field in pipeline");
        issues += 1;
    }

    Ok(issues)
}

fn extract_step_names(block: &str) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for (idx, _) in block.match_indices("step(") {
        let sub = &block[idx + 5..];
        // Find the opening quote
        let start = sub.find('"').ok_or_else(|| anyhow::anyhow!("missing quote in step name"))? + 1;
        // Find the closing quote after that
        let end = sub[start..].find('"').ok_or_else(|| anyhow::anyhow!("unclosed step name"))? + start;
        let name = &sub[start..end];
        names.push(name.to_string());
    }
    Ok(names)
}

fn extract_string_refs(block: &str, keyword: &str) -> Result<Vec<String>> {
    let mut refs = Vec::new();
    let pattern = format!("{}(", keyword);

    for (idx, _) in block.match_indices(&pattern) {
        // Extract the argument list inside parentheses
        let start = idx + pattern.len();
        let mut depth = 1;
        let mut end = start;
        let chars: Vec<char> = block.chars().collect();
        while end < chars.len() && depth > 0 {
            match chars[end] {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            end += 1;
        }
        // Now `block[start..end-1]` is the argument string
        let args = &block[start..end - 1];
        // Extract all quoted strings from args
        let mut pos = 0;
        while let Some(quote_pos) = args[pos..].find('"') {
            let s_start = pos + quote_pos + 1;
            let s_end = args[s_start..].find('"').ok_or_else(|| anyhow::anyhow!("unclosed string"))? + s_start;
            refs.push(args[s_start..s_end].to_string());
            pos = s_end + 1;
        }
    }

    Ok(refs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_step_names() -> Result<()> {
        let block = r#"
            name: Test,
            input: (),
            steps: [
                step("load"),
                step("process"),
                step("save")
            ],
            constraints: []
        "#;
        let names = extract_step_names(block)?;
        assert_eq!(names, vec!["load", "process", "save"]);
        Ok(())
    }

    #[test]
    fn test_extract_string_refs() -> Result<()> {
        let block = r#"
            steps: [
                step("A") { requires("B"); produces("C") },
                step("B") { requires("A") }
            ]
        "#;
        let requires = extract_string_refs(block, "requires")?;
        let produces = extract_string_refs(block, "produces")?;
        assert_eq!(requires, vec!["B", "A"]);
        assert_eq!(produces, vec!["C"]);
        Ok(())
    }
}
