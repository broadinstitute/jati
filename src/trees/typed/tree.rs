use crate::trees::MaybeChanged;
use crate::trees::typed::op::Op;
use crate::trees::typed::transform::Transformer;
use crate::trees::types::Type;
use crate::util::meta::ValueMeta;

pub struct Tree {
    pub op: Box<dyn Op>,
    pub kids: Vec<Tree>,
}

impl Tree {
    pub fn recursive_transform(self, transformer: &dyn Transformer)
                               -> ValueMeta<Self, MaybeChanged> {
        let Tree { op, kids } = self;
        let kids: Vec<Tree> =
            kids.into_iter().map(|kid| kid.recursive_transform(transformer).value).collect();
        let tree = Tree { op, kids};
        transformer.transform(tree)
    }
    pub fn tpe(&self) -> Type { self.op.tpe() }
}