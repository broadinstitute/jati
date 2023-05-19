use uuid::Uuid;
use crate::error::Error;
use crate::trees::types::Type;

#[derive(Copy, Clone)]
pub struct FunKey {
    uuid: Uuid
}

pub trait FunSig {
    fn tpe(&self) -> Type;
    fn check_arg_types(&self, arg_types: &[&Type]) -> Result<(), Error>;
}

pub struct FunTag {
    key: FunKey,
    sig: Box<dyn FunSig>
}

