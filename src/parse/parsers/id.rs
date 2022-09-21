use std::rc::Rc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{map, recognize};
use nom::error::context;
use nom::multi::many0_count;
use nom::sequence::pair;
use crate::trees::raw::id::Id;
use crate::{PResult, Span, SParser};
use crate::parse::error::PError;

pub trait IdParser: SParser<Id> {}

pub struct RustIdParser {}

impl RustIdParser {
    pub fn new_unboxed() -> RustIdParser { RustIdParser {} }
    pub fn new() -> Rc<RustIdParser> { Rc::new(Self::new_unboxed()) }
}

impl SParser<Id> for RustIdParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Id> {
        context("id",
                map(recognize(
                    pair(
                        alt((alpha1::<Span, PError>, tag("_"))),
                        many0_count(alt((alphanumeric1, tag("_")))),
                    )
                ), |s| Id::new(s.to_string())),
        )(span)
    }
}

impl Default for RustIdParser {
    fn default() -> Self { Self::new_unboxed() }
}

impl IdParser for RustIdParser {}