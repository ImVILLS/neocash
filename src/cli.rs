// src/cli.rs

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Show version information
    #[arg(short, long)]
    pub version: bool,

    /// Check for available updates
    #[arg(long, requires = "version")]
    pub check_updates: bool,

    /// Show detailed version information
    #[arg(short = 'V', long, requires = "version")]
    pub verbose: bool,

    /// Execute neocash with a configuration file from the specified path
    #[arg(long)]
    pub config_path: Option<PathBuf>,

    /// Do not load or save command history
    #[arg(short, long)]
    pub no_history: bool,
}
