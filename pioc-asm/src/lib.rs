mod ast;
mod parser;

pub use ast::*;
pub use parser::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {}

pub fn parse(asm: impl AsRef<str>) -> Result<Vec<Statement>, ParseError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum AssembleError {}

pub fn assemble(prog: &Vec<Statement>) -> Result<Vec<u16>, AssembleError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
