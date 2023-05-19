use crate::trees::literal::Literal;
use crate::trees::raw::call::Call;
use crate::symbols::id::Id;

pub enum TreeOld {
    Call(Call),
    Var(Id),
    Lit(Literal),
}
