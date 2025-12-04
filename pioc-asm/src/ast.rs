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
    Label(Ident),
    Num(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString)]
pub enum Mnemonic {
    NOP,
    CLRWDT,
    SLEEP,
    SLEEPX,
    WAITB,
    WAITRO,
    WAITWR,
    WAITSPI,
    RDCODE,
    RCODE,
    WRCODE,
    EXEC,
    PUSHAS,
    POPAS,
    PUSHA2,
    POPA2,
    RET,
    RETZ,
    RETIE,
    CLRA,
    CLR,
    MOVA,
    MOV,
    INC,
    DEC,
    INCSZ,
    DECSZ,
    SWAP,
    AND,
    IOR,
    XOR,
    ADD,
    SUB,
    RCL,
    RCR,
    RETL,
    RETLN,
    MOVIP,
    MOVIA,
    MOVA1F,
    MOVA2F,
    MOVA2P,
    MOVA1P,
    MOVL,
    ANDL,
    IORL,
    XORL,
    ADDL,
    SUBL,
    CMPLN,
    CMPL,
    BC,
    BS,
    BTSC,
    BTSS,
    BCTC,
    BP1F,
    BP2F,
    BG1F,
    BG2F,
    JMP,
    CALL,
    JNZ,
    JZ,
    JNC,
    JC,
    CMPZ,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Op0,
    Op1(Expr),
    Op2(Expr, Expr),
}

/// Represents a raw assembly statement with unresolved symbols and could be invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Define(Ident, Expr),
    Origin(Expr),
    Include(String),
    /// Label, mnemonic and two operands
    Inst(Option<Ident>, Mnemonic, Operand),
}
