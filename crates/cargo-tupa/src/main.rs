#![allow(deprecated)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cargo-tupa")]
#[command(about = "Tupã pipeline tooling", long_about = None)]
struct Cli {
    /// Path to Cargo.toml (default: current directory)
    #[arg(short, long, value_name = "manifest", global = true)]
    manifest_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Typecheck the pipeline macro (no execution)
    Check {
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Execute the pipeline with input data
    Run {
        /// JSON input file (or read stdin)
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Enable parallel execution
        #[arg(long)]
        parallel: bool,

        /// Run a specific example (name)
        #[arg(long)]
        example: Option<String>,

        /// Run a specific binary target (name)
        #[arg(long)]
        bin: Option<String>,
    },
    /// Format legacy .tp files
    Fmt {
        /// Files to format (default: all .tp in src/)
        files: Vec<PathBuf>,
    },
    /// Lint pipeline for issues
    Lint {
        /// Treat warnings as errors
        #[arg(short, long)]
        deny_warnings: bool,
    },
    /// Run pipeline tests (examples with #[cfg(test)])
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

mod check;
mod fmt;
mod lint;
mod plugin_new;
mod run;
mod test_cmd;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { verbose } => {
            check::run(&cli.manifest_path, verbose)?;
        }
        Commands::Run {
            input,
            parallel,
            example,
            bin,
        } => {
            run::run(&cli.manifest_path, input, parallel, example, bin)?;
        }
        Commands::Fmt { files } => {
            fmt::run(&cli.manifest_path, files)?;
        }
        Commands::Lint { deny_warnings } => {
            lint::run(&cli.manifest_path, deny_warnings)?;
        }
        Commands::Test { filter } => {
            test_cmd::run(&cli.manifest_path, filter)?;
        }
        Commands::PluginNew { filename } => {
            plugin_new::run(filename)?;
        }
    }

    Ok(())
}
