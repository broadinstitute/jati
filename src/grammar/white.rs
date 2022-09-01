use nom::branch::alt;
use nom::bytes::complete::{tag, is_not, is_a};
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use crate::{PResult, Span, SParser};
use nom::sequence::separated_pair;

pub trait WhiteSpace: SParser<()> {}

struct RustWhiteSpace {}

impl SParser<()> for RustWhiteSpace {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, ()> {
        let space = map(is_a(" \t\n\r"), |_| {});
        let line_comment =
            map(separated_pair(tag("//"), is_not("\n\r"), line_ending),
                |_| {});
        map(many0(alt((space, line_comment))), |_| {})(span)
    }
}

impl WhiteSpace for RustWhiteSpace {}