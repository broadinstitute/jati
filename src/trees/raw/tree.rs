use crate::engine::fun::Fun;
use crate::engine::var::Var;
use crate::error::Error;
use crate::trees::lit::Literal;
use crate::trees::raw::call::Call;
use crate::trees::raw::id::Id;
use crate::trees::symbols::Symbols;
use crate::trees::typer::to_typed_tree;
use crate::trees::typed::tree::Tree as TypedTree;

pub enum Tree {
    Call(Call),
    Var(Id),
    Lit(Literal),
}

impl Tree {
    pub fn into_typed<V, F, S>(self, symbols: &mut S) -> Result<TypedTree<V, F>, Error>
        where V: Var, F: Fun, S: Symbols<V, F> {
        to_typed_tree(symbols, self)
    }
}