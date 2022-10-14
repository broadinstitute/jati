use crate::trees::types::Type;

pub trait Fun {
    fn tpe(&self) -> Type;
}