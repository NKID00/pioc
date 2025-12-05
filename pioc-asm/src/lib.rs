mod assemble;
mod ast;
mod parse;

pub use assemble::*;
pub use ast::*;
pub use parse::*;

/// Convenient function to parse and assemble an assembly program.
pub fn assemble_to_words(asm: String) -> Result<Vec<u16>, AssembleError> {
    let statements = parse(asm)?;
    let instructions = assemble(&statements)?;
    let words = instructions
        .into_iter()
        .map(|inst| inst.to_word())
        .collect();
    Ok(words)
}
