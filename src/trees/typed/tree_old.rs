use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::trees::literal::Literal;
use crate::trees::typed::call::Call;
use crate::trees::typed::var_ref::VarRef;
use crate::trees::types::Type;

pub enum TreeOld<V: Var, F: Fun> {
    Call(Call<V, F>),
    Var(VarRef<V>),
    Lit(Literal)
}

impl<V: Var, F: Fun> TreeOld<V, F> {
    pub fn tpe(&self) -> Type {
        match self {
            TreeOld::Call(call) => { call.fun.tpe() }
            TreeOld::Var(var) => { var.var.tpe() }
            TreeOld::Lit(lit) => { lit.tpe() }
        }
    }
}
