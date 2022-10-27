use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::types::Type;

pub trait Tree<V: Var, F: Fun> {
    fn tpe(&self) -> Type;
}