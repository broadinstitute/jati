use crate::symbols::id::Id;
use crate::symbols::var::VarTag;
use crate::trees::types::Type;

pub struct Var {
    pub id: Id,
    pub tag: VarTag
}

impl Var {
    pub(crate) fn tpe(&self) -> Type { self.tag.sig.tpe }
}

