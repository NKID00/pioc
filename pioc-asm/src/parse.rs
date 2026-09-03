#![allow(mismatched_lifetime_syntaxes)]

use crate::{Expr, Ident, Mnemonic, Operand, Stmt};

use std::str::FromStr;

use nom::{
    Finish, IResult, Parser as _,
    branch::alt,
    bytes::complete::{tag, take, take_till},
    character::complete::{
        bin_digit1, digit1, hex_digit1, multispace0, none_of, oct_digit1, one_of, satisfy,
    },
    combinator::{all_consuming, complete, fail, map, map_res, opt, recognize, success, value},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
};
use nom_language::precedence::{Assoc, Operation, binary_op, precedence, unary_op};
use thiserror::Error;
use tracing::{trace, warn};

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

/// Parse an assembly program.
pub fn parse(asm: impl AsRef<str>) -> Result<Vec<Stmt>, ParseError> {
    let mut prog = vec![];
    let mut lines = asm.as_ref().lines();
    for line in &mut lines {
        trace!("parsing {line:?}");
        match parse_line(line) {
            Ok(mut stmts) => prog.append(&mut stmts),
            Err(ParseError::End) => break,
            Err(e) => return Err(e),
        }
    }
    if lines.any(|line| line.chars().any(|c| !(c.is_whitespace() || c == ';'))) {
        warn!("END reached, ignoring remaining lines");
    }
    Ok(prog)
}

#[test]
fn test_parse() {
    use Mnemonic::*;
    use Operand::*;
    use Stmt::*;
    assert_eq!(parse("").unwrap(), vec![]);
    assert_eq!(parse("  ").unwrap(), vec![]);
    assert_eq!(parse("  \n  ").unwrap(), vec![]);
    assert_eq!(parse(" NOP\n NOP;").unwrap(), vec![Inst(NOP, Op0); 2]);
    assert_eq!(
        parse(" NOP;comment\n NOP;\n;comment\n;\n").unwrap(),
        vec![Inst(NOP, Op0); 2]
    );
}

/// Parse a single line of assembly. Panics if argument has more than one line.
pub fn parse_line(line: impl AsRef<str>) -> Result<Vec<Stmt>, ParseError> {
    let line = line.as_ref();
    assert!(line.lines().count() <= 1);
    parse_line_unchecked(line)
}

fn parse_line_unchecked(line: &str) -> Result<Vec<Stmt>, ParseError> {
    use Stmt::*;
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
    let result = all_consuming(complete(terminated(
        alt((
            map(equ, |(ident, value)| vec![Define(ident, value)]),
            map(org, |addr| vec![Origin(addr)]),
            map(include, |s| vec![Include(s)]),
            map(inst, |(label, mnemonic, operand)| match label {
                Some(label) => vec![Label(label), Inst(mnemonic, operand)],
                None => vec![Inst(mnemonic, operand)],
            }),
            map(ident, |s| vec![Label(s)]),
            success(vec![]),
        )),
        (opt(separator), opt((tag(";"), many0(take(1usize))))),
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
    use Mnemonic::*;
    use Operand::*;
    use Stmt::*;
    assert_eq!(parse_line("").unwrap(), vec![]);
    assert_eq!(parse_line(";").unwrap(), vec![]);
    assert_eq!(parse_line(" ; comment").unwrap(), vec![]);
    assert_eq!(
        parse("a EQU 42 ; comment").unwrap(),
        vec![Define("a".into(), Const(42))]
    );
    assert_eq!(parse("ORG 42 ; comment").unwrap(), vec![Origin(Const(42))]);
    assert_eq!(
        parse("INCLUDE CH32X035.ASM ; comment").unwrap(),
        vec![Include("CH32X035.ASM".to_owned())]
    );
    assert_eq!(parse(" NOP").unwrap(), vec![Inst(NOP, Op0)]);
    assert_eq!(parse(" NOP ; comment").unwrap(), vec![Inst(NOP, Op0)]);
    assert_eq!(
        parse(" ADDL 0x42").unwrap(),
        vec![Inst(ADDL, Op1(Const(0x42)))]
    );
    assert_eq!(parse("L").unwrap(), vec![Stmt::Label("L".into())]);
    assert_eq!(parse("L:").unwrap(), vec![Stmt::Label("L".into())]);
    assert_eq!(parse("L ").unwrap(), vec![Stmt::Label("L".into())]);
    assert_eq!(parse("L: ").unwrap(), vec![Stmt::Label("L".into())]);
    assert_eq!(
        parse("L\t\tNOP").unwrap(),
        vec![Stmt::Label("L".into()), Stmt::Inst(NOP, Op0)]
    );
    assert_eq!(
        parse("L:\t\tNOP").unwrap(),
        vec![Stmt::Label("L".into()), Stmt::Inst(NOP, Op0)]
    );
}

#[cfg(test)]
use std::fmt::Debug;
#[cfg(test)]
fn assert_parse<F, T>(parser: F, input: &str, expected: T)
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

#[cfg(test)]
fn assert_parse_err<F, T>(parser: F, input: &str)
where
    F: Fn(&str) -> ParseResult<T>,
{
    assert!(
        all_consuming(complete(parser))
            .parse(input)
            .finish()
            .is_err()
    );
}

fn ident(input: &str) -> ParseResult<Ident> {
    map(
        recognize((
            satisfy(|c| c.is_ascii_alphabetic() || "_$#@".contains(c)),
            many0(satisfy(|c| c.is_ascii_alphanumeric() || "_$#@".contains(c))),
        )),
        |s: &str| s.into(),
    )
    .parse(input)
}

#[test]
fn test_ident() {
    assert_parse(ident, "abc123", "abc123".into());
    assert_parse(ident, "_$#@", "_$#@".into());
    assert_parse_err(ident, r"'\'");
    assert_parse_err(ident, "0123");
    assert_parse_err(ident, "0c123");
    assert_parse_err(ident, "1");
    assert_parse_err(ident, "1a");
    assert_parse_err(ident, "1 + 1");
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
            i32::from_str,
        ),
        map_res(
            alt((
                tag("0"),
                recognize((one_of("123456789"), many0(one_of("0123456789")))),
            )),
            i32::from_str,
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

fn constant(input: &str) -> ParseResult<i32> {
    alt((binary, octal, hexadecimal, character, decimal)).parse(input)
}

#[test]
fn test_constant() {
    assert_parse(constant, "0", 0);
    assert_parse(constant, "42", 42);
    assert_parse(constant, "0d42", 42);
    assert_parse(constant, "0D42", 42);
    assert_parse(constant, "d'42'", 42);
    assert_parse(constant, "D'42'", 42);
    assert_parse(constant, "0b101010", 42);
    assert_parse(constant, "0B101010", 42);
    assert_parse(constant, "0b0101010", 42);
    assert_parse(constant, "0b0010_1010", 42);
    assert_parse(constant, "b'101010'", 42);
    assert_parse(constant, "B'101010'", 42);
    assert_parse(constant, "b'0101010'", 42);
    assert_parse(constant, "b'0010_1010'", 42);
    assert_parse(constant, "0o42", 0o42);
    assert_parse(constant, "0x42", 0x42);
    assert_parse(constant, "0X42", 0x42);
    assert_parse(constant, "h'42'", 0x42);
    assert_parse(constant, "H'42'", 0x42);
    assert_parse(constant, "'a'", 'a' as i32);
    assert_parse(constant, r"'\\'", '\\' as i32);
    assert_parse(constant, r"'\''", '\'' as i32);
    assert_parse(constant, r"'\0'", 0x00);
    assert_parse_err(constant, "0123");
}

fn const_or_ident(input: &str) -> ParseResult<Expr> {
    alt((map(constant, Expr::Const), map(ident, Expr::Label))).parse(input)
}

fn expr(input: &str) -> ParseResult<Expr> {
    precedence(
        terminated(
            alt((
                unary_op(2, tag("+")),
                unary_op(2, tag("-")),
                unary_op(2, tag("~")),
            )),
            multispace0,
        ),
        fail(),
        delimited(
            multispace0,
            alt((
                binary_op(4, Assoc::Left, tag("+")),
                binary_op(4, Assoc::Left, tag("-")),
                binary_op(3, Assoc::Left, tag("*")),
                binary_op(3, Assoc::Left, tag("/")),
                binary_op(3, Assoc::Left, tag("%")),
                binary_op(8, Assoc::Left, tag("&")),
                binary_op(10, Assoc::Left, tag("|")),
                binary_op(9, Assoc::Left, tag("^")),
                binary_op(5, Assoc::Left, tag("<<")),
                binary_op(5, Assoc::Left, tag(">>")),
            )),
            multispace0,
        ),
        alt((
            const_or_ident,
            delimited((tag("("), multispace0), expr, (multispace0, tag(")"))),
        )),
        |op: Operation<&str, &str, &str, Expr>| {
            use nom_language::precedence::Operation::*;
            match op {
                Binary(a, "+", b) => Ok(Expr::Add(Box::new(a), Box::new(b))),
                Binary(a, "-", b) => Ok(Expr::Sub(Box::new(a), Box::new(b))),
                Prefix("+", a) => Ok(a),
                Prefix("-", a) => Ok(Expr::Neg(Box::new(a))),
                Binary(a, "*", b) => Ok(Expr::Mul(Box::new(a), Box::new(b))),
                Binary(a, "/", b) => Ok(Expr::Div(Box::new(a), Box::new(b))),
                Binary(a, "%", b) => Ok(Expr::Rem(Box::new(a), Box::new(b))),
                Binary(a, "&", b) => Ok(Expr::And(Box::new(a), Box::new(b))),
                Binary(a, "|", b) => Ok(Expr::Or(Box::new(a), Box::new(b))),
                Binary(a, "^", b) => Ok(Expr::Xor(Box::new(a), Box::new(b))),
                Prefix("~", a) => Ok(Expr::Not(Box::new(a))),
                Binary(a, "<<", b) => Ok(Expr::Lsh(Box::new(a), Box::new(b))),
                Binary(a, ">>", b) => Ok(Expr::Rsh(Box::new(a), Box::new(b))),
                _ => Err(()),
            }
        },
    )
    .parse(input)
}

#[test]
fn test_expr() {
    use Expr::*;
    assert_parse(expr, "abc123", Label("abc123".into()));
    assert_parse_err(expr, r"'\'");
    assert_parse_err(expr, "0123");
    assert_parse_err(expr, "0c123");
    assert_parse_err(expr, "1a");
    assert_parse(expr, "1", Const(1));
    assert_parse(expr, "+1", Const(1));
    assert_parse(expr, "+ 1", Const(1));
    assert_parse(expr, "-1", Neg(Box::new(Const(1))));
    assert_parse(expr, "- 1", Neg(Box::new(Const(1))));
    assert_parse(expr, "1+1", Add(Box::new(Const(1)), Box::new(Const(1))));
    assert_parse(expr, "1 + 1", Add(Box::new(Const(1)), Box::new(Const(1))));
    assert_parse(
        expr,
        "1 * 2 + 3",
        Add(
            Box::new(Mul(Box::new(Const(1)), Box::new(Const(2)))),
            Box::new(Const(3)),
        ),
    );
    assert_parse(
        expr,
        "1 * (2 + 3)",
        Mul(
            Box::new(Const(1)),
            Box::new(Add(Box::new(Const(2)), Box::new(Const(3)))),
        ),
    );
}

fn separator(input: &str) -> ParseResult<()> {
    value((), many1(one_of(" \t:,"))).parse(input)
}

fn equ(input: &str) -> ParseResult<(Ident, Expr)> {
    separated_pair(ident, (separator, tag("EQU"), separator), expr).parse(input)
}

#[test]
fn test_equ() {
    use Expr::*;
    assert_parse(equ, "abc EQU 42", ("abc".into(), Const(42)));
    assert_parse(equ, "abc EQU 0x42", ("abc".into(), Const(0x42)));
    assert_parse(equ, "abc EQU L42", ("abc".into(), Label("L42".into())));
    assert_parse_err(equ, " abc EQU 42");
}

fn org(input: &str) -> ParseResult<Expr> {
    preceded((multispace0, tag("ORG"), separator), expr).parse(input)
}

#[test]
fn test_org() {
    use Expr::*;
    assert_parse(org, "ORG 42", Const(42));
    assert_parse(org, "ORG 0x42", Const(0x42));
    assert_parse(org, "ORG L42", Label("L42".into()));
    assert_parse(org, " ORG L42", Label("L42".into()));
}

fn include(input: &str) -> ParseResult<String> {
    preceded(
        (multispace0, tag("INCLUDE"), separator),
        map(take_till(|c| " \t;".contains(c)), |s: &str| s.to_owned()),
    )
    .parse(input)
}

#[test]
fn test_include() {
    assert_parse(include, "INCLUDE CH32X035.ASM", "CH32X035.ASM".to_owned());
    assert_parse(include, " INCLUDE CH32X035.ASM", "CH32X035.ASM".to_owned());
    assert_parse(
        include,
        r"INCLUDE C:\RISC8B\CH533INC.ASM",
        r"C:\RISC8B\CH533INC.ASM".to_owned(),
    );
}

fn mnemonic(input: &str) -> ParseResult<Mnemonic> {
    map_res(ident, |Ident(s)| Mnemonic::from_str(&s)).parse(input)
}

#[test]
fn test_mnemonic() {
    use Mnemonic::*;
    assert_parse(mnemonic, "NOP", NOP);
    assert_parse(mnemonic, "MOVIA", MOVIA);
    assert_parse(mnemonic, "BC", BC);
    assert_parse(mnemonic, "MOVA1F", MOVA1F);
    assert_parse_err(mnemonic, "HCF");
}

fn operand(input: &str) -> ParseResult<Operand> {
    use Operand::*;
    alt((
        map(
            (preceded(separator, expr), preceded(separator, expr)),
            |(value0, value1)| Op2(value0, value1),
        ),
        map(preceded(separator, expr), Op1),
        success(Op0),
    ))
    .parse(input)
}

#[test]
fn test_operand() {
    use Expr::*;
    use Operand::*;
    assert_parse(operand, "", Op0);
    assert_parse(operand, " 0x42", Op1(Const(0x42)));
    assert_parse(operand, " 1, 2", Op2(Const(1), Const(2)));
}

fn inst(input: &str) -> ParseResult<(Option<Ident>, Mnemonic, Operand)> {
    (opt(ident), preceded(separator, mnemonic), operand).parse(input)
}

#[test]
fn test_inst() {
    use Expr::*;
    use Mnemonic::*;
    use Operand::*;
    assert_parse(inst, " NOP", (None, NOP, Op0));
    assert_parse(inst, "NOP NOP", (Some("NOP".into()), NOP, Op0));
    assert_parse(inst, " ADDL 0x42", (None, ADDL, Op1(Const(0x42))));
    assert_parse(
        inst,
        "L1 ADDL 0x42",
        (Some("L1".into()), ADDL, Op1(Const(0x42))),
    );
    assert_parse(
        inst,
        "L1:ADDL 0x42",
        (Some("L1".into()), ADDL, Op1(Const(0x42))),
    );
    assert_parse(inst, "L1: NOP", (Some("L1".into()), NOP, Op0));
    assert_parse(inst, " BS 0x9B, 3", (None, BS, Op2(Const(0x9B), Const(3))));
}
