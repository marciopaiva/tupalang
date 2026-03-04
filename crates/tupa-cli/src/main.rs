use clap::{Parser, Subcommand};
use std::process;

mod run;

#[derive(Parser)]
#[command(name = "tupa")]
#[command(about = "Tupã Language CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a Tupã program or pipeline
    Run {
        /// Input file (optional if --plan is used)
        #[arg(required_unless_present = "plan")]
        file: Option<String>,
        /// Pipeline to run (optional)
        #[arg(long)]
        pipeline: Option<String>,
        /// Input data file (JSON)
        #[arg(long)]
        input: Option<String>,
        /// Execute a pre-compiled plan file
        #[arg(long)]
        plan: Option<String>,
    },
    /// Check syntax and types
    Check {
        /// Input file
        file: String,
        /// Output format (text/json)
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Audit execution logs
    Audit {
        /// Input file
        file: String,
        /// Input data file (JSON)
        #[arg(long)]
        input: Option<String>,
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Parse and show AST
    Parse {
        /// Input file
        file: String,
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Lex and show tokens
    Lex {
        /// Input file
        file: String,
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// Generate code (LLVM/Rust)
    Codegen {
        /// Input file
        file: String,
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
        /// Generate execution plan only (JSON)
        #[arg(long)]
        plan_only: bool,
    },
    /// Analyze side effects
    Effects {
        /// Input file
        file: String,
        /// Output format
        #[arg(long, default_value = "text")]
        format: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run::run(cli.command).await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
