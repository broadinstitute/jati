use std::rc::Rc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::error::context;
use crate::{PResult, Span, SParser};
use crate::trees::literal::Literal;
use nom::number::complete::double;

pub trait LiteralParser: SParser<Literal> {}

pub struct DefaultLiteralParser {}

impl DefaultLiteralParser {
    pub fn new_unboxed() -> DefaultLiteralParser { DefaultLiteralParser {} }
    pub fn new() -> Rc<DefaultLiteralParser> { Rc::new(Self::new_unboxed()) }
}

impl SParser<Literal> for DefaultLiteralParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Literal> {
        alt((context("float", map(double, |num| { Literal::Float(num) })),
             context("true", value(Literal::Bool(true), tag("true"))),
             context("false", value(Literal::Bool(false), tag("false")))
        ))(span)
    }
}

impl LiteralParser for DefaultLiteralParser {}

impl Default for DefaultLiteralParser {
    fn default() -> Self { Self::new_unboxed() }
}