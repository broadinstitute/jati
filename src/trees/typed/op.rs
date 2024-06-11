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

impl NonIdOp {
    pub fn tpe(&self, kids: &[Tree]) -> Type {
        match self {
            NonIdOp::Block => {
                kids.last().map_or(Type::Unit, |kid| kid.tpe())
            }
        }
    }
}
impl IdOp {
    pub fn new(id: Id, fun: OpTag) -> IdOp { IdOp { id, fun } }
    pub fn tpe(&self) -> Type { self.fun.sig.tpe() }
    pub fn key(&self) -> &OpKey { &self.fun.key }
}

impl OpExpression {
    pub fn new(op: Op, kids: Vec<Tree>) -> OpExpression { OpExpression { op, kids } }
    pub fn tpe(&self) -> Type {
        match &self.op {
            Op::NonId(non_id_op) => non_id_op.tpe(&self.kids),
            Op::Id(id_op) => id_op.tpe()
        }
    }
}