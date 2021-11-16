use crate::pos::Pos;
use std::fmt::{Formatter, Display};

pub(crate) enum Error {
    Parse(ParseError)
}

pub(crate) struct ParseError {
    pos: Pos,
    message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(parse_error) => { write!(f, "{}", parse_error)}
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