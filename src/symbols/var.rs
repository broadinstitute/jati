use uuid::Uuid;
use crate::trees::types::Type;

#[derive(Copy, Clone)]
pub struct VarKey {
    uuid: Uuid
}

pub trait VarSig {
    fn tpe(&self) -> Type;
}

pub struct VarTag {
    key: VarKey,
    pub(crate) sig: Box<dyn VarSig>
}