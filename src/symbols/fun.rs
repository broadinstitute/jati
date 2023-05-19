use std::sync::Arc;
use uuid::Uuid;
use crate::trees::symbols::ArgsFailure;
use crate::trees::types::Type;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FunKey {
    uuid: Uuid
}

impl FunKey {
    pub fn next() -> FunKey {
        let uuid = Uuid::new_v4();
        FunKey { uuid }
    }
}

pub trait FunSig {
    fn tpe(&self) -> Type;
    fn check_arg_types(&self, arg_types: &[&Type]) -> Result<(), ArgsFailure>;
}

#[derive(Clone)]
pub struct FunTag {
    pub key: FunKey,
    pub sig: Arc<dyn FunSig>
}

