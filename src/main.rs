// src/main.rs
/*
 * Main executable for TokenizedArtifactsManagerFrameworkX
 */

use clap::Parser;
use tokenizedartifactsmanagerframeworkx::{Result, run};

#[derive(Parser)]
#[command(version, about = "TokenizedArtifactsManagerFrameworkX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
