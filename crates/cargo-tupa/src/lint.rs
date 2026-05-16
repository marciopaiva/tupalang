use anyhow::Result;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Diagnostic code for a single lint issue.
#[derive(Debug, Clone, Serialize)]
struct LintIssue {
    code: &'static str,
    message: String,
    file: String,
}

/// Lint Rust-DSL pipeline code for common issues.
///
/// Diagnostic codes:
/// - E001: Duplicate step name
/// - E002: Undefined `requires()` reference
/// - E003: Undefined `produces()` reference
/// - E004: Missing `name:` field
/// - E005: Missing `input:` field
/// - E006: Empty steps list
/// - E007: Step name is a Rust keyword
/// - E008: Step name starts with a digit
/// - E009: Large pipeline (≥20 steps — performance warning)
pub fn lint(file: Option<PathBuf>, json: bool) -> Result<()> {
    let files = if let Some(ref f) = file {
        vec![f.clone()]
    } else {
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

    let mut all_issues: Vec<LintIssue> = Vec::new();

    for file in files {
        if let Ok(content) = fs::read_to_string(&file) {
            if !content.contains("pipeline!") {
                continue;
            }

            if !json {
                println!("🔍 Linting {}...", file.display());
            }
            let issues = lint_file(&content, &file, json)?;
            all_issues.extend(issues);
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&all_issues)?);
    } else if all_issues.is_empty() {
        println!("✅ No issues found");
    } else {
        println!("⚠️  Found {} issue(s)", all_issues.len());
    }

    Ok(())
}

fn lint_file(content: &str, file: &std::path::Path, json: bool) -> Result<Vec<LintIssue>> {
    let start = content
        .find("pipeline!")
        .ok_or_else(|| anyhow::anyhow!("no pipeline found"))?;
    let block_start = content[start..].find('{').map(|i| start + i);

    let mut issues = Vec::new();
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
            issues = lint_pipeline_block(block, file, json)?;
        }
    }

    Ok(issues)
}

fn lint_pipeline_block(block: &str, file: &std::path::Path, json: bool) -> Result<Vec<LintIssue>> {
    let mut issues = Vec::new();

    let step_names = extract_step_names(block)?;

    // E001: Duplicate step names
    let mut seen = HashSet::new();
    for name in &step_names {
        if seen.contains(name) {
            let msg = format!("Duplicate step name: '{}'", name);
            if !json {
                println!("  ⚠️  [E001] {}", msg);
            }
            issues.push(LintIssue {
                code: "E001",
                message: msg,
                file: file.display().to_string(),
            });
        } else {
            seen.insert(name);
        }
    }

    let requires = extract_string_refs(block, "requires")?;
    let produces = extract_string_refs(block, "produces")?;

    // E002: Undefined requires reference
    // E002: Undefined requires reference
    for req in &requires {
        if !seen.contains(req) {
            let msg = format!("Undefined step reference in requires: '{}'", req);
            if !json {
                println!("  ⚠️  [E002] {}", msg);
            }
            issues.push(LintIssue {
                code: "E002",
                message: msg,
                file: file.display().to_string(),
            });
        }
    }

    // E003: Undefined produces reference
    for prod in &produces {
        if !seen.contains(prod) {
            let msg = format!("Undefined step reference in produces: '{}'", prod);
            if !json {
                println!("  ⚠️  [E003] {}", msg);
            }
            issues.push(LintIssue {
                code: "E003",
                message: msg,
                file: file.display().to_string(),
            });
        }
    }

    // E004: Missing name field
    if !block.contains("name:") {
        let msg = "Missing 'name:' field in pipeline".to_string();
        if !json {
            println!("  ⚠️  [E004] {}", msg);
        }
        issues.push(LintIssue {
            code: "E004",
            message: msg,
            file: file.display().to_string(),
        });
    }

    // E005: Missing input field
    if !block.contains("input:") {
        let msg = "Missing 'input:' field in pipeline".to_string();
        if !json {
            println!("  ⚠️  [E005] {}", msg);
        }
        issues.push(LintIssue {
            code: "E005",
            message: msg,
            file: file.display().to_string(),
        });
    }

    // E006: Empty steps list
    if !block.contains("step(") {
        let msg = "Empty steps list — pipeline has no steps".to_string();
        if !json {
            println!("  ⚠️  [E006] {}", msg);
        }
        issues.push(LintIssue {
            code: "E006",
            message: msg,
            file: file.display().to_string(),
        });
    }

    // E007: Step name is a Rust keyword
    let rust_keywords = [
        "fn", "let", "match", "if", "else", "for", "while", "loop", "return", "struct", "enum",
        "impl", "trait", "type", "const", "static", "mod", "use", "crate", "self", "Self", "super",
        "move", "ref", "as", "break", "continue", "dyn", "where", "async", "await", "unsafe",
        "extern", "true", "false",
    ];
    for name in &step_names {
        if rust_keywords.contains(&name.as_str()) {
            let msg = format!(
                "Step name '{}' is a Rust keyword — may cause confusion",
                name
            );
            if !json {
                println!("  ⚠️  [E007] {}", msg);
            }
            issues.push(LintIssue {
                code: "E007",
                message: msg,
                file: file.display().to_string(),
            });
        }
    }

    // E008: Step name starts with a digit
    for name in &step_names {
        if name.starts_with(|c: char| c.is_ascii_digit()) {
            let msg = format!(
                "Step name '{}' starts with a digit — not a valid Rust identifier",
                name
            );
            if !json {
                println!("  ⚠️  [E008] {}", msg);
            }
            issues.push(LintIssue {
                code: "E008",
                message: msg,
                file: file.display().to_string(),
            });
        }
    }

    // E009: Large pipeline (≥20 steps — performance warning)
    if step_names.len() >= 20 {
        let msg = format!(
            "Large pipeline with {} steps — consider splitting",
            step_names.len()
        );
        if !json {
            println!("  ⚠️  [E009] {}", msg);
        }
        issues.push(LintIssue {
            code: "E009",
            message: msg,
            file: file.display().to_string(),
        });
    }

    Ok(issues)
}

fn extract_step_names(block: &str) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for (idx, _) in block.match_indices("step(") {
        let sub = &block[idx + 5..];
        // Find the opening quote
        let start = sub
            .find('"')
            .ok_or_else(|| anyhow::anyhow!("missing quote in step name"))?
            + 1;
        // Find the closing quote after that
        let end = sub[start..]
            .find('"')
            .ok_or_else(|| anyhow::anyhow!("unclosed step name"))?
            + start;
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
            let s_end = args[s_start..]
                .find('"')
                .ok_or_else(|| anyhow::anyhow!("unclosed string"))?
                + s_start;
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

    #[test]
    fn test_lint_detects_duplicate_steps() -> Result<()> {
        let block = r#"
            name: Dup,
            input: (),
            steps: [
                step("a"){1},
                step("a"){2}
            ],
            constraints: []
        "#;
        let issues = lint_pipeline_block(block, std::path::Path::new("dup.rs"), false)?;
        assert_eq!(issues.len(), 1, "expected exactly 1 issue (duplicate step)");
        assert_eq!(issues[0].code, "E001");
        Ok(())
    }

    #[test]
    fn test_lint_clean_pipeline_has_no_issues() -> Result<()> {
        let block = r#"
            name: Clean,
            input: (),
            steps: [ step("a"){1}, step("b"){2} ],
            constraints: []
        "#;
        let issues = lint_pipeline_block(block, std::path::Path::new("clean.rs"), false)?;
        assert_eq!(issues.len(), 0, "expected 0 issues for clean pipeline");
        Ok(())
    }

    #[test]
    fn test_lint_json_output() -> Result<()> {
        let block = r#"
            name: TestJson,
            input: (),
            steps: [
                step("a"){1},
                step("fn"){2}
            ],
            constraints: []
        "#;
        let issues = lint_pipeline_block(block, std::path::Path::new("test_json.rs"), true)?;
        assert_eq!(issues.len(), 1, "expected 1 issue (E007 keyword)");
        assert_eq!(issues[0].code, "E007");
        Ok(())
    }

    #[test]
    fn test_lint_e004_missing_name() -> Result<()> {
        let block = r#"
            input: (),
            steps: [],
            constraints: []
        "#;
        let issues = lint_pipeline_block(block, std::path::Path::new("no_name.rs"), false)?;
        assert!(issues.iter().any(|i| i.code == "E004"), "expected E004");
        Ok(())
    }

    #[test]
    fn test_lint_e006_empty_steps() -> Result<()> {
        let block = r#"
            name: Empty,
            input: (),
            steps: [],
            constraints: []
        "#;
        let issues = lint_pipeline_block(block, std::path::Path::new("empty.rs"), false)?;
        assert!(issues.iter().any(|i| i.code == "E006"), "expected E006");
        Ok(())
    }

    #[test]
    fn test_lint_e008_digit_start() -> Result<()> {
        let block = r#"
            name: Digit,
            input: (),
            steps: [ step("1step"){1} ],
            constraints: []
        "#;
        let issues = lint_pipeline_block(block, std::path::Path::new("digit.rs"), false)?;
        assert!(issues.iter().any(|i| i.code == "E008"), "expected E008");
        Ok(())
    }
}
