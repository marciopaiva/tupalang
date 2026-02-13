use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use serde_json::{json, Value};
use tupa_audit::{compiler_version, hash_ast, hash_execution};
use tupa_codegen::generate_stub_with_types;
use tupa_lexer::{lex, lex_with_spans, LexerError, Span, Token, TokenSpan};
use tupa_parser::{parse_program, ParserError};
use tupa_typecheck::{typecheck_program_with_warnings, TypeError, Warning};

#[derive(Parser)]
#[command(name = "tupa", version, about = "Tupã CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
}

#[derive(Subcommand)]
enum Command {
    /// Lex a .tp file and print tokens
    Lex {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Parse a .tp file and print the AST
    Parse {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Parse and typecheck a .tp file
    Check {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Generate code (stub)
    Codegen {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Generate deterministic audit hash
    Audit {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
        /// JSON array file with inputs
        #[arg(long)]
        input: Option<PathBuf>,
        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
        format: OutputFormat,
    },
    /// Print CLI version
    Version,
    /// Print CLI about
    About,
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Lex {
            file,
            stdin,
            format,
        } => {
            let (src, label) = read_source(file.as_ref(), stdin)?;
            match format {
                OutputFormat::Pretty => {
                    let tokens = lex(&src).map_err(|e| format_lex_error(&label, &src, e))?;
                    for tok in tokens {
                        println!("{tok:?}");
                    }
                }
                OutputFormat::Json => {
                    let tokens =
                        lex_with_spans(&src).map_err(|e| format_lex_error_json(&label, &src, e))?;
                    println!("{}", format_tokens_json(&tokens));
                }
            }
            Ok(())
        }
        Command::Parse {
            file,
            stdin,
            format,
        } => {
            let (src, label) = read_source(file.as_ref(), stdin)?;
            let program = match format {
                OutputFormat::Pretty => {
                    parse_program(&src).map_err(|e| format_parse_error(&label, &src, e))?
                }
                OutputFormat::Json => {
                    parse_program(&src).map_err(|e| format_parse_error_json(&label, &src, e))?
                }
            };
            match format {
                OutputFormat::Pretty => println!("{program:#?}"),
                OutputFormat::Json => println!("{}", format_ast_json(&program)),
            }
            Ok(())
        }
        Command::Check {
            file,
            stdin,
            format,
        } => {
            let (src, label) = read_source(file.as_ref(), stdin)?;
            let program = match format {
                OutputFormat::Pretty => {
                    parse_program(&src).map_err(|e| format_parse_error(&label, &src, e))?
                }
                OutputFormat::Json => {
                    parse_program(&src).map_err(|e| format_parse_error_json(&label, &src, e))?
                }
            };
            let warnings = match format {
                OutputFormat::Pretty => typecheck_program_with_warnings(&program)
                    .map_err(|e| format_type_error(&label, &src, &e))?,
                OutputFormat::Json => typecheck_program_with_warnings(&program)
                    .map_err(|e| format_type_error_json(&label, &src, &e))?,
            };
            match format {
                OutputFormat::Pretty => {
                    for warning in warnings {
                        eprintln!("{}", format_type_warning(&warning));
                    }
                    println!("OK");
                }
                OutputFormat::Json => {
                    println!("{}", format_check_json(&warnings));
                }
            }
            Ok(())
        }
        Command::Codegen {
            file,
            stdin,
            format,
        } => {
            let (src, label) = read_source(file.as_ref(), stdin)?;
            let program = match format {
                OutputFormat::Pretty => {
                    parse_program(&src).map_err(|e| format_parse_error(&label, &src, e))?
                }
                OutputFormat::Json => {
                    parse_program(&src).map_err(|e| format_parse_error_json(&label, &src, e))?
                }
            };
            let _warnings = match format {
                OutputFormat::Pretty => typecheck_program_with_warnings(&program)
                    .map_err(|e| format_type_error(&label, &src, &e))?,
                OutputFormat::Json => typecheck_program_with_warnings(&program)
                    .map_err(|e| format_type_error_json(&label, &src, &e))?,
            };
            let output = generate_stub_with_types(&program);
            match format {
                OutputFormat::Pretty => println!("{output}"),
                OutputFormat::Json => println!("{}", format_codegen_json(&output)),
            }
            Ok(())
        }
        Command::Audit {
            file,
            stdin,
            input,
            format,
        } => {
            let (src, label) = read_source(file.as_ref(), stdin)?;
            let program = parse_program(&src).map_err(|e| format_parse_error(&label, &src, e))?;
            let inputs = read_inputs(input.as_ref())?;
            let hash = hash_execution(&program, &inputs);
            let ast_fingerprint = hash_ast(&program);
            match format {
                OutputFormat::Pretty => {
                    println!("{hash}");
                }
                OutputFormat::Json => {
                    let value = json!({
                        "hash": hash.to_string(),
                        "ast_fingerprint": ast_fingerprint.to_string(),
                        "compiler_version": compiler_version(),
                    });
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&value)
                            .unwrap_or_else(|_| value.to_string())
                    );
                }
            }
            Ok(())
        }
        Command::Version => {
            println!(env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::About => {
            println!("Tupã CLI");
            println!("Parse and typecheck Tupã source files");
            Ok(())
        }
    }
}

fn read_source(file: Option<&PathBuf>, stdin: bool) -> Result<(String, String), String> {
    if stdin {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("stdin: {e}"))?;
        return Ok((buf, "<stdin>".to_string()));
    }
    match file {
        Some(path) => fs::read_to_string(path)
            .map(|src| (src, path.display().to_string()))
            .map_err(|e| format!("{path:?}: {e}")),
        None => Err("missing file path or --stdin".to_string()),
    }
}

fn read_inputs(file: Option<&PathBuf>) -> Result<Vec<Value>, String> {
    match file {
        None => Ok(Vec::new()),
        Some(path) => {
            let src = fs::read_to_string(path).map_err(|e| format!("{path:?}: {e}"))?;
            let value: Value =
                serde_json::from_str(&src).map_err(|e| format!("{path:?}: {e}"))?;
            match value {
                Value::Array(items) => Ok(items),
                _ => Err(format!("{path:?}: expected a JSON array")),
            }
        }
    }
}

fn format_lex_error(label: &str, src: &str, err: LexerError) -> String {
    match err {
        LexerError::Unexpected(ch, pos) => format_diagnostic(
            label,
            src,
            &format!("error: unexpected character '{ch}'"),
            Span {
                start: pos,
                end: pos + ch.len_utf8(),
            },
        ),
    }
}

fn format_lex_error_json(label: &str, src: &str, err: LexerError) -> String {
    match err {
        LexerError::Unexpected(ch, pos) => diagnostic_json(
            label,
            src,
            &format!("unexpected character '{ch}'"),
            Some(Span {
                start: pos,
                end: pos + ch.len_utf8(),
            }),
            None,
        ),
    }
}

fn format_parse_error(label: &str, src: &str, err: ParserError) -> String {
    match err {
        ParserError::Unexpected(token, span) => format_diagnostic(
            label,
            src,
            &format!("error: unexpected token {token:?}"),
            span,
        ),
        ParserError::MissingSemicolon(span) => {
            format_diagnostic(label, src, "error: expected ';' after expression", span)
        }
        ParserError::Eof(pos) => format_diagnostic(
            label,
            src,
            "error: unexpected end of input",
            Span {
                start: pos,
                end: pos,
            },
        ),
        ParserError::Lexer(message) => message,
    }
}

fn format_parse_error_json(label: &str, src: &str, err: ParserError) -> String {
    match err {
        ParserError::Unexpected(token, span) => diagnostic_json(
            label,
            src,
            &format!("unexpected token {token:?}"),
            Some(span),
            None,
        ),
        ParserError::MissingSemicolon(span) => diagnostic_json(
            label,
            src,
            "expected ';' after expression",
            Some(span),
            None,
        ),
        ParserError::Eof(pos) => diagnostic_json(
            label,
            src,
            "unexpected end of input",
            Some(Span {
                start: pos,
                end: pos,
            }),
            None,
        ),
        ParserError::Lexer(message) => json!({"error": {"message": message}}).to_string(),
    }
}

fn type_error_code(err: &TypeError) -> &'static str {
    match err {
        TypeError::UnknownType { .. } => "E1001",
        TypeError::UnknownVar { .. } => "E1002",
        TypeError::UnknownFunction { .. } => "E1003",
        TypeError::InvalidTypeArity { .. } => "E1004",
        TypeError::UnknownVariant { .. } => "E1005",
        TypeError::Mismatch { .. } => "E2001",
        TypeError::ArityMismatch { .. } => "E2002",
        TypeError::InvalidBinary { .. } => "E2003",
        TypeError::InvalidUnary { .. } => "E2004",
        TypeError::InvalidCallTarget { .. } => "E2005",
        TypeError::ReturnMismatch { .. } => "E2006",
        TypeError::MissingReturn { .. } => "E2007",
        TypeError::InvalidConstraint { .. } => "E3001",
        TypeError::UnprovenConstraint { .. } => "E3002",
        TypeError::BreakOutsideLoop { .. } => "E4001",
        TypeError::ContinueOutsideLoop { .. } => "E4002",
        TypeError::NonExhaustiveMatch { .. } => "E5001",
    }
}

fn format_type_error(label: &str, src: &str, err: &TypeError) -> String {
    let code = type_error_code(err);
    let mut header = format!("error[{code}]: {err}");
    if let Some(help) = type_error_help(err) {
        header = format!("{header}\n{help}");
    }
    match type_error_span(err) {
        Some(span) => format_diagnostic(label, src, &header, span),
        None => header,
    }
}

fn format_type_error_json(label: &str, src: &str, err: &TypeError) -> String {
    let code = type_error_code(err);
    let mut message = err.to_string();
    if let Some(help) = type_error_help(err) {
        message = format!("{message}\n{help}");
    }
    diagnostic_json(label, src, &message, type_error_span(err), Some(code))
}

fn type_error_help(err: &TypeError) -> Option<String> {
    match err {
        TypeError::InvalidConstraint { .. } => {
            Some("help: supported constraints are !nan and !inf on f64 values, and !hate_speech and !misinformation on string values".to_string())
        }
        TypeError::InvalidTypeArity { .. } => {
            Some("help: check the number of generic arguments in the type".to_string())
        }
        TypeError::UnprovenConstraint { constraint, .. } => match constraint.as_str() {
            "misinformation" | "hate_speech" => Some(
                "help: add safety proof: `@safety(score=0.98, dataset=\"factcheck-v3\")`"
                    .to_string(),
            ),
            _ => Some("help: constraint must be provable at compile time; use a provable literal or pass a Safe value already proven".to_string()),
        },
        TypeError::NonExhaustiveMatch { .. } => {
            Some("help: add missing patterns or a wildcard arm to cover all cases".to_string())
        }
        TypeError::UnknownVariant { .. } => Some("help: check the enum variant name".to_string()),
        _ => None,
    }
}

fn format_type_warning(warning: &Warning) -> String {
    match warning {
        Warning::UnusedVar(name) => format!("warning[W0001]: unused variable '{name}'"),
    }
}

fn format_type_warning_json(warning: &Warning) -> Value {
    match warning {
        Warning::UnusedVar(name) => json!({
            "code": "W0001",
            "message": format!("unused variable '{name}'"),
            "name": name,
        }),
    }
}

fn format_diagnostic(label: &str, src: &str, message: &str, span: Span) -> String {
    let (line, col) = line_col(src, span.start);
    let line_text = src.lines().nth(line.saturating_sub(1)).unwrap_or("");
    let caret_len = (span.end.saturating_sub(span.start)).max(1);
    let mut caret = String::new();
    if col > 1 {
        caret.push_str(&" ".repeat(col - 1));
    }
    caret.push_str(&"^".repeat(caret_len));

    format!("{message}\n  --> {label}:{line}:{col}\n   |\n {line} | {line_text}\n   | {caret}")
}

fn diagnostic_json(
    label: &str,
    src: &str,
    message: &str,
    span: Option<Span>,
    code: Option<&str>,
) -> String {
    let (line, col, line_text) = match span {
        Some(span) => {
            let (line, col) = line_col(src, span.start);
            let line_text = src.lines().nth(line.saturating_sub(1)).unwrap_or("");
            (json!(line), json!(col), json!(line_text))
        }
        None => (Value::Null, Value::Null, Value::Null),
    };
    let span_value = span
        .map(|span| json!({ "start": span.start, "end": span.end }))
        .unwrap_or(Value::Null);
    let code_value = code.map(|value| json!(value)).unwrap_or(Value::Null);
    let value = json!({
        "error": {
            "code": code_value,
            "message": message,
            "label": label,
            "span": span_value,
            "line": line,
            "col": col,
            "line_text": line_text
        }
    });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
}

fn format_tokens_json(tokens: &[TokenSpan]) -> String {
    let entries: Vec<Value> = tokens.iter().map(token_span_json).collect();
    let value = json!({ "tokens": entries });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
}

fn token_span_json(token: &TokenSpan) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("kind".to_string(), Value::String(token_kind(&token.token)));
    if let Some(value) = token_value(&token.token) {
        map.insert("value".to_string(), Value::String(value));
    }
    map.insert(
        "span".to_string(),
        json!({ "start": token.span.start, "end": token.span.end }),
    );
    Value::Object(map)
}

fn token_kind(token: &Token) -> String {
    match token {
        Token::Fn => "Fn",
        Token::Enum => "Enum",
        Token::Trait => "Trait",
        Token::Let => "Let",
        Token::Return => "Return",
        Token::If => "If",
        Token::Else => "Else",
        Token::Match => "Match",
        Token::While => "While",
        Token::For => "For",
        Token::Break => "Break",
        Token::Continue => "Continue",
        Token::In => "In",
        Token::Await => "Await",
        Token::True => "True",
        Token::False => "False",
        Token::Null => "Null",
        Token::Ident(_) => "Ident",
        Token::Int(_) => "Int",
        Token::Float(_) => "Float",
        Token::Str(_) => "Str",
        Token::LParen => "LParen",
        Token::RParen => "RParen",
        Token::LBrace => "LBrace",
        Token::RBrace => "RBrace",
        Token::LBracket => "LBracket",
        Token::RBracket => "RBracket",
        Token::Semicolon => "Semicolon",
        Token::Comma => "Comma",
        Token::Colon => "Colon",
        Token::Equal => "Equal",
        Token::Arrow => "Arrow",
        Token::ThinArrow => "ThinArrow",
        Token::EqualEqual => "EqualEqual",
        Token::BangEqual => "BangEqual",
        Token::Less => "Less",
        Token::LessEqual => "LessEqual",
        Token::Greater => "Greater",
        Token::GreaterEqual => "GreaterEqual",
        Token::AndAnd => "AndAnd",
        Token::OrOr => "OrOr",
        Token::Plus => "Plus",
        Token::PlusEqual => "PlusEqual",
        Token::Minus => "Minus",
        Token::MinusEqual => "MinusEqual",
        Token::Star => "Star",
        Token::StarEqual => "StarEqual",
        Token::Slash => "Slash",
        Token::SlashEqual => "SlashEqual",
        Token::DoubleStar => "DoubleStar",
        Token::DotDot => "DotDot",
        Token::Dot => "Dot",
        Token::Bang => "Bang",
        Token::Pipe => "Pipe",
        Token::At => "At",
    }
    .to_string()
}

fn token_value(token: &Token) -> Option<String> {
    match token {
        Token::Ident(value) | Token::Int(value) | Token::Float(value) | Token::Str(value) => {
            Some(value.clone())
        }
        _ => None,
    }
}

fn format_ast_json(program: &tupa_parser::Program) -> String {
    let value = json!({ "ast": format!("{program:#?}") });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
}

fn format_check_json(warnings: &[Warning]) -> String {
    let warnings_json: Vec<Value> = warnings.iter().map(format_type_warning_json).collect();
    let value = json!({ "status": "ok", "warnings": warnings_json });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
}

fn format_codegen_json(output: &str) -> String {
    let value = json!({ "codegen": output });
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string())
}

fn type_error_span(err: &TypeError) -> Option<Span> {
    match err {
        TypeError::UnknownVar { span, .. }
        | TypeError::UnknownFunction { span, .. }
        | TypeError::UnknownVariant { span, .. }
        | TypeError::Mismatch { span, .. }
        | TypeError::ArityMismatch { span, .. }
        | TypeError::InvalidBinary { span, .. }
        | TypeError::InvalidUnary { span, .. }
        | TypeError::InvalidCallTarget { span, .. }
        | TypeError::ReturnMismatch { span, .. }
        | TypeError::InvalidConstraint { span, .. }
        | TypeError::UnprovenConstraint { span, .. }
        | TypeError::BreakOutsideLoop { span, .. }
        | TypeError::ContinueOutsideLoop { span, .. }
        | TypeError::MissingReturn { span, .. }
        | TypeError::NonExhaustiveMatch { span, .. } => *span,
        TypeError::UnknownType { .. } | TypeError::InvalidTypeArity { .. } => None,
    }
}

fn line_col(src: &str, pos: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (idx, ch) in src.char_indices() {
        if idx >= pos {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
