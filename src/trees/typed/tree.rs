use crate::trees::typed::op::Op;
use crate::trees::types::Type;

pub struct Tree {
    pub op: Box<dyn Op>,
    pub kids: Vec<Tree>,
}

impl Tree {
    pub fn tpe(&self) -> Type { self.op.tpe() }
}