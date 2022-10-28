use uuid::Uuid;
use crate::trees::types::Type;

#[derive(Copy, Clone)]
pub struct FunId {
    uuid: Uuid
}

pub struct FunSig {
    id: FunId,
    tpe: Type
}

pub trait Fun {
    fn tpe(&self) -> Type;
}