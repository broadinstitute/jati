use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::typed::tree_old::TreeOld;

pub struct Call<V: Var, F: Fun> {
    pub name: String,
    pub fun: F,
    pub args: Vec<TreeOld<V, F>>
}


