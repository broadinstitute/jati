use crate::trees::MaybeChanged;
use crate::trees::typed::tree::Tree;
use crate::util::meta::ValueMeta;

pub trait Transformer {
    fn transform(&self, tree: Tree) -> ValueMeta<Tree, MaybeChanged>;
}