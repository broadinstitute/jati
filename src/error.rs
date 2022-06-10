use crate::pos::Pos;
use crate::code_point::Utf8Error;

pub enum ParseError {
    Utf8(Utf8Error, Pos),
    InputEmpty
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
