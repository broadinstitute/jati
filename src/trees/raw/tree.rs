use crate::Error;
use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::symbols::Symbols;
use crate::trees::typed;
use crate::trees::typed::tree::Tree as TypedTree;

pub(crate) trait Tree {
    fn into_typed<V, F, S>(self, symbols: &mut S) -> Result<Box<dyn TypedTree<V, F>>, Error>
        where V: Var, F: Fun, S: Symbols<V, F>;
}