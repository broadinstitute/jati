use nom::branch::alt;
use nom::bytes::complete::{tag, is_not, is_a, take_until};
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many0;
use crate::{PResult, Span, SParser};
use nom::sequence::tuple;

pub trait WhiteSpace: SParser<()> {}

struct RustWhiteSpace {}

impl SParser<()> for RustWhiteSpace {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, ()> {
        let space = value((), is_a(" \t\n\r"));
        let line_comment =
            value((), tuple((tag("//"), is_not("\n\r"), line_ending)));
        let block_comment =
            value((), tuple((tag("/*"), take_until("*/"), tag("*/"))));
        value((), many0(alt((space, line_comment, block_comment))))(span)
    }
}

impl WhiteSpace for RustWhiteSpace {}