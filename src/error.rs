use std::fmt::{Debug, Display, Formatter};
use crate::parse::error::PError;
use crate::trees::symbols::SymbolError;
use crate::trees::raw::error::TreeError;

pub enum ErrorKind { Parse, Tree, Symbol }
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
            ErrorKind::Symbol => "Symbol"
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

impl From<TreeError> for Error {
    fn from(tree_error: TreeError) -> Self { import_error(ErrorKind::Tree, tree_error) }
}

impl From<SymbolError> for Error {
    fn from(symbol_error: SymbolError) -> Self {
        import_error(ErrorKind::Symbol, symbol_error)
    }
}

fn import_error<E: std::error::Error + 'static>(kind: ErrorKind, source: E) -> Error {
    Error::Imported { kind, source: Box::new(source) }
}
