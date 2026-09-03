use std::fmt::{Display, Formatter};

use derive_more::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, DerefMut)]
pub struct Ident(pub String);

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Label(Ident),
    Const(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Lsh(Box<Expr>, Box<Expr>),
    Rsh(Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Label(ident) => write!(f, "{}", ident.0),
            Expr::Const(v) => write!(f, "{v}"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Sub(a, b) => write!(f, "({a} - {b})"),
            Expr::Neg(a) => write!(f, "(-{a})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Div(a, b) => write!(f, "({a} / {b})"),
            Expr::Rem(a, b) => write!(f, "({a} % {b})"),
            Expr::And(a, b) => write!(f, "({a} & {b})"),
            Expr::Or(a, b) => write!(f, "({a} | {b})"),
            Expr::Xor(a, b) => write!(f, "({a} ^ {b})"),
            Expr::Not(a) => write!(f, "(~{a})"),
            Expr::Lsh(a, b) => write!(f, "({a} << {b})"),
            Expr::Rsh(a, b) => write!(f, "({a} >> {b})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString)]
pub enum Mnemonic {
    NOP,
    CLRWDT,
    SLEEP,
    SLEEPX,
    WAITB,
    WAITRD,
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
    WDT,
    HALT,
    PUSH,
    POP,
    RETURN,
    RETOK,
    RETI,
    DB,
    RETER,
    GOTO,
    CLRF,
    MOVAF,
    MOVF,
    INCF,
    DECF,
    SWAPF,
    ANDF,
    IORF,
    XORF,
    ADDF,
    SUBF,
    RCLF,
    RCRF,
    BCF,
    BSF,
    BCTCF,
    INCFSZ,
    DECFSZ,
    RLF,
    RRF,
    BTFSC,
    BTFSS,
    DW,
}

/// Raw operand with unresolved symbols and could be invalid
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Op0,
    Op1(Expr),
    Op2(Expr, Expr),
}

/// Raw assembly statement with unresolved symbols and could be invalid
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Define(Ident, Expr),
    /// Origin address in instructions/words
    Origin(Expr),
    Include(String),
    Label(Ident),
    Inst(Mnemonic, Operand),
}
