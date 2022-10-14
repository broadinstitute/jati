use crate::trees::types::Type;

pub trait Var {
    fn tpe(&self) -> Type;
}