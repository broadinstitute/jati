use crate::Error;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::typed::tree::Tree as TypedTree;

pub trait Tree {
    fn into_typed(self: Box<Self>, symbols: &mut dyn SymbolTable)
        -> Result<Box<dyn TypedTree>, Error>;
}