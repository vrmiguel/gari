use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "A tool to clean up build artifacts and reclaim disk space")]
pub struct Cli {
    /// Paths to scan for cleanable projects (defaults to current directory)
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// Only show reclaimable space without performing cleanup
    #[arg(long, short)]
    pub check: bool,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
