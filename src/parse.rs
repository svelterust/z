use nom::{
    IResult, Parser,
    bytes::{complete::tag, take_until},
    character::complete::{alpha1, digit1},
    combinator::{map, opt, recognize},
    number::double,
    sequence::{delimited, pair},
};

enum Atom {
    Symbol(String),
    String(String),
    Number(i64),
    Float(f64),
}

fn parse_name(input: &str) -> IResult<&str, Atom> {
    map(alpha1, |it: &str| Atom::Symbol(it.to_string())).parse(input)
}

fn parse_string(input: &str) -> IResult<&str, Atom> {
    let parser = delimited(tag("\""), take_until("\""), tag("\""));
    map(parser, |it: &str| Atom::String(it.to_string())).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, Atom> {
    let parser = recognize(pair(opt(tag("-")), digit1));
    map(parser, |it: &str| Atom::Number(it.parse().unwrap())).parse(input)
}

fn parse_float(input: &str) -> IResult<&str, Atom> {
    map(double, |it: f64| Atom::Float(it)).parse(input)
}
