use std::fmt::{Debug, Display, Formatter};
use crate::parse::error::PError;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum ErrorKind {
    Parse
}
#[derive(Clone, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    fn new(kind: ErrorKind, message: String) -> Error {
        Error { kind, message }
    }
}

impl From<PError> for Error {
    fn from(p_error: PError) -> Self {
        let message = p_error.create_report();
        let kind = ErrorKind::Parse;
        Error::new(kind, message)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Display::fmt(self, f) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {

}