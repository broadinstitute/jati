use uuid::Uuid;

use crate::trees::types::Type;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VarKey {
    uuid: Uuid
}

impl VarKey {
    pub fn next() -> VarKey {
        let uuid = Uuid::new_v4();
        VarKey { uuid }
    }
    pub fn create_random() -> VarKey {
        let uuid = Uuid::new_v4();
        VarKey { uuid }
    }
}

#[derive(Clone)]
pub struct VarSig {
    pub(crate) tpe: Type
}

#[derive(Clone)]
pub struct VarTag {
    pub key: VarKey,
    pub(crate) sig: VarSig
}