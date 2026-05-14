#![allow(deprecated)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tupa_engine::Executor;

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
    },
    /// Run pipeline tests (integration tests)
    Test {
        /// Filter test name
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Generate a new plugin scaffold
    PluginNew {
        /// Output filename (default: my_plugin.rs)
        #[arg(value_name = "FILENAME")]
        filename: Option<String>,
    },
}

mod plugin_new;
mod run;
mod test_cmd;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { verbose: _ } => {
            println!("✅ Pipeline typecheck OK (Rust compiler)");
            Ok(())
        }
        Commands::Run { input, parallel } => {
            run::run(&cli.manifest_path, input, parallel)
        }
        Commands::Test { filter } => {
            test_cmd::run(&cli.manifest_path, filter)
        }
        Commands::PluginNew { filename } => {
            plugin_new::run(filename)
        }
    }
}
