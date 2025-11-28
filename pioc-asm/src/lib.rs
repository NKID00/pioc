mod ast;
mod parser;

pub use ast::*;
pub use parser::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid op code")]
    InvalidOpCode,
}

pub fn parse(asm: impl AsRef<str>) -> Result<Vec<Statement>, ParseError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum AssembleError {
    #[error("invalid op code")]
    InvalidOpCode,
}

pub fn assemble(prog: &[Statement]) -> Result<Vec<u16>, AssembleError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pioc_core::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("NOP\nNOP").unwrap(),
            vec![
                Statement::Instruction(OpCode::Nop),
                Statement::Instruction(OpCode::Nop)
            ]
        );
    }

    #[test]
    fn test_assemble() {
        assert_eq!(
            assemble(&vec![
                Statement::Instruction(OpCode::Nop),
                Statement::Instruction(OpCode::Nop)
            ])
            .unwrap(),
            vec![0, 0]
        );
    }
}
