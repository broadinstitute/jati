use std::fmt::{Debug, Display, Formatter};

pub(crate) enum ErrorKind {
    OutOfRange
}

pub(crate) struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, message: String) -> Error {
        Error { kind, message }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OutOfRange => { writeln!(f, "OutOfRange") }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Display::fmt(self, f) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {

}