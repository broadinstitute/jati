use nom::Finish;
use crate::error::Error;
use crate::parse::{PResult, Span, SParser};
use crate::parse::parsers::call::{CallParser, DefaultCallParser};
use crate::parse::parsers::id::IdParser;
use crate::parse::parsers::white::WhiteSpaceParser;

pub mod error;
pub mod parse;
pub mod trees;
pub mod symbols;
mod util;
pub mod runtime;

pub fn parse_string<T, P>(parser: P, string: &str) -> Result<T, Error>
    where P: SParser<T>
{
    let span = Span::new(string);
    let parsed = parser.parse_span(span).finish()?.1;
    Ok(parsed)
}
