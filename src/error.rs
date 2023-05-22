use std::fmt::{Debug, Display, Formatter};
use crate::parse::error::PError;
use crate::trees::symbols::SymbolError;
use crate::trees::raw::error::TreeError;

pub enum Error {
    Parse(PError),
    Tree(TreeError),
    Symbol(SymbolError)
}

impl From<PError> for Error {
    fn from(p_error: PError) -> Self {
        Error::Parse(p_error)
    }
}

impl From<TreeError> for Error {
    fn from(tree_error: TreeError) -> Self {
        Error::Tree(tree_error)
    }
}

impl From<SymbolError> for Error {
    fn from(symbol_error: SymbolError) -> Self {
        Error::Symbol(symbol_error)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Display::fmt(self, f) }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(p_error) => { write!(f, "Parse error: {}", p_error)}
            Error::Symbol(symbol_error) => { write!(f, "{}", symbol_error)}
            Error::Tree(tree_error) => { write!(f, "{}", tree_error)}
        }
    }
}

impl std::error::Error for Error {}