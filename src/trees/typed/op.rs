use crate::trees::types::Type;

pub trait Op {
    fn tpe(&self) -> Type;
}