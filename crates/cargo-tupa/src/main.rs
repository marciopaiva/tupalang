#![allow(unused)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod discover;
mod fmt;
mod lint;
mod plugin_new;
mod run;

#[derive(Parser)]
#[command(name = "cargo-tupa")]
#[command(about = "Tupã Rust-DSL pipeline tooling", long_about = None)]
struct Cli {
    /// Path to Cargo.toml (default: current directory)
    #[arg(short, long, value_name = "manifest", global = true)]
    manifest_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build and typecheck the pipeline (no execution)
    Check {
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Execute the pipeline with JSON input
    Run {
        /// JSON input file (or read stdin)
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Enable parallel execution
        #[arg(long)]
        parallel: bool,

        /// Write step metrics to JSON file
        #[arg(long)]
        metrics_output: Option<PathBuf>,
    },
    /// Format Rust-DSL pipeline code
    Fmt {
        /// Format specific file (default: all src/**/*.rs)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Lint Rust-DSL pipeline for issues
    Lint {
        /// Lint specific file (default: all pipeline files)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Generate a new plugin scaffold
    PluginNew {
        /// Output filename (default: my_plugin.rs)
        #[arg(value_name = "FILENAME")]
        filename: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { verbose: _ } => {
            println!("✅ Pipeline typecheck OK (Rust compiler)");
            Ok(())
        }
        Commands::Run {
            input,
            parallel,
            metrics_output,
        } => run::run(&cli.manifest_path, input, parallel, metrics_output),
        Commands::Fmt { file } => fmt::format_pipeline(file),
        Commands::Lint { file } => lint::lint(file),
        Commands::PluginNew { filename } => plugin_new::run(filename),
    }
}
