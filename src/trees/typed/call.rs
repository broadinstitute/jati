use crate::symbols::fun::FunTag;
use crate::symbols::id::Id;
use crate::trees::typed::tree::Tree;
use crate::trees::types::Type;

pub struct Call {
    pub id: Id,
    pub fun: FunTag,
    pub args: Vec<Box<dyn Tree>>
}

impl Tree for Call {
    fn tpe(&self) -> Type { self.fun.sig.tpe() }
}
