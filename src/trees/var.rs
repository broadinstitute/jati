use std::fmt::{Display, Formatter};
use crate::error::Error;
use crate::runtime::Runtime;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::props::{Props, Typed};
use crate::trees::symbols::SymbolError;
use crate::trees::types::Type;

pub struct Var<P: Props> {
    pub id: Id,
    pub tag: P::VT
}

impl Var<Typed> {
    pub(crate) fn tpe(&self) -> Type { self.tag.sig.tpe }
}

impl<P: Props> Var<P> {
    pub(crate) fn into_typed<R: Runtime>(self, symbols: &mut dyn SymbolTable<R>)
                                         -> Result<Var<Typed>, Error> {
        let id = self.id;
        let tag = symbols.resolve_var(&id)?.ok_or_else(|| SymbolError::no_such_var(&id))?;
        Ok(Var::<Typed> { id, tag })
    }
}

impl<P: Props> Display for Var<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
