use crate::error::Error;

pub(crate) mod error;
pub mod parsers;

pub trait Parser {
    type Output;
    fn parse<B: Iterator<Item=u8> + Clone>(bytes: B) -> ParseResult<Self::Output>;
}

pub enum ParseResult<O> {
    Success(O),
    Error(Error),
    Failure(Failure),
}

pub struct Failure {
    actual: String,
    expected: String,
}