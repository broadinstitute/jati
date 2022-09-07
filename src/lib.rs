use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map};
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
        map(all_consuming(tag("Hello")), |_| {})(span)
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
