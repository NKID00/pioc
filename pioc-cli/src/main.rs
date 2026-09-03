use std::{
    fs,
    io::{Read, Write, stderr, stdin, stdout},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use eyre::Result;
use pioc::Inst;
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
        .with_writer(stderr)
        .init();
    match cli.command {
        Commands::As { output, input } => {
            let assembly = if input == *"-" {
                let mut buf = String::new();
                stdin().read_to_string(&mut buf)?;
                buf
            } else {
                fs::read_to_string(input)?
            };
            let bytes = pioc::assemble(assembly)?;
            if bytes.is_empty() {
                warn!("assembler emits no instruction");
            }
            match output {
                Some(output) if output != *"-" => {
                    fs::write(output, bytes)?;
                }
                _ => stdout().write_all(bytes.as_slice())?,
            }
        }
        Commands::Dis { output, input } => {
            let bytes = if input == *"-" {
                let mut buf = vec![];
                stdin().read_to_end(&mut buf)?;
                buf
            } else {
                fs::read(input)?
            };
            let (chunks, []) = bytes.as_chunks() else {
                panic!("extra byte found, EOF should be at 16-bit word border");
            };
            let asm: String = chunks
                .iter()
                .map(|chunk| {
                    let inst = Inst::from_bytes(*chunk).to_wch_risc8b_asm();
                    format!("    {inst}\n")
                })
                .collect();
            match output {
                Some(output) if output != *"-" => {
                    fs::write(output, asm.as_bytes())?;
                }
                _ => print!("{}", asm),
            }
        }
        Commands::AsOne { mut assembly } => {
            assembly.insert(0, ' '); // insert a no-label mark
            let statements = pioc::parse_line(assembly)?;
            let instructions = pioc::assemble_parsed(statements.as_slice())?;
            match instructions.as_slice() {
                [] => warn!("assembler emits no instruction"),
                [_] => {}
                _ => warn!("assembler emits {} instructions", instructions.len()),
            }
            for inst in instructions {
                println!("{:#06x}", inst.to_word());
            }
        }
        Commands::DisOne { value } => {
            let value = value.trim().to_lowercase();
            let value = value.strip_prefix("0x").unwrap_or(&value);
            let word = u16::from_str_radix(value, 16)?;
            println!("{}", Inst::from_word(word).to_wch_risc8b_asm());
        }
    }
    Ok(())
}
