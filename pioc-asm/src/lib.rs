mod assemble;
mod ast;
mod parse;

pub use assemble::*;
pub use ast::*;
pub use parse::*;

#[cfg(test)]
mod tests {
    use super::*;
    use pioc_core::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("NOP\nNOP").unwrap(),
            vec![Stmt::Inst(None, OpCode::Nop), Stmt::Inst(None, OpCode::Nop)]
        );
    }

    #[test]
    fn test_assemble() {
        assert_eq!(
            assemble(&vec![
                Stmt::Inst(None, OpCode::Nop),
                Stmt::Inst(None, OpCode::Nop)
            ])
            .unwrap(),
            vec![0, 0]
        );
    }
}
