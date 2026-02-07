//! novel-finder: A CLI tool to find the perfect webnovel on RoyalRoad.
//!
//! This tool evaluates novels against user-defined criteria using configurable
//! evaluation strategies (local heuristics or LLM-based analysis) and discovers
//! related novels through RoyalRoad's recommendation system.

mod config;
mod discovery;
mod eval;
mod models;
mod output;
mod pipeline;
mod queue;
mod scraper;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Find the perfect webnovel on RoyalRoad.
#[derive(Parser, Debug)]
#[command(name = "novel-finder", version, about)]
struct Cli {
    /// Path to the configuration TOML file.
    #[arg(short, long)]
    config: PathBuf,

    /// Enable verbose/debug logging output.
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();

    tracing::info!("novel-finder starting up");
    tracing::debug!("Config path: {}", cli.config.display());

    // Load configuration
    let app_config = config::load_config(&cli.config)?;
    tracing::info!("Configuration loaded successfully");

    // Build and run the pipeline
    let mut pipeline = pipeline::Pipeline::new(app_config)?;
    let results = pipeline.run()?;

    // Output results
    output::print_results(&results);

    Ok(())
}
