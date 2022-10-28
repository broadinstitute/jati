use crate::trees::types::Type;

pub trait Tree {
    fn tpe(&self) -> Type;
}