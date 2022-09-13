use crate::parse::{PResult, Span, SParser};
use crate::parse::parsers::call::{CallParser, DefaultCallParser};
use crate::parse::parsers::id::IdParser;
use crate::parse::parsers::white::WhiteSpaceParser;
use crate::trees::raw::tree::Tree;

mod error;
pub mod parse;
pub mod trees;
mod engine;

pub fn parse_string<T, P>(parser: P, string: &str) -> PResult<T>
    where P: SParser<T>
{
    let span = Span::new(string);
    parser.parse_span(span)
}
