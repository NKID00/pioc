use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Assemble a PIOC assembly file
    As {
        /// Output binary file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Input assembly file
        input: PathBuf,
    },
    /// Disassemble a PIOC binary file
    Dis {
        /// Output assembly file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Input binary file
        input: PathBuf,
    },
    /// Assemble a single line of PIOC assembly
    AsOne {
        /// Input assembly
        assembly: String,
    },
    /// Disassemble a single PIOC instruction
    DisOne {
        /// Input instruction value
        value: String,
    },
}

fn main() {
    let cli = Cli::parse();
}
