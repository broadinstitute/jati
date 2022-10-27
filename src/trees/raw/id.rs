use crate::Error;
use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::raw::tree::Tree as RawTree;
use crate::trees::symbols::Symbols;
use crate::trees::typed::tree::Tree as TypedTree;

pub struct Id {
    pub string: String
}

impl Id {
    pub fn new(name: String) -> Id { Id { string: name } }
}

impl RawTree for Id {
    fn into_typed<V, F, S>(self, symbols: &mut S) -> Result<Box<dyn TypedTree<V, F>>, Error>
        where V: Var, F: Fun, S: Symbols<V, F> {
        todo!()
    }
}