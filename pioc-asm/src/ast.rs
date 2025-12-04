use pioc_core::OpCode;

use derive_more::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct Ident(pub String);

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    Num(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Define(Ident, Expr),
    Origin(Expr),
    Include(String),
    /// Optional label, and opcode
    Inst(Option<Ident>, OpCode),
}
