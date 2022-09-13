use crate::trees::lit::Literal;
use crate::trees::raw::call::Call;
use crate::trees::raw::id::Id;

pub enum Tree {
    Call(Call),
    Var(Id),
    Lit(Literal)
}
