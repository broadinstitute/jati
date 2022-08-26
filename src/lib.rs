use nom::bytes::complete::tag;
use crate::parse::{Parser, PResult, Span};

mod error;
pub mod syn;
pub mod parse;

pub fn parse_string<'a, T, P>(parser: P, string: &'a str) -> PResult<T>
    where P: Parser<'a, T>
{
    let span = Span::new(string);
    parser(span)
}

pub fn first_parser<'a>() -> impl Parser<'a, Span<'a>> {
    tag("Hello")
}