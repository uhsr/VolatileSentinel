// src/main.rs
/*
 * Main executable for VolatileSentinel
 */

use clap::Parser;
use volatilesentinel::{Result, run};

#[derive(Parser)]
#[command(version, about = "VolatileSentinel - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
