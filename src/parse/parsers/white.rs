use nom::branch::alt;
use nom::bytes::complete::{tag, is_not, is_a, take_until};
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::error::context;
use nom::multi::many0;
use crate::{PResult, Span, SParser};
use nom::sequence::tuple;

pub trait WhiteSpaceParser: SParser<()> {}

pub struct DefaultWhiteSpaceParser {}

impl DefaultWhiteSpaceParser {
    pub fn new() -> DefaultWhiteSpaceParser { DefaultWhiteSpaceParser {} }
}

impl SParser<()> for DefaultWhiteSpaceParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, ()> {
        let space = value((), is_a(" \t\n\r"));
        let slash_slash_comment =
            value((), tuple((tag("//"), is_not("\n\r"), line_ending)));
        let hash_comment =
            value((), tuple((tag("//"), is_not("\n\r"), line_ending)));
        let block_comment =
            value((), tuple((tag("/*"), take_until("*/"), tag("*/"))));
        context("ws",
                value((), many0(alt((space, slash_slash_comment, hash_comment,
                                     block_comment)))))(span)
    }
}

impl Default for DefaultWhiteSpaceParser {
    fn default() -> Self { Self::new() }
}

impl WhiteSpaceParser for DefaultWhiteSpaceParser {}