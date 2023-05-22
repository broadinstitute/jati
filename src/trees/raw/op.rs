use crate::error::Error;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::tree::Tree;
use crate::trees::typed::op::Op as TypedOp;
use crate::trees::types::Type;

pub trait Op {
    fn new_tree(&self, kids: Vec<Tree>) -> Result<Tree, Error>;
    fn into_typed(self: Box<Self>, kid_types: Vec<Type>, symbols: &mut dyn SymbolTable)
                  -> Result<Box<dyn TypedOp>, Error>;
}