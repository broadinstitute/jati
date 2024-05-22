use crate::error::Error;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::symbols::SymbolError;
use crate::trees::typed::var::Var as TypedVar;

#[derive(Clone)]
pub struct Var {
    id: Id
}

impl Var {
    pub(crate) fn into_typed(self, symbols: &mut dyn SymbolTable)
                             -> Result<TypedVar, Error> {
        let id = self.id;
        let tag = symbols.get_var(&id)?.ok_or_else(|| SymbolError::no_such_var(&id))?;
        Ok(TypedVar { id, tag })
    }
}
