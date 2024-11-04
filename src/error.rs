use std::fmt::{Debug, Display, Formatter};
use crate::parse::error::PError;
use crate::trees::symbols::SymbolError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind { Parse, Tree, Symbol, IO }

pub enum Error {
    Root { message: String },
    Child { message: String, source: Box<Error> },
    Imported { kind: ErrorKind, source: Box<dyn std::error::Error> },
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ErrorKind::Parse => "Parse",
            ErrorKind::Tree => "Tree",
            ErrorKind::Symbol => "Symbol",
            ErrorKind::IO => "IO"
        }
    }
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Root { message } => { write!(f, "{}", message) }
            Error::Child { message, source } =>
                { write!(f, "{message}: {source}") }
            Error::Imported { kind, source } =>
                { write!(f, "{kind}: {source}") }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Root { .. } => None,
            Error::Child { source, .. } => Some(source.as_ref()),
            Error::Imported { source, .. } => Some(source.as_ref()),
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::Root { message } => Error::Root { message: message.clone() },
            Error::Child { message, source } =>
                Error::Child { message: message.clone(), source: source.clone() },
            Error::Imported { kind, source } =>
                Error::Imported { kind: *kind, source: approximate_deep_clone(source.as_ref()) }
        }
    }
}

fn approximate_deep_clone(error: &dyn std::error::Error) -> Box<Error> {
    match error.source() {
        None => Box::new(Error::from(error.to_string())),
        Some(source) => Box::new(Error::Child {
            message: error.to_string(),
            source: approximate_deep_clone(source)
        })
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Root { message }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::Root { message: message.to_string() }
    }
}


impl From<PError> for Error {
    fn from(parse_error: PError) -> Self  {
        import_error(ErrorKind::Parse, parse_error)
    }
}

impl From<SymbolError> for Error {
    fn from(symbol_error: SymbolError) -> Self {
        import_error(ErrorKind::Symbol, symbol_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        import_error(ErrorKind::IO, io_error)
    }
}

fn import_error<E: std::error::Error + 'static>(kind: ErrorKind, source: E) -> Error {
    Error::Imported { kind, source: Box::new(source) }
}
