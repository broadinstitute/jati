use nom::bytes::complete::tag;
use nom::combinator::recognize;
use crate::{Span, SParser};
use nom::Parser;
use crate::parse::error::PError;

pub trait WhiteSpace<'a> {
    fn parser(&self) -> Box<dyn SParser<'a, Span<'a>>>;
}

struct RustWhiteSpace {}

impl<'a> WhiteSpace<'a> for RustWhiteSpace {
    fn parser(&self) -> Box<dyn SParser<'a, Span<'a>>> {
        Box::new(recognize(tag::<&str, Span<'a>, PError>(" ")))
    }
}