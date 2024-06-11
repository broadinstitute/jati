use crate::error::Error;
use crate::runtime::Runtime;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::tree::Tree;
use crate::trees::symbols::SymbolError;
use crate::trees::typed::op::OpExpression as TypedOpExpression;
use crate::trees::typed::op::NonIdOp as TypedNonIdOp;
use crate::trees::types::Type;

#[derive(Clone)]
pub enum Op {
    NonId(NonIdOp),
    Id(IdOp)
}

#[derive(Clone)]
pub enum NonIdOp {
    Block
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct IdOp {
    id: Id,
    syntax: OpSyntax
}

#[derive(Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum OpSyntax {
    Call
}

pub struct OpExpression {
    pub op: Op,
    pub kids: Vec<Tree>
}

impl IdOp {
    pub fn new(id: Id, syntax: OpSyntax) -> IdOp { IdOp { id, syntax } }
}

impl Op {

}
impl OpExpression {
    pub fn new(op: Op, kids: Vec<Tree>) -> OpExpression { OpExpression { op, kids } }
    pub fn into_tree(self) -> Tree {
        Tree::Op(self)
    }
    pub(crate) fn into_typed<R: Runtime>(self, kid_types: Vec<Type>,
                                         symbols: &mut dyn SymbolTable<R>)
                  -> Result<TypedOpExpression, Error> {
        let mut typed_kids: Vec<crate::trees::typed::tree::Tree> = Vec::new();
        for kid in kids.into_iter() {
            typed_kids.push(kid.into_typed(symbols)?)
        }
        match self {
            Op::NonId(non_id_op) => {
                match non_id_op {
                    NonIdOp::Block => {
                        let op = TypedNonIdOp::Block;
                        Ok(TypedOpExpression::new(op, typed_kids))
                    }
                }
            }
            Op::Id(id_op) => {
                let id = id_op.id;
                let fun =
                    symbols.resolve_fun(&id_op, &kid_types)?
                        .ok_or_else(|| SymbolError::no_such_fun(&id))?;
                Ok(TypedOp::new(id, fun))
            }
        }
    }
}