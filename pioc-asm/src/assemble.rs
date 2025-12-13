use std::fs::read_to_string;

use crate::{Expr, Ident, Mnemonic, Operand, ParseError, Stmt, SymTab, parse, resolve_symbol};

use pioc_core::{BitIn, BitInC, BitOut, Dest, Inst, Label, Reg, U2, U3, U7, U9, U10, U12, WaitBit};

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
                let new_addr = eval(&expr, &sym)?;
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
            Stmt::Inst(_, mnemonic, op) => {
                let inst = match mnemonic {
                    NOP => op0(op, Nop)?,
                    CLRWDT | WDT => op0(op, ClearWatchDog)?,
                    SLEEP | HALT => op0(op, Sleep(0.into()))?,
                    SLEEPX => Sleep(op1(op, &sym)?),
                    WAITB => WaitB(WaitBit(op1(op, &sym)?)),
                    WAITRD => WaitB(WaitBit(0.into())),
                    WAITWR | WAITSPI => WaitB(WaitBit(4.into())),
                    RDCODE => op0(op, ReadCode(0.into()))?,
                    RCODE => ReadCode(op1(op, &sym)?),
                    WRCODE => {
                        warn!("WRCODE is interpreted as BCTC BI_C_XOR_IN0");
                        BitToC(BitInC(0.into()))
                    }
                    EXEC => {
                        warn!("EXEC is interpreted as BCTC");
                        BitToC(BitInC(op1(op, &sym)?))
                    }
                    PUSHAS | PUSH => op0(op, PushA)?,
                    POPAS | POP => op0(op, PopA)?,
                    PUSHA2 => op0(op, PushIndirAddr2)?,
                    POPA2 => op0(op, PopIndirAddr2)?,
                    RET | RETURN => op0(op, Return)?,
                    RETZ | RETOK => op0(op, ReturnOk)?,
                    RETIE | RETI => op0(op, ReturnInt)?,
                    CLRA => op0(op, ClearA)?,
                    CLR | CLRF => Clear(Reg(op1(op, &sym)?)),
                    MOVA | MOVAF => MoveA(Reg(op1(op, &sym)?)),
                    MOV | MOVF => Move(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    INC | INCF => Inc(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    DEC | DECF => Dec(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    INCSZ | INCFSZ => IncAndSkipIfZero(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    DECSZ | DECFSZ => DecAndSkipIfZero(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    SWAP | SWAPF => SwapHalfBytes(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    AND | ANDF => And(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    IOR | IORF => Or(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    XOR | XORF => Xor(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    ADD | ADDF => Add(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    SUB | SUBF => Sub(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    RCL | RCLF | RLF => {
                        RotateLeftWithCarry(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?)
                    }
                    RCR | RCRF | RRF => {
                        RotateRightWithCarry(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?)
                    }
                    RETL | DB => ReturnImm(op1(op, &sym)?),
                    RETLN | RETER => ReturnErrImm(op1(op, &sym)?),
                    MOVIP => MoveImmToIndirAddr1(op1(op, &sym)?),
                    MOVIA => MoveImmToIndirAddr2(op1(op, &sym)?),
                    MOVA1F => MoveImmToPortDir(op1(op, &sym)?),
                    MOVA2F => MoveImmToPortIo(op1(op, &sym)?),
                    MOVA2P => MoveImmToP2(op1(op, &sym)?),
                    MOVA1P => MoveImmToP1(op1(op, &sym)?),
                    MOVL => MoveImm(op1(op, &sym)?),
                    ANDL => AndImm(op1(op, &sym)?),
                    IORL => OrImm(op1(op, &sym)?),
                    XORL => XorImm(op1(op, &sym)?),
                    ADDL => AddImm(op1(op, &sym)?),
                    SUBL => SubImm(op1(op, &sym)?),
                    CMPLN => CompareImmNegate(op1(op, &sym)?),
                    CMPL => CompareImm(op1(op, &sym)?),
                    BC | BCF => BitClear(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BS | BSF => BitSet(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BTSC | BTFSC => BitTestSkipIfClear(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BTSS | BTFSS => BitTestSkipIfSet(Reg(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BCTC | BCTCF => BitToC(BitInC(op1(op, &sym)?)),
                    BP1F => BitOut1(BitOut(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BP2F => BitOut2(BitOut(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BG1F => BitIn1(BitIn(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    BG2F => BitIn2(BitIn(op2_0(&op, &sym)?), op2_1(&op, &sym)?),
                    JMP | GOTO => Jump(Label(op1(op, &sym)?)),
                    CALL => Call(Label(op1(op, &sym)?)),
                    JNZ => JumpIfNotZero(Label(op1(op, &sym)?)),
                    JZ => JumpIfZero(Label(op1(op, &sym)?)),
                    JNC => JumpIfNotCarry(Label(op1(op, &sym)?)),
                    JC => JumpIfCarry(Label(op1(op, &sym)?)),
                    CMPZ => JumpIfEqual(op2_0(&op, &sym)?, Label(op2_1(&op, &sym)?)),
                    DW => Unknown(op1(op, &sym)?),
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

fn eval(expr: &Expr, sym: &SymTab) -> AssembleResult<i32> {
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

fn op0(operand: Operand, inst: Inst) -> AssembleResult<Inst> {
    let Operand::Op0 = operand else {
        return Err(AssembleError::InvalidOperand);
    };
    Ok(inst)
}

fn op1<T: TryFromExpr>(operand: Operand, sym: &SymTab) -> AssembleResult<T> {
    let Operand::Op1(v) = operand else {
        return Err(AssembleError::InvalidOperand);
    };
    T::try_from_expr(&v, sym)
}

fn op2_0<T: TryFromExpr>(operand: &Operand, sym: &SymTab) -> AssembleResult<T> {
    let (Operand::Op1(v) | Operand::Op2(v, _)) = operand else {
        return Err(AssembleError::InvalidOperand);
    };
    T::try_from_expr(v, sym)
}

fn op2_1<T: TryFromExpr>(operand: &Operand, sym: &SymTab) -> AssembleResult<T> {
    match operand {
        Operand::Op1(_) => T::try_default().ok_or(AssembleError::InvalidOperand),
        Operand::Op2(_, v) => T::try_from_expr(v, sym),
        _ => Err(AssembleError::InvalidOperand),
    }
}

trait TryFromExpr
where
    Self: Sized,
{
    fn try_from_expr(v: &Expr, sym: &SymTab) -> AssembleResult<Self>;
    fn try_default() -> Option<Self> {
        None
    }
}

impl TryFromExpr for Expr {
    fn try_from_expr(v: &Expr, _sym: &SymTab) -> AssembleResult<Self> {
        Ok(v.clone())
    }
}

trait TryFromExprMarker {}

impl TryFromExprMarker for U2 {}
impl TryFromExprMarker for U3 {}
impl TryFromExprMarker for U7 {}
impl TryFromExprMarker for u8 {}
impl TryFromExprMarker for U9 {}
impl TryFromExprMarker for U10 {}
impl TryFromExprMarker for U12 {}
impl TryFromExprMarker for u16 {}

impl<T> TryFromExpr for T
where
    T: TryFrom<i32> + TryFromExprMarker,
{
    fn try_from_expr(v: &Expr, sym: &SymTab) -> AssembleResult<Self> {
        let v = eval(v, sym)?;
        let v = v.try_into().or(Err(AssembleError::InvalidOperand))?;
        Ok(v)
    }
}

impl TryFromExpr for Dest {
    fn try_from_expr(v: &Expr, sym: &SymTab) -> AssembleResult<Self> {
        if let Expr::Label(Ident(ident)) = v {
            match ident.to_lowercase().as_str() {
                "a" => return Ok(Self::A),
                "f" => return Ok(Self::F),
                _ => {}
            }
        }
        match eval(v, sym) {
            Ok(0) => Ok(Self::A),
            Ok(1) => Ok(Self::F),
            _ => Err(AssembleError::InvalidOperand),
        }
    }

    fn try_default() -> Option<Self> {
        Some(Self::F)
    }
}
