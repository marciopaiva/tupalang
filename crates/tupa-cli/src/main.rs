use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tupa_parser::parse_program;
use tupa_typecheck::typecheck_program;

#[derive(Parser)]
#[command(name = "tupa", version, about = "Tupã CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse a .tp file and print the AST
    Parse {
        /// Path to the source file
        file: PathBuf,
    },
    /// Parse and typecheck a .tp file
    Check {
        /// Path to the source file
        file: PathBuf,
    },
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
        Command::Parse { file } => {
            let src = read_file(&file)?;
            let program = parse_program(&src).map_err(|e| e.to_string())?;
            println!("{program:#?}");
            Ok(())
        }
        Command::Check { file } => {
            let src = read_file(&file)?;
            let program = parse_program(&src).map_err(|e| e.to_string())?;
            typecheck_program(&program).map_err(|e| e.to_string())?;
            println!("OK");
            Ok(())
        }
    }
}

fn read_file(path: &PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{path:?}: {e}"))
}
