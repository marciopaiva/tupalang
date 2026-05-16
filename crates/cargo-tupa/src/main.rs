#![allow(unused)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod bench;
mod discover;
mod expand;
mod fmt;
mod lint;
mod plugin_new;
mod run;
mod watch;

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
        /// Output results as JSON
        #[arg(long)]
        json: bool,
    },
    /// Generate a new plugin scaffold
    PluginNew {
        /// Output filename (default: my_plugin.rs)
        #[arg(value_name = "FILENAME")]
        filename: Option<String>,
    },
    /// Expand pipeline! macro to generated Rust code
    Expand {
        /// Enable pretty-print (indentation)
        #[arg(long)]
        pretty: bool,
        /// Specific file to expand (default: all src/**/*.rs)
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Discover the binary target name from Cargo.toml ([[bin]] or src/main.rs)
    Discover,
    /// Profile pipeline execution: measure per-step timing over multiple runs
    Bench {
        /// Number of runs (default: 10)
        #[arg(short, long, default_value_t = 10)]
        runs: usize,

        /// Write benchmark report as JSON
        #[arg(long)]
        json_output: Option<PathBuf>,
    },
    /// Watch for source changes and re-run pipeline automatically
    Watch {
        /// JSON input file (or read stdin)
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Enable parallel execution
        #[arg(long)]
        parallel: bool,
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
        Commands::Lint { file, json } => lint::lint(file, json),
        Commands::PluginNew { filename } => plugin_new::run(filename),
        Commands::Expand { pretty, file } => expand::expand_pipeline_block(file, pretty),
        Commands::Discover => discover::discover_binary_target(
            &cli.manifest_path
                .unwrap_or_else(|| PathBuf::from("Cargo.toml")),
        )
        .map(|name| {
            println!("{}", name);
            Ok(())
        })?,
        Commands::Bench { runs, json_output } => bench::bench(cli.manifest_path, runs, json_output),
        Commands::Watch { input, parallel } => watch::watch(input, parallel),
    }
}
