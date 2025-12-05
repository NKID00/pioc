use std::path::PathBuf;

use clap::{Parser, Subcommand};
use eyre::Result;
use tracing::{Level, warn};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(
                    match cli.verbose {
                        0 => Level::INFO,
                        1 => Level::DEBUG,
                        2.. => Level::TRACE,
                    }
                    .into(),
                )
                .from_env_lossy(),
        )
        .init();
    match cli.command {
        Commands::As { output, input } => {
            todo!();
        }
        Commands::Dis { output, input } => {
            todo!();
        }
        Commands::AsOne { assembly } => {
            let statements = pioc::parse_line(assembly)?;
            let instructions = pioc::assemble(statements.as_slice())?;
            match instructions.as_slice() {
                [] => warn!("assembler emits no instruction"),
                [_] => {}
                _ => warn!("assembler emits {} instructions", instructions.len()),
            }
            for inst in instructions {
                println!("{:#04x}", inst.to_word());
            }
        }
        Commands::DisOne { value } => {
            todo!();
        }
    }
    Ok(())
}
