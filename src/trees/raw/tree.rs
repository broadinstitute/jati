use crate::Error;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::typed::tree::Tree as TypedTree;

pub(crate) trait Tree {
    fn into_typed(self, symbols: &mut dyn SymbolTable)
        -> Result<Box<dyn TypedTree>, Error>;
}