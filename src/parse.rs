use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::tag, take_until},
    character::complete::{alpha1, digit1, multispace0},
    combinator::{map, opt, recognize},
    error::ParseError,
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
};

fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

#[derive(Debug)]
pub enum Atom {
    Symbol(String),
    String(String),
    Number(i64),
    Float(f64),
}

pub fn parse_name(input: &str) -> IResult<&str, String> {
    map(alpha1, str::to_string).parse(input)
}

pub fn parse_symbol(input: &str) -> IResult<&str, Atom> {
    map(parse_name, Atom::Symbol).parse(input)
}

pub fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |it: &str| Atom::String(it.to_string())).parse(input)
}

pub fn parse_number(input: &str) -> IResult<&str, Atom> {
    let parser = recognize((opt(tag("-")), digit1));
    map(parser, |it: &str| Atom::Number(it.parse().unwrap())).parse(input)
}

pub fn parse_float(input: &str) -> IResult<&str, Atom> {
    map(double, |it: f64| Atom::Float(it)).parse(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_symbol, parse_string, parse_number, parse_float)).parse(input)
}

#[derive(Debug)]
pub enum Expr {
    Constant(Atom),
    Declare {
        name: String,
        value: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Function {
        name: String,
        args: Vec<Expr>,
        body: Vec<Expr>,
    },
}

pub fn parse_constant(input: &str) -> IResult<&str, Expr> {
    map(parse_atom, Expr::Constant).parse(input)
}

pub fn parse_declare(input: &str) -> IResult<&str, Expr> {
    let parser = separated_pair(parse_name, ws(tag(":=")), parse_constant);
    map(parser, |(name, value)| Expr::Declare {
        name,
        value: Box::new(value),
    })
    .parse(input)
}

pub fn parse_call(input: &str) -> IResult<&str, Expr> {
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_constant)),
        tag(")"),
    );
    let parser = (parse_name, parse_args);
    map(parser, |(name, args)| Expr::Call { name, args }).parse(input)
}

pub fn parse_function(input: &str) -> IResult<&str, Expr> {
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_constant)),
        tag(")"),
    );
    let parse_body = delimited(tag("{"), many0(ws(parse_expr)), tag("}"));
    let parser = preceded(tag("fn"), (ws(parse_name), parse_args, ws(parse_body)));
    map(parser, |(name, args, body)| Expr::Function {
        name,
        args,
        body,
    })
    .parse(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_function, parse_declare, parse_call, parse_constant)).parse(input)
}

pub fn parse_ast(input: &str) -> crate::Result<Vec<Expr>> {
    match many0(ws(parse_expr)).parse(input) {
        Ok((_, ast)) => Ok(ast),
        Err(err) => Err(format!("Failed to parse file: {err}"))?,
    }
}
