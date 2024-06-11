use crate::Error;
use crate::runtime::Runtime;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::op::OpExpression;
use crate::trees::raw::var::Var;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::values::Value;

pub enum Tree {
    Var(Var),
    Literal(Value),
    Op(OpExpression)
}

impl Tree {
    pub fn into_typed<R: Runtime>(self, symbols: &mut dyn SymbolTable<R>)
                  -> Result<TypedTree, Error> {
        match self {
            Tree::Var(var) => Ok(TypedTree::Var(var.into_typed(symbols)?)),
            Tree::Literal(value) => Ok(TypedTree::Literal(value)),
            Tree::Op(op_expression) => {
                Ok(TypedTree::Op(op_expression.into_typed(symbols)?))
            }
        }
    }
}