#![allow(mismatched_lifetime_syntaxes)]

use crate::{Expr, Ident, Stmt};

use pioc_core::{OpCode, U2};

use std::str::FromStr;

use nom::{
    Finish, IResult, Parser as _,
    branch::alt,
    bytes::complete::{tag, take, take_till},
    character::complete::{
        bin_digit1, digit1, hex_digit1, multispace0, none_of, oct_digit1, one_of, satisfy,
    },
    combinator::{all_consuming, complete, map, map_res, opt, recognize, value},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
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

/// Parse a assembly program
pub fn parse(asm: impl AsRef<str>) -> Result<Vec<Stmt>, ParseError> {
    let asm = asm.as_ref();
    let prog = asm
        .lines()
        .flat_map(|line| parse_line(line).transpose())
        .take_while(|result| !matches!(result, Err(ParseError::End)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(prog)
}

#[test]
fn test_parse() {
    use OpCode::*;
    use Stmt::*;
    assert_eq!(parse("").unwrap(), vec![]);
    assert_eq!(parse("  ").unwrap(), vec![]);
    assert_eq!(parse("  \n  ").unwrap(), vec![]);
    assert_eq!(parse("NOP\nNOP;").unwrap(), vec![Inst(None, Nop); 2]);
    assert_eq!(
        parse("NOP;comment\nNOP;\n;comment\n;\n").unwrap(),
        vec![Inst(None, Nop); 2]
    );
}

/// Parse a single line of assembly. Panics if argument has more than one line.
pub fn parse_line(line: &str) -> Result<Option<Stmt>, ParseError> {
    assert!(line.lines().count() <= 1);
    parse_line_unchecked(line)
}

fn parse_line_unchecked(line: &str) -> Result<Option<Stmt>, ParseError> {
    if all_consuming(complete((
        multispace0::<&str, nom::error::Error<&str>>,
        tag("END"),
        multispace0,
        opt((tag(";"), many0(take(1usize)))),
    )))
    .parse(line)
    .finish()
    .is_ok()
    {
        return Err(ParseError::End);
    }
    let result = all_consuming(complete(delimited(
        multispace0,
        alt((
            map(equ, |(ident, value)| Some(Stmt::Define(ident, value))),
            map(org, |addr| Some(Stmt::Origin(addr))),
            map(include, |s| Some(Stmt::Include(s))),
            // map(inst, |(label, opcode)| Some(Stmt::Inst(label, opcode))),
            value(None, multispace0),
        )),
        (multispace0, opt((tag(";"), many0(take(1usize))))),
    )))
    .parse(line)
    .finish();
    match result {
        Ok((_, b)) => Ok(b),
        Err(e) => Err(ParseError::Failure(e.to_string())),
    }
}

#[test]
fn test_parse_line() {
    use Expr::*;
    use Stmt::*;
    assert_eq!(parse_line("").unwrap(), None);
    assert_eq!(parse_line(";").unwrap(), None);
    assert_eq!(parse_line(" ; comment").unwrap(), None);
    assert_eq!(
        parse(" a EQU 42 ; comment").unwrap(),
        vec![Define(crate::Ident::from("a"), Num(42))]
    );
    assert_eq!(parse(" ORG 42 ; comment").unwrap(), vec![Origin(Num(42))]);
    assert_eq!(
        parse(" INCLUDE CH32X035.ASM ; comment").unwrap(),
        vec![Include("CH32X035.ASM".to_owned())]
    );
}

#[cfg(test)]
use std::fmt::Debug;
#[cfg(test)]
fn assert_all_consuming_eq<F, T>(parser: F, input: &str, expected: T)
where
    F: Fn(&str) -> ParseResult<T>,
    T: PartialEq + Debug,
{
    assert_eq!(
        all_consuming(complete(parser))
            .parse(input)
            .finish()
            .unwrap()
            .1,
        expected
    );
}

fn ident(input: &str) -> ParseResult<Ident> {
    map(
        recognize((
            satisfy(|c| c.is_ascii_alphabetic() || "_$#@".contains(c)),
            many0(satisfy(|c| c.is_ascii_alphanumeric() || "_$#@".contains(c))),
        )),
        |s: &str| Ident::from(s),
    )
    .parse(input)
}

#[test]
fn test_ident() {
    assert_all_consuming_eq(ident, "abc123", Ident::from("abc123"));
    assert_all_consuming_eq(ident, "_$#@", Ident::from("_$#@"));
    assert!(ident("1").is_err());
}

fn binary(input: &str) -> ParseResult<i32> {
    alt((
        map_res(
            preceded(
                alt((tag("0b"), tag("0B"))),
                map(
                    separated_list1(tag("_"), bin_digit1),
                    |digits: Vec<&str>| digits.concat(),
                ),
            ),
            |s| i32::from_str_radix(&s, 2),
        ),
        map_res(
            delimited(
                alt((tag("b'"), tag("B'"))),
                map(
                    separated_list1(tag("_"), bin_digit1),
                    |digits: Vec<&str>| digits.concat(),
                ),
                tag("'"),
            ),
            |s| i32::from_str_radix(&s, 2),
        ),
    ))
    .parse(input)
}

fn octal(input: &str) -> ParseResult<i32> {
    map_res(preceded(tag("0o"), oct_digit1), |s| {
        i32::from_str_radix(s, 8)
    })
    .parse(input)
}

fn decimal(input: &str) -> ParseResult<i32> {
    alt((
        map_res(preceded(alt((tag("0d"), tag("0D"))), digit1), |s| {
            i32::from_str(s)
        }),
        map_res(
            delimited(alt((tag("d'"), tag("D'"))), digit1, tag("'")),
            |s| i32::from_str(s),
        ),
    ))
    .parse(input)
}

fn hexadecimal(input: &str) -> ParseResult<i32> {
    alt((
        map_res(preceded(alt((tag("0x"), tag("0X"))), hex_digit1), |s| {
            i32::from_str_radix(s, 16)
        }),
        map_res(
            delimited(alt((tag("h'"), tag("H'"))), hex_digit1, tag("'")),
            |s| i32::from_str_radix(s, 16),
        ),
    ))
    .parse(input)
}

fn character(input: &str) -> ParseResult<i32> {
    delimited(
        tag("'"),
        alt((
            preceded(
                tag(r"\"),
                map(one_of("'\"nrt\\0"), |c| match c {
                    '\'' => '\'' as i32,
                    '\"' => '\"' as i32,
                    'n' => '\n' as i32,
                    'r' => '\r' as i32,
                    't' => '\t' as i32,
                    '\\' => '\\' as i32,
                    '0' => '\0' as i32,
                    _ => unreachable!(),
                }),
            ),
            map(none_of("'\\"), |c| c as i32),
        )),
        tag("'"),
    )
    .parse(input)
}

fn expr(input: &str) -> ParseResult<Expr> {
    // TODO: parse arithmetic expressions
    alt((
        map(
            alt((
                binary,
                decimal,
                octal,
                hexadecimal,
                character,
                nom::character::complete::i32,
            )),
            |value| Expr::Num(value),
        ),
        map(ident, |ident| Expr::Ident(ident)),
    ))
    .parse(input)
}

#[test]
fn test_expr() {
    use Expr::*;
    assert_all_consuming_eq(expr, "abc123", Ident(crate::Ident::from("abc123")));
    assert_all_consuming_eq(expr, "42", Num(42));
    assert_all_consuming_eq(expr, "+42", Num(42));
    assert_all_consuming_eq(expr, "-42", Num(-42));
    assert_all_consuming_eq(expr, "0d42", Num(42));
    assert_all_consuming_eq(expr, "0D42", Num(42));
    assert_all_consuming_eq(expr, "d'42'", Num(42));
    assert_all_consuming_eq(expr, "D'42'", Num(42));
    assert_all_consuming_eq(expr, "0b101010", Num(42));
    assert_all_consuming_eq(expr, "0B101010", Num(42));
    assert_all_consuming_eq(expr, "0b0101010", Num(42));
    assert_all_consuming_eq(expr, "0b0010_1010", Num(42));
    assert_all_consuming_eq(expr, "b'101010'", Num(42));
    assert_all_consuming_eq(expr, "B'101010'", Num(42));
    assert_all_consuming_eq(expr, "b'0101010'", Num(42));
    assert_all_consuming_eq(expr, "b'0010_1010'", Num(42));
    assert_all_consuming_eq(expr, "0o42", Num(0o42));
    assert_all_consuming_eq(expr, "0x42", Num(0x42));
    assert_all_consuming_eq(expr, "0X42", Num(0x42));
    assert_all_consuming_eq(expr, "h'42'", Num(0x42));
    assert_all_consuming_eq(expr, "H'42'", Num(0x42));
    assert_all_consuming_eq(expr, "'a'", Num('a' as i32));
    assert_all_consuming_eq(expr, r"'\\'", Num('\\' as i32));
    assert!(ident(r"'\'").is_err());
    assert_all_consuming_eq(expr, r"'\''", Num('\'' as i32));
    assert_all_consuming_eq(expr, r"'\0'", Num(0x00));
    assert!(ident("0123").is_err());
    assert!(ident("0c123").is_err());
    assert!(ident("1a").is_err());

    // TODO: parse arithmetic expressions
    assert!(ident("1 + 1").is_err());
}

fn separator(input: &str) -> ParseResult<()> {
    value((), many1(one_of(" \t:,"))).parse(input)
}

fn equ(input: &str) -> ParseResult<(Ident, Expr)> {
    separated_pair(label, (tag("EQU"), separator), expr).parse(input)
}

#[test]
fn test_equ() {
    assert_all_consuming_eq(equ, "abc EQU 42", (Ident::from("abc"), Expr::Num(42)));
    assert_all_consuming_eq(equ, "abc EQU 0x42", (Ident::from("abc"), Expr::Num(0x42)));
    assert_all_consuming_eq(
        equ,
        "abc EQU L42",
        (Ident::from("abc"), Expr::Ident(Ident::from("L42"))),
    );
}

fn org(input: &str) -> ParseResult<Expr> {
    preceded((tag("ORG"), separator), expr).parse(input)
}

#[test]
fn test_org() {
    use Expr::*;
    assert_all_consuming_eq(org, "ORG 42", Num(42));
    assert_all_consuming_eq(org, "ORG 0x42", Num(0x42));
    assert_all_consuming_eq(org, "ORG L42", Ident(crate::Ident::from("L42")));
}

fn include(input: &str) -> ParseResult<String> {
    preceded(
        (tag("INCLUDE"), separator),
        map(take_till(|c| " \t;".contains(c)), |s: &str| s.to_owned()),
    )
    .parse(input)
}

#[test]
fn test_include() {
    assert_all_consuming_eq(include, "INCLUDE CH32X035.ASM", "CH32X035.ASM".to_owned());
    assert_all_consuming_eq(
        include,
        r"INCLUDE C:\RISC8B\CH533INC.ASM",
        r"C:\RISC8B\CH533INC.ASM".to_owned(),
    );
}

fn label(input: &str) -> ParseResult<Ident> {
    terminated(ident, separator).parse(input)
}

fn operand(input: &str) -> ParseResult<Expr> {
    todo!()
}

fn opcode(input: &str) -> ParseResult<OpCode> {
    todo!()
}

fn inst(input: &str) -> ParseResult<(Option<Ident>, OpCode)> {
    (opt(label), opcode).parse(input)
}

fn u2(s: &str) -> ParseResult<U2> {
    todo!()
    // let value: u8 = s.parse().map_err(|_| "Invalid U2 value".to_string())?;
    // U2::new(value).ok_or_else(|| "U2 value must be 0, 1, 2, or 3".to_string())
}
