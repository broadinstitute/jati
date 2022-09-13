use crate::engine::fun::Fun;
use crate::engine::var::Var;
use crate::trees::typed::tree::Tree;

pub(crate) struct Call<V: Var, F: Fun> {
    pub(crate) name: String,
    pub(crate) fun: F,
    pub(crate) args: Vec<Tree<V, F>>
}


