use crate::trees::gen::tree::Tree;
use crate::trees::MaybeChanged;
use crate::util::meta::ValueMeta;

pub trait Transformer<T: Tree> {
    fn transform(&self, tree: T) -> ValueMeta<T, MaybeChanged>;
}
