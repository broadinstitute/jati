use std::fmt::{Display, Formatter};
use crate::error::Error;
use crate::runtime::Runtime;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::op::OpExpression;
use crate::trees::props::{Props, Typed};
use crate::trees::types::Type;
use crate::trees::values::Value;
use crate::trees::var::Var;

pub enum Tree<P: Props> {
    Var(Var<P>),
    Literal(Value),
    Op(OpExpression<P>),
}

impl<P: Props> Tree<P> {
    pub fn into_typed<R: Runtime>(self, symbols: &mut dyn SymbolTable<R>)
                                  -> Result<Tree<Typed>, Error> {
        match self {
            Tree::Var(var) => Ok(Tree::Var::<Typed>(var.into_typed(symbols)?)),
            Tree::Literal(value) => Ok(Tree::Literal(value)),
            Tree::Op(op_expression) => {
                Ok(Tree::Op::<Typed>(op_expression.into_typed(symbols)?))
            }
        }
    }
}

impl Tree<Typed> {
    pub fn tpe(&self) -> Type {
        match self {
            Tree::Var(var) => var.tpe(),
            Tree::Literal(lit) => lit.tpe(),
            Tree::Op(op) => op.tpe(),
        }
    }
}

impl<P: Props> Display for Tree<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Var(var) => { write!(f, "{var}") }
            Tree::Literal(value) => { write!(f, "{value}") }
            Tree::Op(op) => { write!(f, "{op}") }
        }
    }
}
