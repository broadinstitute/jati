use crate::engine::var::Var;

pub(crate) struct VarRef<V: Var> {
    pub(crate) name: String,
    pub(crate) var: V
}