use crate::pos::Pos;
use std::fmt::{Formatter, Display};
use crate::code_point::Utf8Error;

pub(crate) enum Error {
    Parse(ParseError)
}

pub(crate) enum ParseError {
    Utf8(Utf8Error, Pos)
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::Parse(parse_error) => { Error::Parse(parse_error.clone()) }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(parse_error) => { write!(f, "{}", parse_error) }
        }
    }
}

impl ParseError {
    pub fn message(&self) -> &String {
        match self {
            ParseError::Utf8(utf8_error, _) => { &utf8_error.message }
        }
    }
    pub fn pos(&self) -> &Pos {
        match self {
            ParseError::Utf8(_, pos) => { pos }
        }
    }
}

impl Clone for ParseError {
    fn clone(&self) -> Self {
        match self {
            ParseError::Utf8(utf8_error, pos) => {
                ParseError::Utf8(utf8_error.clone(), pos.clone() )
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Utf8(utf8_error, pos) => {
                write!(f, "At {}: {}", pos, utf8_error)
            }
        }
    }
}