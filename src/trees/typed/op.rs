use crate::symbols::fun::OpTag;
use crate::symbols::id::Id;
use crate::trees::types::Type;

pub struct Op {
    pub id: Id,
    pub fun: OpTag
}

impl Op {
    pub fn new(id: Id, fun: OpTag) -> Op { Op { id, fun } }
    pub fn tpe(&self) -> Type { self.fun.sig.tpe() }

}