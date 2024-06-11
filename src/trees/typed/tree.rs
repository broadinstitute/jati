use std::fmt::Display;

use crate::trees::typed::op::OpExpression;
use crate::trees::typed::var::Var;
use crate::trees::types::Type;
use crate::trees::values::Value;

pub enum Tree {
    Var(Var),
    Literal(Value),
    Op(OpExpression),
}

impl Tree {
    pub fn tpe(&self) -> Type {
        match self {
            Tree::Var(var) => var.tpe(),
            Tree::Literal(lit) => lit.tpe(),
            Tree::Op(op) => op.tpe(),
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Var(var) => { write!(f, "{}", var.id) }
            Tree::Literal(lit) => { write!(f, "{lit}") }
            Tree::Op(op) => { todo!() }
        }
    }
}