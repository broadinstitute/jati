use crate::trees::literal::Literal;
use crate::trees::typed::call::Call;
use crate::trees::typed::var::Var;
use crate::trees::types::Type;

pub enum TreeOld {
    Call(Call),
    Var(Var),
    Lit(Literal)
}

impl TreeOld {
    pub fn tpe(&self) -> Type {
        match self {
            TreeOld::Call(call) => { call.fun.tpe() }
            TreeOld::Var(var) => { var.var.tpe() }
            TreeOld::Lit(lit) => { lit.tpe() }
        }
    }
}
