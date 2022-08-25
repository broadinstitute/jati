use nom::bytes::complete::tag;
use crate::parse::{PResult, Span};

mod error;
pub mod syn;
pub mod parse;

pub fn parse_string<'a, T, P>(parser: P, string: &'a str) -> PResult<T>
    where P: Fn(Span<'a>) -> PResult<T>
{
    let span = Span::new(string);
    parser(span)
}

pub fn first_parser<'a>(span: Span<'a>) -> PResult<'a, Span> {
    tag("Hello")(span)
}