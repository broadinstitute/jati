use crate::engine::var::Var;

pub struct VarRef<V: Var> {
    pub name: String,
    pub(crate) var: V
}