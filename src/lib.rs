use nom::bytes::complete::tag;
use nom::Finish;
use crate::error::Error;
use crate::parse::{PResult, Span};

mod error;
pub mod syn;
mod parse;

pub const TWO: u8 = 2;

pub fn facade(string: &str) -> Result<&str, Error> {
    let span = Span::new(string);
    match first_parser(span).finish() {
        Ok((_, output)) => { Ok(output.fragment()) }
        Err(p_error) => { Err(p_error)? }
    }
}

pub(crate) fn first_parser(span: Span) -> PResult<Span> {
    tag("Hello")(span)
}