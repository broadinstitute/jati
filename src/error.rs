use crate::pos::Pos;
use std::fmt::{Formatter, Display};
use crate::code_point::Utf8Error;

pub(crate) enum Error {
    Parse(ParseError),
    Utf8(Utf8Error, Pos)
}

pub(crate) struct ParseError {
    pos: Pos,
    message: String
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::Parse(parse_error) => { Error::Parse(parse_error.clone())}
            Error::Utf8(utf8_error, pos) => {
                Error::Utf8(utf8_error.clone(), pos.clone())
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(parse_error) => { write!(f, "{}", parse_error)}
            Error::Utf8(utf8_error, pos) => {
                write!(f, "{} at {}", utf8_error, pos)
            }
        }
    }
}

impl Clone for ParseError {
    fn clone(&self) -> Self {
        ParseError { pos: self.pos.clone(), message: self.message.clone() }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "At {}: {}", self.pos, self.message)
    }
}