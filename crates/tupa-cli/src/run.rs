use crate::Commands;
use tupa_codegen::execution_plan::{codegen_pipeline, ExecutionPlan};
use tupa_codegen::generate_stub_with_types;
use tupa_parser::{Item, Program, parse_program, Expr, ExprKind, Span, Stmt, ParserError};
use tupa_lexer::LexerError;
use tupa_runtime::Runtime;
use tupa_audit::hash_ast;
use tupa_typecheck::{typecheck_program_with_warnings, analyze_effects, TypeError};
use std::collections::HashMap;
use serde_json::json;

pub async fn run(command: Commands) -> Result<(), String> {
    match command {
        Commands::Run { file, pipeline, input, plan } => run_pipeline(file, pipeline, input, plan).await,
        Commands::Check { file, format } => run_check(file, format).await,
        Commands::Audit { file, format, input } => run_audit(file, format, input).await,
        Commands::Parse { file, format } => run_parse(file, format).await,
        Commands::Lex { file, format } => run_lex(file, format).await,
        Commands::Codegen { file, format, plan_only } => run_codegen(file, format, plan_only).await,
        Commands::Effects { file, format } => run_effects(file, format).await,
    }
}

async fn run_pipeline(
    file: Option<String>,
    pipeline_name: Option<String>,
    input_file: Option<String>,
    plan_file: Option<String>,
) -> Result<(), String> {
    let runtime = Runtime::new();

    // Read input JSON
    let input = if let Some(path) = input_file {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| format!("Invalid input JSON: {}", e))?
    } else {
        serde_json::Value::Null
    };

    let plan: ExecutionPlan = if let Some(path) = plan_file {
        // Load pre-compiled plan
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| format!("Invalid plan JSON: {}", e))?
    } else if let Some(path) = file {
        // Compile from source
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let program = parse_program(&content).map_err(|e| format!("{:?}", e))?;
        typecheck_program_with_warnings(&program).map_err(|e| format!("{:?}", e))?;

        // Find pipeline
        let target_pipeline = if let Some(name) = &pipeline_name {
            program.items.iter().find_map(|item| {
                if let Item::Pipeline(p) = item {
                    if p.name == *name { Some(p) } else { None }
                } else {
                    None
                }
            }).ok_or_else(|| format!("Pipeline '{}' not found", name))?
        } else {
            // Default to first pipeline
            program.items.iter().find_map(|item| {
                if let Item::Pipeline(p) = item { Some(p) } else { None }
            }).ok_or_else(|| "No pipeline found in file".to_string())?
        };

        let plan_json = codegen_pipeline("main", target_pipeline, &program).map_err(|e| e.to_string())?;
        serde_json::from_str(&plan_json).map_err(|e| e.to_string())?
    } else {
        return Err("Either --plan or file argument must be provided".to_string());
    };

    // Execute
    let result = runtime.run_pipeline_async(&plan, input).await;
    
    match result {
        Ok(output) => {
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

async fn run_check(file: String, format: String) -> Result<(), String> {
    let content = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;

    match parse_program(&content) {
        Ok(program) => {
            match typecheck_program_with_warnings(&program) {
                Ok(_) => {
                    if format == "json" {
                        println!("{}", json!({ "status": "ok" }));
                    } else {
                        println!("OK");
                    }
                    Ok(())
                }
                Err(e) => {
                    let span = match &e {
                        TypeError::UnknownType { .. } |
                        TypeError::InvalidTypeArity { .. } => Span { start: 0, end: 0 },
                        
                        TypeError::UnknownVar { span, .. } |
                        TypeError::UnknownFunction { span, .. } |
                        TypeError::UnknownVariant { span, .. } |
                        TypeError::Mismatch { span, .. } |
                        TypeError::ArityMismatch { span, .. } |
                        TypeError::InvalidBinary { span, .. } |
                        TypeError::InvalidUnary { span, .. } |
                        TypeError::InvalidCallTarget { span, .. } |
                        TypeError::ReturnMismatch { span, .. } |
                        TypeError::MissingReturn { span } |
                        TypeError::InvalidConstraint { span, .. } |
                        TypeError::UnprovenConstraint { span, .. } |
                        TypeError::BreakOutsideLoop { span } |
                        TypeError::ContinueOutsideLoop { span } |
                        TypeError::NonExhaustiveMatch { span } => span.clone().unwrap_or(Span { start: 0, end: 0 }),
                        
                        TypeError::ImpureInDeterministic { span, .. } |
                        TypeError::UndefinedMetric { span, .. } => span.clone(),
                    };
                    let code = get_error_code(&e);
                    if format == "json" {
                        Err(format_json_error(e.to_string(), span, &file, &content, code))
                    } else {
                        Err(format_text_error(e.to_string(), span, &file, &content, code))
                    }
                }
            }
        }
        Err(e) => {
            let span = match &e {
                ParserError::Unexpected(_, s) => s.clone(),
                ParserError::Eof(pos) => Span { start: *pos, end: *pos },
                ParserError::Lexer(tupa_lexer::LexerError::Unexpected(_, pos)) => Span { start: *pos, end: *pos + 1 },
                ParserError::MissingSemicolon(s) => s.clone(),
            };
            let msg = match &e {
                ParserError::Unexpected(tok, _) => format!("unexpected token {:?}", tok),
                _ => e.to_string(),
            };
            if format == "json" {
                Err(format_json_error(msg, span, &file, &content, None))
            } else {
                Err(format_text_error(msg, span, &file, &content, None))
            }
        }
    }
}

async fn run_audit(file: String, format: String, input: Option<String>) -> Result<(), String> {
    let content = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
    let program = parse_program(&content).map_err(|e| format!("{:?}", e))?;

    let inputs = if let Some(input_file) = input {
        let input_content = std::fs::read_to_string(&input_file).map_err(|e| e.to_string())?;
        let input_json: serde_json::Value = serde_json::from_str(&input_content).map_err(|e| e.to_string())?;
        if let serde_json::Value::Array(arr) = input_json {
            arr
        } else {
            return Err("expected a JSON array".to_string());
        }
    } else {
        vec![]
    };

    let hash = if inputs.is_empty() {
        tupa_audit::hash_ast(&program)
    } else {
        tupa_audit::hash_execution(&program, &inputs)
    };

    if format == "json" {
        println!("{}", json!({ "ast_hash": hash.as_str() }));
    } else {
        println!("AST Hash: {}", hash);
    }
    Ok(())
}

fn get_line_col(content: &str, pos: usize) -> (usize, usize, String) {
    let mut line = 1;
    let mut col = 1;
    let mut line_start = 0;
    
    for (i, c) in content.char_indices() {
        if i == pos {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
            line_start = i + 1;
        } else {
            col += 1;
        }
    }
    
    let line_end = content[line_start..].find('\n').map(|i| line_start + i).unwrap_or(content.len());
    let line_text = content[line_start..line_end].to_string();
    
    (line, col, line_text)
}

fn format_json_error(message: String, span: Span, file: &str, content: &str, code: Option<&str>) -> String {
    let (line, col, line_text) = get_line_col(content, span.start);
    let json_err = json!({
        "error": {
            "code": code,
            "col": col,
            "label": file,
            "line": line,
            "line_text": line_text,
            "message": message,
            "span": span
        }
    });
    serde_json::to_string_pretty(&json_err).unwrap()
}

fn format_text_error(message: String, span: Span, file: &str, content: &str, code: Option<&str>) -> String {
    let (line, col, line_text) = get_line_col(content, span.start);
    let line_str = line.to_string();
    let pad = " ".repeat(line_str.len());
    
    let error_part = if let Some(c) = code {
        format!("error[{}]: {}", c, message)
    } else {
        format!("error: {}", message)
    };
    
    format!(
        "{}\n  --> {}:{}:{}\n {} |\n {} | {}\n {} | {}{}",
        error_part, file, line, col, 
        pad, 
        line, line_text, 
        pad, 
        " ".repeat(col - 1), "^"
    )
}

fn get_error_code(e: &TypeError) -> Option<&'static str> {
    match e {
        TypeError::UnknownType { .. } => Some("E1001"),
        TypeError::UnknownVar { .. } => Some("E1002"),
        TypeError::UnknownFunction { .. } => Some("E1003"),
        TypeError::UnknownVariant { .. } => Some("E1004"),
        TypeError::Mismatch { .. } => Some("E2001"),
        TypeError::ArityMismatch { .. } => Some("E2002"),
        TypeError::InvalidTypeArity { .. } => Some("E2002"),
        TypeError::InvalidBinary { .. } => Some("E2003"),
        TypeError::InvalidUnary { .. } => Some("E2004"),
        TypeError::InvalidCallTarget { .. } => Some("E2005"),
        TypeError::ReturnMismatch { .. } => Some("E2006"),
        TypeError::MissingReturn { .. } => Some("E2007"),
        TypeError::InvalidConstraint { .. } => Some("E3001"),
        TypeError::UnprovenConstraint { .. } => Some("E3002"),
        TypeError::BreakOutsideLoop { .. } => Some("E4001"),
        TypeError::ContinueOutsideLoop { .. } => Some("E4002"),
        TypeError::NonExhaustiveMatch { .. } => Some("E5001"),
        TypeError::ImpureInDeterministic { .. } => Some("E2005"),
        TypeError::UndefinedMetric { .. } => Some("E2006"),
    }
}

async fn run_parse(file: String, format: String) -> Result<(), String> {
    let content = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;

    match parse_program(&content) {
        Ok(program) => {
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&program).unwrap());
            } else {
                println!("{:#?}", program);
            }
            Ok(())
        }
        Err(e) => {
            let span = match &e {
                ParserError::Unexpected(_, s) => s.clone(),
                ParserError::Eof(pos) => Span { start: *pos, end: *pos },
                ParserError::Lexer(tupa_lexer::LexerError::Unexpected(_, pos)) => Span { start: *pos, end: *pos + 1 },
                ParserError::MissingSemicolon(s) => s.clone(),
            };
            let msg = match &e {
                ParserError::Unexpected(tok, _) => format!("unexpected token {:?}", tok),
                _ => e.to_string(),
            };
            if format == "json" {
                Err(format_json_error(msg, span, &file, &content, None))
            } else {
                Err(format_text_error(msg, span, &file, &content, None))
            }
        }
    }
}

async fn run_lex(file: String, format: String) -> Result<(), String> {
    let content = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;

    match tupa_lexer::lex_with_spans(&content) {
        Ok(tokens) => {
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&tokens).unwrap());
            } else {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Ok(())
        }
        Err(e) => {
            let span = match &e {
                LexerError::Unexpected(_, pos) => Span { start: *pos, end: *pos + 1 },
            };
            let msg = e.to_string();
            if format == "json" {
                Err(format_json_error(msg, span, &file, &content, None))
            } else {
                Err(format_text_error(msg, span, &file, &content, None))
            }
        }
    }
}

async fn run_codegen(file: String, format: String, plan_only: bool) -> Result<(), String> {
    let content = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
    
    match parse_program(&content) {
        Ok(program) => {
            match typecheck_program_with_warnings(&program) {
                Ok(_) => {
                    if format == "json" || plan_only {
                        // Generate execution plans for all pipelines
                        let mut plans = Vec::new();
                        let pipelines: Vec<_> = program.items.iter().filter_map(|item| {
                            if let Item::Pipeline(p) = item { Some(p) } else { None }
                        }).collect();
                        
                        if pipelines.is_empty() && plan_only {
                            return Err("No pipelines found to generate plan".to_string());
                        }

                        for p in &pipelines {
                            let plan_json = codegen_pipeline("main", p, &program).map_err(|e| e.to_string())?;
                            let plan: serde_json::Value = serde_json::from_str(&plan_json).unwrap();
                            plans.push(plan);
                        }

                        if plan_only {
                             // Write to file
                             // Use input filename stem
                             let path = std::path::Path::new(&file);
                             let stem = path.file_stem().unwrap().to_string_lossy();
                             let output_path = format!("{}.plan.json", stem);
                             
                             if !plans.is_empty() {
                                 // Write single object (first pipeline) to be compatible with `run --plan`
                                 // If there are multiple, this might be ambiguous, but fits the simple test case.
                                 let plan_json = serde_json::to_string_pretty(&plans[0]).unwrap();
                                 std::fs::write(&output_path, plan_json).map_err(|e| e.to_string())?;
                             }
                        } else {
                            println!("{}", serde_json::to_string_pretty(&plans).unwrap());
                        }
                    } else {
                        // Text format (Rust stub)
                        let stub = generate_stub_with_types(&program);
                        println!("{}", stub);
                    }
                    Ok(())
                }
                Err(e) => {
                    if format == "json" {
                        let span = match &e {
                            TypeError::UnknownType { .. } |
                            TypeError::InvalidTypeArity { .. } => Span { start: 0, end: 0 },
                            
                            TypeError::UnknownVar { span, .. } |
                            TypeError::UnknownFunction { span, .. } |
                            TypeError::UnknownVariant { span, .. } |
                            TypeError::Mismatch { span, .. } |
                            TypeError::ArityMismatch { span, .. } |
                            TypeError::InvalidBinary { span, .. } |
                            TypeError::InvalidUnary { span, .. } |
                            TypeError::InvalidCallTarget { span, .. } |
                            TypeError::ReturnMismatch { span, .. } |
                            TypeError::MissingReturn { span } |
                            TypeError::InvalidConstraint { span, .. } |
                            TypeError::UnprovenConstraint { span, .. } |
                            TypeError::BreakOutsideLoop { span } |
                            TypeError::ContinueOutsideLoop { span } |
                            TypeError::NonExhaustiveMatch { span } => span.clone().unwrap_or(Span { start: 0, end: 0 }),
                            
                            TypeError::ImpureInDeterministic { span, .. } |
                            TypeError::UndefinedMetric { span, .. } => span.clone(),
                        };
                        let code = get_error_code(&e);
                        Err(format_json_error(e.to_string(), span, &file, &content, code))
                    } else {
                        Err(e.to_string())
                    }
                }
            }
        }
        Err(e) => {
            let span = match &e {
                ParserError::Unexpected(_, s) => s.clone(),
                ParserError::Eof(pos) => Span { start: *pos, end: *pos },
                ParserError::Lexer(tupa_lexer::LexerError::Unexpected(_, pos)) => Span { start: *pos, end: *pos + 1 },
                ParserError::MissingSemicolon(s) => s.clone(),
            };
            let msg = match &e {
                ParserError::Unexpected(tok, _) => format!("unexpected token {:?}", tok),
                _ => e.to_string(),
            };
            if format == "json" {
                Err(format_json_error(msg, span, &file, &content, None))
            } else {
                Err(format_text_error(msg, span, &file, &content, None))
            }
        }
    }
}

async fn run_effects(file: String, format: String) -> Result<(), String> {
    let content = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
    
    match parse_program(&content) {
        Ok(program) => {
            match typecheck_program_with_warnings(&program) {
                Ok(_) => {
                    let mut effects_map = HashMap::new();

                    for item in &program.items {
                        match item {
                            Item::Function(f) => {
                                // Wrap body in a block expr for analysis
                                let body_expr = Expr {
                                    kind: ExprKind::Block(f.body.clone()),
                                    span: Span { start: 0, end: 0 }, // Dummy span
                                };
                                let effs = analyze_effects(&body_expr, &HashMap::new());
                                effects_map.insert(f.name.clone(), effs);
                            }
                            Item::Pipeline(p) => {
                                for step in &p.steps {
                                    let effs = analyze_effects(&step.body, &HashMap::new());
                                    effects_map.insert(format!("pipeline:{}:{}", p.name, step.name), effs);
                                }
                            }
                            _ => {}
                        }
                    }

                    if format == "json" {
                        let mut serializable_map = HashMap::new();
                        for (k, v) in effects_map {
                             serializable_map.insert(k, v.to_names());
                        }
                        println!("{}", serde_json::to_string_pretty(&serializable_map).unwrap());
                    } else {
                        // Sort keys for deterministic output
                        let mut sorted_keys: Vec<_> = effects_map.keys().collect();
                        sorted_keys.sort();
                        
                        for name in sorted_keys {
                            let effs = effects_map.get(name).unwrap();
                            println!("{}: {:?}", name, effs.to_names());
                        }
                    }
                    Ok(())
                }
                Err(e) => {
                    if format == "json" {
                        let span = match &e {
                            TypeError::UnknownType { .. } |
                            TypeError::InvalidTypeArity { .. } => Span { start: 0, end: 0 },
                            
                            TypeError::UnknownVar { span, .. } |
                            TypeError::UnknownFunction { span, .. } |
                            TypeError::UnknownVariant { span, .. } |
                            TypeError::Mismatch { span, .. } |
                            TypeError::ArityMismatch { span, .. } |
                            TypeError::InvalidBinary { span, .. } |
                            TypeError::InvalidUnary { span, .. } |
                            TypeError::InvalidCallTarget { span, .. } |
                            TypeError::ReturnMismatch { span, .. } |
                            TypeError::MissingReturn { span } |
                            TypeError::InvalidConstraint { span, .. } |
                            TypeError::UnprovenConstraint { span, .. } |
                            TypeError::BreakOutsideLoop { span } |
                            TypeError::ContinueOutsideLoop { span } |
                            TypeError::NonExhaustiveMatch { span } => span.clone().unwrap_or(Span { start: 0, end: 0 }),
                            
                            TypeError::ImpureInDeterministic { span, .. } |
                            TypeError::UndefinedMetric { span, .. } => span.clone(),
                        };
                        let code = get_error_code(&e);
                        Err(format_json_error(e.to_string(), span, &file, &content, code))
                    } else {
                        Err(e.to_string())
                    }
                }
            }
        }
        Err(e) => {
            let span = match &e {
                ParserError::Unexpected(_, s) => s.clone(),
                ParserError::Eof(pos) => Span { start: *pos, end: *pos },
                ParserError::Lexer(tupa_lexer::LexerError::Unexpected(_, pos)) => Span { start: *pos, end: *pos + 1 },
                ParserError::MissingSemicolon(s) => s.clone(),
            };
            let msg = match &e {
                ParserError::Unexpected(tok, _) => format!("unexpected token {:?}", tok),
                _ => e.to_string(),
            };
            if format == "json" {
                Err(format_json_error(msg, span, &file, &content, None))
            } else {
                Err(format_text_error(msg, span, &file, &content, None))
            }
        }
    }
}
