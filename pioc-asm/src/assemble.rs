use std::{collections::BTreeMap, fs::read_to_string};

use crate::{Expr, Ident, Mnemonic, ParseError, Stmt, parse};

use pioc_core::Inst;

use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum AssembleError {
    #[error("invalid op code")]
    InvalidOpCode,
    #[error("cannot resolve symbol {0:?}")]
    SymbolResolveError(String),
    #[error("parse error")]
    ParseError(#[from] ParseError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

/// Assemble statements parsed from assembly program.
pub fn assemble(prog: &[Stmt]) -> Result<Vec<Inst>, AssembleError> {
    let prog = expand_include(prog)?;
    let sym = resolve_symbol(&prog)?;
    emit_inst(prog, sym)
}

type AssembleResult<T> = Result<T, AssembleError>;

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

type SymTab = BTreeMap<String, i32>;

fn resolve_symbol(prog: &[Stmt]) -> AssembleResult<SymTab> {
    use Expr::*;
    use Stmt::*;

    let mut unresolved = BTreeMap::new();
    let mut origin = Expr::Num(0);
    let mut offset = 0;
    for stmt in prog {
        match stmt {
            Define(ident, expr) => {
                unresolved.insert(ident.0.clone(), expr.clone());
            }
            Origin(expr) => {
                origin = expr.clone();
                offset = 0;
            }
            Inst(Some(Ident(label)), _, _) => {
                match &origin {
                    Label(ident) => unresolved.insert(label.clone(), Add(ident.clone(), offset)),
                    Num(v) => unresolved.insert(label.clone(), Num(v + offset)),
                    Add(_, _) => unreachable!(),
                };
                offset += 2;
            }
            Inst(_, _, _) => offset += 2,
            Include(_) => unreachable!(),
        }
    }

    let mut sym = BTreeMap::new();
    loop {
        let mut resolved = Vec::new();
        for (name, expr) in unresolved.iter() {
            match expr {
                Label(Ident(s)) => {
                    if let Some(v) = sym.get(s) {
                        sym.insert(name.clone(), *v);
                        resolved.push(name.clone());
                    }
                }
                Num(v) => {
                    sym.insert(name.clone(), *v);
                    resolved.push(name.clone());
                }
                Add(Ident(s), b) => {
                    if let Some(a) = sym.get(s) {
                        sym.insert(name.clone(), *a + *b);
                        resolved.push(name.clone());
                    }
                }
            }
        }
        if resolved.is_empty() {
            break;
        }
        for name in resolved {
            unresolved.remove(&name);
        }
    }

    if !unresolved.is_empty() {
        return Err(AssembleError::SymbolResolveError(
            unresolved.pop_first().unwrap().0,
        ));
    }

    Ok(sym)
}

fn emit_inst(prog: Vec<Stmt>, sym: SymTab) -> AssembleResult<Vec<Inst>> {
    use Mnemonic::*;
    let mut insts = Vec::new();
    let mut addr = 0;
    for stmt in prog {
        match stmt {
            Stmt::Origin(expr) => {
                let v = calc_expr(&expr, &sym)?;
                if v < addr {
                    warn!("go back by ORG");
                } else {
                    let len = insts.len();
                    let mut padding = vec![Inst::Nop; (v as usize).max(len) - len];
                    insts.append(&mut padding);
                }
                addr = v;
            }
            Stmt::Inst(_, mnemonic, operand) => match mnemonic {
                NOP => todo!(),
                CLRWDT | WDT => todo!(),
                SLEEP | HALT => todo!(),
                SLEEPX => todo!(),
                WAITB => todo!(),
                WAITRO => todo!(),
                WAITWR | WAITSPI => todo!(),
                RDCODE => todo!(),
                RCODE => todo!(),
                WRCODE => todo!(),
                EXEC => todo!(),
                PUSHAS | PUSH => todo!(),
                POPAS | POP => todo!(),
                PUSHA2 => todo!(),
                POPA2 => todo!(),
                RET | RETURN => todo!(),
                RETZ | RETOK => todo!(),
                RETIE | RETI => todo!(),
                CLRA => todo!(),
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
            },
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
