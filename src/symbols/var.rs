use std::sync::Arc;
use uuid::Uuid;
use crate::trees::types::Type;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VarKey {
    uuid: Uuid
}

pub trait VarSig {
    fn tpe(&self) -> Type;
}

pub struct VarTag {
    pub key: VarKey,
    pub(crate) sig: Arc<dyn VarSig>
}