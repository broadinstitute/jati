use crate::Error;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::op::Op;
use crate::trees::raw::tree::{Operation, Tree};
use crate::trees::symbols::SymbolError;
use crate::trees::typed::call::Call as TypedCall;
use crate::trees::typed::op::Op as TypedOp;
use crate::trees::types::Type;

#[derive(Clone)]
pub struct Call {
    pub(crate) id: Id
}

impl Call {
    pub(crate) fn new(id: Id) -> Call { Call { id } }
}

impl Op for Call {
    fn new_tree(&self, kids: Vec<Tree>) -> Result<Tree, Error> {
        let operation = Operation { op: Box::new(self.clone()), kids };
        Ok(Tree::Operation(operation))
    }

    fn into_typed(self: Box<Self>, kid_types: Vec<Type>, symbols: &mut dyn SymbolTable)
        -> Result<Box<dyn TypedOp>, Error> {
        let id = self.id;
        let fun =
            symbols.get_fun(&id, &kid_types)?.ok_or_else(|| SymbolError::no_such_fun(&id))?;
        Ok(Box::new(TypedCall { id, fun }) as Box<dyn TypedOp>)
    }
}

