use crate::error::Error;
use crate::input::Input;

pub(crate) mod error;
pub mod parsers;

pub trait Parser {
    type Output;
    fn parse<C: Iterator<Item=Result<char, Error>> + Clone, I: Into<Input<C>>>(input: I)
        -> ParseResult<Self::Output>;
}

pub enum ParseResult<O> {
    Success(O),
    Error(Error),
    Failure(Failure),
}

pub struct Failure {
    pub actual: String,
    pub expected: String,
}