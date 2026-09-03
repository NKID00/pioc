mod assemble;
mod ast;
mod parse;
mod symbol;

pub use assemble::*;
pub use ast::*;
pub use parse::*;
pub use symbol::*;

/// Convenient function to parse and assemble an assembly program.
pub fn assemble(asm: String) -> Result<Vec<u8>, AssembleError> {
    let statements = parse(asm)?;
    let instructions = assemble_parsed(&statements)?;
    let words = instructions
        .into_iter()
        .flat_map(|inst| inst.to_bytes())
        .collect();
    Ok(words)
}
