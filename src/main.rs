use anyhow::Result;
use clap::Parser;

use crate::runner::{run_file, run_prompt};

mod runner;
mod scanner;

/// Rust implementation of Lox interpreter
#[derive(Parser, Debug)]
struct CliArgs {
    /// Source code file path 
    #[arg(short, long)]
    file_path: Option<String>,
}

fn main() -> Result<()> {
    let cli = CliArgs::parse();

    match cli.file_path {
        Some(file_path) => run_file(&file_path),
        None => run_prompt(),
    }
}
