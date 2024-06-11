use crate::Error;
use crate::runtime::Runtime;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::op::OpExpression;
use crate::trees::raw::var::Var;
use crate::trees::typed::tree::OpCall as TypedOperation;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::types::Type;
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
            Tree::Op(op) => {
                let OpExpression { op, kids } = op;
                let mut typed_kids: Vec<TypedTree> = Vec::new();
                for kid in kids.into_iter() {
                    typed_kids.push(kid.into_typed(symbols)?)
                }
                let kid_types =
                    typed_kids.iter().map(|kid| kid.tpe()).collect::<Vec<Type>>();
                let op = op.into_typed(kid_types, symbols)?;
                Ok(TypedTree::Op(TypedOperation { kids: typed_kids, op }))
            }
        }
    }
}