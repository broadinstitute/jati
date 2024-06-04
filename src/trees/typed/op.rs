use crate::symbols::id::Id;
use crate::symbols::ops::{OpKey, OpTag};
use crate::trees::types::Type;

pub struct Op {
    pub id: Id,
    pub fun: OpTag
}

impl Op {
    pub fn new(id: Id, fun: OpTag) -> Op { Op { id, fun } }
    pub fn tpe(&self) -> Type { self.fun.sig.tpe() }
    pub fn key(&self) -> &OpKey { &self.fun.key }
}