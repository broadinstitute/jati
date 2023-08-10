use crate::Error;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::op::Op;
use crate::trees::raw::transform::Transformer;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::types::Type;

pub struct Tree {
    pub op: Box<dyn Op>,
    pub kids: Vec<Tree>
}

impl Tree {
    pub fn into_typed(self, symbols: &mut dyn SymbolTable)
                  -> Result<TypedTree, Error> {
        let Tree { op, kids: raw_kids} = self;
        let mut kids: Vec<TypedTree> = Vec::new();
        for raw_kid in raw_kids.into_iter() {
            kids.push(Box::new(raw_kid).into_typed(symbols)?)
        }
        let kid_types = kids.iter().map(|kid| kid.tpe()).collect::<Vec<Type>>();
        let op = op.into_typed(kid_types, symbols)?;
        Ok(TypedTree { kids, op })
    }
}