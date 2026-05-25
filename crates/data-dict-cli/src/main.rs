use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "data-dict", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a data-dict.yaml file
    Validate { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Validate { path } => {
            println!("validate: {}", path.display());
        }
    }
}
