use crate::Stmt;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssembleError {
    #[error("invalid op code")]
    InvalidOpCode,
}

pub fn assemble(prog: &[Stmt]) -> Result<Vec<u16>, AssembleError> {
    todo!()
}
