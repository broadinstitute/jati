use crate::trees::typed::op::Op;
use crate::trees::typed::var::Var;
use crate::trees::types::Type;
use crate::trees::values::Value;

pub enum Tree {
    Var(Var),
    Literal(Value),
    Op(Operation),
}

pub struct Operation {
    pub op: Op,
    pub kids: Vec<Tree>,
}

impl Tree {
    pub fn tpe(&self) -> Type {
        match self {
            Tree::Var(var) => var.tpe(),
            Tree::Literal(lit) => lit.tpe(),
            Tree::Op(op) => op.op.tpe(),
        }
    }
}