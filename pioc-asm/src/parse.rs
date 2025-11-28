use crate::Stmt;

use pioc_core::{OpCode, U2};

use nom::{
    Finish, IResult, Parser as _,
    branch::alt,
    bytes::{complete::take, tag},
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, map, opt, value},
    multi::{many, many0},
    sequence::{delimited, separated_pair, terminated},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("END pseudoinstruction reached")]
    End,
    #[error("invalid op code")]
    InvalidOpCode,
    #[error("failed to parse: {0}")]
    Failure(String),
}

type ParseResult<'a, T> = IResult<&'a str, T>;

pub fn parse(asm: impl AsRef<str>) -> Result<Vec<Stmt>, ParseError> {
    let asm = asm.as_ref();
    let prog = asm
        .lines()
        .flat_map(|line| parse_line(line).transpose())
        .take_while(|result| !matches!(result, Err(ParseError::End)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(prog)
}

pub fn parse_line(line: &str) -> Result<Option<Stmt>, ParseError> {
    if all_consuming((
        multispace0::<&str, nom::error::Error<&str>>,
        tag("END"),
        multispace0,
        opt((tag(";"), many0(take(1usize)))),
    ))
    .parse(line)
    .finish()
    .is_ok()
    {
        return Err(ParseError::End);
    }
    let result = all_consuming(terminated(
        alt((
            map(equ, |(ident, value)| Some(Stmt::Define(ident, value))),
            map(org, |addr| Some(Stmt::Origin(addr))),
            map(include, |s| Some(Stmt::Include(s))),
            map(inst, |(label, opcode)| Some(Stmt::Inst(label, opcode))),
            value(None, multispace0),
        )),
        (multispace0, opt((tag(";"), many0(take(1usize))))),
    ))
    .parse(line)
    .finish();
    match result {
        Ok((_, b)) => Ok(b),
        Err(e) => Err(ParseError::Failure(e.to_string())),
    }
}

fn equ(input: &str) -> ParseResult<(String, String)> {
    todo!()
}

fn org(input: &str) -> ParseResult<u16> {
    todo!()
}

fn include(input: &str) -> ParseResult<String> {
    todo!()
}

fn end(input: &str) -> ParseResult<()> {
    todo!()
}

fn inst(input: &str) -> ParseResult<(Option<String>, OpCode)> {
    todo!()
}

fn comment(input: &str) -> ParseResult<()> {
    todo!()
}

fn label(input: &str) -> ParseResult<String> {
    todo!()
}

fn opcode(input: &str) -> ParseResult<OpCode> {
    todo!()
}

fn u2(s: &str) -> ParseResult<U2> {
    todo!()
    // let value: u8 = s.parse().map_err(|_| "Invalid U2 value".to_string())?;
    // U2::new(value).ok_or_else(|| "U2 value must be 0, 1, 2, or 3".to_string())
}
