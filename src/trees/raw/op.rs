use crate::error::Error;
use crate::runtime::Runtime;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::tree::{Operation, Tree};
use crate::trees::symbols::SymbolError;
use crate::trees::typed::op::Op as TypedOp;
use crate::trees::types::Type;


#[derive(Clone)]
pub struct Op {
    id: Id
}

impl Op {
    pub fn new(id: Id) -> Op { Op { id } }
    pub fn new_tree(&self, kids: Vec<Tree>) -> Result<Tree, Error> {
        let operation = Operation { op: self.clone(), kids };
        Ok(Tree::Operation(operation))
    }
    pub(crate) fn into_typed<R: Runtime>(self, kid_types: Vec<Type>,
                                         symbols: &mut dyn SymbolTable<R>)
                  -> Result<TypedOp, Error> {
        let id = self.id;
        let fun =
            symbols.resolve_fun(&id, &kid_types)?.ok_or_else(|| SymbolError::no_such_fun(&id))?;
        Ok(TypedOp::new(id, fun))
    }
}