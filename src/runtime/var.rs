use uuid::Uuid;
use crate::trees::types::Type;

#[derive(Copy, Clone)]
pub struct VarId {
    uuid: Uuid
}

pub struct VarSig {
    id: VarId,
    tpe: Type
}

pub trait Var {
    fn tpe(&self) -> Type;
}