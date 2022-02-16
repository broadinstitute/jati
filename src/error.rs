use crate::pos::Pos;
use std::fmt::{Formatter, Display};
use crate::code_point::Utf8Error;
use crate::pos;

pub enum Error {
    Parse(ParseError)
}

pub enum ParseError {
    Utf8(Utf8Error, Pos),
    InputEmpty
}

const INPUT_EMPTY_MESSAGE: &str = "No input provided.";

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
    pub fn message(&self) -> &str {
        match self {
            ParseError::Utf8(utf8_error, _) => { &utf8_error.message }
            ParseError::InputEmpty => { INPUT_EMPTY_MESSAGE }
        }
    }
    pub fn pos(&self) -> &Pos {
        match self {
            ParseError::Utf8(_, pos) => { pos }
            ParseError::InputEmpty => { &pos::POS_ZERO }
        }
    }
}

impl Clone for ParseError {
    fn clone(&self) -> Self {
        match self {
            ParseError::Utf8(utf8_error, pos) => {
                ParseError::Utf8(utf8_error.clone(), pos.clone() )
            }
            ParseError::InputEmpty => { ParseError::InputEmpty }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Utf8(utf8_error, pos) => {
                write!(f, "At {}: {}", pos, utf8_error)
            }
            ParseError::InputEmpty => {
                write!(f, "At {}: {}", pos::POS_ZERO, INPUT_EMPTY_MESSAGE)
            }
        }
    }
}