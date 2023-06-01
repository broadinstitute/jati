use crate::trees::gen::transform::Transformer;
use crate::trees::MaybeChanged;
use crate::util::meta::ValueMeta;

pub trait Tree {
    fn recursive_transform(self, transformer: &dyn Transformer<Self>)
                               -> ValueMeta<Self, MaybeChanged> {
        let Tree { op, kids } = self;
        let kids: Vec<Self> =
            kids.into_iter().map(|kid| kid.recursive_transform(transformer).value).collect();
        let tree = Tree { op, kids};
        transformer.transform(tree)
    }

}