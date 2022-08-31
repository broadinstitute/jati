use nom::bytes::complete::tag;
use crate::parse::{SParser, PResult, Span};

mod error;
pub mod grammar;
pub mod parse;

pub fn parse_string<'a, T, P>(mut parser: P, string: &'a str) -> PResult<T>
    where P: SParser<'a, T>
{
    let span = Span::new(string);
    parser(span)
}

pub fn first_parser<'a>() -> impl SParser<'a, Span<'a>> {
    tag("Hello")
}