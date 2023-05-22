use crate::symbols::fun::FunTag;
use crate::symbols::id::Id;
use crate::trees::typed::op::Op;
use crate::trees::types::Type;

pub struct Call {
    pub id: Id,
    pub fun: FunTag
}

impl Op for Call {
    fn tpe(&self) -> Type { self.fun.sig.tpe() }
}
