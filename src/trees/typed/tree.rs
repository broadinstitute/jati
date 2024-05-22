use crate::trees::literal::Literal;
use crate::trees::typed::op::Op;
use crate::trees::typed::var::Var;
use crate::trees::types::Type;

pub enum Tree {
    Var(Var),
    Literal(Literal),
    Operation(Operation),
}

pub struct Operation {
    pub op: Box<dyn Op>,
    pub kids: Vec<Tree>,
}

impl Tree {
    pub fn tpe(&self) -> Type {
        match self {
            Tree::Var(var) => var.tpe(),
            Tree::Literal(lit) => lit.tpe(),
            Tree::Operation(op) => op.op.tpe(),
        }
    }
}