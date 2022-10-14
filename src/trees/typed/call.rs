use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::typed::tree::Tree;

pub struct Call<V: Var, F: Fun> {
    pub name: String,
    pub fun: F,
    pub args: Vec<Tree<V, F>>
}


