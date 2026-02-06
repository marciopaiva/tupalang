use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tupa_parser::parse_program;
use tupa_typecheck::typecheck_program_with_warnings;
use tupa_lexer::lex;

#[derive(Parser)]
#[command(name = "tupa", version, about = "Tupã CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
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
    },
    /// Parse a .tp file and print the AST
    Parse {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
    },
    /// Parse and typecheck a .tp file
    Check {
        /// Path to the source file
        file: Option<PathBuf>,
        /// Read source from stdin
        #[arg(long)]
        stdin: bool,
    },
    /// Print CLI version
    Version,
    /// Print CLI about
    About,
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Lex { file, stdin } => {
            let src = read_source(file.as_ref(), stdin)?;
            let tokens = lex(&src).map_err(|e| e.to_string())?;
            for tok in tokens {
                println!("{tok:?}");
            }
            Ok(())
        }
        Command::Parse { file, stdin } => {
            let src = read_source(file.as_ref(), stdin)?;
            let program = parse_program(&src).map_err(|e| e.to_string())?;
            println!("{program:#?}");
            Ok(())
        }
        Command::Check { file, stdin } => {
            let src = read_source(file.as_ref(), stdin)?;
            let program = parse_program(&src).map_err(|e| e.to_string())?;
            let warnings = typecheck_program_with_warnings(&program).map_err(|e| e.to_string())?;
            for warning in warnings {
                eprintln!("warning: {warning:?}");
            }
            println!("OK");
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

fn read_source(file: Option<&PathBuf>, stdin: bool) -> Result<String, String> {
    if stdin {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("stdin: {e}"))?;
        return Ok(buf);
    }
    match file {
        Some(path) => fs::read_to_string(path).map_err(|e| format!("{path:?}: {e}")),
        None => Err("missing file path or --stdin".to_string()),
    }
}
