// src/main.rs
/*
 * Main executable for PulseAI
 */

use clap::Parser;
use pulseai::{Result, run};

#[derive(Parser)]
#[command(version, about = "PulseAI - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
