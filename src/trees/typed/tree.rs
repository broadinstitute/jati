use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::lit::Literal;
use crate::trees::typed::call::Call;
use crate::trees::typed::var_ref::VarRef;
use crate::trees::types::Type;

pub enum Tree<V: Var, F: Fun> {
    Call(Call<V, F>),
    Var(VarRef<V>),
    Lit(Literal)
}

impl<V: Var, F: Fun> Tree<V, F> {
    pub fn tpe(&self) -> Type {
        match self {
            Tree::Call(call) => { call.fun.tpe() }
            Tree::Var(var) => { var.var.tpe() }
            Tree::Lit(lit) => { lit.tpe() }
        }
    }
}
