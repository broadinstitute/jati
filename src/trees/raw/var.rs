use crate::error::Error;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::tree::Tree;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::typed::var::Var as TypedVar;

pub struct Var {
    id: Id
}

impl Var {
    fn into_typed_var(self, symbols: &mut dyn SymbolTable) -> Result<TypedVar, Error> {
        match symbols.get_var(&self.id) {
            Ok(Some(tag)) => { Ok(TypedVar { id: self.id, tag }) }
            Ok(None) => { Err(Error::from(format!("Unknown variable {}.", self.id))) }
            Err(error) => { Err(error) }
        }
    }
}

impl Tree for Var {
    fn into_typed(self, symbols: &mut dyn SymbolTable) -> Result<Box<dyn TypedTree>, Error> {
        Ok(Box::new(self.into_typed_var(symbols)?))
    }
}