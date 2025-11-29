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

fn parse_name(input: &str) -> IResult<&str, String> {
    map(alpha1, str::to_string).parse(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Atom> {
    map(parse_name, Atom::Symbol).parse(input)
}

fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |it: &str| Atom::String(it.to_string())).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Atom> {
    let parser = recognize((opt(tag("-")), digit1));
    map(parser, |it: &str| Atom::Number(it.parse().unwrap())).parse(input)
}

fn parse_float(input: &str) -> IResult<&str, Atom> {
    map(double, |it: f64| Atom::Float(it)).parse(input)
}

fn parse_atom(input: &str) -> IResult<&str, Atom> {
    alt((parse_symbol, parse_string, parse_number, parse_float)).parse(input)
}

#[derive(Debug)]
pub enum Statement {
    Declare { name: String, value: Atom },
    Call { name: String, args: Vec<Atom> },
}

fn parse_declare(input: &str) -> IResult<&str, Statement> {
    let parser = separated_pair(parse_name, ws(tag(":=")), parse_atom);
    map(parser, |(name, value)| Statement::Declare { name, value }).parse(input)
}

fn parse_call(input: &str) -> IResult<&str, Statement> {
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_atom)),
        tag(")"),
    );
    let parser = (parse_name, parse_args);
    map(parser, |(name, args)| Statement::Call { name, args }).parse(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_declare, parse_call)).parse(input)
}

#[derive(Debug)]
pub enum Node {
    Function {
        name: String,
        args: Vec<Atom>,
        body: Vec<Statement>,
    },
}

fn parse_function(input: &str) -> IResult<&str, Node> {
    let parse_args = delimited(
        tag("("),
        separated_list0(tag(","), ws(parse_atom)),
        tag(")"),
    );
    let parse_body = delimited(tag("{"), many0(ws(parse_statement)), tag("}"));
    let parser = preceded(tag("fn"), (ws(parse_name), parse_args, ws(parse_body)));
    map(parser, |(name, args, body)| Node::Function {
        name,
        args,
        body,
    })
    .parse(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    parse_function.parse(input)
}

pub fn parse(input: &str) -> crate::Result<Vec<Node>> {
    match many0(ws(parse_node)).parse(input) {
        Ok((_, ast)) => Ok(ast),
        Err(err) => Err(format!("Failed to parse file: {err}"))?,
    }
}
