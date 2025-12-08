use std::fs::read_to_string;

use crate::{Expr, Ident, Mnemonic, Operand, ParseError, Stmt, SymTab, parse, resolve_symbol};

use pioc_core::Inst;

use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum AssembleError {
    #[error("invalid op code")]
    InvalidOpCode,
    #[error("invalid operand")]
    InvalidOperand,
    #[error("origin statement not aligned to 2 bytes")]
    OriginNotAligned,
    #[error("cannot resolve symbol {0:?}")]
    SymbolResolveError(String),
    #[error("parse error")]
    ParseError(#[from] ParseError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

/// Assemble statements parsed from assembly program.
pub fn assemble(prog: &[Stmt]) -> Result<Vec<Inst>, AssembleError> {
    assemble_with_symbols(&SymTab::default(), prog)
}

/// Assemble statements parsed from assembly program with custom symbols instead of default builtins.
pub fn assemble_with_symbols(sym: &SymTab, prog: &[Stmt]) -> Result<Vec<Inst>, AssembleError> {
    let prog = expand_include(prog)?;
    let sym = resolve_symbol(sym.clone(), &prog)?;
    emit_inst(prog, sym)
}

pub(crate) type AssembleResult<T> = Result<T, AssembleError>;

fn expand_include(prog: &[Stmt]) -> AssembleResult<Vec<Stmt>> {
    let mut expanded = Vec::with_capacity(prog.len());
    for stmt in prog {
        match stmt {
            Stmt::Include(path) => {
                let statments = parse(read_to_string(path)?)?;
                expanded.extend(expand_include(&statments)?);
            }
            _ => expanded.push(stmt.clone()),
        }
    }
    Ok(expanded)
}

fn emit_inst(prog: Vec<Stmt>, sym: SymTab) -> AssembleResult<Vec<Inst>> {
    use Inst::*;
    use Mnemonic::*;
    let mut insts = Vec::new();
    let mut addr = 0;
    for stmt in prog {
        match stmt {
            Stmt::Origin(expr) => {
                let new_addr = calc_expr(&expr, &sym)?;
                if new_addr % 2 != 0 {
                    return Err(AssembleError::OriginNotAligned);
                }
                if new_addr < addr {
                    warn!("go back by ORG");
                } else {
                    let insts_len = insts.len();
                    let mut padding = vec![Nop; (new_addr as usize / 2).max(insts_len) - insts_len];
                    insts.append(&mut padding);
                }
                addr = new_addr;
            }
            Stmt::Inst(_, mnemonic, operand) => {
                let inst = match mnemonic {
                    NOP => expect_op0(operand, Nop)?,
                    CLRWDT | WDT => expect_op0(operand, ClearWatchDog)?,
                    SLEEP | HALT => expect_op0(operand, Sleep(0.into()))?,
                    SLEEPX => todo!(),
                    WAITB => todo!(),
                    WAITRD => todo!(),
                    WAITWR | WAITSPI => todo!(),
                    RDCODE => expect_op0(operand, ReadCode(0.into()))?,
                    RCODE => todo!(),
                    WRCODE => todo!(),
                    EXEC => todo!(),
                    PUSHAS | PUSH => expect_op0(operand, PushA)?,
                    POPAS | POP => expect_op0(operand, PopA)?,
                    PUSHA2 => expect_op0(operand, PushIndirAddr2)?,
                    POPA2 => expect_op0(operand, PopIndirAddr2)?,
                    RET | RETURN => expect_op0(operand, Return)?,
                    RETZ | RETOK => expect_op0(operand, ReturnOk)?,
                    RETIE | RETI => expect_op0(operand, ReturnInt)?,
                    CLRA => expect_op0(operand, ClearA)?,
                    CLR | CLRF => todo!(),
                    MOVA | MOVAF => todo!(),
                    MOV | MOVF => todo!(),
                    INC | INCF => todo!(),
                    DEC | DECF => todo!(),
                    INCSZ | INCFSZ => todo!(),
                    DECSZ | DECFSZ => todo!(),
                    SWAP | SWAPF => todo!(),
                    AND | ANDF => todo!(),
                    IOR | IORF => todo!(),
                    XOR | XORF => todo!(),
                    ADD | ADDF => todo!(),
                    SUB | SUBF => todo!(),
                    RCL | RCLF | RLF => todo!(),
                    RCR | RCRF | RRF => todo!(),
                    RETL | DB => todo!(),
                    RETLN | RETER => todo!(),
                    MOVIP => todo!(),
                    MOVIA => todo!(),
                    MOVA1F => todo!(),
                    MOVA2F => todo!(),
                    MOVA2P => todo!(),
                    MOVA1P => todo!(),
                    MOVL => todo!(),
                    ANDL => todo!(),
                    IORL => todo!(),
                    XORL => todo!(),
                    ADDL => todo!(),
                    SUBL => todo!(),
                    CMPLN => todo!(),
                    CMPL => todo!(),
                    BC | BCF => todo!(),
                    BS | BSF => todo!(),
                    BTSC | BTFSC => todo!(),
                    BTSS | BTFSS => todo!(),
                    BCTC | BCTCF => todo!(),
                    BP1F => todo!(),
                    BP2F => todo!(),
                    BG1F => todo!(),
                    BG2F => todo!(),
                    JMP | GOTO => todo!(),
                    CALL => todo!(),
                    JNZ => todo!(),
                    JZ => todo!(),
                    JNC => todo!(),
                    JC => todo!(),
                    CMPZ => todo!(),
                    DW => todo!(),
                };
                insts.push(inst);
                addr += 2;
            }
            Stmt::Define(_, _) => {}
            Stmt::Include(_) => unreachable!(),
        }
    }
    Ok(insts)
}

fn calc_expr(expr: &Expr, sym: &SymTab) -> AssembleResult<i32> {
    match expr {
        Expr::Label(Ident(s)) => match sym.get(s) {
            Some(v) => Ok(*v),
            None => Err(AssembleError::SymbolResolveError(s.clone())),
        },
        Expr::Num(v) => Ok(*v),
        Expr::Add(Ident(s), b) => match sym.get(s) {
            Some(a) => Ok(*a + *b),
            None => Err(AssembleError::SymbolResolveError(s.clone())),
        },
    }
}

fn expect_op0(operand: Operand, inst: Inst) -> AssembleResult<Inst> {
    match operand {
        Operand::Op0 => Ok(inst),
        _ => Err(AssembleError::InvalidOperand),
    }
}

fn expect_op1(operand: Operand) -> AssembleResult<Expr> {
    match operand {
        Operand::Op1(v) => Ok(v),
        _ => Err(AssembleError::InvalidOperand),
    }
}

fn expect_op2(operand: Operand) -> AssembleResult<(Expr, Expr)> {
    match operand {
        Operand::Op2(v0, v1) => Ok((v0, v1)),
        _ => Err(AssembleError::InvalidOperand),
    }
}
