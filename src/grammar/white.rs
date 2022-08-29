use nom::bytes::complete::tag;
use nom::combinator::recognize;
use crate::{Span, SParser};
use nom::Parser;

pub trait WhiteSpace {
    fn parser<'a, 'b>(&self) -> Box<dyn SParser<'a, Span<'b>>>;
}

struct RustWhiteSpace {}

impl WhiteSpace for RustWhiteSpace {
    fn parser<'a, 'b>(&self) -> Box<dyn SParser<'a, Span<'b>>> {
        let x = recognize(tag(" "));
        Box::from(x)
    }
}