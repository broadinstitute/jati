use crate::Error;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::literal::Literal;
use crate::trees::raw::op::Op;
use crate::trees::raw::var::Var;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::typed::tree::Operation as TypedOperation;
use crate::trees::types::Type;

pub enum Tree {
    Var(Var),
    Literal(Literal),
    Operation(Operation)
}

pub struct Operation {
    pub op: Box<dyn Op>,
    pub kids: Vec<Tree>
}

impl Tree {
    pub fn into_typed(self, symbols: &mut dyn SymbolTable)
                  -> Result<TypedTree, Error> {
        match self {
            Tree::Var(var) => Ok(TypedTree::Var(var.into_typed(symbols)?)),
            Tree::Literal(lit) => Ok(TypedTree::Literal(lit)),
            Tree::Operation(op) => {
                let Operation { op, kids } = op;
                let mut typed_kids: Vec<TypedTree> = Vec::new();
                for kid in kids.into_iter() {
                    typed_kids.push(kid.into_typed(symbols)?)
                }
                let kid_types = typed_kids.iter().map(|kid| kid.tpe()).collect::<Vec<Type>>();
                let op = op.into_typed(kid_types, symbols)?;
                Ok(TypedTree::Operation (TypedOperation { kids: typed_kids, op }))
            }
        }
    }
}