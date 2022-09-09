use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, value};
use nom::error::context;
use crate::parse::{SParser, PResult, Span};

mod error;
pub mod grammar;
pub mod parse;

pub fn parse_string<T, P>(parser: P, string: &str) -> PResult<T>
    where P: SParser<T>
{
    let span = Span::new(string);
    parser.parse_span(span)
}

pub struct FirstParser {}

impl SParser<()> for FirstParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, ()> {
        context("Hello", value((), all_consuming(tag("Hello"))))(span)
    }
}

impl FirstParser {
    pub fn new() -> FirstParser { FirstParser {} }
}

impl Default for FirstParser {
    fn default() -> Self {
        Self::new()
    }
}
