use crate::symbols::id::Id;
use crate::symbols::ops::{OpKey, OpTag};
use crate::trees::typed::tree::Tree;
use crate::trees::types::Type;


pub enum Op {
    NonId(NonIdOp),
    Id(IdOp)
}
pub enum NonIdOp {
    Block
}
pub struct IdOp {
    pub id: Id,
    pub fun: OpTag
}

pub struct OpExpression {
    pub op: Op,
    pub kids: Vec<Tree>
}
impl IdOp {
    pub fn new(id: Id, fun: OpTag) -> IdOp { IdOp { id, fun } }
    pub fn tpe(&self) -> Type { self.fun.sig.tpe() }
    pub fn key(&self) -> &OpKey { &self.fun.key }
}