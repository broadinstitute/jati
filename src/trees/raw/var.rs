use crate::error::Error;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::error::TreeError;
use crate::trees::raw::op::Op;
use crate::trees::raw::tree::Tree;
use crate::trees::symbols::SymbolError;
use crate::trees::typed::op::Op as TypedOp;
use crate::trees::typed::var::Var as TypedVar;
use crate::trees::types::Type;

#[derive(Clone)]
pub struct Var {
    id: Id
}

impl Var {
    fn into_typed_var(self, symbols: &mut dyn SymbolTable) -> Result<TypedVar, Error> {
        match symbols.get_var(&self.id) {
            Ok(Some(tag)) => { Ok(TypedVar { id: self.id, tag }) }
            Ok(None) => { Err(Error::from(SymbolError::no_such_var(&self.id))) }
            Err(error) => { Err(error) }
        }
    }
}

impl Op for Var {
    fn new_tree(&self, kids: Vec<Tree>) -> Result<Tree, Error> {
        if !kids.is_empty() {
            Err(TreeError::new(
                format!("Malformed tree: variable {} cannot have kids", self.id)
            ))?
        } else {
            let op = Box:: new(self.clone()) as Box<dyn Op>;
            Ok(Tree { op, kids })
        }
    }

    fn into_typed(self: Box<Self>, _kid_types: Vec<Type>, symbols: &mut dyn SymbolTable)
        -> Result<Box<dyn TypedOp>, Error> {
        let id = self.id;
        let tag = symbols.get_var(&id)?.ok_or_else(|| SymbolError::no_such_var(&id))?;
        Ok(Box::new(TypedVar { id, tag }) as Box<dyn TypedOp>)
    }
}
