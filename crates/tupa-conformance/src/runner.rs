// Conformance runner for Tupã language implementation
// Tests parser and typechecker against a manifest of expected outcomes.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCase {
    id: String,
    file: String,
    stage: String,
    expect: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct TestResult {
    id: String,
    passed: bool,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct Report {
    total: usize,
    passed: usize,
    failed: usize,
    results: Vec<TestResult>,
}

impl TestCase {
    fn run(&self, workspace_root: &Path) -> TestResult {
        let full_path = workspace_root.join(&self.file);
        if !full_path.exists() {
            return TestResult {
                id: self.id.clone(),
                passed: false,
                error: Some(format!("File not found: {}", full_path.display())),
            };
        }

        let source = match fs::read_to_string(&full_path) {
            Ok(s) => s,
            Err(e) => {
                return TestResult {
                    id: self.id.clone(),
                    passed: false,
                    error: Some(format!("IO error: {}", e)),
                };
            }
        };

        // First, parse always (typecheck depends on parse)
        let parse_res = tupa_parser::parse_program(&source);
        let parse_ok = parse_res.is_ok();

        // Determine expected outcome for this stage
        let expect_ok = self.expect == "ok";

        // Evaluate based on stage
        let (passed, error) = match self.stage.as_str() {
            "parse" => {
                let success = parse_ok;
                if success == expect_ok {
                    (true, None)
                } else {
                    let msg = if expect_ok {
                        Some("Expected parse to succeed but it failed".into())
                    } else {
                        Some("Expected parse to fail but it succeeded".into())
                    };
                    (false, msg)
                }
            }
            "typecheck" => {
                if !parse_ok {
                    // Cannot typecheck if parse fails
                    if expect_ok {
                        (
                            false,
                            Some(format!(
                                "Parse failed before typecheck: {:?}",
                                parse_res.err()
                            )),
                        )
                    } else {
                        // If we expected error overall, parse error counts as pass for the test (typecheck not reached)
                        (true, None)
                    }
                } else {
                    // Parse succeeded; run typecheck
                    let program = parse_res.unwrap();
                    let tc_res = tupa_typecheck::typecheck_program(&program);
                    let tc_ok = tc_res.is_ok();
                    if tc_ok == expect_ok {
                        (true, None)
                    } else {
                        let msg = if expect_ok {
                            Some(format!("Typecheck failed: {:?}", tc_res.err()))
                        } else {
                            Some("Typecheck succeeded but expected error".into())
                        };
                        (false, msg)
                    }
                }
            }
            other => (
                false,
                Some(format!("Unknown stage '{}' in manifest", other)),
            ),
        };

        TestResult {
            id: self.id.clone(),
            passed,
            error,
        }
    }
}

pub fn run() -> Result<()> {
    // Load manifest (embedded)
    let manifest_str = include_str!("../data/manifest.json");
    let tests: Vec<TestCase> =
        serde_json::from_str(manifest_str).expect("Failed to parse manifest.json");

    // Determine workspace root: parent of crates/ directory (two levels up from crate dir)
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir.parent().unwrap().parent().unwrap();

    let mut results = Vec::new();
    for test in &tests {
        let result = test.run(workspace_root);
        results.push(result);
    }

    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = total - passed;

    let report = Report {
        total,
        passed,
        failed,
        results,
    };

    // Print JSON report to stdout
    println!("{}", serde_json::to_string_pretty(&report).unwrap());

    // Exit code: 0 if all passed, 1 otherwise
    if failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}
