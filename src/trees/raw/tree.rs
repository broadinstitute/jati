use crate::Error;
use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::symbols::Symbols;
use crate::trees::typed::tree::Tree as TypedTree;

pub(crate) trait Tree {
    fn into_typed(self, symbols: &mut Symbols) -> Result<Box<dyn TypedTree>, Error>;
}