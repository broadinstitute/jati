use crate::engine::fun::Fun;
use crate::engine::var::Var;
use crate::trees::typed::tree::Tree;

pub struct Call<V: Var, F: Fun> {
    pub name: String,
    pub fun: F,
    pub args: Vec<Tree<V, F>>
}


