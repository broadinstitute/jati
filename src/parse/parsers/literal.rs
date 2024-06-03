use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::error::context;
use nom::number::complete::double;

use crate::{PResult, Span, SParser};
use crate::trees::values::Value;

pub trait LiteralParser: SParser<Value> {}

pub struct DefaultLiteralParser {}

impl DefaultLiteralParser {
    pub fn new_unboxed() -> DefaultLiteralParser { DefaultLiteralParser {} }
    pub fn new() -> Rc<DefaultLiteralParser> { Rc::new(Self::new_unboxed()) }
}

impl SParser<Value> for DefaultLiteralParser {
    fn parse_span<'a>(&self, span: Span<'a>) -> PResult<'a, Value> {
        alt((context("float", map(double, |num| { Value::Float(num) })),
             context("true", value(Value::Bool(true), tag("true"))),
             context("false", value(Value::Bool(false), tag("false")))
        ))(span)
    }
}

impl LiteralParser for DefaultLiteralParser {}

impl Default for DefaultLiteralParser {
    fn default() -> Self { Self::new_unboxed() }
}