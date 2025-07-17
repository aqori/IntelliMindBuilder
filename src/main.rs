// src/main.rs
/*
 * Main executable for IntelliMindBuilder
 */

use clap::Parser;
use intellimindbuilder::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliMindBuilder - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
