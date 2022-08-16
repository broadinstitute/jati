use nom::error::{ErrorKind, ParseError};
use crate::parse::Span;

pub(crate) struct PError {

}

impl PError {
    pub(crate) fn create_report(&self) -> String {
        todo!()
    }
}

impl<'a> ParseError<Span<'a>> for PError {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        todo!()
    }

    fn append(input: Span<'a>, kind: ErrorKind, other: Self) -> Self {
        todo!()
    }

    fn from_char(input: Span<'a>, _: char) -> Self {
        todo!()
    }

    fn or(self, other: Self) -> Self {
        todo!()
    }
}